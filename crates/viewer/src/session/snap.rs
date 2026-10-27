//! **The mate tool's snap** ([`super::SessionOp::AddMate`]): the values
//! a freshly mated copy is written at so that it moves only by the snap.
//!
//! A mate fixes a direction `d` and, for a plane or an axis, a plane or
//! a line the copy's mated pose lies on. The snap is the least motion
//! that makes the mate hold: the copy's mated direction, where the copy
//! stands, turned by the least turn onto `d`, and its mated point carried
//! onto the target's plane or line by the shortest move. The gesture
//! then writes the orientation nearest the snapped turn, the spin the
//! least turn leaves about the target's own direction (`d` with the
//! contact's flip undone), and each slide the copy's snapped
//! coordinate along the target axis the least turn carries — the values
//! that place the copy where the snap stands it (D10
//! `[placement.values]`).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use editor_core::mate::turn::across;
use editor_core::mate::{LeastTurn, slides_left_along};
use editor_core::{Coord, Formula, Orientation, SubgroupFamily, TargetAxis};
use pncad::document::pose::{PoseValue, RoleRead};
use pncad::document::{
    Constraint, ConstraintKind, Dimension, Doc, Evaluation, Frame, OperandSlot, ProfileProgram,
    RecipeNodeId,
};
use pncad::geom_core::Tol;
use pncad::geom_core::linalg::{Mat3, Point3, Vec3};

/// The least turn carrying the unit `from` onto the unit `to`
/// ([`LeastTurn`], the chart's own); a half turn about an axis across
/// `from` where the two are opposite, so a copy standing backwards still
/// snaps.
fn least_turn(from: Vec3<f64>, to: Vec3<f64>) -> Mat3<f64> {
    let turn = LeastTurn::between(from, to);
    if turn.one_plus_c() <= f64::EPSILON {
        let a = across(from).normalize();
        // A half turn about `a`: `2 a aᵀ − I`.
        let col = |e: Vec3<f64>| a * (2.0 * a.dot(e)) - e;
        return Mat3::from_cols(
            col(Vec3::unit_x()),
            col(Vec3::unit_y()),
            col(Vec3::unit_z()),
        );
    }
    turn.matrix()
}

/// A frame's linear part and origin as a matrix and a vector.
fn parts(frame: &Frame) -> (Mat3<f64>, Vec3<f64>) {
    let c = frame.columns;
    let col = |j: usize| Vec3::new(c[j][0], c[j][1], c[j][2]);
    let [x, y, z] = frame.translation;
    (Mat3::from_cols(col(0), col(1), col(2)), Vec3::new(x, y, z))
}

/// A length or an angle as the value that writes it: the constant `0`
/// where it is zero.
fn number(by: f64, dim: Dimension) -> Formula {
    if by == 0.0 {
        Formula::zero(dim)
    } else {
        Formula::literal(by, dim).unwrap_or_else(|_| Formula::zero(dim))
    }
}

/// **The mated pose's direction, a point on it, and its target's**: the
/// copy's in its own coordinates and the target's in the target space,
/// and how the target holds the point — on a plane at an offset, on a
/// line through a foot, or not at all.
struct Seat {
    copy_dir: Vec3<f64>,
    copy_point: Vec3<f64>,
    dir: Vec3<f64>,
    hold: Hold,
}

/// What a mate fixes of the copy's mated point.
enum Hold {
    /// On the plane `dir · p = offset`.
    Plane(f64),
    /// On the line through `foot` along `dir`.
    Line(Vec3<f64>),
    /// Nothing.
    Free,
}

impl Hold {
    /// The point of what holds it nearest `p`, along `dir`.
    fn nearest(&self, dir: Vec3<f64>, p: Vec3<f64>) -> Vec3<f64> {
        match *self {
            Self::Plane(offset) => p - dir * (dir.dot(p) - offset),
            Self::Line(foot) => foot + dir * dir.dot(p - foot),
            Self::Free => p,
        }
    }

    /// How many slides the mate leaves: its residual's
    /// ([`SubgroupFamily::slides`]).
    fn slides(&self) -> usize {
        match self {
            Self::Plane(_) => SubgroupFamily::Planar,
            Self::Line(_) => SubgroupFamily::Cylindrical,
            Self::Free => SubgroupFamily::Parallel,
        }
        .slides()
    }
}

/// The seat of a mate equating `on` with `to`, or `None` for a mate that
/// fixes no one direction.
fn seat_of(on: &PoseValue<f64>, to: &PoseValue<f64>) -> Option<Seat> {
    let origin = Point3::origin();
    Some(match (on, to) {
        (
            PoseValue::Plane {
                normal: o,
                offset: c_on,
            },
            PoseValue::Plane {
                normal: t,
                offset: c_to,
            },
        ) => Seat {
            copy_dir: o.get(),
            copy_point: o.get() * *c_on,
            dir: t.get(),
            hold: Hold::Plane(*c_to),
        },
        (PoseValue::Axis(copy), PoseValue::Axis(target)) => Seat {
            copy_dir: copy.dir().get(),
            copy_point: copy.foot() - origin,
            dir: target.dir().get(),
            hold: Hold::Line(target.foot() - origin),
        },
        (PoseValue::Direction { dir: o }, PoseValue::Direction { dir: t }) => Seat {
            copy_dir: o.get(),
            copy_point: Vec3::zero(),
            dir: t.get(),
            hold: Hold::Free,
        },
        _ => return None,
    })
}

/// `pose`, read in the target space where the copy stands at `(r, t)`,
/// carried back into the copy's own coordinates.
fn unplaced(pose: PoseValue<f64>, r: Mat3<f64>, t: Vec3<f64>, tol: Tol) -> Option<PoseValue<f64>> {
    let back = r.transpose();
    let band = pncad::geom_core::Band::linear(tol).ok()?;
    let unit =
        |v: Vec3<f64>| pncad::geom_core::UnitVec3::new(back * v, "viewer_snap_unplaced", band).ok();
    Some(match pose {
        PoseValue::Plane { normal, offset } => PoseValue::Plane {
            normal: unit(normal.get())?,
            offset: offset - normal.get().dot(t),
        },
        PoseValue::Axis(line) => PoseValue::axis_through(
            Point3::origin() + back * (line.foot() - Point3::origin() - t),
            unit(line.dir().get())?,
        ),
        PoseValue::Direction { dir } => PoseValue::Direction {
            dir: unit(dir.get())?,
        },
        _ => return None,
    })
}

/// **`placement`'s bundle rewritten to stand its copy where the snap
/// puts it from `before`**, where the copy stood before its mate, read
/// off `ev`, that evaluation: the mate fixing a direction carries the
/// orientation nearest the snapped turn, the spin is the turn left about
/// the target's own direction, and the slides are the snapped coordinates along
/// the carried axes the mate leaves. `None` where the copy stood nowhere,
/// or its mate's poses do not evaluate: the bundle is then the door's as
/// it wrote it.
pub(crate) fn snapped(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
    before: Option<Frame>,
    ev: &Evaluation<f64>,
    tol: Tol,
) -> Option<Vec<Constraint<Formula>>> {
    let (r0, t0) = parts(&before?);
    let held = super::held_bundle(doc, placement);
    let (raw, instance) = match doc.node(placement)? {
        pncad::document::Node::Place { constraints, .. } => (constraints.clone(), false),
        pncad::document::Node::InstantiatePart { constraints, .. } => (constraints.clone(), true),
        _ => return None,
    };
    let bind = |id, side, var| {
        editor_core::eval::bound_pose(
            doc,
            ev,
            OperandSlot::Side {
                constraint: id,
                side,
            },
            RoleRead::plain(var),
            tol,
        )
        .ok()
    };
    let (mate_id, seat, to_flipped) = raw.iter().find_map(|c| {
        let m = c.mate()?;
        let on = bind(c.id, pncad::document::MateSide::On, m.on)?;
        // An instance's `on` reads its own output, which stands where the
        // copy stood: carried back into the copy's own coordinates.
        let on = if instance {
            unplaced(on, r0, t0, tol)?
        } else {
            on
        };
        let to = bind(c.id, pncad::document::MateSide::To, m.to)?;
        Some((
            c.id,
            seat_of(&on, &to)?,
            editor_core::mate::flipped(doc, m.to),
        ))
    })?;
    // The snap: the copy's mated direction turned the least way onto
    // `d`, its mated point moved the shortest way onto what holds it.
    let d = seat.dir;
    let turned = least_turn(r0 * seat.copy_dir, d) * r0;
    let stood = r0 * seat.copy_point + t0;
    let t = seat.hold.nearest(d, stood) - turned * seat.copy_point;
    // The orientation nearest the snapped turn, the least turn from
    // where it puts the copy's mated direction, and the spin it leaves.
    let orient = Orientation::nearest(&turned);
    let n = orient.apply(seat.copy_dir);
    let s = least_turn(n, d);
    let across = across(n).normalize();
    let to = s * across;
    let copy = turned * orient.unapply(across);
    // The spin turns about the target's own direction, whichever side
    // spells the contact's sense.
    let about = if to_flipped { -d } else { d };
    let spin = f64::atan2(to.cross(copy).dot(about), to.dot(copy));
    // The slides the mate leaves, by the one rule every gesture writes.
    let slid: Vec<TargetAxis> = slides_left_along(seat.hold.slides(), n).axes().collect();
    let carried = [s.c0, s.c1, s.c2];
    let mut values: Vec<(Coord, Formula)> = slid
        .iter()
        .map(|&a| {
            let by = carried[a.index()].dot(t);
            (Coord::Slide(a), number(by, Dimension::Length))
        })
        .collect();
    values.push((Coord::SPIN, number(spin, Dimension::Angle)));
    let mut bundle: Vec<Constraint<Formula>> = held
        .into_iter()
        .filter(|c| c.value().is_none())
        .map(|mut c| {
            if c.id == mate_id
                && let ConstraintKind::Mate(m) = &mut c.kind
            {
                m.orient = orient;
            }
            c
        })
        .collect();
    bundle.extend((1..).zip(values).map(|(i, (coord, by))| {
        Constraint::of_value(
            pncad::document::ConstraintId::placeholder(i),
            editor_core::Value { coord, by },
        )
    }));
    Some(bundle)
}

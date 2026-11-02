//! **The placement editor's snap** ([`crate::matetool::MateTool::proposal`]):
//! where a draft's copy stands once its mates hold, moved the least way
//! from where it stands now, and the numbers its value rows read there.
//!
//! The mate fixing a direction `d` (a plane, an axis or a direction) is
//! the chart's. Its snap is the least motion that makes it hold: the
//! copy's mated direction, where the copy stands, turned by the least
//! turn onto `d`, and its mated point carried onto the target's plane or
//! line by the shortest move. A point mate beside it then carries the
//! copy's mated point onto the target's. The orientation written is the
//! one nearest the snapped turn; the slides are the ones the mate leaves
//! along the target axes its least turn carries
//! ([`editor_core::mate::slides_left_along`]). With no mate fixing a
//! direction, a point mate moves the copy onto its point and nothing
//! turns it, and with no mate the copy stands where it stands. A frame
//! fixes every freedom and leaves no value.
//!
//! The numbers are read back where the snap stands the copy through the
//! chart that places it ([`editor_core::mate::read_back`]), never a
//! second spelling of it (D10 `[placement.values]`).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use editor_core::mate::turn::across;
use editor_core::mate::{Charted, LeastTurn, read_back, slides_left_along};
use editor_core::{Coord, Formula, Orientation, SlideAxes};
use pncad::document::pose::{PoseValue, RoleRead};
use pncad::document::{
    ConstraintId, Doc, DocEdit, Evaluation, Frame, Mate, MateSide, Node, Operand, OperandSlot,
    Placed, ProfileProgram, Recording,
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

/// **Where a draft's copy stands snapped onto its mates**: the copy's
/// relative pose, the orientation nearest its snapped turn for the mate
/// fixing a direction, the slides that mate leaves, and the chart's two
/// poses its numbers are read through.
pub(crate) struct DraftSnap {
    /// The copy's relative pose, snapped.
    at: pncad::geom_core::linalg::Affine3<f64>,
    /// The orientation nearest the snapped turn, for the chart's mate.
    pub(crate) orient: Option<Orientation>,
    /// The slides the chart's mate leaves, read off the copy's geometry.
    pub(crate) along: Option<SlideAxes>,
    /// The chart's mate, its two poses and its `to`'s sense.
    chart: Option<(PoseValue<f64>, PoseValue<f64>, bool)>,
}

impl DraftSnap {
    /// **The number each of `coords` reads where the snap stands the
    /// copy**, under the chart's orientation `orient` (the one Apply
    /// writes); `None` where the chart does not read it.
    pub(crate) fn read(
        &self,
        orient: Option<Orientation>,
        coords: &[Coord],
        tol: pncad::geom_core::Tol,
    ) -> Vec<Option<f64>> {
        let Ok(band) = pncad::geom_core::Band::linear(tol) else {
            return vec![None; coords.len()];
        };
        let chart = self.chart.as_ref().map(|(on, to, to_flipped)| Charted {
            on,
            to,
            to_flipped: *to_flipped,
            orient: orient.or(self.orient).unwrap_or(Orientation::IDENTITY),
        });
        read_back(chart, &self.at, coords, band)
    }
}

/// **A draft's copy snapped onto `mates`**, read off `ev`, the landed
/// evaluation of `doc`: the copy where it stands now (an Edit's placement
/// where it is solved; a Create's body where it stands, the identity),
/// moved the least way that makes the mates hold (module docs). `None`
/// where the copy stands nowhere, or the mates do not write as a bundle:
/// the verdict then says why.
pub(crate) fn draft_snap(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    door: &crate::matetool::PlaceDoor,
    target: Option<pncad::document::VarId>,
    mates: &[Mate<Formula>],
    tol: pncad::geom_core::Tol,
) -> Option<DraftSnap> {
    use crate::matetool::PlaceDoor;
    let bundle = Placed::of(doc, mates.to_vec(), Vec::new()).bundle();
    let mut run = Recording::start(doc, tol);
    let (placement, before) = match door {
        PlaceDoor::Edit { placement, .. } => {
            // A mate reading another space than the copy's is applied
            // with the copy moved there by name (`SessionOp::SetBundle`'s
            // `into_target_space`): a bundle holding a mate is first
            // stood at the identity, and one of values alone stands on
            // the new space's axes as it stood on the old's.
            let into = mates
                .first()
                .and_then(|m| doc.operand_space(&m.to))
                .filter(|&space| Some(space) != doc.node_space(*placement));
            let mated = super::held_bundle(doc, *placement)
                .iter()
                .any(|c| c.mate().is_some());
            if let Some(space) = into {
                super::into_space(&mut run, *placement, space).ok()?;
            }
            let before = if into.is_some() && mated {
                Frame::IDENTITY
            } else {
                pncad::document::SolvedPoses::of(doc, ev).relative(*placement)?
            };
            run.apply(DocEdit::SetConstraints {
                placement: *placement,
                constraints: bundle,
                fresh: Vec::new(),
            })
            .ok()?;
            (*placement, before)
        }
        PlaceDoor::Create { body } => {
            let placement = run
                .insert(Node::place(
                    pncad::document::BodyRead::plain(body.clone()),
                    Operand::Var(target?),
                    bundle,
                ))
                .ok()?;
            (placement, Frame::IDENTITY)
        }
    };
    let doc = run.doc();
    let (r0, t0) = parts(&before);
    let (raw, instance) = match doc.node(placement)? {
        Node::Place { constraints, .. } => (constraints.clone(), false),
        Node::InstantiatePart { constraints, .. } => (constraints.clone(), true),
        _ => return None,
    };
    let bind = |id: ConstraintId, side, var| {
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
    // Each mate's two poses, `on` in the copy's own coordinates: an
    // instance's `on` reads its own output, which stands where the copy
    // stood, so it is carried back.
    let read: Vec<(ConstraintId, PoseValue<f64>, PoseValue<f64>, bool)> = raw
        .iter()
        .filter_map(|c| {
            let m = c.mate()?;
            let on = bind(c.id, MateSide::On, m.on)?;
            let on = if instance {
                unplaced(on, r0, t0, tol)?
            } else {
                on
            };
            let to = bind(c.id, MateSide::To, m.to)?;
            Some((c.id, on, to, editor_core::mate::flipped(doc, m.to)))
        })
        .collect();
    let point = read.iter().find_map(|(_, on, to, _)| match (on, to) {
        (PoseValue::Point { position: o }, PoseValue::Point { position: t }) => Some((
            *o - pncad::geom_core::linalg::Point3::origin(),
            *t - pncad::geom_core::linalg::Point3::origin(),
        )),
        _ => None,
    });
    let affine = |r: Mat3<f64>, t: Vec3<f64>| pncad::geom_core::linalg::Affine3::from_parts(r, t);
    let Some((on, to, to_flipped, seat)) = read
        .iter()
        .find_map(|(_, on, to, flipped)| Some((on, to, *flipped, seat_of(on, to)?)))
    else {
        // No mate fixes a direction: a point mate carries the copy's
        // point onto the target's, and nothing turns it.
        let t = match point {
            Some((o, p)) => p - r0 * o,
            None => t0,
        };
        return Some(DraftSnap {
            at: affine(r0, t),
            orient: None,
            along: None,
            chart: None,
        });
    };
    let d = seat.dir;
    let turned = least_turn(r0 * seat.copy_dir, d) * r0;
    let t = match point {
        Some((o, p)) => p - turned * o,
        None => {
            let stood = r0 * seat.copy_point + t0;
            seat.hold.nearest(d, stood) - turned * seat.copy_point
        }
    };
    let orient = Orientation::nearest(&turned);
    let n = orient.apply(seat.copy_dir);
    let slides = Placed::freedoms(doc, mates, None)
        .iter()
        .filter(|c| matches!(c, Coord::Slide(_)))
        .count();
    Some(DraftSnap {
        at: affine(turned, t),
        orient: Some(orient),
        along: Some(slides_left_along(slides, n)),
        chart: Some((on.clone(), to.clone(), to_flipped)),
    })
}

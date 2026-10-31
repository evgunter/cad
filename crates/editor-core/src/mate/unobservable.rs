//! **The unobservable-value report** (`docs/INTENT-STAGE3-SPEC.md` §1,
//! `docs/DESIGN.md` [placement.placement]): a number written into the
//! document that moves nothing at these values. A body's symmetry pins
//! nothing and a move is admitted by kind alone (FORK-S3O), so neither
//! is refused; this report names them instead.
//!
//! Two shapes are named, decided at the evaluation's values:
//!
//! - **a pose move** whose pose its move leaves where it is — a plane
//!   slid along a direction lying in it, an axis spun about itself —
//!   that is, a move inside the pose's own symmetry;
//! - **a spin value** about a line its copy is round about — a round
//!   pin's spin — where every face of the placed copy is carried onto
//!   itself by the turn (a plane across the line, a cylinder, cone or
//!   torus about it, a sphere centred on it). A slide never is: a
//!   bounded body is no translation's.
//!
//! The report is advisory: a verdict the band cannot decide, or a pose
//! or a copy that did not evaluate, names nothing.

use geom_core::Tol;
use geom_core::linalg::{Point3, UnitVec3};
use geom_core::predicate::Band;

use super::MateRef;
use super::PoseSymmetry;
use super::coset::{Arm, directions_parallel, on_the_line, slides_along, turn_line};
use crate::eval::{Evaluation, NodeResult, ValuePayload};
use crate::node::RecipeNodeId;
use crate::pose::{PoseDef, PoseValue};
use crate::var::VarDef;
use crate::{ProfileDoc, VarId};

/// **A number that moves nothing at these values.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unobservable {
    /// A pose variable defined by a slide or a spin whose pose the move
    /// carries onto itself: its number (`by`) moves nothing.
    PoseMove {
        /// The pose variable the move defines.
        pose: VarId,
        /// The move's number.
        by: VarId,
    },
    /// A spin value about a line its placed copy is round about.
    Value {
        /// The value.
        value: MateRef,
    },
}

/// **Every number in `doc` that `ev` (its evaluation) shows moves
/// nothing**: the pose moves in variable order, then the values in
/// placement and bundle order.
#[must_use]
pub fn unobservable(doc: &ProfileDoc, ev: &Evaluation<f64>, tol: Tol) -> Vec<Unobservable> {
    let Ok(band) = Band::linear(tol) else {
        return Vec::new();
    };
    let mut out: Vec<Unobservable> = doc
        .vars()
        .iter()
        .filter_map(|(&var, held)| {
            let VarDef::Pose(def) = held.def() else {
                return None;
            };
            pose_move(doc, ev, def, band, tol).then(|| {
                let (PoseDef::Slide { by, .. } | PoseDef::Spin { by, .. }) = def else {
                    unreachable!("only a move is decided")
                };
                Unobservable::PoseMove { pose: var, by: *by }
            })
        })
        .collect();
    for placement in doc.placements() {
        let Some(bundle) = doc.node(placement).and_then(super::solve::bundle_of) else {
            continue;
        };
        for c in bundle {
            let Some(value) = c.value() else { continue };
            if value.coord != super::Coord::SPIN {
                continue;
            }
            // The spin about the target's own direction: the one mate
            // fixing a direction turns it, and a bundle with none spins a
            // chain.
            if bundle
                .iter()
                .filter_map(super::Constraint::mate)
                .any(|mate| spin_moves_nothing(doc, ev, placement, mate, band, tol))
            {
                out.push(Unobservable::Value {
                    value: MateRef::new(placement, c.id),
                });
            }
        }
    }
    out
}

/// The lever a verdict about a pose is decided over: the farthest a
/// point it names sits from the origin, at least the band's own zero.
fn arm_over(points: &[Point3<f64>], band: Band) -> Option<Arm> {
    let datum = points
        .iter()
        .map(|p| (*p - Point3::origin()).norm())
        .fold(0.0_f64, f64::max);
    Arm::of(0.0, datum.max(1.0)).ok()?.decides_over(band).ok()
}

/// A pose's point, for its lever.
fn point_of(pose: &PoseValue<f64>) -> Point3<f64> {
    match *pose {
        PoseValue::Point { position } => position,
        PoseValue::Direction { .. } => Point3::origin(),
        PoseValue::Axis(line) => line.foot(),
        PoseValue::Plane { normal, offset } => PoseValue::plane_foot(normal, offset),
        PoseValue::Frame(f) => f.origin(),
    }
}

/// Whether a slide or a spin definition carries its pose onto itself.
fn pose_move(doc: &ProfileDoc, ev: &Evaluation<f64>, def: &PoseDef, band: Band, tol: Tol) -> bool {
    let read = |r: &crate::pose::RoleRead<VarId>| crate::eval::read_pose_in(doc, ev, *r, tol).ok();
    match def {
        PoseDef::Slide { pose, along, .. } => {
            let (Some(pose), Some(along)) = (read(pose), read(along)) else {
                return false;
            };
            let Some(dir) = along.heading() else {
                return false;
            };
            // A plane slid moves its offset by `by · (along · normal)`
            // alone, so no point of it is levered over.
            let points = match pose {
                PoseValue::Plane { .. } => vec![],
                _ => vec![point_of(&pose)],
            };
            let Some(arm) = arm_over(&points, band) else {
                return false;
            };
            slides_along(pose.symmetry(), dir, band, arm).unwrap_or(false)
        }
        PoseDef::Spin { pose, about, .. } => {
            let (Some(pose), Some(PoseValue::Axis(about))) = (read(pose), read(about)) else {
                return false;
            };
            let (foot, dir) = (about.foot(), about.dir());
            let Some(arm) = arm_over(&[point_of(&pose), foot], band) else {
                return false;
            };
            match turn_line(pose.symmetry(), dir, band, arm) {
                Ok(Some(None)) => true,
                Ok(Some(Some(q))) => on_the_line(q, foot, dir, band).unwrap_or(false),
                Ok(None) | Err(_) => false,
            }
        }
        _ => false,
    }
}

/// Whether a spin value on `mate` turns `placement`'s placed copy onto
/// itself: the line an axis mate's spin turns about is its target axis;
/// a plane's or a direction's, the line through the copy's origin along
/// the mated direction.
fn spin_moves_nothing(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    placement: RecipeNodeId,
    mate: &super::Mate,
    band: Band,
    tol: Tol,
) -> bool {
    let Some(NodeResult::Ok(value)) = ev.result(placement) else {
        return false;
    };
    let to = crate::eval::read_pose_in(doc, ev, crate::pose::RoleRead::plain(mate.to), tol);
    let copy_origin = value.motion.map_or_else(Point3::origin, |m| {
        Point3::origin() + m.to_affine().translation
    });
    let (point, dir) = match to {
        Ok(PoseValue::Axis(line)) => (line.foot(), line.dir()),
        Ok(PoseValue::Plane { normal: dir, .. } | PoseValue::Direction { dir }) => {
            (copy_origin, dir)
        }
        _ => return false,
    };
    let bodies: Vec<&crate::eval::Framed<f64>> = match &value.payload {
        ValuePayload::Body(b) => vec![b],
        _ => return false,
    };
    // Each body's reach in its space, from the space's origin. A body
    // whose space's origin its frame cannot place leaves the arm
    // unknown: the turn is not shown unobservable, rather than measured
    // over the other bodies alone. (A body with no bounded reach adds
    // none, as before E.)
    let mut reach: Option<f64> = None;
    for b in &bodies {
        let Ok(origin) = b.space_origin() else {
            return false;
        };
        let Ok(r) = super::part_reach_from(b.body(), origin) else {
            continue;
        };
        reach = Some(reach.map_or(r, |h: f64| h.max(r)));
    }
    let Some(arm) = reach.and_then(|reach| Arm::of(reach, (point - Point3::origin()).norm()).ok())
    else {
        return false;
    };
    // Each body's turn is read in its own frame: the line, stated in
    // the space, read into it.
    bodies.iter().all(|body| {
        let (point, dir) = match body.frame().map().map(|map| map.inverse()) {
            None => (point, dir),
            Some(Ok(back)) => (back.transform_point(point), back.turn(dir)),
            Some(Err(_)) => return false,
        };
        round_about(body.body(), point, dir, band, arm)
    })
}

/// Whether every face of `body` is carried onto itself by a turn about
/// the line through `point` along `dir`.
fn round_about(
    body: &topo::Body<f64>,
    point: Point3<f64>,
    dir: UnitVec3<f64>,
    band: Band,
    arm: Arm,
) -> bool {
    use geom::SurfaceKind as K;
    body.faces().all(|(face, _)| {
        let (Ok(kind), Ok(at)) = (
            topo::readback::face_carrier_kind(body, face),
            topo::readback::face_pose(body, face),
        ) else {
            return false;
        };
        let axis = UnitVec3::new(at.axis, topo::query::POSE_UNIT_NORM, band);
        let along = |a: UnitVec3<f64>| directions_parallel(a, dir, band, arm).unwrap_or(false);
        let on = |p: Point3<f64>| on_the_line(p, point, dir, band).unwrap_or(false);
        match kind {
            K::Plane => axis.is_ok_and(along),
            K::Sphere => on(at.origin),
            K::Cylinder | K::Cone | K::Torus => axis.is_ok_and(along) && on(at.origin),
            _ => false,
        }
    })
}

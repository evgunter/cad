//! **The mated parts' own extent** — the two body terms of the lever a
//! mate's angular decisions turn on (A11's one geometric read).
//!
//! A parallelism verdict is consumed as "the separation is constant
//! across the parts", so its margin is `sin θ · L` where `L` is the
//! length the two parts span together from the datum. The solve has
//! no body in hand — it works over the recipe — so the extent reaches
//! it through [`MateReach`], one method answering an instance's part
//! reach, implemented by the evaluation over its own part cache
//! (`eval::mate_reach` is the public door; the evaluation's own run
//! reads the cache it already built).
//!
//! # What "reach" is
//!
//! [`body_reach`] answers `R`, an UPPER bound on the distance from the
//! part-local origin to any point of the body, as an `f64`
//! ([`Bounds::hi`] on an interval scalar). The maximum over the
//! body's faces of each face's own bound, and the bound is stated per
//! surface kind rather than assumed:
//!
//! * a **plane** or **cylinder** patch is bounded by its rim: the
//!   distance from a point is convex along every line in a plane,
//!   and strictly convex along a cylinder's rulings, so no interior
//!   point of a patch is a local maximum and the maximum over a
//!   compact patch lies on its boundary. The boundary is walked by
//!   the measure site's own [`reach_of`] — line ends, the containing
//!   circle or ellipse of a conic, a NURBS control hull — which is
//!   an upper bound on the rim, hence on the patch.
//! * a **cone** patch is bounded by its rim OR its apex: distance is
//!   strictly convex along every ruling, so an interior maximum can
//!   only sit where a ruling degenerates, which is the apex (with
//!   the origin on the axis above a cone tip, the tip is the
//!   farthest point of the tip face and no rim point reaches it).
//! * a **sphere** is bounded by the whole carrier, `‖centre‖ + r`;
//!   a **torus** by `‖centre‖ + R + r`. A hemisphere's rim is its
//!   equator, so the rim argument fails for both and the carrier
//!   bound is the honest one.
//! * a **NURBS** or **approximating** patch is bounded by its control
//!   hull (positive weights; the assumption
//!   `geom::surfaces::boxes::nurbs_surface_aabb` states), the fit
//!   being the geometry an approximating surface evaluates as.
//!
//! Over-refusal is the safe direction, so every bound is an upper
//! one and none is dropped: a face whose bound cannot be stated — a
//! boundary that does not walk, an empty control net, a poisoned
//! coordinate — refuses typed through [`Unbounded`], never a guess.

use geom::Surface;
use geom_core::{Bounds, Decide, Point3};
use topo::Body;
use topo::entity::FaceKey;

use super::LeverRefusal;
use crate::eval::measure::reach_of;
use crate::ident::DocRef;
use crate::node::RecipeNodeId;

/// **The reach of a mated instance's part**, asked by the solve at the
/// site it forms a lever (`mate/solve.rs`'s per-pair fold).
///
/// One method, one number: an upper bound on the distance from the
/// instance's part-local origin to any point of its part's body, the
/// `R` in the lever `(R_a + ‖a.origin‖) + (R_b + ‖b.origin‖) + Σ|authored
/// lengths|`. The frames a mate authors are in the part's own
/// coordinates, so `R + ‖origin‖` bounds the part's reach from the
/// mate frame by the triangle inequality, and a pattern copy or a
/// transform on the member's chain moves the part rigidly and changes
/// no reach.
///
/// Answered lazily: a part no mate names is never asked for, and a
/// part that is asked for is evaluated exactly once per evaluation
/// (the evaluation's implementation reads its own part cache, which
/// the instantiate node then hits).
pub trait MateReach {
    /// The reach `R` of `instance`'s part from its own origin.
    ///
    /// # Errors
    ///
    /// [`LeverRefusal`]: the node is not a live instantiate node, its
    /// part does not resolve (carrying the resolver's own fault), or
    /// its body has a face whose reach cannot be bounded.
    fn reach(&self, instance: RecipeNodeId) -> Result<f64, LeverRefusal>;
}

/// **The reach of a door with no resolver in hand**: every part is
/// [`crate::eval::PartFault::NoResolver`], typed, so a solve through
/// it levers no mate and refuses each one in the resolver's own
/// voice. What `eval::mate_reach` answers over options carrying no
/// resolver, as a value for a door that has no options at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NoResolver;

impl MateReach for NoResolver {
    fn reach(&self, instance: RecipeNodeId) -> Result<f64, LeverRefusal> {
        Err(LeverRefusal::PartUnresolved {
            instance,
            fault: crate::eval::PartFault::NoResolver,
        })
    }
}

/// Why a body's reach could not be bounded: the face and its surface
/// kind. [`Unbounded::into_lever`] names the instance and the part
/// once the caller knows them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unbounded {
    /// The face whose reach has no bound this module can state.
    pub face: FaceKey,
    /// Its surface kind, by name.
    pub kind: &'static str,
}

impl Unbounded {
    /// The lever refusal this is, on `instance`'s part `part`.
    pub fn into_lever(self, instance: RecipeNodeId, part: DocRef) -> LeverRefusal {
        LeverRefusal::FaceUnbounded {
            instance,
            part,
            face: self.face,
            kind: self.kind,
        }
    }
}

/// **An upper bound on the distance from the body's own origin to any
/// point of it** — the reach `R` of a part (module docs).
///
/// `Ok(None)` for a body with no faces, which has no extent to lever
/// over; the caller says what that means for it (the solve refuses,
/// because a lever formed over nothing is vacuous rather than wrong).
///
/// # Errors
///
/// [`Unbounded`], naming the first face whose reach cannot be stated
/// — in the body's own face order, so the answer is deterministic.
pub fn body_reach<T: Decide + Bounds>(body: &Body<T>) -> Result<Option<f64>, Unbounded> {
    let origin = Point3::<T>::origin();
    let mut reach: Option<f64> = None;
    for (key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            return Err(Unbounded {
                face: key,
                kind: "unknown",
            });
        };
        let kind = surface_kind(surface);
        let refuse = Unbounded { face: key, kind };
        let bound = face_reach(body, key, surface, origin).ok_or(refuse)?;
        // The upper end of the bracket on an interval scalar, the
        // value itself on `f64`. Poison (NaN) and an infinite bound
        // are not bounds: refuse rather than lever over them.
        let hi = bound.hi();
        if !hi.is_finite() {
            return Err(refuse);
        }
        reach = Some(reach.map_or(hi, |r| r.max(hi)));
    }
    Ok(reach)
}

/// One face's bound from `origin`, per surface kind (module docs).
/// `None` where the bound cannot be stated.
fn face_reach<T: Decide + Bounds>(
    body: &Body<T>,
    key: FaceKey,
    surface: &Surface<T>,
    origin: Point3<T>,
) -> Option<T> {
    let from = |p: Point3<T>| (p - origin).norm();
    match surface {
        Surface::Plane { .. } | Surface::Cylinder { .. } => reach_of(body, key, origin),
        Surface::Cone { apex, .. } => Some(reach_of(body, key, origin)?.max(from(*apex))),
        Surface::Sphere { center, radius, .. } => Some(from(*center) + *radius),
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => Some(from(*center) + *major_radius + *minor_radius),
        Surface::Nurbs(_) | Surface::Approx(_) => surface
            .spline_chart()?
            .control()
            .iter()
            .map(|p| from(*p))
            .reduce(|a, b| a.max(b)),
    }
}

/// The surface kind's name, for a refusal.
fn surface_kind<T: geom_core::Real>(surface: &Surface<T>) -> &'static str {
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs(_) => "nurbs",
        Surface::Approx(_) => "approx",
    }
}

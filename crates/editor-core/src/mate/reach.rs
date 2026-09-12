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
//! part-local origin to any point of the body, at the body's own
//! scalar `T` — the evaluation reads it back as an `f64` upper bound
//! where it holds the ratified compound (`eval`'s `EvalScalar`, the
//! bracket's `hi`). The maximum over the body's faces of each face's
//! own bound, and the bound is stated per surface kind rather than
//! assumed:
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
//! boundary that does not walk, an empty or placeholder (all-poison)
//! control net — refuses typed through [`Unbounded`], never a guess;
//! a bound that reads back non-finite refuses where it is read.

use geom::Surface;
use geom_core::{Decide, Point3};
use topo::Body;
use topo::entity::FaceKey;

use crate::eval::measure::reach_of;
use crate::ident::DocRef;

/// **The reach of a mated part**, asked by the solve at the site it
/// forms a lever (`mate/solve.rs`'s per-pair fold).
///
/// One method, one number: an upper bound on the distance from the
/// part's own origin to any point of its body, the `R` in the lever
/// `(R_a + ‖a.origin‖) + (R_b + ‖b.origin‖) + Σ|authored lengths|`.
/// The frames a mate authors are in the part's own coordinates, so
/// `R + ‖origin‖` bounds the part's reach from the mate frame by the
/// triangle inequality, and a pattern copy or a transform on the
/// member's chain moves the part rigidly and changes no reach.
///
/// Keyed by the PART (its reference), not by the instance: a reach is
/// a fact about a document's body, so one value serves every instance
/// of that part and every document the caller threads through it (the
/// edit door applies a group of edits to a succession of documents
/// against one reach). The solve reads the instance's reference off
/// its own document and names the instance when it wraps a refusal
/// ([`super::LeverRefusal`]).
///
/// Answered lazily: a part no mate names is never asked for, and a
/// part that is asked for is evaluated exactly once per evaluation
/// (the evaluation's implementation reads its own part cache, which
/// the instantiate node then hits).
pub trait MateReach {
    /// The reach `R` of the part `part` names, from its own origin.
    ///
    /// # Errors
    ///
    /// [`ReachRefusal`]: the part does not resolve (carrying the
    /// resolver's own fault), its body has a face whose reach cannot
    /// be bounded, or its body has no faces.
    fn reach(&self, part: &DocRef) -> Result<f64, ReachRefusal>;
}

/// Why a part's reach is not in hand ([`MateReach::reach`]). Named
/// against the part alone; the solve adds the instance it was asking
/// for ([`super::LeverRefusal::of`]).
#[derive(Debug, Clone, PartialEq)]
pub enum ReachRefusal {
    /// The part does not resolve: the evaluation layer's own typed
    /// cause, unaltered.
    PartUnresolved {
        /// The resolver's fault.
        fault: crate::eval::PartFault,
    },
    /// The body has a face whose reach this module cannot bound
    /// ([`body_reach`]'s per-kind table).
    FaceUnbounded {
        /// The face, in the part body's own arena.
        face: FaceKey,
        /// Its surface kind, by name.
        kind: &'static str,
    },
    /// The body has no faces, so it has no extent to lever over.
    NoExtent,
    /// The body's reach read back non-finite — a poisoned coordinate
    /// somewhere in the walk — so no bound can be stated.
    NoFiniteBound,
}

/// **The refusing reach** — the reach of a door with no resolver in
/// hand: every part is [`crate::eval::PartFault::NoResolver`], typed,
/// so a solve through it levers no mate and refuses each one in the
/// resolver's own voice, and an edit whose maintenance needs a solved
/// frame refuses at the door. What `eval::mate_reach` answers over
/// options carrying no resolver, as a value for a door that has no
/// options at all — and the honest reach for an edit that cannot
/// move a cluster's gauge, which never asks it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RefusingReach;

impl MateReach for RefusingReach {
    fn reach(&self, _part: &DocRef) -> Result<f64, ReachRefusal> {
        Err(ReachRefusal::PartUnresolved {
            fault: crate::eval::PartFault::NoResolver,
        })
    }
}

/// Why a body's reach could not be bounded: the face and its surface
/// kind. [`ReachRefusal::FaceUnbounded`] is this, on a part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unbounded {
    /// The face whose reach has no bound this module can state.
    pub face: FaceKey,
    /// Its surface kind, by name.
    pub kind: &'static str,
}

impl From<Unbounded> for ReachRefusal {
    fn from(u: Unbounded) -> Self {
        ReachRefusal::FaceUnbounded {
            face: u.face,
            kind: u.kind,
        }
    }
}

/// **A part's reach as the solve reads it**: [`body_reach`] with a
/// faceless body refused ([`ReachRefusal::NoExtent`]) — the one
/// answer every [`MateReach`] over an evaluated body gives.
///
/// # Errors
///
/// [`ReachRefusal::FaceUnbounded`], [`ReachRefusal::NoExtent`].
pub fn part_reach<T: Decide>(body: &Body<T>) -> Result<T, ReachRefusal> {
    body_reach(body)?.ok_or(ReachRefusal::NoExtent)
}

/// **An upper bound on the distance from the body's own origin to any
/// point of it** — the reach `R` of a part (module docs), at the
/// body's own scalar.
///
/// `Ok(None)` for a body with no faces, which has no extent to lever
/// over; the caller says what that means for it (the solve refuses,
/// because a lever formed over nothing is vacuous rather than wrong).
///
/// # Errors
///
/// [`Unbounded`], naming the first face whose reach cannot be stated
/// — in the body's own face order, so the answer is deterministic.
pub fn body_reach<T: Decide>(body: &Body<T>) -> Result<Option<T>, Unbounded> {
    let origin = Point3::<T>::origin();
    let mut reach: Option<T> = None;
    for (key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            return Err(Unbounded {
                face: key,
                kind: "unknown",
            });
        };
        let kind = surface_kind(surface);
        let bound = face_reach(body, key, surface, origin).ok_or(Unbounded { face: key, kind })?;
        reach = Some(reach.map_or(bound, |r| r.max(bound)));
    }
    Ok(reach)
}

/// One face's bound from `origin`, per surface kind (module docs).
/// `None` where the bound cannot be stated.
fn face_reach<T: Decide>(
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
        // The placeholder net is all-poison by construction — a
        // surface representable but not described — and describes no
        // locus to bound.
        Surface::Nurbs(_) | Surface::Approx(_) => {
            let chart = surface.spline_chart()?;
            if chart.is_placeholder() {
                return None;
            }
            chart
                .control()
                .iter()
                .map(|p| from(*p))
                .reduce(|a, b| a.max(b))
        }
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

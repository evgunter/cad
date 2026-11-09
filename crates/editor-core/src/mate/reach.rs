//! **A mated part's extent, as a lever reads it** — the part's REACH,
//! the body term of the lever a mate's angular decisions turn on.
//!
//! A parallelism verdict is consumed as "the separation is constant
//! across the parts", so its margin is `sin θ · L` where `L` is the
//! length the two parts span together from the datum. [`body_reach`]
//! answers the body term, and [`part_reach`] the solve's reading of it
//! over an evaluated body (the fold reads the copy's own geometry), in
//! the part's own coordinates.
//!
//! # What "reach" is
//!
//! [`body_reach`] answers `R`, an UPPER bound on the distance from the
//! part-local origin to any point of the body, at the body's own
//! scalar `T` — the evaluation reads it back as an `f64` upper bound
//! where it holds the ratified compound (`eval`'s `EvalScalar`, the
//! bracket's `hi`). It is the maximum over the body's faces of each
//! face's farthest distance from the origin as its cover reads it
//! (`topo`'s reading of the region a face can occupy,
//! [`topo::face_cover`]), the measure site's own [`reach_of`].
//!
//! Over-refusal is the safe direction, so every bound is an upper
//! one and none is dropped: a face with no cover refuses typed
//! ([`ReachRefusal::FaceUnbounded`]), never a guess; a face whose
//! surface key does not resolve is a malformed body and refuses in
//! that voice ([`ReachRefusal::MalformedBody`]); a bound that reads
//! back non-finite refuses where it is read.

use geom_core::{Band, Decide, Point3};
use topo::Body;
use topo::entity::FaceKey;

use crate::eval::measure::reach_of;

/// Why a part's reach is not in hand. Named
/// against the part alone; the solve carries it beside the instance
/// it was asking for ([`super::LeverRefusal::Reach`]).
///
/// Displayed as a predicate of the part, after the subject the
/// carrier names (`instance 6's part …`): each sentence ends on its
/// recourse, on the kernel-defect ending where only a malformed body
/// reaches the arm, or on "no way through" where the kernel has no
/// bound to state.
#[derive(Debug, Clone, PartialEq)]
pub enum ReachRefusal {
    /// The part does not resolve: the evaluation layer's own typed
    /// cause, unaltered.
    PartUnresolved {
        /// The resolver's fault.
        fault: crate::eval::PartFault,
    },
    /// The body has a face with no cover ([`topo::face_cover`]), so its
    /// reach cannot be bounded.
    FaceUnbounded {
        /// The face, in the part body's own arena.
        face: FaceKey,
        /// Its surface kind.
        kind: SurfaceKind,
    },
    /// The body has a face whose surface key resolves to no surface
    /// in its own arena: not a face this module cannot bound but a
    /// body that is not well formed, refused in that voice.
    MalformedBody {
        /// The face whose surface is missing.
        face: FaceKey,
    },
    /// The body has no faces, so it has no extent to lever over.
    NoExtent,
    /// The body's reach read back non-finite — a coordinate out of the
    /// format's range somewhere in the walk — so no bound can be
    /// stated.
    NoFiniteBound,
}

impl core::fmt::Display for ReachRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PartUnresolved { fault } => write!(
                f,
                "is not in hand, so the mate has no extent to lever a verdict over: {fault}"
            ),
            Self::FaceUnbounded { kind, .. } => write!(
                f,
                "has {} {} face whose reach from the part's origin cannot be bounded, so no \
                 bound on the part's extent can be stated. There is no way through for a \
                 mate on this part",
                crate::sentence::article(kind.name()),
                kind.name()
            ),
            Self::MalformedBody { .. } => write!(
                f,
                "has a face whose surface resolves to nothing, so the body is not well \
                 formed. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::NoExtent => f.write_str(
                "has no faces, so it has no extent to lever a verdict over. Recourse: mate a \
                 part that has a body",
            ),
            Self::NoFiniteBound => write!(
                f,
                "has a reach that reads back non-finite, so no bound on its extent can be \
                 stated. Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
        }
    }
}

/// The kind a refusal names a face's surface by — `geom`'s closed
/// mirror of [`geom::Surface`], with its one name table
/// ([`SurfaceKind::name`]), so no second string table stands beside
/// it and a caller branches on the value.
pub use geom::SurfaceKind;

/// **A part's reach as the solve reads it**: [`body_reach`] with a
/// faceless body refused ([`ReachRefusal::NoExtent`]) — the one
/// answer the fold reads over an evaluated body. Each face's cover is
/// read under `band`.
///
/// # Errors
///
/// [`ReachRefusal::FaceUnbounded`], [`ReachRefusal::MalformedBody`],
/// [`ReachRefusal::NoExtent`].
pub fn part_reach<T: Decide>(body: &Body<T>, band: Band) -> Result<T, ReachRefusal> {
    part_reach_from(body, Point3::origin(), band)
}

/// [`part_reach`] about `about` rather than the body's own origin: a
/// framed body's reach in its space, `about` being its space's origin
/// in the body's coordinates.
///
/// # Errors
///
/// As [`part_reach`].
pub fn part_reach_from<T: Decide>(
    body: &Body<T>,
    about: Point3<T>,
    band: Band,
) -> Result<T, ReachRefusal> {
    reach_from(body, about, band)?.ok_or(ReachRefusal::NoExtent)
}

/// **An upper bound on the distance from the body's own origin to any
/// point of it** — the reach `R` of a part (module docs), at the
/// body's own scalar, each face's cover read under `band`.
///
/// `Ok(None)` for a body with no faces, which has no extent to lever
/// over; the caller says what that means for it (the solve refuses,
/// because a lever formed over nothing is vacuous rather than wrong).
///
/// # Errors
///
/// [`ReachRefusal::FaceUnbounded`], naming the first face with no
/// cover — in the body's own face order, so the answer is
/// deterministic; [`ReachRefusal::MalformedBody`] for a face whose
/// surface key resolves to nothing, which is a fault of the body and
/// not of this table.
pub fn body_reach<T: Decide>(body: &Body<T>, band: Band) -> Result<Option<T>, ReachRefusal> {
    reach_from(body, Point3::origin(), band)
}

/// [`body_reach`] about `origin`.
fn reach_from<T: Decide>(
    body: &Body<T>,
    origin: Point3<T>,
    band: Band,
) -> Result<Option<T>, ReachRefusal> {
    let mut reach: Option<T> = None;
    for (key, face) in body.faces() {
        let Some(surface) = body.get_surface(face.surface) else {
            return Err(ReachRefusal::MalformedBody { face: key });
        };
        let bound = reach_of(body, key, origin, band).ok_or(ReachRefusal::FaceUnbounded {
            face: key,
            kind: surface.kind(),
        })?;
        reach = Some(reach.map_or(bound, |r| r.max(bound)));
    }
    Ok(reach)
}

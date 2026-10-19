//! **What the region readings share**: the pieces [`super::sphere_region`]
//! and [`super::torus_region`] both read a trimmed curved face with —
//! the one refusal they answer ([`RegionRefusal`]), the targets their aimed
//! paths take ([`TARGET_SHARES`]), the boundary reading of a circle edge
//! on its own carrier ([`circle_edge_hit`]), a root against its edge's span
//! ([`span`]), and the verdict read at a path's closest crossing
//! ([`side_at_closest`]). Each reader keeps its own rows, its paths and
//! its crossing arithmetic.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use super::solid_contain::PointInSolidError;
use crate::entity::{FaceKey, LoopKey};
use crate::ray_walk::{Crossings, RayFault};
use crate::splitting::containment::{ConicArc, ConicArcError, ConicHit, ConicRows};
use crate::splitting::{PointInLoopError, Uncrossable};
use crate::validate::decide;

/// Where along each edge an aimed path's target lies, as shares of its
/// span.
pub(crate) const TARGET_SHARES: [f64; 3] = [0.5, 0.25, 0.75];

/// Why a region reading gave no answer. Each door that reads a region
/// names the face it read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RegionRefusal {
    /// The point is in the band of a boundary edge; or no path decided,
    /// and the first one set aside read in band.
    Escalated(Indeterminate),
    /// No path settled ([`crate::ray_walk::NoRaySettled`]): each grazed or
    /// gave nothing to read. `outer` is the face's outer loop, the loop
    /// the solid door names as the region walk's.
    RayExhausted {
        /// The face's outer loop.
        outer: LoopKey,
    },
    /// A boundary arc winds definitely past a whole turn, which bounds
    /// no region.
    WoundPastPeriod,
    /// The boundary reading of an edge on its own carrier ran out of depth
    /// or budget before it placed the point: no reading in the band, and
    /// no path starts from a point it has not placed.
    Unreadable(Uncrossable),
}

impl RegionRefusal {
    /// The refusal as the point-in-solid door states it, for a point on
    /// `face`'s carrier: a region walk no path of which settled is that
    /// face's loop walk's, as a planar face's is
    /// ([`PointInSolidError::Loop`]) — not the solid's own sweep, whose
    /// exhaustion says the point is off the solid's boundary.
    pub(crate) fn of_face(self, face: FaceKey) -> PointInSolidError {
        match self {
            Self::Escalated(diag) => PointInSolidError::Escalated { face, diag },
            Self::RayExhausted { outer } => {
                PointInSolidError::Loop(PointInLoopError::RayExhausted { r#loop: outer })
            }
            Self::WoundPastPeriod => PointInSolidError::CorruptFace { face },
            Self::Unreadable(cause) => PointInSolidError::EdgeCarrierUnsupported { face, cause },
        }
    }
}

/// **Whether `p` is on the circle arc `circle` over `(t0, t1)`**, read on
/// the arc's own carrier ([`ConicArc::hit`]): on the arc or within the
/// band of one of its ends is `true`; in the band of it refuses. `outer`
/// is named if the reading cannot place `p`.
///
/// # Errors
///
/// [`RegionRefusal::Escalated`] in band; [`RegionRefusal::WoundPastPeriod`]
/// for an arc wound past a turn; [`RegionRefusal::RayExhausted`] where the
/// reading leaves `p` unplaced (a circle's nearest foot walks nothing, so
/// this does not arise; were it to, no path starts from `p`).
pub(crate) fn circle_edge_hit<T: Decide>(
    circle: &geom::Curve3<T>,
    span: (T, T),
    rows: ConicRows,
    p: Point3<T>,
    outer: LoopKey,
    band: Band,
) -> Result<bool, RegionRefusal> {
    let k = match ConicArc::of(circle, span, rows, band) {
        Ok(Some(k)) => k,
        Err(ConicArcError::Escalated(e)) => return Err(RegionRefusal::Escalated(e.diag)),
        Ok(None) | Err(ConicArcError::WoundPastPeriod) => {
            return Err(RegionRefusal::WoundPastPeriod);
        }
    };
    match k
        .hit(p, band)
        .map_err(|e| RegionRefusal::Escalated(e.diag))?
    {
        ConicHit::On(_) | ConicHit::End => Ok(true),
        ConicHit::Off | ConicHit::Carrier => Ok(false),
        ConicHit::Unsettled => Err(RegionRefusal::RayExhausted { outer }),
    }
}

/// An in-band margin about one path, setting it aside.
pub(crate) fn in_band(diag: Indeterminate) -> RayFault<RegionRefusal> {
    RayFault::InBand(RegionRefusal::Escalated(diag))
}

/// **A root `t` against its edge's span `(t0, t1)`**, under `row` at
/// `lever` (the edge's speed, or a lower bound on it): `None` definitely
/// past an end; `Some(sideless)` within, `sideless` where it lies within
/// the band of an end, where no side is read.
///
/// # Errors
///
/// An in-band end sets the path aside.
pub(crate) fn span<T: Decide>(
    row: &'static str,
    t: T,
    (t0, t1): (T, T),
    lever: T,
    band: Band,
) -> Result<Option<bool>, RayFault<RegionRefusal>> {
    let start = decide(row, Margin::of(lever * (t - t0)), band).map_err(in_band)?;
    let end = decide(row, Margin::of(lever * (t1 - t)), band).map_err(in_band)?;
    Ok((start != Sign::Negative && end != Sign::Negative)
        .then_some(start == Sign::Zero || end == Sign::Zero))
}

/// A crossing as the verdict reads it at the closest: the path's unit
/// heading there and the face's side of the edge (`N × τ`).
pub(crate) struct Hit<T: geom_core::Real> {
    pub(crate) heading: Vec3<T>,
    pub(crate) face_side: Vec3<T>,
}

/// **Whether a path's start is in the face**, read at its closest
/// crossing: the heading against the face's side there, a sine levered by
/// `lever`, under `rows = (order, cross)`. Leaving the face (`Negative`)
/// puts the start in it.
///
/// # Errors
///
/// A path that meets nothing gives nothing to read
/// ([`RayFault::Unread`]); a tie, a sideless closest crossing or a
/// tangential one grazes; an in-band one sets the path aside.
pub(crate) fn side_at_closest<T: Decide>(
    crossings: Crossings<T, Hit<T>>,
    (order, cross): (&'static str, &'static str),
    lever: T,
    band: Band,
) -> Result<bool, RayFault<RegionRefusal>> {
    let hit = crossings
        .closest(order, band, |diag, _, _| RegionRefusal::Escalated(diag))?
        .ok_or(RayFault::Unread)?;
    match decide(
        cross,
        Margin::of(lever * hit.heading.dot(hit.face_side)),
        band,
    )
    .map_err(in_band)?
    {
        Sign::Negative => Ok(true),
        Sign::Positive => Ok(false),
        Sign::Zero => Err(RayFault::Graze),
    }
}

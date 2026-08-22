//! **Plane–plane chamfers** — the fillet's ruled sibling: replace each
//! requested edge's neighbourhood with a flat strip at equal setback
//! along both supports, and truncate each corner with a flat patch.
//!
//! # What is new here, and what is not
//!
//! Almost nothing is new here, and that is the point. The chamfer is
//! the same request over the same bodies, judged by the same
//! predicates, carved by the same in-place composition surgery; what
//! changes is the BAND grafted into the carve. So this module is a
//! front door and a scope statement, and the three places the two
//! verbs actually differ are marked on the arms that take them:
//!
//! 1. **The arm table** ([`super::fillet::battery::run_battery_for`]):
//!    plane–plane resolves to [`super::fillet::blend::chamfer_strip`]
//!    instead of the rolling-ball cylinder, and every other support
//!    pair refuses [`FilletError::ChamferArmUnsupported`].
//! 2. **Which predicates are questions.** C8's six are the fillet's;
//!    two of them (radius-vs-curvature headroom, spine regularity) are
//!    facts about a rolling BALL, and a ruled strip has neither
//!    quantity, so a chamfer run does not meter them. The other four
//!    — face clearance with the chamfer's own setbacks, chain G1,
//!    convexity-sign constancy, corner configuration — transfer
//!    unchanged and keep their `fillet3_*` names, because they measure
//!    the same margins over the same inputs.
//! 3. **The corner and band geometry** ([`super::fillet::surgery`]):
//!    the feet are where the two incident trimlines cross on each
//!    support rather than the ball's rest contacts, and the corner
//!    patch is the plane through those three feet.
//!
//! # Scope, and what refuses
//!
//! **Plane–plane support pairs only**, and open chains terminating at
//! trivalent corners whose three edges are all requested — the same
//! door the fillet's blank phase carves, since it is the same carve.
//! Everything else refuses typed and names itself: a curved support
//! ([`FilletError::ChamferArmUnsupported`] — the chamfer over curved
//! supports is VERBS-ARMS' machinery), a corner whose CONFIGURATION is
//! out of scope ([`FilletError::FilletCornerUnsupported`] with the
//! OQ6 corner tags), a request that does not cover a supported
//! corner's other edges ([`FilletError::UnsupportedRunOut`]), and a
//! CONCAVE chain ([`FilletError::UnsupportedChain`]).
//!
//! The symmetric setback is the whole parameter surface at v1. A
//! distance–distance or distance–angle chamfer is a widening of this
//! door — more parameters over the same construction — not a second
//! one, and nothing here forecloses it.
//!
//! # Convexity, deliberately absent from the geometry
//!
//! The chamfer's strip and corner patch carry **no convexity
//! parameter at all**. A trimline's in-plane direction is read off the
//! half-edge traversal (the interior-left convention), and both bands'
//! chart normals are minted as explicit positive combinations of the
//! supports' stored OUTWARD normals — so both are outward on a concave
//! edge exactly as on a convex one, and both faces mint with sense
//! `true` as a derivation rather than an assumption. Convexity enters
//! the chamfer only where it is a question about the REQUEST: the
//! battery's sign-constancy predicate, and the surgery's convex-open
//! admission door. That is what #644 asks of new corner code — the
//! concave widening moves those doors and leaves this geometry alone,
//! rather than finding three convex-only constants to derive one of.

use geom_core::{Band, Bounds, Decide, Tol};
use topo::{Body, EdgeKey};

use crate::fillet::battery::{FilletRequest, run_battery_for};
use crate::fillet::build::Filleted;
use crate::fillet::{BlendKind, FilletError};

/// A chamfered body. The same record the fillet's assembly returns —
/// `blend_faces` are the strips, `corner_faces` the flat patches, and
/// `band_faces` is empty, since a chamfer has no closed-chain band.
pub type Chamfered<T> = Filleted<T>;

/// **Chamfer a set of a body's edges** at equal setback `distance`
/// along both supports.
///
/// The battery runs FIRST and its refusal propagates unchanged — the
/// same ordering contract [`crate::fillet::build::fillet_edges`] keeps,
/// for the same reason: nothing is minted before a verdict exists.
///
/// # Errors
///
/// [`FilletError::RepeatedEdge`] when the request names one edge
/// twice; [`FilletError::ChamferArmUnsupported`] when a requested
/// edge's supports are not both planes; any predicate refusal the
/// battery raises, or [`FilletError::Escalated`] carrying the margin;
/// [`FilletError::UnsupportedBody`], [`FilletError::UnsupportedChain`],
/// [`FilletError::UnsupportedRunOut`],
/// [`FilletError::UnsupportedGeometry`] or
/// [`FilletError::FilletCornerUnsupported`] when the request is outside
/// the assembly's front door; [`FilletError::BodyNotIntact`] when the
/// body does not hold together where the plan reads it;
/// [`FilletError::RingClearance`] when a carried-through ring does not
/// clear a trimline; [`FilletError::Op`] / [`FilletError::Certify`]
/// carrying an operator's or the pcurve pass's own typed refusal.
pub fn chamfer_edges<T: Decide + Bounds>(
    body: &Body<T>,
    edges: &[EdgeKey],
    distance: T,
    band: Band,
    tol: Tol,
) -> Result<Chamfered<T>, FilletError> {
    // A repeated edge is malformed for the chain walk (it would double
    // a link), so it refuses before the battery samples anything.
    let mut requested = edges.to_vec();
    requested.sort_unstable();
    if let Some(edge) = requested.windows(2).find(|w| w[0] == w[1]).map(|w| w[0]) {
        return Err(FilletError::RepeatedEdge { edge });
    }

    let request = FilletRequest {
        body,
        edges: edges.to_vec(),
        radius: distance,
    };
    let verdict = run_battery_for(&request, band, BlendKind::Chamfer)?;
    crate::fillet::surgery::blend_surgery(body, &verdict, band, tol)
}

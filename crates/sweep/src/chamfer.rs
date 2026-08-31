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
//! 1. **The arm table** ([`crate::fillet::battery::run_battery_for`]):
//!    plane–plane resolves to [`crate::fillet::blend::chamfer_strip`]
//!    instead of the rolling-ball cylinder, and every other support
//!    pair refuses [`crate::fillet::BlendError::ChamferArmUnsupported`].
//! 2. **Which predicates are questions.** C8's six are the fillet's;
//!    two of them (radius-vs-curvature headroom, spine regularity) are
//!    facts about a rolling BALL, and a ruled strip has neither
//!    quantity, so a chamfer run does not meter them. The other four
//!    — face clearance with the chamfer's own setbacks, chain G1,
//!    convexity-sign constancy, corner configuration — transfer
//!    unchanged and keep their `fillet3_*` names, because they measure
//!    the same margins over the same inputs.
//! 3. **The corner and band geometry** ([`crate::fillet::surgery`]):
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
//! ([`crate::fillet::BlendError::ChamferArmUnsupported`] — the chamfer over curved
//! supports is VERBS-ARMS' machinery), a corner whose CONFIGURATION is
//! out of scope ([`crate::fillet::BlendError::UnsupportedCorner`] with the
//! OQ6 corner tags), a request that does not cover a supported
//! corner's other edges ([`crate::fillet::BlendError::UnsupportedRunOut`]), and a
//! CONCAVE chain ([`crate::fillet::BlendError::UnsupportedChain`]).
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

// The verb's one public path, mirroring `sweep::fillet`'s: the door is
// named by its module, not re-exported at the crate root as well.
pub use crate::fillet::build::{Chamfered, chamfer_edges};

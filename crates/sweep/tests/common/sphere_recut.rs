//! **The certified sphere-recut fixture**: the 3 x 3 x 0.8 plate, the
//! unit ball that pokes into it without crossing it, and the enclosure
//! width the subtract's arc chain is measured to escalate on at
//! `T = Interval`. Two suites run that subtract —
//! `m5_s12_curved_ops_interval`'s construction row and
//! `review_arceval_r1_probes`'s E2 — and both rows say they use the
//! same plate, the same ball and the same constant. Hosting the three
//! here is what makes that so rather than saying it, and it keeps
//! neither suite the owner of the other's fixture. Body authoring plus
//! the one measurement taken of that exact pair, so it routes here
//! (the module's routing rule).
//!
//! The three stay one group: the constant is a property of this pair
//! of bodies and of nothing else, so a plate or a ball that moved
//! without it would leave the constant describing a fixture no row
//! builds.
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - `m5_s12_curved_ops_interval`'s `notched` plate and `validated`
//!   helper — one suite's own;
//! - `review_arceval_r1_probes`'s E1 cap `block` — one suite's own,
//!   and a cutter for a different operand;
//! - [`super::operands`], which the plate would join by shape (it is a
//!   box) and does not, for the group reason above;
//! - [`super::oracles`], which holds truths derived WITHOUT the
//!   kernel. [`RECUT_MAPPED_ENCLOSURE_HI`] is the opposite — a
//!   measurement OF the kernel — and it comes to a shared home under
//!   that module's rule for spellings only because it has one
//!   spelling: both rows read this constant, and neither restates it.

use geom_core::{Interval, Tol};
use topo::Body;

use super::interval::{iv, v3};

/// The 3x3x0.8 plate the sphere-recut subtract cuts.
pub fn plate() -> Body<Interval> {
    sweep::test_support::block(3.0, 3.0, 0.8, Tol::witness())
}

/// The sphere-recut fixture's cutter: the unit
/// [`ball_poled_y`](sweep::test_support::ball_poled_y) at
/// `(1.5, 1.5, 0.5)`. With [`plate`] it is the whole fixture.
pub fn recut_ball() -> Body<Interval> {
    sweep::test_support::ball_poled_y(iv(1.0), v3(1.5, 1.5, 0.5), Tol::witness())
}

/// The `carrier_matches_mapped_source` enclosure the subtract of
/// [`recut_ball`] from [`plate`] escalates on (metres), measured at the
/// FIRST escalating sample of the crossing insertion's second child —
/// certification aborts there, so later samples of that edge never run
/// and this is not a claim about them. It is ε-INDEPENDENT — the same
/// bits at every ε — because it is the interval lane's enclosure width,
/// a property of the arithmetic that built the two points, not of the
/// tolerance they are judged against. The subtract therefore certifies
/// exactly when ε is at or above it.
///
/// `m5_s12_curved_ops_interval`'s construction row pins `hi` to this
/// value BIT-EXACTLY, in both directions. A regression that widens the
/// arc chain is loud, and so is a tightening that narrows it —
/// including a partial one that lands between the band and this
/// constant, which an upper-bound-only guard would admit in silence.
/// Either way the answer is the same: re-measure and re-state the
/// constant, never loosen the guard around it.
// **Re-measured 2026-08-31.** Was `1.1414768974413613e-12`. The arc
// chain tightened under enclosure work that merged with gates
// drawing default-ε only, so no run compared this constant until a
// later branch drew (interval, 1e-12). Re-stated, not loosened, as
// the constant's own doc requires.
pub const RECUT_MAPPED_ENCLOSURE_HI: f64 = 1.136_277_333_393_965_9e-12;

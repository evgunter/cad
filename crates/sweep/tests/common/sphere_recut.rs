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
/// and this is not a claim about them. It is NOT ε-independent: it
/// tracks ε (1.068e-13 at ε = 1e-13, 5.24e-14 at 5e-14), because the
/// crossing's parameter, which the restricted sub-arc's endpoints are
/// evaluated at, is solved to an enclosure the band sets. At 2e-13 and
/// above the chain certifies. So this value is the one measurement at
/// ε = 1e-13, and the escalation arm is exact at that ε alone.
///
/// `m5_s12_curved_ops_interval`'s construction row pins `hi` to this
/// value BIT-EXACTLY, in both directions. A regression that widens the
/// arc chain is loud, and so is a tightening that narrows it —
/// including a partial one that lands between the band and this
/// constant, which an upper-bound-only guard would admit in silence.
/// Either way the answer is the same: re-measure and re-state the
/// constant, never loosen the guard around it.
// **Measured at ε = 1e-13**, the one decade the escalation is
// reached at: the restricted sub-arc keeps its parent's exact
// carrier, so the chain's enclosure fits inside every gated ε row
// (1e-6, 1e-9, 1e-12) and the escalation arm is reached by none of them
// (`work/tcost/the-sphere-recut-escalation-is-reached-by-no-gated-eps-row.md`).
// Reproduce with `CAD_TOLERANCE_EPS=1e-13`.
pub const RECUT_MAPPED_ENCLOSURE_HI: f64 = 1.067_935_871_462_854_8e-13;

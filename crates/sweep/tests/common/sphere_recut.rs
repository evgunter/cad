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
//!   kernel. [`RECUT_MAPPED_ENCLOSURE_HI`] and its companions are the
//!   opposite — measurements OF the kernel — and they come to a shared
//!   home under that module's rule for spellings only because each has
//!   one spelling: both rows read them, and neither restates them.

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

/// The smallest ε the subtract of [`recut_ball`] from [`plate`] is
/// measured to certify at. It certifies at 1e-13, 1.5e-13 and 2e-13 and
/// escalates on `carrier_matches_mapped_source` at 7e-14 and 5e-14, so
/// the two rows' escalation arm owns `ε < RECUT_DECIDES_FROM` and their
/// definite arm the rest. No gated ε row reaches the escalation
/// (`work/tcost/the-sphere-recut-escalation-is-reached-by-no-gated-eps-row.md`);
/// reproduce it with `CAD_TOLERANCE_EPS=5e-14`.
///
/// Below 5e-14 the subtract refuses otherwise (3e-14: the re-cut
/// rotation fails to re-certify; 2e-14 and under: the ball itself is
/// not finished), and neither row makes a claim there.
pub const RECUT_DECIDES_FROM: f64 = 1e-13;

/// The ε [`RECUT_MAPPED_ENCLOSURE_HI`] is measured at.
pub const RECUT_PIN_EPS: f64 = 5e-14;

/// The `carrier_matches_mapped_source` enclosure the subtract of
/// [`recut_ball`] from [`plate`] escalates on (metres) at ε =
/// [`RECUT_PIN_EPS`], measured at the FIRST escalating sample of the
/// crossing insertion's second child — certification aborts there, so
/// later samples of that edge never run and this is not a claim about
/// them.
///
/// It tracks ε, because the crossing's parameter, which the restricted
/// sub-arc is evaluated at, is solved to an enclosure the band sets:
/// 7.49e-14 at ε = 7e-14 and this value at 5e-14, 1.07–1.11 ε. So the
/// rows pin it BIT-EXACTLY at [`RECUT_PIN_EPS`] alone, in both
/// directions, and elsewhere in the escalation arm hold it within
/// [`RECUT_HI_PER_EPS`] of ε. A regression that widens the arc chain is
/// loud, and so is a tightening that narrows it. Either way the answer
/// is the same: re-measure and re-state the constant, never loosen the
/// guard around it.
pub const RECUT_MAPPED_ENCLOSURE_HI: f64 = 5.552_802_405_695_234_3e-14;

/// The ceiling on `hi / ε` in the escalation arm away from
/// [`RECUT_PIN_EPS`]: 1.25, over the 1.07–1.11 measured.
pub const RECUT_HI_PER_EPS: f64 = 1.25;

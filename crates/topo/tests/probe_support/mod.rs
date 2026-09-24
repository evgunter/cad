//! **Shared vocabulary for the MATE-5 cylinder review probes.**
//!
//! What lives here is the stand-down: the one judgement the adversarial
//! suites make in common about a fixture the current ε cannot build.
//! The builders themselves are `topo::test_support`'s; this module adds
//! nothing to them but a way to decline.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::test_support::{CylFrame, cyl_wall_sheet};
use topo::{Body, FaceKey};

/// These suites' spelling of the shared door: the frame, the source
/// and the two chart windows, flat, so a row that turns on how A's
/// window and B's differ can be read as two adjacent lines.
///
/// One spelling, not one per suite: [`try_wall_sheet`] wraps THIS, so
/// a change to the tolerance or the source convention reaches the
/// fallible spelling too.
pub(crate) fn wall_sheet(
    body: &mut Body<f64>,
    frame: CylFrame,
    src_id: u64,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> FaceKey {
    cyl_wall_sheet(
        body,
        frame,
        Some(src_id),
        (u0, u1),
        (v0, v1),
        Tol::witness(),
    )
}

/// The cylinder-wall sheet builder, fallible at the MINT: a tilted
/// frame's chart images mint as exact structure only by bit-lottery
/// above ~10·ε of tilt (`r2_probes`' `r2_diag_mintable_tilts` maps it;
/// the constants of the rows using this door won the lottery at the
/// default ε row, where their demonstrations were measured).
///
/// A row whose fixture cannot be MINTED at this ε says so and stands
/// down (the `m5_pr7_split_meter` typed-fixture-refusal precedent) —
/// the arm never saw the pair, so neither outcome would be evidence
/// about it. `None` is that stand-down and nothing else: a probe
/// reading it as a verdict is reading a fixture that was never built.
///
/// This is the only `catch_unwind` on the cylinder-sheet family, and
/// it is on the `tests/` side deliberately. `topo::test_support`'s
/// doors panic; a fixture door that returns `None` where its siblings
/// panic would be a posture for the whole module, and the judgement
/// being made here — *this suite's row is not evidence at this ε* — is
/// a suite's to make, not a builder's.
pub(crate) fn try_wall_sheet(
    body: &mut Body<f64>,
    frame: CylFrame,
    src_id: u64,
    u0: f64,
    u1: f64,
    v0: f64,
    v1: f64,
) -> Option<FaceKey> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        wall_sheet(body, frame, src_id, u0, u1, v0, v1)
    }))
    .ok()
}

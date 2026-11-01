//! S6 adversarial review probe (e2e): the profile tangency pair
//! (pair 1) through the real validate API — exactly-tangent hole
//! (definite) vs in-band near-tangent hole (escalated). Both messages
//! must carry the shared recourse exactly once. Adopted from the
//! reviewer's probe set in the S6 fix pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{near_tangent_hole, tangent_hole, tol};
use geom_core::COINCIDENCE_RECOURSE;

fn assert_unified(msg: &str) {
    assert_eq!(
        msg.matches(COINCIDENCE_RECOURSE).count(),
        1,
        "carrier must appear exactly once: {msg}"
    );
}

#[test]
fn probe_profile_tangency_pair_e2e() {
    let msg = tangent_hole()
        .validate(tol())
        .expect_err("exact tangency must refuse")
        .to_string();
    eprintln!("[probe] profile definite:\n  {msg}\n");
    assert_unified(&msg);

    let msg = near_tangent_hole(tol().eps())
        .validate(tol())
        .expect_err("in-band clearance must escalate")
        .to_string();
    eprintln!("[probe] profile in-band:\n  {msg}\n");
    assert_unified(&msg);
    assert!(
        msg.contains("ambiguity band"),
        "margin payload must survive: {msg}"
    );
}

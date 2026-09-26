//! **The import loop asks `topo::per_part_gate_owed` whether to gate
//! each placed solid on its own, and the gate sits under that answer.**
//!
//! The F8/D7 per-part policy has one home, `topo::per_part_gate_owed`,
//! and this crate is one of its callers. What this row holds is the
//! consultation itself: a loop that spelled its own threshold again
//! (`instances.len() > 1`) would behave identically today and give the
//! policy a second home tomorrow, which no behavioural row can see.
//! `review_r1_tier_gate_probes.rs` holds the behaviour on both sides of
//! the threshold — one inverted solid refuses as the aggregate, two
//! refuse naming the guilty solid — and this row holds that the answer
//! comes from the policy.
//!
//! Read in the code view, so a commented-out call is not a call.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{balanced_end, blanked, code_only, required_matches};

const LIB: &str = include_str!("../src/lib.rs");

/// The one `if` whose condition consults the policy, and the per-solid
/// gate call inside its block — and nowhere else.
#[test]
fn the_import_loop_gates_each_solid_under_the_per_part_policy() {
    let code = blanked(code_only, "step-import/src/lib.rs", LIB);
    let what = "step-import/src/lib.rs (code view)";
    let heads = required_matches(&code, what, "if topo::per_part_gate_owed(");
    assert_eq!(
        heads.len(),
        1,
        "{what}: the import loop consults the per-part policy at exactly one `if`"
    );
    let open = heads[0]
        + code[heads[0]..]
            .find('{')
            .expect("the policy's `if` has a block");
    let close = balanced_end(&code, open).expect("the policy's block closes");
    let gates = required_matches(&code, what, "gate(&one");
    assert!(
        gates.iter().all(|&at| open < at && at < close),
        "{what}: the per-solid gate is called outside the block `topo::per_part_gate_owed` \
         guards — the loop has stopped taking its answer from the policy's one home"
    );
}

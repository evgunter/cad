//! **The import loop calls `topo::per_part_gate_owed` around its
//! per-solid gate.**
//!
//! The F8/D7 per-part policy has one home, `topo::per_part_gate_owed`,
//! and this crate is one of its callers. This row holds that the call
//! is PRESENT and that the per-solid gate sits inside the block it
//! guards: a loop that spelled its own threshold again
//! (`instances.len() > 1`) would behave identically today and give the
//! policy a second home, which no behavioural row can see. It does not
//! hold what the call is asked with. `review_r1_tier_gate_probes.rs`
//! holds the behaviour on both sides of the threshold — one inverted
//! solid refuses as the aggregate, two refuse naming the guilty solid.
//!
//! Read in the code view, so a commented-out call is not a call.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{ItemBody, blanked, code_only, item_body, required_matches};

const LIB: &str = include_str!("../src/lib.rs");

/// The one `if` whose condition calls the policy, and the per-solid
/// gate call inside its block — and nowhere else.
#[test]
fn the_import_loop_calls_the_per_part_policy_around_its_per_solid_gate() {
    let code = blanked(code_only, "step-import/src/lib.rs", LIB);
    let what = "step-import/src/lib.rs (code view)";
    let heads = required_matches(&code, what, "if topo::per_part_gate_owed(");
    assert_eq!(
        heads.len(),
        1,
        "{what}: the import loop calls the per-part policy at exactly one `if`"
    );
    let ItemBody::Body(block) = item_body(&code, heads[0]) else {
        panic!("{what}: the policy's `if` has no block");
    };
    let gates = required_matches(&code, what, "gate(&one");
    assert!(
        gates.iter().all(|at| block.contains(at)),
        "{what}: the per-solid gate is called outside the block `topo::per_part_gate_owed` \
         guards"
    );
}

//! **The gather asks `topo::per_part_gate_owed` whether to gate each
//! source body on its own, and the gate sits under that answer.**
//!
//! The F8/D7 per-part policy has one home, `topo::per_part_gate_owed`,
//! and `product_recorded`'s per-source pass is one of its callers. What
//! this row holds is the consultation itself: a pass that spelled its
//! own threshold again (`total_solids > 1`) would behave identically
//! today and give the policy a second home tomorrow, which no
//! behavioural row can see.
//!
//! Read in the code view, so a commented-out call is not a call.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use test_utils::source::{balanced_end, blanked, code_only, required_matches};

const PRODUCT: &str = include_str!("../src/product.rs");

/// The one `if` whose condition consults the policy, and the
/// per-source gate call inside its block — and nowhere else. The
/// per-source call is the one on a SOURCE body (`T::gate_at_rest(body`);
/// the aggregate's own call, on `&aggregate`, is not under the policy
/// and is not matched.
#[test]
fn the_gather_gates_each_source_under_the_per_part_policy() {
    let code = blanked(code_only, "editor-core/src/product.rs", PRODUCT);
    let what = "editor-core/src/product.rs (code view)";
    let heads = required_matches(&code, what, "if topo::per_part_gate_owed(");
    assert_eq!(
        heads.len(),
        1,
        "{what}: the gather consults the per-part policy at exactly one `if`"
    );
    let open = heads[0]
        + code[heads[0]..]
            .find('{')
            .expect("the policy's `if` has a block");
    let close = balanced_end(&code, open).expect("the policy's block closes");
    let gates = required_matches(&code, what, "T::gate_at_rest(body");
    assert!(
        gates.iter().all(|&at| open < at && at < close),
        "{what}: the per-source gate is called outside the block `topo::per_part_gate_owed` \
         guards — the gather has stopped taking its answer from the policy's one home"
    );
}

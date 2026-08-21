//! The wire spelling of a [`Node::Boolean`]'s operation, pinned.
//!
//! The operation is the KERNEL's `topo::BooleanOp`, re-exported rather
//! than re-minted, and its bytes are supplied by a `#[serde(with)]`
//! module in this crate (`persist::kernel_wire::boolean_op`) — the
//! kernel gains no serde dependency (the F3/G1 layering rule). Because
//! the type and its bytes are declared in different crates, nothing in
//! the compiler ties one to the other: these pins are that tie.
//!
//! The wire spelling is a stable STRING. A discriminant would silently
//! re-map if a fourth operation ever landed between two existing ones,
//! and a file would then read a different operation than it was
//! written with.
//!
//! The write door's own refusal — an operation this build can spell but
//! cannot read back — has no row here and can have none: in a build
//! whose read table is complete, nothing constructs the state that
//! refusal guards, inside the `with` module or outside it. The argument
//! for the check lives with the check, in
//! `persist::kernel_wire::boolean_op`'s module doc; that doc's
//! "Pinned by test" reaches the *admit* path below, not the refusal.
//!
//! The vocabulary is written down ONCE here, in [`EVERY_OPERATION`].
//! It is still a hand-written literal — safe Rust cannot tie an array
//! to a variant list without a proc macro, which is why the `with`
//! module carries a run-time check instead — but one copy is one place
//! to update when the kernel grows an operation, and the rows below
//! then cover it without being touched.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{BooleanOp, Node, ProfileProgram, RecipeNodeId};

/// Every operation the vocabulary has, in one place.
const EVERY_OPERATION: [BooleanOp; 3] =
    [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

fn boolean(op: BooleanOp) -> Node<ProfileProgram> {
    Node::Boolean {
        op,
        a: RecipeNodeId(1),
        b: RecipeNodeId(2),
        declare: None,
    }
}

/// The exact bytes, variant by variant. Written as whole-node literals
/// so a change to the ENCLOSING shape (a renamed field, a new key, a
/// tagging change) fails here too, not just a change to the operation
/// spelling.
#[test]
fn the_operation_rides_the_wire_as_its_variant_name() {
    // The expected bytes come from an EXHAUSTIVE match, so an
    // operation added to the kernel fails to compile here until it is
    // given a spelling — the one tie the compiler can carry.
    let expected = |op: BooleanOp| -> &'static str {
        match op {
            BooleanOp::Union => r#"{"Boolean":{"op":"Union","a":1,"b":2,"declare":null}}"#,
            BooleanOp::Intersect => r#"{"Boolean":{"op":"Intersect","a":1,"b":2,"declare":null}}"#,
            BooleanOp::Subtract => r#"{"Boolean":{"op":"Subtract","a":1,"b":2,"declare":null}}"#,
        }
    };
    for op in EVERY_OPERATION {
        let text = serde_json::to_string(&boolean(op)).unwrap();
        assert_eq!(text, expected(op), "the wire spelling of {op:?} moved");
    }
}

/// Both directions, over every operation the vocabulary has.
#[test]
fn every_operation_round_trips() {
    for op in EVERY_OPERATION {
        let node = boolean(op);
        let text = serde_json::to_string(&node).unwrap();
        let back: Node<ProfileProgram> = serde_json::from_str(&text).unwrap();
        assert_eq!(back, node, "{op:?} did not survive the round trip");
    }
}

/// An operation this build has no name for refuses TYPED rather than
/// reading as some other operation.
#[test]
fn an_unknown_operation_spelling_refuses() {
    let text = r#"{"Boolean":{"op":"Xor","a":1,"b":2,"declare":null}}"#;
    let err = serde_json::from_str::<Node<ProfileProgram>>(text)
        .expect_err("an unknown operation spelling must refuse");
    assert!(
        err.to_string().contains("Xor"),
        "the refusal names the spelling it could not read: {err}"
    );
}

/// The map form a derived unit-variant deserializer would also have
/// accepted. Nothing has ever WRITTEN it — serialization emits the
/// string in both the old shape and the new — so refusing it is a
/// deliberate narrowing, and this row is what keeps a later "leniency
/// fix" from undoing it unnoticed.
#[test]
fn the_map_form_of_a_variant_is_refused() {
    let text = r#"{"Boolean":{"op":{"Union":null},"a":1,"b":2,"declare":null}}"#;
    serde_json::from_str::<Node<ProfileProgram>>(text)
        .expect_err("the operation rides the wire as a string, never as a one-key map");
}

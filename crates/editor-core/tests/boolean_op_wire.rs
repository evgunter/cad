//! The wire spelling of the three boolean node kinds — [`Node::Union`],
//! [`Node::Intersect`] and [`Node::Subtract`] — pinned.
//!
//! The operation is the node's KIND: each is its own variant of
//! [`Node`], so the variant name is the operation's spelling, a stable
//! STRING, and no discriminant can silently re-map when a kind lands
//! between two existing ones. A union and an intersect carry their
//! members as a [`editor_core::Bodies`] (a family read whole, or reads
//! spelled at the slot); a subtract carries its `from` and its `tool`.
//!
//! The retired pair node's spelling — `Boolean` with an `op` — is not a
//! kind this build reads: a file that carries it refuses typed rather
//! than reading as some other node.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::AuthoredNode;
use editor_core::{Bodies, Node, RecipeNodeId};

/// The three kinds, each over the operands `0:1` and `0:2`, declaring
/// nothing, with the exact bytes each writes. Written as whole-node
/// literals so a change to the ENCLOSING shape (a renamed field, a new
/// key, a tagging change) fails here too, not just a change to the
/// kind's spelling.
fn rows() -> [(&'static str, AuthoredNode, &'static str); 3] {
    let (a, b) = (RecipeNodeId::new(0, 1), RecipeNodeId::new(0, 2));
    [
        (
            "union",
            Node::Union {
                members: Bodies::Spelled(vec![a.into(), b.into()]),
                declare: Vec::new(),
            },
            r#"{"Union":{"members":{"Spelled":[{"Node":"0:0000000000000001"},{"Node":"0:0000000000000002"}]},"declare":[]}}"#,
        ),
        (
            "intersect",
            Node::Intersect {
                members: Bodies::Spelled(vec![a.into(), b.into()]),
                declare: Vec::new(),
            },
            r#"{"Intersect":{"members":{"Spelled":[{"Node":"0:0000000000000001"},{"Node":"0:0000000000000002"}]},"declare":[]}}"#,
        ),
        (
            "subtract",
            Node::Subtract {
                from: a.into(),
                tool: b.into(),
                declare: Vec::new(),
            },
            r#"{"Subtract":{"from":{"Node":"0:0000000000000001"},"tool":{"Node":"0:0000000000000002"},"declare":[]}}"#,
        ),
    ]
}

/// The exact bytes, kind by kind.
#[test]
fn each_boolean_kind_rides_the_wire_as_its_variant_name() {
    for (what, node, expected) in rows() {
        let text = serde_json::to_string(&node).unwrap();
        assert_eq!(text, expected, "the wire spelling of the {what} moved");
    }
}

/// A union's family form: one read, the whole family.
#[test]
fn a_family_union_rides_the_wire_as_one_read() {
    let node: AuthoredNode = Node::Union {
        members: Bodies::Family(RecipeNodeId::new(0, 1).into()),
        declare: Vec::new(),
    };
    assert_eq!(
        serde_json::to_string(&node).unwrap(),
        r#"{"Union":{"members":{"Family":{"Node":"0:0000000000000001"}},"declare":[]}}"#
    );
}

/// Both directions, over every kind.
#[test]
fn every_boolean_kind_round_trips() {
    for (what, node, _) in rows() {
        let text = serde_json::to_string(&node).unwrap();
        let back: AuthoredNode = serde_json::from_str(&text).unwrap();
        assert_eq!(back, node, "the {what} did not survive the round trip");
    }
}

/// The retired pair node's spelling refuses TYPED, naming what it
/// could not read, rather than reading as some other node.
#[test]
fn the_retired_boolean_spelling_refuses() {
    let text = r#"{"Boolean":{"op":"Union","a":{"Node":"0:0000000000000001"},"b":{"Node":"0:0000000000000002"},"declare":[]}}"#;
    let err = serde_json::from_str::<AuthoredNode>(text)
        .expect_err("the retired Boolean spelling must refuse");
    assert!(
        err.to_string().contains("Boolean"),
        "the refusal names the spelling it could not read: {err}"
    );
}

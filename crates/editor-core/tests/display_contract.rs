//! **The Display contract for editor-core's façade-carried refusals**
//! (#1111): a consumer renders a refusal through the layer's own words
//! rather than composing a sentence about somebody else's failure, so
//! every arm must state what happened in prose — and must never read
//! as the `Debug` struct dump.
//!
//! The variant identifier and the field-name punctuation are the
//! dump's fingerprints; asserting their ABSENCE is what keeps a future
//! `write!(f, "{self:?}")` from passing these tests.
//!
//! `HitTestError`'s own contract test lives with the hit-test suite,
//! beside the behaviour it renders.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::persist::MigrationError;
use editor_core::{
    CapEnd, DeclareError, EntityKind, HitTestError, InterrogateError, MeshPickError, NodePickError,
    ParseError, RecipeNodeId, ResolveFault, ResolveIndeterminate, RoleSeg, SelectRefusal,
    StableName,
};

/// Asserts the F6 shape over one rendering: the wanted content is
/// present, no variant identifier leaks, no Debug punctuation, and the
/// sentence is not simply the dump.
fn assert_f6<E: core::fmt::Debug + core::fmt::Display>(err: &E, wants: &[&str], dumps: &[&str]) {
    let shown = err.to_string();
    for want in wants {
        assert!(
            shown.contains(want),
            "{err:?} renders as {shown:?}, missing {want:?}"
        );
    }
    for dump in dumps {
        assert!(
            !shown.contains(dump),
            "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
        );
    }
    assert!(
        !shown.contains('{') && !shown.contains("node:") && !shown.contains("name:"),
        "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
    );
    assert_ne!(shown, format!("{err:?}"));
}

/// A face name minted by node 7 — enough for the kind + minting-node
/// spelling every user-facing message uses.
fn face_name() -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(7),
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    }
}

/// A stable name renders as its kind plus its minting node — the half
/// a user can act on — and never its role path: a derivation is not
/// something a person reads mid-sentence.
#[test]
fn stable_name_display_is_kind_plus_minting_node() {
    let shown = face_name().to_string();
    assert_eq!(shown, "face name minted by node 7");
    assert!(
        !shown.contains("Cap") && !shown.contains('['),
        "the role path leaked into prose: {shown:?}"
    );
}

#[test]
fn node_pick_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId(4);
    let dumps = ["NotABody", "NoSuchBody", "Standing", "Tessellate", "Index"];
    let cases = [
        (
            NodePickError::NotABody { node },
            vec!["node 4", "not body-denoting"],
        ),
        (
            NodePickError::NoSuchBody { node, body: 2 },
            vec!["node 4", "index 2"],
        ),
        // The wrapped standing/kernel refusals are forwarded in their
        // own doors' words, not paraphrased.
        (
            NodePickError::Standing(HitTestError::NodeFailed { node }),
            vec!["hit test:", "node 4", "failed"],
        ),
        (
            NodePickError::Index(MeshPickError::PositionOutOfRange {
                patch: 1,
                triangle: 5,
                index: 99,
            }),
            vec!["triangle 5", "patch 1", "position 99"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
    // The tessellation arm forwards the kernel's own prose, prefix
    // included.
    let err =
        NodePickError::Tessellate(mesh::TessellateError::InvalidChordalTolerance { value: -1.0 });
    let shown = err.to_string();
    assert!(
        shown.contains("tessellate:") && shown.contains("chordal tolerance"),
        "the kernel refusal was not forwarded: {shown:?}"
    );
}

#[test]
fn resolve_indeterminate_display_names_its_content_not_its_struct() {
    let dumps = ["TargetFailed", "TargetPoisoned", "TargetNotEvaluated"];
    let cases = [
        (
            ResolveIndeterminate::TargetFailed {
                node: RecipeNodeId(6),
            },
            vec!["minting node 6", "failed"],
        ),
        (
            ResolveIndeterminate::TargetPoisoned {
                through: RecipeNodeId(2),
            },
            vec!["poisoned", "node 2", "upstream"],
        ),
        (
            ResolveIndeterminate::TargetNotEvaluated {
                node: RecipeNodeId(6),
            },
            vec!["minting node 6", "no result"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
}

#[test]
fn declare_error_display_names_its_content_not_its_struct() {
    let dumps = ["NoFindings", "NoMintedId"];
    assert_f6(
        &DeclareError::NoFindings,
        &["no findings", "records no intent"],
        &dumps,
    );
    assert_f6(
        &DeclareError::NoMintedId,
        &["minted no node id", "kernel bug"],
        &dumps,
    );
}

#[test]
fn interrogate_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId(7);
    let through = RecipeNodeId(3);
    let dumps = [
        "NodeNotEvaluated",
        "NodeFailed",
        "NodePoisoned",
        "NoSuchName",
        "Ambiguous",
        "WrongKind",
        "WholeBody",
        "NoBodies",
        "NoSuchBody",
        "Readback",
    ];
    let cases = [
        (
            InterrogateError::NodeNotEvaluated { node },
            vec!["node 7", "no result"],
        ),
        (
            InterrogateError::NodeFailed { node },
            vec!["node 7", "failed"],
        ),
        (
            InterrogateError::NodePoisoned { node, through },
            vec!["node 7", "node 3", "poisoned"],
        ),
        (InterrogateError::NoSuchName, vec!["stale", "another node"]),
        (
            InterrogateError::Ambiguous { candidates: 4 },
            vec!["4 entities", "no single geometry"],
        ),
        // Kinds render as prose nouns, never `Debug`.
        (
            InterrogateError::WrongKind {
                wanted: EntityKind::Face,
                found: EntityKind::Edge,
            },
            vec!["kind mismatch", "reads face", "denotes edge"],
        ),
        (
            InterrogateError::WholeBody,
            vec!["whole body", "faces, edges, or vertices"],
        ),
        (
            InterrogateError::NoBodies { payload: "datum" },
            vec!["datum", "no bodies"],
        ),
        (
            InterrogateError::NoSuchBody { index: 2 },
            vec!["index 2", "kernel bug"],
        ),
        // The kernel's own words are forwarded, not paraphrased.
        (
            InterrogateError::Readback(topo::ReadbackError::NoCarrier),
            vec!["interrogate:", "scaffolding"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
}

#[test]
fn select_refusal_display_names_its_content_not_its_struct() {
    let dumps = [
        "InBand",
        "TiedDisagrees",
        "Unreadable",
        "NotADatum",
        "NotALength",
        "PairInBand",
        "BadValue",
    ];
    let cases = [
        (
            SelectRefusal::TiedDisagrees {
                name: Box::new(face_name()),
                matched: 1,
                candidates: 3,
            },
            vec!["face", "node 7", "3 candidates", "1 match"],
        ),
        (
            SelectRefusal::Unreadable {
                name: Box::new(face_name()),
                error: InterrogateError::WholeBody,
            },
            vec!["face", "node 7", "whole body"],
        ),
        (
            SelectRefusal::NotADatum {
                datum: RecipeNodeId(9),
                found: "a body",
            },
            vec!["node 9", "a body", "evaluated datum"],
        ),
        (
            SelectRefusal::NotALength {
                dim: editor_core::Dimension::Angle,
            },
            vec!["distance is a distance", "Angle"],
        ),
        (
            SelectRefusal::Band,
            vec!["ambiguity band", "ambient tolerance"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
}

#[test]
fn resolve_fault_display_names_its_content_not_its_struct() {
    let dumps = ["PinMismatch", "EpsilonSeam", "Unresolved"];
    let cases = [
        (
            ResolveFault::PinMismatch,
            vec!["pin does not hold", "never retargeted silently"],
        ),
        (
            ResolveFault::EpsilonSeam,
            vec!["recorded tolerance", "one process, one ε"],
        ),
        (
            ResolveFault::Unresolved,
            vec!["did not resolve", "unknown id"],
        ),
    ];
    for (fault, wants) in cases {
        assert_f6(&fault, &wants, &dumps);
    }
}

#[test]
fn parse_error_display_names_its_content_not_its_struct() {
    let dumps = [
        "UnexpectedChar",
        "UnexpectedEnd",
        "UnexpectedToken",
        "TrailingInput",
        "MalformedNumber",
        "IntegerOverflow",
        "UnknownUnit",
        "UnknownFunction",
        "WrongArity",
        "UnknownParam",
    ];
    let cases = [
        (
            ParseError::UnexpectedChar { pos: 3, ch: '#' },
            vec!["byte 3", "alphabet"],
        ),
        (
            ParseError::UnexpectedEnd {
                pos: 5,
                expected: "an operand",
            },
            vec!["byte 5", "an operand"],
        ),
        (
            ParseError::UnexpectedToken {
                pos: 2,
                found: ")".to_string(),
                expected: "an operand",
            },
            vec!["byte 2", "an operand"],
        ),
        (
            ParseError::TrailingInput {
                pos: 8,
                found: "mm".to_string(),
            },
            vec!["byte 8", "complete expression"],
        ),
        (
            ParseError::MalformedNumber {
                pos: 0,
                text: "1e".to_string(),
            },
            vec!["byte 0", "malformed"],
        ),
        (
            ParseError::IntegerOverflow {
                pos: 0,
                text: "9223372036854775808".to_string(),
            },
            vec!["byte 0", "counts are exact"],
        ),
        (
            ParseError::UnknownUnit {
                pos: 4,
                symbol: "furlong".to_string(),
            },
            vec!["byte 4", "unit table is closed"],
        ),
        (
            ParseError::UnknownFunction {
                pos: 0,
                name: "tanh".to_string(),
            },
            vec!["byte 0", "not a function"],
        ),
        (
            ParseError::WrongArity {
                pos: 0,
                name: "atan2",
                expected: 2,
                found: 1,
            },
            vec!["byte 0", "atan2", "2 argument", "with 1"],
        ),
        (
            ParseError::UnknownParam {
                pos: 0,
                name: "width".to_string(),
            },
            vec!["byte 0", "not a parameter"],
        ),
    ];
    for (err, wants) in cases {
        assert_f6(&err, &wants, &dumps);
    }
}

#[test]
fn migration_error_display_names_its_content_not_its_struct() {
    let err = MigrationError {
        from: 3,
        reason: "the body carries no `nodes` array".to_string(),
    };
    let shown = err.to_string();
    for want in [
        "version 3",
        "4",
        "the body carries no `nodes` array",
        "migration step",
    ] {
        assert!(
            shown.contains(want),
            "{err:?} renders as {shown:?}, missing {want:?}"
        );
    }
    assert!(
        !shown.contains("MigrationError") && !shown.contains("from:") && !shown.contains("reason:"),
        "{err:?} renders as {shown:?} — that is the struct dump"
    );
    assert_ne!(shown, format!("{err:?}"));
}

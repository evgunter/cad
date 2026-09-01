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
    AssemblyError, CapEnd, DeclareError, Diagnosis, Dimension, DimensionError, DocParamValue,
    EditError, EntityKind, EvalError, HitTestError, InterrogateError, MateSide, MeshPickError,
    NodeErrorKind, NodePickError, ParamName, ParseError, ProgramFault, RecipeNodeId, RefusedRef,
    ResolveFault, ResolveIndeterminate, RoleSeg, SelectRefusal, SlotId, SnapshotError, StableName,
    StepArg,
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
        // The dimension's variant identifier: `NotALength` states the
        // dimension it read, and states it as a word.
        "Angle",
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
            vec!["distance is a distance", "dimension angle"],
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

/// Every rendering of a [`Dimension`] a user can reach, in one place.
///
/// A dimension is a quantity KIND — what a value measures — not an
/// address, so refusal prose says the common noun and never the
/// variant identifier; `Dimension`'s `Display` is the one home of that
/// rule and `with_article` supplies the article the value decides
/// ("an angle", not "a angle"). Each row forbids the identifiers that
/// would appear if the arm went back to `Debug`, so a reverted arm
/// fails here rather than passing on a sentence that still reads
/// almost right.
#[test]
fn a_dimension_reaches_refusal_prose_as_a_word_not_as_its_variant() {
    let name = ParamName("width".to_string());
    let dumps = ["Length", "Angle", "Count", "Scalar"];

    // The edit door.
    assert_f6(
        &EditError::SlotDimensionMismatch {
            slot: SlotId::Radius,
            expected: Dimension::Length,
            found: Dimension::Angle,
        },
        &["needs a length expression", "got an angle"],
        &dumps,
    );
    assert_f6(
        &EditError::PayloadParamDimensionMismatch {
            name: name.clone(),
            node: RecipeNodeId(3),
            declared: Dimension::Length,
            referenced: Dimension::Count,
        },
        &["is declared length", "references it as count"],
        &dumps,
    );
    assert_f6(
        &EditError::AssertionDimension {
            node: RecipeNodeId(5),
            measure: RecipeNodeId(4),
            measured: Dimension::Length,
            bound: Dimension::Angle,
        },
        &["bounds a length measure", "with an angle expression"],
        &dumps,
    );
    assert_f6(
        &EditError::DocParamDimensionMismatch {
            name: name.clone(),
            node: RecipeNodeId(3),
            slot: SlotId::Distance,
            declared: Dimension::Scalar,
            referenced: Dimension::Length,
        },
        &["is declared scalar", "references it as length"],
        &dumps,
    );
    assert_f6(
        &EditError::DocParamValueKindMismatch {
            name: name.clone(),
            declared: Dimension::Length,
            offered: DocParamValue::Count(2),
        },
        &["is declared length"],
        &dumps,
    );

    // The construction-time dimension checker.
    assert_f6(
        &DimensionError::Mismatch {
            op: "+",
            left: Dimension::Length,
            right: Dimension::Angle,
        },
        &["to length and angle"],
        &dumps,
    );
    assert_f6(
        &DimensionError::MulNeedsScalar {
            left: Dimension::Length,
            right: Dimension::Length,
        },
        &["needs a scalar operand", "length x length"],
        &dumps,
    );
    assert_f6(
        &DimensionError::DivNeedsScalarDivisor {
            left: Dimension::Length,
            right: Dimension::Length,
        },
        &["needs a scalar divisor", "length / length"],
        &dumps,
    );
    assert_f6(
        &DimensionError::TrigNeedsAngle {
            op: "sin",
            found: Dimension::Count,
        },
        &["needs an angle operand", "got a count"],
        &dumps,
    );
    assert_f6(
        &DimensionError::NotCount {
            found: Dimension::Scalar,
        },
        &["a count operand is required", "got a scalar"],
        &dumps,
    );
    assert_f6(
        &DimensionError::DisplayUnitMismatch {
            unit: Dimension::Length,
            literal: Dimension::Angle,
        },
        &["measures length", "literal is angle"],
        &dumps,
    );

    // The evaluator.
    assert_f6(
        &EvalError::ParamDimensionMismatch {
            name: name.clone(),
            expected: Dimension::Length,
            found: Dimension::Angle,
        },
        &["referenced as length", "bound as angle"],
        &dumps,
    );
    assert_f6(
        &EvalError::ContinuousExprInCountEval {
            found: Dimension::Length,
        },
        &["a length expression does not evaluate as a count"],
        &dumps,
    );
    assert_f6(
        &NodeErrorKind::AssertionDimension {
            measured: Dimension::Length,
            bound: Dimension::Angle,
        },
        &["bound is an angle", "constrains is a length"],
        &dumps,
    );

    // The load door's checker. Its program-slot arm spells the slot
    // address out, so only the dimensions are at issue there.
    assert_f6(
        &ProgramFault::SlotDimension {
            slot: SlotId::Profile {
                loop_: 0,
                step: 2,
                arg: StepArg::PointX,
            },
            expected: Dimension::Length,
            found: Dimension::Count,
        },
        &["needs a length expression", "got a count"],
        &["Length", "Count"],
    );
    assert_f6(
        &ProgramFault::SlotDimension {
            slot: SlotId::Radius,
            expected: Dimension::Length,
            found: Dimension::Angle,
        },
        &["needs a length expression", "got an angle"],
        &dumps,
    );
    assert_f6(
        &SnapshotError::AssertionBound {
            node: RecipeNodeId(5),
            measure: RecipeNodeId(4),
            measured: Some(Dimension::Length),
            bound: Dimension::Angle,
        },
        &["bounds a length measure", "with an angle expression"],
        &dumps,
    );
    assert_f6(
        &SnapshotError::AssertionBound {
            node: RecipeNodeId(5),
            measure: RecipeNodeId(4),
            measured: None,
            bound: Dimension::Count,
        },
        &["carries a count bound", "which is not a measure"],
        &dumps,
    );
}

/// A predicate flip names the two signs as words: `Sign` has a
/// `Display`, and a diagnosis's payload-holding arms forward the
/// payload's own rendering.
#[test]
fn a_predicate_flip_names_its_signs_as_words() {
    assert_f6(
        &Diagnosis::PredicateFlip {
            predicate: "name_frag_side_of",
            from: geom_core::predicate::Sign::Positive,
            to: geom_core::predicate::Sign::Negative,
        },
        &["name_frag_side_of", "flipped from positive to negative"],
        &["Positive", "Negative", "PredicateFlip"],
    );
}

/// Refusals that name a stable name FORWARD its `Display` rather than
/// re-spelling the kind-plus-minting-node phrase. The expectation is
/// built from the impl, so a copy that stops tracking it fails here —
/// which a literal expectation could not catch.
#[test]
fn refusals_that_name_a_stable_name_forward_its_display() {
    let phrase = face_name().to_string();

    let reference = AssemblyError::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(face_name()),
        why: RefusedRef::Vanished,
    };
    let shown = reference.to_string();
    assert!(
        shown.contains(&format!("(a {phrase})")),
        "the mate reference re-spells the name instead of forwarding it: {shown:?}"
    );

    let both = NodeErrorKind::DeclareBothOperands {
        name: Box::new(face_name()),
    };
    let shown = both.to_string();
    assert!(
        shown.contains(&format!("the declared {phrase} resolves")),
        "the declaration refusal re-spells the name instead of forwarding it: {shown:?}"
    );
}

/// An entity kind carries the article that agrees with it, because the
/// value decides which one is correct: three of the four kinds take
/// "a" and `Edge` takes "an", so a sentence that hard-codes one is
/// wrong for every edge-kind refusal it can reach — and each of these
/// IS reachable with an edge (a mate reference may name any kind,
/// which is what `NotAFace` reports).
#[test]
fn an_entity_kind_carries_the_article_that_agrees_with_it() {
    let edge_name = StableName {
        kind: EntityKind::Edge,
        node: RecipeNodeId(7),
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };

    let reference = AssemblyError::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(edge_name.clone()),
        why: RefusedRef::NotAFace {
            kind: EntityKind::Edge,
        },
    };
    let shown = reference.to_string();
    assert!(
        shown.contains(&format!("(an {edge_name})")) && shown.contains("it names an edge"),
        "an edge-kind mate reference reads as \"a edge\": {shown:?}"
    );

    let face = AssemblyError::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(face_name()),
        why: RefusedRef::NotAFace {
            kind: EntityKind::Vertex,
        },
    };
    let shown = face.to_string();
    assert!(
        shown.contains(&format!("(a {})", face_name())) && shown.contains("it names a vertex"),
        "the consonant kinds must keep \"a\": {shown:?}"
    );
}

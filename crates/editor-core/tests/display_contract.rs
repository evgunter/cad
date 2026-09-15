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

use editor_core::{
    AssemblyError, CapEnd, ContactClass, DeclareError, Diagnosis, Dimension, DimensionError,
    DocParamValue, EditError, EntityKind, EvalError, HitTestError, InterrogateError, MateFault,
    MateSide, MeshPickError, NodeErrorKind, NodePickError, ParamName, ParseError, ProgramFault,
    RecipeNodeId, RefusedRef, ResolveFault, ResolveIndeterminate, RoleSeg, SelectRefusal, SlotId,
    SnapshotError, StableName, StepArg,
};
use geom_core::BandError;

use test_utils::f6::variant_identifier;

/// [`test_utils::f6::assert_f6`] with this binary's field-punctuation
/// roster: the `Debug` field names editor-core's refusal payloads
/// carry. Written ONCE for the binary rather than once per suite —
/// `m4_pr4_hit`'s hit-test row calls this one, and the divergence
/// between its old roster and this one is what a second spelling cost
/// (`work/view/f6-display-predicate-is-spelled-three-times-with-no-home`).
pub(crate) fn assert_f6<E: core::fmt::Debug + core::fmt::Display>(
    err: &E,
    wants: &[&str],
    dumps: &[&str],
) {
    test_utils::f6::assert_f6(err, wants, dumps, &["node:", "name:"]);
}

/// Runs the F6 shape over one error enum's whole case list, with the
/// enum's own variant identifiers as the ban list, and reports any
/// variant the cases do not reach.
///
/// **What each half actually guarantees.** `exhaustive` is a
/// wildcard-free `match` over the enum and NOTHING else: it names no
/// identifiers, so the only thing it can do is stop compiling. That is
/// its whole job — a variant added to the enum, or renamed, leaves the
/// `match` non-exhaustive and forces the author to open this file. It
/// is not itself a census, because the compiler cannot tell whether the
/// author then did the right thing. `all` is the identifier roster,
/// written out; the set difference below is what welds it. Every
/// identifier in `all` must be produced by some case's own `Debug`
/// (`test_utils::f6::variant_identifier`) and every case's must be in
/// `all`, so the roster cannot drift in either direction and a
/// MISSPELLING in it fails — nothing here trusts a string typed beside
/// a pattern, which rustc never checks.
///
/// **The one hole, stated.** An author who adds a variant, adds its arm
/// to `exhaustive` — which the compiler makes them do — and then adds
/// NEITHER a case NOR an `all` entry is not caught: nothing renders the
/// variant, so nothing contradicts a roster that never grew. The
/// compile error is what stands between that and an accident; closing
/// it would need the variant list itself to be derivable, which safe
/// Rust does not offer without a macro or a derive over a type this
/// crate does not own.
///
/// `also_banned` carries identifiers from OTHER enums that a rendering
/// must not leak either.
pub(crate) fn assert_f6_every_variant<E: core::fmt::Debug + core::fmt::Display>(
    cases: &[(E, Vec<&str>)],
    exhaustive: fn(&E),
    all: &[&str],
    also_banned: &[&str],
) {
    let dumps: Vec<&str> = all.iter().chain(also_banned).copied().collect();
    for (err, wants) in cases {
        exhaustive(err);
        assert_f6(err, wants, &dumps);
    }
    let covered_words: Vec<String> = cases
        .iter()
        .map(|(err, _)| variant_identifier(err))
        .collect();
    let covered: Vec<&str> = covered_words.iter().map(String::as_str).collect();
    // The one set comparison in this binary, borrowed rather than
    // re-spelled: a second copy of the comparator kept in step by hand
    // is the defect this file's subject IS.
    if let Some(report) = crate::switch_program_vocabulary::set_difference(
        all,
        &covered,
        &format!(
            "`{}`'s identifier roster and its rendered cases disagree",
            core::any::type_name::<E>()
        ),
        "rendered by a case and absent from the roster — add it, spelled as `Debug` renders it",
        "in the roster and rendered by no case — give it a case, or fix its spelling",
    ) {
        panic!("{report}");
    }
}

/// Every [`Dimension`]'s `Debug`, which is what a refusal that names a
/// dimension must not leak.
///
/// Taken off `Dimension::ALL` rather than written down, so a dimension
/// added to the lattice is forbidden at every site that uses this
/// without an edit — and it is ONE derivation, because two copies of
/// it are two lists to keep in step.
fn dimension_dump_words() -> Vec<String> {
    Dimension::ALL
        .iter()
        .map(|dim| format!("{dim:?}"))
        .collect()
}

/// Every [`Sign`](geom_core::predicate::Sign), so a ban list built from
/// it is the whole enum rather than the signs one row happens to
/// construct. The `match` is the exhaustiveness token: a sign added to
/// the lattice leaves it non-exhaustive and this file stops compiling,
/// which is what sends the author to the array beside it.
fn all_signs() -> Vec<geom_core::predicate::Sign> {
    use geom_core::predicate::Sign;
    let all = vec![Sign::Negative, Sign::Zero, Sign::Positive];
    for sign in &all {
        match sign {
            Sign::Negative | Sign::Zero | Sign::Positive => (),
        }
    }
    all
}

/// The `&str` view of a list of derived identifier words.
fn as_strs(words: &[String]) -> Vec<&str> {
    words.iter().map(String::as_str).collect()
}

/// A face name minted by node 7 — enough for the kind + minting-node
/// spelling every user-facing message uses.
fn face_name() -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(7),
        path: vec![RoleSeg::Cap(CapEnd::End)],
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

/// `NodePickError`'s exhaustiveness token: the `match` has no wildcard
/// arm, so a variant added to the enum — or renamed — leaves it
/// non-exhaustive and this file stops compiling. It returns nothing on
/// purpose; the identifiers come off each value's own `Debug`, never
/// off a string typed beside a pattern.
fn node_pick_error_is_exhaustive(e: &NodePickError) {
    match e {
        NodePickError::Standing(_)
        | NodePickError::NotABody { .. }
        | NodePickError::NoSuchBody { .. }
        | NodePickError::Tessellate(_)
        | NodePickError::Index(_) => (),
    }
}

/// The identifier roster, welded to the cases by the set difference in
/// `assert_f6_every_variant`.
const NODE_PICK_ERROR_VARIANTS: &[&str] =
    &["Standing", "NotABody", "NoSuchBody", "Tessellate", "Index"];

#[test]
fn node_pick_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId(4);
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
        // own doors' words, not paraphrased — prefix included.
        (
            NodePickError::Standing(HitTestError::NodeFailed { node }),
            vec!["hit test:", "node 4", "failed"],
        ),
        (
            NodePickError::Tessellate(mesh::TessellateError::InvalidChordalTolerance {
                value: -1.0,
            }),
            vec!["tessellate:", "chordal tolerance"],
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
    assert_f6_every_variant(
        &cases,
        node_pick_error_is_exhaustive,
        NODE_PICK_ERROR_VARIANTS,
        &[],
    );
}

/// `ResolveIndeterminate`'s exhaustiveness token; wildcard-free, as
/// [`node_pick_error_is_exhaustive`].
fn resolve_indeterminate_is_exhaustive(e: &ResolveIndeterminate) {
    match e {
        ResolveIndeterminate::TargetFailed { .. }
        | ResolveIndeterminate::TargetPoisoned { .. }
        | ResolveIndeterminate::TargetNotEvaluated { .. } => (),
    }
}

/// The identifier roster; welded by the set difference.
const RESOLVE_INDETERMINATE_VARIANTS: &[&str] =
    &["TargetFailed", "TargetPoisoned", "TargetNotEvaluated"];

#[test]
fn resolve_indeterminate_display_names_its_content_not_its_struct() {
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
    assert_f6_every_variant(
        &cases,
        resolve_indeterminate_is_exhaustive,
        RESOLVE_INDETERMINATE_VARIANTS,
        &[],
    );
}

/// `DeclareError`'s exhaustiveness token; wildcard-free, as
/// [`node_pick_error_is_exhaustive`].
fn declare_error_is_exhaustive(e: &DeclareError) {
    match e {
        DeclareError::NoFindings | DeclareError::Edit(_) | DeclareError::NoMintedId => (),
    }
}

/// The identifier roster; welded by the set difference.
const DECLARE_ERROR_VARIANTS: &[&str] = &["NoFindings", "Edit", "NoMintedId"];

#[test]
fn declare_error_display_names_its_content_not_its_struct() {
    let cases = [
        (
            DeclareError::NoFindings,
            vec!["no findings", "records no intent"],
        ),
        // The wrapping arm forwards the document edit's own refusal,
        // which already carries its slot and its recourse.
        (
            DeclareError::Edit(EditError::SlotDimensionMismatch {
                slot: SlotId::Radius,
                expected: Dimension::Length,
                found: Dimension::Angle,
            }),
            vec![
                "the document edit refused",
                "needs a length expression",
                "got an angle",
            ],
        ),
        (
            DeclareError::NoMintedId,
            vec!["minted no node id", "kernel bug"],
        ),
    ];
    assert_f6_every_variant(
        &cases,
        declare_error_is_exhaustive,
        DECLARE_ERROR_VARIANTS,
        &[],
    );
}

/// `InterrogateError`'s exhaustiveness token; wildcard-free, as
/// [`node_pick_error_is_exhaustive`].
fn interrogate_error_is_exhaustive(e: &InterrogateError) {
    match e {
        InterrogateError::NodeNotEvaluated { .. }
        | InterrogateError::NodeFailed { .. }
        | InterrogateError::NodePoisoned { .. }
        | InterrogateError::NoSuchName
        | InterrogateError::Ambiguous { .. }
        | InterrogateError::WrongKind { .. }
        | InterrogateError::WholeBody
        | InterrogateError::NoBodies { .. }
        | InterrogateError::NoSuchBody { .. }
        | InterrogateError::Readback(_) => (),
    }
}

/// The identifier roster; welded by the set difference.
const INTERROGATE_ERROR_VARIANTS: &[&str] = &[
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

#[test]
fn interrogate_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId(7);
    let through = RecipeNodeId(3);
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
    assert_f6_every_variant(
        &cases,
        interrogate_error_is_exhaustive,
        INTERROGATE_ERROR_VARIANTS,
        &[],
    );
}

/// `SelectRefusal`'s exhaustiveness token — **the one here that the
/// compiler does not keep**. `SelectRefusal` carries
/// `#[non_exhaustive]`, so a `match` outside `editor-core` is REQUIRED
/// to carry a wildcard arm and rustc checks nothing about the arms
/// above it: a variant added to the enum compiles fine here. What the
/// arms still buy is the other half — a variant renamed or removed
/// breaks the pattern and the file stops compiling, exactly as for the
/// six ordinary enums. Only ADDITION is unchecked, and the wildcard
/// below does not repair it: reaching the panic needs a case that
/// constructs the new variant, which is the vacuity this file exists to
/// close. The real home is a unit test beside the enum, inside the
/// crate where the attribute does not apply
/// (`work/wire/select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`);
/// the other six are not weakened to match this one.
fn select_refusal_is_exhaustive(e: &SelectRefusal) {
    match e {
        SelectRefusal::InBand { .. }
        | SelectRefusal::TiedDisagrees { .. }
        | SelectRefusal::Unreadable { .. }
        | SelectRefusal::NotADatum { .. }
        | SelectRefusal::NotALength { .. }
        | SelectRefusal::PairInBand { .. }
        | SelectRefusal::BadValue(_)
        | SelectRefusal::Band(_) => (),
        other => panic!("`SelectRefusal` grew a variant with no arm here: {other:?}"),
    }
}

/// The identifier roster; welded by the set difference.
const SELECT_REFUSAL_VARIANTS: &[&str] = &[
    "InBand",
    "TiedDisagrees",
    "Unreadable",
    "NotADatum",
    "NotALength",
    "PairInBand",
    "BadValue",
    "Band",
];

/// An in-band margin with a named predicate — the shape a selection
/// refusal carries out of the funnel.
fn in_band(predicate: &'static str) -> geom_core::Indeterminate {
    geom_core::Indeterminate {
        margin: geom_core::MarginDiag::Value(3e-11),
        band: geom_core::Band::new(1e-12, 1e-9).expect("zero < escalate"),
        predicate: Some(predicate),
    }
}

#[test]
fn select_refusal_display_names_its_content_not_its_struct() {
    // `NotALength` states the dimension it read, and states it as a
    // word, so the dimension identifiers are banned here too.
    let dimension_words = dimension_dump_words();
    let also_banned = as_strs(&dimension_words);

    let cases = [
        (
            SelectRefusal::InBand {
                name: Box::new(face_name()),
                predicate: editor_core::SEL_DATUM_DISTANCE,
                source: in_band(editor_core::SEL_DATUM_DISTANCE),
            },
            vec![
                "face",
                "node 7",
                "neither certified in nor out",
                "ambiguity band",
                editor_core::SEL_DATUM_DISTANCE,
            ],
        ),
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
                dim: Dimension::Angle,
            },
            vec!["distance is a distance", "dimension angle"],
        ),
        // The detector's pair-shaped sibling of `InBand`: it names the
        // PAIR, and says that detection reports only definite findings.
        (
            SelectRefusal::PairInBand {
                pair: Box::new((face_name(), face_name())),
                predicate: "bool_plane_side_of",
                source: in_band("bool_plane_side_of"),
            },
            vec![
                "the pair (",
                "face",
                "node 7",
                "neither certified in nor out",
                "only definite findings",
            ],
        ),
        (
            SelectRefusal::BadValue(EvalError::ContinuousExprInCountEval {
                found: Dimension::Length,
            }),
            vec![
                "the stated value did not evaluate",
                "does not evaluate as a count",
            ],
        ),
        // The F6 shape only; the arm's REACHABILITY and the payload
        // it must forward are pinned through the real doors in
        // `wire_band_cause`, which is where a `BandError` can be
        // obtained from `Band::linear` rather than written down.
        (
            SelectRefusal::Band(BandError::Empty {
                zero: 5e-324,
                escalate: 5e-324,
            }),
            vec!["ambiguity band", "ambient tolerance", "strictly below"],
        ),
    ];
    assert_f6_every_variant(
        &cases,
        select_refusal_is_exhaustive,
        SELECT_REFUSAL_VARIANTS,
        &also_banned,
    );
}

/// `ResolveFault`'s exhaustiveness token; wildcard-free, as
/// [`node_pick_error_is_exhaustive`].
fn resolve_fault_is_exhaustive(e: &ResolveFault) {
    match e {
        ResolveFault::PinMismatch | ResolveFault::EpsilonSeam | ResolveFault::Unresolved => (),
    }
}

/// The identifier roster; welded by the set difference.
const RESOLVE_FAULT_VARIANTS: &[&str] = &["PinMismatch", "EpsilonSeam", "Unresolved"];

#[test]
fn resolve_fault_display_names_its_content_not_its_struct() {
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
    assert_f6_every_variant(
        &cases,
        resolve_fault_is_exhaustive,
        RESOLVE_FAULT_VARIANTS,
        &[],
    );
}

/// `ParseError`'s exhaustiveness token; wildcard-free, as
/// [`node_pick_error_is_exhaustive`].
fn parse_error_is_exhaustive(e: &ParseError) {
    match e {
        ParseError::UnexpectedChar { .. }
        | ParseError::UnexpectedEnd { .. }
        | ParseError::UnexpectedToken { .. }
        | ParseError::TrailingInput { .. }
        | ParseError::MalformedNumber { .. }
        | ParseError::IntegerOverflow { .. }
        | ParseError::UnknownUnit { .. }
        | ParseError::UnknownFunction { .. }
        | ParseError::WrongArity { .. }
        | ParseError::UnknownParam { .. }
        | ParseError::Dimension { .. } => (),
    }
}

/// The identifier roster; welded by the set difference.
const PARSE_ERROR_VARIANTS: &[&str] = &[
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
    "Dimension",
];

#[test]
fn parse_error_display_names_its_content_not_its_struct() {
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
        // The text door forwards the smart constructor's own refusal
        // and adds only the position, so the dimension checker's words
        // are what a reader sees.
        (
            ParseError::Dimension {
                pos: 6,
                error: DimensionError::MulNeedsScalar {
                    left: Dimension::Length,
                    right: Dimension::Length,
                },
            },
            vec!["byte 6", "needs a scalar operand", "length x length"],
        ),
    ];
    assert_f6_every_variant(&cases, parse_error_is_exhaustive, PARSE_ERROR_VARIANTS, &[]);
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
    let dump_words = dimension_dump_words();
    let dumps = as_strs(&dump_words);

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
        &dumps,
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
    let sign_debug: Vec<String> = all_signs().iter().map(|s| format!("{s:?}")).collect();
    let sign_words = as_strs(&sign_debug);
    assert_f6(
        &Diagnosis::PredicateFlip {
            predicate: "name_frag_side_of",
            from: geom_core::predicate::Sign::Positive,
            to: geom_core::predicate::Sign::Negative,
        },
        &["name_frag_side_of", "flipped from positive to negative"],
        // Every `Sign`, not the two this row happens to construct: a
        // rendering that leaked `Zero` would be just as much a dump.
        // `PredicateFlip` is `Diagnosis`'s own identifier, and this row
        // renders that one arm.
        &[sign_words.as_slice(), &["PredicateFlip"]].concat(),
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
    assert!(
        shown.contains(
            "no entity answers to it, in the product or at the node the mate reads it at"
        ),
        "a vanished name is one neither table answers to: {shown:?}"
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

/// The WHY clause of a mate-reference refusal says what the gate
/// checked and no more: a name read below a root names the operand
/// and the rule (a reference resolves against a root's own rows); a
/// tie names its width. Every other row asserting these sentences
/// compares against the impl, so this is their one home.
#[test]
fn a_mate_reference_refusal_says_what_the_gate_checked() {
    let below = AssemblyError::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::B,
        name: Box::new(face_name()),
        why: RefusedRef::ReadBelowARoot {
            at: RecipeNodeId(5),
        },
    };
    assert_f6(
        &below,
        &[
            "mate 2's b reference",
            "does not name a face of the product",
            "it is read at node 5, which is not a root of the product, and a reference \
             resolves against a root's own rows",
        ],
        &["ReadBelowARoot", "Reference"],
    );

    let tied = AssemblyError::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(face_name()),
        why: RefusedRef::Ambiguous { width: 2 },
    };
    assert_f6(
        &tied,
        &[
            "mate 2's a reference",
            "2 entities answer to it",
            "a tie is never broken by picking",
        ],
        &["Ambiguous", "Reference"],
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
        path: vec![RoleSeg::Cap(CapEnd::End)],
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

/// The mint door's at-rest refusal ends on
/// [`editor_core::NO_AT_REST_RECORD_RECOURSE`] — and so does the
/// `MintRefusal` row it is raised from, because one function renders
/// the sentence for both. A recourse reached by only one of the two
/// carriers is a user who sees the repair or not depending on how deep
/// the mate was declared.
#[test]
fn the_mint_doors_at_rest_refusal_ends_on_its_recourse_from_both_carriers() {
    let why = editor_core::class_admission(ContactClass::Tangent).no_record_reason();
    let raised = AssemblyError::NoAtRestRecord {
        mate: RecipeNodeId(5),
        class: ContactClass::Tangent,
        why,
    };
    assert_f6(
        &raised,
        &[
            "mate 5's class Tangent has no at-rest kernel record",
            why,
            editor_core::NO_AT_REST_RECORD_RECOURSE,
        ],
        &["NoAtRestRecord"],
    );

    let row = editor_core::MintRefusal::NoAtRestRecord {
        mate: RecipeNodeId(5),
        class: ContactClass::Tangent,
        why,
    };
    assert_f6(
        &row,
        &[why, editor_core::NO_AT_REST_RECORD_RECOURSE],
        &["NoAtRestRecord"],
    );
}

/// A mate-solve contradiction states WHO is at fault. Two mates that
/// cannot both hold are named as a pair; one mate that contradicts
/// itself is named ONCE, because "mates 6 and 6" reads as an indexing
/// fault and hides the shape the payload states.
///
/// Both shapes end on [`editor_core::CONTRADICTORY_RECOURSE`], which
/// is why that sentence names neither a pair nor a count: one recourse
/// covers a repair that is the same either way.
#[test]
fn a_contradiction_names_one_mate_once_and_a_pair_as_a_pair() {
    let pair = MateFault::Contradictory {
        held: RecipeNodeId(3),
        added: RecipeNodeId(5),
        predicate: "mate_member_translation_zero",
        clash: 0.01,
        lever: None,
    };
    assert_f6(
        &pair,
        &[
            "mates 3 and 5 cannot both hold",
            "a clash of 0.01 m",
            editor_core::CONTRADICTORY_RECOURSE,
        ],
        &["Contradictory"],
    );
    assert!(
        !pair.to_string().contains("contradicts itself"),
        "two distinct mates are a PAIR: {pair}"
    );

    let itself = MateFault::Contradictory {
        held: RecipeNodeId(6),
        added: RecipeNodeId(6),
        predicate: "mate_clocking_redundant",
        clash: core::f64::consts::FRAC_PI_2,
        lever: Some((core::f64::consts::FRAC_PI_2, 1.0)),
    };
    assert_f6(
        &itself,
        &[
            "mate 6 contradicts itself",
            "mate_clocking_redundant",
            editor_core::CONTRADICTORY_RECOURSE,
        ],
        &["Contradictory"],
    );
    assert!(
        !itself.to_string().contains("mates 6 and 6"),
        "one mate at fault is named once: {itself}"
    );
}

/// **A levered clash prints only a product that IS the product.** The
/// metre figure is computed from the two halves the message shows, so
/// the sentence cannot assert an identity the payload failed to keep:
/// a `clash` field that disagrees with its own lever is not what the
/// reader is told.
#[test]
fn a_levered_clash_prints_only_a_product_that_is_the_product() {
    let honest = MateFault::Contradictory {
        held: RecipeNodeId(6),
        added: RecipeNodeId(6),
        predicate: "mate_clocking_redundant",
        clash: core::f64::consts::FRAC_PI_2 * 2.0,
        lever: Some((core::f64::consts::FRAC_PI_2, 2.0)),
    };
    let shown = honest.to_string();
    for want in [
        "a roll of 1.5707963267948966 rad",
        "on a 2 m arm",
        "a deviation of 3.141592653589793 m",
    ] {
        assert!(shown.contains(want), "{shown:?} is missing {want:?}");
    }

    // The same lever, beside a stored figure that is not its product.
    let inconsistent = MateFault::Contradictory {
        held: RecipeNodeId(6),
        added: RecipeNodeId(6),
        predicate: "mate_clocking_redundant",
        clash: 99.0,
        lever: Some((0.25, 2.0)),
    };
    let shown = inconsistent.to_string();
    assert!(
        shown.contains("a deviation of 0.5 m") && !shown.contains("99"),
        "the printed metre figure is the product of the halves shown, never a stored \
         number that disagrees with them: {shown:?}"
    );
}

/// **A non-finite clash that is not the empty set does not claim to
/// be.** The structural refusal is the PREDICATE's fact, so a margin
/// that merely fails to be finite — a NaN, a negative infinity, or an
/// infinity under some other predicate — is reported as the
/// non-measurement it is and never borrows the empty set's sentence.
///
/// The four shapes below are every exit the arm has, and each is
/// checked to end on [`editor_core::CONTRADICTORY_RECOURSE`]: the
/// repair does not depend on which measurement the predicate could
/// report, so no exit may drop it.
#[test]
fn a_non_finite_clash_that_is_not_the_empty_set_does_not_claim_to_be() {
    let empty = MateFault::Contradictory {
        held: RecipeNodeId(3),
        added: RecipeNodeId(5),
        predicate: "mate_member_empty",
        clash: f64::INFINITY,
        lever: None,
    };
    let shown = empty.to_string();
    assert!(
        shown.contains("meet in the empty set") && !shown.contains("inf"),
        "the structural refusal has no metre figure to print: {shown:?}"
    );
    assert!(
        shown.contains(editor_core::CONTRADICTORY_RECOURSE),
        "{shown:?}"
    );

    for (what, clash) in [
        ("NaN", f64::NAN),
        ("-inf", f64::NEG_INFINITY),
        ("+inf", f64::INFINITY),
    ] {
        let fault = MateFault::Contradictory {
            held: RecipeNodeId(3),
            added: RecipeNodeId(5),
            predicate: "mate_member_translation_zero",
            clash,
            lever: None,
        };
        let shown = fault.to_string();
        assert!(
            !shown.contains("empty set"),
            "a {what} margin under another predicate is not the empty set: {shown:?}"
        );
        assert!(
            shown.contains("not a finite length"),
            "a {what} margin says it is no measurement: {shown:?}"
        );
        assert!(
            shown.contains(editor_core::CONTRADICTORY_RECOURSE),
            "{shown:?}"
        );
    }

    // An infinity that carries a lever is still levered: the empty-set
    // sentence must not swallow the halves.
    let levered = MateFault::Contradictory {
        held: RecipeNodeId(6),
        added: RecipeNodeId(6),
        predicate: "mate_clocking_redundant",
        clash: f64::INFINITY,
        lever: Some((core::f64::consts::FRAC_PI_2, 1.0)),
    };
    let shown = levered.to_string();
    assert!(
        shown.contains("a roll of") && !shown.contains("empty set"),
        "a levered clash keeps its halves whatever the stored figure is: {shown:?}"
    );
    assert!(
        shown.contains(editor_core::CONTRADICTORY_RECOURSE),
        "{shown:?}"
    );
}

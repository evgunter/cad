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

use editor_core::mate::SurfaceKind;
use editor_core::{
    AssemblyError, CapEnd, CarriedRefusal, Clash, ClusterMaintenance, ContactClass, DeclareError,
    Diagnosis, Dimension, DimensionError, DocParamValue, DocRef, DocumentId, EditError, EntityKind,
    EvalError, FrameFault, HitTestError, InputFault, InterrogateError, Lever, LeverRefusal,
    Maintenance, MateFault, MateSide, MeasureNodeFault, MeshPickError, MetaVersionError,
    MintRefusal, NamingError, NodeErrorKind, NodePickError, ParamName, ParseError, PartFault,
    PersistError, PlacementRuleFault, ProgramFault, ProvenanceFault, RecipeNodeId,
    RecordedProgramError, RefusedRef, ResolveFault, ResolveIndeterminate, RimShare, RoleSeg,
    RootFault, Route, SelectRefusal, SlotId, SnapshotError, StableName, StepArg, StepSegmentsError,
};
use geom_core::BandError;

/// The gate's refusal over ONE of this document's own mates: the arm
/// carries every row the gather recorded, and a row read here is read
/// through the door that raises it.
fn mint(refusal: MintRefusal) -> AssemblyError {
    AssemblyError::Mint {
        refusals: vec![refusal],
    }
}

/// The `Debug` field-name punctuation **this binary can ban**, which is
/// far short of what its refusal payloads carry.
///
/// Written ONCE for the binary rather than once per suite:
/// `m4_pr4_hit`'s hit-test row goes through the same roster, and the
/// divergence between its old one and this one is what a second
/// spelling cost
/// (`work/view/f6-display-predicate-is-spelled-three-times-with-no-home`).
///
/// **That scope is why the roster is two tokens and not twenty-odd.**
/// It is binary-wide, so a token may be banned only if NO rendering
/// anywhere in the binary opens on it as prose. `index:` is the live
/// counter-example — `MeshPickError::PositionOutOfRange` renders "pick
/// index: triangle … of patch …", which is a sentence — so `index:`
/// is unbannable here although it is a genuine payload field name.
/// Every other field these payloads carry is in the same position
/// until someone checks it against all eight enums' renderings.
///
/// **Which scope is right.** Per-enum, as `mesh` and `topo` spell it:
/// the ban is only as wide as the renderings it must hold against, so a
/// per-enum roster can carry that enum's whole payload vocabulary,
/// while a binary-wide one is bounded by the most prose-like door in
/// the binary. This one stays binary-wide because its eight enums share
/// one wrapper and one suite pair; splitting it is a change to make
/// when a site needs a token this roster cannot hold, not before.
const FIELDS: &[&str] = &["node:", "name:"];

/// [`test_utils::f6::assert_f6`] with this binary's field roster.
pub(crate) fn assert_f6<E: core::fmt::Debug + core::fmt::Display>(
    err: &E,
    wants: &[&str],
    dumps: &[&str],
) {
    test_utils::f6::assert_f6(err, wants, dumps, FIELDS);
}

/// [`test_utils::f6::assert_f6_every_variant`] with this binary's
/// field roster. The weld itself — what the census guarantees, and
/// what it still does not weld — is documented there, where the
/// mechanism is, rather than restated per adopting suite.
pub(crate) fn assert_f6_every_variant<E: core::fmt::Debug + core::fmt::Display>(
    cases: &[(E, Vec<&str>)],
    census: &test_utils::f6::VariantCensus<E>,
    also_banned: &[&str],
) {
    test_utils::f6::assert_f6_every_variant(cases, census, also_banned, FIELDS);
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

test_utils::f6_variants! {
    /// `NodePickError`'s census: one ident per variant, feeding both
    /// the wildcard-free `match` rustc checks and the identifier
    /// roster the weld compares against the rendered cases. A variant
    /// added to the enum stops this file compiling, and writing it here
    /// is writing it into the roster, so it then reds until it has a
    /// case. What the weld does NOT hold is documented on
    /// [`test_utils::f6::assert_f6_every_variant`].
    const NODE_PICK_ERROR: NodePickError =
        [Standing, NotABody, NoSuchBody, Tessellate, Index];
}

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
    assert_f6_every_variant(&cases, &NODE_PICK_ERROR, &[]);
}

test_utils::f6_variants! {
    /// `ResolveIndeterminate`'s census — see [`NODE_PICK_ERROR`].
    const RESOLVE_INDETERMINATE: ResolveIndeterminate =
        [TargetFailed, TargetPoisoned, TargetNotEvaluated];
}

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
    assert_f6_every_variant(&cases, &RESOLVE_INDETERMINATE, &[]);
}

test_utils::f6_variants! {
    /// `DeclareError`'s census — see [`NODE_PICK_ERROR`].
    const DECLARE_ERROR: DeclareError = [NoFindings, Edit, NoMintedId];
}

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
    assert_f6_every_variant(&cases, &DECLARE_ERROR, &[]);
}

test_utils::f6_variants! {
    /// `InterrogateError`'s census — see [`NODE_PICK_ERROR`].
    ///
    /// `pub(crate)` because `lib_u5_interrogate` welds the ladder rungs
    /// it drives against this same roster. A census of its own there
    /// would be a second list of the same enum's identifiers: rustc
    /// keeps both in step, so they could not drift, but the enum has
    /// one roster in this binary and this is it.
    pub(crate) const INTERROGATE_ERROR: InterrogateError = [
        NodeNotEvaluated,
        NodeFailed,
        NodePoisoned,
        NoSuchName,
        Ambiguous,
        WrongKind,
        WholeBody,
        NoBodies,
        NoSuchBody,
        Readback,
    ];
}

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
            vec!["scaffolding", "tier 2 refuses at rest"],
        ),
    ];
    assert_f6_every_variant(&cases, &INTERROGATE_ERROR, &[]);
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

/// `SelectRefusal`'s census, built by hand rather than by
/// [`test_utils::f6_variants!`] — the ONE site in this tree that
/// cannot use the macro, for the reason stated on
/// [`select_refusal_is_exhaustive`]: the macro writes a wildcard-free
/// `match`, which a `#[non_exhaustive]` enum from another crate does
/// not permit. So the token and the roster are two spellings here, as
/// they were everywhere before, and only a rename or a removal is
/// rustc's.
const SELECT_REFUSAL: test_utils::f6::VariantCensus<SelectRefusal> =
    test_utils::f6::VariantCensus::hand_written(
        select_refusal_is_exhaustive,
        &[
            "InBand",
            "TiedDisagrees",
            "Unreadable",
            "NotADatum",
            "NotALength",
            "PairInBand",
            "BadValue",
            "Band",
        ],
    );

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
            vec!["ambiguity band", "ambient tolerance", "not below"],
        ),
    ];
    assert_f6_every_variant(&cases, &SELECT_REFUSAL, &also_banned);
}

test_utils::f6_variants! {
    /// `ResolveFault`'s census — see [`NODE_PICK_ERROR`].
    const RESOLVE_FAULT: ResolveFault = [PinMismatch, EpsilonSeam, Unresolved];
}

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
    assert_f6_every_variant(&cases, &RESOLVE_FAULT, &[]);
}

test_utils::f6_variants! {
    /// `ParseError`'s census — see [`NODE_PICK_ERROR`].
    const PARSE_ERROR: ParseError = [
        UnexpectedChar,
        UnexpectedEnd,
        UnexpectedToken,
        TrailingInput,
        MalformedNumber,
        IntegerOverflow,
        UnknownUnit,
        UnknownFunction,
        WrongArity,
        UnknownParam,
        Dimension,
    ];
}

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
    assert_f6_every_variant(&cases, &PARSE_ERROR, &[]);
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
        &EditError::PayloadDocParamDimension {
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
        &EditError::SlotDocParamDimension {
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
    assert_f6(
        &EditError::DocParamCountHasNoDistribution { name: name.clone() },
        &["is a count", "structural parameter", "no distribution"],
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

    // The load door's checker. Its slot arm spells the slot address
    // out, so only the dimensions are at issue there.
    assert_f6(
        &SnapshotError::SlotDimension {
            node: RecipeNodeId(5),
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
        &SnapshotError::SlotDimension {
            node: RecipeNodeId(5),
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
            measured: Dimension::Length,
            bound: Dimension::Angle,
        },
        &["bounds a length measure", "with an angle expression"],
        &dumps,
    );
    assert_f6(
        &SnapshotError::AssertionTarget {
            node: RecipeNodeId(5),
            measure: RecipeNodeId(4),
            bound: Dimension::Count,
        },
        &["carries a count bound", "which is not a measure"],
        &dumps,
    );
}

test_utils::f6_variants! {
    /// `SnapshotError`'s census — see [`NODE_PICK_ERROR`]. This is the
    /// load door's whole persisted-refusal vocabulary, so a new
    /// invariant that earns an arm earns a rendered case with it.
    const SNAPSHOT_ERROR: SnapshotError = [
        OrderMismatch,
        IdBeyondCounter,
        DanglingInput,
        ForwardInput,
        DeclareInput,
        WitnessSite,
        WitnessOnMissingNode,
        SlotDimension,
        SlotUnknownDocParam,
        SlotDocParamDimension,
        PayloadUnknownDocParam,
        PayloadDocParamDimension,
        EpsilonInvalid,
        Roots,
        PlacementSite,
        PlacementNonFinite,
        PlacementImproper,
        PlacementNotGauge,
        MateAlignment,
        PlacementRule,
        MeasureRefs,
        InputList,
        AssertionTarget,
        AssertionBound,
        MetadataUnversioned,
    ];
}

/// Every arm of the persistence door's snapshot vocabulary states what
/// is wrong with the document and where, and none of them reads as the
/// `Debug` dump.
///
/// The payload-carrying arms forward their payload's own `Display`
/// (`RootFault`, `PlacementRuleFault`, `MeasureNodeFault`,
/// `InputFault`, `MetaVersionError`) rather than restating it, and the
/// two placement-frame arms forward the frame rule's clause — so each
/// case below asks for the payload's words, which is what proves the
/// forwarding happened.
#[test]
fn snapshot_error_display_names_its_content_not_its_struct() {
    let node = RecipeNodeId(5);
    let cases = [
        (
            SnapshotError::OrderMismatch,
            vec!["`order` list", "disagree"],
        ),
        (
            SnapshotError::IdBeyondCounter {
                id: node,
                next_id: 4,
            },
            vec!["node id 5", "mint counter 4"],
        ),
        (
            SnapshotError::DanglingInput {
                node,
                input: RecipeNodeId(9),
            },
            vec!["node 5", "node 9", "not live"],
        ),
        (
            SnapshotError::ForwardInput {
                node,
                input: RecipeNodeId(9),
            },
            vec!["node 5", "does not precede it"],
        ),
        (
            SnapshotError::DeclareInput {
                node,
                input: RecipeNodeId(9),
            },
            vec!["declare input", "not a declaration"],
        ),
        (
            SnapshotError::WitnessSite { node },
            vec!["a witness is attached to node 5", "bears no sketch"],
        ),
        (
            SnapshotError::WitnessOnMissingNode { node },
            vec!["a witness is attached to node 5", "not live"],
        ),
        (
            SnapshotError::SlotDimension {
                node,
                slot: SlotId::Distance,
                expected: Dimension::Length,
                found: Dimension::Angle,
            },
            vec!["node 5", "slot distance", "needs a length expression"],
        ),
        (
            SnapshotError::SlotUnknownDocParam {
                node,
                slot: SlotId::Radius,
                name: ParamName::new("fillet"),
            },
            vec!["slot radius", "fillet", "does not declare"],
        ),
        (
            SnapshotError::SlotDocParamDimension {
                node,
                slot: SlotId::Distance,
                name: ParamName::new("depth"),
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
            vec!["depth", "as a length", "declared angle"],
        ),
        (
            SnapshotError::PayloadUnknownDocParam {
                node,
                name: ParamName::new("depth"),
            },
            vec!["node 5", "payload expression", "depth", "does not declare"],
        ),
        (
            SnapshotError::PayloadDocParamDimension {
                node,
                name: ParamName::new("depth"),
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
            vec![
                // The NODE, which is this arm's whole reason for
                // existing beside the slot one: a payload expression
                // has no slot, so the node is the only address the
                // refusal can carry.
                "node 5",
                "payload expression",
                "depth",
                "as a length",
                "declared angle",
            ],
        ),
        (
            SnapshotError::EpsilonInvalid { value: 0.0 },
            vec!["recorded ε", "finite and strictly positive"],
        ),
        (
            SnapshotError::Roots(RootFault::Ancestor {
                ancestor: RecipeNodeId(1),
                descendant: RecipeNodeId(2),
            }),
            vec!["product root"],
        ),
        (
            SnapshotError::PlacementSite { node },
            vec!["keyed by node 5", "does not instantiate a part"],
        ),
        (
            SnapshotError::PlacementNonFinite { node },
            vec!["placement frame on node 5", "non-finite coordinate"],
        ),
        (
            SnapshotError::PlacementImproper {
                node,
                determinant: -1.0,
            },
            vec!["placement frame on node 5", "improper (mirroring)"],
        ),
        (
            SnapshotError::PlacementNotGauge {
                node,
                gauge: RecipeNodeId(2),
            },
            vec!["cluster's gauge, node 2"],
        ),
        (
            SnapshotError::MateAlignment { node },
            vec!["mate node 5", "alignment datum", "non-finite coordinate"],
        ),
        (
            SnapshotError::PlacementRule {
                node,
                fault: PlacementRuleFault::NoPlacements,
            },
            vec!["placement-rule node 5", "placement list is empty"],
        ),
        (
            SnapshotError::MeasureRefs {
                node,
                fault: MeasureNodeFault::RefIndexOutOfRange {
                    verb: "distance",
                    index: 3,
                    refs: 2,
                },
            },
            vec!["measure node 5", "reads reference 3"],
        ),
        (
            SnapshotError::InputList {
                node,
                fault: InputFault::TooFew { found: 1 },
            },
            vec!["node 5"],
        ),
        (
            SnapshotError::AssertionTarget {
                node,
                measure: RecipeNodeId(4),
                bound: Dimension::Count,
            },
            vec!["carries a count bound", "which is not a measure"],
        ),
        (
            SnapshotError::AssertionBound {
                node,
                measure: RecipeNodeId(4),
                measured: Dimension::Length,
                bound: Dimension::Angle,
            },
            vec!["bounds a length measure", "with an angle expression"],
        ),
        (
            SnapshotError::MetadataUnversioned {
                name: StableName {
                    kind: EntityKind::Face,
                    node,
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                },
                key: "swatch".to_string(),
                error: MetaVersionError::MissingVersion,
            },
            vec!["metadata", "swatch", "\"v\" version field"],
        ),
    ];
    assert_f6_every_variant(&cases, &SNAPSHOT_ERROR, &[]);
}

/// **The param-ref convention, measured in both halves** — the four
/// names at each door, and the MAPPING of name to address.
///
/// The rule the two doors follow is stated once, on `EditError`'s own
/// enum doc; this row is its guard and states none of it again.
///
/// **Half one, the set.** The eight names are read back off `Debug`,
/// the one runtime value that carries them, and each door's four are
/// compared with the `{address} x {fact}` product written as a product
/// rather than as a third list of four names. A door that re-mints a
/// name of its own reds here, and so does a rename applied at one door
/// only.
///
/// **Half two, the mapping.** A set has no opinion about WHICH arm
/// carries which member, so half one alone survives swapping the edit
/// door's slot pair with its payload pair — measured: that swap leaves
/// every row in this file green. Half two ties each name to its
/// address through the one place the address is externally visible,
/// the rendered sentence, which names a slot at a slot arm and says
/// "payload expression" at a payload arm, at both doors. That is what
/// makes `{Slot,Payload}` a convention rather than four interchangeable
/// tokens spelled the same at both doors.
#[test]
fn the_two_doors_spell_the_four_param_ref_refusals_the_same_way_and_each_reports_its_address() {
    /// The variant identifier a `Debug` dump opens with, up to the
    /// first byte that cannot be part of one, paired with what the arm
    /// renders. `Debug` carries the name and `Display` carries the
    /// address, and both halves below read this one pair.
    fn arm<T: core::fmt::Debug + core::fmt::Display>(value: &T) -> (String, String) {
        let variant = format!("{value:?}")
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        (variant, value.to_string())
    }

    let node = RecipeNodeId(5);
    let name = ParamName::new("width");

    let edit_door: Vec<(String, String)> = vec![
        arm(&EditError::SlotUnknownDocParam {
            name: name.clone(),
            node,
            slot: SlotId::Radius,
        }),
        arm(&EditError::SlotDocParamDimension {
            name: name.clone(),
            node,
            slot: SlotId::Radius,
            declared: Dimension::Length,
            referenced: Dimension::Angle,
        }),
        arm(&EditError::PayloadUnknownDocParam {
            name: name.clone(),
            node,
        }),
        arm(&EditError::PayloadDocParamDimension {
            name: name.clone(),
            node,
            declared: Dimension::Length,
            referenced: Dimension::Angle,
        }),
    ];
    let load_door: Vec<(String, String)> = vec![
        arm(&SnapshotError::SlotUnknownDocParam {
            node,
            slot: SlotId::Radius,
            name: name.clone(),
        }),
        arm(&SnapshotError::SlotDocParamDimension {
            node,
            slot: SlotId::Radius,
            name: name.clone(),
            declared: Dimension::Length,
            referenced: Dimension::Angle,
        }),
        arm(&SnapshotError::PayloadUnknownDocParam {
            node,
            name: name.clone(),
        }),
        arm(&SnapshotError::PayloadDocParamDimension {
            node,
            name,
            declared: Dimension::Length,
            referenced: Dimension::Angle,
        }),
    ];

    let mut convention: Vec<String> = ["Slot", "Payload"]
        .into_iter()
        .flat_map(|address| {
            ["UnknownDocParam", "DocParamDimension"]
                .into_iter()
                .map(move |fact| format!("{address}{fact}"))
        })
        .collect();
    convention.sort();

    let names_of = |arms: &[(String, String)]| {
        let mut names: Vec<String> = arms.iter().map(|(variant, _)| variant.clone()).collect();
        names.sort();
        names
    };
    assert_eq!(
        names_of(&edit_door),
        convention,
        "the edit door's four param-ref refusals have left the address-then-fact convention"
    );
    assert_eq!(
        names_of(&load_door),
        convention,
        "the load door's four param-ref refusals have left the address-then-fact convention"
    );

    for (door, arms) in [("edit", &edit_door), ("load", &load_door)] {
        for (variant, rendered) in arms {
            let says_slot = rendered.contains("slot ");
            let says_payload = rendered.contains("payload expression");
            if let Some(fact) = variant.strip_prefix("Slot") {
                assert!(
                    says_slot && !says_payload,
                    "the {door} door's {variant} claims a SLOT address (fact {fact}) but renders \
                     {rendered:?}"
                );
            } else if let Some(fact) = variant.strip_prefix("Payload") {
                assert!(
                    says_payload && !says_slot,
                    "the {door} door's {variant} claims a PAYLOAD address (fact {fact}) but \
                     renders {rendered:?}"
                );
            } else {
                panic!("{door} door: {variant} does not open with an address word");
            }
        }
    }
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
            source: editor_core::FlipSource::VerdictLog,
        },
        &["name_frag_side_of", "flipped from positive to negative"],
        // Every `Sign`, not the two this row happens to construct: a
        // rendering that leaked `Zero` would be just as much a dump.
        // `PredicateFlip` is `Diagnosis`'s own identifier, and this row
        // renders that one arm.
        &[sign_words.as_slice(), &["PredicateFlip"]].concat(),
    );
}

/// The RECOVERED flip says so, and names the partner through the
/// stable name's own `Display` — the two halves a reader needs to know
/// that this flip is in no log they could go and check, and which pair
/// it is about.
#[test]
fn a_recovered_predicate_flip_names_its_partner_and_says_it_was_recovered() {
    let sign_debug: Vec<String> = all_signs().iter().map(|s| format!("{s:?}")).collect();
    let sign_words = as_strs(&sign_debug);
    assert_f6(
        &Diagnosis::PredicateFlip {
            predicate: "name_frag_side_of",
            from: geom_core::predicate::Sign::Positive,
            to: geom_core::predicate::Sign::Negative,
            source: editor_core::FlipSource::ShadowExec {
                partner: Box::new(face_name()),
            },
        },
        &[
            "name_frag_side_of",
            "flipped from positive to negative",
            &face_name().to_string(),
            "recovered by re-running the pair at diagnosis time",
            "one of the two runs recorded no side verdict at the name's minting node",
        ],
        &[
            sign_words.as_slice(),
            &["PredicateFlip", "ShadowExec", "FlipSource"],
        ]
        .concat(),
    );
}

/// The shadow rung's REFUSAL states what stood between the diagnosis
/// and evidence that exists — both arms, because a refusal a reader
/// cannot act on is the fall-through it was written to replace.
#[test]
fn the_shadow_exec_refusal_states_which_wall_it_hit() {
    assert_f6(
        &Diagnosis::ShadowExecDeclined {
            node: RecipeNodeId(7),
            reason: editor_core::ShadowExecRefusal::PairTooWide {
                pairs: 33,
                ceiling: 32,
            },
        },
        &["no side verdict", "minting node", "33", "32", "re-execute"],
        &["ShadowExecDeclined", "PairTooWide", "ShadowExecRefusal"],
    );
    assert_f6(
        &Diagnosis::ShadowExecDeclined {
            node: RecipeNodeId(7),
            reason: editor_core::ShadowExecRefusal::ProbeRefused {
                probe: "a probe's own sentence".to_owned(),
            },
        },
        &["a probe refused", "a probe's own sentence"],
        &["ShadowExecDeclined", "ProbeRefused", "ShadowExecRefusal"],
    );
}

/// The group-size diagnosis states the table fact and nothing more —
/// the same sentence at every count, since a count of one or zero
/// says nothing about where the parent went (N3 merges and undivided
/// pass-throughs are rows the group's spellings do not match). Exact
/// sentences, so a clause that claims more cannot slip in.
#[test]
fn a_resized_group_states_the_table_fact_and_claims_no_flip() {
    for (was, now) in [(2, 1), (3, 0), (2, 3)] {
        let d = Diagnosis::GroupResized {
            node: RecipeNodeId(8),
            was,
            now,
        };
        assert_eq!(
            d.to_string(),
            format!(
                "at node 8, the rows spelled by this fragment's base name, bare or \
                 with one fragment qualifier, held {was} entities in the last-good run \
                 and hold {now} now, and no verdict flip was found that explains the change"
            )
        );
        assert_f6(&d, &[], &["GroupResized"]);
    }
}

/// The two scopes of the with-history lanes say which one answered,
/// in exact sentences: a path arm claims the name's derivation path,
/// and the upstream arm claims only that its node feeds the minting
/// node without being on the path. Exact, so a scope cannot quietly
/// claim the other's relation.
#[test]
fn the_path_and_upstream_scopes_state_which_one_answered() {
    use editor_core::{RecipeEditRef, UpstreamCause};
    use geom_core::predicate::Sign;
    let count = SlotId::Count.label();
    let path = [
        (
            Diagnosis::PredicateFlip {
                predicate: "bool_point_in_solid_plane",
                from: Sign::Negative,
                to: Sign::Positive,
                source: editor_core::FlipSource::VerdictLog,
            },
            "predicate bool_point_in_solid_plane flipped from negative to positive on the \
             name's derivation path"
                .to_owned(),
        ),
        (
            Diagnosis::StructuralParam {
                node: RecipeNodeId(9),
                param: SlotId::Count,
            },
            format!("a structural parameter changed on the derivation path (node 9, slot {count})"),
        ),
        (
            Diagnosis::RecipeEdit {
                edit: RecipeEditRef::NodeDeleted {
                    node: RecipeNodeId(4),
                },
            },
            "the recorded reference disagrees with the recipe as it stands on the derivation \
             path (node 4 was deleted)"
                .to_owned(),
        ),
    ];
    let upstream = |cause| Diagnosis::Upstream {
        node: RecipeNodeId(11),
        cause,
    };
    let tail = ", upstream of node 11, the name's minting node, but not on its derivation path";
    let up = [
        (
            upstream(UpstreamCause::PredicateFlip {
                predicate: "bool_point_in_solid_plane",
                at: RecipeNodeId(10),
                from: Sign::Negative,
                to: Sign::Positive,
            }),
            format!(
                "predicate bool_point_in_solid_plane flipped from negative to positive at node \
                 10{tail}"
            ),
        ),
        (
            upstream(UpstreamCause::StructuralParam {
                node: RecipeNodeId(10),
                param: SlotId::Count,
            }),
            format!("a structural parameter changed at node 10 (slot {count}){tail}"),
        ),
        (
            upstream(UpstreamCause::RecipeEdit {
                edit: RecipeEditRef::NodeDeleted {
                    node: RecipeNodeId(4),
                },
            }),
            format!("the recipe changed (node 4 was deleted){tail}"),
        ),
    ];
    for (d, want) in path.into_iter().chain(up) {
        assert_eq!(d.to_string(), want);
        assert_f6(
            &d,
            &[],
            &[
                "Upstream",
                "UpstreamCause",
                "PredicateFlip",
                "StructuralParam",
                "RecipeEdit",
            ],
        );
    }
}

/// Refusals that name a stable name FORWARD its `Display` rather than
/// re-spelling the kind-plus-minting-node phrase. The expectation is
/// built from the impl, so a copy that stops tracking it fails here —
/// which a literal expectation could not catch.
#[test]
fn refusals_that_name_a_stable_name_forward_its_display() {
    let phrase = face_name().to_string();

    let reference = mint(MintRefusal::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(face_name()),
        why: RefusedRef::Vanished,
    });
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
}

/// The WHY clause of a mate-reference refusal says what the gate
/// checked and no more: a name read below a root names the operand
/// and the rule (a reference resolves against a root's own rows); a
/// tie names its width. Every other row asserting these sentences
/// compares against the impl, so this is their one home.
#[test]
fn a_mate_reference_refusal_says_what_the_gate_checked() {
    let below = mint(MintRefusal::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::B,
        name: Box::new(face_name()),
        why: RefusedRef::ReadBelowARoot {
            at: RecipeNodeId(5),
        },
    });
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

    let tied = mint(MintRefusal::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(face_name()),
        why: RefusedRef::Ambiguous { width: 2 },
    });
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
/// wrong for every edge-kind refusal it can reach.
///
/// Two sentences read it, and they differ in how an edge reaches them.
/// [`editor_core::NotAFaceName`] — what the mate head's one
/// constructor answers — is RAISED with an edge wherever a boundary
/// turns data into a head, so its article is live.
/// `MintRefusal::Reference` renders the NAME's own kind: the gate
/// raises it only over a face name, because a head is a `SitedFace`,
/// but the row is public data with public fields and its article is
/// therefore read off the value a consumer hands it rather than fixed
/// at the raise.
#[test]
fn an_entity_kind_carries_the_article_that_agrees_with_it() {
    let edge_name = StableName {
        kind: EntityKind::Edge,
        node: RecipeNodeId(7),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };

    let shown = editor_core::FaceName::new(edge_name.clone())
        .expect_err("an edge is not a face name")
        .to_string();
    assert!(
        shown.contains("an edge"),
        "an edge-kind head refusal reads as \"a edge\": {shown:?}"
    );
    let shown = editor_core::FaceName::new(StableName {
        kind: EntityKind::Vertex,
        ..edge_name.clone()
    })
    .expect_err("a vertex is not a face name")
    .to_string();
    assert!(
        shown.contains("a vertex"),
        "the consonant kinds must keep \"a\": {shown:?}"
    );

    let reference = mint(MintRefusal::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(edge_name.clone()),
        why: RefusedRef::Vanished,
    });
    let shown = reference.to_string();
    assert!(
        shown.contains(&format!("(an {edge_name})")),
        "an edge-kind mate reference reads as \"a edge\": {shown:?}"
    );

    let face = mint(MintRefusal::Reference {
        mate: RecipeNodeId(2),
        side: MateSide::A,
        name: Box::new(face_name()),
        why: RefusedRef::Vanished,
    });
    let shown = face.to_string();
    assert!(
        shown.contains(&format!("(a {})", face_name())),
        "the consonant kinds must keep \"a\": {shown:?}"
    );
}

/// The mint door's at-rest refusal ends on
/// [`editor_core::NO_AT_REST_RECORD_RECOURSE`] — read as the row's
/// own sentence and read through the gate arm that carries it, because
/// the gate FORWARDS the row rather than restating it. A recourse
/// reached by only one of the two carriers is a user who sees the
/// repair or not depending on how deep the mate was declared.
#[test]
fn the_mint_doors_at_rest_refusal_ends_on_its_recourse_from_both_carriers() {
    let why = editor_core::class_admission(ContactClass::Tangent).no_record_reason();
    let row = MintRefusal::NoAtRestRecord {
        mate: RecipeNodeId(5),
        class: ContactClass::Tangent,
        why,
    };
    assert_f6(
        &row,
        &[
            "mate 5's class Tangent has no at-rest kernel record",
            why,
            editor_core::NO_AT_REST_RECORD_RECOURSE,
        ],
        &["NoAtRestRecord"],
    );
    assert_f6(
        &mint(row),
        &[
            "mate 5's class Tangent has no at-rest kernel record",
            why,
            editor_core::NO_AT_REST_RECORD_RECOURSE,
        ],
        &["NoAtRestRecord"],
    );
}

/// **Every refusal the gate holds is rendered**, each on its own
/// indented line under a header that counts them — the two mint arms
/// answer with the whole list the gather recorded, so an author with
/// two broken mates reads two repairs rather than the first.
///
/// The count is built from the FIXTURE's own length rather than
/// written as a word, so a header that reports a constant, or reports
/// a count off by one, reds here. It is not a guard on the header
/// tracking the body: those are one expression over one slice and
/// cannot drift. What each arm's rows carry — each mate's own
/// sentence, and for the carried arm the route and the ONE recourse in
/// the header — is the rest of it.
#[test]
fn the_mint_arms_render_every_refusal_they_hold() {
    let why = editor_core::class_admission(ContactClass::Tangent).no_record_reason();
    let raised = AssemblyError::Mint {
        refusals: vec![
            MintRefusal::Reference {
                mate: RecipeNodeId(2),
                side: MateSide::A,
                name: Box::new(face_name()),
                why: RefusedRef::Vanished,
            },
            MintRefusal::NoAtRestRecord {
                mate: RecipeNodeId(5),
                class: ContactClass::Tangent,
                why,
            },
        ],
    };
    let AssemblyError::Mint { refusals } = &raised else {
        panic!("built as the mint arm");
    };
    let counted = format!(
        "this document did not mint {} of its own mate(s)",
        refusals.len()
    );
    assert_f6(
        &raised,
        &[
            &counted,
            "mate 2's a reference",
            "mate 5's class Tangent has no at-rest kernel record",
        ],
        &["Reference", "NoAtRestRecord"],
    );

    let route = Route {
        through: RecipeNodeId(1),
        of: editor_core::DocumentId::derive("display-contract-carried"),
        via: vec![],
    };
    let carried = AssemblyError::CarriedMintRefusal {
        refusals: vec![
            CarriedRefusal {
                route: route.clone(),
                refusal: MintRefusal::Reference {
                    mate: RecipeNodeId(2),
                    side: MateSide::A,
                    name: Box::new(face_name()),
                    why: RefusedRef::Vanished,
                },
            },
            CarriedRefusal {
                route,
                refusal: MintRefusal::NoAtRestRecord {
                    mate: RecipeNodeId(5),
                    class: ContactClass::Tangent,
                    why,
                },
            },
        ],
    };
    let AssemblyError::CarriedMintRefusal { refusals } = &carried else {
        panic!("built as the carried arm");
    };
    let counted = format!(
        "{} mate(s) of documents below this one did not mint",
        refusals.len()
    );
    let shown = carried.to_string();
    assert_f6(
        &carried,
        &[
            &counted,
            "mate 2's a reference",
            "mate 5's class Tangent has no at-rest kernel record",
        ],
        &["CarriedMintRefusal", "CarriedRefusal"],
    );
    // ONE recourse for the list, in the header: the repair is the same
    // sentence for every row, and a per-row copy is the generic tail
    // the finding sink exists to forbid.
    assert_eq!(
        shown
            .matches("open those documents and repair the mates there")
            .count(),
        1,
        "the carried repair is stated once, not once per row: {shown:?}"
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
        clash: Clash::Length { metres: 0.01 },
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
        clash: Clash::Levered(Lever::Roll {
            radians: core::f64::consts::FRAC_PI_2,
            arm: 1.0,
        }),
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

/// **A levered clash prints the product of its two halves.** The
/// metre figure is computed from the halves the message shows, and a
/// stored figure that could disagree with them is not representable:
/// `Clash::Levered` holds the lever and nothing beside it.
#[test]
fn a_levered_clash_prints_only_a_product_that_is_the_product() {
    let fault = MateFault::Contradictory {
        held: RecipeNodeId(6),
        added: RecipeNodeId(6),
        predicate: "mate_clocking_redundant",
        clash: Clash::Levered(Lever::Roll {
            radians: core::f64::consts::FRAC_PI_2,
            arm: 2.0,
        }),
    };
    let shown = fault.to_string();
    for want in [
        "a roll of 1.5707963267948966 rad",
        "on a 2 m arm",
        "a deviation of 3.141592653589793 m",
    ] {
        assert!(shown.contains(want), "{shown:?} is missing {want:?}");
    }
    let MateFault::Contradictory { clash, .. } = &fault else {
        unreachable!()
    };
    assert_eq!(
        clash.deviation(),
        Some(core::f64::consts::FRAC_PI_2 * 2.0),
        "the deviation is the product, to the bit"
    );
}

/// **A residual clash names its pure number, says it is one, and
/// prints the product** — the second levered sentence, for the three
/// membership margins that lever a sine, a Frobenius departure or a
/// reach rather than an authored roll. It never borrows the roll's
/// words: a Frobenius norm is not radians, and a reader who multiplies
/// the halves back gets the metre figure.
#[test]
fn a_residual_clash_prints_its_pure_number_and_the_product() {
    let fault = MateFault::Contradictory {
        held: RecipeNodeId(3),
        added: RecipeNodeId(5),
        predicate: "mate_member_rotation_identity",
        clash: Clash::Levered(Lever::Residual {
            value: 0.25,
            arm: 4.0,
        }),
    };
    let shown = fault.to_string();
    for want in [
        "mates 3 and 5 cannot both hold",
        "predicate `mate_member_rotation_identity`",
        "a dimensionless residual of 0.25",
        "on a 4 m arm",
        "a deviation of 1 m",
        editor_core::CONTRADICTORY_RECOURSE,
    ] {
        assert!(shown.contains(want), "{shown:?} is missing {want:?}");
    }
    assert!(
        !shown.contains("rad") && !shown.contains("roll"),
        "a residual is a pure number, never a roll in radians: {shown:?}"
    );
}

/// **A non-finite clash that is not the empty set does not claim to
/// be.** The structural refusal is the TYPE's fact (`Clash::Structural`),
/// so a length that merely fails to be finite — a NaN, a negative
/// infinity, an infinity — is reported as the non-measurement it is
/// and never borrows the empty set's sentence, and a levered clash
/// whose halves are not finite keeps its halves.
///
/// The exits below are every arm the type has, and each is checked
/// to end on [`editor_core::CONTRADICTORY_RECOURSE`]: the repair does
/// not depend on which measurement the predicate could report, so no
/// exit may drop it.
#[test]
fn a_non_finite_clash_that_is_not_the_empty_set_does_not_claim_to_be() {
    let empty = MateFault::Contradictory {
        held: RecipeNodeId(3),
        added: RecipeNodeId(5),
        predicate: "mate_member_empty",
        clash: Clash::Structural,
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
            clash: Clash::Length { metres: clash },
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

    // A lever whose roll is not finite is still levered: the sentence
    // keeps its halves and prints the product they make.
    let levered = MateFault::Contradictory {
        held: RecipeNodeId(6),
        added: RecipeNodeId(6),
        predicate: "mate_clocking_redundant",
        clash: Clash::Levered(Lever::Roll {
            radians: f64::INFINITY,
            arm: 1.0,
        }),
    };
    let shown = levered.to_string();
    assert!(
        shown.contains("a roll of inf rad") && !shown.contains("empty set"),
        "a levered clash keeps its halves whatever they are: {shown:?}"
    );
    assert!(
        shown.contains(editor_core::CONTRADICTORY_RECOURSE),
        "{shown:?}"
    );
}

/// **A lever refusal names the instance and says why its part's reach
/// is not in hand**, one sentence per arm — the resolver's own voice
/// for a part that does not resolve, the face and its kind for one
/// that cannot be bounded, the face for a malformed body, and the
/// plain fact for a faceless body, a non-finite reach, and a member
/// that stands on no instance.
#[test]
fn a_lever_refusal_names_the_instance_and_why() {
    let instance = RecipeNodeId(7);
    let part = DocRef {
        id: DocumentId::derive("display-contract-lever"),
        pin: editor_core::content_pin(
            &editor_core::ProfileDoc::empty(
                DocumentId::derive("display-contract-lever"),
                geom_core::Tol::witness(),
            ),
            geom_core::Tol::witness(),
        )
        .unwrap(),
    };
    let mut body = topo::Body::<f64>::new();
    let face = body
        .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0))
        .unwrap()
        .face;
    assert_f6(
        &LeverRefusal::PartUnresolved {
            instance,
            fault: PartFault::NoResolver,
        },
        &["instance 7", "no part resolver"],
        &["PartUnresolved", "NoResolver"],
    );
    assert_f6(
        &LeverRefusal::FaceUnbounded {
            instance,
            part,
            face,
            kind: SurfaceKind::Nurbs,
        },
        &["instance 7", "nurbs face", "cannot be bounded"],
        &["FaceUnbounded", "Nurbs"],
    );
    assert_f6(
        &LeverRefusal::MalformedBody {
            instance,
            part,
            face,
        },
        &["instance 7", "not well formed"],
        &["MalformedBody"],
    );
    assert_f6(
        &LeverRefusal::NoExtent { instance, part },
        &["instance 7", "no faces"],
        &["NoExtent"],
    );
    assert_f6(
        &LeverRefusal::NoFiniteBound { instance, part },
        &["instance 7", "non-finite"],
        &["NoFiniteBound"],
    );
    assert_f6(
        &LeverRefusal::NotAnInstance { node: instance },
        &["node 7", "not a live instantiate node"],
        &["NotAnInstance"],
    );
}

/// **The maintenance refusals name the gauge and end on what to do**:
/// a refused maintenance solve carries the prior solve's own sentence
/// (or says the solve recorded nothing for the gauge — the typed
/// report of a state its invariants exclude), and an unrecorded row
/// says the entry carries none and that an entry records every row its
/// edit performs.
#[test]
fn the_maintenance_refusals_name_the_gauge_and_the_recourse() {
    let gauge = RecipeNodeId(3);
    assert_f6(
        &EditError::MaintenanceRefused {
            gauge,
            fault: Some(Box::new(MateFault::Unleverable {
                mate: RecipeNodeId(5),
                refusal: LeverRefusal::PartUnresolved {
                    instance: gauge,
                    fault: PartFault::NoResolver,
                },
            })),
        },
        &["could not place gauge 3", "refused: mate 5", "instance 3"],
        &["MaintenanceRefused", "Unleverable"],
    );
    assert_f6(
        &EditError::MaintenanceRefused { gauge, fault: None },
        &["could not place gauge 3", "no pose", "no fault"],
        &["MaintenanceRefused"],
    );
    assert_f6(
        &EditError::MaintenanceUnrecorded { gauge },
        &[
            "gauge 3",
            "no maintenance rows",
            "records every cluster row",
        ],
        &["MaintenanceUnrecorded"],
    );
    assert_f6(
        &PersistError::MaintenanceFrame {
            index: 2,
            row: 0,
            fault: FrameFault::Improper { determinant: -1.0 },
        },
        &[
            "edit 2",
            "row 0",
            "not a placement",
            "determinant -1",
            "mirroring",
        ],
        &["MaintenanceFrame", "Improper"],
    );
    assert_f6(
        &PersistError::MaintenanceFrame {
            index: 2,
            row: 1,
            fault: FrameFault::NonFinite,
        },
        &["edit 2", "row 1", "non-finite coordinate"],
        &["MaintenanceFrame", "NonFinite"],
    );
}

test_utils::f6_variants! {
    /// `NamingError`'s census: one ident per variant, feeding both the
    /// wildcard-free `match` rustc checks and the identifier roster the
    /// weld compares against the rendered cases.
    ///
    /// This enum's sentences are checked in its own crate
    /// (`names::emit`'s `display_tests`), which is where the framing a
    /// variant must open with lives. What that suite cannot do is
    /// notice a variant with no sample at all: its coverage check
    /// compares sampled indices against `0..rows.len()`, which a
    /// variant APPENDED past the end satisfies — measured, by adding a
    /// probe variant and watching it stay green. This census is what
    /// closes that.
    const NAMING_ERROR: NamingError = [
        Duplicate,
        Unnamed,
        MissingUpstream,
        Emission,
        SplitLineage,
        FragmentLineage,
        SeamVertexParentage,
        SeamVertexPartners,
        SharedRim,
        MergedChord,
        MergedChordOffRim,
        SeamLineSides,
        Band,
        Escalated,
    ];
}

/// Two distinct keys of each kind, out of ONE real arena — slotmap keys
/// have no hand constructor, and two bodies hand out the same index
/// twice. Nothing below depends on their values.
fn keys() -> (topo::EdgeKey, topo::FaceKey, topo::VertexKey) {
    let mut body = topo::Body::<f64>::new();
    let born = body
        .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0))
        .expect("mvfs births a solid, shell, face and lone vertex");
    let edge = body
        .mev_line(
            topo::MevSite::Lone {
                r#loop: born.r#loop,
            },
            geom_core::Point3::new(1.0, 0.0, 0.0),
            geom_core::Tol::witness(),
        )
        .expect("mev_line adds an edge")
        .edge;
    (edge, born.face, born.vertex)
}

/// **Every `NamingError` renders its subject, and no variant escapes
/// the census.**
///
/// The emitter's refusals are the one route by which a naming failure
/// reaches a human (Python's typed exception text is exactly this
/// string), and this crate's own `display_tests` check what each
/// SENTENCE says — which framing it opens with, that it leaks no braced
/// payload, that it carries its subject. What they cannot check is that
/// every variant has a sample at all. This does, and the two live
/// together rather than one replacing the other.
#[test]
fn naming_error_display_names_its_content_not_its_struct() {
    let (edge, face, vertex) = keys();
    let cases = [
        (
            NamingError::Duplicate {
                name: Box::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId(7),
                    path: vec![RoleSeg::Cap(CapEnd::End)],
                }),
            },
            vec!["minted twice"],
        ),
        (
            NamingError::Unnamed {
                kind: EntityKind::Edge,
                body: 3,
            },
            vec!["edge", "output body 3", "unnamed"],
        ),
        (
            NamingError::MissingUpstream {
                node: RecipeNodeId(11),
            },
            vec!["upstream node 11"],
        ),
        (
            NamingError::Emission {
                what: "section face classified On",
            },
            vec!["section face classified On"],
        ),
        (
            NamingError::SplitLineage(topo::SplitLineageCycle { edge }),
            vec!["split lineage of edge"],
        ),
        (
            NamingError::FragmentLineage { face },
            vec!["fragment lineage of face"],
        ),
        (
            NamingError::SeamVertexParentage { vertex },
            vec!["seam vertex", "half-decided"],
        ),
        (
            NamingError::SharedRim {
                node: RecipeNodeId(23),
                face,
                other: face,
                found: RimShare::Several,
            },
            vec!["operand node 23", "more than one edge"],
        ),
        (
            NamingError::MergedChord { edge },
            vec!["merged faces", "the join's own edge"],
        ),
        (
            NamingError::SeamVertexPartners {
                vertex,
                candidates: Vec::new(),
            },
            vec!["seam vertex", "differently named vertices"],
        ),
        (
            NamingError::MergedChordOffRim {
                edge,
                node: RecipeNodeId(29),
                rim: edge,
            },
            vec!["merged faces", "operand node 29", "does not lie within"],
        ),
        (
            NamingError::SeamLineSides {
                node: RecipeNodeId(31),
                edge,
            },
            vec!["node 31", "each side of its recorded pair"],
        ),
        (
            NamingError::Band(BandError::Empty {
                zero: 5e-324,
                escalate: 5e-324,
            }),
            vec!["naming band", "5e-324"],
        ),
        (
            NamingError::Escalated {
                predicate: "side_of_plane",
                source: geom_core::Indeterminate {
                    margin: geom_core::predicate::MarginDiag::Invalid,
                    band: geom_core::Band::new(1e-9, 1e-6).expect("a valid band"),
                    predicate: Some("side_of_plane"),
                },
            },
            vec!["side_of_plane", "escalated"],
        ),
    ];
    assert_f6_every_variant(&cases, &NAMING_ERROR, &[]);
}

test_utils::f6_variants! {
    /// `ProgramFault`'s census — see [`NODE_PICK_ERROR`]. The load
    /// door's own refusal over a persisted profile program. A step
    /// argument's DIMENSION is not here: a program slot is a slot like
    /// any other, refused by the document-wide slot walk
    /// ([`SnapshotError::SlotDimension`]), so what is left is the
    /// replay probe's lattice coordinate.
    const PROGRAM_FAULT: ProgramFault = [Lattice];
}

/// **A slot refusal addresses its slot in the slot vocabulary's own
/// words** ([`SlotId::label`], [`StepArg::label`]), not in the enum's
/// — for every slot address alike, because one predicate decides them
/// (`Node::slot_dimension_fault`) and one arm renders them.
///
/// **And the sentence is ONE clause.** The rule's own answer carries
/// its `Display` (`SlotDimensionFault`), and each door forwards it
/// into its own subject, so the last case below reads the load door's
/// rendering as the edit door's under "node 7: ". A door that
/// restated the sentence — as the two of them did, three times over,
/// with a program slot spelled two ways — reds there.
///
/// **What the ban list holds.** What a reverted arm would leak is a
/// `SlotId` or a `StepArg` identifier. Those are read off the very
/// values the cases carry ([`test_utils::f6::variant_identifier`]) so
/// a variant renamed in `src/` cannot leave this list saying the old
/// name. `SlotId::Profile` carries a brace in its `Debug` as well, so
/// that arm is held twice.
#[test]
fn a_slot_refusal_addresses_its_slot_in_the_slot_vocabulary() {
    let profile_slot = SlotId::Profile {
        loop_: 1,
        step: 3,
        arg: StepArg::CenterX,
    };
    let scalar_slot = SlotId::Radius;
    let component_slot = SlotId::Origin(editor_core::Axis3::X);
    let banned = [
        test_utils::f6::variant_identifier(&profile_slot),
        test_utils::f6::variant_identifier(&StepArg::CenterX),
        test_utils::f6::variant_identifier(&scalar_slot),
        test_utils::f6::variant_identifier(&component_slot),
        "{".to_string(),
    ];
    let also_banned = as_strs(&banned);
    let node = RecipeNodeId(7);

    assert_f6(
        &SnapshotError::SlotDimension {
            node,
            slot: profile_slot,
            expected: Dimension::Length,
            found: Dimension::Angle,
        },
        &[
            "node 7",
            "loop 1 step 3 · centre x",
            "needs a length expression",
            "got an angle",
        ],
        &also_banned,
    );
    assert_f6(
        &SnapshotError::SlotDimension {
            node,
            slot: scalar_slot,
            expected: Dimension::Length,
            found: Dimension::Count,
        },
        &["slot radius", "needs a length expression", "got a count"],
        &also_banned,
    );
    assert_f6(
        &SnapshotError::SlotDimension {
            node,
            slot: component_slot,
            expected: Dimension::Length,
            found: Dimension::Scalar,
        },
        &["slot origin x", "got a scalar"],
        &also_banned,
    );
    // One clause, two subjects: whatever the sentence says, the two
    // doors say it in the same words about the same address.
    for slot in [profile_slot, scalar_slot, component_slot] {
        let at_load = SnapshotError::SlotDimension {
            node,
            slot,
            expected: Dimension::Length,
            found: Dimension::Angle,
        };
        let at_edit = EditError::SlotDimensionMismatch {
            slot,
            expected: Dimension::Length,
            found: Dimension::Angle,
        };
        assert_eq!(at_load.to_string(), format!("node 7: {at_edit}"));
    }
}

/// A program fault states the transition table's coordinate, not its
/// struct.
///
/// **What it deliberately does not ban.** The `Lattice` arm renders its
/// tip state and verb through `Debug`: the pair is the transition
/// table's own coordinate, which `profile`'s `ReplayError` and this
/// arm's comment both say, and `Verb`'s `Display` — the authoring
/// spelling — is a different sentence from the coordinate. Banning
/// those identifiers here would be this suite deciding a question
/// settled the other way beside the code.
#[test]
fn a_program_fault_states_its_lattice_coordinate() {
    let cases = [(
        ProgramFault::Lattice {
            loop_: 0,
            step: 2,
            state: profile::TipState::Entry,
            verb: None,
        },
        vec![
            "loop 0 step 2",
            "not a legal chain-lattice walk",
            "unclosed",
        ],
    )];
    assert_f6_every_variant(&cases, &PROGRAM_FAULT, &[]);
}

test_utils::f6_variants! {
    /// `ClusterMaintenance`'s census — see [`NODE_PICK_ERROR`]. The
    /// four registry acts render beside their own type, so this is the
    /// list that guards them; `Maintenance` delegates and carries only
    /// its own two arms.
    const CLUSTER_MAINTENANCE: ClusterMaintenance = [Join, Split, GaugeRewrite, Drop];
}

test_utils::f6_variants! {
    /// `Maintenance`'s census — see [`NODE_PICK_ERROR`].
    const MAINTENANCE: Maintenance = [Cluster, Strand, StrandedAppearance, OrphanedDeclare, Rebound];
}

/// **Each registry act says what it did to the placement registry.**
/// The maintenance column is rendered to a person, so these are prose
/// and not the `Debug` dump of a frame.
#[test]
fn cluster_maintenance_display_names_the_act_not_its_struct() {
    let gauge = RecipeNodeId(3);
    let other = RecipeNodeId(5);
    let cases = [
        (
            ClusterMaintenance::Join {
                survived: gauge,
                absorbed: other,
                absorbed_frame: None,
            },
            vec!["cluster gauged by node 5", "absorbed into", "node 3"],
        ),
        (
            ClusterMaintenance::Split {
                from: gauge,
                to: other,
                frame: None,
            },
            vec!["separated from", "node 3", "now gauged by node 5"],
        ),
        (
            ClusterMaintenance::GaugeRewrite {
                from: gauge,
                to: other,
                frame: None,
            },
            vec!["node 3", "lost that instance", "now gauged by node 5"],
        ),
        (
            ClusterMaintenance::Drop { gauge, frame: None },
            vec!["node 3", "lost its last instance", "placement record"],
        ),
    ];
    assert_f6_every_variant(&cases, &CLUSTER_MAINTENANCE, &[]);
}

/// **What an accepted edit DID reads as prose too** — the strand count
/// beside the cascade count is rendered from these sentences.
///
/// The cluster arm FORWARDS its carried act's own words (the
/// `NodePickError::Standing` shape one row up), which is why its case
/// here asserts the delegated sentence rather than a paraphrase of it.
/// The strand sentence's relative clause binds to the NODE: the name
/// is what survives a strand, so a sentence reading "a name, which
/// this edit deleted" would name the wrong casualty.
/// The appearance arm names the STORE where the payload arm names a
/// carrying node, because that is the difference between the two
/// carriers, and it offers both repairs: `Rebind` moves the key,
/// `ClearAppearance` retires it, and only the second works without a
/// live node to move to.
/// The orphan arm's subject is the SURVIVOR — the node named is the
/// declaration that is still there — where both strand sentences name
/// a carrier and close on the casualty, so it says what the
/// declaration lost (its reader) rather than what was deleted.
#[test]
fn maintenance_display_says_what_the_edit_did() {
    let gauge = RecipeNodeId(3);
    let other = RecipeNodeId(5);
    let cases = [
        (
            Maintenance::Cluster(ClusterMaintenance::Join {
                survived: gauge,
                absorbed: other,
                absorbed_frame: None,
            }),
            vec!["cluster gauged by node 5", "absorbed into", "node 3"],
        ),
        (
            Maintenance::Strand {
                node: other,
                name: face_name(),
            },
            vec![
                "node 5 carries a face name minted by node 7",
                // The row is made by two edits — a delete and a
                // reshaping — and the sentence names what either
                // removed without claiming which.
                "this edit removed what it denoted",
                "its minting node, or the profile segment it named",
                "resolves to nothing until it is rebound",
            ],
        ),
        (
            Maintenance::StrandedAppearance { name: face_name() },
            vec![
                "the appearance store holds an attachment under a face name minted by node 7",
                "this edit removed what it denoted",
                "rebound or cleared",
            ],
        ),
        (
            Maintenance::Rebound {
                from: face_name(),
                to: face_name(),
            },
            vec![
                "a face name minted by node 7 was rewritten in place",
                "draws the same step's segment under the reshaped profile program",
                "still denotes what it did",
            ],
        ),
        (
            Maintenance::OrphanedDeclare { declare: other },
            vec![
                "node 5 declares contacts",
                "deleted the last node that consumed it",
                // What it lost is a CONSUMER. "nothing reads it"
                // would be false — the same delete re-roots the
                // declaration into the document's product roots.
                "so no node consumes the declaration",
                "until a boolean or union names it again",
            ],
        ),
    ];
    assert_f6_every_variant(&cases, &MAINTENANCE, &[]);
}

test_utils::f6_variants! {
    /// `RecordedProgramError`'s census — see [`NODE_PICK_ERROR`].
    /// Five arms: three are about the RECORDING the lift was handed,
    /// two about the notation written beside it, and a consumer
    /// telling those apart is the point of them being five arms
    /// rather than one. One of the five — `NotationBeforeAnyStep` —
    /// is raised at the WRITING door rather than at the lift, so this
    /// census covers a sentence a caller can read without ever
    /// lifting anything.
    const RECORDED_PROGRAM_ERROR: RecordedProgramError = [
        Literal,
        SubdivisionCount,
        CarrierInChain,
        NotationOffProgram,
        NotationBeforeAnyStep,
    ];
}

/// **Every recorded-program refusal states what the lift could not
/// take**, including the two that are about the notation rather than
/// the recording.
///
/// The two notation arms are the pair a caller has to tell apart: one
/// says the entry names an argument the recording has none of, the
/// other that the derived door was asked to write against nothing.
/// They differ by more than a step number, so the sentences differ by
/// more than a step number.
#[test]
fn a_recorded_program_refusal_says_what_the_lift_could_not_take() {
    let cases = [
        (
            RecordedProgramError::Literal(DimensionError::DisplayUnitMismatch {
                unit: Dimension::Angle,
                literal: Dimension::Length,
            }),
            vec!["recorded literal was refused", "measures angle"],
        ),
        (
            RecordedProgramError::SubdivisionCount(1 << 40),
            vec!["subdivision count", "does not fit a u32"],
        ),
        (
            RecordedProgramError::CarrierInChain,
            vec!["complete-loop carrier step", "inside a chain"],
        ),
        (
            RecordedProgramError::NotationOffProgram {
                step: 3,
                arg: StepArg::TargetX,
            },
            vec!["target x", "step 3", "no argument at"],
        ),
        (
            RecordedProgramError::NotationBeforeAnyStep {
                arg: StepArg::Radius,
            },
            vec!["radius", "step just recorded", "nothing has been recorded"],
        ),
    ];
    assert_f6_every_variant(
        &cases,
        &RECORDED_PROGRAM_ERROR,
        &as_strs(&dimension_dump_words()),
    );
}

test_utils::f6_variants! {
    /// `ProvenanceFault`'s census — see [`NODE_PICK_ERROR`]. The
    /// whole-program edit's shape faults: seven ways a provenance can
    /// fail to describe its program, each naming the coordinate the
    /// caller wrote in the caller's own terms.
    const PROVENANCE_FAULT: ProvenanceFault = [
        LoopCount,
        StepCount,
        NoSuchOldLoop,
        NoSuchOldStep,
        StepOfNewLoop,
        OldLoopContinuedTwice,
        OldStepContinuedTwice,
    ];
}

/// **Every provenance shape fault states the coordinate it is about
/// and the count it was checked against**, in the caller's terms — a
/// NEW loop or step index where the entry sits, an OLD one where it
/// points — and the edit's arm that carries one frames it with the
/// node.
#[test]
fn a_provenance_fault_names_the_coordinate_and_the_count() {
    let cases = [
        (
            ProvenanceFault::LoopCount {
                loops: 2,
                provenance: 3,
            },
            vec!["2 loops", "3 entries", "one entry per loop"],
        ),
        (
            ProvenanceFault::StepCount {
                loop_: 1,
                steps: 5,
                provenance: 4,
            },
            vec!["loop 1 authors 5 steps", "4 entries", "one entry per step"],
        ),
        (
            ProvenanceFault::NoSuchOldLoop {
                loop_: 0,
                from: 3,
                old_loops: 2,
            },
            vec!["loop 0 continues old loop 3", "has 2 loops"],
        ),
        (
            ProvenanceFault::NoSuchOldStep {
                loop_: 0,
                step: 2,
                from: 1,
                old_step: 9,
                old_steps: 5,
            },
            vec![
                "loop 0 step 2 continues old step 9 of old loop 1",
                "authors 5 steps",
            ],
        ),
        (
            ProvenanceFault::StepOfNewLoop {
                loop_: 1,
                step: 0,
                old_step: 4,
            },
            vec![
                "loop 1 is a new loop",
                "step 0 continues old step 4",
                "its steps are all new",
            ],
        ),
        (
            ProvenanceFault::OldLoopContinuedTwice {
                from: 0,
                first: 0,
                again: 1,
            },
            vec!["old loop 0 is continued by loop 0 and again by loop 1"],
        ),
        (
            ProvenanceFault::OldStepContinuedTwice {
                loop_: 0,
                from: 0,
                old_step: 1,
                first: 1,
                again: 2,
            },
            vec![
                "old step 1 of old loop 0 is continued by loop 0's step 1 and again by its \
                 step 2",
            ],
        ),
    ];
    assert_f6_every_variant(&cases, &PROVENANCE_FAULT, &[]);
    assert_f6(
        &EditError::ProvenanceMalformed {
            node: RecipeNodeId(4),
            fault: ProvenanceFault::LoopCount {
                loops: 1,
                provenance: 2,
            },
        },
        &["node 4's program provenance", "1 loops", "2 entries"],
        &["ProvenanceMalformed", "LoopCount"],
    );
    assert_f6(
        &EditError::SetProgramOnNonProfile {
            node: RecipeNodeId(4),
        },
        &["node 4 holds no profile program", "no program to set"],
        &["SetProgramOnNonProfile"],
    );
}

test_utils::f6_variants! {
    /// `StepSegmentsError`'s census — see [`NODE_PICK_ERROR`]. The
    /// step→profile-edge door's refusals (DM8): every arm is a question
    /// the door could not answer, so every arm must say which question
    /// in words a consumer can act on.
    const STEP_SEGMENTS_ERROR: StepSegmentsError =
        [NoSuchLoop, NoSuchStep, NoRecord, RecordShape, NoAnchor, RadiusNotAnArgument,
         EmissionOffTheLoop, CarrierRecordsEmissions, SpanOffTheLoop];
}

#[test]
fn step_segments_error_display_names_its_content_not_its_struct() {
    let cases = [
        (StepSegmentsError::NoSuchLoop { loops: 2 }, vec!["2 loops"]),
        (StepSegmentsError::NoSuchStep { steps: 5 }, vec!["5 steps"]),
        (
            StepSegmentsError::NoRecord { loop_: 1 },
            vec!["structure record", "loop 1"],
        ),
        (
            StepSegmentsError::RecordShape {
                loop_: 1,
                authored: 5,
                recorded: 4,
            },
            vec!["loop 1", "authors 5 steps", "describes 4"],
        ),
        (
            StepSegmentsError::NoAnchor { loop_: 0 },
            vec!["naming anchor", "loop 0"],
        ),
        // The WHOLE sentence, not three substrings of it: every
        // radius-role label already ends in the word "radius"
        // (`StepArg::label`), so a template that appended one of its
        // own rendered "carrier radius radius" and passed a
        // substring census without a murmur.
        (
            StepSegmentsError::RadiusNotAnArgument {
                step: 3,
                arg: editor_core::StepArg::CarrierRadius2,
            },
            vec![
                "the record says step 3's arrival carrier radius drew a segment, \
                 and that step holds no such argument",
            ],
        ),
        (
            StepSegmentsError::EmissionOffTheLoop {
                step: 2,
                arg: editor_core::StepArg::CarrierRadius,
                segment: 9,
                segments: 4,
            },
            vec![
                "the record says step 2's carrier radius drew segment 9 on a loop \
                 with 4 of them",
            ],
        ),
        (
            StepSegmentsError::CarrierRecordsEmissions {
                loop_: 0,
                emissions: 3,
            },
            vec![
                "loop 0 is a carrier form, whose one radius is the whole boundary's \
                 and draws no segment of its own, and its record carries 3 radius \
                 emissions",
            ],
        ),
        (
            StepSegmentsError::SpanOffTheLoop {
                step: 2,
                end: 9,
                segments: 4,
            },
            vec!["step 2", "up to 9", "4 of them"],
        ),
    ];
    assert_f6_every_variant(&cases, &STEP_SEGMENTS_ERROR, &[]);
}

/// **A parameter name renders without quotes at every door but parse.**
///
/// [`ParamName`] is a `String` newtype, so a `{:?}` over it renders the
/// name plus `Debug`'s quotes: prose, but carrying a delimiter the
/// sentence did not ask for, and the crate spelled it both ways. The
/// rule is the one [`ParamName`]'s `Display` carries — a door that
/// FRAMES the name in a sentence of its own ("parameter width is
/// declared length") renders it bare, and the one door that echoes the
/// bytes an author typed, [`ParseError::UnknownParam`], keeps the
/// quotes, because there the delimiter is what says which bytes were
/// read.
///
/// The row is over the rendered SENTENCE rather than over the
/// placeholder, so it holds whether a quote returns as a `{:?}`, as a
/// literal pair written around the name, or through a `Debug` the
/// newtype is given later. One arm per door: the spelling is a property
/// of the door, and an arm added to one of these enums inherits
/// whichever spelling its neighbours use.
///
/// The certified-range and stackup doors are censused by
/// [`a_parameter_name_renders_unquoted_at_the_interval_only_doors`].
#[test]
fn a_parameter_name_renders_unquoted_at_every_door_but_parse() {
    use editor_core::{
        DocParamField, InlineError, MeasureUnavailable, NonFiniteSite, ParamBoxError, PersistError,
        SeedError, SplitError,
    };

    let name = ParamName::new("width");
    let node = RecipeNodeId(5);
    let framed: Vec<(&str, String)> = vec![
        (
            "EditError::SlotUnknownDocParam",
            EditError::SlotUnknownDocParam {
                name: name.clone(),
                node,
                slot: SlotId::Radius,
            }
            .to_string(),
        ),
        (
            "EditError::ContinuousParamCannotBeCount",
            EditError::ContinuousParamCannotBeCount { name: name.clone() }.to_string(),
        ),
        (
            "PersistError::DisplayUnit",
            PersistError::DisplayUnit {
                name: name.clone(),
                unit: Dimension::Angle,
                declared: Dimension::Length,
            }
            .to_string(),
        ),
        (
            "NonFiniteSite::DocParam",
            NonFiniteSite::DocParam {
                name: name.clone(),
                field: DocParamField::Nominal,
            }
            .to_string(),
        ),
        (
            "SnapshotError::SlotUnknownDocParam",
            SnapshotError::SlotUnknownDocParam {
                node,
                slot: SlotId::Radius,
                name: name.clone(),
            }
            .to_string(),
        ),
        (
            "EvalError::UnknownParam",
            EvalError::UnknownParam(name.clone()).to_string(),
        ),
        (
            "SplitError::UncutParamReference",
            SplitError::UncutParamReference {
                param: name.clone(),
                cut_node: RecipeNodeId(1),
                kept_node: RecipeNodeId(2),
            }
            .to_string(),
        ),
        (
            "InlineError::ParamConflict",
            InlineError::ParamConflict {
                param: name.clone(),
            }
            .to_string(),
        ),
        (
            "SeedError::UnknownParam",
            SeedError::UnknownParam {
                param: name.clone(),
            }
            .to_string(),
        ),
        (
            "ParamBoxError::UnknownParam",
            ParamBoxError::UnknownParam {
                param: name.clone(),
            }
            .to_string(),
        ),
        (
            "MeasureUnavailable::BandHasNoMeasure",
            MeasureUnavailable::BandHasNoMeasure {
                param: name.clone(),
            }
            .to_string(),
        ),
    ];
    assert_parameter_names_are_bare(&framed, &name);

    // The exception, and the reason it is one: the name is the bytes the
    // author wrote, which may be a typo, so the quotes delimit what was
    // read rather than decorating a name the document holds.
    let echoed = ParseError::UnknownParam {
        pos: 4,
        name: name.0.clone(),
    }
    .to_string();
    assert!(
        echoed.contains(&format!("{:?}", name.0)),
        "the parse door delimits the bytes it read: {echoed}"
    );
}

/// Each sentence names the parameter and does not quote it — the shared
/// predicate of
/// [`a_parameter_name_renders_unquoted_at_every_door_but_parse`] and its
/// certified-lane sibling, so the two cannot drift into asking
/// different questions of the same rule.
fn assert_parameter_names_are_bare(framed: &[(&str, String)], name: &ParamName) {
    let quoted = format!("{:?}", name.0);
    for (door, shown) in framed {
        assert!(
            shown.contains(&name.0),
            "{door} does not name the parameter at all: {shown}"
        );
        assert!(
            !shown.contains(&quoted),
            "{door} quotes the parameter name, which is `Debug`'s delimiter and not this \
             sentence's: {shown}"
        );
    }
}

/// The two certified-lane doors, `range.rs` and `stackup.rs`, beside
/// [`a_parameter_name_renders_unquoted_at_every_door_but_parse`]'s.
#[test]
fn a_parameter_name_renders_unquoted_at_the_interval_only_doors() {
    use editor_core::{RangeRefusal, Unavailable};

    let name = ParamName::new("width");
    let framed: Vec<(&str, String)> = vec![
        (
            "RangeRefusal::NotAContinuousParam",
            RangeRefusal::NotAContinuousParam {
                param: name.clone(),
            }
            .to_string(),
        ),
        (
            "Unavailable::TangentDegraded",
            Unavailable::TangentDegraded {
                param: name.clone(),
            }
            .to_string(),
        ),
    ];
    assert_parameter_names_are_bare(&framed, &name);
}

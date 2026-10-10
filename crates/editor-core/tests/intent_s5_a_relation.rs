//! INTENT stage 5 PR A: an assertion's relation is `≥`, `≤` or `=`
//! (D10's `Assert { measure, relation, bound }`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, cap_ref, insert, len};
use crate::wire::doctored;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, RefusalReason, SymbolicDials, assertion_at, drive};
use editor_core::{
    AssertionRelation, AssertionVerdict, CapEnd, Dimension, Distribution, DocEdit, Formula,
    FreeVar, MeasurePrimitive, Node, NodeResult, PersistError, ProfileDoc, RecipeNodeId,
    UnevaluatedReason, ValuePayload, VarDecl, VarName, load, save,
};
use geom_core::Tol;

/// The slab's depth, metres: the distance between its two caps.
const DEPTH: f64 = 0.010;

/// One slab `DEPTH` deep, and the distance between its caps measured:
/// the document and the measure's output.
fn slab(seed: &str) -> (ProfileDoc, editor_core::VarId) {
    let doc = ProfileDoc::empty_derived(seed, Tol::witness());
    let (doc, _, profile) = fixture::on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let (doc, slab) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(DEPTH),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let (doc, measured) = fixture::measure(
        doc,
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &[cap_ref(slab, CapEnd::Start), cap_ref(slab, CapEnd::End)],
    );
    (doc, measured.outputs[0])
}

fn assert_on(
    doc: ProfileDoc,
    value: Formula,
    relation: AssertionRelation,
    bound: Formula,
) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Assertion {
            value,
            bound,
            relation,
        },
    )
}

fn verdict(doc: &ProfileDoc, id: RecipeNodeId) -> AssertionVerdict<f64> {
    match crate::corpus::eval::<f64>(doc).result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(verdict) => verdict.clone(),
            other => panic!("{id:?} is a {}", other.kind_name()),
        },
        other => panic!("{id:?} did not evaluate: {other:?}"),
    }
}

/// **(A, test 1) `=` decides at f64 through a document.** A 10 mm
/// distance under `= 10 mm` holds; under a bound 100ε above or below
/// (past the K·ε band at every ε row) it is violated with both numbers;
/// a bound a few ε off is undecided.
/// Breaks if `Equal` reuses one of `AtLeast`'s ends (the bound below
/// holds), or the band arm is lost (the 3ε row is `Violated`).
#[test]
fn equal_decides_a_measured_distance() {
    let (doc, out) = slab("s5a-equal");
    let eps = Tol::witness().eps();
    let read = fixture::read_var(&doc, out);
    let rows = [
        (DEPTH, "Holds"),
        (DEPTH + 100.0 * eps, "Violated"),
        (DEPTH - 100.0 * eps, "Violated"),
        (DEPTH + 3.0 * eps, "Unevaluated"),
    ];
    let mut doc = doc;
    for (bound, want) in rows {
        let (next, id) = assert_on(doc, read.clone(), AssertionRelation::Equal, len(bound));
        doc = next;
        let got = verdict(&doc, id);
        assert_eq!(got.label(), want, "`{DEPTH} = {bound}`: {got:?}");
        match got {
            AssertionVerdict::Violated { measured, bound: b } => {
                assert_eq!((measured, b), (DEPTH, bound), "both numbers, unswapped");
            }
            AssertionVerdict::Unevaluated { reason } => assert!(
                matches!(reason, UnevaluatedReason::Indeterminate { .. }),
                "the sliver band, not {reason:?}"
            ),
            AssertionVerdict::Holds { .. } => {}
        }
    }
}

/// **(A) The relation is on the wire as `relation`, and a file still
/// spelling `dir` refuses typed.** The variant tag of `=` is `Equal`, and
/// a saved `=` loads back as one. Breaks if the old field name is still
/// accepted (an alias would read a file the regenerate recourse is for),
/// or `Equal` does not round-trip.
#[test]
fn the_relation_is_the_wire_field_and_dir_refuses() {
    let (doc, out) = slab("s5a-wire");
    let read = fixture::read_var(&doc, out);
    let (doc, id) = assert_on(doc, read, AssertionRelation::Equal, len(DEPTH));
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = load(&text, Tol::witness()).expect("its own bytes load");
    assert!(
        matches!(
            loaded.doc.node(id),
            Some(Node::Assertion {
                relation: AssertionRelation::Equal,
                ..
            })
        ),
        "`=` round-trips"
    );
    let old = doctored(&text, |wire| {
        let node = wire["snapshot"]["nodes"][id.0.to_string()]["Assertion"]
            .as_object_mut()
            .expect("the assertion's fields");
        let relation = node.remove("relation").expect("the field is `relation`");
        assert_eq!(relation, serde_json::json!("Equal"), "the variant tag");
        node.insert("dir".to_owned(), serde_json::json!("AtLeast"));
    });
    match load(&old, Tol::witness()) {
        Err(PersistError::Unreadable { .. }) => {}
        other => panic!("a file spelling `dir` refuses unreadable, got {other:?}"),
    }
}

/// A document declaring toleranced lengths at `values`, each normal
/// with a small sigma, so the analysis box varies them.
fn toleranced(seed: &str, values: &[(&'static str, f64)]) -> ProfileDoc {
    values.iter().fold(
        ProfileDoc::empty_derived(seed, Tol::witness()),
        |doc, &(name, value)| {
            let (doc, _) = fixture::step(
                doc,
                DocEdit::DeclareVar {
                    name: VarName::from_static(name),
                    def: VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
                },
            );
            let var = doc.var_named(name).expect("declared");
            fixture::step(
                doc,
                DocEdit::SetVarDistribution {
                    var: var.into(),
                    distribution: Some(Distribution::Normal { sigma: 1e-4 }),
                },
            )
            .0
        },
    )
}

/// The verdict of `assertion` over `doc`'s whole analysis box, on the
/// lane `symbolic` picks.
fn over_the_box(
    doc: &ProfileDoc,
    assertion: RecipeNodeId,
    symbolic: SymbolicDials,
) -> AssertionVerdict<geom_core::Interval> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    assert!(
        ParamBox::of(&analyzed).varying().next().is_some(),
        "the box varies"
    );
    assertion_at(
        doc,
        assertion,
        &ParamBox::of(&analyzed),
        symbolic,
        Tol::witness(),
    )
    .expect("an assertion has a verdict over the box")
}

/// **(A, test 3) `=` over a box is decided pointwise.** `w − w = 0`
/// (one variable read twice) holds over the box on the symbolic lane,
/// which proves the margin identically zero, and is undecided on the
/// numeric lane, where interval dependency widens it; the driver
/// certifies the whole box. Two toleranced variables of one nominal,
/// `a − b = 0`, never hold: their difference is a genuine interval, so
/// the leaf is undecided and the driver bisects it to its budget, where
/// the same document under `a − b ≥ −1 m` certifies. Breaks if `=` is
/// special-cased to hold on the nominal (the `a − b` row holds, and the
/// drive refuses nothing), or the symbolic lane does not reach the
/// assertion's decision (the `w − w` row is undecided there too).
#[test]
fn equal_over_a_box_holds_only_on_a_structural_zero() {
    let named = |name| Formula::named(VarName::from_static(name), Dimension::Length);
    let minus = |a, b| Formula::sub(named(a), named(b)).expect("Length - Length");

    let (structural, same) = assert_on(
        toleranced("s5a-box-w", &[("w", 0.005)]),
        minus("w", "w"),
        AssertionRelation::Equal,
        len(0.0),
    );
    let symbolic = over_the_box(&structural, same, SymbolicDials::default());
    assert!(
        matches!(symbolic, AssertionVerdict::Holds { .. }),
        "`w − w = 0` is a theorem on the symbolic lane: {symbolic:?}"
    );
    let numeric = over_the_box(&structural, same, SymbolicDials::off());
    assert!(
        matches!(numeric, AssertionVerdict::Unevaluated { .. }),
        "the numeric lane cannot see the identity: {numeric:?}"
    );

    let twins_at = |relation, bound| {
        assert_on(
            toleranced("s5a-box-ab", &[("a", 0.005), ("b", 0.005)]),
            minus("a", "b"),
            relation,
            len(bound),
        )
    };
    let (twins, equal) = twins_at(AssertionRelation::Equal, 0.0);
    let verdict = over_the_box(&twins, equal, SymbolicDials::default());
    assert!(
        matches!(verdict, AssertionVerdict::Unevaluated { .. }),
        "two toleranced variables equal at the nominal are not equal over the box: {verdict:?}"
    );

    let driven = |doc: &ProfileDoc| {
        drive(
            doc,
            &analyzed_box(doc, &AnalysisPolicy::default()),
            &DriveConfig {
                max_leaves: 16,
                ..DriveConfig::default()
            },
            Tol::witness(),
        )
        .expect("the witness builds")
    };
    let proven = driven(&structural);
    assert!(
        proven.refused().is_empty(),
        "`w − w = 0` certifies the whole box: {:?}",
        proven.refused()
    );
    let bisected = driven(&twins);
    assert!(
        !bisected.refused().is_empty()
            && bisected
                .refused()
                .iter()
                .all(|leaf| matches!(leaf.reason, RefusalReason::Budget(_))),
        "an undecided `=` bisects to the budget: {:?}",
        bisected.refused()
    );
    let (loose, _) = twins_at(AssertionRelation::AtLeast, -1.0);
    let control = driven(&loose);
    assert!(
        control.refused().is_empty(),
        "the same document under `a − b ≥ −1 m` certifies: {:?}",
        control.refused()
    );
}

/// **(A) The relation is in the assertion's content key.** One value
/// and one bound under `≥`, `≤` and `=` key three ways, so a document
/// re-related from `≥` to `=` is never served the `≥` verdict from a
/// memo. Breaks if `Equal` writes another relation's tag.
#[test]
fn each_relation_keys_its_assertion_apart() {
    let (doc, out) = slab("s5a-key");
    let read = fixture::read_var(&doc, out);
    let keys: Vec<_> = [
        AssertionRelation::AtLeast,
        AssertionRelation::AtMost,
        AssertionRelation::Equal,
    ]
    .into_iter()
    .map(|relation| {
        let (doc, id) = assert_on(doc.clone(), read.clone(), relation, len(DEPTH));
        crate::corpus::eval::<f64>(&doc)
            .value(id)
            .expect("the assertion evaluates")
            .content_key
    })
    .collect();
    assert_ne!(keys[0], keys[2], "`>=` and `=` key apart");
    assert_ne!(keys[1], keys[2], "`<=` and `=` key apart");
    assert_ne!(keys[0], keys[1], "`>=` and `<=` key apart");
}

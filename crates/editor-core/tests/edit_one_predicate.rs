//! **Document predicates with one home each, asked by both doors.**
//!
//! An assertion's bound against the measure it constrains, a mate's
//! alignment datum, an A11 placement row, a witness row's key, the
//! recorded ε, a document parameter's declaration and its floats: each
//! rule is stated once — `Node::assertion_bound_fault`,
//! `Node::has_non_finite_alignment`, `doc::placement_fault`,
//! `doc::witness_site_fault`, `doc::epsilon_admissible`,
//! `DocParam::is_continuous_count`, `DocParam::first_non_finite` — and
//! the edit door and the persistence door each render the one answer
//! in their own vocabulary.
//!
//! **What that buys, stated exactly**: for each rule below, the two
//! doors cannot come to disagree about it, because there is one
//! decision and two renderings of its answer. A slot's dimension is
//! the same shape one file over (`load_door_slot_dimension` in this
//! binary), over `Node::slot_dimension_fault`.
//!
//! One row per FACT, naming both doors' refusals for it: the pairing
//! is the property a shared predicate buys, and a row that asked only
//! one door could not see the two drift. A predicate dropped at either
//! door therefore reds that fact's row, and the panic says which door
//! let the document through.
//!
//! Two arms of the load door's vocabulary are unreachable from a FILE
//! and are pinned at the save door instead, in-crate
//! (`persist::check`'s unit rows): JSON carries no non-finite token, so
//! a non-finite alignment or placement coordinate can only reach
//! `validate_document` from memory. Every other rule below is
//! reachable from a file and is paired here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, ContactClass, Dimension, DocEdit, DocParam, DocumentId, EditError,
    EntityKind, Expr, Frame, MateFrame, MatePrimitive, MeasureExpr, Node, ParamName, PersistError,
    ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SitedRef, SnapshotError, StableName, apply,
    load, save,
};
use fixture::resolver::{PART_BODY, PartStore};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;

// ---- The assertion's bound ----

/// A document carrying an XY frame (0), a profile (1), an extrude (2)
/// and a LENGTH measure (3) that reads no references — the measurement
/// vocabulary's smallest well-formed sink, which is all an assertion's
/// bound is checked against.
fn with_measure() -> (ProfileDoc, RecipeNodeId) {
    let mut r = fixture::Recorder::new();
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let _ = r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    let measure = r.insert(Node::Measure {
        expr: MeasureExpr::value(len(1.0)),
        refs: Vec::new(),
    });
    (r.doc, measure)
}

fn assertion(measure: RecipeNodeId, bound: Expr) -> Node<ProfileProgram> {
    Node::Assertion {
        measure,
        bound,
        dir: editor_core::AssertionDir::AtLeast,
    }
}

/// **The reference is not a measure — both doors.** The edit door
/// names it `AssertionTarget`; the load door names it
/// `SnapshotError::AssertionTarget`, its own arm since this unit,
/// because a reader should not have to decode an absent dimension to
/// learn which of the two assertion faults happened.
#[test]
fn an_assertion_over_a_non_measure_is_refused_at_both_doors() {
    let (doc, measure) = with_measure();
    // The sketch frame (node 0) is live and precedes any assertion, so
    // the only thing wrong with the document is that it is not a
    // measure.
    let frame_node = RecipeNodeId(0);
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: assertion(frame_node, len(1.0)),
        },
        Tol::witness(),
    ) {
        Err(EditError::AssertionTarget { measure: m, .. }) => assert_eq!(m, frame_node),
        other => panic!("an assertion over a non-measure must refuse typed, got {other:?}"),
    }

    // The same fact at the load door: a file whose assertion is
    // re-pointed at the frame.
    let (text, id) = saved_assertion(&doc, measure, len(1.0));
    let corrupt = repoint_measure(&text, id, measure, frame_node);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AssertionTarget {
            measure: m,
            bound: Dimension::Length,
            ..
        })) => assert_eq!(m, frame_node),
        other => panic!("a non-measure target must refuse typed at load, got {other:?}"),
    }
}

/// **The bound measures something else — both doors.** The measure
/// yields a length; the bound is an angle. Both doors carry BOTH
/// dimensions, so the message says what disagrees rather than that
/// something does.
#[test]
fn an_assertion_bound_of_the_wrong_dimension_is_refused_at_both_doors() {
    let (doc, measure) = with_measure();
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: assertion(measure, fixture::ang(0.5)),
        },
        Tol::witness(),
    ) {
        Err(EditError::AssertionDimension {
            measured: Dimension::Length,
            bound: Dimension::Angle,
            ..
        }) => {}
        other => panic!("a mismatched bound must refuse typed, got {other:?}"),
    }

    // The load door's half: the same assertion, saved with a LENGTH
    // bound and then retyped on the wire.
    let (text, id) = saved_assertion(&doc, measure, len(0.5));
    let corrupt = retype_bound(&text, id);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AssertionBound {
            measured: Dimension::Length,
            bound: Dimension::Angle,
            ..
        })) => {}
        other => panic!("a mismatched bound must refuse typed at load, got {other:?}"),
    }
}

/// The saved text of `doc` plus one well-formed assertion, and that
/// assertion's id — the rows above corrupt the text BY PATH, so they
/// need the id to aim with.
fn saved_assertion(doc: &ProfileDoc, measure: RecipeNodeId, bound: Expr) -> (String, RecipeNodeId) {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: assertion(measure, bound),
        },
        Tol::witness(),
    )
    .expect("a well-dimensioned assertion inserts");
    let id = applied.record.minted.expect("the insert minted an id");
    let text = save(&applied.doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    (text, id)
}

/// **Wire surgery BY PATH.** The saved file is split at its id header,
/// the body is parsed, `edit` moves the named field, and the body is
/// re-serialized under the same header.
///
/// A byte substitution proves only that a byte moved; this proves the
/// INTENDED field moved, because the path names it and `edit` asserts
/// what it found there. A fixture or field-order change then breaks
/// the surgery loudly instead of silently landing it on a neighbour.
fn doctored(text: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let split = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    edit(&mut wire);
    let out = format!("{header}{wire}");
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// Re-points the assertion's `measure` field.
fn repoint_measure(
    text: &str,
    assertion: RecipeNodeId,
    from: RecipeNodeId,
    to: RecipeNodeId,
) -> String {
    doctored(text, |wire| {
        let field = &mut wire["snapshot"]["nodes"][assertion.0.to_string()]["Assertion"]["measure"];
        assert_eq!(
            *field,
            serde_json::json!(from.0),
            "the surgery is aimed at the assertion's target"
        );
        *field = serde_json::json!(to.0);
    })
}

/// Retypes the assertion's BOUND literal from a length to an angle,
/// unit and all — both halves, so the literal is still one the load
/// door's `Expr::literal_with_unit` rebuild accepts and the refusal
/// read is the DIMENSION rule's.
fn retype_bound(text: &str, assertion: RecipeNodeId) -> String {
    doctored(text, |wire| {
        let lit = &mut wire["snapshot"]["nodes"][assertion.0.to_string()]["Assertion"]["bound"]["Literal"];
        assert_eq!(
            lit["dim"],
            serde_json::json!("Length"),
            "the surgery is aimed at a length bound"
        );
        lit["dim"] = serde_json::json!("Angle");
        lit["unit"] = serde_json::json!("rad");
    })
}

// ---- A mate's alignment ----

/// **`n` instances of a part document that exists**: the reference is
/// minted by inserting a real part into a [`PartStore`], so its content
/// pin is that part's digest and the faces `in_part` names are faces
/// the part actually has.
///
/// That is what distinguishes it from the same-shaped fixture beside
/// `persist::check`'s in-crate rows, which mints a `DocRef` by hand
/// over no part at all: this seat builds mate HEADS, which name a face
/// inside the referenced part, and the validator's seat does not. The
/// two cannot be one function — an integration suite cannot reach a
/// `#[cfg(test)]` item in the library, and the library cannot reach
/// `tests/fixture` — so they are two, named for the difference.
fn instances_of_a_stored_part(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    (doc, ids)
}

fn part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

fn in_part(instance: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(editor_core::CapEnd::Start)],
            }
            .into(),
        }],
    }
}

fn mate(a: RecipeNodeId, b: RecipeNodeId, origin: [f64; 3]) -> Node<ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(in_part(a)),
        b: SitedRef::at_mint(in_part(b)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                origin,
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            b: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Aligned,
            clocking: None,
        },
    }
}

/// **A mate the edit door cannot decide on is refused there.** The
/// alignment is authored numbers; a coordinate that is not a number
/// leaves every downstream predicate — the coset solve, the rest
/// classification — with nothing to read.
///
/// The load door asks the SAME question in the same node walk
/// (`Node::has_non_finite_alignment`); it is unreachable from a file,
/// because JSON has no non-finite token, and the save door's half is
/// pinned in-crate beside the validator.
#[test]
fn a_non_finite_alignment_is_refused_at_the_edit_door() {
    let (doc, ids) = instances_of_a_stored_part("onepred-align", 2);
    // Finite, the same mate is accepted — so the refusal below is the
    // coordinate's and not the fixture's.
    let (doc, _) = insert(doc, mate(ids[0], ids[1], [0.0, 0.0, 0.0]));
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: mate(ids[0], ids[1], [f64::NAN, 0.0, 0.0]),
        },
        Tol::witness(),
    ) {
        Err(EditError::NonFiniteAlignment { .. }) => {}
        other => panic!("a non-finite alignment must refuse typed, got {other:?}"),
    }
}

// ---- The A11 placement registry ----

/// **A placement on a node that instantiates nothing — both doors.**
/// A11 puts the frame on an instance's cluster, so nothing else has
/// one; the edit door names it `PlacementOnNonInstance` and the load
/// door `PlacementSite`.
#[test]
fn a_placement_on_a_non_instance_is_refused_at_both_doors() {
    let (doc, ids) = instances_of_a_stored_part("onepred-site", 1);
    // A live NON-instance node, so the refusal is the placement rule's
    // and not a dangling id's.
    let (doc, other) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    match apply(
        &doc,
        &DocEdit::SetPlacement {
            node: other,
            frame: Frame::translation([1.0, 0.0, 0.0]),
        },
        Tol::witness(),
    ) {
        Err(EditError::PlacementOnNonInstance { node }) => assert_eq!(node, other),
        other => panic!("a placement on a non-instance must refuse typed, got {other:?}"),
    }

    let text = saved_placement(&doc, ids[0], Frame::translation([1.0, 0.0, 0.0]));
    let corrupt = rekey_placement(&text, ids[0], other);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PlacementSite { node })) => {
            assert_eq!(node, other);
        }
        other => panic!("a stranded placement must refuse typed at load, got {other:?}"),
    }
}

/// **An IMPROPER frame — both doors.** A mirror is authored data this
/// build declines to admit (A6, gated on R4's equivariance audit), and
/// the load door now says so in its own arm rather than folding it in
/// with a coordinate no predicate can read.
#[test]
fn an_improper_placement_is_refused_at_both_doors() {
    let (doc, ids) = instances_of_a_stored_part("onepred-improper", 1);
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    assert!(mirror.determinant() < 0.0, "the fixture really mirrors");
    match apply(
        &doc,
        &DocEdit::SetPlacement {
            node: ids[0],
            frame: mirror,
        },
        Tol::witness(),
    ) {
        Err(EditError::ImproperPlacement { determinant, .. }) => {
            assert!(determinant < 0.0, "the refusal carries the determinant");
        }
        other => panic!("an improper placement must refuse typed, got {other:?}"),
    }

    // The load door's half: a proper frame on the wire, mirrored by
    // flipping the first column's sign.
    let text = saved_placement(&doc, ids[0], Frame::translation([1.0, 0.0, 0.0]));
    let corrupt = mirror_first_column(&text, ids[0]);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PlacementImproper { node, determinant })) => {
            assert_eq!(node, ids[0]);
            assert!(determinant < 0.0, "the refusal carries the determinant");
        }
        other => panic!("an improper frame must refuse typed at load, got {other:?}"),
    }
}

/// **The gauge rule is the load door's alone, and that is the
/// invariant.** `SetPlacement` does not refuse a key that is not its
/// cluster's gauge — it KEYS THE ROW ON THE GAUGE — so no document the
/// edit doors build can carry a non-gauge row, and the load door's
/// `PlacementNotGauge` is reachable only from a file.
///
/// This is the measurement the predicate's home records: an edit-time
/// refusal here would refuse a placement the edit door instead accepts
/// and normalises.
#[test]
fn a_placement_off_the_gauge_is_keyed_on_it_rather_than_refused() {
    let (doc, ids) = instances_of_a_stored_part("onepred-gauge", 2);
    // Mate the two instances: one cluster, whose gauge is its
    // document-order-first member.
    let (doc, _) = insert(doc, mate(ids[0], ids[1], [0.0, 0.0, 0.0]));
    let frame = Frame::translation([2.0, 0.0, 0.0]);
    // The placement is authored on the LATER instance, which is not the
    // gauge.
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[1],
            frame,
        },
    );
    assert!(
        doc.placements().contains_key(&ids[0]),
        "the row is keyed on the cluster's gauge"
    );
    assert!(
        !doc.placements().contains_key(&ids[1]),
        "and not on the instance it was authored against"
    );
    let text = save(&doc, &[], Tol::witness()).expect("the normalised registry saves");

    // A FILE can carry the key the edit door rewrote, and that is the
    // door that refuses it.
    let corrupt = rekey_placement(&text, ids[0], ids[1]);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PlacementNotGauge { node, gauge })) => {
            assert_eq!(node, ids[1]);
            assert_eq!(gauge, ids[0]);
        }
        other => panic!("a non-gauge placement row must refuse typed at load, got {other:?}"),
    }
}

/// The saved text of `doc` carrying one placement, which the rows
/// above corrupt.
fn saved_placement(doc: &ProfileDoc, node: RecipeNodeId, frame: Frame) -> String {
    let (doc, _) = step(doc.clone(), DocEdit::SetPlacement { node, frame });
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    text
}

/// Negates the first entry of the placement frame's first column,
/// which flips the determinant's sign and nothing else.
fn mirror_first_column(text: &str, node: RecipeNodeId) -> String {
    doctored(text, |wire| {
        let entry = &mut wire["snapshot"]["placements"][node.0.to_string()]["columns"][0][0];
        assert_eq!(
            *entry,
            serde_json::json!(1.0),
            "the surgery is aimed at the first column's own axis"
        );
        *entry = serde_json::json!(-1.0);
    })
}

/// Re-keys the single placement row.
fn rekey_placement(text: &str, from: RecipeNodeId, to: RecipeNodeId) -> String {
    doctored(text, |wire| {
        let registry = wire["snapshot"]["placements"]
            .as_object_mut()
            .expect("the registry is a map on the wire");
        let row = registry
            .remove(&from.0.to_string())
            .expect("the row is keyed by the node it was authored on");
        registry.insert(to.0.to_string(), row);
    })
}

// ---- A witness row's key ----

/// The witness these rows attach — bytes with a schema, which is all a
/// datum is at this layer.
fn witness() -> editor_core::WitnessDatum {
    editor_core::WitnessDatum {
        schema: 1,
        bytes: b"onepred".to_vec(),
    }
}

/// The saved text of `doc` carrying one witness on `node`, which the
/// rows below then re-key.
fn saved_witness(doc: &ProfileDoc, node: RecipeNodeId) -> String {
    let (doc, _) = step(
        doc.clone(),
        DocEdit::ReWitness {
            node,
            witness: witness(),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    text
}

/// Re-keys the single witness row.
fn rekey_witness(text: &str, from: RecipeNodeId, to: RecipeNodeId) -> String {
    doctored(text, |wire| {
        let store = wire["snapshot"]["witnesses"]
            .as_object_mut()
            .expect("the store is a map on the wire");
        let row = store
            .remove(&from.0.to_string())
            .expect("the row is keyed by the node it was authored on");
        store.insert(to.0.to_string(), row);
    })
}

/// **A witness on a node that bears no sketch — both doors.** A witness
/// records a choice about a sketch's branch, so a node with no sketch
/// has no branch for it to be about; the edit door names it
/// `WitnessOnNonSketch` and the load door `SnapshotError::WitnessSite`.
#[test]
fn a_witness_on_a_non_sketch_node_is_refused_at_both_doors() {
    // `with_measure` lays down a frame (0), a profile (1), an extrude
    // (2) and a measure (3): the profile is the only sketch-bearing
    // node in it, and the extrude is a live node that is not one.
    let (doc, _) = with_measure();
    let (sketch, non_sketch) = (RecipeNodeId(1), RecipeNodeId(2));
    match apply(
        &doc,
        &DocEdit::ReWitness {
            node: non_sketch,
            witness: witness(),
        },
        Tol::witness(),
    ) {
        Err(EditError::WitnessOnNonSketch { node }) => assert_eq!(node, non_sketch),
        other => panic!("a witness on a non-sketch must refuse typed, got {other:?}"),
    }

    let text = saved_witness(&doc, sketch);
    let corrupt = rekey_witness(&text, sketch, non_sketch);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::WitnessSite { node })) => {
            assert_eq!(node, non_sketch);
        }
        other => panic!("a witness on a non-sketch must refuse typed at load, got {other:?}"),
    }
}

/// **A witness on a node that is not live — both doors.** Its own
/// refusal at each, rather than folded in with the non-sketch case: one
/// is repaired by moving the witness, the other has no node to move it
/// to. The edit door names it `UnknownNode` and the load door
/// `SnapshotError::WitnessOnMissingNode`.
#[test]
fn a_witness_on_a_missing_node_is_refused_at_both_doors() {
    let (doc, _) = with_measure();
    let (sketch, gone) = (RecipeNodeId(1), RecipeNodeId(2));
    // Deleted rather than invented, so the id stays BELOW the mint
    // counter and the load door's id walk passes it — the refusal read
    // is then the site rule's and not `IdBeyondCounter`.
    let doc = apply(&doc, &DocEdit::DeleteNode { id: gone }, Tol::witness())
        .expect("the extrude has no consumer")
        .doc;
    match apply(
        &doc,
        &DocEdit::ReWitness {
            node: gone,
            witness: witness(),
        },
        Tol::witness(),
    ) {
        Err(EditError::UnknownNode { id }) => assert_eq!(id, gone),
        other => panic!("a witness on a missing node must refuse typed, got {other:?}"),
    }

    let text = saved_witness(&doc, sketch);
    let corrupt = rekey_witness(&text, sketch, gone);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::WitnessOnMissingNode { node })) => {
            assert_eq!(node, gone);
        }
        other => panic!("a witness on a missing node must refuse typed at load, got {other:?}"),
    }
}

// ---- The recorded ε ----

/// **An ε that is not strictly positive — both doors.** ε parameterizes
/// every predicate band the document is read through, so a zero or
/// negative one leaves every geometric answer undefined. The edit door
/// names it `InvalidTolerance` and the load door
/// `SnapshotError::EpsilonInvalid`.
///
/// The NON-FINITE half of the same rule is unreachable from a file —
/// JSON carries no such token — and in memory it refuses one walk
/// earlier, as `PersistError::NonFinite` at `NonFiniteSite::Epsilon`.
#[test]
fn a_non_positive_epsilon_is_refused_at_both_doors() {
    let (doc, _) = with_measure();
    match apply(&doc, &DocEdit::SetTolerance { eps: 0.0 }, Tol::witness()) {
        Err(EditError::InvalidTolerance { value }) => assert_eq!(value, 0.0),
        other => panic!("a zero ε must refuse typed, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let eps = &mut wire["snapshot"]["epsilon"];
        assert!(
            eps.as_f64().is_some_and(|v| v > 0.0),
            "the surgery is aimed at a live ε"
        );
        *eps = serde_json::json!(0.0);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::EpsilonInvalid { value })) => {
            assert_eq!(value, 0.0);
        }
        other => panic!("a zero ε must refuse typed at load, got {other:?}"),
    }
}

// ---- A document parameter's floats ----

/// **A continuous parameter carrying a float that is not a number —
/// both doors, naming WHICH float.**
///
/// `DocParam::first_non_finite` is the one rule and both doors ask it:
/// the nominal first, then the annotation's offsets. The edit door
/// names its answer `EditError::NonFiniteDocParam` and the load door
/// `NonFiniteSite::DocParam`, and each carries the field the predicate
/// identified — a door that dropped it would be answering a coarser
/// question than the one that was asked.
///
/// The load door is reached through an UNAPPLIED edit log rather than
/// a doctored snapshot: JSON carries no non-finite token, so a log —
/// which is data, and has not necessarily been through `apply` — is
/// the only way a NaN reaches `validate_document` at all.
#[test]
fn a_non_finite_doc_param_is_refused_at_both_doors_naming_the_field() {
    use editor_core::{Distribution, DistributionField, DocParamField, persist::NonFiniteSite};

    let (doc, _) = with_measure();
    let name = ParamName::new("wall");
    let annotated = |sigma: f64| {
        let mut value = DocParam::continuous(Dimension::Length, 1.0);
        if let DocParam::Continuous { distribution, .. } = &mut value {
            *distribution = Some(Distribution::Normal { sigma });
        }
        value
    };
    let cases = [
        (
            DocParam::continuous(Dimension::Length, f64::NAN),
            DocParamField::Nominal,
        ),
        (
            annotated(f64::INFINITY),
            DocParamField::Offset(DistributionField::Sigma),
        ),
    ];
    for (value, expected) in cases {
        match apply(
            &doc,
            &DocEdit::SetDocParam {
                name: name.clone(),
                value: value.clone(),
            },
            Tol::witness(),
        ) {
            Err(EditError::NonFiniteDocParam { name: n, field }) => {
                assert_eq!(n, name);
                assert_eq!(field, expected, "the edit door names the offending float");
            }
            other => panic!("a non-finite parameter must refuse typed, got {other:?}"),
        }

        let edit = DocEdit::SetDocParam {
            name: name.clone(),
            value,
        };
        match save(&doc, &[edit], Tol::witness()) {
            Err(PersistError::NonFinite {
                site: NonFiniteSite::Edit { index: 0, inner },
            }) => match *inner {
                NonFiniteSite::DocParam { name: ref n, field } => {
                    assert_eq!(*n, name);
                    assert_eq!(field, expected, "the load door names the same float");
                }
                ref other => panic!("expected a doc-param site, got {other:?}"),
            },
            other => panic!("a non-finite parameter must refuse at the wire, got {other:?}"),
        }
    }
}

// ---- A document parameter's declaration ----

/// **A continuous parameter declared with the count dimension — the
/// edit door, and what the load door actually answers.**
///
/// `DocParam::is_continuous_count` is the one rule and both doors ask
/// it; this row pins the edit door's answer
/// (`ContinuousParamCannotBeCount`).
///
/// At the load door the same document refuses by another name, and
/// that is what this row asserts: no unit in the table measures a
/// count (`UnitSym::measures` answers Length, Angle or Scalar), so a
/// `Continuous` parameter declared `Count` fails the display-unit walk
/// whatever notation it carries. The divide has no second arm in the
/// load door's vocabulary — a `SnapshotError` that nothing could
/// reach was documentation, and deleting it was its repair — so the
/// two doors' answers differ in WORD and agree in verdict, which is
/// the property a caller comparing them relies on.
#[test]
fn a_continuous_parameter_declared_count_is_refused_at_both_doors_in_different_words() {
    let (doc, _) = with_measure();
    let name = ParamName::new("n");
    match apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::continuous(Dimension::Count, 3.0),
        },
        Tol::witness(),
    ) {
        Err(EditError::ContinuousParamCannotBeCount { name: n }) => assert_eq!(n, name),
        other => panic!("a count-dimensioned continuous param must refuse typed, got {other:?}"),
    }

    // The load door's half: a well-formed LENGTH parameter, retyped to
    // a count on the wire.
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::continuous(Dimension::Length, 3.0),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let dim = &mut wire["snapshot"]["params"][&name.0]["Continuous"]["dim"];
        assert_eq!(
            *dim,
            serde_json::json!("Length"),
            "the surgery is aimed at the declared dimension"
        );
        *dim = serde_json::json!("Count");
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::DisplayUnit {
            declared: Dimension::Count,
            name: n,
            ..
        }) => assert_eq!(n, name),
        other => panic!("a count-dimensioned continuous param must refuse at load, got {other:?}"),
    }
}

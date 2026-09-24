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
//!
//! **One rule below has no edit-door half, by construction rather than
//! by omission**: a mate head is a `SitedFace` over a `FaceName`, so
//! "this head names a face" is decided by the TYPE and there is no
//! edit to refuse — the row pairs the load door against `SitedFace`'s
//! own `compile_fail` doctest instead, and says so at the site. That
//! is the same property one rung up: not two doors kept in step, but
//! one decision the second door cannot restate differently.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::wire::doctored;
use editor_core::{
    Alignment, AxisSense, ContactClass, Dimension, DocEdit, DocParam, DocRef, DocumentId,
    EditError, EntityKind, Expr, FaceName, Frame, InterfaceCrossing, InterfaceRecord, MateFrame,
    MatePrimitive, MeasureExpr, Node, ParamName, PersistError, ProfileDoc, ProfileProgram,
    RecipeNodeId, RoleSeg, SnapshotError, StableName, apply, load, save,
};
use editor_core::{LoggedEdit, ParamNameReason, parse_expr};
use fixture::resolver::{PART_BODY, PartStore};
use fixture::{insert, len, on_frame, square, step};
use geom_core::Tol;
use std::collections::BTreeMap;

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
        &editor_core::RefusingReach,
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
        &editor_core::RefusingReach,
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
        &editor_core::RefusingReach,
    )
    .expect("a well-dimensioned assertion inserts");
    let id = applied.record.minted.expect("the insert minted an id");
    let text = save(&applied.doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    (text, id)
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
    let (doc, _, ids) = instances_and_ref_of_a_stored_part(label, n);
    (doc, ids)
}

/// The same fixture, keeping the reference it minted the instances
/// from — for a row that inserts a FURTHER instance of the same part,
/// which is the only way to author one carrying an interface record.
fn instances_and_ref_of_a_stored_part(
    label: &str,
    n: usize,
) -> (ProfileDoc, DocRef, Vec<RecipeNodeId>) {
    let mut store = PartStore::default();
    let doc_ref = store.insert(part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    (doc, doc_ref, ids)
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
            of: part_face().into(),
        }],
    }
}

/// The part-local face `in_part` wraps: one spelling, so a crossing
/// built here names the same face on both sides of the seam.
fn part_face() -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: PART_BODY,
        path: vec![RoleSeg::Cap(editor_core::CapEnd::Start)],
    }
}

fn mate(a: RecipeNodeId, b: RecipeNodeId, origin: [f64; 3]) -> Node<ProfileProgram> {
    Node::Mate {
        a: crate::fixture::head(in_part(a)),
        b: crate::fixture::head(in_part(b)),
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
        &editor_core::RefusingReach,
    ) {
        Err(EditError::NonFiniteAlignment { .. }) => {}
        other => panic!("a non-finite alignment must refuse typed, got {other:?}"),
    }
}

// ---- The mate head's entity kind ----

/// A two-instance document carrying one FACE-TO-FACE mate, saved and
/// loaded once — the control the rows below corrupt, and the round
/// trip in its own right.
fn saved_mate(label: &str) -> (String, RecipeNodeId) {
    let (doc, ids) = instances_of_a_stored_part(label, 2);
    let (doc, id) = insert(doc, mate(ids[0], ids[1], [0.0, 0.0, 0.0]));
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("a face-to-face mate round trips");
    (text, id)
}

/// Retypes one head of a saved mate — its KIND and nothing else.
fn retype_head(text: &str, mate: RecipeNodeId, side: &str, kind: EntityKind) -> String {
    doctored(text, |wire| {
        let field =
            &mut wire["snapshot"]["nodes"][mate.0.to_string()]["Mate"][side]["name"]["kind"];
        assert_eq!(
            *field,
            serde_json::json!("Face"),
            "the surgery is aimed at a face head"
        );
        *field = serde_json::json!(format!("{kind:?}"));
    })
}

/// **A saved mate head that is not a face refuses at the load door.**
///
/// There is no edit-door twin to pair this with, and that is the
/// shape rather than a gap: a head is an `editor_core::SitedFace`
/// over a `FaceName`, so a mate naming an edge is a program that does
/// not compile (`SitedFace`'s own `compile_fail` row). What a FILE
/// can still carry is an edge head spelled in bytes, and the rule is
/// asked there by the same one constructor — `FaceName`'s
/// `Deserialize` — so the refusal is the load door's own
/// `Unreadable`: the reader accepted the bytes and this build's TYPES
/// rejected them, which is exactly what that arm says.
///
/// Both heads and all three non-face kinds, because the type fixes
/// one question and both heads ask it.
#[test]
fn a_saved_mate_head_that_is_not_a_face_refuses_at_the_load_door() {
    for kind in [EntityKind::Body, EntityKind::Edge, EntityKind::Vertex] {
        for side in ["a", "b"] {
            let (text, mate) = saved_mate("onepred-matehead");
            let corrupt = retype_head(&text, mate, side, kind);
            match load(&corrupt, Tol::witness()) {
                Err(PersistError::Unreadable { detail, .. }) => {
                    assert!(
                        detail.contains(kind_noun(kind)),
                        "the refusal names what the head denoted, got {detail:?}"
                    );
                }
                other => panic!("a {kind:?} mate head must refuse typed at load, got {other:?}"),
            }
        }
    }
}

/// The kind's prose noun, as the head constructor's refusal spells it.
fn kind_noun(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Body => "body",
        EntityKind::Face => "face",
        EntityKind::Edge => "edge",
        EntityKind::Vertex => "vertex",
    }
}

/// **The control**: the same document with both heads naming faces
/// saves, loads and keeps its mate — so the rows above measure the
/// kind and not the fixture.
#[test]
fn a_face_to_face_mate_round_trips() {
    let (text, mate) = saved_mate("onepred-matehead-ok");
    let loaded = load(&text, Tol::witness()).expect("a face-to-face mate loads");
    let Some(Node::Mate { a, b, .. }) = loaded.doc.node(mate) else {
        panic!("the mate survives the round trip");
    };
    assert_eq!(a.name.kind, EntityKind::Face);
    assert_eq!(b.name.kind, EntityKind::Face);
}

// ---- An interface crossing's two references ----

/// A name as a face name, for a fixture that spells a face.
fn face(name: StableName) -> FaceName {
    FaceName::new(name).expect("the fixture spells a face")
}

/// A document carrying one instance whose interface record holds a
/// single FACE-TO-FACE crossing, saved and loaded once — the control
/// the row below corrupts, and the round trip in its own right.
///
/// The record is authored through `Node::instantiate_part_with` rather
/// than harvested from a split: the split's own crossings are pinned
/// by `fix_pattern_mate_crossing`, and what this seat needs is a
/// crossing on the WIRE, which the door is public for.
fn saved_crossing(label: &str) -> (String, RecipeNodeId) {
    let (doc, doc_ref, ids) = instances_and_ref_of_a_stored_part(label, 2);
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            class: ContactClass::Rest,
            outer: face(in_part(ids[0])),
            inner: face(part_face()),
        }],
    };
    let (doc, id) = insert(doc, Node::instantiate_part_with(doc_ref, record));
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("a face-referenced crossing round trips");
    (text, id)
}

/// Retypes one reference of a saved crossing — its KIND and nothing
/// else.
///
/// It is also half the receipt that the typed field costs no bytes: it
/// reaches `kind` INSIDE `["Mate"][side]`, and asserts that object is
/// the bare name's own three fields, so a `FaceName` that stopped
/// being `#[serde(transparent)]` would redden here. The other half is
/// the literal `a_crossings_references_are_bare_names_on_the_wire`
/// pins.
fn retype_crossing(text: &str, instance: RecipeNodeId, side: &str, kind: EntityKind) -> String {
    doctored(text, |wire| {
        let reference = &mut wire["snapshot"]["nodes"][instance.0.to_string()]["InstantiatePart"]["interface"]
            ["crossings"][0]["Mate"][side];
        let mut keys: Vec<&str> = reference
            .as_object()
            .expect("a crossing reference is a bare name object")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            ["kind", "node", "path"],
            "a crossing reference is a bare `StableName` on the wire, unwrapped"
        );
        let field = &mut reference["kind"];
        assert_eq!(
            *field,
            serde_json::json!("Face"),
            "the surgery is aimed at a face reference"
        );
        *field = serde_json::json!(format!("{kind:?}"));
    })
}

/// **A saved crossing reference that is not a face refuses at the load
/// door.** A crossing is written out of two mate heads, each a
/// `SitedFace` over a `FaceName`, so its `outer`/`inner` are face names
/// by construction and the record's type says so.
///
/// There is no edit-door twin, for the reason the mate head's row has
/// none: an edge-referenced crossing is a program that does not compile
/// (`InterfaceCrossing`'s own `compile_fail` row), and
/// `Node::instantiate_part_with` — the split's public door, and the
/// only way to author a non-empty record — takes the typed record, so a
/// FILE is the last place one can be spelled. The rule is asked there
/// by the same one constructor, `FaceName`'s `Deserialize`, so the
/// refusal is the load door's own `Unreadable`.
///
/// Both references and all three non-face kinds, because the type fixes
/// one question and both fields ask it.
#[test]
fn a_saved_crossing_reference_that_is_not_a_face_refuses_at_the_load_door() {
    for kind in [EntityKind::Body, EntityKind::Edge, EntityKind::Vertex] {
        for side in ["outer", "inner"] {
            let (text, instance) = saved_crossing("onepred-crossing");
            let corrupt = retype_crossing(&text, instance, side, kind);
            match load(&corrupt, Tol::witness()) {
                Err(PersistError::Unreadable { detail, .. }) => {
                    assert!(
                        detail.contains(kind_noun(kind)),
                        "the refusal names what the crossing reference denoted, got {detail:?}"
                    );
                }
                other => {
                    panic!("a {kind:?} crossing reference must refuse typed at load, got {other:?}")
                }
            }
        }
    }
}

/// **The control**: the same document with both references naming faces
/// saves, loads and keeps its record — so the row above measures the
/// kind and not the fixture.
///
/// Its assertion is THE RECORD SURVIVES, and the two `let … else`
/// panics are the whole of it: the instance comes back an
/// `InstantiatePart` carrying an interface, and that interface carries
/// exactly one `Mate` crossing. Nothing follows them, because the kind
/// is no longer a runtime question here — the fields are `FaceName`s,
/// so a load that answered otherwise would not typecheck.
#[test]
fn a_face_referenced_crossing_round_trips() {
    let (text, instance) = saved_crossing("onepred-crossing-ok");
    let loaded = load(&text, Tol::witness()).expect("a face-referenced crossing loads");
    let Some(Node::InstantiatePart { interface, .. }) = loaded.doc.node(instance) else {
        panic!("the crossing-bearing instance survives the round trip");
    };
    let [InterfaceCrossing::Mate { .. }] = &interface.crossings[..] else {
        panic!("the one crossing survives the round trip");
    };
}

/// **A crossing's two references are BARE names on the wire**, pinned
/// against a literal.
///
/// `FaceName` is `#[serde(transparent)]`, so fixing the kind in the
/// TYPE costs no bytes and moves no pin — and that is a claim about
/// bytes, which only bytes can hold. `wire_rv_bytes`' variant pins
/// never see a crossing (no fixture there carries an interface
/// record), so this row is where the claim lives: the whole crossing,
/// serialized, against the JSON it must be. A wrapper around either
/// field, or a renamed one, reddens it.
#[test]
fn a_crossings_references_are_bare_names_on_the_wire() {
    let reference = |node: u64| StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(node),
        path: vec![RoleSeg::Cap(editor_core::CapEnd::Start)],
    };
    let crossing = InterfaceCrossing::Mate {
        class: ContactClass::Rest,
        outer: face(reference(3)),
        inner: face(reference(5)),
    };
    assert_eq!(
        serde_json::to_value(&crossing).expect("a crossing serializes"),
        serde_json::json!({
            "Mate": {
                "class": "rest",
                "outer": { "kind": "Face", "node": 3, "path": [{ "Cap": "Start" }] },
                "inner": { "kind": "Face", "node": 5, "path": [{ "Cap": "Start" }] },
            }
        })
    );
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
        &editor_core::RefusingReach,
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
        &editor_core::RefusingReach,
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
        &editor_core::RefusingReach,
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
    let doc = apply(
        &doc,
        &DocEdit::DeleteNode { id: gone },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the extrude has no consumer")
    .doc;
    match apply(
        &doc,
        &DocEdit::ReWitness {
            node: gone,
            witness: witness(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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
    match apply(
        &doc,
        &DocEdit::SetTolerance { eps: 0.0 },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
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
    let name = ParamName::literal("wall");
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
            &editor_core::RefusingReach,
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
        match save(&doc, &[edit.into()], Tol::witness()) {
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
    let name = ParamName::literal("n");
    match apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::continuous(Dimension::Count, 3.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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
        let dim = &mut wire["snapshot"]["params"][name.as_str()]["Continuous"]["dim"];
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

// ---- The parameter name ----
//
// The rule one rung up again: a parameter name is admissible exactly
// when the expression parser reads it back as a reference to that
// parameter, and `ParamName` holds that by construction. The edit door
// therefore has no name check to drift — an inadmissible name cannot
// be spelled into a `DocEdit` — and the load door refuses at the
// token, through the same constructor. The rows pair the constructor's
// answer against the load door's for every shape the rule refuses, and
// pin the constructor's rule to the parser's own reading.

/// The texts the lexer does not read as one identifier, each with the
/// finding `ParamName::new` answers for it: blank, whitespace, a
/// leading digit, an embedded operator, a call's bracket, a character
/// outside the alphabet, and padding.
fn inadmissible_names() -> Vec<(&'static str, ParamNameReason)> {
    let s = str::to_string;
    vec![
        ("", ParamNameReason::Blank),
        ("   ", ParamNameReason::Blank),
        (
            "1 2",
            ParamNameReason::NotAnIdentifier {
                pos: 0,
                found: s("1"),
            },
        ),
        (
            "2width",
            ParamNameReason::NotAnIdentifier {
                pos: 0,
                found: s("2"),
            },
        ),
        (
            "a+b",
            ParamNameReason::NotOneToken {
                pos: 1,
                found: s("+"),
            },
        ),
        (
            "hole r",
            ParamNameReason::NotOneToken {
                pos: 5,
                found: s("r"),
            },
        ),
        (
            "sin(",
            ParamNameReason::NotOneToken {
                pos: 3,
                found: s("("),
            },
        ),
        (
            "width#",
            ParamNameReason::OutsideAlphabet { pos: 5, ch: '#' },
        ),
        (" width ", ParamNameReason::Padded),
    ]
}

/// A document declaring one legal parameter, `width`, and its saved
/// text — the load-door rows re-key that declaration to a refused
/// spelling.
fn saved_with_width() -> (ProfileDoc, String) {
    let doc = ProfileDoc::empty(DocumentId::derive("param-name-door"), Tol::witness());
    let applied = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::literal("width"),
            value: DocParam::continuous(Dimension::Length, 1.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a legal name declares");
    let text = save(&applied.doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    (applied.doc, text)
}

/// Re-keys the snapshot's `width` declaration under `spelling`.
fn rekey_width(text: &str, spelling: &str) -> String {
    doctored(text, |wire| {
        let params = wire["snapshot"]["params"]
            .as_object_mut()
            .expect("the param table is an object");
        let decl = params.remove("width").expect("the fixture declares width");
        params.insert(spelling.to_string(), decl);
    })
}

#[test]
fn a_name_the_parser_cannot_read_back_is_refused_at_the_constructor() {
    for (text, reason) in inadmissible_names() {
        let fault = ParamName::new(text).expect_err(text);
        assert_eq!(fault.offered, text, "the fault carries the text verbatim");
        assert_eq!(fault.reason, reason, "{text:?}");
        let shown = fault.to_string();
        assert!(
            shown.starts_with(&format!("parameter name {text:?} ")),
            "the sentence quotes the bytes offered, as the parse door does: {shown}"
        );
    }
}

#[test]
fn a_name_the_parser_cannot_read_back_is_refused_at_the_load_door() {
    let (_, text) = saved_with_width();
    for (spelling, reason) in inadmissible_names() {
        let fault = ParamName::new(spelling).expect_err(spelling);
        assert_eq!(fault.reason, reason);
        let corrupt = rekey_width(&text, spelling);
        match load(&corrupt, Tol::witness()) {
            // At the token, as an off-table unit symbol refuses: this
            // build's types rejected the key, and the detail is the
            // constructor's own sentence.
            Err(PersistError::Unreadable { detail, .. }) => assert!(
                detail.contains(&fault.to_string()),
                "{spelling:?}: the load door's detail is the constructor's sentence, got {detail}"
            ),
            other => panic!("{spelling:?} must refuse at the token, got {other:?}"),
        }
    }
}

/// The edit LOG carries names too, and the same constructor reads
/// them: a logged declaration under a refused spelling is refused
/// before replay is asked.
#[test]
fn a_logged_declaration_under_a_refused_name_is_refused_at_the_load_door() {
    let (doc, _) = saved_with_width();
    let log = vec![LoggedEdit::bare(DocEdit::SetDocParam {
        name: ParamName::literal("depth"),
        value: DocParam::continuous(Dimension::Length, 2.0),
    })];
    let text = save(&doc, &log, Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let fault = ParamName::new("1 2").expect_err("a spaced number is not a name");
    let corrupt = doctored(&text, |wire| {
        let name = &mut wire["edits"][0]["edit"]["SetDocParam"]["name"];
        assert_eq!(
            *name,
            serde_json::json!("depth"),
            "aimed at the logged name"
        );
        *name = serde_json::json!("1 2");
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => assert!(
            detail.contains(&fault.to_string()),
            "the log's refusal is the same sentence: {detail}"
        ),
        other => panic!("a logged name must refuse at the token, got {other:?}"),
    }
}

/// **The rule is the parser's.** Every admissible spelling parses,
/// alone, to a reference to the parameter of that name — including a
/// function word and a unit symbol, because the grammar has no
/// reserved words: `sin` is a call only when `(` follows it — and no
/// refused spelling parses to a reference to itself.
#[test]
fn a_name_is_admissible_exactly_when_the_parser_reads_it_back() {
    for text in ["width", "hole_r", "_", "x1", "sin", "mm", "pi", "δ"] {
        let name = ParamName::new(text).expect(text);
        let table = BTreeMap::from([(name.clone(), Dimension::Scalar)]);
        assert_eq!(
            parse_expr(text, &table).expect(text),
            Expr::param(name, Dimension::Scalar),
            "{text:?} reads back as itself"
        );
    }
    // The fact that admits a function word: a bare one is looked up
    // as a parameter, and the table decides.
    match parse_expr("sin", &BTreeMap::new()) {
        Err(editor_core::ParseError::UnknownParam { name, .. }) => assert_eq!(name, "sin"),
        other => panic!("a bare function word is a parameter reference, got {other:?}"),
    }
    let table = BTreeMap::from([(ParamName::literal("width"), Dimension::Scalar)]);
    for (text, _) in inadmissible_names() {
        let refs = parse_expr(text, &table).ok().map(|expr| {
            let mut out = Vec::new();
            expr.param_refs(&mut out);
            out
        });
        assert!(
            refs.is_none_or(|refs| refs.iter().all(|(name, _)| name.as_str() != text)),
            "{text:?} must not read back as a reference to itself"
        );
    }
}

/// The literal door is the panicking one, and it panics with the
/// same sentence the fallible door answers.
#[test]
#[should_panic(expected = "parameter name \"1 2\" opens with \"1\" at byte 0")]
fn an_inadmissible_literal_panics_with_the_faults_sentence() {
    let _ = ParamName::literal("1 2");
}

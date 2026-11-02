//! **Document predicates with one home each, asked by both doors.**
//!
//! An assertion's bound against the measure it constrains, an A11
//! placement row, a witness row's key, the
//! recorded ε, a document parameter's declaration and its floats: each
//! rule is stated once — `Node::assertion_bound_fault`,
//! `doc::placement_fault`,
//! `doc::witness_site_fault`, `doc::epsilon_admissible`,
//! `FreeVar::is_continuous_count`, `FreeVar::first_non_finite` — and
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
//! a non-finite placement coordinate can only reach
//! `validate_document` from memory. Every other rule below is
//! reachable from a file and is paired here.
//!
//! **One rule below has no edit-door half, by construction rather than
//! by omission**: a mate pose's face is a `SitedFace` over a
//! `FaceName`, so "this head names a face" is decided by the TYPE and
//! there is no edit to refuse — the row pairs the load door against
//! `SitedFace`'s own `compile_fail` doctest instead, and says so at the
//! site. That is the same property one rung up: not two doors kept in
//! step, but one decision the second door cannot restate differently.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;

use crate::wire::doctored;
use editor_core::CapEnd;
use editor_core::{
    Dimension, DocEdit, DocRef, DocumentId, EditError, EntityKind, FaceName, Formula, FreeVar,
    InterfaceCrossing, InterfaceRecord, Mate, MateRef, Node, PersistError, ProfileDoc,
    RecipeNodeId, RoleSeg, SnapshotError, StableName, VarName, apply, load, save,
};
use editor_core::{VarNameReason, parse_formula};
use fixture::resolver::{PartStore, in_part};
use fixture::{insert, insert_pinned, len, on_frame, square, step};
use geom_core::Tol;
use std::collections::BTreeMap;
use test_utils::fuzz;

// ---- The assertion's bound ----

/// A document carrying a profile (0), an extrude (1)
/// and a LENGTH variable `gap` — the smallest value an assertion
/// bounds, which is all its bound is checked against.
fn with_value() -> (ProfileDoc, editor_core::VarId) {
    let mut r = fixture::Recorder::new();
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let _ = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
    });
    let gap = editor_core::VarName::from_static("gap");
    r.push(DocEdit::DeclareVar {
        name: gap.clone(),
        def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(Dimension::Length, 1.0)),
    });
    let value = r.doc.var_named(gap.as_str()).expect("declared");
    (r.doc, value)
}

fn assertion(value: editor_core::VarId, bound: Formula) -> AuthoredNode {
    Node::Assertion { claim: editor_core::Claim::Value {
        value: Formula::var(value, Dimension::Length),
        bound,
        relation: editor_core::AssertionRelation::AtLeast,
    } }
}

/// **The value is no scalar — both doors.** An assertion reads its
/// value at a dimension, so the edit door refuses the profile's output
/// by kind (`PayloadVarKind`), and the load door the same, read by the
/// payload walk.
#[test]
fn an_assertion_over_a_non_scalar_is_refused_at_both_doors() {
    let (doc, value) = with_value();
    // The profile (the first node) is live and precedes any assertion,
    // so the only thing wrong with the document is that its output is
    // no scalar.
    let profile = doc
        .output(doc.ids()[0], 0)
        .expect("a profile defines its output");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(assertion(profile, len(1.0))),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::PayloadVarKind {
            declared: editor_core::VarKind::Profile,
            ..
        }) => {}
        other => panic!("an assertion over a non-scalar must refuse typed, got {other:?}"),
    }

    // The same fact at the load door: a file whose assertion is
    // re-pointed at the profile.
    let (text, id) = saved_assertion(&doc, value, len(1.0));
    let corrupt = repoint_value(&text, id, value, profile);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PayloadVarKind {
            declared: editor_core::VarKind::Profile,
            ..
        })) => {}
        other => panic!("a non-scalar value must refuse typed at load, got {other:?}"),
    }
}

/// **The bound measures something else — both doors.** The value is a
/// length; the bound is an angle. Both doors carry BOTH
/// dimensions, so the message says what disagrees rather than that
/// something does.
#[test]
fn an_assertion_bound_of_the_wrong_dimension_is_refused_at_both_doors() {
    let (doc, measure) = with_value();
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(assertion(measure, fixture::ang(0.5))),
            fresh: Vec::new(),
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
fn saved_assertion(
    doc: &editor_core::ProfileDoc,
    measure: editor_core::VarId,
    bound: Formula,
) -> (String, RecipeNodeId) {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(assertion(measure, bound)),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("a well-dimensioned assertion inserts");
    let id = applied.record.minted.expect("the insert minted an id");
    let text = save(&applied.doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    (text, id)
}

/// Re-points the assertion's `value` read.
fn repoint_value(
    text: &str,
    assertion: RecipeNodeId,
    from: editor_core::VarId,
    to: editor_core::VarId,
) -> String {
    doctored(text, |wire| {
        let field = &mut wire["snapshot"]["nodes"][assertion.0.to_string()]["Assertion"]["value"];
        assert_eq!(
            *field,
            serde_json::json!(from.0),
            "the surgery is aimed at the assertion's value"
        );
        *field = serde_json::json!(to.0);
    })
}

/// Retypes the assertion's BOUND literal from a length to an angle,
/// unit and all — both halves, so the literal is still one the load
/// door's `Formula::literal_with_unit` rebuild accepts and the refusal
/// read is the DIMENSION rule's.
fn retype_bound(text: &str, assertion: RecipeNodeId) -> String {
    doctored(text, |wire| {
        crate::wire::retype_slot_var(
            wire,
            |wire| &wire["snapshot"]["nodes"][assertion.0.to_string()]["Assertion"]["bound"],
            "Angle",
            "rad",
        );
    })
}

// ---- Instances of a stored part ----

/// **`n` instances of a part document that exists**: the reference is
/// minted by inserting a real part into a [`PartStore`], so its content
/// pin is that part's digest and the faces `in_part` names are faces
/// the part actually has. The part's body comes back beside them.
///
/// That is what distinguishes it from the same-shaped fixture beside
/// `persist::check`'s in-crate rows, which mints a `DocRef` by hand
/// over no part at all: this seat builds mate HEADS, which name a face
/// inside the referenced part, and the validator's seat does not. The
/// two cannot be one function — an integration suite cannot reach a
/// `#[cfg(test)]` item in the library, and the library cannot reach
/// `tests/fixture` — so they are two, named for the difference.
fn instances_of_a_stored_part(
    label: &str,
    n: usize,
) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let (doc, _, ids, body) = instances_and_ref_of_a_stored_part(label, n);
    (doc, ids, body)
}

/// The same fixture, keeping the reference it minted the instances
/// from — for a row that inserts a FURTHER instance of the same part,
/// which is the only way to author one carrying an interface record.
/// Each instance is pinned to the world at the identity.
fn instances_and_ref_of_a_stored_part(
    label: &str,
    n: usize,
) -> (ProfileDoc, DocRef, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(part(&format!("{label}-part")), Tol::witness());
    let opts = fixture::resolver::with_resolver(store);
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert_pinned(doc, crate::fixture::mated_instance(doc_ref), &opts);
        doc = next;
        ids.push(id);
    }
    (doc, doc_ref, ids, body)
}

/// **Two instances of a stored part, the second for a mate to place**:
/// the first pinned to the world at the identity, the second's bundle
/// empty, so a constraint between their faces is the second's and pins
/// it rather than over-constraining a copy already pinned.
fn a_pinned_and_a_mated_instance(label: &str) -> (ProfileDoc, [RecipeNodeId; 2], RecipeNodeId) {
    let (doc, doc_ref, ids, body) = instances_and_ref_of_a_stored_part(label, 1);
    let (doc, mated) = insert(doc, fixture::mated_instance(doc_ref));
    (doc, [ids[0], mated], body)
}

fn part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// The part-local face every head here wears under its instance — the
/// start cap of the part's `body`, as `in_part(.., body, CapEnd::Start)`
/// spells it — so a crossing built here names the same face on both
/// sides of the seam.
fn part_face(body: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(editor_core::CapEnd::Start)],
    }
}

/// `placed`'s start cap resting on `on`'s, both instances of the part
/// whose body is `body`: the mate `placed`'s bundle holds.
fn mate(body: RecipeNodeId, on: RecipeNodeId, placed: RecipeNodeId) -> Mate<Formula> {
    fixture::rest(
        fixture::head_at(placed, in_part(placed, body, CapEnd::Start)),
        fixture::head_at(on, in_part(on, body, CapEnd::Start)),
    )
}

// ---- The mate head's entity kind ----

/// A two-instance document carrying one FACE-TO-FACE mate in the
/// second instance's bundle, saved and loaded once — the control the
/// rows below corrupt, and the round trip in its own right. Answers
/// the text, the mate, and the face selection each of its poses reads,
/// `on` then `to`.
fn saved_mate(label: &str) -> (String, MateRef, [editor_core::VarId; 2]) {
    let (doc, [pinned, mated], body) = a_pinned_and_a_mated_instance(label);
    let (doc, id) = fixture::constrain_at(doc, mated, mate(body, pinned, mated));
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("a face-to-face mate round trips");
    let written = fixture::constraint(&doc, id);
    let to = written.to;
    let face = |pose| match fixture::face_reads(&doc, pose).as_slice() {
        [one] => *one,
        other => panic!("a plane pose reads one face, got {other:?}"),
    };
    (text, id, [face(written.on), face(to)])
}

/// Retypes the name the face selection `read` holds — its KIND and
/// nothing else.
fn retype_head(text: &str, read: editor_core::VarId, kind: EntityKind) -> String {
    doctored(text, |wire| {
        let field =
            &mut wire["snapshot"]["vars"][read.0.to_string()]["def"]["Select"]["names"][0]["kind"];
        assert_eq!(
            *field,
            serde_json::json!("Face"),
            "the surgery is aimed at a face selection"
        );
        *field = serde_json::json!(format!("{kind:?}"));
    })
}

/// **A saved mate pose whose selection names no face refuses at load.**
///
/// A pose reads a `Face` selection, and what each of its names denotes
/// is the name's own kind, so the load door holds it as it holds any
/// selection's (`Select::fault`'s per-name arm): a file whose pose
/// names a body, an edge or a vertex is not one a door writes.
///
/// Both poses and all three non-face kinds.
#[test]
fn a_saved_mate_side_that_is_not_a_face_refuses_at_load() {
    for kind in [EntityKind::Body, EntityKind::Edge, EntityKind::Vertex] {
        for side in 0..2 {
            let (text, _, reads) = saved_mate("onepred-matehead");
            let corrupt = retype_head(&text, reads[side], kind);
            match load(&corrupt, Tol::witness()) {
                Err(PersistError::Snapshot(SnapshotError::SelectionShape { fault, .. })) => {
                    assert_eq!(
                        fault,
                        editor_core::SelectionFault::Kind {
                            at: 0,
                            found: kind,
                            expected: EntityKind::Face,
                        },
                        "a {kind:?} selection on pose {side}"
                    );
                }
                other => {
                    panic!("a {kind:?} selection on pose {side} refuses at load, got {other:?}")
                }
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

/// **The control**: the same document with both poses reading faces
/// saves, loads and keeps its mate — so the rows above measure the
/// kind and not the fixture.
#[test]
fn a_face_to_face_mate_round_trips() {
    let (text, mate, reads) = saved_mate("onepred-matehead-ok");
    let loaded = load(&text, Tol::witness()).expect("a face-to-face mate loads");
    let kept = fixture::constraint(&loaded.doc, mate);
    let to = kept.to;
    assert_eq!(
        [
            fixture::face_reads(&loaded.doc, kept.on),
            fixture::face_reads(&loaded.doc, to)
        ],
        [vec![reads[0]], vec![reads[1]]],
        "each pose reads the face it was saved reading"
    );
    for read in reads {
        assert_eq!(
            crate::fixture::side_name(&loaded.doc, read).kind,
            EntityKind::Face
        );
    }
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
    let (doc, doc_ref, ids, body) = instances_and_ref_of_a_stored_part(label, 2);
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            // An `outer` is a remainder name: spelled on a node this
            // document holds.
            outer: face(crate::fixture::resolver::in_world(ids[0], CapEnd::Start)),
            inner: face(part_face(body)),
        }],
    };
    let (doc, id) = insert(
        doc,
        Node::instantiate_part_with(
            doc_ref,
            record,
            editor_core::Operand::Var(editor_core::VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
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
        node: RecipeNodeId::new(0, node),
        path: vec![RoleSeg::Cap(editor_core::CapEnd::Start)],
    };
    let crossing = InterfaceCrossing::Mate {
        outer: face(reference(3)),
        inner: face(reference(5)),
    };
    assert_eq!(
        serde_json::to_value(&crossing).expect("a crossing serializes"),
        serde_json::json!({
            "Mate": {
                "outer": { "kind": "Face", "node": "0:0000000000000003", "path": [{ "Cap": "Start" }] },
                "inner": { "kind": "Face", "node": "0:0000000000000005", "path": [{ "Cap": "Start" }] },
            }
        })
    );
}

// ---- A11 (2): constraint sites and placement frames ----

/// **A constraint on a node that places nothing — the edit door.** Only
/// a `Place` and an instance hold a bundle, so a world mate aimed at a
/// sketch refuses `ConstraintOnNonPlacement` naming it.
///
/// There is no load-door twin, by construction: a bundle is a field
/// only a placement's node carries on the wire, so a file has no
/// spelling for a constraint on any other node.
#[test]
fn a_constraint_on_a_non_placement_is_refused_at_the_edit_door() {
    let (doc, ids, body) = instances_of_a_stored_part("onepred-site", 1);
    // A live non-placement node, so the refusal is the kind rule's and
    // not a dangling id's.
    let (doc, other) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let cap = fixture::plane(fixture::head_at(
        ids[0],
        in_part(ids[0], body, CapEnd::Start),
    ));
    let on_the_world = Mate {
        on: cap.clone(),
        to: cap,
        orient: editor_core::Orientation::IDENTITY,
    };
    match apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: other,
            mate: Box::new(on_the_world),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::ConstraintOnNonPlacement { node }) => assert_eq!(node.id(), other),
        other => panic!("a constraint on a non-placement must refuse typed, got {other:?}"),
    }
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
fn saved_witness(doc: &editor_core::ProfileDoc, node: RecipeNodeId) -> String {
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
    // `with_value` lays down a profile, an extrude and a variable, in
    // that order: the profile is the only sketch-bearing node in it, and
    // the extrude is a live node that is not one.
    let (doc, _) = with_value();
    let (sketch, non_sketch) = (doc.ids()[0], doc.ids()[1]);
    match apply(
        &doc,
        &DocEdit::ReWitness {
            node: non_sketch,
            witness: witness(),
        },
        Tol::witness(),
    ) {
        Err(EditError::WitnessOnNonSketch { node }) => assert_eq!(node.id(), non_sketch),
        other => panic!("a witness on a non-sketch must refuse typed, got {other:?}"),
    }

    let text = saved_witness(&doc, sketch);
    let corrupt = rekey_witness(&text, sketch, non_sketch);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::WitnessSite { node })) => {
            assert_eq!(node.id(), non_sketch);
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
    let (doc, _) = with_value();
    let (sketch, gone) = (doc.ids()[0], doc.ids()[1]);
    // Deleted rather than invented, so the id stays one the document
    // has minted and the load door's id walk passes it — the refusal read
    // is then the site rule's and not `NodeNotMinted`.
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
        Err(EditError::UnknownNode { id }) => assert_eq!(id.id(), gone),
        other => panic!("a witness on a missing node must refuse typed, got {other:?}"),
    }

    let text = saved_witness(&doc, sketch);
    let corrupt = rekey_witness(&text, sketch, gone);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::WitnessOnMissingNode { node })) => {
            assert_eq!(node.id(), gone);
        }
        other => panic!("a witness on a missing node must refuse typed at load, got {other:?}"),
    }
}

// ---- The recorded ε ----

/// **An ε that is not strictly positive, or below the floor — both
/// doors.** ε parameterizes every predicate band the document is read
/// through, so a zero or negative one leaves every geometric answer
/// undefined, and one below `EPS_MIN` is no run's (D4 ¶4). The edit door
/// names it `InvalidTolerance` and the load door
/// `SnapshotError::EpsilonInvalid`.
///
/// The NON-FINITE half of the same rule is unreachable from a file —
/// JSON carries no such token — and in memory it refuses one walk
/// earlier, as `PersistError::NonFinite` at `NonFiniteSite::Epsilon`.
#[test]
fn an_inadmissible_epsilon_is_refused_at_both_doors() {
    let (doc, _) = with_value();
    match apply(&doc, &DocEdit::SetTolerance { eps: 0.0 }, Tol::witness()) {
        Err(EditError::InvalidTolerance { value }) => assert_eq!(value, 0.0),
        other => panic!("a zero ε must refuse typed, got {other:?}"),
    }
    let below = geom_core::tolerance::EPS_MIN.next_down();
    match apply(&doc, &DocEdit::SetTolerance { eps: below }, Tol::witness()) {
        Err(EditError::InvalidTolerance { value }) => assert_eq!(value, below),
        other => panic!("an ε below the floor must refuse typed, got {other:?}"),
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
    let corrupt = doctored(&text, |wire| {
        wire["snapshot"]["epsilon"] = serde_json::json!(below);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::EpsilonInvalid { value })) => {
            assert_eq!(value, below);
        }
        other => panic!("an ε below the floor must refuse typed at load, got {other:?}"),
    }
}

// ---- A document parameter's floats ----

/// **A continuous parameter carrying a float that is not a number —
/// both doors, naming WHICH float.**
///
/// `FreeVar::first_non_finite` is the one rule and both doors ask it:
/// the nominal first, then the annotation's offsets. The edit door
/// names its answer `EditError::NonFiniteVar` and the load door
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

    let (doc, _) = with_value();
    let name = VarName::from_static("wall");
    let annotated = |sigma: f64| {
        let mut value = FreeVar::continuous(Dimension::Length, 1.0);
        if let FreeVar::Continuous { distribution, .. } = &mut value {
            *distribution = Some(Distribution::Normal { sigma });
        }
        value
    };
    let cases = [
        (
            FreeVar::continuous(Dimension::Length, f64::NAN),
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
            &DocEdit::DeclareVar {
                name: name.clone(),
                def: editor_core::VarDecl::Free(value.clone()),
            },
            Tol::witness(),
        ) {
            Err(EditError::NonFiniteVar { var: n, field }) => {
                assert_eq!(n.name(), Some(&name));
                assert_eq!(field, expected, "the edit door names the offending float");
            }
            other => panic!("a non-finite parameter must refuse typed, got {other:?}"),
        }

        let edit = DocEdit::DeclareVar {
            name: name.clone(),
            def: editor_core::VarDecl::Free(value),
        };
        match save(&doc, &[edit], Tol::witness()) {
            Err(PersistError::NonFinite {
                site: NonFiniteSite::Edit { index: 0, inner },
            }) => match *inner {
                NonFiniteSite::DocParam { var: ref n, field } => {
                    assert_eq!(*n, editor_core::VarRef::Name(name.clone()));
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
/// `FreeVar::is_continuous_count` is the one rule and both doors ask
/// it; this row pins the edit door's answer
/// (`ContinuousVarCannotBeCount`).
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
    let (doc, _) = with_value();
    let name = VarName::from_static("n");
    match apply(
        &doc,
        &DocEdit::DeclareVar {
            name: name.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Count, 3.0)),
        },
        Tol::witness(),
    ) {
        Err(EditError::ContinuousVarCannotBeCount { var: n }) => {
            assert_eq!(n.name(), Some(&name));
        }
        other => panic!("a count-dimensioned continuous param must refuse typed, got {other:?}"),
    }

    // The load door's half: a well-formed LENGTH parameter, retyped to
    // a count on the wire.
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: name.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 3.0)),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let id = doc
            .var_named(name.as_str())
            .expect("declared")
            .0
            .to_string();
        let dim = &mut wire["snapshot"]["vars"][id.as_str()]["def"]["Free"]["Continuous"]["dim"];
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
            var: n,
            ..
        }) => assert_eq!(n.name(), Some(&name)),
        other => panic!("a count-dimensioned continuous param must refuse at load, got {other:?}"),
    }
}

// ---- The parameter name ----
//
// The rule one rung up again: a parameter name is admissible exactly
// when the expression parser reads it back as a reference to that
// parameter, and `VarName` holds that by construction. The edit door
// therefore has no name check to drift — an inadmissible name cannot
// be spelled into a `DocEdit` — and the load door refuses at the
// token, through the same constructor. The rows pair the constructor's
// answer against the load door's for every shape the rule refuses, and
// pin the constructor's rule to the parser's own reading.

/// The texts the lexer does not read as one identifier, each with the
/// finding `VarName::new` answers for it: blank, whitespace, a
/// leading digit, an embedded operator, a call's bracket, a character
/// outside the alphabet, and padding.
fn inadmissible_names() -> Vec<(&'static str, VarNameReason)> {
    let s = str::to_string;
    vec![
        ("", VarNameReason::Blank),
        ("   ", VarNameReason::Blank),
        (
            "1 2",
            VarNameReason::NotAnIdentifier {
                pos: 0,
                found: s("1"),
            },
        ),
        (
            "2width",
            VarNameReason::NotAnIdentifier {
                pos: 0,
                found: s("2"),
            },
        ),
        (
            "a+b",
            VarNameReason::NotOneToken {
                pos: 1,
                found: s("+"),
            },
        ),
        (
            "hole r",
            VarNameReason::NotOneToken {
                pos: 5,
                found: s("r"),
            },
        ),
        (
            "sin(",
            VarNameReason::NotOneToken {
                pos: 3,
                found: s("("),
            },
        ),
        ("width#", VarNameReason::OutsideAlphabet { pos: 5, ch: '#' }),
        (" width ", VarNameReason::Padded),
    ]
}

/// A document declaring one legal parameter, `width`, and its saved
/// text — the load-door rows re-key that declaration to a refused
/// spelling.
fn saved_with_width() -> (ProfileDoc, String) {
    let doc = ProfileDoc::empty(DocumentId::derive("param-name-door"), Tol::witness());
    let applied = apply(
        &doc,
        &DocEdit::DeclareVar {
            name: VarName::from_static("width"),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 1.0)),
        },
        Tol::witness(),
    )
    .expect("a legal name declares");
    let text = save(&applied.doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    (applied.doc, text)
}

/// Re-keys the snapshot's `width` declaration under `spelling`.
fn rekey_width(text: &str, spelling: &str) -> String {
    doctored(text, |wire| {
        let id = crate::wire::wire_var_key(wire, "width");
        wire["snapshot"]["var_names"][id.as_str()] = serde_json::json!(spelling);
    })
}

#[test]
fn a_name_the_parser_cannot_read_back_is_refused_at_the_constructor() {
    for (text, reason) in inadmissible_names() {
        let fault = VarName::new(text).expect_err(text);
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
        let fault = VarName::new(spelling).expect_err(spelling);
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
    let log = vec![DocEdit::DeclareVar {
        name: VarName::from_static("depth"),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 2.0)),
    }];
    let text = save(&doc, &log, Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let fault = VarName::new("1 2").expect_err("a spaced number is not a name");
    let corrupt = doctored(&text, |wire| {
        let name = &mut wire["edits"][0]["DeclareVar"]["name"];
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

/// The constructor admits exactly what the parser reads back, both
/// ways. The oracle is the parser asked with nothing declared: the
/// whole text is one unresolved reference to exactly that text. An
/// admitted text is also asked the second reading — declared, it
/// parses to `Param(name)` — so an admitted name is one an expression
/// can refer to.
fn agrees(text: &str, replay: &str) {
    let shown = if text.chars().count() > 40 {
        let head: String = text.chars().take(40).collect();
        format!("{head:?}… ({} bytes)", text.len())
    } else {
        format!("{text:?}")
    };
    let read_back = matches!(
        parse_formula(text, &BTreeMap::new()),
        Err(editor_core::ParseError::UnknownParam { ref name, .. }) if name == text
    );
    match VarName::new(text) {
        Ok(name) => {
            assert!(read_back, "{shown} is admitted but not read back{replay}");
            let table = BTreeMap::from([(name.clone(), Dimension::Scalar)]);
            assert!(
                parse_formula(text, &table) == Ok(Formula::named(name, Dimension::Scalar)),
                "{shown} is admitted, but declared it does not read back as itself{replay}"
            );
        }
        Err(fault) => {
            assert!(
                !read_back,
                "{shown} is read back but refused: {fault}{replay}"
            );
            assert!(
                fault.offered == text,
                "{shown}: the fault carries the text verbatim"
            );
        }
    }
}

/// **The rule is the parser's**, over a table of edge spellings. A
/// function word and a unit symbol are admitted because the grammar
/// has no reserved words (`sin` is a call only when `(` follows it) and
/// no constants.
#[test]
fn a_name_is_admissible_exactly_when_the_parser_reads_it_back() {
    for text in ["width", "hole_r", "_", "x1", "sin", "mm", "pi", "δ"] {
        assert!(VarName::new(text).is_ok(), "{text:?} is admitted");
    }
    let long = "a".repeat(100_000);
    let long_bad = format!("{}-", "a".repeat(10_000));
    let table = [
        "",
        " ",
        "\t",
        "\n",
        "   ",
        "\u{a0}",
        "\u{2003}",
        "\u{200b}",
        "\u{feff}",
        "1",
        "1a",
        "2width",
        "1e3",
        "1e",
        "0x",
        "_",
        "__",
        "_1",
        "a_",
        "_a_b_",
        "δ",
        "Δx",
        "ñ",
        "日本",
        "ß",
        "Ⅷ",
        "x²",
        "x٣",
        "٣x",
        "a\u{301}",
        "\u{301}a",
        "é",
        "e\u{0345}",
        "sin",
        "cos",
        "atan2",
        "scalar",
        "pi",
        "mm",
        "rad",
        "deg",
        "e",
        "E",
        "inf",
        "nan",
        "NaN",
        "Infinity",
        &long,
        &long_bad,
        "a\0",
        "\0",
        "a\0b",
        "width\n",
        "\nwidth",
        "width\r\n",
        " width",
        "width ",
        "a b",
        "a+b",
        "a-b",
        "a.b",
        "a,b",
        "a(",
        "a)",
        "(a)",
        "-a",
        "a#",
        "a:b",
        "query:certified-range:1:distance",
        "query_certified_range_1",
        "a·b",
        "·",
        "sin(x)",
        "pi rad",
        "a\u{2028}",
    ];
    for text in table {
        agrees(text, "");
    }
    for (text, _) in inadmissible_names() {
        agrees(text, "");
    }
}

/// The same equivalence as a counterexample search over an alphabet
/// chosen to sit on the lexer's edges: identifier characters, digits,
/// every operator, whitespace of three kinds, characters outside the
/// alphabet, a combining mark, and non-ASCII letters and digits.
#[test]
fn a_name_is_admissible_exactly_when_the_parser_reads_it_back_over_edge_characters() {
    const ALPHABET: &[char] = &[
        'a', 'Z', 'e', 'E', '_', '0', '9', ' ', '\t', '\n', '\u{a0}', '+', '-', '*', '/', '(', ')',
        ',', '.', '#', ':', '·', 'δ', '\u{301}', '\0', 'é', '²', '٣',
    ];
    let mut rng = fuzz::start("param name vs parser read-back (edge characters)");
    for _ in 0..fuzz::scaled(4096) {
        let len = rng.below(9);
        let text: String = (0..len)
            .map(|_| ALPHABET[rng.below(ALPHABET.len())])
            .collect();
        agrees(&text, &format!(" — {}", fuzz::replay()));
    }
}

/// The same equivalence over any Unicode scalar value, half the draws
/// from ASCII so short texts mix the two.
#[test]
fn a_name_is_admissible_exactly_when_the_parser_reads_it_back_over_any_scalar() {
    let mut rng = fuzz::start("param name vs parser read-back (any scalar)");
    for _ in 0..fuzz::scaled(4096) {
        let len = rng.below(7);
        let text: String = (0..len)
            .map(|_| {
                loop {
                    let bound = if rng.below(2) == 0 { 0x80 } else { 0x11_0000 };
                    if let Some(c) = u32::try_from(rng.below(bound))
                        .ok()
                        .and_then(char::from_u32)
                    {
                        break c;
                    }
                }
            })
            .collect();
        agrees(&text, &format!(" — {}", fuzz::replay()));
    }
}

/// The `from_static` door is the panicking one, and it panics with the
/// same sentence the fallible door answers.
#[test]
#[should_panic(expected = "parameter name \"1 2\" opens with \"1\" at byte 0")]
fn an_inadmissible_static_name_panics_with_the_faults_sentence() {
    let _ = VarName::from_static("1 2");
}

//! **Three document predicates, one home each, asked by both doors.**
//!
//! An assertion's bound against the measure it constrains, a mate's
//! alignment datum, and an A11 placement row: each rule is stated once
//! — `Node::assertion_bound_fault`, `Node::has_non_finite_alignment`,
//! `doc::placement_fault` — and the edit door and the persistence
//! door each render the one answer in their own vocabulary. A
//! document the edit door accepts is therefore a document that saves,
//! and a file that loads is a file the edit door could have produced.
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
//! `validate_document` from memory.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, ContactClass, Dimension, DocEdit, DocumentId, EditError, EntityKind,
    Expr, Frame, MateFrame, MatePrimitive, MeasureExpr, Node, PersistError, ProfileDoc,
    ProfileProgram, RecipeNodeId, RoleSeg, SitedRef, SnapshotError, StableName, apply, load, save,
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
    let text = saved_assertion(&doc, measure, len(1.0));
    let corrupt = repoint_measure(&text, measure, frame_node);
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
    let text = saved_assertion(&doc, measure, len(0.5));
    let corrupt = retype_bound(&text);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AssertionBound {
            measured: Dimension::Length,
            bound: Dimension::Angle,
            ..
        })) => {}
        other => panic!("a mismatched bound must refuse typed at load, got {other:?}"),
    }
}

/// The saved text of `doc` plus one well-formed assertion, which the
/// rows above then corrupt — so a refusal they read is the surgery's
/// and not the fixture's.
fn saved_assertion(doc: &ProfileDoc, measure: RecipeNodeId, bound: Expr) -> String {
    let doc = apply(
        doc,
        &DocEdit::InsertNode {
            node: assertion(measure, bound),
        },
        Tol::witness(),
    )
    .expect("a well-dimensioned assertion inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    text
}

/// Re-points the assertion's `measure` field — the unique `"measure":`
/// key, since the fixture carries one assertion.
fn repoint_measure(text: &str, from: RecipeNodeId, to: RecipeNodeId) -> String {
    let needle = format!("\"measure\": {}", from.0);
    assert_eq!(
        text.matches(&needle).count(),
        1,
        "{needle:?} must be unique"
    );
    text.replace(&needle, &format!("\"measure\": {}", to.0))
}

/// Retypes the assertion's BOUND literal from a length to an angle,
/// unit and all — the last literal in the file, the assertion being
/// the last node.
fn retype_bound(text: &str) -> String {
    let at = text
        .rfind("\"dim\": \"Length\"")
        .expect("the bound is a length");
    let end = text[at..].find('}').expect("the literal closes") + at;
    let tail = text[at..end]
        .replace("\"Length\"", "\"Angle\"")
        .replace("\"m\"", "\"rad\"");
    format!("{}{tail}{}", &text[..at], &text[end..])
}

// ---- A mate's alignment ----

/// An assembly of `n` instances of one part, and the frames a mate
/// between two of them needs.
fn assembly(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>) {
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
    let (doc, ids) = assembly("onepred-align", 2);
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
    let (doc, ids) = assembly("onepred-site", 1);
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
    let (doc, ids) = assembly("onepred-improper", 1);
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
    let corrupt = mirror_first_column(&text);
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
    let (doc, ids) = assembly("onepred-gauge", 2);
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
fn mirror_first_column(text: &str) -> String {
    let at = text
        .find("\"columns\"")
        .expect("the frame reaches the wire");
    let one = at
        + text[at..]
            .find("1.0")
            .expect("the first column starts at 1");
    let out = format!("{}-1.0{}", &text[..one], &text[one + 3..]);
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// Re-keys the single placement row.
fn rekey_placement(text: &str, from: RecipeNodeId, to: RecipeNodeId) -> String {
    let needle = format!("\"{}\": {{", from.0);
    let at = text
        .find("\"placements\"")
        .expect("the registry reaches the wire");
    let key = at + text[at..].find(&needle).expect("the row is keyed");
    format!(
        "{}\"{}\": {{{}",
        &text[..key],
        to.0,
        &text[key + needle.len()..]
    )
}

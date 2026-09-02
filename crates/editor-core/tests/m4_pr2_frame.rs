//! The SKETCH FRAME datum: an oriented plane, and the orthonormal
//! frame its two authored directions become.
//!
//! A [`Datum::Plane`] pins five of a placement's six rigid degrees of
//! freedom — the sixth, the spin about the normal, is what a sketch's
//! x and y axes are. These rows are about that sixth: that it survives
//! evaluation, that a frame is not a plane wearing a hat, and that the
//! pair which cannot span a plane refuses rather than producing one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Axis3, CancelToken, Datum, DatumValue, Dimension, EvalOptions, Evaluation, Node, NodeErrorKind,
    NodeResult, ProfileDoc, SlotId, ValuePayload, evaluate,
};
use fixture::{insert, len, scl};
use geom_core::{Tol, Vec3};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// One frame node in a fresh document, from raw component triples.
fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> (ProfileDoc, editor_core::RecipeNodeId) {
    insert(
        ProfileDoc::empty_derived("m4_pr2_frame", Tol::witness()),
        Node::Datum(Datum::Frame {
            origin: origin.map(len),
            u: u.map(scl),
            v: v.map(scl),
        }),
    )
}

/// The frame a document evaluated to — x axis, y axis and the normal
/// the pair derives — or a panic naming what it got.
fn evaluated(doc: &ProfileDoc, id: editor_core::RecipeNodeId) -> (Vec3<f64>, Vec3<f64>, Vec3<f64>) {
    match run(doc).nodes.get(&id) {
        Some(NodeResult::Ok(val)) => match &val.payload {
            ValuePayload::Datum(DatumValue::Frame { u, v, .. }) => {
                (u.get(), v.get(), DatumValue::frame_normal(*u, *v))
            }
            other => panic!("expected a frame value, got {other:?}"),
        },
        other => panic!("expected Ok, got {other:?}"),
    }
}

fn dot(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

/// **The pair a frame is built from comes out orthonormal**, whatever
/// was typed.
///
/// The y this authors is neither unit (length 3) nor square to x (it
/// leans a long way along it), which is what a person dragging two
/// vector fields produces. Both properties are the frame's contract —
/// `SketchPlane` is rigid by convention, and a sketch drawn on a
/// non-rigid frame would come out sheared and scaled — so both are
/// asserted, not just the one the arithmetic makes obvious.
#[test]
fn an_authored_pair_is_orthonormalized() {
    let (doc, id) = frame([0.0; 3], [2.0, 0.0, 0.0], [3.0, 3.0, 0.0]);
    let (u, v, _) = evaluated(&doc, id);
    assert!((u.norm() - 1.0).abs() < 1e-12, "x axis is unit: {u:?}");
    assert!((v.norm() - 1.0).abs() < 1e-12, "y axis is unit: {v:?}");
    assert!(dot(u, v).abs() < 1e-12, "the axes are square: {u:?} {v:?}");
    // The AUTHORED x survives; y is the axis that yielded. Squaring
    // the other way round would rotate every profile drawn on this
    // frame when only y was edited, which is the failure this pins.
    assert!((u.x - 1.0).abs() < 1e-12 && u.y.abs() < 1e-12, "{u:?}");
    assert!(v.y > 0.0, "y keeps the side it was authored on: {v:?}");
}

/// **The spin about the normal is the datum a plane cannot carry.**
///
/// Two frames on the SAME surface, one turned a quarter turn from the
/// other: identical normals, different x axes. This is the whole
/// reason the variant exists, so it is asserted directly rather than
/// left to follow from the orthonormalization row.
#[test]
fn two_frames_on_one_surface_differ_by_their_spin() {
    let (doc_a, a) = frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let (doc_b, b) = frame([0.0; 3], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0]);
    let (ua, _, na) = evaluated(&doc_a, a);
    let (ub, _, nb) = evaluated(&doc_b, b);
    assert!(
        (na - nb).norm() < 1e-12,
        "the same surface: {na:?} vs {nb:?}"
    );
    assert!(
        (ua - ub).norm() > 0.5,
        "turned differently on it: {ua:?} vs {ub:?}"
    );
}

/// **A pair that spans no plane refuses, typed, naming which axis.**
///
/// Parallel u and v is the authoring mistake a frame has and a plane
/// does not: it is not a degenerate INPUT (both vectors are perfectly
/// good directions), it is a degenerate PAIR. Gram-Schmidt turns that
/// into a decided-zero length, so it lands on the same door every
/// other direction does, under the y axis's role.
#[test]
fn a_pair_that_spans_no_plane_refuses() {
    let (doc, id) = frame([0.0; 3], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    match run(&doc).nodes.get(&id) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                e.kind,
                NodeErrorKind::DegenerateDirection {
                    role: "datum frame y axis"
                }
            ),
            "{:?}",
            e.kind
        ),
        other => panic!("expected Failed, got {other:?}"),
    }

    // And the x axis has its own role: one role reaching the door
    // proves nothing about the other.
    let (doc, id) = frame([0.0; 3], [0.0; 3], [0.0, 1.0, 0.0]);
    match run(&doc).nodes.get(&id) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                e.kind,
                NodeErrorKind::DegenerateDirection {
                    role: "datum frame x axis"
                }
            ),
            "{:?}",
            e.kind
        ),
        other => panic!("expected Failed, got {other:?}"),
    }
}

/// **A frame carries nine slots, and each is addressable.**
///
/// The slot vocabulary is what the editor drives a node through (D5
/// naming, the panel's fields, a range probe), so a variant whose
/// slots were declared but not wired would be editable in the type and
/// dead in the app.
#[test]
fn every_frame_slot_is_declared_and_reachable() {
    let (doc, id) = frame([1.0, 2.0, 3.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let node = doc.node(id).expect("the frame node");
    let slots = node.slots();
    assert_eq!(slots.len(), 9, "{slots:?}");
    for slot in &slots {
        assert!(node.expr(*slot).is_some(), "unreachable slot {slot:?}");
    }
    // The three families, spelled: a frame is an origin plus two
    // directions, and the dimensions say which is which.
    for axis in Axis3::ALL {
        assert_eq!(SlotId::Origin(axis).dimension(), Dimension::Length);
        assert_eq!(SlotId::U(axis).dimension(), Dimension::Scalar);
        assert_eq!(SlotId::V(axis).dimension(), Dimension::Scalar);
    }
}

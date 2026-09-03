//! R1 probes, part 2 — deviation 2's unreachability claim, executed.
//!
//! The disclosed deviation: the kernel door's non-unit-axis verdict is
//! unreachable along the recipe path, because `wire_datum` decides
//! `DATUM_UNIT_NORM` upstream. Two rows execute both halves: a
//! non-unit datum direction is NORMALIZED upstream (the tube builds,
//! and the door's non-unit refusal never fires), and a degenerate
//! direction refuses AT THE DATUM node, one node upstream, never as
//! `NodeErrorKind::Tube`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use corpus::{eval, failures};
use editor_core::{
    Datum, Dimension, DocEdit, Expr, Node, NodeErrorKind, NodeResult, ProfileDoc, ProfileProgram,
    TubeWindow, apply,
};
use fixture::len;
use geom_core::Tol;

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}

fn push(d: &ProfileDoc, e: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(d, e, Tol::witness()).expect("edit applies").doc
}

fn doc_with_axis_dir(dir: [f64; 3]) -> (ProfileDoc, editor_core::RecipeNodeId, editor_core::RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived("r1_probe2", Tol::witness());
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Axis {
                origin: [len(0.0), len(0.0), len(0.0)],
                direction: dir.map(scalar),
            }),
        },
    );
    let spine = *doc.order().last().expect("datum");
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Tube {
                spine,
                u_ref: [scalar(1.0), scalar(0.0), scalar(0.0)],
                major_radius: len(2.0),
                window: TubeWindow::Full,
                minor_radius: len(0.5),
            },
        },
    );
    let tube = *doc.order().last().expect("tube");
    (doc, spine, tube)
}

/// A non-unit (but sound) axis direction is normalized by the datum
/// node, so the tube BUILDS — the door's own non-unit verdict is
/// bypassed by construction, exactly as disclosed.
#[test]
fn a_non_unit_datum_axis_is_normalized_upstream_and_the_tube_builds() {
    let (doc, _, _) = doc_with_axis_dir([0.0, 0.0, 2.0]);
    let ev = eval::<f64>(&doc);
    assert!(
        failures(&ev).is_empty(),
        "a non-unit datum direction should normalize upstream, got {:?}",
        failures(&ev)
    );
}

/// A degenerate direction refuses AT THE DATUM, one node upstream —
/// never as a tube refusal — and the tube node reports the poisoned
/// ancestor rather than inventing its own verdict.
#[test]
fn a_degenerate_datum_axis_refuses_upstream_of_the_tube_door() {
    let (doc, spine, tube) = doc_with_axis_dir([0.0, 0.0, 0.0]);
    let ev = eval::<f64>(&doc);
    match ev.nodes.get(&spine) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(e.kind, NodeErrorKind::DegenerateDirection { .. }),
            "the datum's own refusal, not the tube door's: {:?}",
            e.kind
        ),
        other => panic!("the datum must refuse, got {other:?}"),
    }
    match ev.nodes.get(&tube) {
        Some(NodeResult::Failed(e)) => assert!(
            !matches!(e.kind, NodeErrorKind::Tube(_)),
            "the tube node must not reach the door: {:?}",
            e.kind
        ),
        Some(NodeResult::Poisoned { .. }) | None => {}
        other => panic!("the tube must not build over a dead spine, got {other:?}"),
    }
}

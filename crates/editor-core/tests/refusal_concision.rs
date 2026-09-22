//! **A refusal the viewer shows fits where it is shown.**
//!
//! The viewer renders a failed node's `NodeError` `Display` verbatim on
//! the feature tree's fault line and the status line, so the sentence a
//! kernel door writes is the sentence the person holding the mouse
//! reads. These rows build the case that sentence must serve and read
//! it back: what failed, the short reason, and the recourse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::eval;
use crate::fixture::{Recorder, frame, len, scl};

use editor_core::{BooleanOp, Datum, LoopProgram, Node, NodeResult, ProfileProgram, TubeWindow};

/// A ring torus (R = 2, r = 0.5, about z) unioned with a block that
/// straddles its tube at +x: a torus face against the block's plane
/// faces, the shape of two dumbbell halves joined at a torus bell.
fn torus_block_union_refusal() -> String {
    let mut r = Recorder::new();
    let spine = r.insert(Node::Datum(Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let ring = r.insert(Node::Tube {
        spine,
        u_ref: [scl(1.0), scl(0.0), scl(0.0)],
        major_radius: len(2.0),
        window: TubeWindow::Full,
        minor_radius: len(0.5),
    });
    let plane = r.insert(frame([0.0, 0.0, -0.25], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let block_p = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![
            LoopProgram::polygon([(1.75, -0.25), (3.0, -0.25), (3.0, 0.25), (1.75, 0.25)]).unwrap(),
        ],
    }));
    let block = r.insert(Node::Extrude {
        profile: block_p,
        distance: len(0.5),
    });
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: ring,
        b: block,
        declare: None,
    });
    let ev = eval::<f64>(&r.doc);
    match ev.nodes.get(&union) {
        Some(NodeResult::Failed(e)) => e.to_string(),
        other => panic!("the torus × block union must refuse; got {other:?}"),
    }
}

/// **The worked example**: two solids joined where a torus face meets a
/// plane face. The sentence names the pair in the user's terms, keeps
/// the box test's MAY ("may meet"), and ends on the recourse. The word
/// budget is the concision claim itself: the same refusal was 277 words
/// of routing and dispatch detail, which now lives in
/// `topo::BooleanError::CurvedPairUnsupported`'s rustdoc.
#[test]
fn the_torus_plane_union_refusal_is_short_and_ends_on_its_recourse() {
    let msg = torus_block_union_refusal();
    assert!(
        msg.contains(
            "the Boolean op refused: the first operand's torus face may meet the second \
             operand's plane face"
        ),
        "the refusal names the pair by operand, as a may: {msg}"
    );
    assert!(
        msg.ends_with(
            "Recourse: reshape the parts so they meet only where a plane face meets a \
             plane, cylinder or sphere face, or move them so the torus face stays clear \
             of the other solid"
        ),
        "the refusal ends on its recourse: {msg}"
    );
    assert!(
        !msg.contains("coincidence"),
        "a torus × plane pair is not a coincidence refusal, and the wrapper must not \
         point the reader at that recourse: {msg}"
    );
    assert!(!msg.contains("FaceKey("), "no arena key dump: {msg}");
    let words = msg.split_whitespace().count();
    assert!(words <= 70, "{words} words: {msg}");
}

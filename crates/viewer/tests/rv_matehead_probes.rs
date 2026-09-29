//! **The viewer's boundary between a picked face and a mate head.**
//!
//! A mate head is a `FaceName`, and the viewer is one of the three
//! places that turn data into one. Its input is a `FaceSelection`,
//! whose `name` is a bare `StableName`: a face by the PICKING DOOR's
//! rule (`SelectionRefusal::NotAFace`, refused before a selection
//! exists) and not by its type, with every field public. So the mate
//! tool asks the head constructor and answers its refusal typed, in
//! every build — the two rows below are that answer and its sentence.
//!
//! `work/view/face-selection-carries-a-bare-stable-name` is the row
//! that removes the question: `FaceSelection::name` becomes a
//! `FaceName`, made where the pick is made, and
//! `MateToolError::PickIsNotAFace` goes away with the constructor call
//! it answers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::asm;
use pncad::document::MateSide;
use pncad::geom_core::Tol;
use pncad::select::EntityKind;
use viewer::matetool::{MateTool, MateToolError};

/// **An edge-named selection is refused, not asserted against.** The
/// tool's own public doors take a `FaceSelection` by value, and
/// nothing but the picking door's rule keeps its `name` a face. A
/// selection that names an edge is answered `PickIsNotAFace` — naming
/// the side and carrying the head constructor's refusal — in debug and
/// in release alike.
#[test]
fn an_edge_named_pick_is_refused_by_the_mate_tool() {
    let tol = Tol::witness();
    let bench = asm::bench("rv-matehead", tol);
    let session = asm::open_bench(&bench, tol);
    let (mut a, b) = asm::seat_picks(&session, &bench);
    assert_eq!(a.name.kind, EntityKind::Face);
    // The one field the picking door's rule lives in, and nothing but
    // that rule keeps it a face.
    a.name.kind = EntityKind::Edge;
    let mut tool = MateTool::new();
    tool.pick(a);
    tool.pick(b);
    let (doc, eval) = session.landed_pair().expect("landed");
    match tool.proposal(doc, eval, &session.eval_options(), tol, asm::seat_choice()) {
        Err(MateToolError::PickIsNotAFace { side, refusal }) => {
            assert_eq!(side, MateSide::A);
            assert_eq!(refusal.found, EntityKind::Edge);
        }
        other => panic!("an edge-named pick must refuse as the kind, got {other:?}"),
    }
}

/// **The refusal says the kind, in the constructor's own words.** The
/// operand arm (`NotAnInstancePick`) names a node and says nothing
/// about what the selection denoted, so it is the wrong sentence for
/// this mistake; the kind arm forwards `NotAFaceName`'s rendering
/// rather than re-spelling it.
#[test]
fn the_kind_refusal_forwards_the_head_constructors_sentence() {
    let refusal = pncad::document::FaceName::new(pncad::prelude::StableName {
        kind: EntityKind::Edge,
        node: pncad::document::RecipeNodeId(0),
        path: Vec::new(),
    })
    .expect_err("an edge is not a face name");
    let shown = MateToolError::PickIsNotAFace {
        side: MateSide::B,
        refusal,
    }
    .to_string();
    assert!(
        shown.contains(&refusal.to_string()) && shown.contains("an edge"),
        "the kind refusal drops the constructor's sentence: {shown:?}"
    );

    let operand = MateToolError::NotAnInstancePick {
        side: MateSide::A,
        node: pncad::document::RecipeNodeId(0),
    }
    .to_string();
    assert!(
        !operand.contains("face") && !operand.contains("edge"),
        "the operand arm must stay silent about the kind: {operand:?}"
    );
}

//! **An authored step reaches its id through its address**
//! (`names/README.md`, "N1, the profile pieces", "The id."):
//! `AuthoredStep` is the step's index and its loop's shape up to it,
//! values erased, and a profile's program binds it to the id that
//! placement minted, against the loop the author states.
//!
//! The chain is the README's own: `at`, `toward(+x)`, `fillet(r)`,
//! `toward(+y)`, `to (2, 2)`, `line_to(0, 2)`, `line_to(Start)`, whose
//! right wall is the fillet's `RunOut`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/step_handle.rs",
    "crates/editor-core/src/program.rs",
    "crates/profile/src/structure.rs",
    "crates/editor-core/tests/fixture/",
];

use crate::fixture::{self, insert, tol};
use editor_core::{
    AuthoredStep, DocEdit, EditError, LoopProgram, Node, ParamEnv, PieceRole, ProfileDoc,
    ProfileEdgeRef, ProfileProgram, RecipeNodeId, StepHandleRefusal, StepIdFault, keep_grid,
};
use geom_core::{Point2, Tol};
use profile::{Open, Start, Step, Verb};

/// The handles an author holds after writing the chain, beside the
/// recording it closes to.
struct Authored {
    fillet: AuthoredStep,
    top: AuthoredStep,
    close: AuthoredStep,
    program: Vec<Step<f64>>,
}

/// The chain at size `s` (the far corner at `(s, s)`), handles taken
/// where the author writes each verb.
fn authored(s: f64) -> Authored {
    let t = Tol::witness();
    let filleted = Open
        .at(Point2::new(0.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(0.5, t)
        .unwrap();
    let fillet = AuthoredStep::after(filleted.recorded()).unwrap();
    let topped = filleted
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(Point2::new(s, s), t)
        .unwrap()
        .line_to(Point2::new(0.0, s), t)
        .unwrap();
    let top = AuthoredStep::after(topped.recorded()).unwrap();
    let closed = topped.line_to(Start, t).unwrap();
    let close = AuthoredStep::after(&closed.program).unwrap();
    Authored {
        fillet,
        top,
        close,
        program: closed.program,
    }
}

fn lifted(program: &[Step<f64>]) -> LoopProgram {
    LoopProgram::from_recorded(program).unwrap()
}

/// A document holding the chain placed as two profiles on one frame.
fn placed_twice(loop_: &LoopProgram) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("step-handle-binding", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let profile = |doc| {
        insert(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![loop_.clone()],
                ids: Vec::new(),
            }),
        )
    };
    let (doc, a) = profile(doc);
    let (doc, b) = profile(doc);
    (doc, a, b)
}

fn program_of(doc: &ProfileDoc, node: RecipeNodeId) -> &ProfileProgram {
    match doc.node(node) {
        Some(Node::Profile(p)) => p,
        other => panic!("node {node:?} is not a profile: {other:?}"),
    }
}

#[test]
fn a_handle_binds_to_the_id_its_placement_minted() {
    let a = authored(2.0);
    assert_eq!(
        (a.fillet.index(), a.top.index(), a.close.index()),
        (2, 5, 6),
        "a handle's index is its step's place in the recording"
    );
    let (doc, first, second) = placed_twice(&lifted(&a.program));
    let (p, q) = (program_of(&doc, first), program_of(&doc, second));
    assert_eq!(
        p.step(0, &a.top),
        Ok(p.ids[0][5]),
        "the top leg binds to step 5's id"
    );
    assert_eq!(
        p.step(0, &a.close),
        Ok(p.ids[0][6]),
        "the closer binds to step 6's id"
    );
    assert_ne!(
        p.step(0, &a.top),
        q.step(0, &a.top),
        "one loop placed twice resolves one handle to two ids"
    );
    // The README's right wall: the fillet's run out, drawn and named.
    let wall = p.piece(0, &a.fillet, PieceRole::RunOut).unwrap();
    assert_eq!(
        wall,
        ProfileEdgeRef::Piece {
            step: p.ids[0][2],
            role: PieceRole::RunOut
        }
    );
    let drawn = p.pieces(&ParamEnv::default(), tol()).unwrap();
    assert!(
        drawn.edges[0].contains(&wall),
        "the run out is a drawn piece: {drawn:?}"
    );
}

#[test]
fn a_value_edit_leaves_a_handle_valid_and_a_reshape_refuses_it() {
    let small = authored(2.0);
    let large = authored(3.0);
    assert_eq!(small.top, large.top, "values are erased from the address");
    let (doc, first, _) = placed_twice(&lifted(&large.program));
    let p = program_of(&doc, first);
    assert_eq!(p.step(0, &small.top), Ok(p.ids[0][5]));

    // The same chain with its first direction bound by `angle`: a
    // different prefix, so every handle past step 0 is off it.
    let t = Tol::witness();
    let reshaped = Open
        .at(Point2::new(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .fillet(0.5, t)
        .unwrap()
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .line_to(Point2::new(0.0, 2.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
        .program;
    let lp = lifted(&reshaped);
    assert!(
        !small.top.is_in(&lp),
        "a reshaped prefix is not the handle's"
    );
    assert!(
        AuthoredStep::of_program(&lp, 0).is_some_and(|h| h.is_in(&lifted(&small.program))),
        "the shared first step is"
    );
    assert_eq!(
        p.step(1, &small.top),
        Err(StepHandleRefusal::OffProgram { loop_: 1, index: 5 }),
        "a loop the program does not have"
    );
}

#[test]
fn a_piece_door_refuses_a_role_its_verb_never_draws() {
    let a = authored(2.0);
    let (doc, first, _) = placed_twice(&lifted(&a.program));
    let p = program_of(&doc, first);
    assert_eq!(
        p.piece(0, &a.top, PieceRole::Arc),
        Err(StepHandleRefusal::RoleNotDrawn {
            verb: Verb::LineTo,
            role: PieceRole::Arc
        })
    );
    assert_eq!(
        p.piece(0, &a.fillet, PieceRole::Leg),
        Err(StepHandleRefusal::RoleNotDrawn {
            verb: Verb::Fillet,
            role: PieceRole::Leg
        })
    );
    let bind = AuthoredStep::of_program(&p.loops[0], 1).unwrap();
    assert!(
        matches!(
            p.piece(0, &bind, PieceRole::Leg),
            Err(StepHandleRefusal::RoleNotDrawn { .. })
        ),
        "a binder draws nothing"
    );
    // A role the verb draws but the values do not: still a piece.
    assert!(p.piece(0, &a.fillet, PieceRole::RunIn).is_ok());
    // Unminted: the program before it entered the document.
    let free = ProfileProgram {
        plane: p.plane,
        loops: p.loops.clone(),
        ids: Vec::new(),
    };
    assert_eq!(free.step(0, &a.top), Err(StepHandleRefusal::Unminted));
}

#[test]
fn a_keep_map_lowers_to_the_grid_the_door_stores() {
    let a = authored(2.0);
    let (doc, first, _) = placed_twice(&lifted(&a.program));
    let old = program_of(&doc, first).clone();
    let top_id = old.step(0, &a.top).unwrap();
    let fillet_id = old.step(0, &a.fillet).unwrap();

    // The reshaped chain: one more leg before the close.
    let t = Tol::witness();
    let base = Open
        .at(Point2::new(0.0, 0.0))
        .toward(1.0, 0.0, t)
        .unwrap()
        .fillet(0.5, t)
        .unwrap();
    let fillet = AuthoredStep::after(base.recorded()).unwrap();
    let topped = base
        .toward(0.0, 1.0, t)
        .unwrap()
        .to(Point2::new(2.0, 2.0), t)
        .unwrap()
        .line_to(Point2::new(0.0, 2.0), t)
        .unwrap();
    let top = AuthoredStep::after(topped.recorded()).unwrap();
    let program = topped
        .line_to(Point2::new(-0.5, 1.0), t)
        .unwrap()
        .line_to(Start, t)
        .unwrap()
        .program;
    let loops = vec![lifted(&program)];

    let grid = keep_grid(&loops, &[vec![(fillet, fillet_id), (top.clone(), top_id)]]).unwrap();
    assert_eq!(
        grid,
        vec![vec![
            None,
            None,
            Some(fillet_id),
            None,
            None,
            Some(top_id),
            None,
            None
        ]]
    );
    let edit = DocEdit::SetProgram {
        node: first,
        loops: loops.clone(),
        ids: grid,
    };
    let after = doc
        .apply(&edit, tol(), &editor_core::RefusingReach)
        .unwrap()
        .doc;
    let new = program_of(&after, first);
    assert_eq!(new.step(0, &top), Ok(top_id), "the kept step keeps its id");
    assert_ne!(new.ids[0][6], old.ids[0][6], "an unkept step mints fresh");

    // An address off the new program, and an old id kept twice.
    assert_eq!(
        keep_grid(&loops, &[vec![(a.close.clone(), top_id)]]),
        Err(StepHandleRefusal::OffProgram { loop_: 0, index: 6 }),
        "the old closer's address is a line_to(point) in the new program"
    );
    let twice = keep_grid(&loops, &[vec![(top, top_id), (a.fillet.clone(), top_id)]]).unwrap();
    let refused = doc.apply(
        &DocEdit::SetProgram {
            node: first,
            loops,
            ids: twice,
        },
        tol(),
        &editor_core::RefusingReach,
    );
    assert!(
        matches!(
            refused,
            Err(EditError::StepIdsRefused {
                fault: StepIdFault::Repeated { .. },
                ..
            })
        ),
        "an old id kept twice: {refused:?}"
    );
}

//! **A profile program whose replayed loops fail `Profile::validate`
//! is legal at rest: the load door passes it and evaluation refuses it
//! typed** (`ProgramFault`'s doc, the validate class).
//!
//! The load door rebuilds a profile program by its derive and re-checks
//! it with one walk, which refuses the lattice class alone
//! (`m4_pr6_refusal::program_structure_doors_refuse_typed_at_load`).
//! A validate refusal is not that class, so it reaches evaluation and
//! answers there as `NodeErrorKind::Profile`, carrying the validator's
//! own `ProfileError`.
//!
//! The witness is the sharpest case of the class: a bowtie drawn in
//! LITERALS, which no binding rescues and the insert door refuses. So
//! the only way to write it is a hand-edited file, and this row makes
//! one by surgery on a saved square.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::eval;
use crate::fixture::{desc, on_frame_keeping, square};
use crate::wire::doctored;
use editor_core::{
    DocEdit, EditError, Node, NodeErrorKind, NodeResult, ProfileDoc, ProgramRefusal, apply, load,
    save,
};
use geom_core::Tol;
use profile::{ContactKind, ProfileError, SegmentRef};

/// The unit square's corners in the bowtie's order: its first and
/// third segments cross at the centre.
const BOWTIE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)];

/// The validator's answer for [`BOWTIE`]: segments 0 and 2 of loop 0
/// cross.
fn is_the_bowtie_crossing(e: &ProfileError) -> bool {
    *e == ProfileError::NonSimple {
        first: SegmentRef {
            loop_index: 0,
            segment_index: 0,
        },
        second: SegmentRef {
            loop_index: 0,
            segment_index: 2,
        },
        kind: ContactKind::Crossing,
    }
}

#[test]
fn a_program_failing_validate_loads_clean_and_refuses_typed_at_evaluation() {
    let (doc, plane, profile) = on_frame_keeping(
        ProfileDoc::empty_derived("load_door_program_validate", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.5, 0.5, 0.5)],
    );

    // The witness is the VALIDATE class: the insert door's check
    // replays it cleanly and refuses the assembled loop.
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Profile(desc(plane, vec![BOWTIE.to_vec()]))),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::ProfileProgramRefused { refusal, .. }) => match *refusal {
            ProgramRefusal::Validate(e) => assert!(
                is_the_bowtie_crossing(&e),
                "the insert door names the crossing: {e:?}"
            ),
            other => panic!("the witness must be the validate class, got {other:?}"),
        },
        other => panic!("the insert door must refuse the bowtie, got {other:?}"),
    }

    // The same program in a file: the square's second and third
    // corners trade y, which is the bowtie.
    let text = save(&doc, &[], Tol::witness()).expect("the square saves");
    let bowtie = doctored(&text, |v| {
        let chain =
            &mut v["snapshot"]["nodes"][profile.0.to_string()]["Profile"]["loops"][0]["Chain"];
        for (step, from, to) in [(1, 0.0, 1.0), (2, 1.0, 0.0)] {
            let y = &mut chain[step]["LineTo"]["Point"][1]["Literal"]["value"];
            assert_eq!(
                *y,
                serde_json::json!(from),
                "step {step}'s y is the square's"
            );
            *y = serde_json::json!(to);
        }
    });

    let loaded = match load(&bowtie, Tol::witness()) {
        Ok(loaded) => loaded,
        Err(e) => panic!("a validate refusal is legal at rest; the load door refused {e:?}"),
    };

    let ev = eval::<f64>(&loaded.doc);
    match ev
        .nodes
        .iter()
        .find(|(n, _)| **n == profile)
        .map(|(_, r)| r)
    {
        Some(NodeResult::Failed(err)) => match &err.kind {
            NodeErrorKind::Profile(e) => assert!(
                is_the_bowtie_crossing(e),
                "evaluation names the crossing: {e:?}"
            ),
            other => panic!("expected the validate class at evaluation, got {other:?}"),
        },
        other => panic!("the loaded bowtie must refuse at evaluation, got {other:?}"),
    }
}

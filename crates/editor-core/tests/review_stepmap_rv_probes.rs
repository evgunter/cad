//! **Review probe (`stepmap-rv`): the door's answer is checked against
//! the STEP, not only against the segment index.**
//!
//! `edit_step_segments.rs` checks that a ref named `segment: s` lands on
//! the wall program segment `s` swept — a property of the anchor rewrite
//! (`eval::anchor`), which holds whatever the per-step spans say — and
//! that the spans partition the loop, which `Core::step_spans` makes
//! true by construction. Neither reads the step's own authored
//! argument, so a wrong ATTRIBUTION (step `j` credited with step
//! `j+1`'s segment) passes both.
//!
//! This row reads the authored argument: a plain `line_to(p)` step
//! produces exactly one segment, and that segment ENDS at `p`. Nothing
//! here recomputes the door's arithmetic either.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Evaluation, Node, ProfileDoc, ProfileProgram, ValuePayload, evaluate,
};
use fixture::{insert, len, on_frame};
use geom_core::{Point2, Tol};
use profile::{ProfileStructure, SketchPlane, Step, Target};

fn tol() -> Tol {
    Tol::witness()
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

/// The same records the door's own suite rebuilds, plus the resolved
/// program so a step's authored argument can be read.
fn records(
    doc: &ProfileDoc,
    program: &ProfileProgram,
) -> (ProfileStructure, Vec<Vec<Step<f64>>>, Vec<Vec<Point2<f64>>>) {
    let env = doc.param_env::<f64>();
    let resolved = program.resolve::<f64>(&env).expect("resolves");
    let mut loops = Vec::new();
    let mut replay = Vec::new();
    for steps in &resolved {
        let (lp, record) = profile::replay_recording(steps, tol()).expect("replays");
        loops.push(lp);
        replay.push(record);
    }
    let assembled = profile::Profile::new(SketchPlane::xy(), loops);
    let points = assembled
        .loops
        .iter()
        .map(|lp| lp.vertices().iter().map(|v| v.pos()).collect())
        .collect();
    let (_, canonical) = assembled.validate_recording(tol()).expect("validates");
    (ProfileStructure { replay, canonical }, resolved, points)
}

/// **A `line_to(p)` step is answered with the one segment that ends at
/// `p`.** Red when the attribution is shifted, green when it is not.
#[test]
fn a_line_to_step_is_answered_with_the_segment_that_ends_where_it_says() {
    let doc = ProfileDoc::empty_derived("stepmap-rv-attribution", tol());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let (structure, resolved, points) = records(&doc, program);
    let n = points[0].len();
    let mut checked = 0_usize;
    for (j, step) in resolved[0].iter().enumerate() {
        let Step::LineTo(Target::Point(p)) = step else {
            continue;
        };
        let edges = program
            .canonical_segments_of(&structure, &pv.naming, 0, j as u32)
            .expect("the door answers");
        assert_eq!(
            edges.len(),
            1,
            "step {j} is a plain line_to: it produced exactly one segment, \
             the door gave it {edges:?}"
        );
        let s = edges[0].segment as usize;
        let end = points[0][(s + 1) % n];
        assert_eq!(
            (end.x.to_bits(), end.y.to_bits()),
            (p.x.to_bits(), p.y.to_bits()),
            "step {j} authored line_to({p:?}); the segment it was given \
             ({s}) ends at {end:?} instead"
        );
        checked += 1;
    }
    assert!(checked >= 2, "the walk reached {checked} line_to steps");
}

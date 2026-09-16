//! **The authored-step → profile-edge map** (DM8,
//! `ProfileProgram::canonical_segments_of`).
//!
//! The door answers "which profile edges did this authored step
//! become", composing the replay's per-step segment span with the
//! permutation canonicalization recorded. What these rows check is that
//! the answer LANDS ON THE GEOMETRY: the refs a step is given name the
//! walls the step's own segments bound, measured in 3-space against the
//! extruded body, not asserted by re-doing the door's index arithmetic
//! in the test.
//!
//! The mutant the geometric arm is here to catch is a door that applies
//! canonicalization's permutation to its answer. A profile ref reaches a
//! name table already rewritten canonical → program (`eval::anchor`), so
//! a permuted answer names the wrong wall on exactly the loops where the
//! permutation is not the identity — a loop authored clockwise, and one
//! whose lexicographic-minimum vertex is not the one it was authored
//! from. Both are built below, and each asserts that it IS the case it
//! claims to be before it asserts anything about the door.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, Node, ProfileDoc,
    ProfileEdgeRef, ProfileProgram, RecipeNodeId, RoleSeg, StableName, StepSegmentsError,
    ValuePayload, eval::ProfileNaming, evaluate,
};
use fixture::{insert, len, on_frame};
use geom_core::{Point2, Tol};
use profile::{ProfileStructure, SketchPlane};
use topo::{Body, FaceKey, LoopBoundary};

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

/// One profile node's records, rebuilt through the SAME public doors
/// the evaluation's pre-pass runs: resolve at `f64`, replay each loop
/// recording, validate the assembly recording.
///
/// The plane is the conventional one because no decision below reads
/// it — validation is 2-D and the naming anchor is loop-derived — and
/// the replayed loops are the profile's own 2-D chains either way.
fn records(
    doc: &ProfileDoc,
    program: &ProfileProgram,
) -> (ProfileStructure, Vec<Vec<Point2<f64>>>) {
    let env = doc.param_env::<f64>();
    let resolved = program.resolve::<f64>(&env).expect("the corpus resolves");
    let mut loops = Vec::new();
    let mut replay = Vec::new();
    for steps in &resolved {
        let (lp, record) = profile::replay_recording(steps, tol()).expect("the corpus replays");
        loops.push(lp);
        replay.push(record);
    }
    let assembled = profile::Profile::new(SketchPlane::xy(), loops);
    let program_points = assembled
        .loops
        .iter()
        .map(|lp| lp.vertices().iter().map(|v| v.pos()).collect())
        .collect();
    let (_, canonical) = assembled
        .validate_recording(tol())
        .expect("the corpus validates");
    (ProfileStructure { replay, canonical }, program_points)
}

/// The door's answer for every step of one loop, concatenated in
/// program order, having first checked that the spans PARTITION the
/// loop: every segment claimed by exactly one step, and no segment left
/// unclaimed.
fn edges_by_step(
    program: &ProfileProgram,
    structure: &ProfileStructure,
    naming: &ProfileNaming,
    loop_: u32,
    steps: usize,
    segments: usize,
    what: &str,
) -> Vec<Vec<ProfileEdgeRef>> {
    let mut per_step = Vec::with_capacity(steps);
    let mut seen: Vec<u32> = Vec::new();
    for step in 0..steps {
        let edges = program
            .canonical_segments_of(structure, naming, loop_, step as u32)
            .unwrap_or_else(|e| panic!("{what} loop {loop_} step {step}: {e}"));
        for e in &edges {
            assert_eq!(e.loop_index, loop_, "{what}: the ref names its own loop");
            seen.push(e.segment);
        }
        per_step.push(edges);
    }
    let want: Vec<u32> = (0..segments as u32).collect();
    assert_eq!(
        seen, want,
        "{what} loop {loop_}: the steps' segments, in program order, are the loop's own — \
         a segment claimed twice, or by nobody, is a map that cannot be read"
    );
    per_step
}

/// A step out of range, and a loop out of range, refuse rather than
/// answering.
fn assert_refuses_off_the_program(
    program: &ProfileProgram,
    structure: &ProfileStructure,
    naming: &ProfileNaming,
    loops: usize,
    steps: usize,
) {
    assert_eq!(
        program.canonical_segments_of(structure, naming, 0, steps as u32),
        Err(StepSegmentsError::NoSuchStep { steps })
    );
    assert_eq!(
        program.canonical_segments_of(structure, naming, loops as u32, 0),
        Err(StepSegmentsError::NoSuchLoop { loops })
    );
}

/// Where one vertex sits.
fn point_of(body: &Body<f64>, v: topo::VertexKey) -> geom_core::Point3<f64> {
    let key = body.get_vertex(v).expect("vertex").point;
    *body.get_point(key).expect("the vertex's point")
}

/// Every vertex the face touches, as 3-D points.
fn face_points(body: &Body<f64>, face: FaceKey) -> Vec<geom_core::Point3<f64>> {
    let data = body.get_face(face).expect("the named face is in the body");
    let mut out = Vec::new();
    for l in core::iter::once(data.outer).chain(data.rings.iter().copied()) {
        match body.get_loop(l).expect("loop").boundary {
            LoopBoundary::Empty { vertex } => out.push(point_of(body, vertex)),
            LoopBoundary::Cycle { first } => {
                for he in body.loop_cycle(first).expect("cycle") {
                    out.push(point_of(body, body.get_half_edge(he).expect("he").start));
                }
            }
        }
    }
    out
}

/// The face a lateral name addresses, `None` where the table has no
/// such name.
fn lateral(ev: &Evaluation<f64>, node: RecipeNodeId, e: ProfileEdgeRef) -> Option<FaceKey> {
    let name = StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Lateral(e)],
    };
    match ev.value(node)?.name_table.lookup(&name)? {
        Entry::Unique(r) => match r.key {
            EntityKey::Face(f) => Some(f),
            _ => None,
        },
        Entry::Tied(_) => None,
    }
}

/// Whether some vertex of the face sits within `eps` of `p`.
fn touches(pts: &[geom_core::Point3<f64>], p: geom_core::Point3<f64>, eps: f64) -> bool {
    pts.iter().any(|q| (*q - p).norm_squared() <= eps * eps)
}

// ------------------------------------------------------------------
// 1. The corpus: every step of every profile program
// ------------------------------------------------------------------

/// **Every authored step of every corpus profile is answered, and the
/// answers partition its loop.**
///
/// The partition is the property a consumer relies on and the one an
/// off-by-one in the span arithmetic breaks: a step credited with its
/// neighbour's segment leaves a gap or a duplicate, and either shows
/// here. The count assertion at the end holds the walk itself honest —
/// a corpus reached through a filter that stopped matching would
/// otherwise pass by checking nothing.
#[test]
fn every_corpus_step_is_answered_and_the_answers_partition_the_loop() {
    let mut answered = 0_usize;
    let mut loops_seen = 0_usize;
    for d in corpus::documents() {
        let ev = run(&d.doc);
        for &id in &ev.order {
            let Some(Node::Profile(program)) = d.doc.node(id) else {
                continue;
            };
            let Some(value) = ev.value(id) else { continue };
            let ValuePayload::Profile(pv) = &value.payload else {
                continue;
            };
            let (structure, points) = records(&d.doc, program);
            for (li, loop_points) in points.iter().enumerate() {
                let segments = loop_points.len();
                let steps = structure.replay[li].steps.len();
                let per_step = edges_by_step(
                    program, &structure, &pv.naming, li as u32, steps, segments, d.name,
                );
                answered += per_step.len();
                loops_seen += 1;
            }
            assert_refuses_off_the_program(
                program,
                &structure,
                &pv.naming,
                program.loops.len(),
                structure.replay[0].steps.len(),
            );
        }
    }
    assert!(
        loops_seen >= 20 && answered >= 40,
        "the corpus walk reached {loops_seen} loops and {answered} steps — \
         too few to be the corpus, so the filter above stopped matching"
    );
}

// ------------------------------------------------------------------
// 2. The geometry: the refs name the walls the step's segments bound
// ------------------------------------------------------------------

/// A four-corner loop on the xy plane, extruded 1 unit. `points` is
/// authored exactly as given, so a caller can hand it clockwise.
fn prism(id: &str, points: Vec<(f64, f64)>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(id, tol());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![points],
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, profile, ext)
}

/// The shared body of the three rows below: every step's refs name
/// walls whose boundary carries that step's own segment endpoints,
/// placed into 3-space.
///
/// Measured against the solid: the wall the ref addresses is looked up
/// by name and its vertices are compared with the segment's endpoints
/// through the profile's placement. Nothing here recomputes the door's
/// arithmetic, so a door that permuted its answer would have to be
/// wrong about the geometry too in order to pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Perm {
    /// Canonicalization neither reversed nor rotated the chain.
    Identity,
    /// It reversed the chain to reach the role's winding.
    Reversed,
    /// It rotated the chain to the lexicographic-minimum start.
    Rotated,
}

fn assert_steps_bound_their_walls(id: &str, points: Vec<(f64, f64)>, want: Perm) {
    let (doc, profile, ext) = prism(id, points);
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("{id}: the profile node is a program");
    };
    let value = ev.value(profile).expect("the profile evaluates");
    let ValuePayload::Profile(pv) = &value.payload else {
        panic!("{id}: the profile node carries a profile");
    };
    let (structure, points3) = records(&doc, program);

    // The fixture is the case it claims to be. Without this the row
    // still passes on a loop canonicalization left alone, and then
    // proves nothing about a permuted answer.
    let canonical = &structure.canonical.loops[0];
    let found = match (canonical.reversed, canonical.start) {
        (true, _) => Perm::Reversed,
        (false, 0) => Perm::Identity,
        (false, _) => Perm::Rotated,
    };
    assert_eq!(
        found, want,
        "{id}: the fixture is the {found:?} case, not the {want:?} one it is \
         written to be — a row whose permutation is the identity cannot see a \
         permuted answer"
    );

    let ValuePayload::Body(body) = &ev.value(ext).expect("the extrude evaluates").payload else {
        panic!("{id}: the extrude carries a body");
    };
    let placement = pv.validated.plane().placement;
    let place = |p: Point2<f64>| placement.transform_point(geom_core::Point3::new(p.x, p.y, 0.0));

    let n = points3[0].len();
    let steps = structure.replay[0].steps.len();
    let per_step = edges_by_step(program, &structure, &pv.naming, 0, steps, n, id);
    let mut walls = BTreeSet::new();
    for (step, edges) in per_step.iter().enumerate() {
        for e in edges {
            let face = lateral(&ev, ext, *e)
                .unwrap_or_else(|| panic!("{id}: step {step}'s ref {e:?} names no wall"));
            walls.insert(face);
            let pts = face_points(body, face);
            let s = e.segment as usize;
            for end in [points3[0][s], points3[0][(s + 1) % n]] {
                assert!(
                    touches(&pts, place(end), 1e-9),
                    "{id}: step {step}'s wall for {e:?} does not touch \
                     the endpoint {end:?} of the segment that step produced — \
                     the map names a wall the step's geometry does not bound"
                );
            }
        }
    }
    assert_eq!(
        walls.len(),
        n,
        "{id}: the steps between them name every wall exactly once"
    );
}

/// The identity case: authored counterclockwise from its own
/// lexicographic-minimum corner, so canonicalization neither reverses
/// nor rotates and every reading agrees.
#[test]
fn a_ccw_loop_names_the_walls_its_steps_bound() {
    assert_steps_bound_their_walls(
        "step-segments-ccw",
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        Perm::Identity,
    );
}

/// **Reversed.** The same rectangle authored clockwise: canonicalization
/// orients an outer loop counterclockwise, so it reverses the chain. A
/// door that handed back canonical indices would name the walls in the
/// reversed order here and the endpoints would not match.
#[test]
fn a_reversed_loop_names_the_walls_its_steps_bound() {
    assert_steps_bound_their_walls(
        "step-segments-cw",
        vec![(0.0, 0.0), (0.0, 1.0), (2.0, 1.0), (2.0, 0.0)],
        Perm::Reversed,
    );
}

/// **Rotated.** Authored counterclockwise but starting from a corner
/// that is not the lexicographic minimum, so canonicalization rotates
/// the chain without reversing it. A door that handed back canonical
/// indices would name the walls shifted by that rotation.
#[test]
fn a_rotated_loop_names_the_walls_its_steps_bound() {
    assert_steps_bound_their_walls(
        "step-segments-rot",
        vec![(2.0, 1.0), (0.0, 1.0), (0.0, 0.0), (2.0, 0.0)],
        Perm::Rotated,
    );
}

// ------------------------------------------------------------------
// 3. The refusals
// ------------------------------------------------------------------

/// **A naming anchor from another profile is refused, not read.**
///
/// The permutation is recorded twice — once by canonicalization, once
/// by the bit-match that anchors the names — and the door composes both.
/// Handed two that describe different loops it says so, rather than
/// answering from whichever it read first.
#[test]
fn two_records_describing_different_loops_refuse() {
    let (doc, profile, _) = prism(
        "step-segments-refusal",
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let (structure, _) = records(&doc, program);
    let value = ev.value(profile).expect("evaluates");
    let ValuePayload::Profile(pv) = &value.payload else {
        panic!("carries a profile");
    };
    // An anchor for the same loop, reversed the other way: one of the
    // two records is not about this loop and the door cannot tell which.
    let mut naming = pv.naming.clone();
    naming.loops[0].reversed = !naming.loops[0].reversed;
    assert_eq!(
        program.canonical_segments_of(&structure, &naming, 0, 0),
        Err(StepSegmentsError::RecordsDisagree { loop_: 0 })
    );
    // An anchor that does not mention the loop at all.
    let empty = ProfileNaming::default();
    assert_eq!(
        program.canonical_segments_of(&structure, &empty, 0, 0),
        Err(StepSegmentsError::NoAnchor { loop_: 0 })
    );
}

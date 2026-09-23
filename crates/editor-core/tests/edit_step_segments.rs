//! **The authored-step → profile-edge map** (DM8,
//! `ProfileProgram::profile_edges_of`).
//!
//! The door answers "which profile edges did this authored step
//! become", composing the replay's per-step segment span with the
//! permutation canonicalization recorded. Two independent things can be
//! wrong with that answer, and the sections below are one apiece.
//!
//! **1. The ANCHORING** (§2). A ref named `segment: s` must land on the
//! wall program segment `s` swept. The mutant is a door that applies
//! canonicalization's permutation to its answer: a profile ref reaches
//! a name table already rewritten canonical → program
//! (`eval::anchor`), so a permuted answer names the wrong wall on
//! exactly the loops where the permutation is not the identity. Three
//! prisms build the non-identity cases — reversed, rotated, and
//! reversed-AND-rotated at a start the reversal arithmetic cannot
//! confuse with its own mirror — and each asserts that it IS the case it
//! claims to be before it asserts anything about the door. Measured in
//! 3-space against the extruded body, never by re-doing the door's
//! index arithmetic here.
//!
//! **2. The ATTRIBUTION** (§3). Which STEP a segment is credited to.
//! §2 cannot see this at all: it reads `e.segment` and the wall that
//! segment swept, which is a property of the anchor rewrite and holds
//! whatever the per-step spans say. Neither can the partition check,
//! which holds by construction (`profile`'s `assert_spans_partition`
//! says so in its own words). So §3 reads each step's OWN AUTHORED
//! ARGUMENTS — the point a `line_to` names, the length a `line` names,
//! the centre and radius a carrier form names — and asks whether the
//! segments the door credited it with are the ones that geometry
//! describes. A mutant that shifts every step's attribution one step
//! along passes §1 and §2 and reds here.
//!
//! **3. The PAIRING** (§5 and §5b), for the door's one built
//! consumer: `ProfileProgram::segment_radii` reads the replay's
//! per-radius emission record and answers which radius each EDGE is
//! drawn at — every radius argument of every step, at the segment the
//! record says its arc became. That is a third thing that can be
//! wrong independently — the refs can be right and the expression
//! beside one of them belong to another step, or to another of the
//! same step's three roles — so it has its own rows. §5b adds the
//! cases §5's fixtures cannot reach: a fillet on an anchor hop that
//! is not the identity, a record of somebody else's program at each
//! of the doors that check one, and the two mode vocabularies that
//! decide whether a spec carries a radius at all.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, CapEnd, EntityKey, Entry, EvalOptions, Evaluation, Expr, LoopProgram, Node,
    ProfileDoc, ProfileEdgeRef, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, RoleSeg, StepSegmentsError, ValuePayload,
    eval::{Anchoring, ProfileNaming},
    evaluate,
};
use fixture::{insert, len, on_frame, tol};
use geom_core::Point2;
use profile::{CanonicalStructure, ProfileStructure, SketchPlane, Step, Target};
use topo::{Body, EdgeKey, FaceKey};

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
///
/// **Why this is a REBUILD and not the evaluation's own record.**
/// Measured: nothing public carries it. The pre-pass's
/// `profile::ProfileStructure` lives on `eval::anchor::ProfilePre`,
/// which is `pub(crate)`; `ProfileValue` — the payload a `Profile`
/// node's value carries — holds the validated profile, the naming
/// anchor and the per-edge radii derived from the record, but not the
/// record, so from outside the crate there is no path to the one the
/// geometry was actually made from. That is exactly the gap
/// `work/wire/section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made.md`
/// records, and the same one that keeps the door from having a caller
/// outside these rows and the in-crate one that reads those radii. So
/// these rows pair a rebuilt structure with the
/// evaluation's REAL `naming`, and the door's own two-record check is
/// what holds that pairing honest: a rebuild that had drifted from the
/// evaluation would disagree with the published anchor's permutation
/// and every row here would refuse rather than pass.
fn records(doc: &ProfileDoc, program: &ProfileProgram) -> Records {
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
    let verts = assembled
        .loops
        .iter()
        .map(|lp| lp.vertices().iter().map(|v| (v.pos(), v.bulge())).collect())
        .collect();
    let (_, canonical) = assembled
        .validate_recording(tol())
        .expect("the corpus validates");
    Records {
        structure: ProfileStructure { replay, canonical },
        steps: resolved,
        verts,
    }
}

/// One profile node's records, in the PROGRAM anchoring: the structure
/// the door reads, the authored steps §3 reads back, and the replayed
/// chains both are about.
struct Records {
    structure: ProfileStructure,
    steps: Vec<Vec<Step<f64>>>,
    /// Per loop, per vertex: where it sits and the bulge of the segment
    /// LEAVING it. Segment `k` leaves vertex `k`.
    verts: Vec<Vec<(Point2<f64>, f64)>>,
}

/// The door's answer for every step of one loop, in program order.
///
/// **Answers only.** The partition is [`assert_partition`]'s claim, and
/// the two are separate so that a row can say which of them it is
/// making: four rows share this walk, and a helper that asserted as
/// well as answered would have made all four the same check.
fn edges_by_step<'a>(
    program: &ProfileProgram,
    structure: &ProfileStructure,
    anchoring: impl Into<Anchoring<'a>> + Copy,
    loop_: u32,
    steps: usize,
    what: &str,
) -> Vec<Vec<ProfileEdgeRef>> {
    (0..steps)
        .map(|step| {
            program
                .profile_edges_of(structure, anchoring, loop_, step as u32)
                .unwrap_or_else(|e| panic!("{what} loop {loop_} step {step}: {e}"))
        })
        .collect()
}

/// **The answers partition the loop**: every segment claimed by exactly
/// one step, in program order, and no segment left unclaimed.
///
/// What this can catch is narrower than it reads, and the profile
/// side's `assert_spans_partition` documents why: contiguity and cover
/// are what `Core::step_spans` makes true by construction. Here it is
/// also a statement about the DOOR — that the refs it mints are its
/// loop's own and that no step's span was dropped or duplicated on the
/// way through the permutation check — which the profile-side row
/// cannot see.
fn assert_partition(per_step: &[Vec<ProfileEdgeRef>], loop_: u32, segments: usize, what: &str) {
    let mut seen: Vec<u32> = Vec::new();
    for edges in per_step {
        for e in edges {
            assert_eq!(e.loop_index, loop_, "{what}: the ref names its own loop");
            seen.push(e.segment);
        }
    }
    let want: Vec<u32> = (0..segments as u32).collect();
    assert_eq!(
        seen, want,
        "{what} loop {loop_}: the steps' segments, in program order, are the loop's own — \
         a segment claimed twice, or by nobody, is a map that cannot be read"
    );
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
        program.profile_edges_of(structure, naming, 0, steps as u32),
        Err(StepSegmentsError::NoSuchStep { steps })
    );
    assert_eq!(
        program.profile_edges_of(structure, naming, loops as u32, 0),
        Err(StepSegmentsError::NoSuchLoop { loops })
    );
}

/// The point of every vertex the face touches — a LIST, not the set
/// `fixture::face_vertices` answers with: `Point3<f64>` is not
/// hashable, and the one caller matches a point within a tolerance
/// rather than by equality.
fn face_vertex_points(body: &Body<f64>, face: FaceKey) -> Vec<geom_core::Point3<f64>> {
    fixture::face_vertices(body, face)
        .into_iter()
        .map(|v| fixture::point(body, v))
        .collect()
}

/// The face a lateral name addresses, `None` where the table has no
/// such name.
fn lateral(ev: &Evaluation<f64>, node: RecipeNodeId, e: ProfileEdgeRef) -> Option<FaceKey> {
    let name = fixture::fname(node, RoleSeg::Lateral(e));
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
/// The partition is what a consumer reads the map through: a segment
/// claimed twice, or by nobody, is a map that cannot be used at all.
/// It is NOT a check on attribution — §3 is — and it is not a check on
/// the span arithmetic either, which makes it true by construction; it
/// is a check that the door minted a ref for every segment of every
/// loop of every document the corpus has, without refusing.
///
/// The walk holds itself honest against a count that MOVES WITH THE
/// CORPUS rather than a literal: every profile node the corpus
/// evaluates is counted as the walk passes it, and the row asserts that
/// every one of them was answered. A filter that stopped matching drops
/// the answered count without dropping the node count, and a corpus
/// that grew or shrank moves both together.
#[test]
fn every_corpus_step_is_answered_and_the_answers_partition_the_loop() {
    let mut answered = 0_usize;
    let mut loops_seen = 0_usize;
    let mut profile_nodes = 0_usize;
    let mut answered_nodes = 0_usize;
    let documents = corpus::documents();
    for d in &documents {
        let ev = run(&d.doc);
        for &id in &ev.order {
            let Some(Node::Profile(program)) = d.doc.node(id) else {
                continue;
            };
            profile_nodes += 1;
            let Some(value) = ev.value(id) else { continue };
            let ValuePayload::Profile(pv) = &value.payload else {
                continue;
            };
            answered_nodes += 1;
            let r = records(&d.doc, program);
            for (li, loop_verts) in r.verts.iter().enumerate() {
                let steps = r.structure.replay[li].steps.len();
                let per_step =
                    edges_by_step(program, &r.structure, &pv.naming, li as u32, steps, d.name);
                assert_partition(&per_step, li as u32, loop_verts.len(), d.name);
                answered += per_step.len();
                loops_seen += 1;
            }
            assert_refuses_off_the_program(
                program,
                &r.structure,
                &pv.naming,
                program.loops.len(),
                r.structure.replay[0].steps.len(),
            );
        }
    }
    assert_eq!(
        answered_nodes, profile_nodes,
        "the corpus has {profile_nodes} profile nodes and the walk answered \
         {answered_nodes} — a profile node that evaluates to something else, or \
         not at all, is not a node this row may skip in silence"
    );
    // Every profile node has at least one loop and every loop at least
    // an entry verb and a close, so these move with the corpus too.
    assert!(
        profile_nodes >= documents.len()
            && loops_seen >= profile_nodes
            && answered >= 2 * loops_seen,
        "the corpus walk reached {profile_nodes} profile nodes, {loops_seen} loops \
         and {answered} steps over {} documents — too few to be the corpus, so the \
         filter above stopped matching",
        documents.len()
    );
}

// ------------------------------------------------------------------
// 2. The geometry: the refs name the walls the step's segments bound
// ------------------------------------------------------------------

/// A polygonal loop on the xy plane, extruded 1 unit. `points` is
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

/// Which permutation canonicalization applied to a fixture's loop.
///
/// A row names the one it is written to exercise and the body checks
/// the fixture IS that case before it asserts anything about the door:
/// a row whose permutation turned out to be the identity cannot see a
/// permuted answer, and would pass while proving nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Perm {
    /// Neither reversed nor rotated.
    Identity,
    /// Reversed to reach the role's winding, from canonical start 0.
    Reversed,
    /// Rotated to the lexicographic-minimum start, not reversed.
    Rotated,
    /// **Both**, at a start the reversed arithmetic cannot confuse with
    /// its own mirror: `(n - start) % n != start`, which needs
    /// `2 * start != n`. The two-record check maps `start` on the
    /// ORIENTED chain to the anchor's `offset` on the program chain
    /// through exactly that expression, and every other fixture here
    /// satisfies it accidentally — an identity or a rotation has
    /// `start = 0` on the reversed branch, and a 4-gon reversed at
    /// `start = 2` is its own mirror.
    ReversedAndRotated,
}

/// The shared body of the anchoring rows: every step's refs name walls
/// whose boundary carries that step's own segment endpoints, placed
/// into 3-space.
///
/// Measured against the solid: the wall the ref addresses is looked up
/// by name and its vertices are compared with the segment's endpoints
/// through the profile's placement. Nothing here recomputes the door's
/// arithmetic, so a door that permuted its answer would have to be
/// wrong about the geometry too in order to pass.
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
    let r = records(&doc, program);
    let n = r.verts[0].len();

    let canonical = &r.structure.canonical.loops[0];
    let found = match (canonical.reversed, canonical.start) {
        (true, 0) => Perm::Reversed,
        (true, s) if 2 * s == n => Perm::Reversed,
        (true, _) => Perm::ReversedAndRotated,
        (false, 0) => Perm::Identity,
        (false, _) => Perm::Rotated,
    };
    assert_eq!(
        found, want,
        "{id}: the fixture is the {found:?} case (reversed = {}, start = {} of \
         {n}), not the {want:?} one it is written to be",
        canonical.reversed, canonical.start
    );

    let ValuePayload::Body(body) = &ev.value(ext).expect("the extrude evaluates").payload else {
        panic!("{id}: the extrude carries a body");
    };
    let placement = pv.validated.plane().placement;
    let place = |p: Point2<f64>| placement.transform_point(geom_core::Point3::new(p.x, p.y, 0.0));

    let steps = r.structure.replay[0].steps.len();
    let per_step = edges_by_step(program, &r.structure, &pv.naming, 0, steps, id);
    assert_partition(&per_step, 0, n, id);
    let mut walls = BTreeSet::new();
    for (step, edges) in per_step.iter().enumerate() {
        for e in edges {
            let face = lateral(&ev, ext, *e)
                .unwrap_or_else(|| panic!("{id}: step {step}'s ref {e:?} names no wall"));
            walls.insert(face);
            let pts = face_vertex_points(body, face);
            let s = e.segment as usize;
            for end in [r.verts[0][s].0, r.verts[0][(s + 1) % n].0] {
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

/// **Reversed AND rotated, at a start that is not its own mirror.** A
/// convex pentagon authored clockwise from a corner that is not the
/// lexicographic minimum: canonicalization reverses it to reach the
/// outer role's winding and then rotates by a `start` with
/// `2 * start != n`.
///
/// This is the only fixture that can see the reversed branch of the
/// two-record check. That branch maps canonicalization's `start`, which
/// counts on the ORIENTED chain, to the naming anchor's `offset`, which
/// counts on the program chain, through `(n - start) % n`. Replace that
/// expression with `start` and every other row here still passes: an
/// identity and a rotation never reach the branch, and a 4-gon reversed
/// at `start = 2` has `(4 - 2) % 4 = 2`. Here the two differ (1 against
/// 4), so the mutant makes the door read its two records as different
/// permutations, the two-record assertion fires and the row reds.
#[test]
fn a_reversed_and_rotated_loop_names_the_walls_its_steps_bound() {
    assert_steps_bound_their_walls(
        "step-segments-cw-rot",
        vec![(0.0, 0.0), (-1.0, 1.0), (1.0, 2.0), (3.0, 1.0), (2.0, 0.0)],
        Perm::ReversedAndRotated,
    );
}

// ------------------------------------------------------------------
// 2b. A loft: every section's steps name the walls they bound
// ------------------------------------------------------------------
//
// A loft publishes ONE table for all of its sections — one ref per
// wall, through section 0's anchor — and the door reaches it from any
// section through `SectionAnchors::section`, read off the loft's own
// value. The rows author later sections rotated and reversed relative
// to section 0, so a door that answered in a section's own program
// indices would name another section's walls.

/// The same 2 × 1 rectangle at every section; section `i` sits at
/// `z = i`, authored as `sections[i].0`, which canonicalization treats
/// as the `sections[i].1` case.
///
/// Every section being one rectangle makes the loft a vertical prism,
/// so a wall carries each section's segment at that segment's plan
/// position — the reading a MIDDLE section, whose segment endpoints
/// are no vertex of the body, is checked through.
fn loft_of(
    id: &str,
    sections: &[(Vec<(f64, f64)>, Perm)],
) -> (ProfileDoc, Vec<RecipeNodeId>, RecipeNodeId) {
    let mut doc = ProfileDoc::empty_derived(id, tol());
    let mut ids = Vec::new();
    for (i, (points, _)) in sections.iter().enumerate() {
        let (d, s) = on_frame(
            doc,
            [0.0, 0.0, i as f64],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![points.clone()],
        );
        doc = d;
        ids.push(s);
    }
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: ids.clone(),
            v_degree: Expr::count(1),
        },
    );
    (doc, ids, loft)
}

/// Which permutation canonicalization applied to loop 0 of `r`.
fn perm_of(r: &Records) -> Perm {
    let n = r.verts[0].len();
    let canonical = &r.structure.canonical.loops[0];
    match (canonical.reversed, canonical.start) {
        (true, 0) => Perm::Reversed,
        (true, s) if 2 * s == n => Perm::Reversed,
        (true, _) => Perm::ReversedAndRotated,
        (false, 0) => Perm::Identity,
        (false, _) => Perm::Rotated,
    }
}

/// The edge a rim name addresses, `None` where the table has no such
/// name.
fn rim(
    ev: &Evaluation<f64>,
    node: RecipeNodeId,
    end: CapEnd,
    e: ProfileEdgeRef,
) -> Option<EdgeKey> {
    let name = fixture::ename(node, RoleSeg::RimEdge(end, e));
    match ev.value(node)?.name_table.lookup(&name)? {
        Entry::Unique(r) => match r.key {
            EntityKey::Edge(k) => Some(k),
            _ => None,
        },
        Entry::Tied(_) => None,
    }
}

/// The shared body of the loft rows. For EACH section, the door is
/// asked with that section's own program and records and the anchoring
/// the LOFT's value hands out for it, and every ref it answers must:
///
/// - name a loft wall that carries the step's segment — its endpoints,
///   placed, for the first and last sections, whose segments are the
///   walls' own corners; their plan positions for a middle one;
/// - on the first and last sections, name the `Start` / `End` rim
///   whose two endpoints ARE that segment's, placed.
///
/// Between them the steps must name every wall exactly once.
fn assert_loft_sections_bound_their_walls(id: &str, sections: &[(Vec<(f64, f64)>, Perm)]) {
    let (doc, ids, loft) = loft_of(id, sections);
    let ev = run(&doc);
    let value = ev
        .value(loft)
        .unwrap_or_else(|| panic!("{id}: the loft evaluates: {:?}", ev.node_error(loft)));
    let ValuePayload::Body(body) = &value.payload else {
        panic!("{id}: the loft carries a body");
    };
    let anchors = value
        .section_anchors
        .as_ref()
        .unwrap_or_else(|| panic!("{id}: a loft's value carries its section anchors"));
    let last = ids.len() - 1;
    for (si, (&section, (_, want))) in ids.iter().zip(sections).enumerate() {
        let Some(Node::Profile(program)) = doc.node(section) else {
            panic!("{id}: section {si} is a program");
        };
        let ValuePayload::Profile(pv) = &ev.value(section).expect("the section evaluates").payload
        else {
            panic!("{id}: section {si} carries a profile");
        };
        let r = records(&doc, program);
        let n = r.verts[0].len();
        let found = perm_of(&r);
        assert_eq!(
            found, *want,
            "{id}: section {si} is the {found:?} case, not the {want:?} one it is written to be"
        );
        let anchoring = anchors
            .section(si)
            .unwrap_or_else(|| panic!("{id}: the loft anchors section {si}"));
        assert_eq!(
            anchoring.own(),
            &pv.naming,
            "{id}: the loft's record of section {si}'s anchor is the section's own"
        );
        let placement = pv.validated.plane().placement;
        let place =
            |p: Point2<f64>| placement.transform_point(geom_core::Point3::new(p.x, p.y, 0.0));
        let steps = r.structure.replay[0].steps.len();
        let per_step = edges_by_step(program, &r.structure, anchoring, 0, steps, id);
        let mut walls = BTreeSet::new();
        for (step, edges) in per_step.iter().enumerate() {
            // The door answers in the published numbering, where a ref
            // indexes the loop by SECTION 0's program order; the
            // segment it bounds is read by the step's own span.
            let span = r.structure.replay[0].steps[step];
            assert_eq!(
                edges.len(),
                span.iter().count(),
                "{id}: section {si} step {step} answers one ref per segment it produced"
            );
            for (e, s) in edges.iter().zip(span.iter()) {
                let face = lateral(&ev, loft, *e).unwrap_or_else(|| {
                    panic!("{id}: section {si} step {step}'s ref {e:?} names no loft wall")
                });
                walls.insert(face);
                let pts = face_vertex_points(body, face);
                let ends = [r.verts[0][s].0, r.verts[0][(s + 1) % n].0];
                for end in ends {
                    let bound = if si == 0 || si == last {
                        touches(&pts, place(end), 1e-9)
                    } else {
                        let p = place(end);
                        pts.iter()
                            .any(|q| (q.x - p.x).abs() <= 1e-9 && (q.y - p.y).abs() <= 1e-9)
                    };
                    assert!(
                        bound,
                        "{id}: section {si} step {step}'s loft wall for {e:?} does not \
                         carry the endpoint {end:?} of the segment that step produced \
                         (wall vertices {pts:?})"
                    );
                }
                let cap = match si {
                    0 => Some(CapEnd::Start),
                    i if i == last => Some(CapEnd::End),
                    _ => None,
                };
                if let Some(cap) = cap {
                    let edge = rim(&ev, loft, cap, *e).unwrap_or_else(|| {
                        panic!("{id}: section {si} step {step}'s ref {e:?} names no {cap:?} rim")
                    });
                    let [a, b] = fixture::ends(body, edge);
                    let got = [fixture::point(body, a), fixture::point(body, b)];
                    for end in ends {
                        assert!(
                            touches(&got, place(end), 1e-9),
                            "{id}: section {si} step {step}'s {cap:?} rim for {e:?} does \
                             not end at {end:?}, an endpoint of the segment that step \
                             produced (rim ends {got:?})"
                        );
                    }
                }
            }
        }
        assert_eq!(
            walls.len(),
            n,
            "{id}: section {si}'s steps between them name every wall exactly once"
        );
    }
}

/// Section 0: counterclockwise from its lexicographic-minimum corner.
fn loft_identity() -> (Vec<(f64, f64)>, Perm) {
    (
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        Perm::Identity,
    )
}

/// The same rectangle authored clockwise.
fn loft_reversed() -> (Vec<(f64, f64)>, Perm) {
    (
        vec![(0.0, 0.0), (0.0, 1.0), (2.0, 1.0), (2.0, 0.0)],
        Perm::Reversed,
    )
}

/// The same rectangle authored counterclockwise from another corner.
fn loft_rotated() -> (Vec<(f64, f64)>, Perm) {
    (
        vec![(2.0, 1.0), (0.0, 1.0), (0.0, 0.0), (2.0, 0.0)],
        Perm::Rotated,
    )
}

/// **The second section authored REVERSED relative to the first**: its
/// steps name the walls, and the `End` rims, they bound.
#[test]
fn a_loft_section_authored_reversed_names_the_walls_its_steps_bound() {
    assert_loft_sections_bound_their_walls(
        "loft-anchor-reversed",
        &[loft_identity(), loft_reversed()],
    );
}

/// **The second section authored ROTATED relative to the first.**
#[test]
fn a_loft_section_authored_rotated_names_the_walls_its_steps_bound() {
    assert_loft_sections_bound_their_walls(
        "loft-anchor-rotated",
        &[loft_identity(), loft_rotated()],
    );
}

/// **Three sections: a reversed middle and a rotated last.** The middle
/// section carries no rim and no body vertex, so it is the one whose
/// answer only a wall can check; the last is the `End` cap's.
#[test]
fn a_three_section_loft_names_the_walls_every_section_bounds() {
    assert_loft_sections_bound_their_walls(
        "loft-anchor-three",
        &[loft_identity(), loft_reversed(), loft_rotated()],
    );
}

// ------------------------------------------------------------------
// 3. The attribution: read back against the step's own arguments
// ------------------------------------------------------------------

/// What ONE step's own authored arguments claim about the segments it
/// produced — the reading §2 cannot make, because §2 never looks at the
/// step at all.
#[derive(Debug, Clone, Copy)]
enum Claim {
    /// An entry verb: it seeds the chain at this authored point and
    /// produces no segment of its own.
    Seeds(Point2<f64>),
    /// A tip-state verb — a direction, a declared tangency, opening a
    /// fillet — which produces no segment.
    Binds,
    /// The step's LAST segment arrives at this authored point.
    EndsAt(Point2<f64>),
    /// The step targets `Start`: its last segment arrives at vertex 0.
    Closes,
    /// A straight leg of this authored length: one segment, no bulge.
    Straight(f64),
    /// A one-step complete-loop carrier form: every segment of the
    /// loop, every vertex on the circle it authored.
    Carrier(Point2<f64>, f64),
    /// The step's arguments say nothing this row knows how to read
    /// back. The fused fillet verbs are here: what they emit depends on
    /// the arrival spec's own completion story, so an endpoint read off
    /// the spec is not necessarily the end of THIS step's span. They
    /// are read through their neighbours instead, which is enough to
    /// catch a shifted attribution.
    Unread,
}

/// The claim, read off the step and its position in the program.
fn claim(j: usize, step: &Step<f64>) -> Claim {
    let of_target = |t: &Target<f64>| match t {
        Target::Point(p) => Claim::EndsAt(*p),
        Target::Start | Target::StartArriving => Claim::Closes,
    };
    let of_spec = |s: &profile::ArcData<f64>| match s.target() {
        Some(t) => of_target(t),
        None => Claim::Unread,
    };
    match step {
        // Only the ENTRY `at` seeds; on a fillet arrival the same verb
        // resolves the corner and emits, and what it ends at is the
        // trimmed tangency rather than the anchor it authored.
        Step::At(p) if j == 0 => Claim::Seeds(*p),
        Step::At(_) => Claim::Unread,
        Step::Angle(_)
        | Step::Toward { .. }
        | Step::Tangent
        | Step::Cusp
        | Step::Turn(_)
        | Step::Fillet { .. } => Claim::Binds,
        Step::Line(len) => Claim::Straight(*len),
        Step::LineTo(t) | Step::ContinueTo(t) | Step::TangentArcTo(t) => of_target(t),
        Step::ArcTo(spec) => of_spec(spec),
        Step::FarEndTo(p) => Claim::EndsAt(*p),
        Step::CloseTo => Claim::Closes,
        Step::Circle { centre, radius } => Claim::Carrier(*centre, *radius),
        Step::CircleSplit { centre, radius, .. } => Claim::Carrier(*centre, *radius),
        Step::FilletArc { .. } | Step::ArcFillet { .. } | Step::ArcFilletArc { .. } => {
            Claim::Unread
        }
    }
}

/// How many steps of each shape one walk reached, so a row can refuse
/// to pass on a corpus that stopped containing the cases it is about.
#[derive(Default, Debug)]
struct Tally {
    seeds: usize,
    binds: usize,
    ends_at: usize,
    closes: usize,
    straight: usize,
    carrier: usize,
    unread: usize,
    /// Steps credited with TWO OR MORE segments — a fillet's trimmed
    /// leg, its arc and its arrival, the `k != 1` case.
    multi: usize,
    /// Steps credited with at least one ARC segment.
    arcs: usize,
}

/// **Every step the door answered is answered with the segments its own
/// authored geometry describes.**
fn assert_attribution(
    steps: &[Step<f64>],
    per_step: &[Vec<ProfileEdgeRef>],
    verts: &[(Point2<f64>, f64)],
    what: &str,
    tally: &mut Tally,
) {
    let n = verts.len();
    let near = |a: Point2<f64>, b: Point2<f64>| (a - b).norm_squared() <= 1e-18;
    for (j, step) in steps.iter().enumerate() {
        let edges = &per_step[j];
        let arrives = |edges: &[ProfileEdgeRef]| {
            let e = edges.last().unwrap_or_else(|| {
                panic!(
                    "{what} step {j} ({:?}) names where it ends, so it produced \
                     a segment; the door credited it with none",
                    step.verb()
                )
            });
            verts[(e.segment as usize + 1) % n].0
        };
        match claim(j, step) {
            Claim::Seeds(p) => {
                tally.seeds += 1;
                assert!(
                    edges.is_empty(),
                    "{what} step {j} is the entry verb and lays down no segment, \
                     but the door credited it with {edges:?}"
                );
                assert!(
                    near(verts[0].0, p),
                    "{what} step {j} authored the chain's start at {p:?}; vertex 0 \
                     is at {:?} — the record's first boundary is not the entry's",
                    verts[0].0
                );
            }
            Claim::Binds => {
                tally.binds += 1;
                assert!(
                    edges.is_empty(),
                    "{what} step {j} ({:?}) binds tip state and emits nothing, but \
                     the door credited it with {edges:?}",
                    step.verb()
                );
            }
            Claim::EndsAt(p) => {
                tally.ends_at += 1;
                let end = arrives(edges);
                assert!(
                    near(end, p),
                    "{what} step {j} authored an arrival at {p:?}; the last segment \
                     the door gave it ({edges:?}) ends at {end:?} instead"
                );
            }
            Claim::Closes => {
                tally.closes += 1;
                let end = arrives(edges);
                assert!(
                    near(end, verts[0].0),
                    "{what} step {j} targets Start, so its last segment arrives at \
                     vertex 0 ({:?}); the door gave it {edges:?}, ending at {end:?}",
                    verts[0].0
                );
            }
            Claim::Straight(want) => {
                tally.straight += 1;
                assert_eq!(
                    edges.len(),
                    1,
                    "{what} step {j} is a `line` of one straight leg; the door gave \
                     it {edges:?}"
                );
                let s = edges[0].segment as usize;
                assert_eq!(
                    verts[s].1, 0.0,
                    "{what} step {j} is a `line`, so the segment it produced carries \
                     no bulge"
                );
                let got = (verts[(s + 1) % n].0 - verts[s].0).norm_squared();
                assert!(
                    (got - want * want).abs() <= 1e-9,
                    "{what} step {j} authored a leg of length {want}; the segment the \
                     door gave it is {} long",
                    got.sqrt()
                );
            }
            Claim::Carrier(centre, radius) => {
                tally.carrier += 1;
                assert_eq!(j, 0, "{what}: a carrier form authors one step, numbered 0");
                assert_eq!(
                    edges.len(),
                    n,
                    "{what}: the carrier's one step produced the whole loop"
                );
                for e in edges {
                    let v = verts[e.segment as usize].0;
                    let d = (v - centre).norm_squared();
                    assert!(
                        (d - radius * radius).abs() <= 1e-9,
                        "{what}: the carrier authored radius {radius} about {centre:?}, \
                         and segment {} leaves {v:?}, which is not on it",
                        e.segment
                    );
                }
            }
            Claim::Unread => tally.unread += 1,
        }
        if edges.len() >= 2 {
            tally.multi += 1;
        }
        if edges.iter().any(|e| verts[e.segment as usize].1 != 0.0) {
            tally.arcs += 1;
        }
    }
}

/// **Every authored step of every corpus profile is answered with the
/// segments its own arguments describe.**
///
/// This is the row a shifted ATTRIBUTION reds on, and it is the only
/// one: §1's partition and §2's walls both hold under a door that
/// credits every step with its neighbour's segments. The reading is
/// each step's own authored geometry — the point a `line_to` names, the
/// length a `line` names, the circle a carrier form names — never the
/// span the record carries or the door's own arithmetic.
///
/// The tally at the end is what makes "every step kind" a claim rather
/// than a hope: the corpus must still contain an entry verb, a
/// state-binding verb, an arrival at an authored point, a close, a
/// length-authored leg, a carrier form, a step credited with several
/// segments, and a step credited with an arc.
#[test]
fn every_corpus_step_is_answered_against_its_own_authored_geometry() {
    let mut tally = Tally::default();
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
            let r = records(&d.doc, program);
            for (li, loop_verts) in r.verts.iter().enumerate() {
                let per_step = edges_by_step(
                    program,
                    &r.structure,
                    &pv.naming,
                    li as u32,
                    r.steps[li].len(),
                    d.name,
                );
                assert_attribution(&r.steps[li], &per_step, loop_verts, d.name, &mut tally);
            }
        }
    }
    for (what, got) in [
        ("an entry verb", tally.seeds),
        ("a state-binding verb", tally.binds),
        ("an arrival at an authored point", tally.ends_at),
        ("a close", tally.closes),
        ("a length-authored leg", tally.straight),
        ("a carrier form", tally.carrier),
        ("a step credited with several segments", tally.multi),
        ("a step credited with an arc", tally.arcs),
    ] {
        assert!(
            got > 0,
            "the corpus walk reached no {what}, so this row is not the check it \
             says it is: {tally:?}"
        );
    }
}

/// **A `line_to(p)` step is answered with the one segment that ends at
/// `p`** — bit for bit, on a fixture small enough to name every step.
///
/// The corpus row above reads the same property over everything; this
/// one pins it exactly, because a straight leg stores its target
/// VERBATIM and so the arrival is not merely near `p`, it IS `p`.
/// Adopted from the `stepmap-rv` review probe.
#[test]
fn a_line_to_step_is_answered_with_the_segment_that_ends_where_it_says() {
    let (doc, profile, _) = prism(
        "step-segments-attribution",
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let r = records(&doc, program);
    let n = r.verts[0].len();
    let mut checked = 0_usize;
    for (j, step) in r.steps[0].iter().enumerate() {
        let Step::LineTo(Target::Point(p)) = step else {
            continue;
        };
        let edges = program
            .profile_edges_of(&r.structure, &pv.naming, 0, j as u32)
            .expect("the door answers");
        assert_eq!(
            edges.len(),
            1,
            "step {j} is a plain line_to: it produced exactly one segment, \
             the door gave it {edges:?}"
        );
        let s = edges[0].segment as usize;
        let end = r.verts[0][(s + 1) % n].0;
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

// ------------------------------------------------------------------
// 4. The refusals
// ------------------------------------------------------------------

/// **Two records of one permutation that disagree assert, naming both
/// of them.**
///
/// The permutation is recorded twice — once by canonicalization, once
/// by the bit-match that anchors the names — and ONE evaluation
/// produces both. So a disagreement is not a question the caller asked
/// badly, it is the evaluation contradicting itself, and DM8 rules
/// that it panics. The `expected` text is the whole message: the
/// invariant in words, then both records' values, which is what a
/// reader of the panic needs in order to tell which of the two lied.
#[test]
#[should_panic(expected = "the evaluation's two records of loop 0's permutation \
     disagree: canonicalization recorded reversed=false start=0 over 4 segments, \
     the naming anchor recorded reversed=true offset=0 over 4 vertices. One \
     evaluation produces both, so they describe one permutation or the kernel \
     has contradicted itself")]
fn two_records_describing_different_loops_assert() {
    let (doc, profile, _) = prism(
        "step-segments-refusal",
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let structure = records(&doc, program).structure;
    let value = ev.value(profile).expect("evaluates");
    let ValuePayload::Profile(pv) = &value.payload else {
        panic!("carries a profile");
    };
    // An anchor for the same loop, reversed the other way: the two
    // records now describe different permutations of one loop, which
    // one evaluation cannot have produced.
    let mut naming = pv.naming.clone();
    naming.loops[0].reversed = !naming.loops[0].reversed;
    let _ = program.profile_edges_of(&structure, &naming, 0, 0);
}

/// **A naming that does not mention the loop at all refuses typed.**
///
/// The sibling of the row above, and the reason the two are separate:
/// an ABSENT anchor is a record the caller did not supply, not two
/// records of one evaluation contradicting each other, so the door
/// answers the caller rather than panicking.
#[test]
fn a_naming_without_this_loop_refuses_rather_than_asserting() {
    let (doc, profile, _) = prism(
        "step-segments-no-anchor",
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let structure = records(&doc, program).structure;
    ev.value(profile).expect("evaluates");
    let empty = ProfileNaming::default();
    assert_eq!(
        program.profile_edges_of(&structure, &empty, 0, 0),
        Err(StepSegmentsError::NoAnchor { loop_: 0 })
    );
}

/// **A record that does not reach this loop, a record of the wrong
/// shape, and a span that runs off the end of the loop each refuse
/// typed.**
///
/// The three are the arms that stand between a FOREIGN record and an
/// out-of-range `ProfileEdgeRef`. A consumer holds a structure and a
/// naming that it believes go with this program, and nothing in the
/// types says they do; `SpanOffTheLoop` in particular is the last guard
/// before the door mints refs for segments the loop does not have.
#[test]
fn a_record_that_is_not_this_programs_refuses_rather_than_naming_segments() {
    let (doc, profile, _) = prism(
        "step-segments-foreign-record",
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let r = records(&doc, program);
    let n = r.verts[0].len();

    // A record that reaches no loop at all.
    let empty = ProfileStructure {
        replay: Vec::new(),
        canonical: CanonicalStructure { loops: Vec::new() },
    };
    assert_eq!(
        program.profile_edges_of(&empty, &pv.naming, 0, 0),
        Err(StepSegmentsError::NoRecord { loop_: 0 })
    );

    // A record about a program with a different number of steps: every
    // step index would then be about somebody else's program.
    let authored = r.steps[0].len();
    let mut short = r.structure.clone();
    short.replay[0].steps.pop();
    assert_eq!(
        program.profile_edges_of(&short, &pv.naming, 0, 0),
        Err(StepSegmentsError::RecordShape {
            loop_: 0,
            authored,
            recorded: authored - 1,
        })
    );

    // A span that reaches past the loop's last segment. The step count
    // is left alone, so the shape check passes and this is the arm that
    // fires.
    let mut off = r.structure.clone();
    let last = off.replay[0].steps.len() - 1;
    off.replay[0].steps[last] = profile::StepSpan::new(0, n + 1);
    assert_eq!(
        program.profile_edges_of(&off, &pv.naming, 0, last as u32),
        Err(StepSegmentsError::SpanOffTheLoop {
            step: last as u32,
            end: n + 1,
            segments: n,
        })
    );
}

// ------------------------------------------------------------------
// 5. The per-edge radius door
//
// `ProfileProgram::segment_radii` answers which radius each of a
// loop's edges is drawn at, from the replay's record of which segment
// each authored radius drew and through the permutation
// `profile_edges_of` checks. The rows here are about the PAIRING —
// that the expression handed back with a ref is the one the wall that
// ref names is drawn from — which §2 and §3 say nothing about,
// because neither reads a step's radius. The sharp case is the one
// the spans cannot answer: the step a radius is AUTHORED on is not
// the step its arc is credited to.
//
// Measured against the extruded solid: an answered ref's wall must be a
// CYLINDER at the answered expression's own radius, and every wall the
// door did not answer for must not be one. A door that paired the
// loop's first radius with every edge, or that answered in canonical
// indices, names a plane on the reversed and two-arc fixtures and reds
// here without any index arithmetic being re-done.
// ------------------------------------------------------------------

/// A chain of straight-then-arc legs, closed back to its start, on the
/// xy plane and extruded 1 unit. Answers the authored radius
/// expressions alongside, in program-step order.
///
/// `side` is what makes a fixture's winding: `Left` turns the chain
/// counterclockwise and canonicalization leaves it alone, `Right` turns
/// it clockwise and canonicalization reverses it. `radii` is one
/// quarter-turn arc apiece, each after a straight leg.
fn arc_prism(
    id: &str,
    side: profile::ArcSide,
    radii: &[f64],
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, Vec<Expr>) {
    let mut exprs = Vec::new();
    let mut steps = vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Toward {
            dx: fixture::scl(1.0),
            dy: fixture::scl(0.0),
        },
    ];
    for (i, &r) in radii.iter().enumerate() {
        if i > 0 {
            steps.push(ProgramStep::Tangent);
        }
        steps.push(ProgramStep::Line(len(if i == 0 { 4.0 } else { 2.0 })));
        steps.push(ProgramStep::Tangent);
        let expr = len(r);
        exprs.push(expr.clone());
        steps.push(ProgramStep::ArcTo(ProgramArcData::Sweep {
            r: expr,
            side,
            angle: fixture::ang(core::f64::consts::FRAC_PI_2),
        }));
    }
    steps.push(ProgramStep::LineTo(ProgramTarget::Start));
    let doc = ProfileDoc::empty_derived(id, tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(steps)],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, profile, ext, exprs)
}

/// The radius the wall named by `e` stores, `None` where that wall is
/// not a cylinder at all.
fn wall_radius(ev: &Evaluation<f64>, ext: RecipeNodeId, e: ProfileEdgeRef) -> Option<f64> {
    let face = lateral(ev, ext, e)?;
    let ValuePayload::Body(body) = &ev.value(ext)?.payload else {
        return None;
    };
    match body.get_surface(body.get_face(face)?.surface)? {
        geom::Surface::Cylinder { radius, .. } => Some(*radius),
        _ => None,
    }
}

/// **Every edge the door answered for is an arc at the answered
/// radius, and every edge it did not answer for is not an arc's.**
///
/// The shared body of the chain rows. The answer is checked to hold one
/// pair per authored arc, each carrying that arc's own expression and
/// naming a cylindrical wall at that expression's radius; the walls
/// left over are checked not to be cylinders, which is the half that
/// catches a door answering too widely.
fn assert_arcs_are_answered(id: &str, side: profile::ArcSide, radii: &[f64], want_reversed: bool) {
    let (doc, profile, ext, exprs) = arc_prism(id, side, radii);
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("{id}: the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("{id}: the profile node carries a profile");
    };
    let r = records(&doc, program);
    assert_eq!(
        r.structure.canonical.loops[0].reversed,
        want_reversed,
        "{id}: the fixture is written to be the {} case",
        if want_reversed {
            "reversed"
        } else {
            "identity"
        }
    );
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .unwrap_or_else(|e| panic!("{id}: the door answers: {e}"));
    assert_eq!(
        answer.len(),
        radii.len(),
        "{id}: one arc step, one answer — got {answer:?}"
    );
    let mut answered = BTreeSet::new();
    for (k, ((e, expr), want)) in answer.iter().zip(&exprs).enumerate() {
        assert_eq!(
            *expr, want,
            "{id}: the edges are answered in program-step order, so pair {k} carries \
             arc {k}'s own expression"
        );
        let got = wall_radius(&ev, ext, *e).unwrap_or_else(|| {
            panic!("{id}: {e:?} names no cylindrical wall, so it is not an arc's edge")
        });
        assert!(
            (got - radii[k]).abs() < 1e-9,
            "{id}: {e:?} was paired with the radius {} and its wall stores {got}",
            radii[k]
        );
        answered.insert(e.segment);
    }
    let n = r.verts[0].len();
    for segment in 0..n as u32 {
        if answered.contains(&segment) {
            continue;
        }
        let e = ProfileEdgeRef {
            loop_index: 0,
            segment,
        };
        assert_eq!(
            wall_radius(&ev, ext, e),
            None,
            "{id}: {e:?} was answered for by nobody, so its wall must not be an arc's"
        );
    }
}

/// **Two arcs of one chain are answered with two different radii, each
/// on the edge its own step drew.**
///
/// The mutant this reds is a door that hands every edge the loop's
/// FIRST radius — the shape the carrier forms' one-radius-per-loop rule
/// invites. It reds twice over: the second pair carries the wrong
/// expression, and the wall it names stores the wrong radius.
#[test]
fn each_arc_step_is_answered_with_the_edge_it_drew() {
    assert_arcs_are_answered(
        "segment-radii-two-arcs",
        profile::ArcSide::Left,
        &[1.0, 0.25],
        false,
    );
}

/// **The answer is in PROGRAM indices, on a loop canonicalization
/// REVERSED.**
///
/// Authored clockwise, so canonicalization reverses the chain to reach
/// the outer role's winding and canonical segment `k` is a different
/// segment from program segment `k`. A door that answered in canonical
/// indices — or a consumer that paired the program's radii with
/// canonical positions — names the wrong wall here and reds, while
/// every identity-permutation row above still passes.
#[test]
fn a_reversed_chains_arcs_are_answered_in_program_indices() {
    assert_arcs_are_answered(
        "segment-radii-reversed",
        profile::ArcSide::Right,
        &[1.0, 0.25],
        true,
    );
}

/// **A carrier loop answers its one radius on EVERY edge**, which is
/// what makes one door serve both loop shapes.
///
/// `circle_split` at n = 3 is the fixture, because a plain `circle`'s
/// two segments cannot tell "every edge" from "the first two". The
/// carrier form's single step replays to the whole loop, so the answer
/// is that step's radius repeated — not one pair, which is what a door
/// that treated every loop as a chain would give.
#[test]
fn a_carrier_loop_is_answered_at_every_edge() {
    let doc = ProfileDoc::empty_derived("segment-radii-carrier", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let radius = len(0.5);
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::CircleSplit {
                centre: [len(0.0), len(0.0)],
                radius: radius.clone(),
                n: 3,
                phase: fixture::ang(0.3),
            }],
        }),
    );
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let r = records(&doc, program);
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    let segments: Vec<u32> = answer.iter().map(|(e, _)| e.segment).collect();
    assert_eq!(
        segments,
        vec![0, 1, 2],
        "a split carrier's every edge is drawn at the loop's one radius"
    );
    for (_, expr) in &answer {
        assert_eq!(
            **expr, radius,
            "every edge carries the loop's own expression"
        );
    }
}

/// **A step carrying more than one radius answers EACH of them**, in
/// its own argument order, and a straight step answers nothing.
///
/// Read off the program alone, because that is where the rule lives:
/// `arc_fillet_arc` authors three radii — the incoming spec's, the
/// fillet's, the arrival spec's — and every one of them is a spelling
/// this program can lower onto a wall, so every one of them must reach
/// the content key. WHICH segment each drew is not a question the
/// program can answer and is not asked here; that is the replay's
/// emission record, read by [`ProfileProgram::segment_radii`].
///
/// The program is never replayed here, so the fused step's arguments
/// need not be a geometry that closes: what is under test is which
/// arguments the enumeration calls radii.
#[test]
fn a_step_with_several_radii_answers_each_of_them() {
    let spec = || ProgramArcData::Radius {
        r: len(1.0),
        side: profile::ArcSide::Left,
    };
    let fillet = len(0.5);
    let fused = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Line(len(1.0)),
        ProgramStep::ArcFilletArc {
            spec: spec(),
            radius: fillet.clone(),
            spec2: spec(),
        },
    ]);
    let answer = fused.step_radii();
    assert_eq!(
        answer.len(),
        3,
        "three radius arguments, three answers — got {answer:?}"
    );
    assert!(
        answer.iter().all(|(step, _)| *step == 2),
        "all three are the fused step's, and the straight leg beside it has none: \
         {answer:?}"
    );
    assert_eq!(
        *answer[1].1, fillet,
        "in the step's own argument order: incoming spec, fillet, arrival spec"
    );
    let one = LoopProgram::Chain(vec![ProgramStep::ArcTo(spec())]);
    assert_eq!(
        one.step_radii().len(),
        1,
        "the same spec ALONE carries exactly one radius and is answered, or the row \
         above is passing because the shape was never reached"
    );
}

/// Which authored step EMITTED a segment — the step whose recorded
/// span contains it, which is a different question from which step's
/// radius drew it and is the whole reason the emission record exists.
fn emitter_of(r: &Records, segment: u32) -> usize {
    let segment = segment as usize;
    r.structure.replay[0]
        .steps
        .iter()
        .position(|s| s.start() <= segment && segment < s.end())
        .unwrap_or_else(|| panic!("segment {segment} is in some step's span"))
}

/// **A `fillet(r)`'s radius reaches the wall its arc drew**, though
/// the step it is authored on emitted no segment at all.
///
/// `fillet` is a tip-state binder: it holds the radius and its
/// recorded span is EMPTY, and the arc it opens is emitted by the
/// ARRIVAL step, which holds no radius of its own. So the pairing
/// cannot come from the spans, and it does not: the replay records
/// which segment each authored radius drew as it emits, and the door
/// reads that. The row measures the answer against the extruded
/// solid — the answered edge's wall is a cylinder at the authored
/// radius — so a door crediting the arc to the arrival step, or
/// shifting the segment by one, names a plane here and reds.
#[test]
fn a_fillets_radius_reaches_its_arcs_wall() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let radius = len(0.5);
    let filleted = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 1.0))),
        ProgramStep::Toward {
            dx: fixture::scl(-1.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::Fillet(radius.clone()),
        ProgramStep::Toward {
            dx: fixture::scl(0.0),
            dy: fixture::scl(1.0),
        },
        ProgramStep::FarEndTo(pt(1.0, 3.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 3.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    assert_eq!(
        filleted.step_radii(),
        vec![(4, &radius)],
        "the program answers the fillet's radius at the step that holds it"
    );
    let doc = ProfileDoc::empty_derived("segment-radii-fillet", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![filleted],
        }),
    );
    let (doc, ext) = insert(
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
    let r = records(&doc, program);
    assert!(
        r.structure.replay[0].steps[4].is_empty(),
        "the fixture's fillet is the binder shape this row is about: it emitted \
         {}",
        r.structure.replay[0].steps[4]
    );
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    let [(edge, expr)] = answer[..] else {
        panic!("one radius, one arc, one pair — got {answer:?}");
    };
    assert_eq!(*expr, radius, "the pair carries the fillet's own spelling");
    let emitter = emitter_of(&r, edge.segment);
    assert_ne!(
        emitter, 4,
        "the arc it names was emitted by the ARRIVAL step and credited to the \
         BINDER, which is why the credit cannot be read off a span"
    );
    let LoopProgram::Chain(steps) = &program.loops[0] else {
        panic!("the fixture is a chain");
    };
    assert!(
        LoopProgram::Chain(vec![steps[emitter].clone()])
            .step_radii()
            .is_empty(),
        "and that arrival step holds no radius of its own: {:?}",
        steps[emitter]
    );
    let got = wall_radius(&ev, ext, edge)
        .unwrap_or_else(|| panic!("{edge:?} names no cylindrical wall, so it is not the arc's"));
    assert!(
        (got - 0.5).abs() < 1e-9,
        "and that wall is a cylinder at the authored radius, not {got}"
    );
    let attached: Vec<&Expr> = pv.edge_radii[0].iter().flatten().collect();
    assert_eq!(
        attached,
        vec![&radius],
        "the attach carries it too, on exactly one canonical segment"
    );
}

/// **A closed chain whose fillet arc is authored by the ARRIVAL
/// step** — the one shape of the finding nothing pinned.
///
/// `fillet_arc(r, spec)` emits the fillet arc AND the arrival spec's
/// arc, so its span is longer than one segment and the old "one
/// radius, one segment" rule dropped it. Here the arrival is a `Via`
/// close: the fillet arc is credited to the arrival step and carries
/// `r`, and the `Via` arc carries nothing, because a through-point is
/// not a radius argument. The filed row could not author this shape
/// inside its budget — its attempts refused `NoCornerForFillet` — and
/// the geometry that meets is a straight incoming ray east off the
/// chain's corner closing onto the circle about the origin through
/// (√2, √2).
#[test]
fn an_arrival_steps_fillet_arc_is_answered_and_its_via_arc_is_not() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let h = 2.0_f64.sqrt();
    let radius = len(0.5);
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
        ProgramStep::Toward {
            dx: fixture::scl(2.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::FilletArc {
            radius: radius.clone(),
            spec: ProgramArcData::Via {
                q: pt(h, h),
                target: ProgramTarget::Start,
            },
        },
        ProgramStep::Toward {
            dx: fixture::scl(-1.0),
            dy: fixture::scl(0.0),
        },
    ]);
    assert_eq!(
        chain.step_radii(),
        vec![(3, &radius)],
        "the arrival step holds one radius: its own fillet's, the Via spec carrying \
         a through-point instead"
    );
    let doc = ProfileDoc::empty_derived("segment-radii-via-arrival", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![chain],
        }),
    );
    let (doc, ext) = insert(
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
    let r = records(&doc, program);
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    let [(edge, expr)] = answer[..] else {
        panic!("one of the step's two arcs was drawn by a radius — got {answer:?}");
    };
    assert_eq!(*expr, radius);
    let emitter = emitter_of(&r, edge.segment);
    assert!(
        r.structure.replay[0].steps[emitter].len() > 1,
        "one step emitted the fillet arc AND the Via arc — telling those two apart \
         is what this row is about, and step {emitter} emitted {}",
        r.structure.replay[0].steps[emitter]
    );
    let got =
        wall_radius(&ev, ext, edge).unwrap_or_else(|| panic!("{edge:?} names no cylindrical wall"));
    assert!(
        (got - 0.5).abs() < 1e-9,
        "the answered edge is the FILLET arc's wall, at its own radius, not {got}"
    );
    // The Via arc is a cylinder too — at the carrier's radius, which
    // no argument of this program spells — so a door answering the
    // step's whole span would stamp it with the fillet's radius.
    let others: Vec<u32> = (0..r.verts[0].len() as u32)
        .filter(|s| *s != edge.segment)
        .filter(|s| {
            wall_radius(
                &ev,
                ext,
                ProfileEdgeRef {
                    loop_index: 0,
                    segment: *s,
                },
            )
            .is_some()
        })
        .collect();
    assert_eq!(
        others.len(),
        1,
        "the Via arc is the loop's other cylindrical wall: {others:?}"
    );
    assert_eq!(
        pv.edge_radii[0].iter().flatten().count(),
        1,
        "and nothing is attached to it: {:?}",
        pv.edge_radii[0]
    );
}

/// **The per-edge door's refusals are the map's own**, unaltered.
///
/// It composes [`ProfileProgram::profile_edges_of`] and adds no
/// question of its own, so it owes no refusal of its own either: a loop
/// this program does not have, and a record that does not describe it,
/// come back exactly as the map states them. The row pins that the
/// composition does not swallow one into an EMPTY answer, which is the
/// failure that would leave a caller attaching nothing and calling it a
/// profile with no radii.
#[test]
fn the_per_edge_door_refuses_where_the_map_does() {
    let (doc, profile, _, _) = arc_prism("segment-radii-refusal", profile::ArcSide::Left, &[1.0]);
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let r = records(&doc, program);
    assert_eq!(
        program.segment_radii(&r.structure, &pv.naming, 1),
        Err(StepSegmentsError::NoSuchLoop { loops: 1 })
    );
    let empty = ProfileStructure {
        replay: Vec::new(),
        canonical: CanonicalStructure { loops: Vec::new() },
    };
    assert_eq!(
        program.segment_radii(&empty, &pv.naming, 0),
        Err(StepSegmentsError::NoRecord { loop_: 0 })
    );
    let mut short = r.structure.clone();
    let authored = short.replay[0].steps.len();
    short.replay[0].steps.pop();
    assert_eq!(
        program.segment_radii(&short, &pv.naming, 0),
        Err(StepSegmentsError::RecordShape {
            loop_: 0,
            authored,
            recorded: authored - 1,
        })
    );
    assert_eq!(
        program.segment_radii(&r.structure, &ProfileNaming::default(), 0),
        Err(StepSegmentsError::NoAnchor { loop_: 0 })
    );
}

/// **A radius emission that does not describe this program refuses
/// typed**, in the two ways it can be wrong, and the sentence each
/// refusal writes is read once here.
///
/// The record arrives as a second argument, so nothing in the types
/// says it belongs to this program. An emission crediting a segment
/// the loop does not have would mint an out-of-range
/// [`ProfileEdgeRef`]; one crediting an argument the step it names
/// does not hold — a different role, or a step past the end of the
/// program — would pair an edge with somebody else's expression, or
/// with none. Both refuse where they are read, and neither guesses.
///
/// The RENDERING is compared whole rather than by substrings,
/// because the way this sentence went wrong was a duplicated word:
/// every radius role's label already ends in "radius"
/// (`StepArg::label`), so a template appending one of its own said
/// "carrier radius radius" and every substring a census could name
/// was still in it.
#[test]
fn a_radius_emission_that_is_not_this_programs_refuses_typed() {
    let (doc, profile, _, _) =
        arc_prism("segment-radii-bad-emission", profile::ArcSide::Left, &[1.0]);
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("carries a profile");
    };
    let r = records(&doc, program);
    let n = r.verts[0].len();
    assert_eq!(
        r.structure.replay[0].radii.len(),
        1,
        "the fixture's one arc step recorded one emission"
    );

    // A segment the loop does not have.
    let mut off = r.structure.clone();
    off.replay[0].radii[0].segment = n;
    let step = off.replay[0].radii[0].step as u32;
    assert_eq!(
        program.segment_radii(&off, &pv.naming, 0),
        Err(StepSegmentsError::EmissionOffTheLoop {
            step,
            arg: editor_core::StepArg::CarrierRadius,
            segment: n,
            segments: n,
        }),
        "an emission names ONE segment, so it draws the emission arm and not the \
         span arm, whose payload would be a range the record never carried"
    );
    assert_eq!(
        program
            .segment_radii(&off, &pv.naming, 0)
            .expect_err("it refuses")
            .to_string(),
        format!(
            "the record says step {step}'s carrier radius drew segment {n} on a loop with {n} of them"
        )
    );

    // A role the step it names does not hold: the `Sweep` spec is an
    // incoming carrier, and no argument of that step is an ARRIVAL
    // spec's radius.
    let mut role = r.structure.clone();
    role.replay[0].radii[0].role = profile::RadiusRole::Carrier2;
    assert_eq!(
        program.segment_radii(&role, &pv.naming, 0),
        Err(StepSegmentsError::RadiusNotAnArgument {
            step,
            arg: editor_core::StepArg::CarrierRadius2,
        })
    );
    assert_eq!(
        program
            .segment_radii(&role, &pv.naming, 0)
            .expect_err("it refuses")
            .to_string(),
        format!(
            "the record says step {step}'s arrival carrier radius drew a segment, \
             and that step holds no such argument"
        )
    );

    // A step past the end of the program, which is the same answer:
    // the record names an argument this program does not hold. The
    // STEP-COUNT guard cannot see it — the record's own step list is
    // the right length, and only the emission is wrong.
    let mut gone = r.structure.clone();
    let past = gone.replay[0].steps.len();
    gone.replay[0].radii[0].step = past;
    assert_eq!(
        program.segment_radii(&gone, &pv.naming, 0),
        Err(StepSegmentsError::RadiusNotAnArgument {
            step: past as u32,
            arg: editor_core::StepArg::CarrierRadius,
        })
    );
}

/// A closed chain whose lexicographic-minimum vertex is NOT its
/// program start, so canonicalization ROTATES it; `side` reverses it as
/// well. `Right` mirrors the whole chain in y, so its arcs close the
/// same shape the `Left` ones do.
///
/// [`arc_prism`]'s chains all begin at their own minimum, which makes
/// their anchor hop a pure reversal or the identity. This one adds the
/// other half of the permutation.
fn rotated_arc_prism(
    id: &str,
    side: profile::ArcSide,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, Vec<Expr>) {
    let s = if matches!(side, profile::ArcSide::Left) {
        1.0
    } else {
        -1.0
    };
    let r1 = len(1.0);
    let r2 = len(0.25);
    let pt = |x: f64, y: f64| [len(x), len(y * s)];
    let steps = vec![
        ProgramStep::At(pt(2.0, 2.0)),
        ProgramStep::Toward {
            dx: fixture::scl(1.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::Line(len(3.0)),
        ProgramStep::Tangent,
        ProgramStep::ArcTo(ProgramArcData::Sweep {
            r: r1.clone(),
            side,
            angle: fixture::ang(core::f64::consts::FRAC_PI_2),
        }),
        ProgramStep::Tangent,
        ProgramStep::Line(len(2.0)),
        ProgramStep::Tangent,
        ProgramStep::ArcTo(ProgramArcData::Sweep {
            r: r2.clone(),
            side,
            angle: fixture::ang(core::f64::consts::FRAC_PI_2),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 5.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ];
    let doc = ProfileDoc::empty_derived(id, tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(steps)],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, profile, ext, vec![r1, r2])
}

/// The shared body of the rotated rows: the door's answer AND the
/// attach's own list are both read against the geometry, on a loop
/// whose anchor hop is a rotation.
///
/// Two claims, because the rotation can be dropped at either end. The
/// door answers in program indices, checked by the wall each answered
/// ref names being a cylinder at the answered expression's radius. The
/// value's `edge_radii` is that answer re-addressed to CANONICAL
/// positions, checked position by position against the wall canonical
/// segment `j` swept: a consumer that carried the reversal through the
/// hop and dropped the rotation names a different wall here.
fn assert_rotated_arcs_are_answered(id: &str, side: profile::ArcSide, want_reversed: bool) {
    let (doc, profile, ext, exprs) = rotated_arc_prism(id, side);
    let ev = run(&doc);
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("{id}: the profile node is a program");
    };
    let ValuePayload::Profile(pv) = &ev.value(profile).expect("evaluates").payload else {
        panic!("{id}: carries a profile");
    };
    let r = records(&doc, program);
    let c = &r.structure.canonical.loops[0];
    assert_eq!(c.reversed, want_reversed, "{id}: the winding case");
    assert_ne!(c.start, 0, "{id}: the fixture is written to be ROTATED too");
    let a = &pv.naming.loops[0];
    assert_ne!(a.offset, 0, "{id}: the anchor hop is a rotation too");
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .unwrap_or_else(|e| panic!("{id}: the door answers: {e}"));
    assert_eq!(answer.len(), 2, "{id}: two arc steps, two answers");
    let radii = [1.0, 0.25];
    for (k, ((e, expr), want)) in answer.iter().zip(&exprs).enumerate() {
        assert_eq!(*expr, want, "{id}: pair {k} carries arc {k}'s expression");
        let got = wall_radius(&ev, ext, *e)
            .unwrap_or_else(|| panic!("{id}: {e:?} names no cylindrical wall"));
        assert!(
            (got - radii[k]).abs() < 1e-9,
            "{id}: {e:?} paired with {} and its wall stores {got}",
            radii[k]
        );
    }
    for (j, slot) in pv.edge_radii[0].iter().enumerate() {
        let e = ProfileEdgeRef {
            loop_index: 0,
            segment: a.segment(j as u32),
        };
        let wall = wall_radius(&ev, ext, e);
        match (slot, wall) {
            (Some(expr), Some(got)) => {
                let want = if *expr == exprs[0] { 1.0 } else { 0.25 };
                assert!(
                    (got - want).abs() < 1e-9,
                    "{id}: canonical segment {j} carries {expr:?} and its wall stores {got}"
                );
            }
            (None, None) => {}
            (radius, wall) => {
                panic!("{id}: canonical segment {j}: radius {radius:?} but wall {wall:?}")
            }
        }
    }
}

/// **The answer is in program indices on a loop canonicalization
/// ROTATES**, without reversing it.
///
/// Authored counterclockwise from a corner that is not the
/// lexicographic minimum, so `start` is non-zero and canonical segment
/// `k` is a different segment from program segment `k` by a shift. The
/// fixture asserts it IS that case before it asserts anything about the
/// door, per §2's rule.
#[test]
fn a_rotated_chains_arcs_are_answered_in_program_indices() {
    assert_rotated_arcs_are_answered("segment-radii-rot", profile::ArcSide::Left, false);
}

/// **Reversed AND rotated** — the anchor hop non-identity in both
/// senses, which is the case the other chain rows cannot see.
///
/// Every `arc_prism` chain starts at its own lexicographic-minimum
/// vertex, so its hop is a pure reversal and a consumer that applied
/// the reversal and dropped the rotation still names the right wall.
/// Here `offset` is non-zero too and it names the wrong one, at both
/// ends the rotation can be dropped: the door's own answer and the
/// canonical re-addressing `ProfileValue::edge_radii` carries.
#[test]
fn a_reversed_and_rotated_chains_arcs_are_answered_in_program_indices() {
    assert_rotated_arcs_are_answered("segment-radii-rot-rev", profile::ArcSide::Right, true);
}

/// **Every expression the ATTACH carries was written by the KEY's feed
/// first**, over every profile the corpus holds.
///
/// This is the inclusion the memo's stale-token guard rests on, read
/// mechanically rather than argued: `ProfileValue::edge_radii` is what
/// `param_source::profile_radius_tokens` lowers onto walls, and
/// `LoopProgram::step_radii` is what `eval::content_key` feeds, so a
/// spelling that appears in the first and not the second is one a wall
/// could carry while the document's key never named it.
///
/// The STRICT side is asserted too — at least one spelling that is
/// keyed and never attached — because an inclusion that happened to be
/// an equality would make the row pass while saying nothing about the
/// direction. Its instance is AUTHORED here rather than found in the
/// corpus: now that the emission record pairs a fillet's radius with
/// the arc it drew, the corpus's fillets are all attached, and the
/// remaining shape is a radius argument that draws NO segment — the
/// `Radius` spec of an `arc_fillet` whose carrier the arriving leg is
/// already on, which EXTENDS that leg's own segment rather than
/// emitting one of its own.
#[test]
fn every_attached_radius_was_keyed_first() {
    let mut attached = 0_usize;
    let mut keyed_only = 0_usize;
    let mut check = |name: &str, doc: &ProfileDoc| {
        let ev = run(doc);
        for &id in &ev.order {
            let Some(Node::Profile(program)) = doc.node(id) else {
                continue;
            };
            let Some(value) = ev.value(id) else { continue };
            let ValuePayload::Profile(pv) = &value.payload else {
                continue;
            };
            assert_eq!(
                pv.edge_radii.len(),
                pv.naming.loops.len(),
                "{name}: one canonical row per anchor"
            );
            for (ci, row) in pv.edge_radii.iter().enumerate() {
                let anchor = &pv.naming.loops[ci];
                assert_eq!(
                    row.len() as u32,
                    anchor.len,
                    "{name}: canonical loop {ci} has one slot per segment"
                );
                let fed: Vec<&Expr> = program.loops[anchor.program_loop as usize]
                    .step_radii()
                    .into_iter()
                    .map(|(_, e)| e)
                    .collect();
                for slot in row.iter().flatten() {
                    attached += 1;
                    assert!(
                        fed.contains(&slot),
                        "{name}: canonical loop {ci} attaches {slot:?}, which its \
                         program loop's key feed never wrote"
                    );
                }
            }
            let attached_here: Vec<&Expr> = pv.edge_radii.iter().flatten().flatten().collect();
            for lp in &program.loops {
                for (_, e) in lp.step_radii() {
                    if !attached_here.contains(&e) {
                        keyed_only += 1;
                    }
                }
            }
        }
    };
    for d in &corpus::documents() {
        check(d.name, &d.doc);
    }
    check("segment-radii-extension", &keyed_but_never_attached());
    assert!(attached > 0, "the corpus exercises the attach at all");
    assert!(
        keyed_only > 0,
        "the rows exercise the STRICT side of the inclusion too — a spelling that \
         is keyed and never attached, which is what makes the direction a claim"
    );
}

/// The document whose program authors a radius that draws no segment:
/// an `arc_fillet(Radius { .. })` off a directed leg end whose derived
/// carrier IS the leg's own, so the incoming side EXTENDS that leg
/// (the §4 item 4 vertex-move exemption) instead of emitting a segment
/// of its own. The spelling reaches the content key, as every authored
/// radius does, and no wall carries it.
fn keyed_but_never_attached() -> ProfileDoc {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let program = LoopProgram::Chain(vec![
        ProgramStep::ArcFilletArc {
            spec: ProgramArcData::Center {
                c: pt(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                target: ProgramTarget::Point(pt(5.0, 0.0)),
            },
            radius: len(0.5),
            spec2: ProgramArcData::Center {
                c: pt(0.0, 7.0),
                winding: profile::ArcSweep::Cw,
                target: ProgramTarget::Point(pt(0.0, 4.0)),
            },
        },
        ProgramStep::ArcFillet {
            spec: ProgramArcData::Radius {
                r: len(3.0),
                side: profile::ArcSide::Right,
            },
            radius: len(0.3),
        },
        ProgramStep::At(pt(-2.0, 2.0)),
        ProgramStep::Toward {
            dx: fixture::scl(0.0),
            dy: fixture::scl(-1.0),
        },
        ProgramStep::Line(len(1.0)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let doc = ProfileDoc::empty_derived("segment-radii-extension", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, _) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![program],
        }),
    );
    doc
}

/// **A bare `fillet(r)` BINDER has exactly one authorable position**:
/// mid-chain, with an arrival step of its own after it.
///
/// This is what bounds [`a_fillets_radius_reaches_its_arcs_wall`] to
/// the whole of `ProgramStep::Fillet`'s case rather than to one shape
/// of it. `Fillet` exists only inside `LoopProgram::Chain` — the
/// carrier forms hold no steps — and the loop's closer is refused in
/// the tip state a bare fillet leaves behind, in both of its
/// spellings.
///
/// **Scope: `ProgramStep::Fillet` and nothing else.** A fillet arc
/// CAN be a loop's closing segment — the fused arrival verbs close
/// on one at [`an_exact_fit_closing_fillet_arc_reaches_its_wall`] and
/// at the seam — and those are `FilletArc`/`ArcFilletArc` steps,
/// which author their own arrival rather than leaving the tip for a
/// later step to bind. What has no authorable position is the BARE
/// binder followed directly by the closer.
#[test]
fn a_fillet_cannot_be_a_loops_closing_corner() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let head = |closer: ProgramStep| {
        LoopProgram::Chain(vec![
            ProgramStep::At(pt(0.0, 0.0)),
            ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 0.0))),
            ProgramStep::LineTo(ProgramTarget::Point(pt(3.0, 3.0))),
            ProgramStep::Toward {
                dx: fixture::scl(-1.0),
                dy: fixture::scl(0.0),
            },
            ProgramStep::Fillet(len(0.5)),
            ProgramStep::Toward {
                dx: fixture::scl(0.0),
                dy: fixture::scl(-1.0),
            },
            closer,
        ])
    };
    for closer in [
        ProgramStep::LineTo(ProgramTarget::Start),
        ProgramStep::ContinueTo(ProgramTarget::Start),
    ] {
        let doc = ProfileDoc::empty_derived("segment-radii-closing-fillet", tol());
        let (doc, plane) = insert(doc, fixture::xy_frame());
        let attempt = doc.apply(
            &editor_core::DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane,
                    loops: vec![head(closer.clone())],
                }),
            },
            tol(),
            &editor_core::RefusingReach,
        );
        assert!(
            attempt.is_err(),
            "a fillet whose arrival is the loop closer is refused, so there is no \
             closing-fillet position a per-edge pairing would have to answer for"
        );
    }
}

/// **A fused step holds one radius ROLE per radius-bearing spec, and
/// three of the six arc specs bear none.**
///
/// `Radius`, `Sweep` and `ArcLen` hold a `CarrierRadius`; `Bulge`,
/// `Via` and `Center` hold a bulge, a through-point or a centre and no
/// radius at all. So a fused step over one of those three holds exactly
/// the fillet's own `StepArg::Radius`, `LoopProgram::step_radii`
/// answers one entry for it, and its spelling reaches the content key
/// like any other one-radius step's. What decides which SEGMENT that
/// radius reaches is the emission record — which is why
/// [`a_one_radius_fused_step_attaches_to_its_fillet_arc`] is a
/// separate row and not a corollary of
/// [`a_step_with_several_radii_answers_each_of_them`]: the count of
/// radius ROLES a step holds says nothing about which of its arcs each
/// one drew.
#[test]
fn a_fused_step_over_a_radius_less_spec_holds_one_radius() {
    let bulge = || ProgramArcData::Bulge {
        target: ProgramTarget::Point([len(1.0), len(1.0)]),
        b: fixture::scl(0.4),
    };
    let radius = len(0.5);
    let fused = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Line(len(1.0)),
        ProgramStep::ArcFillet {
            spec: bulge(),
            radius: radius.clone(),
        },
    ]);
    assert_eq!(
        fused.step_radii(),
        vec![(2, &radius)],
        "an arc_fillet over a BULGE spec holds exactly one radius role, so the \
         multi-radius filter does not reach it"
    );
    let three = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Line(len(1.0)),
        ProgramStep::ArcFilletArc {
            spec: bulge(),
            radius: radius.clone(),
            spec2: bulge(),
        },
    ]);
    assert_eq!(
        three.step_radii(),
        vec![(2, &radius)],
        "nor an arc_fillet_arc over two of them"
    );
}

/// **A one-radius FUSED step attaches to its fillet arc**, on a step
/// whose own span is empty.
///
/// `arc_fillet(spec, r)` authors an incoming arc carrier AND opens a
/// fillet off it in one act, and the step is a BINDER: its recorded
/// span is EMPTY, and the incoming arc, the fillet arc and the
/// arrival leg are all emitted by later steps, which hold no radius
/// of their own. The emission record is what pairs the radius with
/// the one of those segments it drew.
///
/// The fixture's incoming spec is a BULGE, which bears no radius role
/// at all — so the step holds exactly one radius and the row is about
/// WHICH of the step's arcs that radius drew, not about how many
/// radii it holds. The bulge's own arc is answered by nobody, because
/// no argument of this program is its radius.
#[test]
fn a_one_radius_fused_step_attaches_to_its_fillet_arc() {
    let radius = len(0.2);
    let program = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::ArcFillet {
            spec: ProgramArcData::Bulge {
                target: ProgramTarget::Point([len(3.0), len(0.0)]),
                b: fixture::scl(0.2),
            },
            radius: radius.clone(),
        },
        ProgramStep::Toward {
            dx: fixture::scl(0.0),
            dy: fixture::scl(1.0),
        },
        ProgramStep::FarEndTo([len(3.3), len(4.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(4.0)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    assert_eq!(
        program.step_radii(),
        vec![(1, &radius)],
        "the program answers the fused step's one radius, so its spelling reaches \
         the content key"
    );
    let doc = ProfileDoc::empty_derived("segment-radii-fused-bulge", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let applied = doc
        .apply(
            &editor_core::DocEdit::InsertNode {
                node: Node::Profile(ProfileProgram {
                    plane,
                    loops: vec![program],
                }),
            },
            tol(),
            &editor_core::RefusingReach,
        )
        .expect("a fused step over a bulge spec is authorable and replays");
    let doc = applied.doc;
    let profile = *doc.order().last().expect("the inserted profile node");
    let (doc, ext) = insert(
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
    let r = records(&doc, program);
    assert!(
        r.structure.replay[0].steps[1].is_empty(),
        "the fused step is the binder shape this row is about: it emitted {}",
        r.structure.replay[0].steps[1]
    );
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    let [(edge, expr)] = answer[..] else {
        panic!("the step's one radius drew one arc — got {answer:?}");
    };
    assert_eq!(*expr, radius);
    assert_ne!(
        emitter_of(&r, edge.segment),
        1,
        "and the step that drew it is not the step that emitted it"
    );
    let got =
        wall_radius(&ev, ext, edge).unwrap_or_else(|| panic!("{edge:?} names no cylindrical wall"));
    assert!(
        (got - 0.2).abs() < 1e-9,
        "the answered wall is the fillet arc's, at its own radius, not {got}"
    );
    assert_eq!(
        pv.edge_radii[0].iter().flatten().collect::<Vec<_>>(),
        vec![&radius],
        "the attach carries exactly that one: {:?}",
        pv.edge_radii[0]
    );
}

/// **Three radii on one step, three segments, three addresses.**
///
/// `arc_fillet_arc(Sweep, r, Radius)` authors the incoming arc
/// carrier, the fillet off it and the arrival arc carrier in one act,
/// each from its own argument — and, because a `Radius` arrival binds
/// its anchor and its director on later steps, ALL THREE arcs are
/// emitted by a step that holds no radius at all. Every wall is
/// measured against the extruded solid, so a record that swapped the
/// incoming and arrival roles, or shifted a segment by one, stamps two
/// walls with the wrong expression and reds here.
#[test]
fn a_fused_steps_three_radii_each_reach_their_own_wall() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let side = profile::ArcSide::Left;
    let (carrier, fillet, carrier2) = (len(2.0), len(0.25), len(3.0));
    let program = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Angle(fixture::ang(0.0)),
        ProgramStep::Line(len(4.0)),
        ProgramStep::Tangent,
        ProgramStep::ArcFilletArc {
            spec: ProgramArcData::Sweep {
                r: carrier.clone(),
                side,
                angle: fixture::ang(0.6),
            },
            radius: fillet.clone(),
            spec2: ProgramArcData::Radius {
                r: carrier2.clone(),
                side,
            },
        },
        ProgramStep::At(pt(2.0, 6.0)),
        ProgramStep::Toward {
            dx: fixture::scl(-1.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::Line(len(2.0)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    assert_eq!(
        program.step_radii(),
        vec![(4, &carrier), (4, &fillet), (4, &carrier2)],
        "the key's feed takes all three spellings"
    );
    let doc = ProfileDoc::empty_derived("segment-radii-three", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![program],
        }),
    );
    let (doc, ext) = insert(
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
    let r = records(&doc, program);
    assert!(
        r.structure.replay[0].steps[4].is_empty(),
        "the fused step emitted {} — the `Radius` arrival's binders emit its arcs",
        r.structure.replay[0].steps[4]
    );
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    assert_eq!(answer.len(), 3, "three radii, three arcs — got {answer:?}");
    for ((edge, expr), want) in answer.iter().zip([2.0, 0.25, 3.0]) {
        let got = wall_radius(&ev, ext, *edge)
            .unwrap_or_else(|| panic!("{edge:?} names no cylindrical wall"));
        assert!(
            (got - want).abs() < 1e-9,
            "{edge:?} was paired with {expr:?} and its wall stores {got}, not {want}"
        );
    }
    let mut segments: Vec<u32> = answer.iter().map(|(e, _)| e.segment).collect();
    segments.sort_unstable();
    segments.dedup();
    assert_eq!(segments.len(), 3, "three DIFFERENT segments");
    assert_eq!(
        pv.edge_radii[0].iter().flatten().count(),
        3,
        "and the attach carries all three: {:?}",
        pv.edge_radii[0]
    );
}

// ------------------------------------------------------------------
// 5b. The permutation, the record's own shape, and the two mode
//     vocabularies
//
// §5's rows above are all authored counter-clockwise from the
// lexicographic minimum, so the identity map `segment_radii` applies
// is exercised against a fillet only where the anchor hop IS the
// identity. The rows here put a fillet on the hops that are not, read
// the record's own shape back at the two doors that check it, and
// hold the two vocabularies that decide "does this spec carry a
// radius" to one answer.
// ------------------------------------------------------------------

/// A closed chain with a `fillet(r)` BINDER whose canonical loop is
/// ROTATED (its lexicographic minimum (0,0) is not vertex 0) and,
/// under `s = -1` (a mirror in y), REVERSED as well. The binder's arc
/// is emitted by the far-end arrival, so the emission record is the
/// only thing pairing it, and the anchor hop is a non-identity in
/// both senses.
fn rotated_fillet_prism(id: &str, s: f64) -> (fixture::Swept, Expr) {
    let pt = |x: f64, y: f64| [len(x), len(y * s)];
    let radius = len(0.5);
    let steps = vec![
        ProgramStep::At(pt(2.0, 2.0)),
        ProgramStep::Toward {
            dx: fixture::scl(1.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::Line(len(2.0)),
        ProgramStep::Fillet(radius.clone()),
        ProgramStep::Toward {
            dx: fixture::scl(0.0),
            dy: fixture::scl(s),
        },
        ProgramStep::FarEndTo(pt(5.0, 5.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 6.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ];
    (
        fixture::wall_row(id, vec![LoopProgram::Chain(steps)]),
        radius,
    )
}

/// The one answered edge is the fillet arc's wall, a cylinder at `r`,
/// on a loop whose anchor hop is a rotation (and a reversal); and the
/// canonical re-addressing `edge_radii` carries lands on that same
/// wall.
fn assert_rotated_fillet_is_answered(id: &str, s: f64, want_reversed: bool) {
    let (row, radius) = rotated_fillet_prism(id, s);
    let program = row.program();
    let pv = row.profile_value();
    let r = records(&row.doc, program);
    let c = &r.structure.canonical.loops[0];
    assert_eq!(c.reversed, want_reversed, "{id}: the winding case");
    assert_ne!(c.start, 0, "{id}: the fixture is ROTATED");
    let a = &pv.naming.loops[0];
    assert_ne!(a.offset, 0, "{id}: the anchor hop is a rotation");
    assert!(
        r.structure.replay[0].steps[3].is_empty(),
        "{id}: the binder emitted nothing"
    );
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .unwrap_or_else(|e| panic!("{id}: the door answers: {e}"));
    let [(edge, expr)] = answer[..] else {
        panic!("{id}: one binder, one arc, one pair — got {answer:?}");
    };
    assert_eq!(*expr, radius);
    assert_ne!(
        emitter_of(&r, edge.segment),
        3,
        "{id}: credited to the binder, emitted elsewhere"
    );
    let got = wall_radius(&row.ev, row.ext, edge)
        .unwrap_or_else(|| panic!("{id}: {edge:?} names no cylindrical wall — the neighbour"));
    assert!((got - 0.5).abs() < 1e-9, "{id}: the wall stores {got}");
    let mut cylinders = 0;
    for (j, slot) in pv.edge_radii[0].iter().enumerate() {
        let e = ProfileEdgeRef {
            loop_index: 0,
            segment: a.segment(j as u32),
        };
        match (slot, wall_radius(&row.ev, row.ext, e)) {
            (Some(expr), Some(got)) => {
                cylinders += 1;
                assert_eq!(*expr, radius);
                assert!((got - 0.5).abs() < 1e-9, "{id}: canonical {j} stores {got}");
            }
            (None, None) => {}
            (slot, wall) => {
                panic!("{id}: canonical segment {j}: radius {slot:?} but wall {wall:?}")
            }
        }
    }
    assert_eq!(cylinders, 1, "{id}: exactly one wall is the fillet arc's");
}

/// **A `fillet(r)` binder on a ROTATED loop reaches its own wall.**
#[test]
fn a_rotated_loops_fillet_binder_reaches_its_arcs_wall() {
    assert_rotated_fillet_is_answered("fillet-rot", 1.0, false);
}

/// **A `fillet(r)` binder on a REVERSED AND ROTATED loop reaches its
/// own wall** — the identity map from recorded segment to published
/// ref, on the hop that is a non-identity both ways.
#[test]
fn a_reversed_and_rotated_loops_fillet_binder_reaches_its_arcs_wall() {
    assert_rotated_fillet_is_answered("fillet-rot-rev", -1.0, true);
}

/// **An arrival step's fillet arc on a REVERSED AND ROTATED loop** —
/// [`an_arrival_steps_fillet_arc_is_answered_and_its_via_arc_is_not`]
/// mirrored in x: the chain runs clockwise and its lexicographic
/// minimum is the fillet arc's own end, so the hop is both.
#[test]
fn a_reversed_and_rotated_via_closes_fillet_arc_reaches_its_wall() {
    let s = -1.0;
    let pt = |x: f64, y: f64| [len(x * s), len(y)];
    let h = 2.0_f64.sqrt();
    let radius = len(0.5);
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
        ProgramStep::Toward {
            dx: fixture::scl(2.0 * s),
            dy: fixture::scl(0.0),
        },
        ProgramStep::FilletArc {
            radius: radius.clone(),
            spec: ProgramArcData::Via {
                q: pt(h, h),
                target: ProgramTarget::Start,
            },
        },
        ProgramStep::Toward {
            dx: fixture::scl(-s),
            dy: fixture::scl(0.0),
        },
    ]);
    let row = fixture::wall_row("via-rot-rev", vec![chain]);
    let program = row.program();
    let pv = row.profile_value();
    let r = records(&row.doc, program);
    let c = &r.structure.canonical.loops[0];
    assert!(c.reversed, "the mirrored chain is clockwise");
    assert_ne!(c.start, 0, "and rotated");
    assert_ne!(pv.naming.loops[0].offset, 0);
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    let [(edge, expr)] = answer[..] else {
        panic!("one radius — got {answer:?}");
    };
    assert_eq!(*expr, radius);
    let got =
        wall_radius(&row.ev, row.ext, edge).unwrap_or_else(|| panic!("{edge:?} names no cylinder"));
    assert!(
        (got - 0.5).abs() < 1e-9,
        "the FILLET arc's wall, not the Via arc's: {got}"
    );
    let attached: Vec<usize> = pv.edge_radii[0]
        .iter()
        .enumerate()
        .filter_map(|(j, s)| s.as_ref().map(|_| j))
        .collect();
    let [j] = attached[..] else {
        panic!("one canonical slot: {attached:?}");
    };
    let e = ProfileEdgeRef {
        loop_index: 0,
        segment: pv.naming.loops[0].segment(j as u32),
    };
    let got = wall_radius(&row.ev, row.ext, e).expect("the attached slot is a cylinder");
    assert!(
        (got - 0.5).abs() < 1e-9,
        "the attach lands on the fillet arc's wall: {got}"
    );
}

/// **An exact-fit closing fillet arc reaches its wall**, credited to
/// the arrival step at the closing segment.
///
/// `family::resolve_arc_close`'s exact-fit arm: the fillet arc is the
/// whole arrival side and IS the closing segment (three vertices, no
/// carrier run). The entry sits exactly at the tangent point of the
/// r = 0.5 fillet between the ray y = 0 from the origin and the circle
/// of radius 2 about the origin.
#[test]
fn an_exact_fit_closing_fillet_arc_reaches_its_wall() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let radius = len(0.5);
    let tp = (2.0_f64.sqrt() * 4.0 / 3.0, 2.0 / 3.0);
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(pt(tp.0, tp.1)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
        ProgramStep::Toward {
            dx: fixture::scl(2.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::FilletArc {
            radius: radius.clone(),
            spec: ProgramArcData::Center {
                c: pt(0.0, 0.0),
                winding: profile::ArcSweep::Ccw,
                target: ProgramTarget::Start,
            },
        },
    ]);
    let row = fixture::wall_row("exact-fit", vec![chain]);
    let program = row.program();
    let pv = row.profile_value();
    let r = records(&row.doc, program);
    assert_eq!(
        r.verts[0].len(),
        3,
        "exact fit: no carrier run was emitted, the fillet arc closes: {:?}",
        r.verts[0]
    );
    let rec: Vec<(usize, profile::RadiusRole, usize)> = r.structure.replay[0]
        .radii
        .iter()
        .map(|e| (e.step, e.role, e.segment))
        .collect();
    assert_eq!(rec, vec![(3, profile::RadiusRole::Fillet, 2)]);
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    let [(edge, expr)] = answer[..] else {
        panic!("one radius — got {answer:?}");
    };
    assert_eq!(*expr, radius);
    assert_eq!(edge.segment, 2, "the closing segment");
    let got =
        wall_radius(&row.ev, row.ext, edge).unwrap_or_else(|| panic!("{edge:?} names no cylinder"));
    assert!(
        (got - 0.5).abs() < 1e-9,
        "a cylinder at the fillet's radius: {got}"
    );
    assert_eq!(pv.edge_radii[0].iter().flatten().count(), 1);
}

/// **A `fillet_arc(r, Radius spec)` reaches BOTH its walls** — the
/// fillet arc at `r` and the arrival carrier's arc at the spec's own
/// radius, two roles on one step.
///
/// The arrival `Radius` spec is the only one of the six that carries a
/// radius in second position, and the two rows that reach it today are
/// both `arc_fillet_arc(Sweep, r, Radius)`, where a THIRD radius is in
/// play. This is the two-role shape on its own, so a `Carrier`/
/// `Carrier2` confusion has nowhere to hide behind the incoming
/// spec's own carrier.
#[test]
fn a_fillet_arcs_two_radii_each_reach_their_own_wall() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let fillet = len(0.25);
    let carrier = len(3.0);
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Angle(fixture::ang(0.0)),
        ProgramStep::Line(len(4.0)),
        ProgramStep::FilletArc {
            radius: fillet.clone(),
            spec: ProgramArcData::Radius {
                r: carrier.clone(),
                side: profile::ArcSide::Left,
            },
        },
        ProgramStep::At(pt(2.0, 5.0)),
        ProgramStep::Toward {
            dx: fixture::scl(-1.0),
            dy: fixture::scl(0.0),
        },
        ProgramStep::Line(len(2.0)),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    assert_eq!(
        chain.step_radii(),
        vec![(3, &fillet), (3, &carrier)],
        "one step, two radius arguments, in slot order"
    );
    let row = fixture::wall_row("fillet-arc-radius", vec![chain]);
    let program = row.program();
    let pv = row.profile_value();
    let r = records(&row.doc, program);
    let answer = program
        .segment_radii(&r.structure, &pv.naming, 0)
        .expect("the door answers");
    assert_eq!(answer.len(), 2, "two radii, two arcs — got {answer:?}");
    for (edge, expr) in &answer {
        let want = if **expr == fillet { 0.25 } else { 3.0 };
        assert!(
            **expr == fillet || **expr == carrier,
            "each pair carries one of the step's own two radii"
        );
        let got = wall_radius(&row.ev, row.ext, *edge)
            .unwrap_or_else(|| panic!("{edge:?} names no cylindrical wall"));
        assert!(
            (got - want).abs() < 1e-9,
            "{edge:?} was paired with {want} and its wall stores {got}"
        );
    }
    assert_ne!(
        answer[0].0, answer[1].0,
        "the two radii drew two DIFFERENT segments"
    );
}

/// **A carrier loop's record is checked at the same two doors a
/// chain's is.**
///
/// `segment_radii`'s carrier arm answers per LOOP, and the shape that
/// invites is an enumeration of its own — `0..segments` off the
/// canonical record, reading neither the recorded span nor the
/// emission list. Then a record this program did not produce is
/// refused by `profile_edges_of` and answered by `segment_radii`,
/// which is the divergence factoring `checked_records` out was for,
/// one arm further out. Both halves are measured: step 0's span is
/// the one walk both doors take, and a record carrying emissions is a
/// chain's record under a carrier program.
#[test]
fn a_carrier_loops_record_is_checked_at_the_same_doors_a_chains_is() {
    let row = fixture::wall_row(
        "carrier-record-shape",
        vec![LoopProgram::CircleSplit {
            centre: [len(0.0), len(0.0)],
            radius: len(1.0),
            n: 4,
            phase: fixture::ang(0.0),
        }],
    );
    let program = row.program();
    let pv = row.profile_value();
    let structure = records(&row.doc, program).structure;
    let segments = structure.canonical.loops[0].segments.len();
    assert_eq!(segments, 4, "the split loop has four segments");
    assert_eq!(
        program
            .segment_radii(&structure, &pv.naming, 0)
            .expect("the door answers")
            .len(),
        segments,
        "and every one of them is answered at the loop's one radius"
    );

    // Step 0's span cut wrong — a record of a DIFFERENT program with
    // the right step count, which is what `profile_edges_of`'s span
    // bound exists to refuse.
    let mut short = structure.clone();
    short.replay[0].steps[0] = profile::StepSpan::new(0, segments + 3);
    let want = Err(StepSegmentsError::SpanOffTheLoop {
        step: 0,
        end: segments + 3,
        segments,
    });
    assert_eq!(
        program.profile_edges_of(&short, &pv.naming, 0, 0),
        want.map(|()| Vec::new()),
        "the per-step door refuses a span off the loop"
    );
    assert_eq!(
        program
            .segment_radii(&short, &pv.naming, 0)
            .map(|v| v.len()),
        want.map(|()| 0),
        "and so does the per-edge radius door, through the same walk"
    );

    // A record carrying emissions. A carrier form emits none — its
    // radius is the whole boundary's — so this record is a chain's.
    let mut emitting = structure.clone();
    emitting.replay[0].radii.push(profile::RadiusEmission {
        step: 0,
        role: profile::RadiusRole::Fillet,
        segment: 0,
    });
    assert_eq!(
        program.segment_radii(&emitting, &pv.naming, 0),
        Err(StepSegmentsError::CarrierRecordsEmissions {
            loop_: 0,
            emissions: 1,
        }),
        "read before the answer, not after the arm has returned"
    );
}

/// **The two vocabularies that decide whether an arc spec carries a
/// radius agree, mode for mode, in both positions.**
///
/// `profile::ArcData::carries_radius` decides whether an emitted arc
/// records a `Carrier`/`Carrier2` address at all; `editor-core`'s
/// `spec_slots` decides whether the same spec holds a
/// `CarrierRadius`/`CarrierRadius2` argument for that address to be
/// read against. They are two total matches over one six-mode
/// vocabulary deciding one fact, in two crates, and the only thing
/// that notices a disagreement in a running kernel is
/// `eval::wire::edge_radii`'s `unreachable!` — after the emission has
/// already been recorded with nowhere to land.
///
/// The array below is total over [`ProgramArcData`]: the match that
/// names each mode's answer binds every variant, so a mode the
/// vocabulary gains is an E0004 here rather than a silent gap.
#[test]
fn every_arc_mode_carries_a_radius_in_both_vocabularies_or_in_neither() {
    let h = 2.0_f64.sqrt();
    let modes = [
        ProgramArcData::Radius {
            r: len(2.0),
            side: profile::ArcSide::Left,
        },
        ProgramArcData::Bulge {
            target: ProgramTarget::Point([len(2.0), len(1.0)]),
            b: fixture::scl(0.3),
        },
        ProgramArcData::Via {
            q: [len(h), len(h)],
            target: ProgramTarget::Point([len(2.0), len(1.0)]),
        },
        ProgramArcData::Center {
            c: [len(0.0), len(0.0)],
            winding: profile::ArcSweep::Ccw,
            target: ProgramTarget::Point([len(2.0), len(1.0)]),
        },
        ProgramArcData::Sweep {
            r: len(2.0),
            side: profile::ArcSide::Left,
            angle: fixture::ang(0.6),
        },
        ProgramArcData::ArcLen {
            r: len(2.0),
            side: profile::ArcSide::Left,
            len: len(1.5),
        },
    ];
    let env = ProfileDoc::empty_derived("mode-vocabularies", tol()).param_env::<f64>();
    for spec in modes {
        let carries = match &spec {
            ProgramArcData::Radius { .. }
            | ProgramArcData::Sweep { .. }
            | ProgramArcData::ArcLen { .. } => true,
            ProgramArcData::Bulge { .. }
            | ProgramArcData::Via { .. }
            | ProgramArcData::Center { .. } => false,
        };

        // `profile`'s side, read off the resolved spec itself. No
        // geometry: the question is about the spec's own vocabulary,
        // and every mode is resolvable in every position whether or
        // not the lattice admits it there.
        for (position, step) in [
            ("incoming", ProgramStep::ArcTo(spec.clone())),
            (
                "arrival",
                ProgramStep::FilletArc {
                    radius: len(0.5),
                    spec: spec.clone(),
                },
            ),
        ] {
            let resolved = LoopProgram::Chain(vec![step])
                .resolve::<f64>(&env, 0)
                .expect("a literal spec resolves");
            let wire = match &resolved[0] {
                Step::ArcTo(w) => w,
                Step::FilletArc { spec, .. } => spec,
                other => panic!("{position}: the step resolved to {other:?}"),
            };
            assert_eq!(
                wire.carries_radius(),
                carries,
                "{spec:?} in {position} position: `ArcData::carries_radius` and this \
                 row disagree, so the emission record's Carrier roles and the \
                 document's CarrierRadius arguments are two different sets"
            );
        }

        // `editor-core`'s side, read through the public argument
        // enumerator rather than a second copy of `spec_slots`.
        assert_eq!(
            LoopProgram::Chain(vec![ProgramStep::ArcTo(spec.clone())])
                .step_args()
                .contains(&(0, editor_core::StepArg::CarrierRadius)),
            carries,
            "{spec:?}: the incoming position's argument enumeration"
        );
        assert_eq!(
            LoopProgram::Chain(vec![ProgramStep::FilletArc {
                radius: len(0.5),
                spec: spec.clone(),
            }])
            .step_args()
            .contains(&(0, editor_core::StepArg::CarrierRadius2)),
            carries,
            "{spec:?}: the arrival position's argument enumeration"
        );
    }
}

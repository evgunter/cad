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
//! **3. The PAIRING** (§5), for the door's one built consumer:
//! `ProfileProgram::segment_radii` reads the map for one argument role
//! and answers which radius each EDGE is drawn at. That is a third
//! thing that can be wrong independently — the refs can be right and
//! the expression beside one of them belong to another step — so it
//! has its own rows.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use std::collections::BTreeSet;

use crate::corpus;
use crate::fixture;

use editor_core::{
    CancelToken, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, Expr, LoopProgram, Node,
    ProfileDoc, ProfileEdgeRef, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, RoleSeg, StableName, StepSegmentsError, ValuePayload, eval::ProfileNaming,
    evaluate,
};
use fixture::{insert, len, on_frame};
use geom_core::{Point2, Tol};
use profile::{CanonicalStructure, ProfileStructure, SketchPlane, Step, Target};
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
fn edges_by_step(
    program: &ProfileProgram,
    structure: &ProfileStructure,
    naming: &ProfileNaming,
    loop_: u32,
    steps: usize,
    what: &str,
) -> Vec<Vec<ProfileEdgeRef>> {
    (0..steps)
        .map(|step| {
            program
                .profile_edges_of(structure, naming, loop_, step as u32)
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
            let pts = face_points(body, face);
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
// `ProfileProgram::segment_radii` is `profile_edges_of` read for one
// argument role: which radius each of a loop's edges is drawn at. The
// rows here are about the PAIRING — that the expression handed back
// with a ref is the one the wall that ref names is drawn from — which
// §2 and §3 say nothing about, because neither reads a step's radius.
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

/// **A step carrying more than one radius answers NOTHING**, and a
/// straight step answers nothing either.
///
/// Read off the program alone, because that is where the rule lives:
/// `arc_fillet_arc` authors three radii — the incoming spec's, the
/// fillet's, the arrival spec's — and the record says only which
/// segments the step emitted, never which of its radii drew which. A
/// door that answered the first would stamp two of those walls with an
/// expression they are not drawn from, and no per-edge row over a
/// single-radius chain can see it.
///
/// The program is never replayed here, so the fused step's arguments
/// need not be a geometry that closes: what is under test is which
/// arguments the enumeration calls radii.
#[test]
fn a_step_with_several_radii_answers_no_radius() {
    let spec = || ProgramArcData::Radius {
        r: len(1.0),
        side: profile::ArcSide::Left,
    };
    let fused = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::Line(len(1.0)),
        ProgramStep::ArcFilletArc {
            spec: spec(),
            radius: len(0.5),
            spec2: spec(),
        },
    ]);
    assert!(
        fused.step_radii().is_empty(),
        "a step with three radii says which segment none of them drew, and the \
         straight leg beside it has none at all"
    );
    let one = LoopProgram::Chain(vec![ProgramStep::ArcTo(spec())]);
    assert_eq!(
        one.step_radii().len(),
        1,
        "the same spec ALONE carries exactly one radius and is answered, or the row \
         above is passing because the shape was never reached"
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

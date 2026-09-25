//! **A live profile's program is replaced whole, and a profile piece's
//! name spells the step that drew it** (`DocEdit::SetProgram`; V2 and V3
//! of `crates/profile/README.md`; `names/README.md`, "N1, the profile
//! pieces"; DM7).
//!
//! Every holder of a profile name — a fillet's selection, a derived
//! frame's face, a paint in the appearance store — holds a
//! `ProfileEdgeRef`/`ProfileVertexRef` spelled by the id its step was
//! minted with and the role the piece plays in that step. A reshaping
//! states which old step each new step keeps, by id; a kept step's
//! names keep denoting its pieces wherever the new program draws them,
//! and nothing is rewritten or reported, while a dropped step's names
//! keep their spelling, resolve `Vanished`, and are reported stranded.
//! A value edit moves no name at all: which loop is outer, which way a
//! loop runs and how many segments a step draws are decisions about
//! geometry, not about what the author made.
//!
//! The fixture most rows share is `edit_ruled_carve`'s sunk rod: a
//! block with a rod's section standing on its top edge, whose two
//! cylinder-meets-plane creases are the one straight edge a fillet
//! carves on a prism with transverse caps — so a fillet on ONE
//! profile vertex evaluates, before and after the reshaping, and the
//! wall it lands on is measured on the solid rather than read off a
//! spelling.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/node.rs",
    "crates/editor-core/src/program.rs",
    "crates/editor-core/src/eval/anchor.rs",
    "crates/editor-core/src/persist/",
    "crates/editor-core/tests/corpus/",
    "crates/editor-core/tests/fixture/",
];

use crate::corpus;
use crate::fixture;

use editor_core::{
    Attr, CapEnd, Datum, Dimension, DocEdit, DocParam, EditError, EntityKind, EvalOptions, Expr,
    LoggedEdit, LoopProgram, Maintenance, NameRef, Node, NodeErrorKind, NodeResult, ParamName,
    PersistError, PieceRole, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProgramStep,
    ProgramTarget, RecipeNodeId, ResolveError, Rgba8, RoleSeg, SlotId, StableName, StepArg, StepId,
    StepIdFault, apply, load, save,
};
use fixture::{ang, edge_of, ends, fname, insert, len, minted, point, scl, table, tol};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, rod_chord_at};

// ---------------------------------------------------------------- //
// The fixture: the sunk rod, and the leg the reshaping inserts
// ---------------------------------------------------------------- //

// The rod's loop, its bump and its ids are the corpus document's
// (`reshaped_rod`, the first persisted `SetProgram`): one spelling, so
// the rows here measure the program the corpus replays.
use crate::corpus::reshaped_rod::{
    BUMP, CREASE, CREASE_RESHAPED, bump_ids, lateral_edge, rod_ids, rod_loop,
};

/// The area the bump adds to the section — the triangle
/// `(1, −1), (1.25, −0.5), (1, 0)` — so the reshaped rod's volume is
/// the old one's plus this times the length.
const BUMP_AREA: f64 = 0.125;

/// A document holding the sunk rod, with `creases` (canonical profile
/// vertices) filleted when any are named.
struct Rod {
    doc: ProfileDoc,
    profile: RecipeNodeId,
    rod: RecipeNodeId,
    fillet: Option<RecipeNodeId>,
}

fn rod(label: &str, creases: &[usize]) -> Rod {
    let doc = ProfileDoc::empty_derived(label, tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![rod_loop(false)],
            ids: Vec::new(),
        }),
    );
    let (doc, rod) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(ROD_L),
        },
    );
    let (doc, fillet) = if creases.is_empty() {
        (doc, None)
    } else {
        let selection = creases
            .iter()
            .map(|&v| lateral_edge(&doc, rod, v))
            .collect();
        let (doc, f) = insert(doc, Node::fillet(rod, len(ROD_FILLET), selection));
        (doc, Some(f))
    };
    Rod {
        doc,
        profile,
        rod,
        fillet,
    }
}

/// The wall at canonical segment `k` of loop `l` of the profile `ext`
/// sweeps, spelled by the piece it is under `doc`'s current values.
fn wall_of(doc: &editor_core::ProfileDoc, ext: RecipeNodeId, l: usize, k: usize) -> StableName {
    fname(ext, RoleSeg::Lateral(fixture::piece(doc, ext, l, k)))
}

/// A wall spelled by a step id and a role directly.
fn wall_by(ext: RecipeNodeId, step: StepId, role: PieceRole) -> StableName {
    fname(ext, RoleSeg::Lateral(ProfileEdgeRef::Piece { step, role }))
}

/// The ids a profile node holds, per loop.
fn ids_of(doc: &editor_core::ProfileDoc, profile: RecipeNodeId) -> Vec<Vec<StepId>> {
    match doc.node(profile) {
        Some(Node::Profile(p)) => p.ids.clone(),
        other => panic!("node {} is a profile: {other:?}", profile.0),
    }
}

/// Every id of `doc`'s profile kept, as `SetProgram` spells it.
fn keep_all(doc: &editor_core::ProfileDoc, profile: RecipeNodeId) -> Vec<Vec<Option<StepId>>> {
    ids_of(doc, profile)
        .into_iter()
        .map(|l| l.into_iter().map(Some).collect())
        .collect()
}

fn set_program(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    loops: Vec<LoopProgram>,
    ids: Vec<Vec<Option<StepId>>>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(
        doc,
        &DocEdit::SetProgram { node, loops, ids },
        tol(),
        &editor_core::RefusingReach,
    )
}

fn accepted(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    loops: Vec<LoopProgram>,
    ids: Vec<Vec<Option<StepId>>>,
) -> editor_core::Applied<ProfileProgram> {
    set_program(doc, node, loops, ids).expect("the reshaping is accepted")
}

/// A blend node's selection as the document holds it.
fn selection_of(doc: &editor_core::ProfileDoc, node: RecipeNodeId) -> Vec<StableName> {
    match doc.node(node) {
        Some(Node::Fillet { selection, .. }) => selection.clone(),
        other => panic!("node {node:?} is a fillet, got {other:?}"),
    }
}

/// A derived frame carrying `face` — the payload carrier the rows
/// that need a face name on a node use.
fn frame_on(doc: ProfileDoc, at: RecipeNodeId, face: StableName) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at,
            face,
            spin: ang(0.0),
        }),
    )
}

/// The face a derived frame carries, as the document holds it.
fn frame_face(doc: &editor_core::ProfileDoc, frame: RecipeNodeId) -> StableName {
    match doc.node(frame) {
        Some(Node::Datum(Datum::FaceFrame { face, .. })) => face.clone(),
        other => panic!("a frame, got {other:?}"),
    }
}

fn paint(doc: &editor_core::ProfileDoc, name: &StableName) -> ProfileDoc {
    apply(
        doc,
        &DocEdit::SetAppearance {
            name: name.clone(),
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the painted name's node is live")
    .doc
}

fn volume(doc: &editor_core::ProfileDoc, node: RecipeNodeId) -> f64 {
    let ev = fixture::run(doc, &EvalOptions::default());
    let bad = corpus::failures(&ev);
    assert!(
        bad.is_empty(),
        "the document evaluates:\n{}",
        bad.join("\n")
    );
    topo::mass_properties(corpus::body_of(&ev, node), tol())
        .expect("closed-form mass properties")
        .volume
}

/// The `(x, y)` both ends of the strut edge `name` stand at, on the
/// evaluated extrude `rod` — a strut runs along `z`, so the two ends
/// share them, and they are the profile vertex the name denotes.
fn strut_at(doc: &editor_core::ProfileDoc, rod: RecipeNodeId, name: &StableName) -> (f64, f64) {
    let ev = fixture::run(doc, &EvalOptions::default());
    let body = corpus::body_of(&ev, rod);
    let [a, b] = ends(body, edge_of(table(&ev, rod), "strut", name));
    let (pa, pb) = (point(body, a), point(body, b));
    assert!(
        (pa.x - pb.x).abs() < 1e-12 && (pa.y - pb.y).abs() < 1e-12,
        "a strut edge runs along z: {pa:?} and {pb:?}"
    );
    (pa.x, pa.y)
}

fn near(got: (f64, f64), want: (f64, f64)) -> bool {
    (got.0 - want.0).abs() < 1e-12 && (got.1 - want.1).abs() < 1e-12
}

/// The sorted `(x, y, z)` corners of the face `name` denotes on the
/// evaluated `node` — what a name DENOTES, read off the solid rather
/// than off a spelling.
fn corners_of(
    doc: &editor_core::ProfileDoc,
    node: RecipeNodeId,
    name: &StableName,
) -> Vec<(f64, f64, f64)> {
    let ev = fixture::run(doc, &EvalOptions::default());
    let body = corpus::body_of(&ev, node);
    let mut out: Vec<(f64, f64, f64)> =
        fixture::face_vertices(body, fixture::face_of(table(&ev, node), "wall", name))
            .into_iter()
            .map(|v| {
                let p = point(body, v);
                (p.x, p.y, p.z)
            })
            .collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out
}

fn has_corner3(corners: &[(f64, f64, f64)], want: (f64, f64, f64)) -> bool {
    corners.iter().any(|c| {
        (c.0 - want.0).abs() < 1e-9 && (c.1 - want.1).abs() < 1e-9 && (c.2 - want.2).abs() < 1e-9
    })
}

/// The frame at `frame` refuses `Vanished` on exactly `name` at the
/// next evaluation — the node is live and its table has no such entry.
fn frame_refuses_vanished(doc: &editor_core::ProfileDoc, frame: RecipeNodeId, name: &StableName) {
    let ev = fixture::run(doc, &EvalOptions::default());
    match ev.nodes.get(&frame) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::FaceFrameResolve { error } => match error.as_ref() {
                ResolveError::Vanished { name: gone, .. } => assert_eq!(gone, name),
                other => panic!("the name resolves to nothing: got {other:?}"),
            },
            other => panic!("the frame refuses on its face, got {other:?}"),
        },
        other => panic!("the frame refuses rather than re-anchoring, got {other:?}"),
    }
}

// ---------------------------------------------------------------- //
// A reshaping that keeps the step
// ---------------------------------------------------------------- //

/// **A fillet on a crease survives a leg inserted before it, untouched
/// and unreported.** The crease at canonical vertex 4 (the arc's end,
/// the rod's left crease — where the leg to `(−1, 0)` starts) is
/// filleted; a bump is inserted between the corners at `(1, −1)` and
/// `(1, 0)`, two new steps drawing one segment, and every old step is
/// kept. The crease's name spells the leg's step, so the accepted
/// edit reports nothing, the fillet's selection is the one it was
/// authored with, and the evaluated fillet is on the SAME crease: the
/// strut stands at the arc's end `(−xv, 0)`, the fillet's own end arcs
/// compose on the name, and the solid's volume is the old one's plus
/// exactly the bump's prism — the crease now being canonical vertex 5,
/// a position no name spells.
#[test]
fn a_fillet_on_a_crease_survives_a_leg_inserted_before_it() {
    let r = rod("set-program-finding", &[CREASE]);
    let fillet = r.fillet.unwrap();
    let crease = lateral_edge(&r.doc, r.rod, CREASE);
    let before = volume(&r.doc, fillet);
    let xv = rod_chord_at(ROD_FLAT).half;
    assert!(
        near(strut_at(&r.doc, r.rod, &crease), (-xv, 0.0)),
        "the fixture's crease is the arc's end"
    );

    let ids = bump_ids(&rod_ids(&r.doc, r.profile));
    let applied = accepted(&r.doc, r.profile, vec![rod_loop(true)], ids);
    assert_eq!(
        applied.maintenance,
        Vec::new(),
        "a reshaping that keeps the step has nothing to report"
    );
    assert!(
        applied.record.structural,
        "a reshaped program is structural"
    );
    assert_eq!(
        selection_of(&applied.doc, fillet),
        vec![crease.clone()],
        "the fillet's selection is untouched"
    );
    assert_eq!(
        crease,
        lateral_edge(&applied.doc, r.rod, CREASE_RESHAPED),
        "the crease's canonical vertex moved one along; its name did not"
    );

    // Measured on the solid: the strut stands where it did, the
    // fillet composes on it, and the volume moved by the bump alone.
    assert!(
        near(strut_at(&applied.doc, r.rod, &crease), (-xv, 0.0)),
        "the name denotes the same crease"
    );
    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    let t = table(&ev, fillet);
    let vertex = fixture::vpiece(&applied.doc, r.rod, 0, CREASE_RESHAPED);
    for end in [CapEnd::Start, CapEnd::End] {
        let arc = minted(
            EntityKind::Edge,
            fillet,
            RoleSeg::EndArc {
                vertex: NameRef::new(fixture::cap_vertex(r.rod, end, vertex)),
                edge: NameRef::new(crease.clone()),
            },
        );
        let _ = edge_of(t, "the cut-off arc at the crease", &arc);
    }
    let after = volume(&applied.doc, fillet);
    let want = before + BUMP_AREA * ROD_L;
    assert!(
        (after - want).abs() <= 1e-9 * want,
        "the volume moved by the bump's prism alone: {after} vs {want}"
    );
}

/// **A vertex is named by the piece that STARTS at it.** Two struts on
/// the block's right wall: canonical vertex 1, `(1, −1)`, where the leg
/// to `(1, 0)` starts, and canonical vertex 2, `(1, 0)`, where the leg
/// to `(xv, 0)` starts. The bump is inserted between them and every
/// old step kept: the leg to `(1, 0)` now starts at the bump's point,
/// so its vertex name stands there — the piece moved, and its start
/// with it — while the leg to `(xv, 0)` still starts at `(1, 0)`.
/// Measured on the solid, both before and after, and nothing is
/// reported: no piece was removed.
#[test]
fn a_vertex_is_named_by_the_piece_starting_at_it() {
    let r = rod("set-program-vertex", &[]);
    let one = lateral_edge(&r.doc, r.rod, 1);
    let two = lateral_edge(&r.doc, r.rod, 2);
    assert!(near(strut_at(&r.doc, r.rod, &one), (1.0, -1.0)));
    assert!(near(strut_at(&r.doc, r.rod, &two), (1.0, 0.0)));

    let ids = bump_ids(&rod_ids(&r.doc, r.profile));
    let applied = accepted(&r.doc, r.profile, vec![rod_loop(true)], ids);
    assert_eq!(applied.maintenance, Vec::new());
    assert!(near(strut_at(&applied.doc, r.rod, &one), BUMP));
    assert!(near(strut_at(&applied.doc, r.rod, &two), (1.0, 0.0)));
    assert!(
        near(
            strut_at(&applied.doc, r.rod, &lateral_edge(&applied.doc, r.rod, 1)),
            (1.0, -1.0)
        ),
        "the corner `(1, −1)` is where the new leg starts, and is its piece's now"
    );
}

/// **A leg inserted into a square keeps every other wall's name.** The
/// square `(0,0)–(2,2)` gets a leg to `(3, 1)` between `(2, 0)` and
/// `(2, 2)`; a paint on each of the four walls is held across the edit.
/// Nothing is reported, and every paint's name denotes the wall its
/// step draws now: the leg to `(2, 2)` — shortened to start at
/// `(3, 1)` — keeps its name, and the new leg `(2, 0) → (3, 1)` answers
/// to none of them.
#[test]
fn a_leg_inserted_into_a_square_keeps_every_walls_name() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let (doc, profile, ext) = extruded("set-program-square-leg", vec![square]);
    let walls: Vec<StableName> = (0..4).map(|k| wall_of(&doc, ext, 0, k)).collect();
    let mut painted = doc.clone();
    for w in &walls {
        painted = paint(&painted, w);
    }
    let mut ids = keep_all(&painted, profile);
    ids[0].insert(2, None);
    let leg =
        LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let applied = accepted(&painted, profile, vec![leg], ids);
    assert_eq!(applied.maintenance, Vec::new());
    let right = corners_of(&applied.doc, ext, &walls[1]);
    assert!(
        has_corner3(&right, (3.0, 1.0, 0.0)) && has_corner3(&right, (2.0, 2.0, 0.0)),
        "the leg to (2, 2) keeps its name and starts at (3, 1) now: {right:?}"
    );
    for (k, w) in walls.iter().enumerate() {
        assert!(
            !has_corner3(&corners_of(&applied.doc, ext, w), (2.0, 0.0, 0.0))
                || !has_corner3(&corners_of(&applied.doc, ext, w), (3.0, 1.0, 0.0)),
            "wall {k}'s name does not answer to the new leg"
        );
    }
    let top = corners_of(&applied.doc, ext, &walls[2]);
    assert!(
        has_corner3(&top, (2.0, 2.0, 0.0)) && has_corner3(&top, (0.0, 2.0, 0.0)),
        "the top keeps its name: {top:?}"
    );
}

/// **The identity edit is accepted, reports nothing, and is
/// structural.** The program the node holds, every step kept.
#[test]
fn the_identity_edit_is_accepted_and_reports_nothing() {
    let r = rod("set-program-identity", &[CREASE]);
    let ids = keep_all(&r.doc, r.profile);
    let applied = accepted(&r.doc, r.profile, vec![rod_loop(false)], ids);
    assert!(applied.maintenance.is_empty());
    assert!(applied.record.structural);
    assert!(applied.doc.bit_eq(&r.doc), "nothing moved");
}

// ---------------------------------------------------------------- //
// A reshaping that drops the step
// ---------------------------------------------------------------- //

/// **A step the reshaping drops strands every name on its pieces, and
/// the name never aliases.** The crease's step — the leg to `(−1, 0)`
/// — is stated as new: the new program still draws that leg, with a
/// freshly minted id, so the old id is gone for good. The fillet's name
/// keeps its spelling, is reported `Strand`, and at the next
/// evaluation the fillet refuses `Vanished` on exactly that name (rung
/// 3 of the N5 ladder), rather than carving whatever the new id's leg
/// is.
#[test]
fn a_dropped_step_strands_the_names_on_its_pieces_and_they_never_alias() {
    let r = rod("set-program-dropped", &[CREASE]);
    let fillet = r.fillet.unwrap();
    let crease = lateral_edge(&r.doc, r.rod, CREASE);
    let old = rod_ids(&r.doc, r.profile);
    let mut ids = bump_ids(&old);
    // The leg to (−1, 0) is step 5 of the plain program, 7 of the bumped.
    assert_eq!(ids[0][7], Some(old[5]));
    ids[0][7] = None;
    let applied = accepted(&r.doc, r.profile, vec![rod_loop(true)], ids);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Strand {
            node: fillet,
            name: crease.clone(),
        }],
        "the crease's name strands, spelled as it was"
    );
    assert_eq!(selection_of(&applied.doc, fillet), vec![crease.clone()]);
    assert_ne!(
        lateral_edge(&applied.doc, r.rod, CREASE_RESHAPED),
        crease,
        "the leg is drawn again, under an id never minted before"
    );

    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    match ev.nodes.get(&fillet) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::BlendSelectionResolve { error, .. } => match error.as_ref() {
                ResolveError::Vanished { name, .. } => assert_eq!(name, &crease),
                other => panic!("the stranded name resolves to nothing: got {other:?}"),
            },
            other => panic!("the fillet refuses on its selection, got {other:?}"),
        },
        other => panic!("the fillet refuses, got {other:?}"),
    }
}

/// **A name may spell a step a `SetProgram` dropped, never one the
/// document has not minted.** The dropped id was minted, so a frame on
/// it inserts (and resolves to nothing, as the stranded fillet does
/// above); an id at the step counter would be minted for the next new
/// step, and a name written on it before then would come to denote that
/// step — the insert, rebind and appearance doors refuse it typed, as
/// the load door does.
#[test]
fn a_name_on_a_dropped_step_inserts_and_one_on_a_never_minted_step_refuses() {
    let r = rod("set-program-never-minted", &[]);
    let old = rod_ids(&r.doc, r.profile);
    let mut ids = bump_ids(&old);
    ids[0][7] = None;
    let reshaped = accepted(&r.doc, r.profile, vec![rod_loop(true)], ids).doc;
    let dropped = wall_by(r.rod, old[5], PieceRole::Leg);
    let (_, frame) = frame_on(reshaped.clone(), r.rod, dropped.clone());
    assert!(frame.0 > 0, "a name on a dropped step inserts");

    let next = StepId(reshaped.next_step());
    let unminted = wall_by(r.rod, next, PieceRole::Leg);
    let never = |edit: DocEdit<ProfileProgram>| match apply(
        &reshaped,
        &edit,
        tol(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::NameStepNeverMinted {
            name,
            step,
            next_step,
        }) => {
            assert_eq!((name, step, next_step), (unminted.clone(), next, next.0));
        }
        other => panic!("a never-minted step refuses typed, got {other:?}"),
    };
    never(DocEdit::InsertNode {
        node: Node::Datum(editor_core::Datum::FaceFrame {
            at: r.rod,
            face: unminted.clone(),
            spin: fixture::ang(0.0),
        }),
    });
    let painted = paint(&reshaped, &dropped);
    never(DocEdit::SetAppearance {
        name: unminted.clone(),
        attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
    });
    assert!(
        apply(
            &painted,
            &DocEdit::Rebind {
                from: dropped.clone(),
                to: unminted.clone(),
            },
            tol(),
            &editor_core::RefusingReach,
        )
        .is_err_and(|e| matches!(e, EditError::NameStepNeverMinted { .. })),
        "a rebind onto a never-minted step refuses typed"
    );
}

/// **A dropped step's names strand in every carrier, in the contract's
/// order**: the payload strands first, in document order, then the
/// store's keys. A frame and a paint on one dropped wall.
#[test]
fn a_reshaping_reports_its_strands_then_its_stranded_keys() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let (doc, profile, ext) = extruded("set-program-order", vec![square.clone()]);
    let right = wall_of(&doc, ext, 0, 1);
    let (doc, frame) = frame_on(doc, ext, right.clone());
    let doc = paint(&doc, &right);
    let mut ids = keep_all(&doc, profile);
    ids[0][2] = None;
    let applied = accepted(&doc, profile, vec![square], ids);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Strand {
                node: frame,
                name: right.clone(),
            },
            Maintenance::StrandedAppearance { name: right },
        ]
    );
}

// ---------------------------------------------------------------- //
// The ids' shape, and the doors around them
// ---------------------------------------------------------------- //

/// **Every way a `SetProgram`'s ids can be wrong refuses typed before
/// the program is checked**, and leaves the document as it was: one
/// list too few, a list one id short, an id of another profile, and
/// one id kept twice.
#[test]
fn every_step_id_fault_refuses_typed_before_the_program_is_checked() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let (doc, profile, _) = extruded("set-program-faults", vec![square.clone()]);
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, other) = insert(
        doc,
        Node::Profile(fixture::desc(plane, vec![fixture::square(5.0, 5.0, 1.0)])),
    );
    let ids = keep_all(&doc, profile);
    let foreign = ids_of(&doc, other)[0][0];
    let refused = |ids: Vec<Vec<Option<StepId>>>| {
        let err =
            set_program(&doc, profile, vec![square.clone()], ids).expect_err("the ids refuse");
        match err {
            EditError::StepIdsRefused { node, fault } => {
                assert_eq!(node, profile);
                fault
            }
            other => panic!("a step-id refusal, got {other:?}"),
        }
    };
    assert_eq!(
        refused(Vec::new()),
        StepIdFault::LoopCount { loops: 1, given: 0 }
    );
    let mut short = ids.clone();
    short[0].pop();
    assert_eq!(
        refused(short),
        StepIdFault::Shape {
            loop_: 0,
            authored: 5,
            given: 4
        }
    );
    let mut stolen = ids.clone();
    stolen[0][1] = Some(foreign);
    assert_eq!(
        refused(stolen),
        StepIdFault::NotThisProfiles { step: foreign }
    );
    let mut twice = ids.clone();
    twice[0][2] = twice[0][1];
    assert_eq!(
        refused(twice),
        StepIdFault::Repeated {
            step: ids[0][1].unwrap()
        }
    );
}

/// **A program entering the document carries no ids: the insert door
/// mints them, and refuses a program that brings its own.** The minted
/// ids run one per authored step, in loop then step order, from the
/// document's step counter — a second profile's continue where the
/// first's stopped.
#[test]
fn the_insert_door_mints_every_step_and_refuses_ids_of_the_callers() {
    let doc = ProfileDoc::empty_derived("set-program-mint", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let square = fixture::desc(plane, vec![fixture::square(0.0, 0.0, 1.0)]);
    let (doc, first) = insert(doc, Node::Profile(square.clone()));
    let (doc, second) = insert(doc, Node::Profile(square.clone()));
    assert_eq!(
        ids_of(&doc, first),
        vec![(0..5).map(StepId).collect::<Vec<_>>()]
    );
    assert_eq!(
        ids_of(&doc, second),
        vec![(5..10).map(StepId).collect::<Vec<_>>()]
    );
    let mut preminted = square;
    preminted.ids = ids_of(&doc, first);
    let err = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(preminted),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect_err("ids are the door's to mint");
    assert!(
        matches!(
            err,
            EditError::StepIdsRefused {
                fault: StepIdFault::Preminted,
                ..
            }
        ),
        "{err:?}"
    );
}

/// **`SetProgram` aimed at what holds no program refuses typed.**
#[test]
fn a_node_that_holds_no_program_refuses() {
    let r = rod("set-program-non-profile", &[]);
    let ids = keep_all(&r.doc, r.profile);
    assert_eq!(
        set_program(&r.doc, r.rod, vec![rod_loop(false)], ids.clone()).err(),
        Some(EditError::SetProgramOnNonProfile { node: r.rod })
    );
    assert_eq!(
        set_program(&r.doc, RecipeNodeId(99), vec![rod_loop(false)], ids).err(),
        Some(EditError::UnknownNode {
            id: RecipeNodeId(99)
        })
    );
}

/// **A new program naming an undeclared parameter refuses the slot
/// door's own arm** — the same `SlotUnknownDocParam`, at the same
/// address, that `SetParam` refuses for the same expression written
/// into the same slot. One function, not a mirror.
#[test]
fn a_program_naming_an_undeclared_parameter_refuses_the_slot_doors_own_arm() {
    let (doc, profile, _) = extruded(
        "set-program-param-refs",
        vec![LoopProgram::Chain(square_steps())],
    );
    let nope = Expr::param(ParamName::new("nope"), Dimension::Length);
    let mut steps = square_steps();
    steps[1] = ProgramStep::LineTo(ProgramTarget::Point([nope.clone(), len(0.0)]));
    let slot = SlotId::Profile {
        loop_: 0,
        step: 1,
        arg: StepArg::TargetX,
    };
    let through_the_slot = apply(
        &doc,
        &DocEdit::SetParam {
            node: profile,
            slot,
            expr: nope,
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect_err("an undeclared parameter refuses at the slot door");
    let ids = keep_all(&doc, profile);
    let through_the_program = set_program(&doc, profile, vec![LoopProgram::Chain(steps)], ids)
        .expect_err("an undeclared parameter refuses at the program door");
    assert!(
        matches!(&through_the_slot, EditError::SlotUnknownDocParam { .. }),
        "{through_the_slot:?}"
    );
    assert_eq!(through_the_program, through_the_slot);
}

/// A unit square, as a chain the rows below reshape.
fn square_steps() -> Vec<ProgramStep> {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]
}

/// A document holding `loops` extruded; `(doc, profile, extrude)`.
fn extruded(label: &str, loops: Vec<LoopProgram>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived(label, tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops,
            ids: Vec::new(),
        }),
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

// ---------------------------------------------------------------- //
// Persistence and replay identity
// ---------------------------------------------------------------- //

/// The rod document as a LOG: the empty document plus every edit that
/// built it, the reshaping last.
fn rod_log() -> (ProfileDoc, Vec<LoggedEdit<ProfileProgram>>) {
    let empty = ProfileDoc::empty_derived("set-program-log", tol());
    let r = rod("set-program-log", &[CREASE]);
    let profile_node = RecipeNodeId(1);
    let rod_node = RecipeNodeId(2);
    assert_eq!((r.profile, r.rod), (profile_node, rod_node));
    let edits = vec![
        DocEdit::InsertNode {
            node: fixture::xy_frame(),
        },
        DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: RecipeNodeId(0),
                loops: vec![rod_loop(false)],
                ids: Vec::new(),
            }),
        },
        DocEdit::InsertNode {
            node: Node::Extrude {
                profile: profile_node,
                distance: len(ROD_L),
            },
        },
        DocEdit::InsertNode {
            node: Node::fillet(
                rod_node,
                len(ROD_FILLET),
                vec![lateral_edge(&r.doc, rod_node, CREASE)],
            ),
        },
        DocEdit::SetProgram {
            node: profile_node,
            loops: vec![rod_loop(true)],
            ids: bump_ids(&rod_ids(&r.doc, profile_node)),
        },
    ];
    (empty, LoggedEdit::bare_all(&edits))
}

/// **A document whose log holds a `SetProgram` saves, loads and
/// replays identically** — the edit rides the unversioned format like
/// every other, its minting is a function of the log, and the loaded
/// document is bit for bit the applied one.
#[test]
fn a_log_holding_a_set_program_saves_loads_and_replays_identically() {
    let (empty, log) = rod_log();
    let mut applied = empty.clone();
    for entry in &log {
        applied = editor_core::apply_logged(&applied, entry, tol())
            .expect("the log applies")
            .doc;
    }
    let text = save(&empty, &log, tol()).expect("saves");
    let loaded = load(&text, tol()).expect("loads");
    assert_eq!(loaded.edits, log, "the log round-trips");
    assert!(
        loaded.doc.bit_eq(&applied),
        "the replayed document is the applied one"
    );
    let replayed = ProfileDoc::replay(empty.id(), &log, tol()).expect("replays");
    assert!(
        replayed.bit_eq(&applied),
        "replay is a function of the log alone"
    );
    assert_eq!(
        save(&loaded.snapshot, &loaded.edits, tol()).expect("re-saves"),
        text,
        "the bytes are canonical"
    );
}

/// **The persisted spelling of the edit** — externally tagged, the
/// loops in their own wire form, the ids as plain integers with `null`
/// for a step the door mints — pinned as bytes, since this is the
/// format's compatibility contract. And a file written before profile
/// pieces were named by step ids — a program with no `ids`, or a name
/// spelled `{"loop_index", "segment"}` — refuses `Unreadable` with the
/// regenerate recourse rather than loading into a numbering nothing
/// reads any more.
#[test]
fn the_persisted_spelling_is_pinned_and_an_old_file_refuses_typed() {
    let edit: DocEdit<ProfileProgram> = DocEdit::SetProgram {
        node: RecipeNodeId(1),
        loops: vec![LoopProgram::circle(0.0, 0.0, 1.0).unwrap()],
        ids: vec![vec![None]],
    };
    let wire = serde_json::to_string(&LoggedEdit::bare(edit)).expect("serializes");
    assert_eq!(
        wire,
        r#"{"edit":{"SetProgram":{"node":1,"loops":[{"Circle":{"centre":[{"Literal":{"value":0.0,"dim":"Length","unit":"m"}},{"Literal":{"value":0.0,"dim":"Length","unit":"m"}}],"radius":{"Literal":{"value":1.0,"dim":"Length","unit":"m"}}}}],"ids":[[null]]}},"maintenance":[]}"#
    );

    let r = rod("set-program-old-file", &[CREASE]);
    let text = save(&r.doc, &[], tol()).expect("saves");
    let unreadable = |text: &str, what: &str| match load(text, tol()) {
        Err(e @ PersistError::Unreadable { .. }) => {
            let PersistError::Unreadable { detail, .. } = &e else {
                unreachable!()
            };
            assert!(detail.contains(what), "{detail}");
            assert!(
                e.to_string().contains(editor_core::REGENERATE_RECOURSE),
                "{e}"
            );
        }
        other => panic!("an old file refuses Unreadable, got {other:?}"),
    };
    // No ids on the program.
    let (header, body) = text.split_once('\n').expect("an id line, then the body");
    let mut v: serde_json::Value = serde_json::from_str(body).expect("the body is JSON");
    let program = v["snapshot"]["nodes"][r.profile.0.to_string()]["Profile"]
        .as_object_mut()
        .expect("the profile node");
    assert!(
        program.remove("ids").is_some(),
        "the program carries its ids"
    );
    let no_ids = format!("{header}\n{v}\n");
    unreadable(&no_ids, "ids");
    // A positional locator in a name, written compact so the piece is
    // one run of bytes.
    let compact: serde_json::Value = serde_json::from_str(body).expect("the body is JSON");
    let compact = compact.to_string();
    let piece = serde_json::to_value(fixture::vpiece(&r.doc, r.rod, 0, CREASE))
        .unwrap()
        .to_string();
    assert!(
        compact.contains(&piece),
        "the fillet's selection spells the piece"
    );
    let positional = format!(
        "{header}\n{}\n",
        compact.replace(&piece, r#"{"loop_index":0,"vertex":4}"#)
    );
    unreadable(&positional, "loop_index");
}

// ---------------------------------------------------------------- //
// What cannot move a name: value edits (N1)
// ---------------------------------------------------------------- //

/// `At, Toward(+x), Fillet(r), Toward(+y), FarEndTo(2,2), LineTo(0,2),
/// LineTo(Start)` — the corner-fillet chain whose segment count is a
/// function of its RADIUS. Measured: at `r = 2` both runs of the
/// fillet have a `Zero` fit and emit nothing, so the loop draws THREE
/// segments; at `r = 0.3` they emit and it draws FIVE.
fn filleted_square(r: f64) -> LoopProgram {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Fillet(len(r)),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::FarEndTo(pt(2.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The corner fillet's radius written through `SetParam`.
fn set_radius(
    doc: &ProfileDoc,
    profile: RecipeNodeId,
    r: f64,
) -> editor_core::Applied<ProfileProgram> {
    apply(
        doc,
        &DocEdit::SetParam {
            node: profile,
            slot: SlotId::Profile {
                loop_: 0,
                step: 2,
                arg: StepArg::Radius,
            },
            expr: len(r),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the radius is a legal slot write")
}

/// **A slot edit through a `Zero` fit keeps a live name, and reports
/// nothing.** At `r = 2` the loop draws three segments and canonical
/// wall 2 is the left edge `(0, 2) → (0, 0)`, the close's leg; the
/// radius goes to `0.3` and the loop draws five. The name held on the
/// left edge spells the close's step, so it is the left edge still —
/// where a positional name would have become the right edge — and the
/// slot edit has nothing to report.
#[test]
fn a_slot_edit_through_a_zero_fit_keeps_a_live_name_and_reports_nothing() {
    let (doc, profile, ext) = extruded("set-param-zero-fit", vec![filleted_square(2.0)]);
    assert_eq!(fixture::pieces(&doc, profile).edges[0].len(), 3);
    let left = wall_of(&doc, ext, 0, 2);
    let (doc, frame) = frame_on(doc, ext, left.clone());
    let before = corners_of(&doc, ext, &left);
    assert!(
        has_corner3(&before, (0.0, 2.0, 0.0)) && has_corner3(&before, (0.0, 0.0, 0.0)),
        "the held wall is the left edge at r = 2: {before:?}"
    );
    let grown = set_radius(&doc, profile, 0.3);
    assert_eq!(grown.maintenance, vec![], "the slot edit reports nothing");
    assert_eq!(fixture::pieces(&grown.doc, profile).edges[0].len(), 5);
    let ev = fixture::run(&grown.doc, &EvalOptions::default());
    assert!(
        ev.value(frame).is_some(),
        "the frame still evaluates — {:?}",
        corpus::failures(&ev)
    );
    let after = corners_of(&grown.doc, ext, &left);
    assert!(
        has_corner3(&after, (0.0, 2.0, 0.0)) && has_corner3(&after, (0.0, 0.0, 0.0)),
        "the held name is the left edge still: {after:?}"
    );
}

/// The square `(0,0)–(2,2)` with a hole circle about `(1, 1)` whose
/// radius is the document parameter `hole_r`, the square described
/// first; `(doc, profile, extrude)`.
fn square_and_driven_hole(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = declared(label, "hole_r", 0.3);
    extrude_of(
        doc,
        vec![
            LoopProgram::Chain(square_steps()),
            LoopProgram::Circle {
                centre: [len(1.0), len(1.0)],
                radius: param_len("hole_r"),
            },
        ],
    )
}

/// A document parameter's value written through the value door.
fn set_value(
    doc: &editor_core::ProfileDoc,
    name: &str,
    v: f64,
) -> editor_core::Applied<ProfileProgram> {
    apply(
        doc,
        &DocEdit::SetDocParamValue {
            name: ParamName::new(name),
            value: editor_core::DocParamValue::Continuous(v),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the value lands")
}

fn declared(label: &str, name: &str, v: f64) -> ProfileDoc {
    let (doc, _) = fixture::step(
        ProfileDoc::empty_derived(label, tol()),
        DocEdit::SetDocParam {
            name: ParamName::new(name),
            value: DocParam::continuous(Dimension::Length, v),
        },
    );
    doc
}

fn param_len(name: &str) -> Expr {
    Expr::param(ParamName::new(name), Dimension::Length)
}

/// A profile of `loops` extruded, in `doc`; `(doc, profile, extrude)`.
fn extrude_of(
    doc: ProfileDoc,
    loops: Vec<LoopProgram>,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops,
            ids: Vec::new(),
        }),
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

/// **A value edit moves no name** — the hole grown `0.3 → 0.4` keeps
/// every role and sense, and nothing is reported.
#[test]
fn a_value_edit_moves_no_name() {
    let (doc, _, ext) = square_and_driven_hole("value-keep-hole");
    let top = wall_of(&doc, ext, 0, 2);
    let (doc, frame) = frame_on(doc, ext, top.clone());
    let applied = set_value(&doc, "hole_r", 0.4);
    assert_eq!(applied.maintenance, Vec::new());
    assert_eq!(frame_face(&applied.doc, frame), top);
    assert_eq!(wall_of(&applied.doc, ext, 0, 2), top);
}

/// **A hole that becomes the outer loop moves no name.** `hole_r`
/// `0.3 → 1.5`: the circle now encloses the square (its corners are √2
/// from the centre), so the profile still validates with the ROLES
/// swapped — the circle is canonical loop 0 and the square canonical
/// loop 1, reversed. Nothing is reported, the square's side
/// `(2,2)→(0,2)` (framed) and the circle's first half (painted) keep
/// their names, and each name denotes the wall it denoted before.
#[test]
fn an_outer_and_hole_swap_moves_no_name() {
    let (doc, _, ext) = square_and_driven_hole("value-jump-hole");
    let side = wall_of(&doc, ext, 0, 2);
    let half = wall_of(&doc, ext, 1, 0);
    let (doc, _) = frame_on(doc, ext, side.clone());
    let doc = paint(&doc, &half);
    let applied = set_value(&doc, "hole_r", 1.5);
    assert_eq!(applied.maintenance, Vec::new(), "nothing is reported");
    let (before, after) = (
        fixture::pieces(&doc, fixture::swept(&doc, ext)),
        fixture::pieces(&applied.doc, fixture::swept(&applied.doc, ext)),
    );
    let set = |l: &[ProfileEdgeRef]| l.iter().copied().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        (set(&after.edges[0]), set(&after.edges[1])),
        (set(&before.edges[1]), set(&before.edges[0])),
        "canonical loop 0 is the circle now and the square is loop 1"
    );
    let now = corners_of(&applied.doc, ext, &side);
    assert!(
        has_corner3(&now, (2.0, 2.0, 0.0)) && has_corner3(&now, (0.0, 2.0, 0.0)),
        "the side's name is still the side (2,2)→(0,2): {now:?}"
    );
    let arc = corners_of(&applied.doc, ext, &half);
    assert!(
        arc.iter()
            .all(|c| ((c.0 - 1.0).hypot(c.1 - 1.0) - 1.5).abs() < 1e-9),
        "the circle's half is on the circle: {arc:?}"
    );
}

/// **A loop whose sense flips moves no name.** A triangle
/// `(0,0) → (2,0) → (1, 1)`, counterclockwise as authored; `SetParam`
/// moves the apex to `(1, −1)` and the same three steps now wind
/// clockwise, so canonical wall `k` is program segment `2 − k`. The
/// base's name spells its step and denotes the base still.
#[test]
fn a_sense_flip_moves_no_name() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let triangle = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, profile, ext) = extruded("value-flip-sense", vec![triangle]);
    let base = wall_of(&doc, ext, 0, 0);
    let (doc, _) = frame_on(doc, ext, base.clone());
    let applied = apply(
        &doc,
        &DocEdit::SetParam {
            node: profile,
            slot: SlotId::Profile {
                loop_: 0,
                step: 2,
                arg: StepArg::TargetY,
            },
            expr: len(-1.0),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the apex moves");
    assert_eq!(applied.maintenance, Vec::new());
    assert_eq!(
        wall_of(&applied.doc, ext, 0, 2),
        base,
        "canonically reversed"
    );
    let now = corners_of(&applied.doc, ext, &base);
    assert!(
        has_corner3(&now, (0.0, 0.0, 0.0)) && has_corner3(&now, (2.0, 0.0, 0.0)),
        "the base's name is still the base (0,0)→(2,0): {now:?}"
    );
}

/// **A document parameter passing through a state that does not
/// replay moves no name.** `hole_r` `0.3 → 0.0 → 0.3`: at zero the
/// profile does not replay and the extrude cannot evaluate, but nothing
/// is reported at either edit, the paint on the hole's half keeps its
/// spelling, and back at 0.3 it denotes that half again.
#[test]
fn a_parameter_through_a_state_that_does_not_replay_moves_no_name() {
    let (doc, _, ext) = square_and_driven_hole("value-through-nonreplay");
    let half = wall_of(&doc, ext, 1, 0);
    let doc = paint(&doc, &half);
    let mid = set_value(&doc, "hole_r", 0.0);
    assert_eq!(mid.maintenance, Vec::new());
    let ev = fixture::run(&mid.doc, &EvalOptions::default());
    assert!(
        ev.value(ext).is_none(),
        "the parked profile does not evaluate"
    );
    let end = set_value(&mid.doc, "hole_r", 0.3);
    assert_eq!(end.maintenance, Vec::new());
    assert!(end.doc.appearance().contains_key(&half));
    assert!(
        end.doc.bit_eq(&doc),
        "the round trip is the document it left"
    );
    let ev = fixture::run(&end.doc, &EvalOptions::default());
    let table = &ev.value(ext).expect("the extrude evaluates").name_table;
    assert!(
        table.lookup(&half).is_some(),
        "the hole's half is named again"
    );
}

// ---------------------------------------------------------------- //
// Undrawn pieces vanish rather than alias (N1)
// ---------------------------------------------------------------- //

/// **A piece a `Zero` fit suppresses vanishes, and comes back.** At
/// `r = 0.3` the right edge `(2, 0.3) → (2, 2)` is the fillet's run
/// out, `{fillet, RunOut}`; at `r = 2` the run has no length, the arc
/// ends at `(2, 2)`, and a frame on the run refuses `Vanished` —
/// nothing else answers to its name. Back at `r = 0.3` the run is
/// drawn again and the frame on it evaluates. The far end's own leg
/// is the same segment as the run and never denotes anything.
#[test]
fn a_zero_fit_piece_vanishes_and_comes_back() {
    let (doc, profile, ext) = extruded("vanish-zero-fit", vec![filleted_square(0.3)]);
    let ids = ids_of(&doc, profile);
    let run_out = wall_by(ext, ids[0][2], PieceRole::RunOut);
    assert_eq!(
        run_out,
        wall_of(&doc, ext, 0, 2),
        "the run out is canonical wall 2"
    );
    let right = corners_of(&doc, ext, &run_out);
    assert!(
        has_corner3(&right, (2.0, 0.3, 0.0)) && has_corner3(&right, (2.0, 2.0, 0.0)),
        "{right:?}"
    );
    let far_end = wall_by(ext, ids[0][4], PieceRole::Leg);
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(
        table(&ev, ext).lookup(&far_end).is_none(),
        "the far end's leg is the run out's segment, which answers to the run out"
    );
    let (doc, frame) = frame_on(doc, ext, run_out.clone());
    let tight = set_radius(&doc, profile, 2.0);
    assert_eq!(tight.maintenance, Vec::new());
    frame_refuses_vanished(&tight.doc, frame, &run_out);
    let back = set_radius(&tight.doc, profile, 0.3);
    let ev = fixture::run(&back.doc, &EvalOptions::default());
    assert!(ev.value(frame).is_some(), "{:?}", corpus::failures(&ev));
}

/// **Two pieces drawn as one segment answer to the earlier one.** A
/// `line` leg continued by a fillet's run: the emitter extends the leg
/// instead of drawing the run on its own, so the segment is the leg's
/// piece and the fillet's run in denotes nothing.
#[test]
fn two_pieces_drawn_as_one_segment_answer_to_the_earlier() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Line(len(1.0)),
        ProgramStep::Fillet(len(0.3)),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::FarEndTo(pt(2.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, profile, ext) = extruded("vanish-one-segment", vec![chain]);
    let ids = ids_of(&doc, profile);
    let leg = wall_by(ext, ids[0][2], PieceRole::Leg);
    let run_in = wall_by(ext, ids[0][3], PieceRole::RunIn);
    assert_eq!(leg, wall_of(&doc, ext, 0, 0));
    let bottom = corners_of(&doc, ext, &leg);
    assert!(
        has_corner3(&bottom, (0.0, 0.0, 0.0)) && has_corner3(&bottom, (1.7, 0.0, 0.0)),
        "the leg is the whole bottom run to the fillet: {bottom:?}"
    );
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(
        table(&ev, ext).lookup(&run_in).is_none(),
        "the fillet's run in answers to nothing"
    );
}

// ---------------------------------------------------------------- //
// Every sweep of a profile names by its pieces
// ---------------------------------------------------------------- //

/// **Two sweeps of one profile name a wall by the same piece**, each
/// under its own node, and both keep it across a jump that swaps the
/// outer loop.
#[test]
fn both_sweeps_of_a_profile_name_by_its_pieces() {
    let doc = declared("value-two-sweeps", "p", 0.15);
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let radius = Expr::mul(param_len("p"), scl(2.0)).unwrap();
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::Chain(square_steps()),
                LoopProgram::Circle {
                    centre: [len(1.0), len(1.0)],
                    radius,
                },
            ],
            ids: Vec::new(),
        }),
    );
    let (doc, a) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0),
        },
    );
    let piece = fixture::piece(&doc, profile, 0, 2);
    assert_eq!(wall_of(&doc, a, 0, 2), fname(a, RoleSeg::Lateral(piece)));
    assert_eq!(wall_of(&doc, b, 0, 2), fname(b, RoleSeg::Lateral(piece)));
    let applied = set_value(&doc, "p", 0.75);
    assert_eq!(applied.maintenance, Vec::new());
    for ext in [a, b] {
        let side = corners_of(&applied.doc, ext, &fname(ext, RoleSeg::Lateral(piece)));
        assert!(
            has_corner3(&side, (2.0, 2.0, 0.0)) && has_corner3(&side, (0.0, 2.0, 0.0)),
            "{side:?}"
        );
    }
}

/// Two squares on parallel frames, lofted; `(doc, sec0, sec1, loft)`.
fn lofted(label: &str, lower: LoopProgram, upper: LoopProgram) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived(label, tol());
    let (doc, p0) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, sec0) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: p0,
            loops: vec![lower],
            ids: Vec::new(),
        }),
    );
    let (doc, p1) = insert(
        doc,
        fixture::frame([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, sec1) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: p1,
            loops: vec![upper],
            ids: Vec::new(),
        }),
    );
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![sec0, sec1],
            v_degree: Expr::count(1),
        },
    );
    (doc, [sec0, sec1, loft])
}

fn square_of(s: f64) -> LoopProgram {
    LoopProgram::polygon([
        (0.0, 0.0),
        (2.0 * s, 0.0),
        (2.0 * s, 2.0 * s),
        (0.0, 2.0 * s),
    ])
    .unwrap()
}

/// A loft wall at canonical segment `k`, spelled by each section's
/// piece there.
fn loft_wall(
    doc: &ProfileDoc,
    loft: RecipeNodeId,
    secs: [RecipeNodeId; 2],
    k: usize,
) -> StableName {
    fname(
        loft,
        RoleSeg::LoftWall(secs.iter().map(|&s| fixture::piece(doc, s, 0, k)).collect()),
    )
}

/// **A loft names a wall by the pieces its skin paired, one per
/// section, and a seam by the vertices it paired**; its caps' rims are
/// each end section's own pieces.
#[test]
fn a_loft_names_its_walls_and_seams_by_every_sections_piece() {
    let (doc, [sec0, sec1, loft]) = lofted("loft-pieces", square_of(1.0), square_of(1.5));
    let ev = fixture::run(&doc, &EvalOptions::default());
    let t = table(&ev, loft);
    for k in 0..4 {
        let _ = fixture::face_of(t, "loft wall", &loft_wall(&doc, loft, [sec0, sec1], k));
        let seam = fixture::ename(
            loft,
            RoleSeg::LoftSeam(vec![
                fixture::vpiece(&doc, sec0, 0, k),
                fixture::vpiece(&doc, sec1, 0, k),
            ]),
        );
        let _ = edge_of(t, "loft seam", &seam);
        let bottom = fixture::rim_edge(loft, CapEnd::Start, fixture::piece(&doc, sec0, 0, k));
        let top = fixture::rim_edge(loft, CapEnd::End, fixture::piece(&doc, sec1, 0, k));
        let _ = edge_of(t, "bottom rim", &bottom);
        let _ = edge_of(t, "top rim", &top);
    }
}

/// **A reshaping of a later section that keeps the pairing moves no
/// loft name; one that changes the pairing makes the old wall vanish.**
/// Each section gets a leg inserted — the lower before its wall 1, the
/// upper before its wall 2 — every old step kept, so both still skin
/// with five walls each. Wall 0 pairs the same two pieces and keeps its
/// name; the old wall 1 paired the lower's leg to `(2, 2)` with the
/// upper's leg to `(3, 3)`, which the skin now puts at different
/// positions, so no wall pairs them and the old wall's name (a paint
/// key here: a loft's walls are not planar carriers a frame could
/// stand on) denotes nothing rather than following `k` to the new
/// pairing.
#[test]
fn a_loft_wall_whose_pairing_changes_vanishes() {
    let (doc, [sec0, sec1, loft]) = lofted("loft-pairing", square_of(1.0), square_of(1.5));
    let kept = loft_wall(&doc, loft, [sec0, sec1], 0);
    let moved = loft_wall(&doc, loft, [sec0, sec1], 1);
    let doc = paint(&paint(&doc, &kept), &moved);
    let leg_at = |s: f64, at: usize, pt: (f64, f64)| {
        let mut v = vec![
            (0.0, 0.0),
            (2.0 * s, 0.0),
            (2.0 * s, 2.0 * s),
            (0.0, 2.0 * s),
        ];
        v.insert(at, pt);
        LoopProgram::polygon(v).unwrap()
    };
    let mut ids0 = keep_all(&doc, sec0);
    ids0[0].insert(2, None);
    let doc = accepted(&doc, sec0, vec![leg_at(1.0, 2, (3.0, 1.0))], ids0).doc;
    let mut ids1 = keep_all(&doc, sec1);
    ids1[0].insert(3, None);
    let applied = accepted(&doc, sec1, vec![leg_at(1.5, 3, (1.5, 4.0))], ids1);
    assert_eq!(applied.maintenance, Vec::new(), "no step was dropped");
    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    assert!(ev.value(loft).is_some(), "{:?}", corpus::failures(&ev));
    assert!(
        applied.doc.appearance().contains_key(&kept)
            && applied.doc.appearance().contains_key(&moved),
        "both painted names keep their spelling"
    );
    let table = &ev.value(loft).expect("the loft evaluates").name_table;
    assert!(table.lookup(&kept).is_some(), "wall 0 pairs what it paired");
    assert!(
        table.lookup(&moved).is_none(),
        "no wall pairs the old wall 1's pieces, so its name denotes nothing"
    );
}

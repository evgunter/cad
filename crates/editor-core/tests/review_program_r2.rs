//! Review lane `program-r2` probes for `DocEdit::SetProgram` (PR 2927).
//!
//! Each row targets one claim of the PR body; the row's doc says which.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/node.rs",
    "crates/editor-core/src/program.rs",
];

use crate::corpus;
use crate::fixture;

use editor_core::{
    CapEnd, Datum, Dimension, DocEdit, EditError, EvalOptions, Expr, LoggedEdit, LoopProgram,
    LoopProvenance, Maintenance, MeasureExpr, MeasurePrimitive, Node, NodeErrorKind, NodeResult,
    ParamName, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef, ProgramArcData,
    ProgramStep, ProgramTarget, ProvenanceFault, RecipeNodeId, ResolveError, RoleSeg, SitedRef,
    StableName, apply, load, save,
};
use fixture::{ang, edge_of, ends, face_of, fname, insert, len, point, scl, table, tol};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, rod_chord_at};

const BUMP: (f64, f64) = (1.25, -0.5);

/// The sunk rod's loop with an optional bump, inserted `at` one of
/// three seams: after the first leg (the PR's fixture), at the very
/// start (before the first drawing step) or as the last leg before the
/// close.
#[derive(Clone, Copy, PartialEq, Eq)]
enum At {
    None,
    AfterFirstLeg,
    Start,
    BeforeClose,
}

fn rod_loop(at: At) -> LoopProgram {
    let c = rod_chord_at(ROD_FLAT);
    let xv = c.half;
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let bump = |steps: &mut Vec<ProgramStep>, from: (f64, f64), to: (f64, f64)| {
        let (dx, dy) = (to.0 - from.0, to.1 - from.1);
        steps.push(ProgramStep::Toward {
            dx: scl(dx),
            dy: scl(dy),
        });
        steps.push(ProgramStep::Line(len(dx.hypot(dy))));
    };
    let mut steps = vec![ProgramStep::At(pt(-1.0, -1.0))];
    if at == At::Start {
        // A bump below the bottom edge, leaving (-1,-1).
        bump(&mut steps, (-1.0, -1.0), (-0.5, -1.25));
    }
    steps.push(ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))));
    if at == At::AfterFirstLeg {
        bump(&mut steps, (1.0, -1.0), BUMP);
    }
    steps.extend([
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(xv, 0.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(pt(-xv, 0.0)),
            b: scl(c.section_bulge),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(pt(-1.0, 0.0))),
    ]);
    if at == At::BeforeClose {
        // A bump on the left wall, between (-1,0) and (-1,-1).
        bump(&mut steps, (-1.0, 0.0), (-1.25, -0.5));
    }
    steps.push(ProgramStep::LineTo(ProgramTarget::Start));
    LoopProgram::Chain(steps)
}

/// Provenance of `rod_loop(at)` over `rod_loop(At::None)`: every old
/// step continues, the two bump steps are new.
fn provenance(at: At) -> Vec<LoopProvenance> {
    let mut steps: Vec<Option<u32>> = (0..7).map(Some).collect();
    let insert_at = match at {
        At::None => None,
        At::Start => Some(1),
        At::AfterFirstLeg => Some(2),
        At::BeforeClose => Some(6),
    };
    if let Some(i) = insert_at {
        steps.insert(i, None);
        steps.insert(i, None);
    }
    vec![LoopProvenance {
        from: Some(0),
        steps,
    }]
}

struct Rod {
    doc: ProfileDoc,
    profile: RecipeNodeId,
    rod: RecipeNodeId,
    fillet: Option<RecipeNodeId>,
}

fn rod(label: &str, creases: &[u32]) -> Rod {
    let doc = ProfileDoc::empty_derived(label, tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![rod_loop(At::None)],
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
        let selection = creases.iter().map(|&v| lateral_edge(rod, v)).collect();
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

fn lateral_edge(rod: RecipeNodeId, vertex: u32) -> StableName {
    fixture::ename(
        rod,
        RoleSeg::LateralEdge(ProfileVertexRef {
            loop_index: 0,
            vertex,
        }),
    )
}

fn wall(rod: RecipeNodeId, segment: u32) -> StableName {
    fname(
        rod,
        RoleSeg::Lateral(ProfileEdgeRef {
            loop_index: 0,
            segment,
        }),
    )
}

fn cap_vertex(rod: RecipeNodeId, end: CapEnd, vertex: u32) -> StableName {
    fixture::cap_vertex(
        rod,
        end,
        ProfileVertexRef {
            loop_index: 0,
            vertex,
        },
    )
}

fn set_program(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    loops: Vec<LoopProgram>,
    provenance: Vec<LoopProvenance>,
) -> Result<editor_core::Applied<ProfileProgram>, EditError> {
    apply(
        doc,
        &DocEdit::SetProgram {
            node,
            loops,
            provenance,
        },
        tol(),
        &editor_core::RefusingReach,
    )
}

fn accepted(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    loops: Vec<LoopProgram>,
    provenance: Vec<LoopProvenance>,
) -> editor_core::Applied<ProfileProgram> {
    set_program(doc, node, loops, provenance).expect("the reshaping is accepted")
}

fn selection_of(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<StableName> {
    match doc.node(node) {
        Some(Node::Fillet { selection, .. }) => selection.clone(),
        other => panic!("node {node:?} is a fillet, got {other:?}"),
    }
}

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

fn strut_at(doc: &ProfileDoc, rod: RecipeNodeId, name: &StableName) -> (f64, f64) {
    let ev = fixture::run(doc, &EvalOptions::default());
    let body = corpus::body_of(&ev, rod);
    let [a, b] = ends(body, edge_of(table(&ev, rod), "strut", name));
    let (pa, pb) = (point(body, a), point(body, b));
    assert!((pa.x - pb.x).abs() < 1e-12 && (pa.y - pb.y).abs() < 1e-12);
    (pa.x, pa.y)
}

fn vertex_at(doc: &ProfileDoc, rod: RecipeNodeId, name: &StableName) -> (f64, f64, f64) {
    let ev = fixture::run(doc, &EvalOptions::default());
    let body = corpus::body_of(&ev, rod);
    let p = point(
        body,
        fixture::vertex_of(table(&ev, rod), "cap vertex", name),
    );
    (p.x, p.y, p.z)
}

type Pose6 = (f64, f64, f64, f64, f64, f64);

fn face_origin(doc: &ProfileDoc, node: RecipeNodeId, name: &StableName) -> Pose6 {
    let ev = fixture::run(doc, &EvalOptions::default());
    let body = corpus::body_of(&ev, node);
    let pose = topo::readback::face_pose(body, face_of(table(&ev, node), "wall", name))
        .expect("a planar wall");
    (
        pose.origin.x,
        pose.origin.y,
        pose.origin.z,
        pose.axis.x,
        pose.axis.y,
        pose.axis.z,
    )
}

/// The `(x, y)` corners of the wall `name`: a kept step whose START
/// moved (the leg was inserted just before it) still draws the wall
/// that ARRIVES at its old corner, so the corner is what a rebound
/// wall shares with its old self.
fn wall_corners(doc: &ProfileDoc, node: RecipeNodeId, name: &StableName) -> Vec<(f64, f64)> {
    let ev = fixture::run(doc, &EvalOptions::default());
    let body = corpus::body_of(&ev, node);
    let mut out: Vec<(f64, f64)> =
        fixture::face_vertices(body, face_of(table(&ev, node), "wall", name))
            .into_iter()
            .map(|v| {
                let p = point(body, v);
                (p.x, p.y)
            })
            .collect();
    out.sort_by(|a, b| a.partial_cmp(b).unwrap());
    out.dedup_by(|a, b| near(*a, *b));
    out
}

fn has_corner(corners: &[(f64, f64)], want: (f64, f64)) -> bool {
    corners.iter().any(|&c| near(c, want))
}

fn near(got: (f64, f64), want: (f64, f64)) -> bool {
    (got.0 - want.0).abs() < 1e-12 && (got.1 - want.1).abs() < 1e-12
}

fn fillet_refuses_vanished(doc: &ProfileDoc, fillet: RecipeNodeId, retired: &StableName) {
    let ev = fixture::run(doc, &EvalOptions::default());
    match ev.nodes.get(&fillet) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::BlendSelectionResolve { error, .. } => match error.as_ref() {
                ResolveError::Vanished { name, .. } => assert_eq!(name, retired),
                other => panic!("the retired name resolves to nothing: got {other:?}"),
            },
            other => panic!("the fillet refuses on its selection, got {other:?}"),
        },
        other => panic!("the fillet refuses, got {other:?}"),
    }
}

// ---------------------------------------------------------------- //
// Claim 1: the retired coordinate under a SECOND reshaping that grows
// the loop past it, on the wire, and under Rebind.
// ---------------------------------------------------------------- //

/// The bumped rod plus six extra legs zig-zagging along the bottom
/// edge, so the loop's segment count (13) passes the retired vertex
/// index 11.
fn many_legged_rod() -> LoopProgram {
    let c = rod_chord_at(ROD_FLAT);
    let xv = c.half;
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let mut steps = vec![ProgramStep::At(pt(-1.0, -1.0))];
    let zig = [
        (-0.7, -1.1),
        (-0.4, -1.0),
        (-0.1, -1.1),
        (0.2, -1.0),
        (0.5, -1.1),
        (0.8, -1.0),
    ];
    for (x, y) in zig {
        steps.push(ProgramStep::LineTo(ProgramTarget::Point(pt(x, y))));
    }
    steps.push(ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))));
    let (dx, dy) = (BUMP.0 - 1.0, BUMP.1 + 1.0);
    steps.push(ProgramStep::Toward {
        dx: scl(dx),
        dy: scl(dy),
    });
    steps.push(ProgramStep::Line(len(dx.hypot(dy))));
    steps.extend([
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(xv, 0.0))),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(pt(-xv, 0.0)),
            b: scl(c.section_bulge),
        }),
        ProgramStep::LineTo(ProgramTarget::Point(pt(-1.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    LoopProgram::Chain(steps)
}

/// **Claim 1.** Strand the fillet's crease (retired to vertex 11 of a
/// 7-segment loop), then reshape AGAIN into a 13-segment loop whose
/// vertex 11 is a live corner. The retired name must not come back to
/// life; what the second edit reports about it is measured.
#[test]
fn r2_a_retired_name_stays_retired_when_a_later_reshaping_grows_past_it() {
    let r = rod("r2-retire-grow", &[4]);
    let fillet = r.fillet.unwrap();
    let mut without_arc = provenance(At::AfterFirstLeg);
    without_arc[0].steps[6] = None;
    let first = accepted(
        &r.doc,
        r.profile,
        vec![rod_loop(At::AfterFirstLeg)],
        without_arc,
    );
    let retired = lateral_edge(r.rod, 7 + 4);
    assert_eq!(selection_of(&first.doc, fillet), vec![retired.clone()]);

    // Old (bumped) steps: At, LineTo(1,-1), Toward, Line, LineTo(1,0),
    // LineTo(xv,0), Arc, LineTo(-1,0), Close = 9. New: At, 6 zig,
    // LineTo(1,-1), Toward, Line, LineTo(1,0), LineTo(xv,0), Arc,
    // LineTo(-1,0), Close = 15.
    let mut steps: Vec<Option<u32>> = vec![Some(0)];
    steps.extend([None; 6]);
    steps.extend((1..9).map(Some));
    let second = accepted(
        &first.doc,
        r.profile,
        vec![many_legged_rod()],
        vec![LoopProvenance {
            from: Some(0),
            steps,
        }],
    );
    let now = selection_of(&second.doc, fillet);
    assert_eq!(now.len(), 1);
    assert_ne!(
        now[0], retired,
        "the retired name did not stay at vertex 11"
    );
    let re_retired = lateral_edge(r.rod, 13 + 11);
    assert_eq!(now[0], re_retired, "retired again, further past the end");
    fillet_refuses_vanished(&second.doc, fillet, &re_retired);
    println!("second edit's maintenance: {:?}", second.maintenance);
    assert_eq!(
        second.maintenance,
        vec![Maintenance::Strand {
            node: fillet,
            name: re_retired
        }],
        "measured: an already-stranded name is reported Strand again by an edit that removed \
         nothing of it"
    );
}

/// **Claim 1, the wire and the repair.** A retired name in a snapshot
/// round-trips (the load door admits a locator past the loop's end),
/// and `Rebind` from the retired spelling to the crease's live name
/// repairs the fillet.
#[test]
fn r2_a_retired_name_round_trips_the_wire_and_rebinds() {
    let r = rod("r2-retire-wire", &[4]);
    let fillet = r.fillet.unwrap();
    let mut without_arc = provenance(At::AfterFirstLeg);
    without_arc[0].steps[6] = None;
    let stranded = accepted(
        &r.doc,
        r.profile,
        vec![rod_loop(At::AfterFirstLeg)],
        without_arc.clone(),
    );
    let retired = lateral_edge(r.rod, 7 + 4);
    let text = save(&stranded.doc, &[], tol()).expect("saves");
    let loaded = load(&text, tol()).expect("a retired locator loads");
    assert!(loaded.doc.bit_eq(&stranded.doc));
    assert_eq!(selection_of(&loaded.doc, fillet), vec![retired.clone()]);
    let log = vec![LoggedEdit::bare(DocEdit::SetProgram {
        node: r.profile,
        loops: vec![rod_loop(At::AfterFirstLeg)],
        provenance: without_arc,
    })];
    let text = save(&r.doc, &log, tol()).expect("saves");
    let loaded = load(&text, tol()).expect("loads");
    assert!(loaded.doc.bit_eq(&stranded.doc));

    let repaired = apply(
        &stranded.doc,
        &DocEdit::Rebind {
            from: retired,
            to: lateral_edge(r.rod, 5),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("rebind from a retired name");
    assert_eq!(
        selection_of(&repaired.doc, fillet),
        vec![lateral_edge(r.rod, 5)]
    );
    let xv = rod_chord_at(ROD_FLAT).half;
    assert!(near(
        strut_at(&repaired.doc, r.rod, &lateral_edge(r.rod, 5)),
        (-xv, 0.0)
    ));
    let ev = fixture::run(&repaired.doc, &EvalOptions::default());
    assert!(ev.value(fillet).is_some(), "{:?}", corpus::failures(&ev));
}

// ---------------------------------------------------------------- //
// Claim 3: the loop's seams.
// ---------------------------------------------------------------- //

/// **Claim 3.** A leg inserted at the loop's START (before the first
/// drawing step): vertex 0 is the end of the closing segment, which is
/// kept, so vertex 0 stays vertex 0; wall 0 becomes wall 1; the last
/// vertex 5 becomes 6; the closing wall 5 becomes 6. Measured on the
/// solid, with a cap vertex and a strut on vertex 0.
#[test]
fn r2_a_leg_inserted_at_the_loops_start_keeps_vertex_zero_and_moves_the_rest() {
    let r = rod("r2-seam-start", &[]);
    let doc = r.doc;
    let (doc, _on_wall_0) = frame_on(doc, r.rod, wall(r.rod, 0));
    let (doc, _on_wall_5) = frame_on(doc, r.rod, wall(r.rod, 5));
    let (doc, fil) = insert(
        doc,
        Node::fillet(
            r.rod,
            len(0.05),
            vec![lateral_edge(r.rod, 0), lateral_edge(r.rod, 5)],
        ),
    );
    let cap0 = cap_vertex(r.rod, CapEnd::End, 0);
    let cap5 = cap_vertex(r.rod, CapEnd::Start, 5);
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: wall(r.rod, 0),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .unwrap()
    .doc;
    let v0_before = strut_at(&doc, r.rod, &lateral_edge(r.rod, 0));
    let v5_before = strut_at(&doc, r.rod, &lateral_edge(r.rod, 5));
    let w0_before = face_origin(&doc, r.rod, &wall(r.rod, 0));
    let w5_before = face_origin(&doc, r.rod, &wall(r.rod, 5));
    let c0_before = vertex_at(&doc, r.rod, &cap0);
    let c5_before = vertex_at(&doc, r.rod, &cap5);
    assert!(near(v0_before, (-1.0, -1.0)));
    assert!(near(v5_before, (-1.0, 0.0)));

    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(At::Start)],
        provenance(At::Start),
    );
    println!("start-seam maintenance: {:#?}", applied.maintenance);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Rebound {
                from: wall(r.rod, 0),
                to: wall(r.rod, 1),
            },
            Maintenance::Rebound {
                from: wall(r.rod, 5),
                to: wall(r.rod, 6),
            },
            Maintenance::Rebound {
                from: lateral_edge(r.rod, 5),
                to: lateral_edge(r.rod, 6),
            },
        ],
        "vertex 0 stays; wall 0, wall 5 and vertex 5 move"
    );
    assert_eq!(
        selection_of(&applied.doc, fil),
        vec![lateral_edge(r.rod, 0), lateral_edge(r.rod, 6)]
    );
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 0)),
        v0_before
    ));
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 6)),
        v5_before
    ));
    // Old wall 0's START is the inserted point now, so its plane
    // moved; what it keeps is the corner it arrives at.
    let c = wall_corners(&applied.doc, r.rod, &wall(r.rod, 1));
    assert!(
        has_corner(&c, (1.0, -1.0)) && has_corner(&c, (-0.5, -1.25)),
        "{c:?}"
    );
    assert_ne!(face_origin(&applied.doc, r.rod, &wall(r.rod, 1)), w0_before);
    assert_eq!(face_origin(&applied.doc, r.rod, &wall(r.rod, 6)), w5_before);
    assert_eq!(vertex_at(&applied.doc, r.rod, &cap0), c0_before);
    assert_eq!(
        vertex_at(&applied.doc, r.rod, &cap_vertex(r.rod, CapEnd::Start, 6)),
        c5_before
    );
    assert!(applied.doc.appearance_of(&wall(r.rod, 1)).is_some());
}

/// **Claim 3.** A leg inserted as the LAST leg before the close: the
/// closing wall 5 becomes 6, vertex 5 stays (its arriving segment 4
/// is kept in place), vertex 0 stays (the close's end).
#[test]
fn r2_a_leg_inserted_before_the_close_moves_only_the_closing_wall() {
    let r = rod("r2-seam-close", &[]);
    let doc = r.doc;
    let (doc, _on_wall_5) = frame_on(doc, r.rod, wall(r.rod, 5));
    let (doc, _on_wall_4) = frame_on(doc, r.rod, wall(r.rod, 4));
    let (doc, fil) = insert(
        doc,
        Node::fillet(
            r.rod,
            len(0.05),
            vec![lateral_edge(r.rod, 0), lateral_edge(r.rod, 5)],
        ),
    );
    let v0 = strut_at(&doc, r.rod, &lateral_edge(r.rod, 0));
    let v5 = strut_at(&doc, r.rod, &lateral_edge(r.rod, 5));
    let w5 = face_origin(&doc, r.rod, &wall(r.rod, 5));
    let w4 = face_origin(&doc, r.rod, &wall(r.rod, 4));
    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(At::BeforeClose)],
        provenance(At::BeforeClose),
    );
    println!("close-seam maintenance: {:#?}", applied.maintenance);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Rebound {
            from: wall(r.rod, 5),
            to: wall(r.rod, 6),
        }]
    );
    assert_eq!(
        selection_of(&applied.doc, fil),
        vec![lateral_edge(r.rod, 0), lateral_edge(r.rod, 5)]
    );
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 0)),
        v0
    ));
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 5)),
        v5
    ));
    // The close's START is the inserted point now; it still arrives
    // at (-1, -1).
    let c = wall_corners(&applied.doc, r.rod, &wall(r.rod, 6));
    assert!(
        has_corner(&c, (-1.0, -1.0)) && has_corner(&c, (-1.25, -0.5)),
        "{c:?}"
    );
    assert_ne!(face_origin(&applied.doc, r.rod, &wall(r.rod, 6)), w5);
    assert_eq!(face_origin(&applied.doc, r.rod, &wall(r.rod, 4)), w4);
}

/// **Claim 3, the canonical permutation.** The same square authored
/// CLOCKWISE (canonicalization reverses it) and starting at a corner
/// other than the canonical start: a frame on wall 1 and one on wall
/// 3; a leg inserted before wall 1's step. The names must move in
/// PROGRAM coordinates and the walls they denote must be the same
/// planes as before.
#[test]
fn r2_names_on_a_reversed_and_rotated_loop_move_in_program_coordinates() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let cw = |bump: bool| {
        let mut steps = vec![
            ProgramStep::At(pt(2.0, 2.0)),
            ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ];
        if bump {
            steps.push(ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -0.5))));
        }
        steps.extend([
            ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
            ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
            ProgramStep::LineTo(ProgramTarget::Start),
        ]);
        LoopProgram::Chain(steps)
    };
    let doc = ProfileDoc::empty_derived("r2-cw", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![cw(false)],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, _f1) = frame_on(doc, ext, wall(ext, 1));
    let (doc, _f3) = frame_on(doc, ext, wall(ext, 3));
    let w1 = face_origin(&doc, ext, &wall(ext, 1));
    let w3 = face_origin(&doc, ext, &wall(ext, 3));
    let w0 = face_origin(&doc, ext, &wall(ext, 0));
    println!("cw walls before: w0={w0:?} w1={w1:?} w3={w3:?}");
    let applied = accepted(
        &doc,
        profile,
        vec![cw(true)],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
        }],
    );
    println!("cw maintenance: {:#?}", applied.maintenance);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Rebound {
                from: wall(ext, 1),
                to: wall(ext, 2),
            },
            Maintenance::Rebound {
                from: wall(ext, 3),
                to: wall(ext, 4),
            },
        ]
    );
    // Old wall 1 (2,0)→(0,0) now starts at the inserted (1,-0.5) and
    // still arrives at (0,0); wall 3 (the close) is untouched.
    let c = wall_corners(&applied.doc, ext, &wall(ext, 2));
    assert!(
        has_corner(&c, (0.0, 0.0)) && has_corner(&c, (1.0, -0.5)),
        "{c:?}"
    );
    assert_ne!(face_origin(&applied.doc, ext, &wall(ext, 2)), w1);
    assert_eq!(face_origin(&applied.doc, ext, &wall(ext, 4)), w3);
    assert_eq!(face_origin(&applied.doc, ext, &wall(ext, 0)), w0);
}

// ---------------------------------------------------------------- //
// Claim 2: the loft's second section, and the other holder kinds.
// ---------------------------------------------------------------- //

/// **Claim 2, the loft.** A two-section loft; a frame on its lateral
/// face 1. Reshaping section 1 (a leg inserted) is accepted and moves
/// nothing (names are section 0's); what the loft then does at
/// evaluation is printed. Reshaping section 0 rebinds the name.
#[test]
fn r2_a_loft_names_are_section_zeros_so_reshaping_the_second_section_moves_none() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let bumped =
        LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let doc = ProfileDoc::empty_derived("r2-loft", tol());
    let (doc, p0) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, sec0) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: p0,
            loops: vec![square.clone()],
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
            loops: vec![square],
        }),
    );
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![sec0, sec1],
            v_degree: Expr::count(1),
        },
    );
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(ev.value(loft).is_some(), "{:?}", corpus::failures(&ev));
    let names: Vec<String> = table(&ev, loft)
        .iter()
        .map(|(n, _)| format!("{n:?}"))
        .collect();
    println!("loft names: {names:#?}");
    let (doc, _frame) = frame_on(doc, loft, wall(loft, 1));
    let bump_prov = vec![LoopProvenance {
        from: Some(0),
        steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
    }];
    let s1_reshaped = accepted(&doc, sec1, vec![bumped.clone()], bump_prov.clone());
    assert_eq!(s1_reshaped.maintenance, vec![]);
    let ev = fixture::run(&s1_reshaped.doc, &EvalOptions::default());
    println!(
        "loft after section-1 reshape: value={} failures={:?}",
        ev.value(loft).is_some(),
        corpus::failures(&ev)
    );
    let s0_reshaped = accepted(&doc, sec0, vec![bumped], bump_prov);
    assert_eq!(
        s0_reshaped.maintenance,
        vec![Maintenance::Rebound {
            from: wall(loft, 1),
            to: wall(loft, 2),
        }]
    );
    let ev = fixture::run(&s0_reshaped.doc, &EvalOptions::default());
    println!(
        "loft after section-0 reshape: value={} failures={:?}",
        ev.value(loft).is_some(),
        corpus::failures(&ev)
    );
}

/// **Claim 2, every holder kind the suite lacks**: chamfer, shell open
/// list, measure refs, declare pairs — each rebound and each stranded,
/// in the contract's order.
#[test]
fn r2_every_other_holder_kind_is_rebound_and_stranded() {
    let r = rod("r2-holders", &[]);
    let doc = r.doc;
    let (doc, chamfer) = insert(
        doc,
        Node::Chamfer {
            target: r.rod,
            distance: len(0.05),
            selection: vec![lateral_edge(r.rod, 4)],
        },
    );
    let (doc, shell) = insert(doc, Node::shell(r.rod, len(0.05), vec![wall(r.rod, 3)]));
    let (doc, measure) = insert(
        doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(r.rod, wall(r.rod, 1)),
                SitedRef::new(r.rod, wall(r.rod, 3)),
            ],
        )
        .unwrap(),
    );
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(r.rod, wall(r.rod, 3)),
            SitedRef::new(r.rod, wall(r.rod, 1)),
        )]),
    );
    let mut without_arc = provenance(At::AfterFirstLeg);
    without_arc[0].steps[6] = None;
    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(At::AfterFirstLeg)],
        without_arc,
    );
    println!("holders maintenance: {:#?}", applied.maintenance);
    let retired_wall = wall(r.rod, 7 + 3);
    let retired_vertex = lateral_edge(r.rod, 7 + 4);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Strand {
                node: chamfer,
                name: retired_vertex.clone(),
            },
            Maintenance::Strand {
                node: shell,
                name: retired_wall.clone(),
            },
            Maintenance::Strand {
                node: measure,
                name: retired_wall.clone(),
            },
            Maintenance::Strand {
                node: decl,
                name: retired_wall.clone(),
            },
            Maintenance::Rebound {
                from: wall(r.rod, 1),
                to: wall(r.rod, 2),
            },
        ]
    );
    match applied.doc.node(measure) {
        Some(Node::Measure { refs, .. }) => {
            assert_eq!(refs[0].name, wall(r.rod, 2));
            assert_eq!(refs[1].name, retired_wall);
        }
        other => panic!("{other:?}"),
    }
    match applied.doc.node(shell) {
        Some(Node::Shell { open, .. }) => assert_eq!(open, &vec![retired_wall.clone()]),
        other => panic!("{other:?}"),
    }
    match applied.doc.node(decl) {
        Some(Node::Declare { pairs }) => {
            assert_eq!(pairs[0].0.0.name, retired_wall);
            assert_eq!(pairs[0].0.1.name, wall(r.rod, 2));
        }
        other => panic!("{other:?}"),
    }
}

// ---------------------------------------------------------------- //
// Claim 4/5: a shape fault beside an undeclared parameter.
// ---------------------------------------------------------------- //

#[test]
fn r2_a_shape_fault_wins_over_an_undeclared_parameter() {
    let r = rod("r2-fault-param", &[]);
    let nope = Expr::param(ParamName::new("nope"), Dimension::Length);
    let LoopProgram::Chain(mut steps) = rod_loop(At::None) else {
        panic!()
    };
    steps[1] = ProgramStep::LineTo(ProgramTarget::Point([nope, len(-1.0)]));
    let before = r.doc.clone();
    let err = set_program(
        &r.doc,
        r.profile,
        vec![LoopProgram::Chain(steps)],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![
                Some(0),
                Some(0),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
            ],
        }],
    )
    .unwrap_err();
    assert!(
        matches!(
            err,
            EditError::ProvenanceMalformed {
                fault: ProvenanceFault::OldStepContinuedTwice { .. },
                ..
            }
        ),
        "{err:?}"
    );
    assert!(r.doc.bit_eq(&before));
}

// ---------------------------------------------------------------- //
// Claim 8: the Python fixture, from Rust.
// ---------------------------------------------------------------- //

/// The Python rows' fixture built in Rust: a square prism, a fillet on
/// the rim edge wall 2 shares with the end cap, the leg `(3, 1)`
/// inserted. Provenance `[0,1,None,2,3,4]` rebinds 2 → 3; provenance
/// `[0,1,None,2,None,4]` strands it.
#[test]
fn r2_the_python_fixture_rebinds_and_strands_as_the_rows_say() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let bumped =
        LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let doc = ProfileDoc::empty_derived("r2-python", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![square],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let rim = |seg: u32| {
        fixture::rim_edge(
            ext,
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment: seg,
            },
        )
    };
    let (doc, fillet) = insert(doc, Node::fillet(ext, len(0.1), vec![rim(2)]));
    let rebound = accepted(
        &doc,
        profile,
        vec![bumped.clone()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
        }],
    );
    assert_eq!(
        rebound.maintenance,
        vec![Maintenance::Rebound {
            from: rim(2),
            to: rim(3)
        }]
    );
    let stranded = accepted(
        &doc,
        profile,
        vec![bumped],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), None, Some(4)],
        }],
    );
    assert_eq!(
        stranded.maintenance,
        vec![Maintenance::Strand {
            node: fillet,
            name: rim(5 + 2)
        }]
    );
}

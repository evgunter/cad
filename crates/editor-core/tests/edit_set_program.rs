//! **A live profile's program is replaced whole, and every name its
//! reshaping touches is reported or rebound** (`DocEdit::SetProgram`;
//! V2 of `crates/profile/README.md`, DM7 widened).
//!
//! Every holder of a profile name — a fillet's selection, a derived
//! frame's face, a paint in the appearance store — holds a
//! `ProfileEdgeRef`/`ProfileVertexRef` by INDEX in the program's own
//! coordinates. A reshaped program (a leg inserted before a wall)
//! leaves the index valid and denoting another segment, so "state the
//! new program in full" is not enough: the edit also carries, per new
//! loop and step, which old loop and step it continues, and the door
//! reads which segments each old step drew and each new step draws off
//! the two replay records, rewrites every name on a kept step to its
//! new coordinates in place (reported `Rebound`), and retires every
//! name on a dropped or changed step to a coordinate no program draws
//! (reported `Strand` / `StrandedAppearance`, resolving `Vanished`
//! until it is rebound).
//!
//! The fixture most rows share is `edit_ruled_carve`'s sunk rod: a
//! block with a rod's section standing on its top edge, whose two
//! cylinder-meets-plane creases are the one straight edge a fillet
//! carves on a prism with transverse caps — so a fillet on ONE
//! profile vertex evaluates, before and after the reshaping, and the
//! wall it lands on is measured on the solid rather than read off an
//! index.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/src/node.rs",
    "crates/editor-core/src/program.rs",
    "crates/editor-core/src/persist/",
    "crates/editor-core/tests/corpus/",
    "crates/editor-core/tests/fixture/",
];

use crate::corpus;
use crate::fixture;

use editor_core::{
    Attr, CapEnd, Datum, Dimension, DocEdit, DocParam, EditError, EntityKind, EvalOptions, Expr,
    LoggedEdit, LoopProgram, LoopProvenance, Maintenance, NameRef, Node, NodeErrorKind, NodeResult,
    ParamName, PersistError, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef,
    ProgramArcData, ProgramStep, ProgramTarget, ProvenanceFault, RecipeNodeId, ResolveError, Rgba8,
    RoleSeg, SlotId, StableName, StepArg, apply, load, save,
};
use fixture::{ang, edge_of, ends, fname, insert, len, minted, point, scl, table, tol};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, rod_chord_at};

// ---------------------------------------------------------------- //
// The fixture: the sunk rod, and the leg the reshaping inserts
// ---------------------------------------------------------------- //

/// Where the inserted leg lands: a bump on the block's right wall,
/// between the corner `(1, −1)` and the corner `(1, 0)`.
const BUMP: (f64, f64) = (1.25, -0.5);

/// The area the bump adds to the section — the triangle
/// `(1, −1), (1.25, −0.5), (1, 0)` — so the reshaped rod's volume is
/// the old one's plus this times the length.
const BUMP_AREA: f64 = 0.125;

/// The sunk rod's loop (`edit_ruled_carve::sunk_rod`, verbatim), with
/// or without the bump.
///
/// Without: steps `At, LineTo(1,−1), LineTo(1,0), LineTo(xv,0), Arc,
/// LineTo(−1,0), LineTo(Start)` — six segments, the arc being segment
/// 3 and the two creases the vertices 3 and 4 it runs between.
///
/// With: the bump is authored as a DIRECTION and a LENGTH — `Toward`
/// then `Line` — two steps that draw ONE segment, so the step indices
/// after it move by two while the segment indices move by one. That
/// asymmetry is deliberate: a door that rebound names by step index
/// instead of by the span's segments would land the crease one
/// vertex too far, and the rows below measure where it lands.
fn rod_loop(bump: bool) -> LoopProgram {
    let c = rod_chord_at(ROD_FLAT);
    let xv = c.half;
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let mut steps = vec![
        ProgramStep::At(pt(-1.0, -1.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))),
    ];
    if bump {
        let (dx, dy) = (BUMP.0 - 1.0, BUMP.1 + 1.0);
        steps.push(ProgramStep::Toward {
            dx: scl(dx),
            dy: scl(dy),
        });
        steps.push(ProgramStep::Line(len(dx.hypot(dy))));
    }
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

/// The provenance of the bumped loop over the plain one: every step
/// continues its old self except the two the bump inserted.
fn bump_provenance() -> Vec<LoopProvenance> {
    vec![LoopProvenance {
        from: Some(0),
        steps: vec![
            Some(0),
            Some(1),
            None,
            None,
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
        ],
    }]
}

/// [`bump_provenance`] with the ARC step (old step 4, new step 6)
/// stated as new — the reshaping that keeps the arc's segment in the
/// program and drops the step that drew it.
fn bump_provenance_without_the_arc() -> Vec<LoopProvenance> {
    let mut p = bump_provenance();
    p[0].steps[6] = None;
    p
}

/// The number of segments the bumped loop replays to.
const BUMPED_SEGMENTS: u32 = 7;

/// A document holding the sunk rod, with `creases` (profile vertices)
/// filleted when any are named.
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
            loops: vec![rod_loop(false)],
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

fn wall_of(rod: RecipeNodeId, loop_index: u32, segment: u32) -> StableName {
    fname(
        rod,
        RoleSeg::Lateral(ProfileEdgeRef {
            loop_index,
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

/// The rebound rows, in report order.
fn rebounds(rows: &[Maintenance]) -> Vec<(StableName, StableName)> {
    rows.iter()
        .filter_map(|row| match row {
            Maintenance::Rebound { from, to } => Some((from.clone(), to.clone())),
            Maintenance::Cluster(_)
            | Maintenance::Strand { .. }
            | Maintenance::StrandedAppearance { .. }
            | Maintenance::OrphanedDeclare { .. } => None,
        })
        .collect()
}

/// A blend node's selection as the document holds it.
fn selection_of(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<StableName> {
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

fn paint(doc: &ProfileDoc, name: &StableName) -> ProfileDoc {
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

fn volume(doc: &ProfileDoc, node: RecipeNodeId) -> f64 {
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
fn strut_at(doc: &ProfileDoc, rod: RecipeNodeId, name: &StableName) -> (f64, f64) {
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

// ---------------------------------------------------------------- //
// The finding's row, and its variant
// ---------------------------------------------------------------- //

/// **A fillet on a crease survives a leg inserted before it, rebound
/// and reported.** The crease at profile vertex 4 (the arc's end, the
/// rod's left crease) is filleted; a bump is inserted between the
/// corners at vertices 1 and 2, two steps drawing one segment. The
/// accepted edit carries exactly one `Rebound` — the crease's strut
/// name, vertex 4 → vertex 5 — the fillet's selection holds the new
/// spelling, and the evaluated fillet is on the SAME crease: the
/// rebound strut stands at the arc's end `(−xv, 0)`, the fillet's own
/// end arcs compose on the rebound name, and the solid's volume is the
/// old one's plus exactly the bump's prism.
///
/// The mutants this reds: a door that rebinds by step index (the
/// crease would land at vertex 6, the block's corner, where no
/// cylinder meets a plane and the fillet refuses); a door that leaves
/// the name alone (vertex 4 is now the arc's START, the other crease,
/// and the volume differs by the fillet's material on the wrong
/// crease); a door that reports nothing (the `Rebound` row is asserted
/// whole).
#[test]
fn a_fillet_on_a_crease_survives_a_leg_inserted_before_it_rebound_and_reported() {
    let r = rod("set-program-finding", &[4]);
    let fillet = r.fillet.unwrap();
    let before = volume(&r.doc, fillet);
    let xv = rod_chord_at(ROD_FLAT).half;
    assert!(
        near(strut_at(&r.doc, r.rod, &lateral_edge(r.rod, 4)), (-xv, 0.0)),
        "the fixture's crease 4 is the arc's end"
    );

    let applied = accepted(&r.doc, r.profile, vec![rod_loop(true)], bump_provenance());
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Rebound {
            from: lateral_edge(r.rod, 4),
            to: lateral_edge(r.rod, 5),
        }],
        "exactly one name moved, and the edit says which and where to"
    );
    assert!(
        applied.record.structural,
        "a reshaped program is structural"
    );
    assert_eq!(
        selection_of(&applied.doc, fillet),
        vec![lateral_edge(r.rod, 5)],
        "the fillet's selection was rewritten in place"
    );

    // Measured on the solid: the rebound strut stands where the old
    // one did, the fillet composes on it, and the volume moved by the
    // bump alone.
    assert!(
        near(
            strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 5)),
            (-xv, 0.0)
        ),
        "the rebound name denotes the same crease"
    );
    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    let t = table(&ev, fillet);
    for end in [CapEnd::Start, CapEnd::End] {
        let arc = minted(
            EntityKind::Edge,
            fillet,
            RoleSeg::EndArc {
                vertex: NameRef::new(cap_vertex(r.rod, end, 5)),
                edge: NameRef::new(lateral_edge(r.rod, 5)),
            },
        );
        let _ = edge_of(t, "the cut-off arc at the rebound crease", &arc);
    }
    let after = volume(&applied.doc, fillet);
    let want = before + BUMP_AREA * ROD_L;
    assert!(
        (after - want).abs() <= 1e-9 * want,
        "the volume moved by the bump's prism alone: {after} vs {want}"
    );
}

/// **The same edit with the crease's step not continued strands the
/// name and rebinds nothing.** The provenance states the arc step as
/// new; the arc is still in the program, but the step that drew the
/// crease's arriving segment is not a kept one, so the fillet's name
/// is retired past the loop's end — vertex 4 becomes vertex
/// `7 + 4 = 11` of a seven-segment loop — reported `Strand` with that
/// spelling, and at the next evaluation the fillet refuses `Vanished`
/// on exactly that name (rung 3 of the N5 ladder: the node is live and
/// its table has no such entry).
///
/// A door that left the name in place would red here twice: no
/// `Vanished`, because vertex 4 of the reshaped loop IS a vertex — the
/// arc's start, the other crease — and the fillet would carve it
/// without a word.
#[test]
fn a_step_the_provenance_does_not_continue_strands_the_names_on_its_segments() {
    let r = rod("set-program-dropped", &[4]);
    let fillet = r.fillet.unwrap();
    let applied = accepted(
        &r.doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired = lateral_edge(r.rod, BUMPED_SEGMENTS + 4);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Strand {
            node: fillet,
            name: retired.clone(),
        }],
        "the crease's name strands, and nothing is rebound"
    );
    assert_eq!(selection_of(&applied.doc, fillet), vec![retired.clone()]);

    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    match ev.nodes.get(&fillet) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::BlendSelectionResolve { error, .. } => match error.as_ref() {
                ResolveError::Vanished { name, .. } => assert_eq!(name, &retired),
                other => panic!("the retired name resolves to nothing: got {other:?}"),
            },
            other => panic!("the fillet refuses on its selection, got {other:?}"),
        },
        other => panic!("the fillet refuses, got {other:?}"),
    }
}

/// **A vertex is carried by the segment ARRIVING at it.** Two struts
/// on the block's right wall, at the corners the bump is inserted
/// between: vertex 1 is where the inserted leg STARTS and stays where
/// it is, vertex 2 is where it ends and moves one along. Measured on
/// the solid, both before and after.
///
/// The door the spec described — a vertex following the segment
/// LEAVING it — reds here: it would move vertex 1 onto the bump's
/// own point.
#[test]
fn a_vertex_is_carried_by_the_segment_arriving_at_it() {
    let r = rod("set-program-vertex", &[1, 2]);
    let fillet = r.fillet.unwrap();
    assert!(near(
        strut_at(&r.doc, r.rod, &lateral_edge(r.rod, 1)),
        (1.0, -1.0)
    ));
    assert!(near(
        strut_at(&r.doc, r.rod, &lateral_edge(r.rod, 2)),
        (1.0, 0.0)
    ));

    let applied = accepted(&r.doc, r.profile, vec![rod_loop(true)], bump_provenance());
    assert_eq!(
        rebounds(&applied.maintenance),
        vec![(lateral_edge(r.rod, 2), lateral_edge(r.rod, 3))],
        "vertex 2 moved one along; vertex 1 did not move at all"
    );
    assert_eq!(
        selection_of(&applied.doc, fillet),
        vec![lateral_edge(r.rod, 1), lateral_edge(r.rod, 3)]
    );
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 1)),
        (1.0, -1.0)
    ));
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 3)),
        (1.0, 0.0)
    ));
    assert!(near(
        strut_at(&applied.doc, r.rod, &lateral_edge(r.rod, 2)),
        BUMP
    ));
}

// ---------------------------------------------------------------- //
// Changed steps, dropped and moved loops
// ---------------------------------------------------------------- //

/// A unit square, extruded, as a chain the rows below reshape.
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
    let (doc, profile) = insert(doc, Node::Profile(ProfileProgram { plane, loops }));
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, profile, ext)
}

/// **A step whose segment count moved is not a kept one.** The
/// square's first corner is re-authored as a fillet — a directed
/// side, `fillet(r)`, the arrival's direction and its far end — so the
/// step that arrives at `(2, 2)` draws three segments (the trimmed
/// side, the arc, its own leg) where it drew one; the provenance
/// claims that step kept, and the door does not believe it: the wall
/// it drew strands, retired past the loop's end, while the wall of the
/// next step — one segment before and after — is rebound. The two rows
/// come back strands first.
#[test]
fn a_step_whose_segment_count_moved_is_not_a_kept_one() {
    let (doc, profile, ext) = extruded(
        "set-program-changed",
        vec![LoopProgram::Chain(square_steps())],
    );
    let (doc, frame_on_wall_1) = frame_on(doc, ext, wall(ext, 1));
    let (doc, _frame_on_wall_2) = frame_on(doc, ext, wall(ext, 2));

    let pt = |x: f64, y: f64| [len(x), len(y)];
    let steps = vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::Fillet(len(0.3)),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(1.0),
        },
        ProgramStep::FarEndTo(pt(2.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ];
    // The far end is where the old step 2 arrived, so the provenance
    // claims it continues that step; the old step 1's corner is what
    // the fillet consumed.
    let provenance = vec![LoopProvenance {
        from: Some(0),
        steps: vec![Some(0), None, None, None, Some(2), Some(3), Some(4)],
    }];
    let applied = accepted(&doc, profile, vec![LoopProgram::Chain(steps)], provenance);
    // Five segments now: the filleted corner is two vertices.
    let retired = wall(ext, 5 + 1);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Strand {
                node: frame_on_wall_1,
                name: retired.clone(),
            },
            Maintenance::Rebound {
                from: wall(ext, 2),
                to: wall(ext, 3),
            },
        ],
        "the changed step's wall strands, the next step's wall moves, strands first"
    );
    match applied.doc.node(frame_on_wall_1) {
        Some(Node::Datum(Datum::FaceFrame { face, .. })) => assert_eq!(face, &retired),
        other => panic!("the frame carries the retired name, got {other:?}"),
    }
}

/// **A dropped loop strands every name on it; a loop that moved index
/// rebinds every name on it.** A square with a round hole; a frame on
/// the square's wall 1 and one on the hole's single wall. Dropping the
/// hole strands the hole's name, retired onto a loop past the
/// program's end, and leaves the square's alone. Swapping the two
/// loops' order rebinds both — the hole to loop 0, the square's wall
/// to loop 1.
#[test]
fn a_dropped_loop_strands_and_a_moved_loop_rebinds_every_name_on_it() {
    let square = LoopProgram::Chain(square_steps());
    let hole = LoopProgram::circle(1.0, 1.0, 0.3).unwrap();
    let (doc, profile, ext) = extruded("set-program-loops", vec![square.clone(), hole.clone()]);
    let (doc, on_square) = frame_on(doc, ext, wall_of(ext, 0, 1));
    let (doc, on_hole) = frame_on(doc, ext, wall_of(ext, 1, 0));

    let dropped = accepted(
        &doc,
        profile,
        vec![square.clone()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), Some(2), Some(3), Some(4)],
        }],
    );
    assert_eq!(
        dropped.maintenance,
        vec![Maintenance::Strand {
            node: on_hole,
            name: wall_of(ext, 1 + 1, 0),
        }],
        "the hole's name strands on a loop past the one-loop program's end"
    );

    let swapped = accepted(
        &doc,
        profile,
        vec![hole, square],
        vec![
            LoopProvenance {
                from: Some(1),
                steps: vec![Some(0)],
            },
            LoopProvenance {
                from: Some(0),
                steps: vec![Some(0), Some(1), Some(2), Some(3), Some(4)],
            },
        ],
    );
    assert_eq!(
        swapped.maintenance,
        vec![
            Maintenance::Rebound {
                from: wall_of(ext, 0, 1),
                to: wall_of(ext, 1, 1),
            },
            Maintenance::Rebound {
                from: wall_of(ext, 1, 0),
                to: wall_of(ext, 0, 0),
            },
        ],
        "both names moved with their loops, in the carriers' document order"
    );
    let _ = on_square;
    // The swapped program still extrudes: description order is recipe
    // data and the anchor maps it. (The frame on the hole's wall
    // refuses on its own account — a cylinder carries no planar
    // frame — which is the carrier's business, not the reshaping's.)
    let ev = fixture::run(&swapped.doc, &EvalOptions::default());
    assert!(ev.value(ext).is_some(), "{:?}", corpus::failures(&ev));
}

/// **A program that no longer replays has no spans, so every name on
/// it strands.** A hole whose radius a document parameter drives is
/// driven to zero — legal at rest (V1 class 2) — and the program is
/// then replaced by one whose radius is a literal, under the identity
/// provenance. The old program's spans cannot be read, so the
/// provenance cannot be honoured: the hole's name strands, reported,
/// never guessed kept.
#[test]
fn a_program_that_no_longer_replays_has_no_spans_so_every_name_on_it_strands() {
    let square = LoopProgram::Chain(square_steps());
    let radius = ParamName::new("hole_r");
    let driven = LoopProgram::Circle {
        centre: [len(1.0), len(1.0)],
        radius: Expr::param(radius.clone(), Dimension::Length),
    };
    let doc = ProfileDoc::empty_derived("set-program-refusing", tol());
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetDocParam {
            name: radius.clone(),
            value: DocParam::continuous(Dimension::Length, 0.3),
        },
    );
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![square.clone(), driven],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, on_hole) = frame_on(doc, ext, wall_of(ext, 1, 0));
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetDocParam {
            name: radius,
            value: DocParam::continuous(Dimension::Length, 0.0),
        },
    );
    let applied = accepted(
        &doc,
        profile,
        vec![square, LoopProgram::circle(1.0, 1.0, 0.3).unwrap()],
        vec![
            LoopProvenance {
                from: Some(0),
                steps: vec![Some(0), Some(1), Some(2), Some(3), Some(4)],
            },
            LoopProvenance {
                from: Some(1),
                steps: vec![Some(0)],
            },
        ],
    );
    // The hole loop is continued (into new loop 1, which draws the
    // circle's segments), so its retired coordinate sits past THAT
    // loop's end.
    let hole_segments = match applied.maintenance.as_slice() {
        [Maintenance::Strand { node, name }] => {
            assert_eq!(*node, on_hole);
            match name.path.as_slice() {
                [RoleSeg::Lateral(e)] => {
                    assert_eq!(e.loop_index, 1);
                    e.segment
                }
                other => panic!("a retired wall, got {other:?}"),
            }
        }
        other => panic!("one strand and nothing else, got {other:?}"),
    };
    assert!(
        hole_segments >= 1,
        "retired past the circle's segments: {hole_segments}"
    );
}

// ---------------------------------------------------------------- //
// The appearance store, and the order contract
// ---------------------------------------------------------------- //

/// **An attachment keyed on a moved wall is re-keyed and reported;
/// one on a dropped wall is reported stranded under its retired key
/// and still reachable there.** Paint on the rod's right wall
/// (segment 1) and on its arc wall (segment 3); the bump moves both,
/// then the bump without the arc's step strands the arc's.
#[test]
fn an_appearance_attachment_keyed_on_a_moved_wall_is_rekeyed_and_reported() {
    let r = rod("set-program-paint", &[]);
    let doc = paint(&r.doc, &wall(r.rod, 1));
    let doc = paint(&doc, &wall(r.rod, 3));
    let red = doc.appearance_of(&wall(r.rod, 3)).cloned().unwrap();

    let moved = accepted(&doc, r.profile, vec![rod_loop(true)], bump_provenance());
    assert_eq!(
        moved.maintenance,
        vec![
            Maintenance::Rebound {
                from: wall(r.rod, 1),
                to: wall(r.rod, 2),
            },
            Maintenance::Rebound {
                from: wall(r.rod, 3),
                to: wall(r.rod, 4),
            },
        ],
        "both keys moved, in the store's key order"
    );
    assert_eq!(moved.doc.appearance_of(&wall(r.rod, 4)), Some(&red));
    assert!(moved.doc.appearance_of(&wall(r.rod, 3)).is_none());
    assert!(moved.doc.appearance_of(&wall(r.rod, 1)).is_none());

    let dropped = accepted(
        &doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired = wall(r.rod, BUMPED_SEGMENTS + 3);
    assert_eq!(
        dropped.maintenance,
        vec![
            Maintenance::StrandedAppearance {
                name: retired.clone(),
            },
            Maintenance::Rebound {
                from: wall(r.rod, 1),
                to: wall(r.rod, 2),
            },
        ],
        "the stranded key first, then the moved one"
    );
    assert_eq!(
        dropped.doc.appearance_of(&retired),
        Some(&red),
        "the attachment is reachable under the retired key, untouched"
    );
}

/// **A reshaping reports its strands, then its stranded keys, then
/// its rebounds** — the contract [`editor_core::Applied::maintenance`]
/// states, held by one edit that produces all three kinds: a frame on
/// the arc wall strands, a paint on the arc wall strands, and a wall
/// held by BOTH a frame and a paint moves — under ONE `Rebound` row,
/// since the row is about the name and not about where it was found.
#[test]
fn a_reshaping_reports_its_strands_then_its_stranded_keys_then_its_rebounds() {
    let r = rod("set-program-order", &[]);
    let (doc, _on_wall_1) = frame_on(r.doc, r.rod, wall(r.rod, 1));
    let (doc, on_wall_3) = frame_on(doc, r.rod, wall(r.rod, 3));
    let doc = paint(&doc, &wall(r.rod, 1));
    let doc = paint(&doc, &wall(r.rod, 3));

    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired = wall(r.rod, BUMPED_SEGMENTS + 3);
    assert_eq!(
        applied.maintenance,
        vec![
            Maintenance::Strand {
                node: on_wall_3,
                name: retired.clone(),
            },
            Maintenance::StrandedAppearance { name: retired },
            Maintenance::Rebound {
                from: wall(r.rod, 1),
                to: wall(r.rod, 2),
            },
        ]
    );
}

// ---------------------------------------------------------------- //
// The refusals
// ---------------------------------------------------------------- //

/// **Every provenance shape fault refuses typed before the program is
/// checked.** Each fault is provoked on a program that would ALSO
/// refuse the VQ9 door (the square re-authored to cross itself), and
/// the refusal is the shape's, not the program's: the shape is read
/// first. The document handed in is untouched by every refusal —
/// `apply` is pure, and the row says so by bits.
#[test]
fn every_provenance_shape_fault_refuses_typed_before_the_program_is_checked() {
    let square = LoopProgram::Chain(square_steps());
    let hole = LoopProgram::circle(1.0, 1.0, 0.3).unwrap();
    let (doc, profile, _) = extruded("set-program-shape", vec![square, hole.clone()]);
    let before = doc.clone();
    let pt = |x: f64, y: f64| [len(x), len(y)];
    // A bow tie: replays, does not validate.
    let crossing = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let full = |from: Option<u32>| LoopProvenance {
        from,
        steps: vec![Some(0), Some(1), Some(2), Some(3), Some(4)],
    };
    let cases: Vec<(&str, Vec<LoopProgram>, Vec<LoopProvenance>, ProvenanceFault)> = vec![
        (
            "loop count",
            vec![crossing.clone()],
            vec![full(Some(0)), full(Some(1))],
            ProvenanceFault::LoopCount {
                loops: 1,
                provenance: 2,
            },
        ),
        (
            "step count",
            vec![crossing.clone()],
            vec![LoopProvenance {
                from: Some(0),
                steps: vec![Some(0), Some(1)],
            }],
            ProvenanceFault::StepCount {
                loop_: 0,
                steps: 5,
                provenance: 2,
            },
        ),
        (
            "no such old loop",
            vec![crossing.clone()],
            vec![full(Some(3))],
            ProvenanceFault::NoSuchOldLoop {
                loop_: 0,
                from: 3,
                old_loops: 2,
            },
        ),
        (
            "no such old step",
            vec![crossing.clone()],
            vec![LoopProvenance {
                from: Some(0),
                steps: vec![Some(0), Some(1), Some(9), Some(3), Some(4)],
            }],
            ProvenanceFault::NoSuchOldStep {
                loop_: 0,
                step: 2,
                from: 0,
                old_step: 9,
                old_steps: 5,
            },
        ),
        (
            "a step of a new loop",
            vec![crossing.clone()],
            vec![LoopProvenance {
                from: None,
                steps: vec![None, Some(1), None, None, None],
            }],
            ProvenanceFault::StepOfNewLoop {
                loop_: 0,
                step: 1,
                old_step: 1,
            },
        ),
        (
            "an old loop continued twice",
            vec![crossing.clone(), hole.clone()],
            vec![
                full(Some(0)),
                LoopProvenance {
                    from: Some(0),
                    steps: vec![Some(0)],
                },
            ],
            ProvenanceFault::OldLoopContinuedTwice {
                from: 0,
                first: 0,
                again: 1,
            },
        ),
        (
            "an old step continued twice",
            vec![crossing.clone()],
            vec![LoopProvenance {
                from: Some(0),
                steps: vec![Some(0), Some(1), Some(1), Some(3), Some(4)],
            }],
            ProvenanceFault::OldStepContinuedTwice {
                loop_: 0,
                from: 0,
                old_step: 1,
                first: 1,
                again: 2,
            },
        ),
    ];
    for (what, loops, provenance, fault) in cases {
        assert_eq!(
            set_program(&doc, profile, loops, provenance).err(),
            Some(EditError::ProvenanceMalformed {
                node: profile,
                fault,
            }),
            "{what}"
        );
    }
    // The same crossing program under a well-shaped provenance is the
    // program's own refusal, and the document is still the one handed
    // in.
    match set_program(&doc, profile, vec![crossing], vec![full(Some(0))]) {
        Err(EditError::ProfileProgramRefused { node, .. }) => assert_eq!(node, profile),
        other => panic!("the bow tie refuses at the VQ9 door, got {other:?}"),
    }
    assert!(
        doc.bit_eq(&before),
        "a refused edit leaves the document as it was"
    );
}

/// **`SetProgram` aimed at what holds no program refuses typed.**
#[test]
fn a_node_that_holds_no_program_refuses() {
    let r = rod("set-program-non-profile", &[]);
    assert_eq!(
        set_program(&r.doc, r.rod, vec![rod_loop(false)], bump_provenance()).err(),
        Some(EditError::SetProgramOnNonProfile { node: r.rod })
    );
    assert_eq!(
        set_program(
            &r.doc,
            RecipeNodeId(99),
            vec![rod_loop(false)],
            bump_provenance()
        )
        .err(),
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
    let through_the_program = set_program(
        &doc,
        profile,
        vec![LoopProgram::Chain(steps)],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), Some(2), Some(3), Some(4)],
        }],
    )
    .expect_err("an undeclared parameter refuses at the program door");
    assert!(
        matches!(&through_the_slot, EditError::SlotUnknownDocParam { .. }),
        "{through_the_slot:?}"
    );
    assert_eq!(through_the_program, through_the_slot);
}

/// **The identity edit is accepted, reports nothing, and is
/// structural.** The program the node holds, under the provenance
/// that continues every loop and step into itself.
#[test]
fn the_identity_edit_is_accepted_and_reports_nothing() {
    let r = rod("set-program-identity", &[4]);
    let loops = vec![rod_loop(false)];
    let applied = accepted(
        &r.doc,
        r.profile,
        loops.clone(),
        LoopProvenance::identity(&loops),
    );
    assert!(applied.maintenance.is_empty());
    assert!(applied.record.structural);
    assert!(applied.doc.bit_eq(&r.doc), "nothing moved");
}

// ---------------------------------------------------------------- //
// Persistence, replay identity, DM8
// ---------------------------------------------------------------- //

/// The rod document as a LOG: the empty document plus every edit that
/// built it, the reshaping last.
fn rod_log() -> (ProfileDoc, Vec<LoggedEdit<ProfileProgram>>) {
    let plane = fixture::xy_frame();
    let mut edits = vec![DocEdit::InsertNode { node: plane }];
    let profile_node = RecipeNodeId(1);
    let rod_node = RecipeNodeId(2);
    edits.push(DocEdit::InsertNode {
        node: Node::Profile(ProfileProgram {
            plane: RecipeNodeId(0),
            loops: vec![rod_loop(false)],
        }),
    });
    edits.push(DocEdit::InsertNode {
        node: Node::Extrude {
            profile: profile_node,
            distance: len(ROD_L),
        },
    });
    edits.push(DocEdit::InsertNode {
        node: Node::fillet(rod_node, len(ROD_FILLET), vec![lateral_edge(rod_node, 4)]),
    });
    edits.push(DocEdit::SetProgram {
        node: profile_node,
        loops: vec![rod_loop(true)],
        provenance: bump_provenance(),
    });
    (
        ProfileDoc::empty_derived("set-program-log", tol()),
        LoggedEdit::bare_all(&edits),
    )
}

/// **A document whose log holds a `SetProgram` saves, loads and
/// replays identically** — the edit rides the unversioned format like
/// every other, the rewrite it performs is a function of the log, and
/// the loaded document is bit for bit the applied one, its fillet
/// selection rewritten.
#[test]
fn a_log_holding_a_set_program_saves_loads_and_replays_identically() {
    let (empty, log) = rod_log();
    let mut applied = empty.clone();
    for entry in &log {
        applied = editor_core::apply_logged(&applied, entry, tol())
            .expect("the log applies")
            .doc;
    }
    assert_eq!(
        selection_of(&applied, RecipeNodeId(3)),
        vec![lateral_edge(RecipeNodeId(2), 5)]
    );
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
/// loops in their own wire form, the provenance as plain integers
/// with `null` for new — pinned as bytes, since this is the first
/// `SetProgram` a file carries and its spelling is the format's
/// compatibility contract from here on. And what a build without the
/// variant does with those bytes: the tag is one such a build cannot
/// place, and the load door refuses `Unreadable`, which is the row
/// spelled by a tag THIS build cannot place either.
///
/// The refusal's `detail` does NOT name the tag: a log entry reads
/// through `LoggedEdit`'s untagged wrapper (bare edit, or edit with
/// rows), and serde reports an untagged miss as "data did not match
/// any variant of untagged enum Wire" at the entry's line, whichever
/// tag inside it was unknown. That is measured here and filed —
/// `work/edit/an-unknown-edit-tag-in-a-log-refuses-without-naming-it.md`
/// — rather than asserted away; what this row pins is the typed arm
/// and the line, which is the entry's.
#[test]
fn the_persisted_spelling_is_pinned_and_a_build_without_it_refuses_typed() {
    let edit: DocEdit<ProfileProgram> = DocEdit::SetProgram {
        node: RecipeNodeId(1),
        loops: vec![LoopProgram::circle(0.0, 0.0, 1.0).unwrap()],
        provenance: vec![LoopProvenance {
            from: Some(0),
            steps: vec![None],
        }],
    };
    let wire = serde_json::to_string(&LoggedEdit::bare(edit)).expect("serializes");
    assert_eq!(
        wire,
        r#"{"SetProgram":{"node":1,"loops":[{"Circle":{"centre":[{"Literal":{"value":0.0,"dim":"Length","unit":"m"}},{"Literal":{"value":0.0,"dim":"Length","unit":"m"}}],"radius":{"Literal":{"value":1.0,"dim":"Length","unit":"m"}}}}],"provenance":[{"from":0,"steps":[null]}]}}"#
    );

    let (empty, log) = rod_log();
    let text = save(&empty, &log, tol()).expect("saves");
    assert!(text.contains("\"SetProgram\""), "the file carries the tag");
    let older = text.replace("\"SetProgram\"", "\"SetProgramme\"");
    let entry_line = older
        .lines()
        .position(|l| l.contains("\"SetProgramme\""))
        .expect("the mutated tag is in the file")
        + 1;
    match load(&older, tol()) {
        Err(PersistError::Unreadable { line, detail, .. }) => {
            assert!(
                line >= entry_line,
                "the refusal sits at or after the entry's line {entry_line}: {line} ({detail})"
            );
            assert!(
                detail.contains("untagged enum"),
                "the log wrapper's own miss, the tag unnamed (filed): {detail}"
            );
        }
        other => panic!("a tag this build lacks refuses Unreadable, got {other:?}"),
    }
}

/// **DM8 is unchanged: the step map on the reshaped profile answers
/// the NEW program's record**, and the rebound names agree with it —
/// the wall the arc step draws under the new program is the wall the
/// rebound name spells.
#[test]
fn the_step_map_on_the_reshaped_profile_answers_the_new_programs_record() {
    let r = rod("set-program-dm8", &[]);
    let (doc, on_arc) = frame_on(r.doc, r.rod, wall(r.rod, 3));
    let applied = accepted(&doc, r.profile, vec![rod_loop(true)], bump_provenance());
    let program = match applied.doc.node(r.profile) {
        Some(Node::Profile(p)) => p.clone(),
        other => panic!("a profile, got {other:?}"),
    };
    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    let naming = match &ev.value(r.profile).expect("the profile evaluates").payload {
        editor_core::ValuePayload::Profile(pv) => pv.naming.clone(),
        other => panic!("a profile value, got {}", other.kind_name()),
    };
    // The records, rebuilt through the same public doors the
    // evaluation's pre-pass runs (the door's own two-record check
    // holds the rebuild honest against the published anchor).
    let env = applied.doc.param_env::<f64>();
    let (loops, replay) = program.replay_records(&env, tol()).expect("replays");
    let (_, canonical) = profile::Profile::new(profile::SketchPlane::xy(), loops)
        .validate_recording(tol())
        .expect("validates");
    let structure = profile::ProfileStructure { replay, canonical };
    // The arc is new step 6 and draws new segment 4.
    let edges = program
        .profile_edges_of(&structure, &naming, 0, 6)
        .expect("the record describes the new program");
    assert_eq!(
        edges,
        vec![ProfileEdgeRef {
            loop_index: 0,
            segment: 4
        }]
    );
    match applied.doc.node(on_arc) {
        Some(Node::Datum(Datum::FaceFrame { face, .. })) => {
            assert_eq!(face, &fname(r.rod, RoleSeg::Lateral(edges[0])));
        }
        other => panic!("the frame carries the arc's wall, got {other:?}"),
    }
}

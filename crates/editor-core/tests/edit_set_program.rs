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
    LoggedEdit, LoopProgram, LoopProvenance, Maintenance, MeasureExpr, MeasurePrimitive, NameRef,
    Node, NodeErrorKind, NodeResult, ParamName, PersistError, ProfileDoc, ProfileEdgeRef,
    ProfileProgram, ProfileVertexRef, ProgramArcData, ProgramStep, ProgramTarget, ProvenanceFault,
    RETIRED_FLOOR, RecipeNodeId, Resolution, ResolveError, Rgba8, RoleSeg, RunCtx, SitedRef,
    SlotId, StableName, StepArg, apply, band, load, resolve, save,
};
use fixture::{ang, edge_of, ends, fname, insert, len, minted, point, scl, table, tol};
use sweep::test_support::{ROD_FILLET, ROD_FLAT, ROD_L, rod_chord_at};

// ---------------------------------------------------------------- //
// The fixture: the sunk rod, and the leg the reshaping inserts
// ---------------------------------------------------------------- //

// The rod's loop, its bump and its provenance are the corpus
// document's (`reshaped_rod`, the first persisted `SetProgram`): one
// spelling, so the rows here measure the program the corpus replays.
use crate::corpus::reshaped_rod::{BUMP, bump_provenance, lateral_edge, rod_loop};

/// The area the bump adds to the section — the triangle
/// `(1, −1), (1.25, −0.5), (1, 0)` — so the reshaped rod's volume is
/// the old one's plus this times the length.
const BUMP_AREA: f64 = 0.125;

/// [`bump_provenance`] with the ARC step (old step 4, new step 6)
/// stated as new — the reshaping that keeps the arc's segment in the
/// program and drops the step that drew it.
fn bump_provenance_without_the_arc() -> Vec<LoopProvenance> {
    let mut p = bump_provenance();
    p[0].steps[6] = None;
    p
}

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
/// is retired to the floor — vertex 4 becomes vertex
/// `RETIRED_FLOOR + 4`, which no loop draws — reported `Strand` with
/// that spelling, and at the next evaluation the fillet refuses `Vanished`
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
    let retired = lateral_edge(r.rod, RETIRED_FLOOR + 4);
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
/// it drew strands, retired to the floor, while the wall of the
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
    // Retired at the floor plus its old segment, whatever the loop
    // now draws (five segments: the filleted corner is two vertices).
    let retired = wall(ext, RETIRED_FLOOR + 1);
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
/// hole strands the hole's name, retired onto loop `RETIRED_FLOOR +
/// 1` — no loop continues it, so it is filed by its OLD loop index at
/// the floor — and leaves the square's alone. Swapping the two
/// loops' AUTHORED order rebinds nothing: names index canonical loops,
/// and the canonical order is the outer first whatever order the
/// author writes the loops in.
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
            name: wall_of(ext, RETIRED_FLOOR + 1, 0),
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
        Vec::new(),
        "the canonical loop order is the outer first either way, so no name moves"
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
    // The old program does not replay, so neither its spans nor its
    // canonical numbering can be read — not even which of its loops the
    // name's canonical loop 1 is — and the retired coordinate is filed
    // by the name's own coordinates: loop `RETIRED_FLOOR + 1`,
    // segment 0.
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Strand {
            node: on_hole,
            name: wall_of(ext, RETIRED_FLOOR + 1, 0),
        }],
        "one strand at the retired spelling and nothing else"
    );
}

/// **A program that replays but does not validate still reads its
/// names.** The names on it were published by an evaluation that
/// validated, and the canonical form keeps each loop's authored start,
/// so a name's canonical locator needs only each loop's canonical
/// position and sense — both read off the replay alone. The hole is
/// authored FIRST and counter-clockwise, so both facts matter: its
/// canonical loop is 1 (the square is outer) and it is reversed. Its
/// radius is driven to 1.05, past the square's walls 1 away, so the
/// profile replays and refuses validation (the hole crosses the outer
/// loop) while the square still encloses the larger area; the
/// program is then replaced by one whose radius is a literal, under
/// the identity provenance. Every name is kept where it was — no
/// strand and no rebind — where reading the names without an anchor
/// would strand all three, and a wrong order or sense would rebind
/// them.
#[test]
fn a_program_that_replays_but_does_not_validate_keeps_its_names() {
    let square = LoopProgram::Chain(square_steps());
    let radius = ParamName::new("hole_r");
    let driven = LoopProgram::Circle {
        centre: [len(1.0), len(1.0)],
        radius: Expr::param(radius.clone(), Dimension::Length),
    };
    let doc = ProfileDoc::empty_derived("set-program-unvalidated", tol());
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
            loops: vec![driven, square.clone()],
        }),
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, _) = frame_on(doc, ext, wall_of(ext, 1, 0));
    let (doc, _) = frame_on(doc, ext, wall_of(ext, 1, 1));
    let (doc, _) = frame_on(doc, ext, wall_of(ext, 0, 2));
    let square_wall = face_origin(&doc, ext, &wall_of(ext, 0, 2));
    let (doc, _) = fixture::step(
        doc,
        DocEdit::SetDocParam {
            name: radius,
            value: DocParam::continuous(Dimension::Length, 1.05),
        },
    );
    let applied = accepted(
        &doc,
        profile,
        vec![LoopProgram::circle(1.0, 1.0, 0.3).unwrap(), square],
        vec![
            LoopProvenance {
                from: Some(0),
                steps: vec![Some(0)],
            },
            LoopProvenance {
                from: Some(1),
                steps: vec![Some(0), Some(1), Some(2), Some(3), Some(4)],
            },
        ],
    );
    assert_eq!(
        applied.maintenance,
        Vec::new(),
        "every name is read through the replay's anchor and kept where it was"
    );
    assert_eq!(
        face_origin(&applied.doc, ext, &wall_of(ext, 0, 2)),
        square_wall,
        "the square's canonical wall 2 is the wall it was before the radius moved"
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
    let retired = wall(r.rod, RETIRED_FLOOR + 3);
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
    let retired = wall(r.rod, RETIRED_FLOOR + 3);
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
/// The refusal's `detail` NAMES the tag, and its line is the entry's
/// `edit` key — the line before the tag's, inside the entry's braces
/// — measured: the log has one wire shape (`{"edit": …,
/// "maintenance": […]}`), so the miss serde reports is `DocEdit`'s
/// own "unknown variant `SetProgramme`". A log entry used to read
/// through an untagged wrapper whose miss named no tag
/// (`work/edit/an-unknown-edit-tag-in-a-log-refuses-without-naming-it.md`,
/// closed when the wrapper went); this row pins the typed arm, the
/// line and the named tag, so a wrapper that swallowed the name again
/// would red here.
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
        r#"{"edit":{"SetProgram":{"node":1,"loops":[{"Circle":{"centre":[{"Literal":{"value":0.0,"dim":"Length","unit":"m"}},{"Literal":{"value":0.0,"dim":"Length","unit":"m"}}],"radius":{"Literal":{"value":1.0,"dim":"Length","unit":"m"}}}}],"provenance":[{"from":0,"steps":[null]}]}},"maintenance":[]}"#
    );

    let (empty, mut log) = rod_log();
    // An entry AFTER the bad one, so the end of the bad entry and the
    // end of the log are different lines and a refusal that drifted to
    // the file's end would be visible.
    log.push(LoggedEdit::bare(DocEdit::InsertNode {
        node: fixture::xy_frame(),
    }));
    let text = save(&empty, &log, tol()).expect("saves");
    assert!(text.contains("\"SetProgram\""), "the file carries the tag");
    let older = text.replace("\"SetProgram\"", "\"SetProgramme\"");
    let tag_line = older
        .lines()
        .position(|l| l.contains("\"SetProgramme\""))
        .expect("the mutated tag is in the file")
        + 1;
    // The bad entry runs from the `{` that opens it to the line before
    // the next entry opens.
    let opens = older
        .lines()
        .enumerate()
        .filter(|(i, l)| i + 1 < tag_line && *l == "    {")
        .map(|(i, _)| i + 1)
        .last()
        .expect("the bad entry opens");
    let next = older
        .lines()
        .enumerate()
        .position(|(i, l)| i + 1 > tag_line && l == "    {")
        .expect("the entry after the bad one opens")
        + 1;
    match load(&older, tol()) {
        Err(PersistError::Unreadable { line, detail, .. }) => {
            assert!(
                opens <= line && line < next,
                "the refusal sits inside the bad entry's span {opens}..{next}: {line} ({detail})"
            );
            assert_eq!(
                line,
                tag_line - 1,
                "measured: the line is the entry's `edit` key, the line before the tag's — the \
                 index of the entry is recoverable from it ({detail})"
            );
            assert!(
                detail.contains("unknown variant `SetProgramme`"),
                "the refusal names the tag this build lacks: {detail}"
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

// ---------------------------------------------------------------- //
// The retired coordinate: the floor, and what keeps it dead
// ---------------------------------------------------------------- //

/// The face a derived frame carries, as the document holds it.
fn frame_face(doc: &ProfileDoc, frame: RecipeNodeId) -> StableName {
    match doc.node(frame) {
        Some(Node::Datum(Datum::FaceFrame { face, .. })) => face.clone(),
        other => panic!("a frame, got {other:?}"),
    }
}

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

/// How many segments loop 0 draws, counted off the extrude's published
/// name table.
fn drawn_segments(doc: &ProfileDoc, ext: RecipeNodeId) -> u32 {
    let ev = fixture::run(doc, &EvalOptions::default());
    let t = table(&ev, ext);
    let mut n = 0;
    while t.lookup(&wall_of(ext, 0, n)).is_some() {
        n += 1;
    }
    n
}

/// The sorted `(x, y, z)` corners of the face `name` denotes on the
/// evaluated `node` — what a name DENOTES, read off the solid rather
/// than off an index.
fn corners_of(doc: &ProfileDoc, node: RecipeNodeId, name: &StableName) -> Vec<(f64, f64, f64)> {
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

/// **A retired name stays dead when a SLOT edit grows the loop.** A
/// loop's segment count is a function of its ARGUMENTS too: at `r = 2`
/// the corner fillet's two runs have a `Zero` fit and emit nothing
/// (three segments), at `r = 0.3` they emit (five). A retired name
/// spelled one past the OLD end would go live under a plain
/// `DocEdit::SetParam` on that radius — the door reports nothing for a
/// slot edit, and the retired name would silently denote the segment
/// drawn at its coordinate, the DI1 aliasing class the retirement
/// exists to end. Spelled at `RETIRED_FLOOR` it is past every
/// coordinate a program can draw, under this edit and every other.
#[test]
fn a_retired_name_stays_dead_when_a_slot_edit_grows_the_loop() {
    let (doc, profile, ext) = extruded("set-program-retire-grow", vec![filleted_square(2.0)]);
    let n_tight = drawn_segments(&doc, ext);
    assert_eq!(n_tight, 3, "at r = 2 both runs fit Zero and emit nothing");
    let (doc, frame) = frame_on(doc, ext, wall_of(ext, 0, 0));
    let applied = accepted(
        &doc,
        profile,
        vec![filleted_square(2.0)],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), Some(2), Some(3), None, Some(5), Some(6)],
        }],
    );
    let retired = frame_face(&applied.doc, frame);
    assert_eq!(
        retired,
        wall_of(ext, 0, RETIRED_FLOOR),
        "retired at the floor"
    );
    let grown = apply(
        &applied.doc,
        &DocEdit::SetParam {
            node: profile,
            slot: SlotId::Profile {
                loop_: 0,
                step: 2,
                arg: StepArg::Radius,
            },
            expr: len(0.3),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the radius is a legal slot write");
    assert_eq!(grown.maintenance, vec![], "the slot edit reports nothing");
    let n_loose = drawn_segments(&grown.doc, ext);
    assert_eq!(n_loose, 5, "the loop grew under the slot edit");
    let ev = fixture::run(&grown.doc, &EvalOptions::default());
    assert!(
        table(&ev, ext).lookup(&retired).is_none(),
        "the retired name is not drawn by the grown loop"
    );
    match ev.nodes.get(&frame) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::FaceFrameResolve { error } => match error.as_ref() {
                ResolveError::Vanished { name, .. } => assert_eq!(name, &retired),
                other => panic!("the retired name resolves to nothing: got {other:?}"),
            },
            other => panic!("the frame refuses on its face, got {other:?}"),
        },
        other => panic!("the frame refuses, got {other:?}"),
    }
}

/// The bumped rod plus six extra legs zig-zagging along the bottom
/// edge: thirteen segments, so a coordinate one past the seven-segment
/// loop's end would be a live corner here.
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

/// **A retired name is untouched and unreported by a later reshaping
/// that grows the loop past its old coordinate.** Strand the fillet's
/// crease (vertex 4 of a seven-segment loop, retired to
/// `RETIRED_FLOOR + 4`), then reshape AGAIN into a thirteen-segment
/// loop. The name keeps its retired spelling exactly, the second edit
/// reports NOTHING about it — DM7's clause is the referent THE EDIT
/// removed, and this name lost its referent at the first edit — and
/// the fillet still refuses `Vanished` on it.
#[test]
fn a_retired_name_is_untouched_and_unreported_by_a_later_reshaping_that_grows_past_it() {
    let r = rod("set-program-retire-again", &[4]);
    let fillet = r.fillet.unwrap();
    let first = accepted(
        &r.doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired = lateral_edge(r.rod, RETIRED_FLOOR + 4);
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
    assert_eq!(
        selection_of(&second.doc, fillet),
        vec![retired.clone()],
        "the retired spelling is left exactly as it was"
    );
    assert_eq!(
        second.maintenance,
        vec![],
        "an already-retired name is not a strand of the edit that grew past it"
    );
    fillet_refuses_vanished(&second.doc, fillet, &retired);
}

/// **A retired name round-trips the wire and is repaired by `Rebind`
/// from its retired spelling.** The load door admits a locator at the
/// floor in a snapshot and replays one through a logged `SetProgram`;
/// `Rebind` from the retired spelling to the crease's live name
/// repairs the fillet, measured on the solid.
#[test]
fn a_retired_name_round_trips_the_wire_and_rebinds() {
    let r = rod("set-program-retire-wire", &[4]);
    let fillet = r.fillet.unwrap();
    let stranded = accepted(
        &r.doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired = lateral_edge(r.rod, RETIRED_FLOOR + 4);
    let text = save(&stranded.doc, &[], tol()).expect("saves");
    let loaded = load(&text, tol()).expect("a retired locator loads");
    assert!(loaded.doc.bit_eq(&stranded.doc));
    assert_eq!(selection_of(&loaded.doc, fillet), vec![retired.clone()]);
    let log = vec![LoggedEdit::bare(DocEdit::SetProgram {
        node: r.profile,
        loops: vec![rod_loop(true)],
        provenance: bump_provenance_without_the_arc(),
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

/// **A retired name through the resolver door answers `Vanished` and
/// never panics** — the N5 ladder does no arithmetic on a locator's
/// index, so a coordinate at the floor, and one at `u32::MAX` on a
/// loop at `u32::MAX`, are looked up and refused typed exactly as any
/// name the table does not hold.
#[test]
fn a_retired_name_resolves_vanished_through_the_resolver_door() {
    let r = rod("set-program-retire-resolve", &[]);
    let (doc, _on_arc) = frame_on(r.doc, r.rod, wall(r.rod, 3));
    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired = wall(r.rod, RETIRED_FLOOR + 3);
    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    let ctx = RunCtx {
        doc: &applied.doc,
        eval: &ev,
    };
    let vanished = |name: &StableName| match resolve(ctx, name) {
        Resolution::Failed(failure) => match failure.error {
            ResolveError::Vanished { name: got, .. } => assert_eq!(&got, name),
            other => panic!("a name nothing draws is Vanished, got {other:?}"),
        },
        other => panic!("a name nothing draws fails to resolve, got {other:?}"),
    };
    vanished(&retired);
    vanished(&wall_of(r.rod, u32::MAX, u32::MAX));
    vanished(&lateral_edge(r.rod, u32::MAX));
}

/// **A frame on a wall a reshaping dropped refuses `Vanished` at
/// evaluation rather than evaluating on the plane the new program
/// draws at its old index** — the edge twin of the fillet rows'
/// `Vanished`. The block's right wall is segment 1, drawn by old step
/// 2 (`LineTo(1, 0)`); the bump inserted before it with THAT step
/// stated as new drops it, and the new program draws a wall AT index
/// 1 (the bump's first leg, a different plane), so a name left in
/// place would have re-anchored the frame to that plane silently.
#[test]
fn a_frame_on_a_wall_a_reshaping_dropped_refuses_vanished_at_evaluation() {
    let r = rod("set-program-frame-dropped", &[]);
    let (doc, frame) = frame_on(r.doc, r.rod, wall(r.rod, 1));
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(ev.value(frame).is_some(), "{:?}", corpus::failures(&ev));
    let mut without_the_right_wall = bump_provenance();
    without_the_right_wall[0].steps[4] = None;
    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(true)],
        without_the_right_wall,
    );
    let retired = wall(r.rod, RETIRED_FLOOR + 1);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Strand {
            node: frame,
            name: retired.clone(),
        }]
    );
    let ev = fixture::run(&applied.doc, &EvalOptions::default());
    assert!(
        table(&ev, r.rod).lookup(&wall(r.rod, 1)).is_some(),
        "the new program draws a wall at the old index"
    );
    match ev.nodes.get(&frame) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::FaceFrameResolve { error } => match error.as_ref() {
                ResolveError::Vanished { name, .. } => assert_eq!(name, &retired),
                other => panic!("the retired name resolves to nothing: got {other:?}"),
            },
            other => panic!("the frame refuses on its face, got {other:?}"),
        },
        other => panic!("the frame refuses rather than re-anchoring, got {other:?}"),
    }
}

// ---------------------------------------------------------------- //
// Which profile a solid's names are spelled in: `anchoring_profile`
// tied to what the evaluation publishes
// ---------------------------------------------------------------- //

/// The unit square with a leg `(3, 1)` inserted between `(2, 0)` and
/// `(2, 2)`, and the provenance that continues every old step.
fn square_with_a_leg() -> (LoopProgram, Vec<LoopProvenance>) {
    (
        LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0), (0.0, 2.0)]).unwrap(),
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
        }],
    )
}

/// **An extrude's names carry its profile's coordinates**, and that
/// profile is [`Node::anchoring_profile`]'s answer: a paint on wall 1
/// (`(2, 0) → (2, 2)`) follows the leg inserted before it to wall 2,
/// and the wall the rebound name denotes on the solid still arrives at
/// `(2, 2)`.
#[test]
fn an_extrudes_names_carry_its_profiles_coordinates() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let (doc, profile, ext) = extruded("set-program-anchor-extrude", vec![square]);
    assert_eq!(
        doc.node(ext).unwrap().anchoring_profile(),
        Some(profile),
        "the extrude anchors to its profile"
    );
    let before = corners_of(&doc, ext, &wall_of(ext, 0, 1));
    assert!(has_corner3(&before, (2.0, 2.0, 0.0)), "{before:?}");
    let doc = paint(&doc, &wall_of(ext, 0, 1));
    let (leg, provenance) = square_with_a_leg();
    let applied = accepted(&doc, profile, vec![leg], provenance);
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Rebound {
            from: wall_of(ext, 0, 1),
            to: wall_of(ext, 0, 2),
        }]
    );
    let after = corners_of(&applied.doc, ext, &wall_of(ext, 0, 2));
    assert!(
        has_corner3(&after, (2.0, 2.0, 0.0)) && has_corner3(&after, (3.0, 1.0, 0.0)),
        "the rebound name denotes the wall arriving at (2, 2): {after:?}"
    );
}

/// **A revolve's names carry its profile's coordinates**: a square
/// standing off the axis, turned a quarter turn about the sketch's
/// `+y`; the band drawn by segment 1 (`(2, 0) → (2, 1)`, the outer
/// cylinder) follows the leg inserted before it, and the band the
/// rebound name denotes still has the corner `(2, 1)` on the sketch
/// plane.
#[test]
fn a_revolves_names_carry_its_profiles_coordinates() {
    let doc = ProfileDoc::empty_derived("set-program-anchor-revolve", tol());
    let (doc, plane, profile) = fixture::on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    );
    let (doc, axis) = insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(std::f64::consts::FRAC_PI_2),
        },
    );
    assert_eq!(doc.node(rev).unwrap().anchoring_profile(), Some(profile));
    let before = corners_of(&doc, rev, &band(rev, 0, 1));
    assert!(has_corner3(&before, (2.0, 1.0, 0.0)), "{before:?}");
    let doc = paint(&doc, &band(rev, 0, 1));
    let leg =
        LoopProgram::polygon([(1.0, 0.0), (2.0, 0.0), (3.0, 0.5), (2.0, 1.0), (1.0, 1.0)]).unwrap();
    let applied = accepted(
        &doc,
        profile,
        vec![leg],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
        }],
    );
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Rebound {
            from: band(rev, 0, 1),
            to: band(rev, 0, 2),
        }]
    );
    let after = corners_of(&applied.doc, rev, &band(rev, 0, 2));
    assert!(
        has_corner3(&after, (2.0, 1.0, 0.0)) && has_corner3(&after, (3.0, 0.5, 0.0)),
        "the rebound name denotes the band arriving at (2, 1): {after:?}"
    );
}

/// **A loft's names follow its FIRST section's reshaping.** A loft
/// wall's one name is canonical segment `k` of every section at once
/// (DM8), and only the first section is [`Node::anchoring_profile`]'s
/// answer, so only a SetProgram on it moves the loft's names
/// (`work/emit/a-lofts-names-follow-only-its-first-sections-reshaping.md`). Two DIFFERING sections — the upper one scaled — skin a
/// loft; reshaping the second section moves none of the loft's names
/// and leaves the sections with different segment counts, so the
/// loft refuses rather than re-skinning under unchanged names;
/// reshaping the first section the same way rebinds the painted wall,
/// and the loft skins again with the rebound name at the wall that
/// arrives at `(2, 2, 0)`.
#[test]
fn a_lofts_names_follow_its_first_sections_reshaping() {
    let square = |s: f64| {
        LoopProgram::polygon([
            (0.0, 0.0),
            (2.0 * s, 0.0),
            (2.0 * s, 2.0 * s),
            (0.0, 2.0 * s),
        ])
        .unwrap()
    };
    let leg = |s: f64| {
        LoopProgram::polygon([
            (0.0, 0.0),
            (2.0 * s, 0.0),
            (3.0 * s, 1.0 * s),
            (2.0 * s, 2.0 * s),
            (0.0, 2.0 * s),
        ])
        .unwrap()
    };
    let (_, provenance) = square_with_a_leg();
    let doc = ProfileDoc::empty_derived("set-program-anchor-loft", tol());
    let (doc, p0) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, sec0) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: p0,
            loops: vec![square(1.0)],
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
            loops: vec![square(1.5)],
        }),
    );
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![sec0, sec1],
            v_degree: Expr::count(1),
        },
    );
    assert_eq!(doc.node(loft).unwrap().anchoring_profile(), Some(sec0));
    let before = corners_of(&doc, loft, &wall_of(loft, 0, 1));
    assert!(has_corner3(&before, (2.0, 2.0, 0.0)), "{before:?}");
    let doc = paint(&doc, &wall_of(loft, 0, 1));

    let upper = accepted(&doc, sec1, vec![leg(1.5)], provenance.clone());
    assert_eq!(
        upper.maintenance,
        vec![],
        "the loft's locators are section 0's and this reshaped section 1"
    );
    let ev = fixture::run(&upper.doc, &EvalOptions::default());
    assert!(
        ev.value(loft).is_none(),
        "sections of different segment counts do not skin"
    );

    let both = accepted(&upper.doc, sec0, vec![leg(1.0)], provenance);
    assert_eq!(
        both.maintenance,
        vec![Maintenance::Rebound {
            from: wall_of(loft, 0, 1),
            to: wall_of(loft, 0, 2),
        }]
    );
    let after = corners_of(&both.doc, loft, &wall_of(loft, 0, 2));
    assert!(
        has_corner3(&after, (2.0, 2.0, 0.0)) && has_corner3(&after, (3.0, 1.0, 0.0)),
        "the rebound name denotes the wall arriving at (2, 2): {after:?}"
    );
}

/// **A sweep publishes no name today, so a reshaping of its profile
/// moves none** — [`Node::anchoring_profile`] answers the sweep's
/// profile for the day its frontier lowers, and this row is what reds
/// that day: the sweep refuses at its frontier, mints nothing, and a
/// leg inserted into its profile or its path reports nothing. When a
/// sweep evaluates, the author who lowers it decides which section
/// its names are spelled in and moves this row with it.
#[test]
fn a_sweep_publishes_no_name_today_so_a_reshaping_of_its_profile_moves_none() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let doc = ProfileDoc::empty_derived("set-program-anchor-sweep", tol());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![square.clone()],
        }),
    );
    let (doc, path) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![square],
        }),
    );
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile,
            path,
            stations: Expr::count(4),
            v_degree: Expr::count(2),
        },
    );
    assert_eq!(doc.node(sweep).unwrap().anchoring_profile(), Some(profile));
    let ev = fixture::run(&doc, &EvalOptions::default());
    match ev.nodes.get(&sweep) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(e.kind, NodeErrorKind::CurvedSolidFrontier { .. }),
            "the sweep's frontier, got {:?}",
            e.kind
        ),
        other => panic!("the sweep publishes no value today, got {other:?}"),
    }
    let (leg, provenance) = square_with_a_leg();
    for node in [profile, path] {
        let applied = accepted(&doc, node, vec![leg.clone()], provenance.clone());
        assert_eq!(
            applied.maintenance,
            vec![],
            "no name of the sweep exists to move"
        );
    }
}

// ---------------------------------------------------------------- //
// The loop's seams, measured on the solid
// ---------------------------------------------------------------- //

/// Where a bump is inserted into the rod's loop: after the first leg
/// (the corpus's bump), at the very start, or as the last leg before
/// the close.
#[derive(Clone, Copy, PartialEq, Eq)]
enum At {
    Start,
    BeforeClose,
}

/// The sunk rod's loop with a bump at `at` — two steps drawing one
/// segment, as the corpus's bump is authored.
fn rod_loop_at(at: At) -> LoopProgram {
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
    steps.extend([
        ProgramStep::LineTo(ProgramTarget::Point(pt(1.0, -1.0))),
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

/// Provenance of [`rod_loop_at`] over the plain rod: every old step
/// continues, the two bump steps are new.
fn provenance_at(at: At) -> Vec<LoopProvenance> {
    let mut steps: Vec<Option<u32>> = (0..7).map(Some).collect();
    let insert_at = match at {
        At::Start => 1,
        At::BeforeClose => 6,
    };
    steps.insert(insert_at, None);
    steps.insert(insert_at, None);
    vec![LoopProvenance {
        from: Some(0),
        steps,
    }]
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
    let pose = topo::readback::face_pose(body, fixture::face_of(table(&ev, node), "wall", name))
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
    let mut out: Vec<(f64, f64)> = corners_of(doc, node, name)
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect();
    out.dedup_by(|a, b| near(*a, *b));
    out
}

fn has_corner(corners: &[(f64, f64)], want: (f64, f64)) -> bool {
    corners.iter().any(|&c| near(c, want))
}

/// **A leg inserted at the loop's START (before the first drawing
/// step)**: vertex 0 is the end of the closing segment, which is kept,
/// so vertex 0 stays vertex 0; wall 0 becomes wall 1; the last vertex
/// 5 becomes 6; the closing wall 5 becomes 6. Measured on the solid,
/// with a cap vertex and a strut on vertex 0.
#[test]
fn a_leg_inserted_at_the_loops_start_keeps_vertex_zero_and_moves_the_rest() {
    let r = rod("set-program-seam-start", &[]);
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
    let doc = paint(&doc, &wall(r.rod, 0));
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
        vec![rod_loop_at(At::Start)],
        provenance_at(At::Start),
    );
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

/// **A leg inserted as the LAST leg before the close**: the closing
/// wall 5 becomes 6, vertex 5 stays (its arriving segment 4 is kept
/// in place), vertex 0 stays (the close's end).
#[test]
fn a_leg_inserted_before_the_close_moves_only_the_closing_wall() {
    let r = rod("set-program-seam-close", &[]);
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
        vec![rod_loop_at(At::BeforeClose)],
        provenance_at(At::BeforeClose),
    );
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

/// **Names on a CLOCKWISE loop move in CANONICAL coordinates.** The
/// square authored clockwise from `(2, 2)`: canonicalization reverses
/// it and keeps its start, so canonical segment `k` is program segment
/// `n − 1 − k`. A frame on canonical wall 2 (program wall 1,
/// `(2,0)→(0,0)`) and one on canonical wall 3 (program wall 0,
/// `(2,2)→(2,0)`); a leg inserted before program wall 1's step.
///
/// Read through the two programs' anchors, program wall 1 becomes
/// program wall 2 of five, which is canonical 5 − 1 − 2 = 2 again: the
/// name stays and the wall it denotes now starts at the inserted
/// point. Program wall 0 stays program wall 0, which is canonical 4
/// now: the name moves, onto the same plane.
#[test]
fn names_on_a_clockwise_loop_move_in_canonical_coordinates() {
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
    let (doc, profile, ext) = extruded("set-program-cw", vec![cw(false)]);
    let (doc, _f2) = frame_on(doc, ext, wall_of(ext, 0, 2));
    let (doc, _f3) = frame_on(doc, ext, wall_of(ext, 0, 3));
    let w2 = face_origin(&doc, ext, &wall_of(ext, 0, 2));
    let w3 = face_origin(&doc, ext, &wall_of(ext, 0, 3));
    let applied = accepted(
        &doc,
        profile,
        vec![cw(true)],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), Some(3), Some(4)],
        }],
    );
    assert_eq!(
        applied.maintenance,
        vec![Maintenance::Rebound {
            from: wall_of(ext, 0, 3),
            to: wall_of(ext, 0, 4),
        }]
    );
    // Canonical wall 2 is still old program wall 1's image: it now
    // starts at the inserted (1,-0.5) and still arrives at (0,0).
    let c = wall_corners(&applied.doc, ext, &wall_of(ext, 0, 2));
    assert!(
        has_corner(&c, (0.0, 0.0)) && has_corner(&c, (1.0, -0.5)),
        "{c:?}"
    );
    assert_ne!(face_origin(&applied.doc, ext, &wall_of(ext, 0, 2)), w2);
    assert_eq!(face_origin(&applied.doc, ext, &wall_of(ext, 0, 4)), w3);
}

/// Every name minted by `node` that `name` carries inside its path,
/// read off the name's serialized form — a test's own walk, so it
/// cannot share the crate's rewrite walk's answer.
fn carried_names_of(name: &StableName, node: RecipeNodeId) -> Vec<StableName> {
    fn visit(v: &serde_json::Value, node: RecipeNodeId, out: &mut Vec<StableName>) {
        match v {
            serde_json::Value::Object(map) => {
                if map.contains_key("kind")
                    && map.contains_key("node")
                    && map.contains_key("path")
                    && let Ok(inner) = serde_json::from_value::<StableName>(v.clone())
                    && inner.node == node
                {
                    out.push(inner);
                }
                for child in map.values() {
                    visit(child, node, out);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    visit(item, node, out);
                }
            }
            _ => {}
        }
    }
    let json = serde_json::to_value(name).expect("a name serializes");
    let mut out = Vec::new();
    // The name itself is minted by another node; only what it CARRIES
    // is asked for.
    if let serde_json::Value::Object(map) = &json
        && let Some(path) = map.get("path")
    {
        visit(path, node, &mut out);
    }
    out
}

/// **A name carried inside a shell's table keeps its operand's
/// PROGRAM spelling over a reversed loop.** The square authored
/// clockwise, so canonicalization reverses it and the anchor rewrite
/// is not the identity; a shell opened at wall 1 publishes names that
/// carry the extrude's names (its rims, its inner walls, its
/// survivors), and every carried name is one the extrude's own table
/// holds — the spelling `eval::anchor` published, never re-anchored
/// a second time on the way through a downstream op.
///
/// What this row does NOT hold, measured: whether the anchor's own
/// rewriter descends into a carried name. `remap_table` runs only on
/// the sweep emitters' tables, which carry no names, so a rewriter
/// that descended is today indistinguishable from one that does not
/// (the mutant passes this row and every other). The row pins the
/// invariant a re-anchoring of a downstream table would break, which
/// is the day the descent question becomes observable.
#[test]
fn a_name_carried_inside_a_shells_table_keeps_its_operands_program_spelling_over_a_reversed_loop() {
    let pt = |x: f64, y: f64| [len(x), len(y)];
    let cw = LoopProgram::Chain(vec![
        ProgramStep::At(pt(2.0, 2.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, _profile, ext) = extruded("set-program-carried-cw", vec![cw]);
    let mouth = wall_of(ext, 0, 1);
    let (doc, shell) = insert(doc, Node::shell(ext, len(0.1), vec![mouth.clone()]));
    let ev = fixture::run(&doc, &EvalOptions::default());
    assert!(ev.value(shell).is_some(), "{:?}", corpus::failures(&ev));
    let ext_table = table(&ev, ext);
    assert!(
        ext_table.lookup(&mouth).is_some(),
        "the open list names a published wall"
    );
    let mut carried = 0usize;
    for (name, _) in table(&ev, shell).iter() {
        for inner in carried_names_of(name, ext) {
            carried += 1;
            assert!(
                ext_table.lookup(&inner).is_some(),
                "{name} carries {inner}, which the extrude never published"
            );
        }
    }
    assert!(carried > 0, "the shell's table carries the extrude's names");
}

// ---------------------------------------------------------------- //
// Every other holder kind, and the refusal order
// ---------------------------------------------------------------- //

/// **Every holder kind the finding's rows lack — chamfer, shell open
/// list, measure refs, declare pairs — is rebound and stranded, in
/// the contract's order**, each stranded name at its retired
/// spelling.
#[test]
fn every_other_holder_kind_is_rebound_and_stranded_in_the_contracts_order() {
    let r = rod("set-program-holders", &[]);
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
    let applied = accepted(
        &doc,
        r.profile,
        vec![rod_loop(true)],
        bump_provenance_without_the_arc(),
    );
    let retired_wall = wall(r.rod, RETIRED_FLOOR + 3);
    let retired_vertex = lateral_edge(r.rod, RETIRED_FLOOR + 4);
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

/// **A shape fault wins over an undeclared parameter**: the provenance
/// is read before the program, so a program that would ALSO refuse
/// its parameter references refuses the provenance's fault, and the
/// document is untouched.
#[test]
fn a_shape_fault_wins_over_an_undeclared_parameter() {
    let r = rod("set-program-fault-param", &[]);
    let nope = Expr::param(ParamName::new("nope"), Dimension::Length);
    let LoopProgram::Chain(mut steps) = rod_loop(false) else {
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

/// **The Python rows' fixture, measured in Rust**: a square prism, a
/// fillet on the rim edge wall 2 shares with the end cap, the leg
/// `(3, 1)` inserted. Provenance `[0,1,None,2,3,4]` rebinds 2 → 3;
/// `[0,1,None,2,None,4]` strands it at `RETIRED_FLOOR + 2` — the
/// spelling `test_document.py`'s strand row asserts.
#[test]
fn the_python_fixture_rebinds_and_strands_as_its_rows_say() {
    let square = LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap();
    let (doc, profile, ext) = extruded("set-program-python", vec![square]);
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
    let (leg, provenance) = square_with_a_leg();
    let rebound = accepted(&doc, profile, vec![leg.clone()], provenance);
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
        vec![leg],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(2), None, Some(4)],
        }],
    );
    assert_eq!(
        stranded.maintenance,
        vec![Maintenance::Strand {
            node: fillet,
            name: rim(RETIRED_FLOOR + 2)
        }]
    );
}

// ---------------------------------------------------------------- //
// What a SLOT edit does to a LIVE name, pinned as measured
// ---------------------------------------------------------------- //

/// **A slot edit through a `Zero` fit renumbers a loop's LIVE names
/// and reports nothing** — pinned as measured, not as wanted
/// (`work/edit/a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names.md`).
/// At `r = 2` the corner fillet's two runs fit `Zero` and the loop
/// draws three segments — the arc, the top edge, and wall 2, the LEFT
/// edge `(0, 2) → (0, 0)`; a plain `SetParam` to `r = 0.3` makes the
/// runs emit, the loop draws five, and wall 2 is now the RIGHT edge
/// `(2, 0.3) → (2, 2)`. The frame on wall 2 keeps evaluating, on the
/// opposite wall, with no row reported: the renumbering `eval::anchor`
/// rules out for a parameter edit is exactly what a fit gate
/// reintroduces, and nothing in the edit vocabulary says so today.
#[test]
fn a_slot_edit_through_a_zero_fit_renumbers_a_live_name_and_reports_nothing() {
    let (doc, profile, ext) = extruded("set-param-zero-fit", vec![filleted_square(2.0)]);
    assert_eq!(drawn_segments(&doc, ext), 3);
    let (doc, frame) = frame_on(doc, ext, wall_of(ext, 0, 2));
    let before = corners_of(&doc, ext, &wall_of(ext, 0, 2));
    assert!(
        has_corner3(&before, (0.0, 2.0, 0.0)) && has_corner3(&before, (0.0, 0.0, 0.0)),
        "wall 2 is the left edge at r = 2: {before:?}"
    );
    let grown = apply(
        &doc,
        &DocEdit::SetParam {
            node: profile,
            slot: SlotId::Profile {
                loop_: 0,
                step: 2,
                arg: StepArg::Radius,
            },
            expr: len(0.3),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the radius is a legal slot write");
    assert_eq!(
        grown.maintenance,
        vec![],
        "measured: the slot edit reports nothing"
    );
    assert_eq!(drawn_segments(&grown.doc, ext), 5);
    let ev = fixture::run(&grown.doc, &EvalOptions::default());
    assert!(
        ev.value(frame).is_some(),
        "measured: the frame still evaluates — {:?}",
        corpus::failures(&ev)
    );
    let after = corners_of(&grown.doc, ext, &wall_of(ext, 0, 2));
    assert!(
        has_corner3(&after, (2.0, 2.0, 0.0)) && !has_corner3(&after, (0.0, 2.0, 0.0)),
        "measured: wall 2 is the right edge now — the live name renumbered: {after:?}"
    );
}

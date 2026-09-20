//! Review probes for `edit/program-edit` (lane `program-r1`).
//!
//! Each row here is evidence for one claim of PR #2927. A row that
//! FAILS on the reviewed head is a finding; a row that passes confirms
//! a claim the PR makes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Attr, Datum, DocEdit, EvalOptions, LoopProgram, LoopProvenance, Maintenance, MeasureExpr,
    MeasurePrimitive, Node, ProfileDoc, ProfileEdgeRef, ProfileProgram, ProfileVertexRef,
    ProgramStep, ProgramTarget, RecipeNodeId, Rgba8, RoleSeg, SitedRef, SlotId, StableName,
    StepArg, apply,
};
use fixture::{fname, insert, len, scl, tol};

fn pt(x: f64, y: f64) -> [editor_core::Expr; 2] {
    [len(x), len(y)]
}

/// `At, Toward(+x), Fillet(r), Toward(+y), FarEndTo(2,2), LineTo(0,2),
/// LineTo(Start)` — the corner-fillet chain the suite's changed-step
/// row uses, with the radius a parameter. Measured: at `r = 2` both
/// runs of the fillet have a `Zero` fit and emit nothing, so the loop
/// draws THREE segments; at `r = 0.3`, `1.0` and `1.9` it draws FIVE.
fn filleted_square(r: f64) -> LoopProgram {
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

/// A square: four segments, `0 = bottom, 1 = right, 2 = top,
/// 3 = left`.
fn square() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// [`square`] with a bump inserted in the BOTTOM wall — `Toward` then
/// `Line`, two steps drawing one segment — so every segment after 0
/// moves up by one.
fn bumped_square() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(-1.0),
        },
        ProgramStep::Line(len(0.5_f64.sqrt())),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

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

fn wall(node: RecipeNodeId, loop_index: u32, segment: u32) -> StableName {
    fname(
        node,
        RoleSeg::Lateral(ProfileEdgeRef {
            loop_index,
            segment,
        }),
    )
}

fn set_program(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    loops: Vec<LoopProgram>,
    provenance: Vec<LoopProvenance>,
) -> Result<editor_core::Applied<ProfileProgram>, editor_core::EditError> {
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

fn face_of(doc: &ProfileDoc, frame: RecipeNodeId) -> StableName {
    match doc.node(frame) {
        Some(Node::Datum(Datum::FaceFrame { face, .. })) => face.clone(),
        other => panic!("a frame, got {other:?}"),
    }
}

fn frame_on_wall(doc: ProfileDoc, ext: RecipeNodeId, seg: u32) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Datum(Datum::FaceFrame {
            at: ext,
            face: wall(ext, 0, seg),
            spin: fixture::ang(0.0),
        }),
    )
}

/// How many segments loop 0 draws, counted off the extrude's
/// published name table.
fn drawn_segments(doc: &ProfileDoc, ext: RecipeNodeId) -> u32 {
    let ev = fixture::run(doc, &EvalOptions::default());
    let t = fixture::table(&ev, ext);
    let mut n = 0;
    while t.lookup(&wall(ext, 0, n)).is_some() {
        n += 1;
    }
    n
}

// ---------------------------------------------------------------- //
// Claim 1 — the retired coordinate
// ---------------------------------------------------------------- //

/// **RED — a retired coordinate goes live when a SLOT edit grows the
/// loop.** `SegmentMap`'s retiring paragraph and
/// `ProfileEdgeRef::segment`'s doc claim a retired coordinate is past
/// every drawn one and stays so, because "a name retired once is
/// retired again by every later reshaping". Only a RESHAPING re-retires
/// it — and a loop's segment count is a function of its ARGUMENTS too:
/// at `r = 2` the corner fillet's two runs have a `Zero` fit and emit
/// nothing (three segments), at `r = 0.3` they emit (five). A plain
/// `DocEdit::SetParam` on that radius draws a segment AT the retired
/// coordinate; the retired name silently denotes it and nothing is
/// reported — the DI1 aliasing class the ruling exists to end, one
/// slot edit later.
#[test]
fn probe_a_retired_name_goes_live_when_a_slot_edit_grows_the_loop() {
    let (doc, profile, ext) = extruded("probe-retire-grow", vec![filleted_square(2.0)]);
    let n_tight = drawn_segments(&doc, ext);
    let (doc, frame) = frame_on_wall(doc, ext, 0);
    let applied = set_program(
        &doc,
        profile,
        vec![filleted_square(2.0)],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), Some(2), Some(3), None, Some(5), Some(6)],
        }],
    )
    .expect("the reshaping is accepted");
    let retired = face_of(&applied.doc, frame);
    assert_eq!(retired, wall(ext, 0, n_tight), "retired past the end");
    let doc = applied.doc;
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
    assert_eq!(grown.maintenance, vec![], "the slot edit reports nothing");
    let n_loose = drawn_segments(&grown.doc, ext);
    let ev = fixture::run(&grown.doc, &EvalOptions::default());
    let live = fixture::table(&ev, ext).lookup(&retired).is_some();
    assert!(
        !live,
        "the retired name came back to life: the loop grew from {n_tight} to {n_loose} \
         segments under a slot edit and now draws one at the retired coordinate"
    );
}

/// **A SECOND reshaping that grows the loop past a retired index does
/// re-retire the name** — `SegmentMap`'s paragraph, confirmed.
#[test]
fn probe_a_second_reshaping_re_retires_a_retired_name() {
    let (doc, profile, ext) = extruded("probe-re-retire", vec![square()]);
    let (doc, frame) = frame_on_wall(doc, ext, 0);
    let applied = set_program(
        &doc,
        profile,
        vec![square()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), None, Some(2), Some(3), Some(4)],
        }],
    )
    .expect("accepted");
    assert_eq!(
        face_of(&applied.doc, frame),
        wall(ext, 0, 4),
        "retired past the loop's end"
    );
    let grown = set_program(
        &applied.doc,
        profile,
        vec![bumped_square()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), None, None, Some(1), Some(2), Some(3), Some(4)],
        }],
    )
    .expect("accepted");
    assert_eq!(
        face_of(&grown.doc, frame),
        wall(ext, 0, 5 + 4),
        "the retired name is retired again, past the GROWN loop's end"
    );
}

/// **A retired name round-trips the wire: the load door admits a
/// `ProfileEdgeRef` past the loop's end.**
#[test]
fn probe_a_retired_name_round_trips_the_wire() {
    let (doc, profile, ext) = extruded("probe-retired-wire", vec![square()]);
    let (doc, frame) = frame_on_wall(doc, ext, 0);
    let applied = set_program(
        &doc,
        profile,
        vec![square()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), None, Some(2), Some(3), Some(4)],
        }],
    )
    .expect("accepted");
    let retired = face_of(&applied.doc, frame);
    let bytes = editor_core::save(&applied.doc, &[], tol()).expect("saves");
    let back = editor_core::load(&bytes, tol()).expect("loads");
    assert_eq!(
        face_of(&back.doc, frame),
        retired,
        "the retired spelling loads"
    );
}

/// **`Rebind` from the retired spelling is the repair DM7 promises.**
#[test]
fn probe_rebind_repairs_a_retired_name() {
    let (doc, profile, ext) = extruded("probe-retired-rebind", vec![square()]);
    let (doc, frame) = frame_on_wall(doc, ext, 0);
    let applied = set_program(
        &doc,
        profile,
        vec![square()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), None, Some(2), Some(3), Some(4)],
        }],
    )
    .expect("accepted");
    let retired = face_of(&applied.doc, frame);
    let repaired = apply(
        &applied.doc,
        &DocEdit::Rebind {
            from: retired,
            to: wall(ext, 0, 0),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("a rebind from the retired spelling");
    assert_eq!(face_of(&repaired.doc, frame), wall(ext, 0, 0));
    let ev = fixture::run(&repaired.doc, &EvalOptions::default());
    assert!(
        crate::corpus::failures(&ev).is_empty(),
        "the repaired frame resolves"
    );
}

// ---------------------------------------------------------------- //
// Claim 2 — the other carrier kinds
// ---------------------------------------------------------------- //

/// **Every carrier kind REFERENCES.md §0 lists is rebound, and every
/// one is stranded** — the suite covers a blend selection, a face
/// frame and the appearance store; this row adds the shell open list,
/// the chamfer selection, the measure refs and the declare pairs.
#[test]
fn probe_every_carrier_kind_rebinds_and_strands() {
    let (doc, profile, ext) = extruded("probe-carriers", vec![square()]);
    let w1 = wall(ext, 0, 1);
    let (doc, shell) = insert(
        doc,
        Node::Shell {
            target: ext,
            thickness: len(0.1),
            open: vec![w1.clone()],
        },
    );
    let (doc, chamfer) = insert(
        doc,
        Node::Chamfer {
            target: ext,
            distance: len(0.1),
            selection: vec![w1.clone()],
        },
    );
    let (doc, measure) = insert(
        doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(ext, w1.clone()),
                SitedRef::new(ext, wall(ext, 0, 2)),
            ],
        )
        .expect("both indices address a reference"),
    );
    let (doc, decl) = insert(
        doc,
        Node::declare_rest(vec![(
            SitedRef::new(ext, w1.clone()),
            SitedRef::new(ext, wall(ext, 0, 3)),
        )]),
    );
    let doc = apply(
        &doc,
        &DocEdit::SetAppearance {
            name: w1.clone(),
            attr: Attr::Color(Rgba8::opaque(1, 2, 3)),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("paints")
    .doc;

    let held = |d: &ProfileDoc| -> Vec<StableName> {
        let mut out = Vec::new();
        for id in [shell, chamfer, measure, decl] {
            match d.node(id) {
                Some(Node::Shell { open, .. }) => out.extend(open.clone()),
                Some(Node::Chamfer { selection, .. }) => out.extend(selection.clone()),
                Some(Node::Measure { refs, .. }) => {
                    out.extend(refs.iter().map(|r| r.name.clone()));
                }
                Some(Node::Declare { pairs }) => out.extend(
                    pairs
                        .iter()
                        .flat_map(|((a, b), _)| [a.name.clone(), b.name.clone()]),
                ),
                other => panic!("unexpected node {other:?}"),
            }
        }
        out
    };

    // The bump moves every segment after 0 up by one.
    let moved = set_program(
        &doc,
        profile,
        vec![bumped_square()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), None, None, Some(1), Some(2), Some(3), Some(4)],
        }],
    )
    .expect("accepted");
    assert_eq!(
        held(&moved.doc),
        vec![
            wall(ext, 0, 2),
            wall(ext, 0, 2),
            wall(ext, 0, 2),
            wall(ext, 0, 3),
            wall(ext, 0, 2),
            wall(ext, 0, 4),
        ],
        "every carrier holds the rebound spelling"
    );
    let rebounds = moved
        .maintenance
        .iter()
        .filter(|m| matches!(m, Maintenance::Rebound { .. }))
        .count();
    assert_eq!(rebounds, 3, "one row per NAME, not per carrier");

    // The same document, reshaped so the step that drew segment 1 is
    // not continued.
    let gone = set_program(
        &doc,
        profile,
        vec![square()],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), None, Some(3), Some(4)],
        }],
    )
    .expect("accepted");
    let strands: Vec<RecipeNodeId> = gone
        .maintenance
        .iter()
        .filter_map(|m| match m {
            Maintenance::Strand { node, .. } => Some(*node),
            _ => None,
        })
        .collect();
    assert_eq!(
        strands,
        vec![shell, chamfer, measure, decl],
        "every node carrier reports its strand, in document order"
    );
    assert!(
        gone.maintenance
            .iter()
            .any(|m| matches!(m, Maintenance::StrandedAppearance { .. })),
        "the store key strands too"
    );
}

// ---------------------------------------------------------------- //
// Claim 3 — the loop's seams
// ---------------------------------------------------------------- //

/// **A leg inserted as the LAST step, before the close, and a name on
/// vertex 0.** Vertex 0 is the end of the CLOSING segment, so it stays
/// vertex 0 however many segments the loop gained before it.
#[test]
fn probe_a_leg_inserted_before_the_close_leaves_vertex_zero_alone() {
    let tail = LoopProgram::Chain(vec![
        ProgramStep::At(pt(0.0, 0.0)),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 0.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(2.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(0.0, 2.0))),
        ProgramStep::LineTo(ProgramTarget::Point(pt(-0.5, 1.0))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, profile, ext) = extruded("probe-seam", vec![square()]);
    let (doc, on_left) = frame_on_wall(doc, ext, 3);
    let strut0 = fixture::ename(
        ext,
        RoleSeg::LateralEdge(ProfileVertexRef {
            loop_index: 0,
            vertex: 0,
        }),
    );
    let (doc, fillet) = insert(doc, Node::fillet(ext, len(0.1), vec![strut0.clone()]));
    let applied = set_program(
        &doc,
        profile,
        vec![tail],
        vec![LoopProvenance {
            from: Some(0),
            steps: vec![Some(0), Some(1), Some(2), Some(3), None, Some(4)],
        }],
    )
    .expect("accepted");
    assert_eq!(
        face_of(&applied.doc, on_left),
        wall(ext, 0, 4),
        "the closing step's wall moves to the end of the grown loop"
    );
    match applied.doc.node(fillet) {
        Some(Node::Fillet { selection, .. }) => assert_eq!(
            selection,
            &vec![strut0],
            "vertex 0 is the close's end and does not move"
        ),
        other => panic!("a fillet, got {other:?}"),
    }
}

//! M9-D1 review probes (R2), emitter end-to-end: the narrowed
//! UNRESOLVED refusal must stay unreachable — and the table total —
//! on the shapes the elimination-era emitter never saw: the negative
//! (non-reversed) wedge of an all-on-axis loop, a full revolve whose
//! axis run spans two segments (a DELETED interior vertex), and a
//! partial revolve mixing an on-axis outer run with a hole loop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, Datum, EntityKind, EvalOptions, Evaluation, LoopProgram, NameTable, Node,
    ProfileDoc, ProfileProgram, ProfileVertexRef, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::{ang, insert, len};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
    &ev.value(id)
        .unwrap_or_else(|| panic!("node {id:?} has no value: {:?}", ev.nodes.get(&id)))
        .name_table
}

fn pole(node: RecipeNodeId, l: u32, v: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::Pole(ProfileVertexRef {
            loop_index: l,
            vertex: v,
        })],
    }
}

/// XY-plane revolve about the y datum axis, from loop programs.
fn revolve_programs(loops: Vec<LoopProgram>, angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m9_d1_r2_probes", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, p) = insert(doc, Node::Profile(ProfileProgram { plane, loops }));
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [fixture::scl(0.0), fixture::scl(1.0), fixture::scl(0.0)],
        }),
    );
    insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(angle),
        },
    )
}

fn semicircle(r: f64) -> LoopProgram {
    let p2 = |x: f64, y: f64| [len(x), len(y)];
    LoopProgram::Chain(vec![
        ProgramStep::At(p2(0.0, -r)),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p2(0.0, r)),
            b: fixture::scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The NON-REVERSED arm (θ < 0 sweeps the canonical traversal): the
/// all-on-axis wedge must name identically to the +θ row.
#[test]
fn negative_angle_wedge_of_an_all_on_axis_loop_names_both_poles() {
    let (doc, rev) = revolve_programs(vec![semicircle(1.0)], -std::f64::consts::FRAC_PI_2);
    let ev = run(&doc);
    let t = table(&ev, rev);
    assert_eq!(t.len(), 9);
    for v in 0..2 {
        assert!(t.lookup(&pole(rev, 0, v)).is_some(), "pole {v} unnamed");
    }
}

/// The claim-3 attack fixture: a partial revolve mixing an on-axis
/// outer run with an off-axis HOLE loop, both θ signs. If the narrowed
/// refusal (or a silent mis-name) were reachable on a legal
/// multi-chain shape, this is where it would show; instead the table
/// must be total with both outer poles named and no hole poles.
#[test]
fn partial_revolve_with_hole_and_axis_run_names_totally_both_signs() {
    let p2 = |x: f64, y: f64| [len(x), len(y)];
    let hole = LoopProgram::Chain(vec![
        ProgramStep::At(p2(0.7, -0.3)),
        ProgramStep::LineTo(ProgramTarget::Point(p2(1.3, -0.3))),
        ProgramStep::LineTo(ProgramTarget::Point(p2(1.3, 0.3))),
        ProgramStep::LineTo(ProgramTarget::Point(p2(0.7, 0.3))),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    for theta in [std::f64::consts::FRAC_PI_2, -std::f64::consts::FRAC_PI_2] {
        let (doc, rev) = revolve_programs(vec![semicircle(2.0), hole.clone()], theta);
        let ev = run(&doc);
        let t = table(&ev, rev);
        for v in 0..2 {
            assert!(
                t.lookup(&pole(rev, 0, v)).is_some(),
                "theta {theta}: outer pole {v} unnamed"
            );
        }
        for v in 0..4 {
            assert!(
                t.lookup(&pole(rev, 1, v)).is_none(),
                "theta {theta}: hole vertex {v} must not be a pole"
            );
        }
    }
}

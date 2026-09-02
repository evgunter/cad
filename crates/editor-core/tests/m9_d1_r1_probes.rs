//! M9-D1 review probes (R1), naming level: the narrowed refusal and
//! the None-export honesty, attacked with profiles the shipped rows
//! don't cover — a SUBDIVIDED axis run (interior on-axis vertex: the
//! full case deletes it, the partial keeps it as a third pole) and a
//! MIXED on/off-axis dome. Every row stands on `check_total`: a
//! silently mis-named or unnamed vertex cannot pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, Datum, EntityKind, EvalOptions, Evaluation, LoopProgram, NameTable, Node,
    ProfileDoc, ProfileProgram, ProfileVertexRef, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::{ang, insert, len, scl};
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

fn pole(node: RecipeNodeId, v: u32) -> StableName {
    StableName {
        kind: EntityKind::Vertex,
        node,
        path: vec![RoleSeg::Pole(ProfileVertexRef {
            loop_index: 0,
            vertex: v,
        })],
    }
}

/// A revolve doc for one authored chain on the xz-authoring plane of
/// [`m4_pr3_names`]'s ball fixture (axis = sketch y).
fn revolve_chain(steps: Vec<ProgramStep>, angle: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m9_d1_r1_probes", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(steps)],
        }),
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
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

fn p2(x: f64, y: f64) -> [editor_core::Expr; 2] {
    [len(x), len(y)]
}

/// A SUBDIVIDED axis run (an interior on-axis vertex) is
/// unrepresentable through the program layer: every authoring of a
/// collinear line-line junction is refused upstream (overdetermined
/// close / tangent junction / same-carrier #101), so an
/// editor-reachable profile's axis run is always ONE segment and
/// every live on-axis vertex is a run TIP — the pole export's Some.
/// The full case's deleted-interior branch (None export) is covered
/// at the sweep API level (sweep/tests/m9_d1_r1_probes.rs); here we
/// pin that the editor cannot reach it.
#[test]
fn subdivided_axis_run_is_refused_upstream_of_the_emitter() {
    use editor_core::DocEdit;
    let doc = ProfileDoc::empty_derived("m9_d1_r1_probes", Tol::witness());
    // The frame goes in first: this row is about the PROGRAM's
    // same-carrier refusal, and a profile naming a plane the document
    // does not have would be turned away for that instead.
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let node = Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At(p2(0.0, 1.0)),
            ProgramStep::LineTo(ProgramTarget::Point(p2(0.0, 0.0))),
            ProgramStep::Tangent,
            ProgramStep::Line(len(1.0)),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Start,
                b: scl(1.0),
            }),
        ])],
    });
    let err = doc
        .apply(&DocEdit::InsertNode { node }, Tol::witness())
        .unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("carrier identity is not tangency"),
        "expected the same-carrier refusal, got {msg}"
    );
}

/// The mixed dome: (0,0) →line→ (1,0) →quarter arc→ (0,1) →axis
/// line→ close. One off-axis anchor + two poles, full revolve: the
/// narrowed refusal must NOT fire (the off-axis vertex anchors), and
/// both poles come from the export.
#[test]
fn full_mixed_profile_names_poles_and_anchors_the_off_axis_vertex() {
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let (doc, rev) = revolve_chain(
        vec![
            ProgramStep::At(p2(0.0, 0.0)),
            ProgramStep::LineTo(ProgramTarget::Point(p2(1.0, 0.0))),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Point(p2(0.0, 1.0)),
                b: scl(b),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
        std::f64::consts::TAU,
    );
    let ev = run(&doc);
    let t = table(&ev, rev);
    // Canonical v0=(0,0), v1=(1,0) off-axis, v2=(0,1).
    assert!(t.lookup(&pole(rev, 0)).is_some());
    assert!(
        t.lookup(&pole(rev, 1)).is_none(),
        "off-axis vertex is not a pole"
    );
    assert!(t.lookup(&pole(rev, 2)).is_some());
}

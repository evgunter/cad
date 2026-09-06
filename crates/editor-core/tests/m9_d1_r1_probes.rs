//! M9-D1 review probes (R1), naming level: the narrowed refusal and
//! the None-export honesty, attacked with profiles the shipped rows
//! don't cover — a SUBDIVIDED axis run (interior on-axis vertex: the
//! full case deletes it, the partial keeps it as a third pole) and a
//! MIXED on/off-axis dome. Every row stands on `check_total`: a
//! silently mis-named or unnamed vertex cannot pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EntityKind, EvalOptions, Evaluation, LoopProgram, NameTable, Node, ProfileDoc,
    ProfileProgram, ProfileVertexRef, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId,
    RoleSeg, StableName, ValuePayload, evaluate, vertex_position,
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
        // The axis, in the frame's own coordinates: the profile's v is
        // world +Y, so the line the revolve turns about is that
        // frame's +y through (0, 0).
        fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)),
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

/// A SUBDIVIDED axis run — an on-axis side carried by two collinear
/// legs, so the run has an INTERIOR on-axis vertex — is representable
/// through the program layer: `.tangent()` then `line(len)` is a
/// declared tangent joint, and the lattice takes the collinear
/// line-line junction on that declaration. This row pins the
/// authoring; the two rows below revolve the same chain and pin what
/// the naming lane emits at its interior vertex.
#[test]
fn subdivided_axis_run_is_representable_through_the_program_layer() {
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
    doc.apply(&DocEdit::InsertNode { node }, Tol::witness())
        .expect("a declared collinear joint is a tangent joint, so this authors");
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

/// The subdivided axis run, authored through the program layer: the
/// chain of the row above, whose PROGRAM vertices are v0 = (0, 1),
/// v1 = (0, 0) — the interior on-axis vertex — and v2 = (0, −1).
fn subdivided_axis_run(angle: f64) -> (ProfileDoc, RecipeNodeId) {
    revolve_chain(
        vec![
            ProgramStep::At(p2(0.0, 1.0)),
            ProgramStep::LineTo(ProgramTarget::Point(p2(0.0, 0.0))),
            ProgramStep::Tangent,
            ProgramStep::Line(len(1.0)),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Start,
                b: scl(1.0),
            }),
        ],
        angle,
    )
}

/// The `poles` export of the doc's profile, re-swept at the same
/// revolution, reindexed by PROGRAM vertex: the emitter reads canonical
/// indices and the published table is program-anchored, so the arm a
/// row compares against the emitter's outcome has to cross the anchor.
fn export_poles_by_program_vertex(
    doc: &ProfileDoc,
    ev: &Evaluation<f64>,
    revolution: sweep::Revolution<f64>,
) -> Vec<bool> {
    let profile = *doc
        .order()
        .iter()
        .find(|id| matches!(doc.node(**id), Some(Node::Profile(_))))
        .expect("the doc's profile node");
    let ValuePayload::Profile(vp) = &ev.value(profile).expect("the profile evaluated").payload
    else {
        panic!("node {profile:?} is not a profile");
    };
    let built = sweep::revolve(
        &vp.validated,
        sweep::RevolveAxis {
            origin: geom_core::Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        },
        revolution,
        Tol::witness(),
    )
    .expect("the revolve the evaluation already ran");
    let anchor = vp.naming.loops[0];
    let mut by_program = vec![false; built.poles[0].len()];
    for (k, p) in built.poles[0].iter().enumerate() {
        by_program[anchor.vertex(u32::try_from(k).expect("a loop index")) as usize] = p.is_some();
    }
    by_program
}

/// **FULL revolve of a subdivided axis run: the interior on-axis vertex
/// is named nothing, and the table is still total.** The full case
/// deletes the axis run outright, so no body entity stands at the
/// interior vertex and the pole export's `None` is the whole answer;
/// the two run TIPS survive as poles. `check_total` is the gate: the
/// evaluation carries a name table only when every live vertex of the
/// body has a name, so a `None` covering a LIVE vertex could not reach
/// these assertions.
#[test]
fn full_subdivided_axis_run_names_no_vertex_for_the_interior() {
    let (doc, rev) = subdivided_axis_run(std::f64::consts::TAU);
    let ev = run(&doc);
    let t = table(&ev, rev);
    assert!(t.lookup(&pole(rev, 0)).is_some(), "run tip v0 unnamed");
    assert!(
        t.lookup(&pole(rev, 1)).is_none(),
        "the deleted interior vertex must have no name"
    );
    assert!(t.lookup(&pole(rev, 2)).is_some(), "run tip v2 unnamed");
    assert_eq!(
        export_poles_by_program_vertex(&doc, &ev, sweep::Revolution::Full),
        vec![true, false, true],
        "the export's arm must agree with what the emitter named"
    );
}

/// **PARTIAL revolve of the same run: the interior on-axis vertex IS a
/// pole.** The partial case keeps the axis run, the rotation fixes
/// every point of it, and both meridian chains meet at the interior
/// vertex — structurally what the run tips are — so it takes
/// `Pole(v1)` and the export says `Some`.
#[test]
fn partial_subdivided_axis_run_names_the_interior_vertex_a_pole() {
    let (doc, rev) = subdivided_axis_run(std::f64::consts::FRAC_PI_2);
    let ev = run(&doc);
    let t = table(&ev, rev);
    for v in 0..3 {
        assert!(t.lookup(&pole(rev, v)).is_some(), "pole {v} unnamed");
    }
    // The interior vertex is the run's midpoint, not a third tip.
    let at = |v| vertex_position(&ev, rev, &pole(rev, v)).expect("a named pole has a position");
    let (a, b, c) = (at(0), at(1), at(2));
    for (mid, ends) in [(b.x, a.x + c.x), (b.y, a.y + c.y), (b.z, a.z + c.z)] {
        assert!(
            (2.0 * mid - ends).abs() < 1e-12,
            "the interior pole is off the run's midpoint"
        );
    }
    assert_eq!(
        export_poles_by_program_vertex(
            &doc,
            &ev,
            sweep::Revolution::Partial(std::f64::consts::FRAC_PI_2)
        ),
        vec![true, true, true],
        "the export's arm must agree with what the emitter named"
    );
}

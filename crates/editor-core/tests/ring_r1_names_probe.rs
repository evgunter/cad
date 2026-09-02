//! VERBS-RING review probe (reviewer lane r1): naming totality for
//! the WIRE-outer + hole composition — `m4_pr3_names.rs` covers the
//! lamina outer + hole ring; this covers the axis-touching outer
//! whose holes still insert as lamina cavities. `check_total` runs
//! inside `name_revolve`, so a hole entity the emitter missed fails
//! the evaluation itself; the row also pins the hole's names landing
//! under loop index 1.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, Datum, EntityKind, EvalOptions, Evaluation, MeridianEnd, NameTable, Node,
    ProfileDoc, ProfileEdgeRef, ProfileVertexRef, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use fixture::{ang, desc, insert, len, on_frame};
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

fn name1(kind: EntityKind, node: RecipeNodeId, seg: RoleSeg) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

fn pe(l: u32, s: u32) -> ProfileEdgeRef {
    ProfileEdgeRef {
        loop_index: l,
        segment: s,
    }
}

fn pv(l: u32, v: u32) -> ProfileVertexRef {
    ProfileVertexRef {
        loop_index: l,
        vertex: v,
    }
}

#[test]
fn full_wire_holed_revolve_names_totally() {
    let doc = ProfileDoc::empty_derived("ring_r1_names_probe", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![
            // Outer touches the axis along its left edge (wire
            // case); the hole is strictly off-axis.
            vec![(0.0, 0.0), (2.0, 0.0), (2.0, 3.0), (0.0, 3.0)],
            vec![(0.5, 1.0), (1.5, 1.0), (1.5, 2.0), (0.5, 2.0)],
        ],
    );
    let (doc, axis) = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [fixture::scl(0.0), fixture::scl(1.0), fixture::scl(0.0)],
        }),
    );
    let (doc, rev) = insert(
        doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    );
    let ev = run(&doc);
    // Totality: `check_total` ran inside name_revolve — reaching a
    // table at all means every body entity is named.
    let t = table(&ev, rev);
    // The hole's entities land under loop index 1, seam-meridian
    // taxonomy (holes are lamina even under a wire outer).
    for s in 0..4 {
        assert!(
            t.lookup(&name1(EntityKind::Face, rev, RoleSeg::Band(pe(1, s))))
                .is_some()
        );
        assert!(
            t.lookup(&name1(EntityKind::Edge, rev, RoleSeg::BandRim(pv(1, s))))
                .is_some()
        );
        assert!(
            t.lookup(&name1(
                EntityKind::Edge,
                rev,
                RoleSeg::Meridian(MeridianEnd::Seam, pe(1, s))
            ))
            .is_some()
        );
        assert!(
            t.lookup(&name1(
                EntityKind::Vertex,
                rev,
                RoleSeg::MeridianVertex(MeridianEnd::Seam, pv(1, s))
            ))
            .is_some()
        );
    }
    // And the wire outer keeps its π-band names (loop 0).
    assert!((0..4).any(|s| {
        t.lookup(&name1(EntityKind::Face, rev, RoleSeg::BandPi(pe(0, s))))
            .is_some()
    }));
}

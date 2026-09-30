//! VERBS-RING review probe (reviewer lane r1): naming totality for
//! the WIRE-outer + hole composition — `m4_pr3_names.rs` covers the
//! lamina outer + hole ring; this covers the axis-touching outer
//! whose holes still insert as lamina cavities. `check_total` runs
//! inside `name_revolve`, so a hole entity the emitter missed fails
//! the evaluation itself; the row also pins the hole's names landing
//! under loop index 1.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EntityKind, EvalOptions, Evaluation, MeridianEnd, Node, ProfileDoc,
    ProfileEdgeRef, RecipeNodeId, RoleSeg, band, band_pi, band_rim, evaluate, meridian_vertex,
};
use fixture::{ang, axis_in_plane, insert, minted, on_frame_keeping, table};
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

fn pe(doc: &editor_core::ProfileDoc, node: RecipeNodeId, l: u32, s: u32) -> ProfileEdgeRef {
    crate::fixture::piece(doc, node, l as usize, s as usize)
}

fn pv(
    doc: &editor_core::ProfileDoc,
    node: RecipeNodeId,
    l: u32,
    v: u32,
) -> editor_core::ProfileVertexRef {
    crate::fixture::vpiece(doc, node, l as usize, v as usize)
}

#[test]
fn full_wire_holed_revolve_names_totally() {
    let doc = ProfileDoc::empty_derived("ring_r1_names_probe", Tol::witness());
    let (doc, plane, p) = on_frame_keeping(
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
        // The axis, in the frame's own coordinates: the profile's v is
        // world +Y, so the line the revolve turns about is that
        // frame's +y through (0, 0).
        axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)),
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
        assert!(t.lookup(&band(rev, pe(&doc, rev, 1, s))).is_some());
        assert!(t.lookup(&band_rim(rev, pv(&doc, rev, 1, s))).is_some());
        assert!(
            t.lookup(&minted(
                EntityKind::Edge,
                rev,
                RoleSeg::Meridian(MeridianEnd::Seam, pe(&doc, rev, 1, s))
            ))
            .is_some()
        );
        assert!(
            t.lookup(&meridian_vertex(
                MeridianEnd::Seam,
                rev,
                pv(&doc, rev, 1, s)
            ))
            .is_some()
        );
    }
    // And the wire outer keeps its π-band names (loop 0).
    assert!((0..4).any(|s| t.lookup(&band_pi(rev, pe(&doc, rev, 0, s))).is_some()));
}

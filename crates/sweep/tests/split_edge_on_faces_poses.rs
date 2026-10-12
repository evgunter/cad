//! **A split whose plane holds an edge at the zero band's edge refuses
//! as the input's reading of that edge.** The poses are the near-tangent
//! census's ([`NtPrism::near_normal`]) as splits: the prism cut by the
//! cube's near face, the plane through the corner `v` and `d` rad off its
//! edge `e`, so at `|d| = ε` the edge's far vertex lies on the zero
//! band's edge. The split reads that vertex on the plane, and the edge
//! with it. Its carrier, read one rounding past the vertex, can then
//! read just in band of a surface it lies on, at one of three sites: the
//! describe pass's transverse arm (an `Intersection` residual), its
//! smooth arm (the image in the section chart), or the promotion's
//! re-chart (the restated image). Each refuses typed as the edge's own
//! reading, with the lever, never as a kernel defect or the fit's last
//! resort.
//!
//! The lever tells the truth: the far vertex moved onto the plane
//! (`d = 0`), or the edge tilted off it at a clear angle on either side,
//! builds. Every pinned edge is a line chord, the one carrier whose
//! in-band reading is the input's (`merge_faces::edge_on_faces_reading`).
//!
//! Each ε row has its own poses, every one measured refusing at that row
//! (`examples/split_near_tangent_census_probe.rs`), beside the site it
//! refused at before the typing: `surface` the transverse arm, `chart`
//! the smooth arm, `rechart` the promotion.
//!
//! [`NtPrism::near_normal`]: crate::common::differential::NtPrism::near_normal

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitFinishError, split};

use crate::common::differential::nt_prisms;

/// `(ε, prism, edge, sixteenths, tilt)`, each beside its site.
const POSES: [(f64, &str, usize, f64, f64); 21] = [
    (1e-9, "Ltop", 0, 5.0, 1e-9),       // surface
    (1e-9, "w345", 2, 10.0, 1e-9),      // surface
    (1e-9, "asym", 2, 14.0, -1e-9),     // surface
    (1e-9, "Ltop", 2, 0.0, -1e-9),      // chart
    (1e-9, "Lmirror", 2, 4.0, 1e-9),    // chart
    (1e-9, "Ltop", 0, 12.0, 1e-9),      // rechart
    (1e-9, "convex", 1, 0.0, 1e-9),     // rechart
    (1e-6, "w345", 2, 1.0, 1e-6),       // surface
    (1e-6, "w60", 2, 6.0, 1e-6),        // surface
    (1e-6, "w345", 2, 13.0, -1e-6),     // surface
    (1e-6, "Ltop", 2, 1.0, 1e-6),       // chart
    (1e-6, "vee300", 2, 0.0, 1e-6),     // chart
    (1e-6, "Ltop", 0, 12.0, -1e-6),     // rechart
    (1e-6, "convex", 1, 1.0, 1e-6),     // rechart
    (1e-12, "w345", 2, 13.0, 1e-12),    // surface
    (1e-12, "w345", 2, 13.0, -1e-12),   // surface
    (1e-12, "Ltop", 2, 0.0, 1e-12),     // chart
    (1e-12, "notch307", 2, 0.0, 1e-12), // chart
    (1e-12, "Ltop", 0, 12.0, 1e-12),    // rechart
    (1e-12, "convex", 1, 0.0, 1e-12),   // rechart
    (1e-12, "asym", 2, 4.0, -1e-12),    // rechart
];

const LEVER: &str = "Recourse: move the split plane or the geometry so each edge and vertex that \
                     meets the plane lies clearly on it or clearly off it, and an edge that ends on \
                     it leaves it at a clear angle";

/// The clear angle the lever asks of an edge that ends on the plane, in
/// rad: the edge still meets the plane at `v`, so moving its far vertex
/// clearly off is not enough (`3Kε` there leaves about half the census's
/// poses reading a face of the corner in band, `SliverSector`). A
/// centimetre over the unit edge clears every reading at every ε row; a
/// milliradian does not at ε = 1e-6.
const CLEARLY_OFF: f64 = 1e-2;

/// At each ε row, each of its poses refuses
/// [`SplitFinishError::EdgeOnFacesEscalated`] ending in the lever; and
/// with the far vertex moved onto the plane, or [`CLEARLY_OFF`] off it on
/// either side, it builds.
#[test]
fn an_edge_at_the_zero_bands_edge_refuses_typed_and_its_lever_builds() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let rows: Vec<_> = POSES
        .iter()
        .filter(|p| (p.0 - eps).abs() <= 1e-3 * eps)
        .collect();
    assert!(
        !rows.is_empty(),
        "no poses are pinned at ε = {eps:e}: the rows are ε = 1e-9, 1e-6 and 1e-12"
    );
    let prisms = nt_prisms();
    let mut wrong = Vec::new();
    for &&(_, name, edge, a, d) in &rows {
        let pr = prisms.iter().find(|p| p.name == name).unwrap();
        let body = pr.body(tol);
        let cut = |d: f64| {
            let m = pr.near_normal(edge, a, d);
            let n = Vec3::new(m[0], m[1], m[2]);
            let plane = topo::test_support::split_plane(
                Point3::new(pr.v[0], pr.v[1], pr.v[2]),
                n / n.norm(),
                tol,
            );
            split(&body, &plane, tol)
        };
        let tag = format!("{name} nt e{edge} a{a} d{d:e}");
        match cut(d) {
            Err(SplitError::Finish(e @ SplitFinishError::EdgeOnFacesEscalated { .. })) => {
                let text = e.to_string();
                if !text.starts_with("whether an edge along the cut lies on both faces it bounds")
                    || !text.ends_with(LEVER)
                {
                    wrong.push(format!("{tag}: reads otherwise: {text}"));
                }
            }
            other => wrong.push(format!("{tag}: {:?}", other.err())),
        }
        for moved in [0.0, CLEARLY_OFF, -CLEARLY_OFF] {
            if let Err(e) = cut(moved) {
                wrong.push(format!("{tag} moved to d{moved:e}: {e:?}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

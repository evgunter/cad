//! **The collinear-seam differential, closed.** R1's P2 and R2's
//! mid-vertex probes placed their interior vertex deliberately OFF the
//! chord; this file builds the same subdivided chord with the vertex
//! ON it, so the two bodies differ in one coordinate. The merge's
//! repair reads no coordinate — it deletes the seam edge left
//! dangling once the faces are joined, with its free end — so the two
//! must repair identically.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism_z;
use geom_core::{Point3, Tol};
use topo::{MefSite, MevSite, validate_closed};

/// A prism whose top face is split by a chord from `(0,0)` to `(2,2)`
/// carrying an interior vertex; `on_segment` places that vertex on the
/// chord (collinear) or off it (bent).
fn split_top(on_segment: bool) -> topo::Body<f64> {
    let p = prism_z::<f64>(
        &[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)],
        0.0,
        1.0,
        Tol::witness(),
    );
    let mut b = p.body;
    let tol = Tol::witness();
    let he_at = |b: &topo::Body<f64>, x: f64, y: f64| {
        let outer = b.get_face(p.top_face).unwrap().outer;
        let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
            panic!("top face is a cycle")
        };
        b.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| {
                let v = b.get_half_edge(he).unwrap().start;
                let pt = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
                (pt.x - x).abs() < 1e-12 && (pt.y - y).abs() < 1e-12
            })
            .expect("a half-edge starting there")
    };
    let he1 = he_at(&b, 0.0, 0.0);
    let mid = if on_segment {
        Point3::new(1.0, 1.0, 1.0) // ON the (0,0)–(2,2) diagonal
    } else {
        Point3::new(0.9, 0.6, 1.0) // R1's P2 point, off it
    };
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    let he2 = he_at(&b, 2.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    b
}

/// The DIFFERENTIAL, one screen tall: the same construction twice,
/// differing only in whether the interior vertex sits on the chord,
/// and the merge's outcome identical in every count.
#[test]
fn collinear_and_bent_seams_are_one_coordinate_apart() {
    let outcomes: Vec<_> = [true, false]
        .map(|on_segment| {
            let mut b = split_top(on_segment);
            let out = b
                .merge_coplanar_faces(Tol::witness())
                .unwrap_or_else(|e| panic!("on_segment={on_segment}: the seam repairs, got {e:?}"));
            assert_eq!(validate_closed(&b), Ok(()), "on_segment={on_segment}");
            let group = &out.groups[..];
            let [g] = group else {
                panic!("one group: {group:?}")
            };
            (
                g.absorbed.len(),
                g.killed_edges.len(),
                g.killed_vertices.len(),
                g.rings_made.len(),
                b.faces().count(),
                b.vertices().count(),
            )
        })
        .into_iter()
        .collect();
    assert_eq!(outcomes[0], outcomes[1], "collinearity changes nothing");
    let (absorbed, killed_edges, killed_vertices, rings, _, _) = outcomes[0];
    assert_eq!(
        (absorbed, killed_edges, killed_vertices, rings),
        (1, 2, 1, 0),
        "kef one seam edge, kev the other with its free end, no ring"
    );
}

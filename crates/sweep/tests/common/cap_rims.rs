//! Reading a built body's CAP RIMS: the boundary walk, the face across
//! a rim, and the description each rim carries.
//!
//! **Routing rule** (`common/mod.rs`'s, applied here): what a suite
//! CHECKS of a body it built. Not [`super::orient`], which reads
//! POSITIONS off the shipped charts to decide which side material is
//! on — nothing here evaluates a surface. Not `sweep::test_support`,
//! whose consumers are other crates: these three suites are all in
//! this one.
//!
//! What it deliberately does NOT absorb: `m5_pr12_die_body`'s
//! `face_edges`, which is a DIFFERENT walk — outer loop only, and it
//! panics on a non-cycle boundary, because a blend face has no rings
//! and a wire boundary there is a bug rather than a case to skip.

#![allow(dead_code)]

use geom_brep::EdgeDescription;
use sweep::Extruded;
use topo::{Body, EdgeKey, FaceKey, LoopBoundary, SurfaceKey};

/// Every edge of `face`, over its outer loop and every ring. A
/// boundary that is not a cycle contributes nothing.
pub fn face_edges(body: &Body<f64>, face: FaceKey) -> Vec<EdgeKey> {
    let fd = body.get_face(face).unwrap();
    let mut edges = Vec::new();
    for lk in core::iter::once(fd.outer).chain(fd.rings.iter().copied()) {
        let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
            continue;
        };
        for he in body.loop_cycle(first).unwrap() {
            edges.push(body.get_half_edge(he).unwrap().edge);
        }
    }
    edges
}

/// The face on the other side of `edge` from `face`.
pub fn face_across(body: &Body<f64>, edge: EdgeKey, face: FaceKey) -> FaceKey {
    let e = body.get_edge(edge).unwrap();
    let of = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let (plus, minus) = (of(e.he_plus), of(e.he_minus));
    assert!(plus == face || minus == face, "edge is not on the face");
    if plus == face { minus } else { plus }
}

/// The description `edge`'s certified curve carries.
pub fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeDescription<f64> {
    body.get_curve_geom(body.get_edge(edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap()
        .description()
        .clone()
}

/// One cap rim, read from the cap side.
pub struct CapRim {
    /// The surface of the cap this rim was read from.
    pub cap: SurfaceKey,
    /// The surface of the wall across it.
    pub wall: SurfaceKey,
    /// The description the rim edge carries.
    pub description: EdgeDescription<f64>,
}

/// Every cap rim of `built`, both caps, outer loop and rings.
pub fn cap_rims(built: &Extruded<f64>) -> Vec<CapRim> {
    let body = &built.body;
    [built.bottom, built.top]
        .into_iter()
        .flat_map(|cap| {
            face_edges(body, cap).into_iter().map(move |edge| {
                let surface = |f: FaceKey| body.get_face(f).unwrap().surface;
                CapRim {
                    cap: surface(cap),
                    wall: surface(face_across(body, edge, cap)),
                    description: description(body, edge),
                }
            })
        })
        .collect()
}

/// Over every cap rim: how many carry a description other than
/// `Intersection`, and of those how many are a non-seam chart image in
/// the WALL's chart and how many in the CAP's.
pub fn chart_counts(built: &Extruded<f64>) -> (usize, usize, usize) {
    let (mut conventional, mut wall_chart, mut cap_chart) = (0usize, 0usize, 0usize);
    for rim in cap_rims(built) {
        if matches!(rim.description, EdgeDescription::Intersection { .. }) {
            continue;
        }
        conventional += 1;
        if let EdgeDescription::Chart(c) = &rim.description {
            if c.seam {
                continue;
            }
            wall_chart += usize::from(c.surface == rim.wall);
            cap_chart += usize::from(c.surface == rim.cap);
        }
    }
    (conventional, wall_chart, cap_chart)
}

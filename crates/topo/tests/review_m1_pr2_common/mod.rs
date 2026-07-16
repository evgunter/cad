//! Shared helpers for the promoted M1 PR 2 adversarial review suites
//! (`review_m1_pr2_*.rs`). Adversarial e2e review artifact for M1 PR 2
//! (2026-07-16); promoted per Evan's request (PR #17 thread).
//!
//! These are **independent derivations** — do not "simplify" them to
//! match shipped fixtures (e.g. `fixtures.rs::deep_snapshot`); the
//! independence is the regression value. This directory is not a test
//! crate; each `review_m1_pr2_*` test pulls it in via `mod`, so any
//! given test binary may use only a subset of the helpers.

#![allow(dead_code)]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use topo::{Body, EntityId, LoopBoundary};

/// A full deep snapshot of a body: every key in all 10 arenas with its
/// Debug-formatted payload, plus the provenance record of every topology
/// entity. Two bodies (or one body before/after a failed op) are equal
/// iff their snapshots are string-equal.
pub fn deep_snapshot(body: &Body<f64>) -> String {
    let mut s = String::new();
    use std::fmt::Write;
    for (k, v) in body.solids() {
        writeln!(
            s,
            "solid {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::Solid(k))
        )
        .unwrap();
    }
    for (k, v) in body.shells() {
        writeln!(
            s,
            "shell {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::Shell(k))
        )
        .unwrap();
    }
    for (k, v) in body.faces() {
        writeln!(
            s,
            "face {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::Face(k))
        )
        .unwrap();
    }
    for (k, v) in body.loops() {
        writeln!(
            s,
            "loop {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::Loop(k))
        )
        .unwrap();
    }
    for (k, v) in body.half_edges() {
        writeln!(
            s,
            "he {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::HalfEdge(k))
        )
        .unwrap();
    }
    for (k, v) in body.edges() {
        writeln!(
            s,
            "edge {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::Edge(k))
        )
        .unwrap();
    }
    for (k, v) in body.vertices() {
        writeln!(
            s,
            "vertex {k:?} = {v:?} prov={:?}",
            body.provenance(EntityId::Vertex(k))
        )
        .unwrap();
    }
    for (k, v) in body.points() {
        writeln!(s, "point {k:?} = {v:?}").unwrap();
    }
    for (k, v) in body.curves() {
        writeln!(s, "curve {k:?} = {v:?}").unwrap();
    }
    for (k, v) in body.surfaces() {
        writeln!(s, "surface {k:?} = {v:?}").unwrap();
    }
    s
}

/// The coordinates of a half-edge's start vertex.
pub fn start_xyz(body: &Body<f64>, he: topo::HalfEdgeKey) -> (f64, f64, f64) {
    let v = body.get_half_edge(he).unwrap().start;
    let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    (p.x, p.y, p.z)
}

/// Walks a face's outer loop and returns the start coordinates of each
/// half-edge in `next` order.
pub fn face_polygon(body: &Body<f64>, face: topo::FaceKey) -> Vec<(f64, f64, f64)> {
    let f = body.get_face(face).unwrap();
    let LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
        panic!("face has an empty outer loop");
    };
    body.loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| start_xyz(body, he))
        .collect()
}

/// Signed area of a 3D polygon projected onto the plane with orthonormal
/// basis (u, v). Positive iff the polygon winds counterclockwise as seen
/// from the tip of n = u x v.
pub fn signed_area(poly: &[(f64, f64, f64)], u: (f64, f64, f64), v: (f64, f64, f64)) -> f64 {
    let dot = |a: (f64, f64, f64), b: (f64, f64, f64)| a.0 * b.0 + a.1 * b.1 + a.2 * b.2;
    let pts2: Vec<(f64, f64)> = poly.iter().map(|&p| (dot(p, u), dot(p, v))).collect();
    let n = pts2.len();
    let mut area2 = 0.0;
    for i in 0..n {
        let (x1, y1) = pts2[i];
        let (x2, y2) = pts2[(i + 1) % n];
        area2 += x1 * y2 - x2 * y1;
    }
    area2 / 2.0
}

/// Euler-Poincare ledger check: v - e + f - r == 2(s - h), with r =
/// number of ring loops (loops that are some face's ring) and s/h given
/// by the caller (h = genus, not derivable structurally until PR 5).
pub fn euler_poincare_holds(body: &Body<f64>, shells: i64, genus: i64) -> bool {
    let v = body.vertices().count() as i64;
    let e = body.edges().count() as i64;
    let f = body.faces().count() as i64;
    let r: i64 = body.faces().map(|(_, face)| face.rings.len() as i64).sum();
    v - e + f - r == 2 * (shells - genus)
}

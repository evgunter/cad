//! Adversarial review suite for M3 PR 2 (split part 1: reduction +
//! neighborhood classification). Independent derivations — fixtures and
//! expected censuses derived from the geometry by the reviewer, not
//! copied from the shipped tests.
//!
//! Naming: R1.. tags match the review report's findings/witnesses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::{Point3, Vec3};
use topo::{Body, PlaneSide, SplitPlane, SplitReduceError, VertexKey, split_reduce};

fn plane_y<T: geom_core::Decide>(y: f64, ny: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(y), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(0.0), T::from_f64(ny), T::from_f64(0.0)),
    }
}

fn point_of(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn vertex_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> VertexKey {
    let hits: Vec<_> = body
        .vertices()
        .filter(|(_, v)| {
            let p = *body.get_point(v.point).unwrap();
            p.x == x && p.y == y && p.z == z
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(hits.len(), 1, "expected one vertex at ({x},{y},{z})");
    hits[0]
}

/// Does any edge of `body` join vertices `a` and `b` (either order)?
fn joins(body: &Body<f64>, a: VertexKey, b: VertexKey) -> bool {
    body.edges().any(|(_, e)| {
        let s1 = body.get_half_edge(e.he_plus).unwrap().start;
        let s2 = body.get_half_edge(e.he_minus).unwrap().start;
        (s1 == a && s2 == b) || (s1 == b && s2 == a)
    })
}

/// R1a — independent tangent-edge fixture (reviewer's own dims: block
/// [0,10]x[0,4], V-notch tip at (6,2), split plane y = 2). The AOA
/// adjudication's load-bearing consequence, derived independently:
/// Above's material at the tip is two wedges whose face fans are
/// DISJOINT (slantL+capL vs slantR+capR share no face), and a half-edge
/// vertex admits exactly one cyclic orbit — so one shared copy cannot
/// host both wedges. AOA→BELOW must therefore mint TWO copies, and each
/// copy's orbit must be exactly {its slant half-edge, its null half}.
#[test]
fn r1a_tangent_tip_two_disjoint_copies_with_two_edge_orbits() {
    let profile = [
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (8.0, 4.0),
        (6.0, 2.0), // V tip ON y = 2
        (4.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(2.0, 1.0)).unwrap();
    for z in [0.0, 1.0] {
        let tip = vertex_at(&fx.body, 6.0, 2.0, z);
        let recs: Vec<_> = red
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        assert_eq!(recs.len(), 2, "two ABOVE runs at the tip");
        assert!(recs.iter().all(|r| !r.dangling));
        let copies = [recs[0].attr.above_end, recs[1].attr.above_end];
        assert_ne!(copies[0], copies[1], "distinct copies (book AOA→BELOW)");
        // Each copy's orbit: exactly one real (slant) half-edge plus its
        // null half — a single wedge fan per copy, never both.
        for &c in &copies {
            let anchor = red.body.get_vertex(c).unwrap().emanating.unwrap();
            let orbit = red.body.vertex_orbit(anchor).unwrap();
            assert_eq!(orbit.len(), 2, "copy orbit = {{slant, null}}");
            // Exactly one orbit member ends Above (the slant); the other
            // ends ON (the null edge back to the old vertex).
            let mut ends: Vec<_> = orbit
                .iter()
                .map(|&he| red.sides[red.body.half_edge_end(he).unwrap()])
                .collect();
            ends.sort_by_key(|s| matches!(s, PlaneSide::On));
            assert_eq!(ends, vec![PlaneSide::Above, PlaneSide::On]);
        }
        // The tip edge itself stays on the old (Below-side) vertices.
        let tip_other = vertex_at(&fx.body, 6.0, 2.0, 1.0 - z);
        assert!(joins(&red.body, tip, tip_other), "tip edge kept below");
    }
}

/// R1b — split-plane orientation equivariance: the argument that
/// actually FORCES BOB→ABOVE once AOA→BELOW is established. Splitting
/// the touching-wedge body by (o, +n) and by (o, −n) must assign the
/// tangent tip edge to the same PHYSICAL side (the physically-above
/// piece), because the −n run reads the same physical configuration as
/// AOA. Book's table (AOA→B, BOB→A) is swap-symmetric and passes; a
/// mixed table (e.g. book AOA + TOG BOB) would fail this test.
/// Reviewer's own wedge: block [0,10]x[0,4], Λ tip from below at (3,2).
#[test]
fn r1b_orientation_equivariance_pins_bob_from_aoa() {
    let profile = [
        (0.0, 0.0),
        (2.0, 0.0),
        (3.0, 2.0), // Λ tip ON y = 2, material above
        (4.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let (tip_b, tip_t) = (
        vertex_at(&fx.body, 3.0, 2.0, 0.0),
        vertex_at(&fx.body, 3.0, 2.0, 1.0),
    );

    // +n (Above = +y): BOB→ABOVE moves the tip edge to Above copies —
    // physically the above side. One dangling null (wide cap bisector).
    let red_pos = split_reduce(&fx.body, &plane_y(2.0, 1.0)).unwrap();
    assert!(
        !joins(&red_pos.body, tip_b, tip_t),
        "+n: tip edge left the old vertices (BOB→ABOVE)"
    );
    for tip in [tip_b, tip_t] {
        let recs: Vec<_> = red_pos
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs.iter().filter(|r| r.dangling).count(), 1);
    }

    // −n (frame-Above = physical below): the tip context reads AOA;
    // AOA→BELOW keeps the tip edge on the OLD vertices — which are the
    // frame-below = physically-ABOVE side. Same physical assignment.
    let red_neg = split_reduce(&fx.body, &plane_y(2.0, -1.0)).unwrap();
    assert!(
        joins(&red_neg.body, tip_b, tip_t),
        "−n: tip edge stays on old vertices (AOA→BELOW) = physically above"
    );
    for tip in [tip_b, tip_t] {
        let recs: Vec<_> = red_neg
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        // Two frame-Above (physically below) wedge runs ⇒ two copies,
        // none dangling: the physically-below rim edges get the copies.
        assert_eq!(recs.len(), 2);
        assert!(recs.iter().all(|r| !r.dangling));
        let copies: Vec<_> = recs.iter().map(|r| r.attr.above_end).collect();
        assert_ne!(copies[0], copies[1]);
    }
}

/// R2 — the tangency residue, executed: a triangular prism touching the
/// plane from above along its apex edge only (no below material
/// anywhere). Documents EXACTLY what PR 3 receives: one wrapped ABOVE
/// run per apex vertex ⇒ one non-dangling null edge; the old vertex is
/// left holding only the bare tangent edge plus the null half — the
/// degenerate Below piece the adjudication record promises PR 3 must
/// detect and refuse. NOTE the `dangling` flag is FALSE here: PR 3 has
/// no PR 2-level flag for this residue, only the reconstructable fact
/// that the below side of these ON vertices holds no real material.
#[test]
fn r2_one_sided_tangency_residue_documented() {
    let profile = [(3.0, 4.0), (6.0, 1.0), (9.0, 4.0)]; // CCW, apex down
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0)).unwrap();
    assert_eq!(red.on_vertices.len(), 2); // apex bottom + top
    assert_eq!(red.null_edges.len(), 2); // one per apex vertex
    assert!(red.null_edges.iter().all(|r| !r.dangling));
    for z in [0.0, 1.0] {
        let apex = vertex_at(&fx.body, 6.0, 1.0, z);
        let anchor = red.body.get_vertex(apex).unwrap().emanating.unwrap();
        let orbit = red.body.vertex_orbit(anchor).unwrap();
        // Old vertex keeps: the tangent (apex) edge + the null half.
        assert_eq!(orbit.len(), 2, "below residue = bare edge + null");
        let ends: Vec<_> = orbit
            .iter()
            .map(|&he| red.sides[red.body.half_edge_end(he).unwrap()])
            .collect();
        assert!(ends.iter().all(|&s| s == PlaneSide::On));
    }
}

/// R3 — rule (a) neighbor propagation on an all-ON neighborhood: a
/// collinear ON run (6,1)-(4,1)-(2,1) puts a vertex whose EVERY orbit
/// entry starts ON (two coplanar-sector edges + the strut). Rule (a)
/// must reclassify all three Below via the two flanking coplanar side
/// faces (material below ⇒ outward +y ⇒ BELOW), the straight (exactly
/// 180°) cap corner must take the convex-subdivision duplicate without
/// escalating, and NO ConsecutiveOnSectors false-refusal may fire.
/// Census derived independently: ON = 6 structural + 4 crossings; null
/// edges = 4 crossings + 2 notch corners × 2 rims = 8; the mid vertex
/// (4,1) mints ZERO null edges (its whole neighborhood is Below).
#[test]
fn r3_collinear_on_run_all_on_neighborhood() {
    let profile = [
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (6.0, 1.0),
        (4.0, 1.0), // mid vertex of the collinear ON run
        (2.0, 1.0),
        (0.0, 2.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0)).unwrap();
    assert_eq!(red.on_vertices.len(), 10);
    assert_eq!(red.null_edges.len(), 8);
    for z in [0.0, 1.0] {
        let mid = vertex_at(&fx.body, 4.0, 1.0, z);
        assert_eq!(
            red.null_edges.iter().filter(|r| r.at_vertex == mid).count(),
            0,
            "all-Below neighborhood mints nothing"
        );
        for (x, expect) in [(6.0, 1), (2.0, 1)] {
            let v = vertex_at(&fx.body, x, 1.0, z);
            assert_eq!(red.null_edges.iter().filter(|r| r.at_vertex == v).count(), expect);
        }
    }
}

/// R4 — straight-band soundness probe: an ON vertex whose cap-face
/// corner is EXACTLY 180° (collinear diagonal boundary through the
/// plane) exercises the sin≈0/cosine-disambiguation path and the
/// claimed no-escalation-cliff. Independent truth: the above material
/// at (4,1) is ONE contiguous wedge (side-face above part + cap above
/// part), so exactly ONE null edge may be minted here — a duplicate
/// landing cyclically non-adjacent to its material would split the
/// wedge into two runs and falsify the convex-subdivision claim.
#[test]
fn r4_straight_cap_corner_single_wedge_single_null_edge() {
    let profile = [
        (0.0, 0.0),
        (3.0, 0.0),
        (4.0, 1.0), // ON; neighbors (3,0) and (5,2) are collinear
        (5.0, 2.0),
        (8.0, 2.0),
        (8.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0)).unwrap();
    // Crossings: x=0 wall rims at y=1 (2 of them: z=0, z=1) — plus the
    // two structural ON vertices at (4,1).
    assert_eq!(red.on_vertices.len(), 4);
    for z in [0.0, 1.0] {
        let v = vertex_at(&fx.body, 4.0, 1.0, z);
        let recs: Vec<_> = red
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == v)
            .collect();
        assert_eq!(recs.len(), 1, "one contiguous above wedge ⇒ one run");
        assert!(!recs[0].dangling);
    }
    // Total: 2 crossings + 2 straight-corner vertices.
    assert_eq!(red.null_edges.len(), 4);
}

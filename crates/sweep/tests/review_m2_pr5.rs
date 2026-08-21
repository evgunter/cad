//! Adversarial e2e review suite for M2 PR 5 (revolve) — promotable as
//! `review_m2_pr5`. `survives_*` tests pass as-is and pin behavior the
//! review verified; `finding_*` tests pin defects (FINDING comments).
//!
//! Assignments covered here (f64 lane): two-band wire construction
//! (multi-segment wires, cosurface across bands, pole valence), washer
//! zip lineage, Seam-vs-tier-3 honesty (rotated frames, forged seams),
//! witness antipode bitwise pin, an independent volume oracle
//! (dense fan vs Pappus), axis-contact trilean rows, error-path
//! reachability, and the D9 cross-profile dump harness.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use profile::RawLoop;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_8, PI, TAU};

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeGeometry;
use geom_core::{Point2, Point3, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use revolve_common::*;
use sweep::{Revolution, RevolveAxis, RevolveError, Revolved, RevolvedKind, revolve};
use topo::{Body, EdgeKey, FaceKey, LoopBoundary};
use geom_core::Tol;

// =====================================================================
// Shared attack helpers
// =====================================================================

/// Vertex valence: number of half-edges starting at the vertex.
fn valence(body: &Body<f64>, v: topo::VertexKey) -> usize {
    body.half_edges().filter(|(_, h)| h.start == v).count()
}

/// A vertex key by (approximate) world position.
fn vertex_at(body: &Body<f64>, p: Point3<f64>) -> topo::VertexKey {
    body.vertices()
        .find(|(_, v)| body.get_point(v.point).unwrap().distance(p) < 1e-9)
        .map(|(k, _)| k)
        .expect("vertex at position")
}

/// Dense (`m` interior samples per half-edge) probe points of a loop,
/// traversal-ordered — the reviewer's independently written probe
/// (mirrors `loop_probe_points`' direction rule, denser).
fn dense_loop_points(body: &Body<f64>, lk: topo::LoopKey, m: usize) -> Vec<Point3<f64>> {
    let LoopBoundary::Cycle { first } = body.get_loop(lk).unwrap().boundary else {
        panic!("empty loop");
    };
    let mut pts = Vec::new();
    for he in body.loop_cycle(first).unwrap() {
        let hd = body.get_half_edge(he).unwrap();
        pts.push(
            *body
                .get_point(body.get_vertex(hd.start).unwrap().point)
                .unwrap(),
        );
        let edge = body.get_edge(hd.edge).unwrap();
        let forward = edge.he_plus == he;
        let ec = body
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = ec.params();
        for i in 1..m {
            let s = i as f64 / m as f64;
            let t = if forward {
                t0 + (t1 - t0) * s
            } else {
                t1 + (t0 - t1) * s
            };
            pts.push(ec.carrier().eval(t));
        }
    }
    pts
}

/// The reviewer's independently written signed-volume oracle: dense
/// boundary fans (64 samples per half-edge). Sound for faces whose
/// boundary is not coplanar; magnitude converges to the true volume as
/// the sampling refines (chordal).
fn my_signed_volume(body: &Body<f64>) -> f64 {
    let mut six_v = 0.0;
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = dense_loop_points(body, lk, 64);
            let p1 = pts[0];
            for i in 1..pts.len() - 1 {
                let a = p1 - Point3::origin();
                let b = pts[i] - Point3::origin();
                let c = pts[i + 1] - Point3::origin();
                six_v += a.dot(b.cross(c));
            }
        }
    }
    six_v / 6.0
}

/// Azimuth of a point about an axisymmetric surface's own frame
/// (u_ref/axis), in (−π, π]. Used to verify a `Seam` edge really is
/// the u = 0 iso-curve of ITS surface.
fn azimuth_about(surface: &Surface<f64>, p: Point3<f64>) -> f64 {
    let (anchor, axis, u_ref): (Point3<f64>, Vec3<f64>, Vec3<f64>) = match *surface {
        Surface::Cylinder {
            origin,
            axis,
            u_ref,
            ..
        } => (origin, axis, u_ref),
        Surface::Cone {
            apex, axis, u_ref, ..
        } => (apex, axis, u_ref),
        Surface::Sphere {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
        Surface::Torus {
            center,
            axis,
            u_ref,
            ..
        } => (center, axis, u_ref),
        _ => panic!("not axisymmetric"),
    };
    let w = p - anchor;
    let w_perp = w - axis * w.dot(axis);
    let v_ref = axis.cross(u_ref);
    w_perp.dot(v_ref).atan2(w_perp.dot(u_ref))
}

/// Every `Seam` edge of the body: (edge, surface key).
fn seam_edges(body: &Body<f64>) -> Vec<(EdgeKey, topo::SurfaceKey)> {
    body.edges()
        .filter_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            match *c.description() {
                EdgeGeometry::Seam { surface } => Some((k, surface)),
                _ => None,
            }
        })
        .collect()
}

/// Asserts every Seam edge's samples sit at azimuth ≈ 0 of its own
/// surface (the u = 0 alignment claim, checked by the reviewer's own
/// azimuth computation, not by re-running certification).
fn assert_seams_on_u0(body: &Body<f64>) {
    let seams = seam_edges(body);
    assert!(!seams.is_empty(), "shape was expected to carry Seam edges");
    for (ek, sk) in seams {
        let surface = body.get_surface(sk).unwrap();
        let e = body.get_edge(ek).unwrap();
        let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = c.params();
        for i in 1..8 {
            let t = t0 + (t1 - t0) * (i as f64 / 8.0);
            let p = c.carrier().eval(t);
            let az = azimuth_about(surface, p);
            // Pole-adjacent samples have shrinking w_perp; the angle is
            // still well-defined off the exact pole.
            assert!(az.abs() < 1e-9, "seam edge {ek:?} sample at azimuth {az}");
        }
    }
}

fn wall_key(body: &Body<f64>, f: FaceKey) -> topo::SurfaceKey {
    body.get_face(f).unwrap().surface
}

/// The reviewer's kernel-coupled MAGNITUDE oracle: Pappus/Green from
/// the STORED meridian chain carriers — V = |θ|/2 · |∮ r² dz| with
/// r/z measured against the placed axis, the line integral sampled
/// densely (256 chords per edge) along each meridian edge's carrier in
/// he_plus order. An axis-run gap contributes r = 0, so open (wire)
/// chains integrate correctly. Independent of faces, loops, zip, and
/// the fan oracle; converges to the true volume.
///
/// (Found while auditing the fan oracle: the boundary-point fan
/// replaces each curved face by the CONE over its boundary, so its
/// magnitude is far below the enclosed volume — e.g. π vs 3π for the
/// washer. Its sign remains a sound orientation oracle, which is all
/// the shipped tests assert. This helper is the magnitude side the
/// fan cannot provide.)
fn meridian_pappus_volume(
    body: &Body<f64>,
    meridians: &[EdgeKey],
    angle: f64,
    axis_o: Point3<f64>,
    axis_d: Vec3<f64>,
) -> f64 {
    let d = axis_d.normalize();
    let n = 256;
    let mut integral = 0.0;
    for &ek in meridians {
        let e = body.get_edge(ek).unwrap();
        let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = c.params();
        let mut prev = c.carrier().eval(t0);
        for i in 1..=n {
            let t = t0 + (t1 - t0) * (i as f64 / n as f64);
            let p = c.carrier().eval(t);
            let r_mid = {
                let m = prev.lerp(p, 0.5);
                let w = m - axis_o;
                let w_perp = w - d * w.dot(d);
                w_perp.norm()
            };
            let dz = (p - axis_o).dot(d) - (prev - axis_o).dot(d);
            integral += r_mid * r_mid * dz;
            prev = p;
        }
    }
    (angle.abs() / 2.0) * integral.abs()
}

/// [`meridian_pappus_volume`] for the canonical y-axis tests, taking
/// the meridian chain out of a full revolve's key bundle (omitted
/// on-axis segments contribute nothing).
fn full_pappus_y(t: &Revolved<f64>) -> f64 {
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve")
    };
    let chain: Vec<EdgeKey> = meridians.iter().filter_map(|m| *m).collect();
    meridian_pappus_volume(
        &t.body,
        &chain,
        TAU,
        Point3::origin(),
        Vec3::new(0.0, 1.0, 0.0),
    )
}

// =====================================================================
// Assignment 1 — the two-band wire construction, attacked with more
// interior vertices than any shipped test (3 interior wire vertices,
// mixed plane/cylinder/cone walls) and with a cosurface pair INSIDE
// the wire.
// =====================================================================

/// Dome: base disc + cylinder + cone frustum wall + top annulus, axis
/// run closing. Wire k = 4, interior vertices 3.
fn dome() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 1.0),
        p2(0.5, 1.5),
        p2(0.0, 1.5),
    ])
}

#[test]
fn survives_wire_four_segment_dome_two_band_structure() {
    let vp = validated(vec![dome()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // k = 4 wire segments, 3 interior vertices:
    // V = 2 tips + 3·2 = 8; E = 4 + 4 meridians + 3 + 3 half-rims = 14;
    // F = 4 + 4 = 8; R = 0. Component E–P: 8 − 14 + 8 − 0 = 2 ⇒ g = 0.
    assert_eq!(counts(&t.body), (8, 14, 8, 0));
    assert_eq!(t.body.solids().count(), 1);
    assert_eq!(t.body.shells().count(), 1);
    // Exactly 4 surfaces: each segment's band pair shares ONE key.
    assert_eq!(t.body.surfaces().count(), 4);
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_meridians,
        pi_rims,
    } = &t.kind
    else {
        panic!("full");
    };
    for (j, pw) in pi_walls.iter().enumerate().take(4) {
        let b1 = t.walls[0][j].expect("band-1 wall");
        let b2 = pw.expect("band-2 wall");
        assert_ne!(b1, b2, "bands are distinct faces");
        assert_eq!(
            wall_key(&t.body, b1),
            wall_key(&t.body, b2),
            "segment {j}: bands must share one surface key"
        );
    }
    assert!(
        t.walls[0][4].is_none() && pi_walls[4].is_none(),
        "axis run omitted"
    );
    // Wall catalog: plane, cylinder, cone, plane.
    let kind_of = |f: FaceKey| t.body.get_surface(wall_key(&t.body, f)).unwrap().clone();
    assert!(matches!(
        kind_of(t.walls[0][0].unwrap()),
        Surface::Plane { .. }
    ));
    assert!(matches!(
        kind_of(t.walls[0][1].unwrap()),
        Surface::Cylinder { .. }
    ));
    let Surface::Cone {
        apex, half_angle, ..
    } = kind_of(t.walls[0][2].unwrap())
    else {
        panic!("cone wall");
    };
    assert!(apex.distance(Point3::new(0.0, 2.0, 0.0)) < 1e-12);
    assert!((half_angle - FRAC_PI_2 / 2.0).abs() < 1e-12);
    assert!(matches!(
        kind_of(t.walls[0][3].unwrap()),
        Surface::Plane { .. }
    ));
    // Tips (axis endpoints) have valence exactly 2 (the two-band
    // requirement; tier 2's strut ban forces this).
    for p in [Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.5, 0.0)] {
        assert_eq!(valence(&t.body, vertex_at(&t.body, p)), 2, "tip {p:?}");
    }
    // Interior vertices (both copies): two meridians + the band-1
    // half-rim + the band-2 half-rim = valence 4. (Cross-check:
    // 2E = 28 = 2 tips·2 + 6 interior·4.)
    for p in [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.5, 1.5, 0.0),
        Point3::new(-1.0, 0.0, 0.0),
        Point3::new(-1.0, 1.0, 0.0),
        Point3::new(-0.5, 1.5, 0.0),
    ] {
        assert_eq!(valence(&t.body, vertex_at(&t.body, p)), 4, "interior {p:?}");
    }
    // Meridian descriptions: angle-0 seam on periodic walls, plane
    // walls conventional; every angle-π copy conventional.
    assert!(matches!(
        description(&t.body, meridians[0].unwrap()),
        EdgeGeometry::MappedCurve(_)
    ));
    assert!(matches!(
        description(&t.body, meridians[1].unwrap()),
        EdgeGeometry::Seam { .. }
    ));
    assert!(matches!(
        description(&t.body, meridians[2].unwrap()),
        EdgeGeometry::Seam { .. }
    ));
    assert!(matches!(
        description(&t.body, meridians[3].unwrap()),
        EdgeGeometry::MappedCurve(_)
    ));
    assert!(meridians[4].is_none());
    for pm in pi_meridians.iter().take(4) {
        assert!(matches!(
            description(&t.body, pm.unwrap()),
            EdgeGeometry::MappedCurve(_)
        ));
    }
    // All 3 interior joins are transverse (plane×cyl, cyl×cone,
    // cone×plane): Intersection in BOTH bands.
    for (v, pr) in pi_rims.iter().enumerate().take(4).skip(1) {
        for e in [t.rims[0][v].unwrap(), pr.unwrap()] {
            assert!(matches!(
                description(&t.body, e),
                EdgeGeometry::Intersection { .. }
            ));
        }
    }
    assert!(t.rims[0][0].is_none() && t.rims[0][4].is_none());
    // u = 0 alignment of every Seam (reviewer's own azimuth check).
    assert_seams_on_u0(&t.body);
    // Sign + kernel-coupled magnitude: cylinder π + frustum 7π/24.
    assert!(my_signed_volume(&t.body) > 0.0);
    let v = full_pappus_y(&t);
    let exact = PI * 31.0 / 24.0;
    assert!(
        (v - exact).abs() / exact < 1e-6,
        "dome volume {v} vs {exact}"
    );
}

#[test]
fn survives_wire_cosurface_pair_inside_the_wire() {
    // Cylinder wall split into two collinear segments at (1, 1): the
    // cosurface run must yield ONE cylinder key across all FOUR band
    // faces, and the split rims (both bands) stay conventional.
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (8, 14, 8, 0));
    // plane + ONE cylinder + plane.
    assert_eq!(t.body.surfaces().count(), 3);
    let RevolvedKind::Full {
        pi_walls, pi_rims, ..
    } = &t.kind
    else {
        panic!("full");
    };
    let keys: Vec<_> = [
        t.walls[0][1].unwrap(),
        t.walls[0][2].unwrap(),
        pi_walls[1].unwrap(),
        pi_walls[2].unwrap(),
    ]
    .iter()
    .map(|&f| wall_key(&t.body, f))
    .collect();
    assert!(
        keys.windows(2).all(|w| w[0] == w[1]),
        "one cylinder key: {keys:?}"
    );
    // The cosurface split vertex (canonical 2, at (1,1)): rims stay
    // conventional in both bands (same key both sides).
    for e in [t.rims[0][2].unwrap(), pi_rims[2].unwrap()] {
        assert!(matches!(
            description(&t.body, e),
            EdgeGeometry::MappedCurve(_)
        ));
    }
    // The genuine corners at (1,0) and (1,2): Intersection both bands.
    for v in [1, 3] {
        for e in [t.rims[0][v].unwrap(), pi_rims[v].unwrap()] {
            assert!(matches!(
                description(&t.body, e),
                EdgeGeometry::Intersection { .. }
            ));
        }
    }
    assert_seams_on_u0(&t.body);
    // Independent volume: Pappus for the cylinder solid of revolution:
    // V = π·r²·h = π·1²·2. Dense chordal fan (64 samples) from
    // boundary points is sound here (wire faces are half-bands whose
    // boundaries are NOT coplanar — they include half-rims).
    // Fan oracle: sign only (see meridian_pappus_volume docs).
    assert!(my_signed_volume(&t.body) > 0.0);
    // Kernel-coupled magnitude: Pappus V = π·r²·h = 2π.
    let v = full_pappus_y(&t);
    let exact = PI * 2.0;
    assert!(
        (v - exact).abs() / exact < 1e-6,
        "cylinder volume {v} vs {exact}"
    );
}

#[test]
fn survives_ball_pole_valence_and_volume() {
    // The ball's poles must have valence exactly 2, and the reviewer's
    // dense fan (sound here: each half-band boundary contains only the
    // two coplanar meridians, so fan from a boundary point degenerates
    // — instead fan each band from an equatorial interior point, the
    // implementer's lifted-oracle shape, but with the reviewer's dense
    // samples and a magnitude bound from an independently computed
    // chordal expectation).
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    for p in [Point3::new(0.0, -1.0, 0.0), Point3::new(0.0, 1.0, 0.0)] {
        assert_eq!(valence(&t.body, vertex_at(&t.body, p)), 2, "pole {p:?}");
    }
    // Independent volume oracle for the two-band ball: mesh each band
    // by revolving the meridian carrier samples about the y-axis over
    // the band's angular range — built by the reviewer from the stored
    // carrier only, then compared against 4π/3.
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full")
    };
    let e = t.body.get_edge(meridians[0].unwrap()).unwrap();
    let c = t.body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (t0, t1) = c.params();
    let nv = 64;
    let nu = 128;
    let mut six_v = 0.0;
    for iu in 0..nu {
        let (u0, u1) = (
            TAU * iu as f64 / nu as f64,
            TAU * (iu + 1) as f64 / nu as f64,
        );
        for iv in 0..nv {
            let (v0, v1) = (
                t0 + (t1 - t0) * iv as f64 / nv as f64,
                t0 + (t1 - t0) * (iv + 1) as f64 / nv as f64,
            );
            let rot = |t: f64, u: f64| {
                let p = c.carrier().eval(t);
                // Rotate about the y-axis by angle −u (θ > 0 sweeps
                // toward −n = −z; the sign only flips orientation
                // globally, checked by the final sign assertion).
                let (s, co) = (-u).sin_cos();
                Point3::new(p.x * co + p.z * s, p.y, -p.x * s + p.z * co)
            };
            let q = [rot(v0, u0), rot(v1, u0), rot(v1, u1), rot(v0, u1)];
            for tri in [[q[0], q[1], q[2]], [q[0], q[2], q[3]]] {
                let a = tri[0] - Point3::origin();
                let b = tri[1] - Point3::origin();
                let cc = tri[2] - Point3::origin();
                six_v += a.dot(b.cross(cc));
            }
        }
    }
    let v = six_v / 6.0;
    let exact = 4.0 * PI / 3.0;
    assert!(
        (v.abs() - exact).abs() / exact < 2e-3,
        "ball meridian-revolution volume {v} vs {exact}"
    );
}

// =====================================================================
// Assignment 2 — washer zip lineage: which entities survive the
// kfmrh + mekr/kev + mef/kev/kef sequence (flagged unasserted by the
// implementer).
// =====================================================================

#[test]
fn survives_washer_zip_lineage_and_seam_state() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full")
    };
    // Lineage: the surviving edge set is EXACTLY the 4 original chain
    // edges (the meridians) plus the 4 full-period rims — every copied
    // chain edge, every zip null edge, and both seam discs are dead.
    let mut expected: Vec<EdgeKey> = meridians.iter().map(|m| m.unwrap()).collect();
    expected.extend(t.rims[0].iter().map(|r| r.unwrap()));
    expected.sort();
    let mut actual: Vec<EdgeKey> = t.body.edges().map(|(k, _)| k).collect();
    actual.sort();
    assert_eq!(actual, expected, "zip must kill all scaffolding");
    // Surviving vertices are the 4 original chain vertices (the
    // rotated copies died in the kevs), at the original coordinates.
    assert_eq!(t.body.vertices().count(), 4);
    for p in [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(2.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    ] {
        vertex_at(&t.body, p); // panics if missing
    }
    // The Seam state: each wall's outer loop traverses its own
    // meridian TWICE (both halves in one face), each full-period rim
    // is a self-loop (start vertex = end vertex) spanning exactly τ.
    for (j, w) in t.walls[0].iter().enumerate() {
        let face = t.body.get_face(w.unwrap()).unwrap();
        let LoopBoundary::Cycle { first } = t.body.get_loop(face.outer).unwrap().boundary else {
            panic!("wall loop");
        };
        let mer = meridians[j].unwrap();
        let hits = t
            .body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .filter(|&he| t.body.get_half_edge(he).unwrap().edge == mer)
            .count();
        assert_eq!(hits, 2, "wall {j}: both meridian halves in one face");
    }
    for r in &t.rims[0] {
        let e = t.body.get_edge(r.unwrap()).unwrap();
        let start = t.body.get_half_edge(e.he_plus).unwrap().start;
        let end = t.body.half_edge_end(e.he_plus).unwrap();
        assert_eq!(start, end, "full-period rim is a self-loop");
        let c = t.body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = c.params();
        assert_eq!(t0, 0.0);
        assert_eq!(t1, TAU, "rim spans exactly the full period");
    }
    // Independent magnitude oracle: Pappus V = 2π·R̄·A = 2π·1.5·1.
    // Fan oracle: sign only. Kernel-coupled magnitude via Pappus:
    // V = 2π·R̄·A = 3π (∮r²dz = 3 around the rectangle).
    assert!(my_signed_volume(&t.body) > 0.0);
    let v = full_pappus_y(&t);
    let exact = TAU * 1.5;
    assert!(
        (v - exact).abs() / exact < 1e-6,
        "washer volume {v} vs {exact}"
    );
}

#[test]
fn survives_four_arc_donut_wrap_run_single_torus() {
    // The full circle split into FOUR same-carrier arcs: the cosurface
    // run reaches segment 0 through the WRAP pair; the zip runs with
    // every wall sharing one torus key.
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.0, 0.5), b),
        ProfileVertex::new(p2(1.5, 1.0), b),
        ProfileVertex::new(p2(1.0, 0.5), b),
        ProfileVertex::new(p2(1.5, 0.0), b),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // V4 E8 F4 R0: E–P = 0 ⇒ g = 1.
    assert_eq!(counts(&t.body), (4, 8, 4, 0));
    assert_eq!(t.body.surfaces().count(), 1, "one torus for all four walls");
    let keys: Vec<_> = t.walls[0]
        .iter()
        .map(|w| wall_key(&t.body, w.unwrap()))
        .collect();
    assert!(keys.windows(2).all(|w| w[0] == w[1]), "{keys:?}");
    assert!(matches!(
        t.body.get_surface(keys[0]),
        Some(Surface::Torus { .. })
    ));
    // All rims conventional (same key both sides); all four meridian
    // arcs are Seam { torus } on the u = 0 minor circle.
    for r in &t.rims[0] {
        assert!(matches!(
            description(&t.body, r.unwrap()),
            EdgeGeometry::MappedCurve(_)
        ));
    }
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full")
    };
    for m in meridians {
        assert!(matches!(
            description(&t.body, m.unwrap()),
            EdgeGeometry::Seam { .. }
        ));
    }
    assert_seams_on_u0(&t.body);
    // Pappus: V = 2π·R̄·A = 2π·1.5·(π·0.5²).
    assert!(my_signed_volume(&t.body) > 0.0);
    // Pappus for the torus: V = 2π·R̄·A = 2π·1.5·π·0.25 (arc chords at
    // 256 samples per quarter carrier: relative error < 1e-4).
    let v = full_pappus_y(&t);
    let exact = TAU * 1.5 * PI * 0.25;
    assert!(
        (v - exact).abs() / exact < 1e-4,
        "donut volume {v} vs {exact}"
    );
}

// =====================================================================
// Assignment 3 — Seam semantics vs tier 3: the exemption must not be
// able to hide a wrong seam, and the u = 0 claim must survive frame
// changes.
// =====================================================================

#[test]
fn survives_forged_seam_on_pi_meridian_is_refused() {
    // The angle-π meridian of the ball is NOT the u = 0 iso-curve of
    // the sphere. Seam is exempt from tier 3's prefer-intrinsic pass
    // BY KIND — so the only thing standing between a forged Seam and a
    // silently wrong model is set_edge_curve's certification gate
    // (SeamSide: samples must sit on the u_ref side). Verify the gate
    // actually refuses.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let mut t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    let RevolvedKind::Full { pi_meridians, .. } = &t.kind else {
        panic!("full")
    };
    let pi_edge = pi_meridians[0].unwrap();
    let sphere_key = wall_key(&t.body, t.walls[0][0].unwrap());
    let e = t.body.get_edge(pi_edge).unwrap();
    let c = t.body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (carrier, (t0, t1)) = (c.carrier().clone(), c.params());
    let forged = geom_brep::EdgeCurveSpec {
        description: EdgeGeometry::Seam {
            surface: sphere_key,
        },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let err = t.body.set_edge_curve(pi_edge, forged, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, topo::EulerOpError::Certification { .. }),
        "forged Seam on the π meridian must be refused by certification: {err:?}"
    );
    // The body is untouched on Err: still fully tier-valid.
    assert_all_tiers(&t.body);
}

#[test]
fn survives_seam_alignment_under_rotated_placement_and_oblique_axis() {
    // Attack the "u = 0 = profile half-plane" claim away from the cozy
    // identity frame: a translated/rotated sketch placement AND an
    // in-sketch axis that is neither x nor y and misses the origin.
    let axis = RevolveAxis {
        origin: p2(0.25, -0.5),
        dir: Vec2::new(1.0, 2.0), // normalized internally
    };
    let d = Vec2::new(1.0, 2.0).normalize();
    let e_r = Vec2::new(d.y, -d.x);
    // Rectangle in axis-adapted coordinates: r ∈ [1, 2], z ∈ [0, 1].
    let corner = |r: f64, z: f64| {
        let p = axis.origin + d * z + e_r * r;
        p2(p.x, p.y)
    };
    let lp = ProfileLoop::polygon([
        corner(1.0, 0.0),
        corner(2.0, 0.0),
        corner(2.0, 1.0),
        corner(1.0, 1.0),
    ]);
    // Placed plane: origin off to the side, u/v a rotated orthonormal
    // pair (normal = u × v).
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let plane = SketchPlane::from_frame(
        Point3::new(5.0, -3.0, 2.0),
        Vec3::new(s, 0.0, s),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let t = revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (4, 8, 4, 0));
    // The reviewer's own azimuth check: every Seam edge sits at u = 0
    // of ITS surface, in the placed frame.
    assert_seams_on_u0(&t.body);
    // And the seam meridians coincide with the placed profile corners
    // (the u = 0 half-plane IS the profile half-plane): every surviving
    // vertex is a placed profile corner.
    for r in [1.0, 2.0] {
        for z in [0.0, 1.0] {
            let c = corner(r, z);
            let world = plane.to_world(c);
            vertex_at(&t.body, world); // panics if missing
        }
    }
    // Pappus in the tilted frame: V = 2π·1.5·1 regardless of placement
    // (kernel-coupled: measured against the PLACED axis).
    assert!(my_signed_volume(&t.body) > 0.0);
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full")
    };
    let chain: Vec<EdgeKey> = meridians.iter().filter_map(|m| *m).collect();
    let axis_o3 = plane.to_world(axis.origin);
    let axis_d3 = plane.to_world(axis.origin + d) - axis_o3;
    let v = meridian_pappus_volume(&t.body, &chain, TAU, axis_o3, axis_d3);
    let exact = TAU * 1.5;
    assert!(
        (v - exact).abs() / exact < 1e-6,
        "tilted washer volume {v} vs {exact}"
    );
}

// =====================================================================
// Assignment 4 — witness antipode, bitwise (flagged unasserted).
// =====================================================================

#[test]
fn survives_rim_witness_is_bitwise_mid_parameter_antipode() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    for r in &t.rims[0] {
        let e = t.body.get_edge(r.unwrap()).unwrap();
        let c = t.body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let EdgeGeometry::Intersection { witness, .. } = *c.description() else {
            panic!("washer rims are Intersections");
        };
        let (t0, t1) = c.params();
        let mid = c.carrier().eval(t0 + (t1 - t0) * 0.5);
        // Bitwise: the witness IS the certification schedule's own
        // mid-parameter association.
        assert_eq!(witness.x.to_bits(), mid.x.to_bits());
        assert_eq!(witness.y.to_bits(), mid.y.to_bits());
        assert_eq!(witness.z.to_bits(), mid.z.to_bits());
        // And it is the start point's antipode, not the start point.
        let start = c.carrier().eval(t0);
        let Curve3::Circle { center, .. } = c.carrier().clone() else {
            panic!("rim carrier")
        };
        assert!(
            witness.distance(start) > 1.0,
            "witness must not be the start point"
        );
        let antipode = Point3::new(
            2.0 * center.x - start.x,
            2.0 * center.y - start.y,
            2.0 * center.z - start.z,
        );
        assert!(
            witness.distance(antipode) < 1e-12,
            "witness at the antipode"
        );
    }
}

#[test]
fn survives_start_point_witness_on_full_rim_is_refused() {
    // WitnessMidpoint contract honesty: put the witness at the rim's
    // START point (which IS on both surfaces — the naive witness) and
    // verify certification refuses it on a full-period rim.
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let mut t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    let rim = t.rims[0][0].unwrap();
    let e = t.body.get_edge(rim).unwrap();
    let c = t.body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let EdgeGeometry::Intersection { s1, s2, .. } = *c.description() else {
        panic!("intersection rim");
    };
    let (carrier, (t0, t1)) = (c.carrier().clone(), c.params());
    let start = carrier.eval(t0);
    let forged = geom_brep::EdgeCurveSpec {
        description: EdgeGeometry::Intersection {
            s1,
            s2,
            witness: start,
        },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let err = t.body.set_edge_curve(rim, forged, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, topo::EulerOpError::Certification { .. }),
        "start-point witness must fail the mid-parameter pin: {err:?}"
    );
    assert_all_tiers(&t.body);
}

// =====================================================================
// Assignment 5 — orientation/volume oracle, magnitude-checked both
// sweep directions with the reviewer's independent dense fan.
// =====================================================================

#[test]
fn survives_wedge_volume_magnitude_both_signs() {
    // Unit square on the axis, |θ| = π/2. Pappus:
    // V = |θ|/2 · ∮ r² dz = (π/4)·1 (only the r = 1 edge contributes).
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let vp = validated(vec![lp]);
    for theta in [FRAC_PI_2, -FRAC_PI_2] {
        let t = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap();
        assert_all_tiers(&t.body);
        // Outwardness (sign) both sweep directions, by the reviewer's
        // own dense fan.
        assert!(
            my_signed_volume(&t.body) > 0.0,
            "outward orientation, theta {theta}"
        );
        // Kernel-coupled magnitude from the stored start-meridian
        // chain (the full profile loop, axis edge included at r = 0).
        let RevolvedKind::Partial {
            start_meridians, ..
        } = &t.kind
        else {
            panic!("partial")
        };
        let v = meridian_pappus_volume(
            &t.body,
            &start_meridians[0],
            theta,
            Point3::origin(),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let exact = FRAC_PI_2 / 2.0;
        assert!(
            (v - exact).abs() / exact < 1e-6,
            "wedge theta {theta}: volume {v} vs {exact}"
        );
    }
}

// =====================================================================
// Assignment 6 — axis-contact trilean honesty at every ε row (all
// margins are metered in multiples of eps(), so each row exercises its
// own band), plus the conservative-refusal false-positive hunt.
// =====================================================================

#[test]
fn survives_near_parallel_line_escalates_sliver_axis_clearance() {
    // Closing edge with dr = 3ε over dz = 1: neither a definite
    // cylinder nor a definite cone — must escalate as
    // SliverAxisClearance, never silently classify.
    let d = 3.0 * eps();
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0 + d, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(
        matches!(e, RevolveError::SliverAxisClearance { .. }),
        "{e:?}"
    );
}

#[test]
fn survives_near_perpendicular_line_escalates_sliver_axis_clearance() {
    // dz = 3ε over dr = 1: neither a definite plane annulus nor a
    // definite cone.
    let d = 3.0 * eps();
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, d), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(
        matches!(e, RevolveError::SliverAxisClearance { .. }),
        "{e:?}"
    );
}

#[test]
fn survives_negative_sliver_radius_is_typed_not_crossing() {
    // r = −3ε is in the sliver band, not a definite crossing: the
    // trilean must surface SliverRadius (escalation), not
    // VertexCrossesAxis.
    let d = 3.0 * eps();
    let lp = ProfileLoop::polygon([p2(-d, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(-d, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::SliverRadius { .. }), "{e:?}");
}

#[test]
fn survives_definite_near_band_classes_do_not_flip() {
    // Just OUTSIDE the band (r = 100ε > Kε = 10ε): definitely
    // off-axis — the revolve must succeed with a tiny inner cylinder,
    // at every ε row.
    let r = 100.0 * eps();
    let lp = ProfileLoop::polygon([p2(r, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(r, 1.0)]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (4, 8, 4, 0));
}

#[test]
fn survives_two_isolated_axis_vertices_pinch_is_non_manifold() {
    // Axis contact at two isolated VERTICES (no on-axis segment at
    // all): the revolved boundary pinches at both points — must be
    // NonManifoldAxisContact, not accepted and not MultipleAxisRuns
    // (there are zero runs).
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
        p2(0.5, 1.0),
    ]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(
        matches!(e, RevolveError::NonManifoldAxisContact { .. }),
        "{e:?}"
    );
    // Partially it revolves fine (ordinary shared cap vertices).
    let t = revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
}

#[test]
fn survives_tight_but_definite_torus_clearance_is_accepted() {
    // False-positive hunt on the conservative UnsupportedToroid
    // refusal: an arc whose carrier clears the axis by 1e−3 m — a
    // DEFINITE clearance at every ε row — must build a ring torus,
    // however visually extreme the tube is.
    let r_c = 0.998;
    let (cx, cy) = (1.0, 0.5);
    let a0 = -0.2_f64;
    let a1 = 0.2_f64;
    let p_lo = p2(cx + r_c * a0.cos(), cy + r_c * a0.sin());
    let p_hi = p2(cx + r_c * a1.cos(), cy + r_c * a1.sin());
    let bulge = ((a1 - a0) / 4.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p_lo, bulge),
        ProfileVertex::new(p_hi, 0.0),
        ProfileVertex::new(p2(1.5, cy + r_c * a1.sin()), 0.0),
        ProfileVertex::new(p2(1.5, cy + r_c * a0.sin()), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let torus_walls = t.walls[0]
        .iter()
        .filter(|w| {
            matches!(
                t.body.get_surface(wall_key(&t.body, w.unwrap())),
                Some(Surface::Torus { .. })
            )
        })
        .count();
    assert_eq!(
        torus_walls, 1,
        "the tight-clearance arc must still be a ring torus"
    );
}

// =====================================================================
// Assignment 7 — full-period rim certification honesty at the span
// boundary (the τ headroom band), driven through topo's public door.
// =====================================================================

#[test]
fn survives_near_full_period_rim_span_escalates() {
    // A lone-vertex strut whose circle spec spans τ − 3ε/r (headroom
    // in the band): certification must escalate, not accept-as-full
    // and not alias. Exactly-τ spans are accepted throughout the PR's
    // rims (every full-revolve test), so only the near-τ side needs
    // pinning here.
    let mut body = Body::<f64>::new();
    let q = Point3::new(1.0, 0.0, 0.0);
    let seed = body.mvfs(q).unwrap();
    let center = Point3::new(0.0, 0.0, 0.0);
    let radius = 1.0;
    let span = TAU - 3.0 * eps() / radius;
    let spec = geom_brep::EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(geom_brep::MappedCurve::RevolvedPoint {
            point: Point2::new(1.0, 0.0),
            place: geom_core::Affine3::identity(),
            axis_origin: center,
            axis_dir: Vec3::new(0.0, 1.0, 0.0),
            angle: span,
        }),
        carrier: Curve3::Circle {
            center,
            axis: Vec3::new(0.0, 1.0, 0.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        param_start: 0.0,
        param_end: span,
    };
    // A self-loop mev site back onto the same vertex: endpoints
    // coincide, so the only honest failure is the span headroom band.
    let err = body
        .mev(
            topo::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            q,
            spec,
            Tol::witness(),
        )
        .unwrap_err();
    assert!(
        matches!(err, topo::EulerOpError::Certification { .. }),
        "near-τ rim span must escalate: {err:?}"
    );
}

// =====================================================================
// Assignment 9 — remaining error classes: cosurface escalation IS
// reachable from a validated profile (contra the "believed
// unreachable" defensive posture), and the sliver-dihedral classes.
// =====================================================================

#[test]
fn survives_near_collinear_cone_join_is_refused_upstream() {
    // Two nearly collinear oblique segments: the join's cosurface
    // margin (perp distance d/√2 ≈ 2.1ε) is in the band. The typed
    // outcome must be an escalation (CosurfaceEscalated), never a
    // silent share/split choice.
    let d = 3.0 * eps();
    let lp = ProfileLoop::polygon([
        p2(1.0, 0.0),
        p2(2.0, 1.0),
        p2(3.0, 2.0 + d),
        p2(3.0, 3.0),
        p2(1.0, 3.0),
    ]);
    // FINDING-check resolved as SURVIVES: profile VALIDATION refuses
    // the near-collinear join first (chord_side escalation), so
    // revolve's CosurfaceEscalated defensive posture ("believed
    // unreachable from validated profiles") holds for line joins — the
    // sliver is caught one layer up, loudly, and never reaches the
    // cosurface predicate.
    let err = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("Escalated") || msg.contains("chord_side"),
        "expected an upstream validation escalation, got {msg}"
    );
}

// =====================================================================
// Assignment 8 — D9: cross-profile dump harness (diffed debug vs
// release by the review gates), covering all three construction paths
// (lamina zip, two-band wire, partial wedge).
// =====================================================================

#[test]
fn dump_for_cross_profile_diff() {
    let shapes: Vec<(Vec<ProfileLoop<f64>>, Revolution<f64>)> = vec![
        (
            vec![ProfileLoop::polygon([
                p2(1.0, 0.0),
                p2(2.0, 0.0),
                p2(2.0, 1.0),
                p2(1.0, 1.0),
            ])],
            Revolution::Full,
        ),
        (vec![dome()], Revolution::Full),
        (
            vec![ProfileLoop::polygon([
                p2(0.0, 0.0),
                p2(1.0, 0.0),
                p2(1.0, 1.0),
                p2(0.0, 1.0),
            ])],
            Revolution::Partial(FRAC_PI_2),
        ),
    ];
    let profile_tag = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let mut all = String::new();
    for (loops, rev) in shapes {
        let vp = validated(loops);
        let t: Revolved<f64> = revolve(&vp, axis_y(), rev, Tol::witness()).unwrap();
        all.push_str(&dump(&t));
        all.push_str("----\n");
    }
    // Same latent order-dependency as review_m2_pr4's dump: the tmpdir
    // may not exist on the test runner — create it first.
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join(format!("review-m2-pr5-dump-{profile_tag}.txt"));
    std::fs::write(&path, all).unwrap();
}

#[test]
#[ignore]
fn probe_fan_oracle_coned_volume() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Partial(FRAC_PI_2), Tol::witness()).unwrap();
    println!("impl oracle: {}", signed_volume(&t.body));
    for (fk, face) in t.body.faces() {
        let mut six_v = 0.0;
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = dense_loop_points(&t.body, lk, 64);
            let p1 = pts[0];
            for i in 1..pts.len() - 1 {
                let a = p1 - Point3::origin();
                let b = pts[i] - Point3::origin();
                let c = pts[i + 1] - Point3::origin();
                six_v += a.dot(b.cross(c));
            }
        }
        let surf = t
            .body
            .get_surface(t.body.get_face(fk).unwrap().surface)
            .unwrap();
        println!(
            "face {fk:?} contrib {} surf {:?}",
            six_v / 6.0,
            core::mem::discriminant(surf)
        );
        // print first few loop points
        let pts = dense_loop_points(&t.body, face.outer, 2);
        println!("  pts: {:?}", &pts[..pts.len().min(10)]);
    }
}

// =====================================================================
// Assignment 9 (continued) — sliver-dihedral hunts: drive the
// remaining typed classes (SliverJoin / SliverRim / the θ ≈ τ and
// θ ≈ π boundaries) from real profiles and pin what actually happens.
// =====================================================================

#[test]
fn survives_near_tangent_arc_join_classification() {
    // Cylinder wall then an arc leaving (2,1) at a start tangent
    // rotated ~4.5ε off vertical (bulge = tan(π/8 − 1.5ε·...)): the
    // wall–wall dihedral at the join is a genuine sliver. Mixed
    // line/arc kinds skip the cosurface share structurally, so this
    // reaches the DIHEDRAL classifier — the trilean must escalate
    // (SliverJoin), never silently pick corner-or-smooth. If profile
    // validation refuses first (its own tangency band), that is an
    // equally honest upstream refusal and this test pins whichever
    // layer speaks.
    let delta = 3.0 * eps();
    let bulge = (FRAC_PI_8 - delta / 2.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.0), bulge),
        ProfileVertex::new(p2(1.0, 2.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
    ]);
    match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Err(e) => {
            // Upstream tangency/join escalation: loud, typed. Fine.
            let msg = format!("{e:?}");
            assert!(msg.contains("Escalated"), "unexpected upstream error {msg}");
        }
        Ok(vp) => match revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()) {
            Err(RevolveError::SliverJoin { .. }) => {}
            Err(RevolveError::SliverRim { .. }) => {}
            other => panic!("sliver dihedral must escalate somewhere, got {other:?}"),
        },
    }
}

#[test]
fn survives_theta_near_pi_axis_edge_dihedral() {
    // θ = π − 3ε on the unit square (r_max = 1): the two cap planes
    // are a sliver away from coplanar. The cap–cap axis-edge dihedral
    // must escalate (SliverRim), never silently choose smooth-or-
    // transverse; θ = π ± 100ε must classify definitely (transverse ⇒
    // Intersection axis edge) — the near-band definite class must not
    // flip.
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let vp = validated(vec![lp]);
    match revolve(&vp, axis_y(), Revolution::Partial(PI - 3.0 * eps()), Tol::witness()) {
        Err(RevolveError::SliverRim { .. }) => {}
        Ok(t) => {
            // If the dihedral band did NOT trip (the margin is metered
            // through a lever that may scale it out of the band), the
            // result must still be fully tier-valid with an honest
            // axis-edge description.
            assert_all_tiers(&t.body);
        }
        other => panic!("theta near pi: unexpected {other:?}"),
    }
    // Definite side: θ = π − 1000ε is transverse — Intersection.
    let t = revolve(&vp, axis_y(), Revolution::Partial(PI - 1000.0 * eps()), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    let RevolvedKind::Partial {
        start_meridians, ..
    } = &t.kind
    else {
        panic!("partial")
    };
    assert!(matches!(
        description(&t.body, start_meridians[0][3]),
        EdgeGeometry::Intersection { .. }
    ));
}

#[test]
fn survives_angle_full_range_boundary_rows() {
    // θ = τ − 3ε (r_max = 2): headroom in the band — must escalate
    // (AngleEscalated), never accept-as-partial or alias to Full.
    // θ = τ − 1000ε: definite partial — must build.
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Partial(TAU - 3.0 * eps() / 2.0), Tol::witness()).unwrap_err();
    assert!(
        matches!(
            e,
            RevolveError::AngleEscalated { .. } | RevolveError::FullRangeAngle
        ),
        "{e:?}"
    );
    let t = revolve(&vp, axis_y(), Revolution::Partial(TAU - 1000.0 * eps()), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(counts(&t.body), (8, 12, 6, 0));
}

#[test]
fn survives_forged_seam_on_plane_wall_meridian_is_refused() {
    // The plane-meridian exception's other half: Seam on a
    // non-periodic chart must be structurally malformed
    // (SeamOnNonPeriodic), so the washer's plane-wall meridian can
    // never be silently "upgraded".
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let mut t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full")
    };
    // Canonical segment 1 ((2,0)->(2,1)) is a cylinder; segment 0
    // ((1,0)->(2,0)) sweeps the bottom plane annulus.
    let plane_meridian = meridians[0].unwrap();
    assert!(matches!(
        description(&t.body, plane_meridian),
        EdgeGeometry::MappedCurve(_)
    ));
    let plane_key = wall_key(&t.body, t.walls[0][0].unwrap());
    assert!(matches!(
        t.body.get_surface(plane_key),
        Some(Surface::Plane { .. })
    ));
    let e = t.body.get_edge(plane_meridian).unwrap();
    let c = t.body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    let (carrier, (t0, t1)) = (c.carrier().clone(), c.params());
    let forged = geom_brep::EdgeCurveSpec {
        description: EdgeGeometry::Seam { surface: plane_key },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    let err = t.body.set_edge_curve(plane_meridian, forged, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, topo::EulerOpError::Certification { .. }),
        "Seam on a plane chart must be refused: {err:?}"
    );
    assert_all_tiers(&t.body);
}

#[test]
fn survives_wire_cosurface_pair_at_segment_zero() {
    // Two collinear oblique segments FROM the axis tip ((0,0)->(1,1)->
    // (2,2)): the cosurface pair sits at wire vertex 1, so band-1 face
    // 1 shares band-1 face 0's cone key, and the band-2 carving +
    // set_face_surface(seed, Shared(wall0)) bookkeeping must still
    // leave ONE cone key across all four cone band faces. The seed
    // face still carries its Nurbs placeholder while the band-2 mefs
    // run — this pins that ordering.
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 1.0), p2(2.0, 2.0), p2(0.0, 2.0)]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // k = 3 wire segments (cone, cone, plane), 2 interior vertices:
    // V = 2 + 2·2 = 6; E = 3 + 3 + 2 + 2 = 10; F = 6; E–P = 2, g = 0.
    assert_eq!(counts(&t.body), (6, 10, 6, 0));
    // Surfaces: ONE cone + one plane.
    assert_eq!(t.body.surfaces().count(), 2);
    let RevolvedKind::Full {
        pi_walls, pi_rims, ..
    } = &t.kind
    else {
        panic!("full")
    };
    let cone_faces = [
        t.walls[0][0].unwrap(),
        t.walls[0][1].unwrap(),
        pi_walls[0].unwrap(),
        pi_walls[1].unwrap(),
    ];
    let k0 = wall_key(&t.body, cone_faces[0]);
    for f in &cone_faces[1..] {
        assert_eq!(wall_key(&t.body, *f), k0, "one cone key across bands");
    }
    assert!(matches!(t.body.get_surface(k0), Some(Surface::Cone { .. })));
    // The cosurface split rims (vertex 1) conventional in both bands;
    // the cone-plane corner (vertex 2) transverse in both bands.
    for e in [t.rims[0][1].unwrap(), pi_rims[1].unwrap()] {
        assert!(matches!(
            description(&t.body, e),
            EdgeGeometry::MappedCurve(_)
        ));
    }
    for e in [t.rims[0][2].unwrap(), pi_rims[2].unwrap()] {
        assert!(matches!(
            description(&t.body, e),
            EdgeGeometry::Intersection { .. }
        ));
    }
    // Pappus: cone r=z from 0..2 -> integral of z^2 dz = 8/3; top
    // annulus dz = 0. V = pi * 8/3.
    assert!(my_signed_volume(&t.body) > 0.0);
    let v = full_pappus_y(&t);
    let exact = PI * 8.0 / 3.0;
    assert!(
        (v - exact).abs() / exact < 1e-6,
        "megaphone volume {v} vs {exact}"
    );
}

/// Review probe (ignored in CI): prints which layer speaks for the
/// near-tangent arc join and the theta-near-pi cap dihedral.
#[test]
#[ignore]
fn probe_sliver_dihedral_arms() {
    let delta = 3.0 * eps();
    let bulge = (FRAC_PI_8 - delta / 2.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.0), bulge),
        ProfileVertex::new(p2(1.0, 2.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
    ]);
    match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Err(e) => println!("near-tangent arc: upstream validation: {e:?}"),
        Ok(vp) => println!(
            "near-tangent arc: revolve says {:?}",
            revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()).map(|_| "built")
        ),
    }
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let vp = validated(vec![lp]);
    println!(
        "theta = pi - 3eps: {:?}",
        revolve(&vp, axis_y(), Revolution::Partial(PI - 3.0 * eps()), Tol::witness()).map(|_| "built")
    );
}

#[test]
fn survives_wire_quarter_arc_sphere_cap_with_tangent_join() {
    // Silo: base disc + cylinder + quarter-arc sphere cap (arc
    // centered ON the axis at (0,1)) + axis run. The cylinder-sphere
    // join at (1,1) is TANGENT (smooth, distinct keys): the D2
    // conventional split must survive in BOTH bands, and tier 3 must
    // accept it while the transverse base join upgrades.
    let mut lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        // quarter arc to (0, 2), center (0,1)
        ProfileVertex::new(p2(1.0, 1.0), FRAC_PI_8.tan()),
        ProfileVertex::new(p2(0.0, 2.0), 0.0),
    ]);
    // The cylinder-sphere tangency this test is ABOUT is declared
    // (#101): the discipline gates the profile door; the D2 split and
    // tier-3 acceptance downstream are what the test pins.
    lp = lp.with_tangent_joints(vec![2]);
    let vp = validated(vec![lp]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    // k = 3 (plane, cylinder, sphere), 2 interior vertices:
    // V6 E10 F6 R0, 3 surfaces.
    assert_eq!(counts(&t.body), (6, 10, 6, 0));
    assert_eq!(t.body.surfaces().count(), 3);
    let RevolvedKind::Full {
        meridians,
        pi_walls,
        pi_rims,
        ..
    } = &t.kind
    else {
        panic!("full")
    };
    let sphere_key = wall_key(&t.body, t.walls[0][2].unwrap());
    assert!(matches!(
        t.body.get_surface(sphere_key),
        Some(Surface::Sphere { .. })
    ));
    assert_eq!(wall_key(&t.body, pi_walls[2].unwrap()), sphere_key);
    // Transverse base join: Intersection in both bands.
    for e in [t.rims[0][1].unwrap(), pi_rims[1].unwrap()] {
        assert!(matches!(
            description(&t.body, e),
            EdgeGeometry::Intersection { .. }
        ));
    }
    // Tangent cylinder-sphere join: INTRINSIC in both bands since M5
    // PR 12. HISTORY: this pin read `MappedCurve` from M2 to S13,
    // because the jet certificate's span bounds covered only LINE
    // carriers and a latitude join's carrier is a circle — so tier 3's
    // must-carry could not demand the intrinsic description and the
    // constructor did not store it. PR 12's circle arm retired that
    // class (with its equivariance proof), and prefer-intrinsic
    // (D2/OQ7) then demands what it always meant to: the cylinder and
    // the sphere DETERMINE this locus, `κ_rel = 1/R` is bounded away
    // from zero, and the description says so.
    for e in [t.rims[0][2].unwrap(), pi_rims[2].unwrap()] {
        assert!(matches!(
            description(&t.body, e),
            EdgeGeometry::TangentIntersection { .. }
        ));
    }
    // The sphere meridian arc is the seam; its pi copy conventional.
    assert!(matches!(
        description(&t.body, meridians[2].unwrap()),
        EdgeGeometry::Seam { .. }
    ));
    assert_seams_on_u0(&t.body);
    // Pappus: cylinder 1 + spherical cap 2/3 -> V = 5pi/3 (256-chord
    // arc sampling: relative error < 1e-4).
    assert!(my_signed_volume(&t.body) > 0.0);
    let v = full_pappus_y(&t);
    let exact = PI * 5.0 / 3.0;
    assert!(
        (v - exact).abs() / exact < 1e-4,
        "silo volume {v} vs {exact}"
    );
}

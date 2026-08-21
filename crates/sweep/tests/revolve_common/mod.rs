//! Shared helpers for the revolve acceptance suites (M2 PR 5).
//! Each `revolve_*.rs` integration binary includes this via
//! `mod revolve_common;`.
#![allow(dead_code)] // each test binary uses a subset

use geom_brep::EdgeGeometry;
use geom_core::{Point2, Point3, Tolerance, Vec2};
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::RevolveAxis;
use topo::{Body, EdgeKey, LoopBoundary, LoopKey, validate, validate_closed, validate_geometric};
use geom_core::Tol;

pub fn eps() -> f64 {
    Tol::witness().get().eps
}

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

/// The y-axis of the sketch plane (the canonical test axis: profiles
/// live in x ≥ 0).
pub fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

pub fn assert_all_tiers(body: &Body<f64>) {
    assert_eq!(validate(body), Ok(()));
    assert_eq!(validate_closed(body), Ok(()));
    assert_eq!(validate_geometric(body), Ok(()));
}

/// (v, e, f, r) of a body.
pub fn counts(body: &Body<f64>) -> (usize, usize, usize, usize) {
    let rings: usize = body.faces().map(|(_, f)| f.rings.len()).sum();
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        rings,
    )
}

/// The edge's stored description.
pub fn description(body: &Body<f64>, edge: EdgeKey) -> EdgeGeometry<f64> {
    let curve = body.get_edge(edge).unwrap().curve;
    *body
        .get_curve_geom(curve)
        .unwrap()
        .certified()
        .unwrap()
        .description()
}

/// Probe points of a loop in `next` order: each start vertex plus
/// interior carrier samples **in the half-edge's traversal direction**
/// (the mate half runs its carrier backward). Dense enough (7 interior
/// samples) that full-period rim circles polygonalize honestly — the
/// PR 4 review's oracle refined for revolve's closed carriers.
pub fn loop_probe_points(body: &Body<f64>, r#loop: LoopKey) -> Vec<Point3<f64>> {
    let LoopBoundary::Cycle { first } = body.get_loop(r#loop).unwrap().boundary else {
        panic!("loop has no cycle");
    };
    let mut pts = Vec::new();
    for he in body.loop_cycle(first).unwrap() {
        let he_data = body.get_half_edge(he).unwrap();
        pts.push(
            *body
                .get_point(body.get_vertex(he_data.start).unwrap().point)
                .unwrap(),
        );
        let edge = body.get_edge(he_data.edge).unwrap();
        let forward = edge.he_plus == he;
        let ec = body
            .get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap();
        let (t0, t1) = ec.params();
        for i in 1..8 {
            let s = f64::from(i) / 8.0;
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

/// Independent orientation oracle: signed volume by the divergence
/// fan over every loop of every face (sign is the oracle — chordal on
/// curved faces; the PR 4 review's `signed_volume`).
pub fn signed_volume(body: &Body<f64>) -> f64 {
    let mut six_v = 0.0;
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
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

/// [`signed_volume`] with per-face interior **lift points**: a face
/// listed in `lifts` is fanned from its lift point over the full
/// cyclic boundary (wrap included) instead of from a boundary vertex.
/// Needed for faces whose boundary probes are coplanar and span no
/// volume — the two-band sphere/cone patches, whose two meridians lie
/// in one plane. The lift must be an interior surface point of that
/// face (test-local geometric knowledge).
pub fn signed_volume_lifted(body: &Body<f64>, lifts: &[(topo::FaceKey, Point3<f64>)]) -> f64 {
    let mut six_v = 0.0;
    for (fk, face) in body.faces() {
        let lift = lifts.iter().find(|(k, _)| *k == fk).map(|(_, q)| *q);
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let pts = loop_probe_points(body, lk);
            match lift {
                Some(q) => {
                    let n = pts.len();
                    for i in 0..n {
                        let a = q - Point3::origin();
                        let b = pts[i] - Point3::origin();
                        let c = pts[(i + 1) % n] - Point3::origin();
                        six_v += a.dot(b.cross(c));
                    }
                }
                None => {
                    let p1 = pts[0];
                    for i in 1..pts.len() - 1 {
                        let a = p1 - Point3::origin();
                        let b = pts[i] - Point3::origin();
                        let c = pts[i + 1] - Point3::origin();
                        six_v += a.dot(b.cross(c));
                    }
                }
            }
        }
    }
    six_v / 6.0
}

/// A byte-comparable dump of the body's arenas plus the key bundle
/// (the D9 rebuild oracle; the PR 4 review's `dump` shape).
pub fn dump(t: &sweep::Revolved<f64>) -> String {
    let mut s = String::new();
    for (k, p) in t.body.points() {
        s.push_str(&format!("{k:?} {p:?}\n"));
    }
    for (k, c) in t.body.curves() {
        // Sweep bodies carry certified carriers only (no M3 null
        // scaffolding); dump the whole entry so a scaffolding entry
        // would still show loudly rather than being skipped.
        match c.certified() {
            Some(c) => s.push_str(&format!(
                "{k:?} {:?} {:?} {:?}\n",
                c.description(),
                c.params(),
                c.certificate()
            )),
            None => s.push_str(&format!("{k:?} {c:?}\n")),
        }
    }
    for (k, srf) in t.body.surfaces() {
        s.push_str(&format!("{k:?} {srf:?}\n"));
    }
    for (k, f) in t.body.faces() {
        s.push_str(&format!("{k:?} {f:?}\n"));
    }
    for (k, l) in t.body.loops() {
        s.push_str(&format!("{k:?} {l:?}\n"));
    }
    for (k, h) in t.body.half_edges() {
        s.push_str(&format!("{k:?} {h:?}\n"));
    }
    s.push_str(&format!("{:?} {:?} {:?}\n", t.walls, t.rims, t.kind));
    s
}

/// The kernel-coupled Pappus MAGNITUDE oracle, promoted from the PR 5
/// review suite (M2 PR 7): `V = |θ|/2 · |∮ r² dz|` with r/z measured
/// against the placed axis, the meridian line integral sampled densely
/// (256 chords per edge) along each meridian edge's stored carrier in
/// `he_plus` order. Independent of faces, loops, and the closed-form
/// divergence formulation — the sanity cross-check for revolved
/// bodies' exact mass properties (never the source of truth: it is a
/// converging quadrature, not a closed form).
pub fn meridian_pappus_volume(
    body: &Body<f64>,
    meridians: &[EdgeKey],
    angle: f64,
    axis_o: Point3<f64>,
    axis_d: geom_core::Vec3<f64>,
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
            let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(n));
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

/// [`meridian_pappus_volume`] for the canonical y-axis full revolves,
/// taking the meridian chain out of the key bundle (omitted on-axis
/// segments contribute nothing).
pub fn full_pappus_y(t: &sweep::Revolved<f64>) -> f64 {
    let sweep::RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve expected")
    };
    let chain: Vec<EdgeKey> = meridians.iter().filter_map(|m| *m).collect();
    meridian_pappus_volume(
        &t.body,
        &chain,
        core::f64::consts::TAU,
        Point3::origin(),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    )
}

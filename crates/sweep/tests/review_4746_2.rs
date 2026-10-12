//! Second-review probes for PR 4746 (seam regions in tier 3's check 6).
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic, clippy::expect_used)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use std::f64::consts::PI;
use sweep::test_support::{ball_poled, brick, finished};
use topo::{Body, FaceKey, FaceSurface, HalfEdgeKey, MefSite, ValidationError};

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn sphere_faces(b: &Body<f64>) -> Vec<FaceKey> {
    b.faces()
        .filter(|(_, f)| matches!(b.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect()
}

fn face(b: &Body<f64>, k: FaceKey) -> topo::Face {
    b.faces().find(|(x, _)| *x == k).unwrap().1.clone()
}

fn point_of(b: &Body<f64>, h: HalfEdgeKey) -> Point3<f64> {
    let s = b.get_half_edge(h).unwrap().start;
    b.vertex_points().find(|(k, _)| *k == s).unwrap().1
}

fn dump(label: &str, b: &Body<f64>) {
    eprintln!("== {label}");
    for k in sphere_faces(b) {
        let f = face(b, k);
        let (edges, hes) = topo::props::loop_edges(b, f.outer).unwrap();
        eprintln!("  face {k:?} surf {:?} sense {} rings {}", f.surface, f.sense, f.rings.len());
        for (e, h) in edges.iter().zip(&hes) {
            let p = point_of(b, *h);
            let other = b.face_of_half_edge(b.mate(*h).unwrap()).unwrap();
            eprintln!("    from {:.3},{:.3},{:.3} {:?} t {:.3}..{:.3} -> {other:?}", p.x, p.y, p.z,
                match e.carrier { Curve3::Circle { axis, radius, .. } => format!("circle r={radius:.3} ax={:.2},{:.2},{:.2}", axis.x, axis.y, axis.z), _ => "other".into() }, e.t0, e.t1);
        }
    }
}

fn tier3(label: &str, b: &Body<f64>, t: Tol) -> Result<f64, Vec<ValidationError>> {
    let r = topo::validate_geometric(b, t);
    let vol = topo::mass_properties(b, t).map(|m| m.volume);
    eprintln!("{label}: tier3 {:?} volume {vol:?}", r.as_ref().map_err(|e| e.len()));
    r.map(|()| vol.unwrap())
}

#[test]
fn review_4746_2_explore() {
    let t = Tol::witness();
    let ball = finished("ball", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), t), t);
    dump("ball", &ball);
    let cut = finished("cut", brick((0.8, 3.0), (-2.0, 2.0), (-2.0, 2.0), t), t);
    let b = Body::clone(&topo::boolean::subtract(&ball, &cut, t).unwrap().body().unwrap().body);
    dump("side cut", &b);
    let _ = tier3("side cut", &b, t);
}

/// Splits the face `key` (both poles on its loop as vertices) from pole
/// to pole along the meridian leaving the north pole in direction `dir`.
/// `Inherit` or a coincident new entity for the new face.
fn split_meridian(b: &mut Body<f64>, key: FaceKey, dir: Vec3<f64>, new_entity: bool, t: Tol) -> Result<(), String> {
    let f = face(b, key);
    let Some(&Surface::Sphere { center, radius, axis, .. }) = b.get_surface(f.surface) else { panic!() };
    let pole = axis.normalize();
    let (_, hes) = topo::props::loop_edges(b, f.outer).unwrap();
    let find = |p: Point3<f64>| *hes.iter().find(|&&h| (point_of(b, h) - p).norm() < 1e-9).expect("pole vertex");
    let (he1, he2) = (find(center + pole * radius), find(center - pole * radius));
    let circle = Curve3::Circle { center, axis: pole.cross(dir), radius, u_ref: pole };
    let surface = if new_entity {
        FaceSurface::New { surface: b.get_surface(f.surface).unwrap().clone(), sense: f.sense }
    } else {
        FaceSurface::Inherit
    };
    b.mef(
        MefSite::Chords { he1, he2 },
        EdgeCurveSpec::arc_of_circle(circle, 0.0, PI).unwrap().at_rest_in_chart(f.surface, false),
        surface,
        t,
    )
    .map(|_| ())
    .map_err(|e| format!("{e:?}"))
}

/// Splits a quarter-lune `key` at the equator: both its meridian edges
/// are split at their midpoints, then `mef` joins the two new vertices
/// along the equator, the NORTH piece being the new face, on a new
/// coincident entity when `new_entity`.
fn split_equator(b: &mut Body<f64>, key: FaceKey, new_entity: bool, t: Tol) -> Result<FaceKey, String> {
    let f = face(b, key);
    let Some(&Surface::Sphere { center, radius, axis, .. }) = b.get_surface(f.surface) else { panic!() };
    let pole = axis.normalize();
    let (edges, hes) = topo::props::loop_edges(b, f.outer).unwrap();
    for (e, h) in edges.iter().zip(&hes) {
        let ek = b.get_half_edge(*h).unwrap().edge;
        b.split_edge(ek, 0.5 * (e.t0 + e.t1), t).map_err(|e| format!("split {e:?}"))?;
    }
    let f = face(b, key);
    let (_, hes) = topo::props::loop_edges(b, f.outer).unwrap();
    let north = center + pole * radius;
    let eq: Vec<HalfEdgeKey> = hes.iter().copied().filter(|&h| (point_of(b, h) - center).dot(pole).abs() < 1e-9).collect();
    assert_eq!(eq.len(), 2, "two equator vertices");
    let end = |h: HalfEdgeKey| point_of(b, b.get_half_edge(h).unwrap().next);
    let (he1, he2) = if (end(eq[0]) - north).norm() < 1e-9 { (eq[0], eq[1]) } else { (eq[1], eq[0]) };
    let (a, c) = (point_of(b, he1), point_of(b, he2));
    let u = (a - center) / radius;
    let w = (c - center) / radius;
    let circle = Curve3::Circle { center, axis: u.cross(w).normalize(), radius, u_ref: u };
    let surface = if new_entity {
        FaceSurface::New { surface: b.get_surface(f.surface).unwrap().clone(), sense: f.sense }
    } else {
        FaceSurface::Inherit
    };
    let before: Vec<FaceKey> = b.faces().map(|(k, _)| k).collect();
    b.mef(
        MefSite::Chords { he1, he2 },
        EdgeCurveSpec::arc_of_circle(circle, 0.0, u.dot(w).clamp(-1.0, 1.0).acos()).unwrap().at_rest_in_chart(f.surface, false),
        surface,
        t,
    )
    .map_err(|e| format!("mef {e:?}"))?;
    Ok(b.faces().map(|(k, _)| k).find(|k| !before.contains(k)).unwrap())
}

/// The quarter-lunes of a z-poled ball, split by `mef` on one entity,
/// in rotation order about the north pole (+x+y, -x+y, -x-y, +x-y).
fn quarters(t: Tol) -> (Body<f64>, Vec<FaceKey>) {
    let ball = finished("ball", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), t), t);
    let mut b = Body::clone(&ball);
    for d in [v(0.0, 1.0, 0.0), v(0.0, -1.0, 0.0)] {
        // The lune whose interior holds d: try each, keep the valid one.
        let mut done = false;
        for k in sphere_faces(&b) {
            let mut c = b.clone();
            let ok = split_meridian(&mut c, k, d, false, t).is_ok();
            let r = topo::validate_geometric(&c, t);
            let vol = topo::mass_properties(&c, t).map(|m| m.volume);
            eprintln!("split {k:?} along {d:?}: mef ok {ok} tier3 {:?} volume {vol:?}", r.as_ref().map_err(|e| e.iter().map(|e| format!("{e:?}").chars().take(50).collect::<String>()).collect::<Vec<_>>()));
            if ok && r.is_ok() && vol.is_ok_and(|v| (v - 4.0 * PI / 3.0).abs() < 1e-9) {
                b = c;
                done = true;
                break;
            }
        }
        assert!(done, "a meridian split along {d:?}");
    }
    // Sort by the centroid direction of each quarter's loop vertices off the poles.
    let mut qs: Vec<(f64, FaceKey)> = sphere_faces(&b)
        .into_iter()
        .map(|k| {
            let f = face(&b, k);
            let (edges, _) = topo::props::loop_edges(&b, f.outer).unwrap();
            // midpoints of the two meridian edges, summed
            let mut s = v(0.0, 0.0, 0.0);
            for e in &edges {
                if let Curve3::Circle { center, axis, radius, u_ref } = e.carrier {
                    let m = 0.5 * (e.t0 + e.t1);
                    s = s + (u_ref * m.cos() + axis.normalize().cross(u_ref) * m.sin()) * radius + (center - Point3::new(0.0, 0.0, 0.0));
                }
            }
            (s.y.atan2(s.x).rem_euclid(2.0 * PI), k)
        })
        .collect();
    qs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    eprintln!("quarters at angles {:?}", qs.iter().map(|q| q.0).collect::<Vec<_>>());
    (b, qs.into_iter().map(|q| q.1).collect())
}

fn flips_report(label: &str, b: &Body<f64>, t: Tol) {
    let sph = sphere_faces(b);
    let truth = topo::mass_properties(b, t).map(|m| m.volume);
    let n = sph.len();
    let mut silent = 0;
    for mask in 1..(1u32 << n) {
        let set: Vec<FaceKey> = (0..n).filter(|i| mask & (1 << i) != 0).map(|i| sph[i]).collect();
        let fb = set.iter().fold(b.clone(), |b, &k| b.flipped_face_sense_for_tests(k).unwrap());
        if topo::validate_geometric(&fb, t).is_ok() {
            silent += 1;
            eprintln!("  {label}: flip {set:?} PASSES, volume {:?} vs {truth:?}", topo::mass_properties(&fb, t).map(|m| m.volume));
        }
    }
    eprintln!("{label}: {} flips, {silent} pass", (1u32 << n) - 1);
}

/// A pinched (bow-tie) seam region needs a second, coincident sphere
/// entity next to the first. The Euler path refuses it: `mef` minting
/// the north piece of a quarter-lune on a NEW coincident sphere is
/// `RechartUnvouched` (the run's edges are at rest on the old chart).
/// STEP import merges coincident SPHERICAL_SURFACEs into one entity
/// (`review_4746_2_import::review_4746_2_bowtie`). Pinned as found.
#[test]
fn review_4746_2_bowtie_pinch_is_refused_by_the_euler_path() {
    let t = Tol::witness();
    let (mut b, q) = quarters(t);
    let truth = 4.0 * PI / 3.0;
    assert!((tier3("quarters", &b, t).unwrap() - truth).abs() < 1e-9);
    let err = split_equator(&mut b, q[1], true, t).unwrap_err();
    assert!(err.contains("RechartUnvouched"), "{err}");
}

/// `mef` along a meridian OUTSIDE the lune it splits: the faces then
/// cover the sphere twice, every sphere face holds both poles on its
/// loop, and the seam region is the whole sphere (no boundary, so
/// `Unencoded`). Tier 3 passes and the volume is twice the ball's.
/// Same on main (458b60ec). Pinned as it stands.
#[test]
fn review_4746_2_an_exterior_meridian_doubles_the_ball_silently() {
    let t = Tol::witness();
    let ball = finished("ball", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), t), t);
    let mut b = Body::clone(&ball);
    let key = sphere_faces(&b)[0];
    split_meridian(&mut b, key, v(0.0, 1.0, 0.0), false, t).unwrap();
    assert_eq!(sphere_faces(&b).len(), 3);
    let got = tier3("exterior meridian", &b, t).expect("tier 3 passes the doubled ball");
    assert!((got - 8.0 * PI / 3.0).abs() < 1e-9, "measures {got}");
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn dir(&mut self) -> Vec3<f64> {
        loop {
            let v = v(self.range(-1.0, 1.0), self.range(-1.0, 1.0), self.range(-1.0, 1.0));
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
}

/// Every subset of sphere faces (up to 5) flipped; returns (regressions, silent).
fn sweep_body(label: &str, b: &Body<f64>, t: Tol, out: &mut Vec<String>) -> (usize, usize, usize) {
    if let Err(e) = topo::validate_geometric(b, t) {
        if e.iter().any(|e| matches!(e, ValidationError::CurvedSenseInverted { .. })) {
            out.push(format!("{label}: UNFLIPPED refused CurvedSenseInverted: {e:?}"));
            return (1, 0, 0);
        }
        return (0, 0, 0);
    }
    let truth = topo::mass_properties(b, t).map(|m| m.volume).unwrap_or(f64::NAN);
    let sph = sphere_faces(b);
    let n = sph.len().min(5);
    let mut silent = 0;
    let mut flips = 0;
    for mask in 1..(1u32 << n) {
        let set: Vec<FaceKey> = (0..n).filter(|i| mask & (1 << i) != 0).map(|i| sph[i]).collect();
        let fb = set.iter().fold(b.clone(), |b, &k| b.flipped_face_sense_for_tests(k).unwrap());
        flips += 1;
        if topo::validate_geometric(&fb, t).is_ok() {
            let got = topo::mass_properties(&fb, t).map(|m| m.volume);
            if got.as_ref().map_or(true, |g| (g - truth).abs() > 1e-9 * truth.abs().max(1.0)) {
                silent += 1;
                out.push(format!("{label}: flip {set:?} of {} sphere faces PASSES, {got:?} vs {truth}", sph.len()));
            }
        }
    }
    (0, silent, flips)
}

#[test]
fn review_4746_2_random_cuts() {
    let t = Tol::witness();
    let seed = std::env::var("R2_SEED").ok().map_or(0x9E37_79B9_7F4A_7C15, |s| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap());
    let n = std::env::var("R2_N").ok().map_or(30, |s| s.parse().unwrap());
    let mut g = Rng(seed);
    let mut out = Vec::new();
    let (mut reg, mut sil, mut fl, mut bodies) = (0, 0, 0, 0);
    for i in 0..n {
        let pole = g.dir();
        // Offsets: near the centre (near-great), mid, near the surface (near-pole caps when the pole is near x).
        let off = match i % 4 {
            0 => g.range(-1e-6, 1e-6),
            1 => g.range(-0.7, 0.7),
            2 => g.range(0.95, 0.999),
            _ => g.range(-0.3, 0.3),
        };
        // A pole on or within a hair of the cut plane x = off, every third body.
        let pole = if i % 3 == 0 {
            let s = (1.0 - off * off).max(0.0).sqrt();
            let a = g.range(0.0, 2.0 * PI);
            v(off + g.range(-1e-9, 1e-9), s * a.cos(), s * a.sin()).normalize()
        } else {
            pole
        };
        let ball = finished("ball", ball_poled(1.0, v(0.0, 0.0, 0.0), pole, t), t);
        let cut = finished("cut", brick((off, 3.0), (-2.0, 2.0), (-2.0, 2.0), t), t);
        let off2 = g.range(-0.9, -0.2);
        let cut2 = finished("cut2", brick((-3.0, off2), (-2.0, 2.0), (-2.0, 2.0), t), t);
        let w = g.range(0.05, 0.5);
        let slab = finished("slab", brick((-2.0, 2.0), (-w, w), (-2.0, 2.0), t), t);
        let mut cases: Vec<(String, Body<f64>)> = Vec::new();
        for (name, r) in [
            ("ball-cut", topo::boolean::subtract(&ball, &cut, t)),
            ("ball^cut", topo::boolean::intersect(&ball, &cut, t)),
            ("cut-ball", topo::boolean::subtract(&cut, &ball, t)),
            ("ball+cut", topo::boolean::union(&ball, &cut, t)),
            ("ball-slab", topo::boolean::subtract(&ball, &slab, t)),
            ("ball^slab", topo::boolean::intersect(&ball, &slab, t)),
        ] {
            if let Some(b) = r.ok().and_then(|r| r.body().map(|b| Body::clone(&b.body))) {
                cases.push((name.into(), b));
            }
        }
        if let Some(b1) = topo::boolean::subtract(&ball, &cut, t).ok().and_then(|r| r.body().map(|b| b.body.clone())) {
            if let Some(b2) = topo::boolean::subtract(&b1, &cut2, t).ok().and_then(|r| r.body().map(|b| Body::clone(&b.body))) {
                cases.push(("ball-cut-cut2".into(), b2));
            }
        }
        for (name, b) in cases {
            bodies += 1;
            let (r, s, f) = sweep_body(&format!("#{i} off={off:.3e} {name}"), &b, t, &mut out);
            reg += r;
            sil += s;
            fl += f;
        }
    }
    for o in &out {
        eprintln!("  {o}");
    }
    eprintln!("RANDOM_CUTS bodies {bodies} flips {fl} regressions {reg} silent {sil}");
    assert_eq!(reg, 0);
}

/// The side-cut ball built by boolean: its two lunes meet along the
/// full meridian x < 0 (holding both poles, interior) and the cut circle
/// is the region's one boundary loop. Read at an interior-edge pole.
#[test]
fn review_4746_2_side_cut_boolean_flips() {
    let t = Tol::witness();
    let ball = finished("ball", ball_poled(1.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), t), t);
    let cut = finished("cut", brick((0.8, 3.0), (-2.0, 2.0), (-2.0, 2.0), t), t);
    let b = Body::clone(&topo::boolean::subtract(&ball, &cut, t).unwrap().body().unwrap().body);
    let mut out = Vec::new();
    let r = sweep_body("side cut", &b, t, &mut out);
    eprintln!("{out:#?} {r:?}");
    assert_eq!(r.0 + r.1, 0, "{out:#?}");
}

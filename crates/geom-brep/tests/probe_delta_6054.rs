//! Delta-review probes for PR 6054 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_4, TAU};
use geom::{Curve3, Surface};
use geom_brep::chart_singularity::{Lies, Located, Unread, locate, traversal_corners, FaceLoop};
use geom_brep::props::LoopEdge;
use geom_core::{Band, Point3, Vec3};

fn run(s: &Surface<f64>, loops: &[Vec<LoopEdge<f64>>], sense: bool, band: Band) -> Vec<Located<f64>> {
    let corners: Vec<Vec<Point3<f64>>> = loops.iter().map(|l| traversal_corners(l)).collect();
    let views: Vec<FaceLoop<'_, f64>> = loops
        .iter()
        .zip(&corners)
        .map(|(edges, corners)| FaceLoop { edges, corners })
        .collect();
    locate(s, &views, sense, band)
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}
fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v / v.norm()
}
const O: Point3<f64> = Point3 { x: 0.0, y: 0.0, z: 0.0 };

fn sphere() -> Surface<f64> {
    Surface::Sphere { center: O, radius: 1.0, axis: Vec3::new(0.0, 0.0, 1.0), u_ref: Vec3::new(1.0, 0.0, 0.0) }
}

/// Small-circle arc from a to b (unit), its plane tilted by beta about the chord.
fn small_arc(a: Vec3<f64>, b: Vec3<f64>, beta: f64, sampled: bool, s: u32, e: u32) -> LoopEdge<f64> {
    let g = unit(a.cross(b));
    let m = unit(b - a);
    let n = unit(g * beta.cos() + m.cross(g) * beta.sin());
    let d = n.dot(a);
    let c = O + n * d;
    let r = (1.0 - d * d).sqrt();
    let u = unit((O + a) - c);
    let wv = (O + b) - c;
    let mut t1 = wv.dot(n.cross(u)).atan2(wv.dot(u));
    if t1 <= 0.0 {
        t1 += TAU;
    }
    let carrier = if sampled {
        Curve3::Ellipse { center: c, axis: n, semi_u: r, semi_v: r, u_ref: u }
    } else {
        Curve3::Circle { center: c, axis: n, radius: r, u_ref: u }
    };
    LoopEdge::hand_built(carrier, 0.0, t1, true, s, e)
}

fn trav(e: &LoopEdge<f64>, k: usize, n: usize) -> f64 {
    let (a, b) = if e.forward { (e.t0, e.t1) } else { (e.t1, e.t0) };
    a + (b - a) * (k as f64) / (n as f64)
}

/// Winding of a loop about unit p, seen from outside at p, by adaptive sampling.
fn winding(edges: &[LoopEdge<f64>], p: Vec3<f64>) -> i64 {
    let e1 = unit(if p.z.abs() < 0.9 { Vec3::new(0.0, 0.0, 1.0) } else { Vec3::new(1.0, 0.0, 0.0) }.cross(p));
    let e2 = p.cross(e1);
    let ang = |q: Point3<f64>| {
        let w = q - O;
        w.dot(e2).atan2(w.dot(e1))
    };
    let mut total = 0.0;
    for e in edges {
        let mut stack = vec![];
        let n = 64;
        for k in (0..n).rev() {
            stack.push((trav(e, k, n), trav(e, k + 1, n), 0u32));
        }
        while let Some((a, b, depth)) = stack.pop() {
            let (pa, pb) = (e.carrier.eval(a), e.carrier.eval(b));
            let mut d = ang(pb) - ang(pa);
            while d > core::f64::consts::PI { d -= TAU; }
            while d < -core::f64::consts::PI { d += TAU; }
            if d.abs() > 0.2 && depth < 60 {
                let m = 0.5 * (a + b);
                stack.push((m, b, depth + 1));
                stack.push((a, m, depth + 1));
            } else {
                total += d;
            }
        }
    }
    // junction gaps are rounding-sized; ignore
    (total / TAU).round() as i64
}

fn reversed(edges: &[LoopEdge<f64>]) -> Vec<LoopEdge<f64>> {
    let n = edges.len() as u32;
    edges
        .iter()
        .rev()
        .enumerate()
        .map(|(i, e)| {
            let mut f = e.clone();
            f.forward = !e.forward;
            f.start = i as u32;
            f.end = (i as u32 + 1) % n;
            f
        })
        .collect()
}

/// Dense gnomonic polyline about c; true if it self-intersects.
fn self_intersects(edges: &[LoopEdge<f64>], c: Vec3<f64>) -> bool {
    let e1 = unit(if c.z.abs() < 0.9 { Vec3::new(0.0, 0.0, 1.0) } else { Vec3::new(1.0, 0.0, 0.0) }.cross(c));
    let e2 = c.cross(e1);
    let mut pts = vec![];
    for e in edges {
        for k in 0..200 {
            let w = e.carrier.eval(trav(e, k, 200)) - O;
            let h = w.dot(c);
            if h <= 0.05 { return true; }
            pts.push((w.dot(e1) / h, w.dot(e2) / h));
        }
    }
    let n = pts.len();
    let orient = |a: (f64, f64), b: (f64, f64), c: (f64, f64)| (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
    for i in 0..n {
        for j in i + 2..n {
            if i == 0 && j == n - 1 { continue; }
            let (a, b, c2, d) = (pts[i], pts[(i + 1) % n], pts[j], pts[(j + 1) % n]);
            let o1 = orient(a, b, c2);
            let o2 = orient(a, b, d);
            let o3 = orient(c2, d, a);
            let o4 = orient(c2, d, b);
            if o1 * o2 < 0.0 && o3 * o4 < 0.0 { return true; }
        }
    }
    false
}

fn bands() -> [Band; 3] {
    [Band::new(1e-12, 1e-11).unwrap(), Band::new(1e-9, 1e-8).unwrap(), Band::new(1e-6, 1e-5).unwrap()]
}

#[derive(Default, Debug)]
struct Tally { right: usize, wrong: usize, straddle: usize, banded: usize, carrier: usize, lp: usize, boundary: usize }

fn judge(t: &mut Tally, bad: &mut Vec<String>, got: &[Located<f64>], left: &dyn Fn(Vec3<f64>) -> bool, sense: bool, label: &str) {
    for l in got {
        let p = l.point - O;
        match &l.lies {
            Ok(Lies::Interior) | Ok(Lies::Outside) => {
                let want = left(p) == sense;
                let interior = matches!(l.lies, Ok(Lies::Interior));
                if interior == want { t.right += 1 } else { t.wrong += 1; bad.push(format!("{label} {:?}: {:?} want interior={want}", l.singularity, l.lies)); }
            }
            Ok(_) | Err(Unread::Boundary { .. }) => t.boundary += 1,
            Err(Unread::Side { why, .. }) => match why {
                geom_brep::chart_singularity::SideUnread::Straddle => t.straddle += 1,
                _ => t.banded += 1,
            },
            Err(Unread::Carrier { .. }) => t.carrier += 1,
            Err(Unread::Loop { .. }) => t.lp += 1,
        }
    }
}

/// Star polygons of small-circle arcs, slivers included, near and far from a pole.
fn fuzz(sampled: bool, cases: usize, seed: u64) {
    let mut rng = Lcg(seed);
    let mut bad = vec![];
    let mut t = Tally::default();
    let mut skipped = 0;
    for case in 0..cases {
        let band = bands()[case % 3];
        let rho = 10f64.powf(-4.0 + 4.0 * rng.next()).min(1.0);
        let aspect = 10f64.powf(-6.0 * rng.next());
        // centre at distance s*rho*aspect.. from the north pole
        let near = rng.next();
        let dist = if near < 0.7 { rho * (aspect * 10f64.powf(2.0 * rng.next() - 1.0)).min(2.0) * rng.next() * 2.0 } else { (rng.next() * 2.0 - 1.0).acos() };
        let phi = rng.next() * TAU;
        let c = Vec3::new(dist.sin() * phi.cos(), dist.sin() * phi.sin(), dist.cos());
        let e1 = unit(if c.z.abs() < 0.9 { Vec3::new(0.0, 0.0, 1.0) } else { Vec3::new(1.0, 0.0, 0.0) }.cross(c));
        let e2 = c.cross(e1);
        let rot = rng.next() * TAU;
        let (f1, f2) = (e1 * rot.cos() + e2 * rot.sin(), e2 * rot.cos() - e1 * rot.sin());
        let k = 2 + (rng.next() * 6.0) as usize;
        let mut angs: Vec<f64> = (0..k).map(|_| rng.next() * TAU).collect();
        angs.sort_by(f64::total_cmp);
        let verts: Vec<Vec3<f64>> = angs
            .iter()
            .map(|&a| unit(c + (f1 * (a.cos() * rho) + f2 * (a.sin() * rho * aspect))))
            .collect();
        if verts.iter().enumerate().any(|(i, v)| (*v - verts[(i + 1) % k]).norm() < 1e-9) { skipped += 1; continue; }
        let bmax = if k == 2 { 0.3 } else { 0.3 * rng.next() };
        let edges: Vec<LoopEdge<f64>> = (0..k)
            .map(|i| {
                let (a, b) = (verts[i], verts[(i + 1) % k]);
                let mut beta = bmax * (rng.next() * 2.0 - 1.0);
                if k == 2 { beta = if i == 0 { bmax * (0.2 + rng.next()) } else { -bmax * (0.2 + rng.next()) }; }
                small_arc(a, b, beta, sampled, i as u32, ((i + 1) % k) as u32)
            })
            .collect();
        if self_intersects(&edges, c) { skipped += 1; continue; }
        // orientation: must wind +1 about c (ccw about c)
        let wc = winding(&edges, c);
        let edges = match wc { 1 => edges, -1 => reversed(&edges), _ => { skipped += 1; continue; } };
        for (lp, ccw) in [(edges.clone(), true), (reversed(&edges), false)] {
            let left = |p: Vec3<f64>| {
                let w = winding(&edges, p);
                if ccw { w == 1 } else { w != 1 }
            };
            for sense in [true, false] {
                let got = run(&sphere(), core::slice::from_ref(&lp), sense, band);
                judge(&mut t, &mut bad, &got, &left, sense, &format!("case {case} band {band:?} rho {rho:e} aspect {aspect:e} dist {dist:e} k {k} ccw {ccw} sense {sense}"));
            }
        }
    }
    eprintln!("sampled={sampled} {t:?} skipped {skipped}");
    assert!(bad.is_empty(), "{} wrong:\n{}", bad.len(), bad.iter().take(30).cloned().collect::<Vec<_>>().join("\n"));
}

#[test]
fn probe_sphere_small_arc_fuzz_closed_form() {
    fuzz(false, 6000, 0xD1);
}

#[test]
fn probe_sphere_small_arc_fuzz_walk() {
    fuzz(true, 600, 0xD2);
}

fn cone() -> Surface<f64> {
    Surface::Cone { apex: O, axis: Vec3::new(0.0, 0.0, 1.0), half_angle: FRAC_PI_4, u_ref: Vec3::new(1.0, 0.0, 0.0) }
}
fn parallel(z: f64) -> Curve3<f64> {
    Curve3::Circle { center: Point3::new(0.0, 0.0, z), axis: Vec3::new(0.0, 0.0, 1.0), radius: z.abs(), u_ref: Vec3::new(1.0, 0.0, 0.0) }
}
fn generator(u: f64) -> Curve3<f64> {
    Curve3::Line { origin: O, dir: Vec3::new(u.cos(), u.sin(), 1.0) }
}

/// Cone patches beside the apex: lower parallel at z0, upper at z0+w, u span `span`.
#[test]
fn probe_cone_patches_beside_the_apex() {
    let mut bad = vec![];
    let mut t = Tally::default();
    for band in bands() {
        for z0 in [1e-4, 1e-2, 1.0, -1.0, -1e-3] {
            for w in [1e-7, 1e-5, 1e-3, 1.0] {
                for span in [0.1, 3.0, TAU - 1e-3, TAU - 1e-6] {
                    let u0 = 0.7;
                    let u1 = u0 + span;
                    let (lo, hi) = if z0 > 0.0 { (z0, z0 + w) } else { (z0 - w, z0) };
                    // CCW in (u, slant-v) on the positive nappe; on the negative nappe slant v = -|p| ...
                    let ccw = vec![
                        LoopEdge::hand_built(parallel(lo), u0, u1, true, 0, 1),
                        LoopEdge::hand_built(generator(u1 + if z0 < 0.0 { core::f64::consts::PI } else { 0.0 }), lo, hi, true, 1, 2),
                        LoopEdge::hand_built(parallel(hi), u0, u1, false, 2, 3),
                        LoopEdge::hand_built(generator(u0 + if z0 < 0.0 { core::f64::consts::PI } else { 0.0 }), lo, hi, false, 3, 0),
                    ];
                    for (lp, fwd) in [(ccw.clone(), true), (reversed(&ccw), false)] {
                        for sense in [true, false] {
                            let got = run(&cone(), core::slice::from_ref(&lp), sense, band);
                            // Truth: apex is outside the patch. The patch is the face
                            // when its traversal is ccw in the chart and sense is true,
                            // with the nappe flipping the chart's handedness by the
                            // existing pinned row: positive nappe east-run holds the apex
                            // on its right.
                            // Patch == face iff (fwd == sense) on the positive nappe.
                            // On the negative nappe, generator param increasing z means
                            // moving toward the apex: u east, z up -> ccw about... pin by
                            // left-of-traversal reading: apex left of an east-run parallel
                            // on the negative nappe.
                            let patch_is_face = fwd == sense;
                            let want_interior = !patch_is_face;
                            let left = |_: Vec3<f64>| true; // unused
                            let _ = left;
                            for l in &got {
                                match &l.lies {
                                    Ok(Lies::Interior) | Ok(Lies::Outside) => {
                                        if matches!(l.lies, Ok(Lies::Interior)) == want_interior { t.right += 1 } else {
                                            t.wrong += 1;
                                            bad.push(format!("band {band:?} z0 {z0} w {w} span {span} fwd {fwd} sense {sense}: {:?}", l.lies));
                                        }
                                    }
                                    Err(Unread::Side { .. }) => t.straddle += 1,
                                    Err(Unread::Loop { .. }) => t.lp += 1,
                                    _ => t.boundary += 1,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("cone {t:?}");
    assert!(bad.is_empty(), "{} wrong:\n{}", bad.len(), bad.join("\n"));
}

/// A polygon with one vertex `delta` from the north pole, corner opening
/// `open` (reflex included), its other vertices far, small-circle arcs.
#[test]
fn probe_vertex_beside_the_pole() {
    let mut bad = vec![];
    let mut t = Tally::default();
    let mut rng = Lcg(0xB0);
    for band in bands() {
        let esc = band.escalate();
        for delta in [2.0 * esc, 5.0 * esc, 100.0 * esc, 1e-4, 1e-2] {
            for open in [1e-3_f64, 0.5, 2.0, 3.1, 3.2, 4.5, 6.2] {
                for dir in [0.0_f64, 1.0, 2.5, 4.0] {
                    for &beta in &[0.0, 0.2, -0.2] {
                        let n = Vec3::new(0.0, 0.0, 1.0);
                        let v0 = unit(n + Vec3::new(dir.cos(), dir.sin(), 0.0) * delta);
                        // two far vertices at angular distance 0.5 from v0, around v0 at angles a and a+open
                        let a = rng.next() * TAU;
                        let e1 = unit(Vec3::new(0.0, 0.0, 1.0).cross(v0).cross(v0) * -1.0 + Vec3::new(1e-300, 0.0, 0.0));
                        let e1 = if e1.x.is_finite() && e1.norm() > 0.5 { e1 } else { Vec3::new(1.0, 0.0, 0.0) };
                        let e1 = unit(e1 - v0 * e1.dot(v0));
                        let e2 = v0.cross(e1);
                        let far = |ang: f64| unit(v0 * 0.5f64.cos() + (e1 * ang.cos() + e2 * ang.sin()) * 0.5f64.sin());
                        let (v1, v2) = (far(a), far(a + open));
                        let mid = far(a + open * 0.5);
                        let verts = if open < 3.14 { vec![v0, v1, v2] } else { vec![v0, v1, mid, v2] };
                        let k = verts.len();
                        let edges: Vec<LoopEdge<f64>> = (0..k)
                            .map(|i| small_arc(verts[i], verts[(i + 1) % k], beta, false, i as u32, ((i + 1) % k) as u32))
                            .collect();
                        let c = unit(verts.iter().fold(Vec3::new(0.0, 0.0, 0.0), |s, v| s + *v));
                        if self_intersects(&edges, c) || winding(&edges, c).abs() != 1 { continue; }
                        for (lp, rev) in [(edges.clone(), false), (reversed(&edges), true)] {
                            let w_raw = |p: Vec3<f64>| winding(&edges, p);
                            // orientation of `edges`: sign of winding about the far centroid side
                            let orient = winding(&edges, c);
                            let left = |p: Vec3<f64>| {
                                let w = w_raw(p);
                                let inside = w != 0 && orient != 0 || (orient == 0 && w != 0);
                                // left of `edges` = inside if edges wind +1 about interior
                                let l = if w == 1 { true } else if w == -1 { false } else { orient == -1 };
                                let l = if (w_raw(-p)).abs() > 1 { panic!() } else { l };
                                let _ = inside;
                                if rev { !l } else { l }
                            };
                            for sense in [true, false] {
                                let got = run(&sphere(), core::slice::from_ref(&lp), sense, band);
                                judge(&mut t, &mut bad, &got, &left, sense, &format!("band {band:?} delta {delta:e} open {open} dir {dir} beta {beta} rev {rev} sense {sense}"));
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("vertex {t:?}");
    assert!(bad.is_empty(), "{} wrong:\n{}", bad.len(), bad.iter().take(30).cloned().collect::<Vec<_>>().join("\n"));
}

/// Lenses of two arcs between the same endpoints with a tip angle down to
/// rounding, the pole placed beside them past the band.
#[test]
fn probe_thin_lens_tips() {
    let mut bad = vec![];
    let mut t = Tally::default();
    for band in bands() {
        for tip in [1e-15, 1e-13, 1e-11, 1e-9, 1e-6, 1e-3] {
            for off in [1e-8_f64, 1e-5, 1e-2, 0.3] {
                for chord in [1e-3_f64, 0.5, 2.0] {
                    for dir in [0.0_f64, 1.3, 3.0] {
                        // endpoints symmetric about a point at angular distance off from the pole
                        let cpt = unit(Vec3::new(off.sin() * dir.cos(), off.sin() * dir.sin(), off.cos()));
                        let e1 = unit(Vec3::new(-dir.sin(), dir.cos(), 0.0));
                        let a = unit(cpt * (chord * 0.5).cos() + e1 * (chord * 0.5).sin());
                        let b = unit(cpt * (chord * 0.5).cos() - e1 * (chord * 0.5).sin());
                        for base in [0.0, 0.1] {
                            let edges = vec![
                                small_arc(a, b, base + tip, false, 0, 1),
                                small_arc(b, a, -(base), false, 1, 0),
                            ];
                            for (lp, rev) in [(edges.clone(), false), (reversed(&edges), true)] {
                                let left = |p: Vec3<f64>| {
                                    let w = winding(&edges, p);
                                    let l = w == 1;
                                    if rev { !l } else { l }
                                };
                                for sense in [true, false] {
                                    let got = run(&sphere(), core::slice::from_ref(&lp), sense, band);
                                    judge(&mut t, &mut bad, &got, &left, sense, &format!("band {band:?} tip {tip:e} off {off:e} chord {chord} dir {dir} base {base} rev {rev} sense {sense}"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("lens {t:?}");
    assert!(bad.is_empty(), "{} wrong:\n{}", bad.len(), bad.iter().take(40).cloned().collect::<Vec<_>>().join("\n"));
}

fn dense_area(surface: &Surface<f64>, edges: &[LoopEdge<f64>], band: Band, n: usize) -> f64 {
    let chart = geom::ChartInverse::of(surface).unwrap();
    let g = |v: f64| match surface {
        Surface::Sphere { radius, .. } => radius * radius * v.sin(),
        Surface::Cone { half_angle, .. } => half_angle.sin() * v.abs() * v * 0.5,
        _ => 0.0,
    };
    let mut uv = vec![];
    for e in edges {
        for k in 0..n {
            uv.push(chart.point(e.carrier.eval(trav(e, k, n)), band).uv);
        }
    }
    let mut area = 0.0;
    for i in 0..uv.len() {
        let (a, b) = (uv[i], uv[(i + 1) % uv.len()]);
        let mut d = b.x - a.x;
        let (s, c) = d.sin_cos();
        d = s.atan2(c);
        area += -(g(a.y) + g(b.y)) * 0.5 * d;
    }
    area
}

/// The walk's area against a dense trapezoid, its bound against the gap.
#[test]
fn probe_walk_area_bound_holds() {
    let mut rng = Lcg(0xA2);
    let mut worst: f64 = 0.0;
    let mut bad = vec![];
    let mut n_read = 0;
    for case in 0..400 {
        let band = bands()[case % 3];
        let rho = 10f64.powf(-3.0 + 3.0 * rng.next()).min(0.8);
        let aspect = 10f64.powf(-5.0 * rng.next());
        let dist = rho * (0.5 + 2.0 * rng.next());
        let phi = rng.next() * TAU;
        let c = Vec3::new(dist.sin() * phi.cos(), dist.sin() * phi.sin(), dist.cos());
        let e1 = unit(Vec3::new(0.0, 0.0, 1.0).cross(c));
        let e2 = c.cross(e1);
        let k = 3 + (rng.next() * 4.0) as usize;
        let mut angs: Vec<f64> = (0..k).map(|_| rng.next() * TAU).collect();
        angs.sort_by(f64::total_cmp);
        let verts: Vec<Vec3<f64>> = angs.iter().map(|&a| unit(c + (e1 * (a.cos() * rho) + e2 * (a.sin() * rho * aspect)))).collect();
        if verts.iter().enumerate().any(|(i, v)| (*v - verts[(i + 1) % k]).norm() < 1e-9) { continue; }
        let edges: Vec<LoopEdge<f64>> = (0..k).map(|i| small_arc(verts[i], verts[(i + 1) % k], 0.1 * (rng.next() * 2.0 - 1.0), true, i as u32, ((i + 1) % k) as u32)).collect();
        if self_intersects(&edges, c) || winding(&edges, c).abs() != 1 { continue; }
        let s = sphere();
        let chart = geom::ChartInverse::of(&s).unwrap();
        let pts = geom_brep::chart_singularity::singular_points(&chart);
        let Some(w) = geom_brep::chart_singularity::walk_loop(&chart, &pts, &edges, band) else { continue };
        if w.winding() != 0 { continue; }
        n_read += 1;
        let a1 = dense_area(&s, &edges, band, 20000);
        let a2 = dense_area(&s, &edges, band, 40000);
        let unc = (a1 - a2).abs() * 2.0 + 1e-15;
        let gap = (w.area() - a2).abs();
        let ratio = gap / w.error();
        worst = worst.max(ratio);
        if gap > w.error() + unc {
            bad.push(format!("case {case} rho {rho:e} aspect {aspect:e} dist {dist:e}: area {:e} err {:e} dense {a2:e} unc {unc:e}", w.area(), w.error()));
        }
    }
    eprintln!("walk bound: read {n_read}, worst gap/error {worst:e}");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

#[test]
fn probe_cone_walk_bound_and_exempts() {
    let s = cone();
    let chart = geom::ChartInverse::of(&s).unwrap();
    let pts = geom_brep::chart_singularity::singular_points(&chart);
    let mut worst: f64 = 0.0;
    for band in bands() {
        for z0 in [1e-4_f64, 1e-2, 1.0, -1.0, -1e-3] {
            for w in [1e-7, 1e-5, 1e-3, 1.0] {
                for span in [0.1, 3.0, TAU - 1e-3, TAU - 1e-6] {
                    let u0 = 0.7;
                    let u1 = u0 + span;
                    let (lo, hi) = if z0 > 0.0 { (z0, z0 + w) } else { (z0 - w, z0) };
                    let ccw = vec![
                        LoopEdge::hand_built(parallel(lo), u0, u1, true, 0, 1),
                        LoopEdge::hand_built(generator(u1 + if z0 < 0.0 { core::f64::consts::PI } else { 0.0 }), lo, hi, true, 1, 2),
                        LoopEdge::hand_built(parallel(hi), u0, u1, false, 2, 3),
                        LoopEdge::hand_built(generator(u0 + if z0 < 0.0 { core::f64::consts::PI } else { 0.0 }), lo, hi, false, 3, 0),
                    ];
                    match geom_brep::chart_singularity::walk_loop(&chart, &pts, &ccw, band) {
                        None => eprintln!("cone exempt: band {:e} z0 {z0} w {w} span {span}", band.zero()),
                        Some(wk) => {
                            let a = dense_area(&s, &ccw, band, 40000);
                            let a2 = dense_area(&s, &ccw, band, 80000);
                            let unc = (a - a2).abs() * 2.0;
                            let gap = (wk.area() - a2).abs();
                            if gap > wk.error() { eprintln!("RATIO {:e} band {:e} z0 {z0} w {w} span {span} area {:e} err {:e} dense {a2:e} unc {unc:e} wind {}", gap / wk.error(), band.zero(), wk.area(), wk.error(), wk.winding()); }
                            worst = worst.max(gap / wk.error());
                            if gap > wk.error() * 1.0 + 1e-12 * a.abs() { eprintln!("CONE BAD band {:e} z0 {z0} w {w} span {span} area {:e} err {:e} dense {a:e}", band.zero(), wk.area(), wk.error()); }
                        }
                    }
                }
            }
        }
    }
    eprintln!("cone worst {worst:e}");
}

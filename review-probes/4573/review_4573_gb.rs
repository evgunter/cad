//! Reviewer fuzz probe (PR 4573): Gauss–Bonnet sphere faces against an
//! independent Stokes-integral area, with the side reader's answers
//! checked against a numeric pole-in-region test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face};
use geom_core::{Band, Point3, Tol, Vec3};

const TAU: f64 = core::f64::consts::TAU;
struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 {
        self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn r(&mut self, a: f64, b: f64) -> f64 { a + (b - a) * self.u() }
    fn dir(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1.0, 1.0), self.r(-1.0, 1.0), self.r(-1.0, 1.0));
            let n = v.norm();
            if n > 0.1 && n < 1.0 { return v / n; }
        }
    }
}
fn perp(a: Vec3<f64>) -> Vec3<f64> {
    let t = if a.x.abs() < 0.6 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let p = a.cross(t);
    p / p.norm()
}
fn rot(v: Vec3<f64>, k: Vec3<f64>, a: f64) -> Vec3<f64> {
    v * a.cos() + k.cross(v) * a.sin() + k * (k.dot(v) * (1.0 - a.cos()))
}

/// One arc on the unit sphere (about the origin) from A to B, CCW about n.
#[derive(Clone, Copy)]
struct Arc { n: Vec3<f64>, d: f64, a: Vec3<f64>, b: Vec3<f64>, full: bool }
impl Arc {
    fn rho(&self) -> f64 { (1.0 - self.d * self.d).sqrt() }
    fn span(&self) -> f64 {
        if self.full { return TAU; }
        let c = self.n * self.d;
        let u = (self.a - c) / self.rho();
        let v = self.n.cross(u);
        let w = self.b - c;
        let t = w.dot(v).atan2(w.dot(u));
        if t <= 0.0 { t + TAU } else { t }
    }
    fn at(&self, s: f64) -> (Vec3<f64>, Vec3<f64>) {
        let c = self.n * self.d;
        let u = (self.a - c) / self.rho();
        let v = self.n.cross(u);
        let t = s * self.span();
        let r = self.rho();
        (c + (u * t.cos() + v * t.sin()) * r, (v * t.cos() - u * t.sin()) * (r * self.span()))
    }
}

/// Build LoopEdges for the arcs on a sphere (centre c0, radius rr), with random
/// stored phases, random reversed carriers, and random splitting.
fn edges(arcs: &[Arc], c0: Vec3<f64>, rr: f64, rng: &mut Rng) -> Vec<LoopEdge<f64>> {
    let mut out = vec![];
    let mut vid = 0u32;
    let n_total = arcs.len();
    let _ = n_total;
    for arc in arcs {
        let pieces = if !arc.full && rng.u() < 0.3 { 2 } else { 1 };
        let span = arc.span();
        for k in 0..pieces {
            let s0 = k as f64 / pieces as f64;
            let s1 = (k + 1) as f64 / pieces as f64;
            let (pa, _) = arc.at(s0);
            let c = arc.n * arc.d;
            let rho = arc.rho();
            let ua = (pa - c) / rho;
            let phase = rng.r(-3.0, 3.0);
            let dt = span * (s1 - s0);
            let reversed = rng.u() < 0.4;
            let (axis, u_ref, t0, t1, fwd) = if !reversed {
                // pa at parameter `phase`
                let u = rot(ua, arc.n, -phase);
                (arc.n, u, phase, phase + dt, true)
            } else {
                // carrier about -n; traverse from t1 (=pa) down to t0
                let (pb, _) = arc.at(s1);
                let ub = (pb - c) / rho;
                let u = rot(ub, -arc.n, -phase);
                (-arc.n, u, phase, phase + dt, false)
            };
            let carrier = Curve3::Circle {
                center: Point3::origin() + c0 + c * rr,
                axis,
                radius: rho * rr,
                u_ref,
            };
            out.push(LoopEdge::hand_built(carrier, t0, t1, fwd, vid, vid + 1));
            vid += 1;
        }
    }
    let n = out.len() as u32;
    if let Some(l) = out.last_mut() { l.end = 0; }
    let _ = n;
    out
}

fn reverse(es: &[LoopEdge<f64>]) -> Vec<LoopEdge<f64>> {
    es.iter().rev().map(|e| LoopEdge::hand_built(e.carrier.clone(), e.t0, e.t1, !e.forward, e.end, e.start)).collect()
}

/// Independent area of the region left of the loop, by ∮(1 − x·Q) dφ_Q,
/// with Q = m (−m must lie outside the region).
fn stokes(arcs: &[Arc], m: Vec3<f64>) -> f64 {
    let e1 = perp(m);
    let e2 = m.cross(e1);
    let gl = [(-0.9061798459386640, 0.2369268850561891), (-0.5384693101056831, 0.4786286704993665), (0.0, 0.5688888888888889), (0.5384693101056831, 0.4786286704993665), (0.9061798459386640, 0.2369268850561891)];
    let mut sum = 0.0;
    for arc in arcs {
        let nsub = 400;
        for k in 0..nsub {
            let a = k as f64 / nsub as f64;
            let b = (k + 1) as f64 / nsub as f64;
            for (x, w) in gl {
                let s = 0.5 * (a + b) + 0.5 * (b - a) * x;
                let (p, dp) = arc.at(s);
                let (x1, x2) = (p.dot(e1), p.dot(e2));
                let dphi = (dp.dot(e2) * x1 - dp.dot(e1) * x2) / (x1 * x1 + x2 * x2);
                sum += 0.5 * (b - a) * w * (1.0 - p.dot(m)) * dphi;
            }
        }
    }
    sum
}

/// Gnomonic polygon of the loop about m (dense), and point-in-polygon.
fn poly(arcs: &[Arc], m: Vec3<f64>) -> Option<Vec<(f64, f64)>> {
    let e1 = perp(m);
    let e2 = m.cross(e1);
    let mut pts = vec![];
    for arc in arcs {
        for k in 0..300 {
            let (p, _) = arc.at(k as f64 / 300.0);
            let h = p.dot(m);
            if h < 0.05 { return None; }
            pts.push((p.dot(e1) / h, p.dot(e2) / h));
        }
    }
    Some(pts)
}
fn simple(pts: &[(f64, f64)]) -> bool {
    let n = pts.len();
    let seg = |i: usize| (pts[i], pts[(i + 1) % n]);
    let cr = |o: (f64, f64), a: (f64, f64), b: (f64, f64)| (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0);
    for i in 0..n {
        for j in i + 2..n {
            if i == 0 && j == n - 1 { continue; }
            let (p, q) = seg(i);
            let (r, s) = seg(j);
            if cr(p, q, r) * cr(p, q, s) < 0.0 && cr(r, s, p) * cr(r, s, q) < 0.0 { return false; }
        }
    }
    true
}
fn winding(pts: &[(f64, f64)], q: (f64, f64)) -> (i32, f64) {
    let mut w = 0.0;
    let mut dmin = f64::MAX;
    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[(i + 1) % pts.len()];
        let (ax, ay, bx, by) = (a.0 - q.0, a.1 - q.1, b.0 - q.0, b.1 - q.1);
        w += (ax * by - ay * bx).atan2(ax * bx + ay * by);
        dmin = dmin.min((ax * ax + ay * ay).sqrt());
    }
    ((w / TAU).round() as i32, dmin)
}

#[derive(Default, Debug)]
struct Tally { ok: u32, esc: u32, wrong: u32, skipped: u32, worst: f64, pole_in: u32, refused_other: u32 }

fn check(arcs: &[Arc], m: Vec3<f64>, axis: Vec3<f64>, c0: Vec3<f64>, rr: f64, rng: &mut Rng, t: &mut Tally, tag: &str) {
    let Some(pts) = poly(arcs, m) else { t.skipped += 1; return };
    if !simple(&pts) { t.skipped += 1; return; }
    let area = stokes(arcs, m);
    if !(area > 1e-6) { t.skipped += 1; return; }
    let e1 = perp(m);
    let e2 = m.cross(e1);
    let mut inside = 0;
    for p in [axis, -axis] {
        let h = p.dot(m);
        if h <= 0.05 { continue; }
        let (w, d) = winding(&pts, (p.dot(e1) / h, p.dot(e2) / h));
        if d < 1e-3 { t.skipped += 1; return; } // near-loop pole: skip
        if w != 0 { inside += 1; }
    }
    if inside > 0 { t.pole_in += 1; }
    let sphere = Surface::Sphere { center: Point3::origin() + c0, radius: rr, axis, u_ref: perp(axis) };
    let band = Band::linear(Tol::witness()).unwrap();
    let fwd = edges(arcs, c0, rr, rng);
    let rev = reverse(&fwd);
    let r2 = rr * rr;
    let a_in = area * r2;
    let a_out = (2.0 * TAU - area) * r2;
    // (loop, sense) -> expected: Some(area) or None (SenseContradicted)
    let cases: [(&[LoopEdge<f64>], bool, Option<f64>); 4] = if inside == 0 {
        [(&fwd, true, Some(a_in)), (&fwd, false, None), (&rev, false, Some(a_in)), (&rev, true, None)]
    } else {
        [(&fwd, true, Some(a_in)), (&fwd, false, Some(a_out)), (&rev, false, Some(a_in)), (&rev, true, Some(a_out))]
    };
    for (lp, sense, want) in cases {
        let got = curved_face(&sphere, lp, sense, band);
        match (got, want) {
            (Ok(fc), Some(a)) => {
                let e = (fc.area - a).abs() / a.max(r2);
                if e > 1e-9 {
                    t.wrong += 1;
                    println!("WRONG AREA {tag}: got {} want {a} (inside {inside}) sense {sense}", fc.area);
                } else { t.ok += 1; t.worst = t.worst.max(e); }
                // independent flux sign check: σR·Area + c·A ; just check radial sign
            }
            (Err(PropsError::SenseContradicted), None) => t.ok += 1,
            (Err(PropsError::Escalated { .. }), _) => t.esc += 1,
            (got, want) => {
                if matches!(got, Err(PropsError::SphereLoop { .. })) { t.refused_other += 1; }
                t.wrong += 1;
                println!("WRONG {tag}: got {:?} want {want:?} (inside {inside}) sense {sense} area {a_in}", got.map(|f| f.area));
            }
        }
    }
    // side reader directly
    let side = boundary_material_sign(&sphere, &fwd, band);
    let want_side = if inside == 0 { Ok(MaterialSign::Encoded(geom_core::Sign::Positive)) } else { Ok(MaterialSign::Unencoded) };
    if side != want_side && !matches!(side, Err(PropsError::Escalated { .. })) {
        t.wrong += 1;
        println!("WRONG SIDE {tag}: {side:?} want {want_side:?}");
    }
}

fn tri(rng: &mut Rng, m: Vec3<f64>, size: f64, bulge: f64) -> Vec<Arc> {
    let e1 = perp(m);
    let e2 = m.cross(e1);
    let k = 3 + (rng.u() * 3.0) as usize;
    let mut angs: Vec<f64> = (0..k).map(|_| rng.r(0.0, TAU)).collect();
    angs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let vs: Vec<Vec3<f64>> = angs.iter().map(|&a| {
        let s = rng.r(0.3, 1.0) * size;
        let dir = e1 * a.cos() + e2 * a.sin();
        m * s.cos() + dir * s.sin()
    }).collect();
    (0..k).map(|i| {
        let a = vs[i];
        let b = vs[(i + 1) % k];
        let g = a.cross(b);
        let g = g / g.norm();
        let w = (b - a) / (b - a).norm();
        let beta = rng.r(-bulge, bulge);
        let n = g * beta.cos() + w.cross(g) * beta.sin();
        Arc { n, d: n.dot(a), a, b, full: false }
    }).collect()
}

#[test]
fn gb_fuzz() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut t = Tally::default();
    for i in 0..3000 {
        let m = rng.dir();
        let size = rng.r(0.05, 1.3);
        let bulge = rng.r(0.0, 1.2);
        let arcs = tri(&mut rng, m, size, bulge);
        let axis = match i % 4 { 0 => m, 1 => rot(m, perp(m), rng.r(0.0, 0.3)), _ => rng.dir() };
        let c0 = Vec3::new(rng.r(-5.0, 5.0), rng.r(-5.0, 5.0), rng.r(-5.0, 5.0));
        let rr = 10f64.powf(rng.r(-2.0, 2.0));
        check(&arcs, m, axis, c0, rr, &mut rng, &mut t, &format!("tri {i}"));
    }
    println!("tri: {t:?}");
    // caps: one full circle, or the circle split at two points (arcs > π)
    let mut t = Tally::default();
    for i in 0..1500 {
        let m = rng.dir();
        let alpha = rng.r(0.02, 1.5);
        let n = m;
        let d = alpha.cos();
        let a = m * d + perp(m) * alpha.sin();
        let arcs = if i % 2 == 0 {
            vec![Arc { n, d, a, b: a, full: true }]
        } else {
            let f = rng.r(0.05, 0.95) * TAU;
            let b = rot(a, n, f);
            vec![Arc { n, d, a, b, full: false }, Arc { n, d, a: b, b: a, full: false }]
        };
        let axis = match i % 3 { 0 => rot(m, perp(m), rng.r(0.0, alpha * 0.9)), _ => rng.dir() };
        let c0 = Vec3::new(rng.r(-5.0, 5.0), rng.r(-5.0, 5.0), rng.r(-5.0, 5.0));
        let rr = 10f64.powf(rng.r(-2.0, 2.0));
        check(&arcs, m, axis, c0, rr, &mut rng, &mut t, &format!("cap {i}"));
    }
    println!("cap: {t:?}");
}

#[test]
fn near_pole_and_near_great() {
    println!("eps = {:e}", Tol::witness().get().eps);
    let mut rng = Rng(0xD1B54A32D192ED03);
    let band = Band::linear(Tol::witness()).unwrap();
    let mut stats = std::collections::BTreeMap::<String, u32>::new();
    for i in 0..4000 {
        let m = rng.dir();
        let size = rng.r(0.1, 1.2);
        let bulge = if i % 5 == 0 { 1e-12 } else { rng.r(0.0, 1.0) };
        let arcs = tri(&mut rng, m, size, bulge);
        let Some(pts) = poly(&arcs, m) else { continue };
        if !simple(&pts) { continue; }
        let area = stokes(&arcs, m);
        if !(area > 1e-6) { continue; }
        let j = (rng.u() * arcs.len() as f64) as usize;
        let s = if i % 3 == 0 { rng.r(1e-9, 1e-3) } else { rng.r(0.0, 1.0) };
        let (p, dp) = arcs[j].at(s);
        let left = p.cross(dp);
        let left = left / left.norm();
        let lg = -(rng.r(2.0, 14.0));
        let delta = if i % 7 == 0 { 0.0 } else { 10f64.powf(lg) * if rng.u() < 0.5 { 1.0 } else { -1.0 } };
        let axis = p + left * delta;
        let axis = axis / axis.norm();
        let rr = 10f64.powf(rng.r(-1.0, 1.0));
        let c0 = Vec3::new(rng.r(-3.0, 3.0), rng.r(-3.0, 3.0), rng.r(-3.0, 3.0));
        let sphere = Surface::Sphere { center: Point3::origin() + c0, radius: rr, axis, u_ref: perp(axis) };
        let fwd = edges(&arcs, c0, rr, &mut rng);
        let side = boundary_material_sign(&sphere, &fwd, band);
        let a_in = area * rr * rr;
        let inside = delta > 0.0;
        let key = match (&side, inside) {
            (Ok(MaterialSign::Encoded(geom_core::Sign::Positive)), false) => "out:Encoded+ (right)".to_string(),
            (Ok(MaterialSign::Unencoded), true) => "in:Unencoded (right)".to_string(),
            (Ok(MaterialSign::Encoded(geom_core::Sign::Positive)), true) => format!("in:Encoded+ (tolerated? delta*R={:.0e})", (delta * rr).abs().max(1e-16).log10().round()),
            (Err(PropsError::Escalated { .. }), _) => format!("{}:Escalated", if inside {"in"} else {"out"}),
            (other, _) => {
                println!("WRONG i={i} inside={inside} delta={delta:e} R={rr} side={other:?}");
                format!("WRONG {other:?}")
            }
        };
        *stats.entry(key).or_default() += 1;
        // the measured area under sense true is always the region (bit right either way)
        if let Ok(fc) = curved_face(&sphere, &fwd, true, band) {
            if (fc.area - a_in).abs() > 1e-9 * a_in.max(rr * rr) {
                println!("WRONG AREA i={i} {} vs {a_in}", fc.area);
                *stats.entry("WRONG AREA".into()).or_default() += 1;
            }
        }
    }
    for (k, v) in &stats { println!("{v:6}  {k}"); }
    assert!(!stats.keys().any(|k| k.starts_with("WRONG")));
}

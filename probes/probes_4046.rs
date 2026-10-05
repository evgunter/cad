//! Reviewer probes for PR #4046 (lane reach-dual4046-r1). Independent
//! oracle: a CSG signed-depth function of the primitives, and an exact
//! per-line slice integral (breakpoints of every primitive on the line,
//! midpoint of every sub-interval classified) for volumes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};
use topo::{AtRestBody, BooleanOp, SolidContainment, point_in_solid};

// ---------------------------------------------------------------- oracle
#[derive(Clone)]
enum Csg {
    Ball(Vec3<f64>, f64),
    /// axis-aligned box in the BODY frame
    Boxx([f64; 3], [f64; 3]),
    U(Box<Csg>, Box<Csg>),
    I(Box<Csg>, Box<Csg>),
    D(Box<Csg>, Box<Csg>),
}
use Csg::*;
fn u(a: Csg, b: Csg) -> Csg { U(Box::new(a), Box::new(b)) }
fn i(a: Csg, b: Csg) -> Csg { I(Box::new(a), Box::new(b)) }
fn d(a: Csg, b: Csg) -> Csg { D(Box::new(a), Box::new(b)) }
fn csg(op: BooleanOp, a: Csg, b: Csg) -> Csg {
    match op { BooleanOp::Union => u(a, b), BooleanOp::Intersect => i(a, b), BooleanOp::Subtract => d(a, b) }
}

/// Positive inside (a Lipschitz-1 lower bound on distance for unions etc.).
fn depth(c: &Csg, q: Vec3<f64>) -> f64 {
    match c {
        Ball(cc, r) => r - (q - *cc).norm(),
        Boxx(lo, hi) => {
            let qa = [q.x, q.y, q.z];
            (0..3).map(|k| (qa[k] - lo[k]).min(hi[k] - qa[k])).fold(f64::INFINITY, f64::min)
        }
        U(a, b) => depth(a, q).max(depth(b, q)),
        I(a, b) => depth(a, q).min(depth(b, q)),
        D(a, b) => depth(a, q).min(-depth(b, q)),
    }
}

/// Parameters `t` where the line `o + t·e_x` crosses a primitive surface.
fn breaks(c: &Csg, y: f64, z: f64, out: &mut Vec<f64>) {
    match c {
        Ball(cc, r) => {
            let h = r * r - (y - cc.y).powi(2) - (z - cc.z).powi(2);
            if h > 0.0 { out.push(cc.x - h.sqrt()); out.push(cc.x + h.sqrt()); }
        }
        Boxx(lo, hi) => { out.push(lo[0]); out.push(hi[0]); }
        U(a, b) | I(a, b) | D(a, b) => { breaks(a, y, z, out); breaks(b, y, z, out); }
    }
}

/// Exact-per-line volume over the box `lo..hi` (body frame), midpoint in (y,z).
fn volume(c: &Csg, lo: [f64; 3], hi: [f64; 3], n: usize) -> f64 {
    let (hy, hz) = ((hi[1] - lo[1]) / n as f64, (hi[2] - lo[2]) / n as f64);
    let mut v = 0.0;
    let mut bs = Vec::new();
    for jy in 0..n {
        let y = lo[1] + (jy as f64 + 0.5) * hy;
        for jz in 0..n {
            let z = lo[2] + (jz as f64 + 0.5) * hz;
            bs.clear();
            breaks(c, y, z, &mut bs);
            bs.push(lo[0]); bs.push(hi[0]);
            bs.retain(|t| *t >= lo[0] && *t <= hi[0]);
            bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for w in bs.windows(2) {
                if w[1] > w[0] && depth(c, Vec3::new(0.5 * (w[0] + w[1]), y, z)) > 0.0 {
                    v += w[1] - w[0];
                }
            }
        }
    }
    v * hy * hz
}

// ---------------------------------------------------------------- frame
/// The world pose of every body: rotation about `axis` by `angle` through the origin.
#[derive(Clone, Copy)]
struct Pose { axis: Vec3<f64>, angle: f64 }
impl Pose {
    fn id() -> Self { Pose { axis: Vec3::new(0.0, 0.0, 1.0), angle: 0.0 } }
    fn map(&self) -> Affine3<f64> {
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), self.axis / self.axis.norm(), self.angle)
    }
    fn inv(&self) -> Affine3<f64> {
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), self.axis / self.axis.norm(), -self.angle)
    }
    fn to_world(&self, q: Vec3<f64>) -> Point3<f64> { self.map().transform_point(Point3::new(q.x, q.y, q.z)) }
    fn to_body(&self, p: Point3<f64>) -> Vec3<f64> {
        let q = self.inv().transform_point(p);
        Vec3::new(q.x, q.y, q.z)
    }
}

fn tol() -> Tol { Tol::witness() }
fn eps() -> f64 { std::env::var("CAD_TOLERANCE_EPS").ok().and_then(|s| s.parse().ok()).unwrap_or(1e-9) }

fn posed(b: topo::Body<f64>, pose: Pose) -> AtRestBody<f64> {
    let b = if pose.angle == 0.0 { b } else { topo::transform_rigid(&b, &pose.map(), tol()).unwrap() };
    finished("posed", b, tol())
}
fn ball(r: f64, c: Vec3<f64>, pose: Pose) -> AtRestBody<f64> {
    let b = revolved_about_y(vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)], Revolution::Full, tol());
    let b = topo::transform_rigid(&b, &Affine3::translation(c), tol()).unwrap();
    posed(b, pose)
}
fn boxb(lo: [f64; 3], hi: [f64; 3], pose: Pose) -> AtRestBody<f64> {
    posed(brick((lo[0], hi[0]), (lo[1], hi[1]), (lo[2], hi[2]), tol()), pose)
}

fn run(op: BooleanOp, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Result<Option<AtRestBody<f64>>, String> {
    let r = match op {
        BooleanOp::Union => topo::boolean::union(a, b, tol()),
        BooleanOp::Intersect => topo::boolean::intersect(a, b, tol()),
        BooleanOp::Subtract => topo::boolean::subtract(a, b, tol()),
    };
    r.map(|r| r.body().map(|bb| bb.body.clone())).map_err(|e| format!("{e:?}"))
}

/// Tally of one probe family.
#[derive(Default, Debug)]
struct Tally { asked: usize, right: usize, refused: usize, on: usize, wrong: Vec<String>, near_opposite: usize, refusals: Vec<String> }

/// Classify `q` (body frame) on `body` against `c`; `clear` = depth beyond which a wrong answer is a defect.
fn check(t: &mut Tally, label: &str, body: &AtRestBody<f64>, c: &Csg, pose: Pose, q: Vec3<f64>, clear: f64) {
    let dq = depth(c, q);
    let p = pose.to_world(q);
    t.asked += 1;
    let band = Band::linear(tol()).unwrap();
    match point_in_solid(body, p, band, tol()) {
        Err(e) => {
            t.refused += 1;
            if dq.abs() > clear && t.refusals.len() < 6 { t.refusals.push(format!("{label} {q:?} depth {dq:.3e}: {e:?}")); }
        }
        Ok(SolidContainment::OnBoundary) => {
            t.on += 1;
            if dq.abs() > clear { t.wrong.push(format!("{label} {q:?} depth {dq:.3e}: OnBoundary")); }
        }
        Ok(got) => {
            let inside = got == SolidContainment::In;
            if (dq > 0.0) == inside { t.right += 1; }
            else if dq.abs() > clear { t.wrong.push(format!("{label} {q:?} depth {dq:.3e}: {got:?}")); }
            else { t.near_opposite += 1; }
        }
    }
}

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; (self.0 >> 11) as f64 / (1u64 << 53) as f64 }
    fn unit(&mut self) -> Vec3<f64> {
        loop { let v = Vec3::new(2.0*self.f()-1.0, 2.0*self.f()-1.0, 2.0*self.f()-1.0); let n = v.norm(); if n > 0.1 && n < 1.0 { return v / n; } }
    }
}

/// Points sampled near every pairwise sphere-sphere circle and sphere-plane circle of the
/// primitives, at offsets `deltas` in random directions, plus uniform points in the box.
fn sample_points(c: &Csg, lo: [f64; 3], hi: [f64; 3], scale: f64, rng: &mut Rng, n_uniform: usize, deltas: &[f64]) -> Vec<Vec3<f64>> {
    let mut prims = Vec::new();
    fn collect(c: &Csg, out: &mut Vec<Csg>) { match c { U(a,b)|I(a,b)|D(a,b) => { collect(a,out); collect(b,out); } x => out.push(x.clone()) } }
    collect(c, &mut prims);
    let mut pts = Vec::new();
    for _ in 0..n_uniform {
        pts.push(Vec3::new(lo[0] + (hi[0]-lo[0])*rng.f(), lo[1] + (hi[1]-lo[1])*rng.f(), lo[2] + (hi[2]-lo[2])*rng.f()));
    }
    // points on each sphere (random), near its poles in body y, and on pairwise circles
    let mut seeds = Vec::new();
    for p in &prims {
        if let Ball(cc, r) = p {
            for _ in 0..40 { seeds.push(*cc + rng.unit() * *r); }
            seeds.push(*cc + Vec3::new(0.0, *r, 0.0)); seeds.push(*cc - Vec3::new(0.0, *r, 0.0));
        }
    }
    for (a, pa) in prims.iter().enumerate() {
        for pb in prims.iter().skip(a + 1) {
            match (pa, pb) {
                (Ball(c1, r1), Ball(c2, r2)) => {
                    let dd = *c2 - *c1; let dn = dd.norm();
                    if dn >= r1 + r2 || dn <= (r1 - r2).abs() { continue; }
                    let x = (dn*dn + r1*r1 - r2*r2) / (2.0*dn);
                    let rho = (r1*r1 - x*x).sqrt(); let nhat = dd / dn;
                    let e1 = { let t = if nhat.x.abs() < 0.9 { Vec3::new(1.0,0.0,0.0) } else { Vec3::new(0.0,1.0,0.0) }; let e = t - nhat * nhat.dot(t); e / e.norm() };
                    let e2 = nhat.cross(e1);
                    for k in 0..24 { let th = k as f64 * std::f64::consts::TAU / 24.0 + 0.1; seeds.push(*c1 + nhat*x + (e1*th.cos() + e2*th.sin())*rho); }
                }
                (Ball(cc, r), Boxx(blo, bhi)) | (Boxx(blo, bhi), Ball(cc, r)) => {
                    // circles of the sphere with each face plane; and box corners/edges near sphere
                    for k in 0..3 { for &w in &[blo[k], bhi[k]] {
                        let ca = [cc.x, cc.y, cc.z]; let h = w - ca[k];
                        if h.abs() >= *r { continue; }
                        let rho = (r*r - h*h).sqrt();
                        for m in 0..24 { let th = m as f64 * std::f64::consts::TAU / 24.0 + 0.05;
                            let mut v = [0.0; 3]; v[k] = h; v[(k+1)%3] = rho*th.cos(); v[(k+2)%3] = rho*th.sin();
                            seeds.push(*cc + Vec3::new(v[0], v[1], v[2])); }
                    }}
                    // where box edges pierce the sphere (vertices of the face)
                    for k in 0..3 { for &a1 in &[blo[(k+1)%3], bhi[(k+1)%3]] { for &a2 in &[blo[(k+2)%3], bhi[(k+2)%3]] {
                        let ca = [cc.x, cc.y, cc.z];
                        let h = r*r - (a1-ca[(k+1)%3]).powi(2) - (a2-ca[(k+2)%3]).powi(2);
                        if h <= 0.0 { continue; }
                        for s in [-1.0, 1.0] { let mut v = [0.0;3]; v[k] = ca[k] + s*h.sqrt(); v[(k+1)%3] = a1; v[(k+2)%3] = a2; seeds.push(Vec3::new(v[0],v[1],v[2])); }
                    }}}
                }
                _ => {}
            }
        }
    }
    for s in seeds {
        for &dl in deltas { pts.push(s + rng.unit() * (dl * scale)); }
    }
    pts
}

fn report(name: &str, t: &Tally) {
    println!("PROBE {name}: asked {} right {} on {} refused {} near-opposite {} WRONG {}", t.asked, t.right, t.on, t.refused, t.near_opposite, t.wrong.len());
    for w in t.wrong.iter().take(8) { println!("   WRONG {w}"); }
    for w in &t.refusals { println!("   REFUSED(clear) {w}"); }
}

/// classify a body against its oracle with the standard sampling.
fn classify(name: &str, body: &AtRestBody<f64>, c: &Csg, pose: Pose, lo: [f64;3], hi: [f64;3], scale: f64, seed: u64) -> Tally {
    let mut rng = Rng(seed);
    let deltas = [1e-2, 1e-4, 1e-7, 1e-9, 1e-12, 3e-3];
    let clear = 20.0 * eps() * scale.max(1.0);
    let mut t = Tally::default();
    for q in sample_points(c, lo, hi, scale, &mut rng, 150, &deltas) {
        check(&mut t, name, body, c, pose, q, clear);
    }
    report(name, &t);
    t
}

/// Volume of a result against the slice oracle.
fn vol_check(name: &str, body: &AtRestBody<f64>, c: &Csg, lo: [f64;3], hi: [f64;3], log: &mut Vec<String>) {
    let v = topo::mass_properties(body, tol()).map(|m| m.volume);
    let o = volume(c, lo, hi, 900);
    let line = match v {
        Ok(v) => { let rel = (v - o).abs() / o.abs().max(1e-300); let flag = if rel > 2e-3 { "MISMATCH" } else { "ok" }; format!("VOL {name}: kernel {v:.9e} oracle {o:.9e} rel {rel:.1e} {flag}") }
        Err(e) => format!("VOL {name}: mass_properties refused {e:?} (oracle {o:.6e})"),
    };
    println!("{line}");
    log.push(line);
}

fn bbox(c: &Csg) -> ([f64;3],[f64;3]) {
    match c {
        Ball(cc, r) => ([cc.x-r, cc.y-r, cc.z-r], [cc.x+r, cc.y+r, cc.z+r]),
        Boxx(lo, hi) => (*lo, *hi),
        I(a,b) => { let (l1,h1) = bbox(a); let (l2,h2) = bbox(b);
            ([l1[0].max(l2[0]), l1[1].max(l2[1]), l1[2].max(l2[2])], [h1[0].min(h2[0]), h1[1].min(h2[1]), h1[2].min(h2[2])]) }
        D(a,_) => bbox(a),
        U(a,b) => { let (l1,h1) = bbox(a); let (l2,h2) = bbox(b);
            ([l1[0].min(l2[0]), l1[1].min(l2[1]), l1[2].min(l2[2])], [h1[0].max(h2[0]), h1[1].max(h2[1]), h1[2].max(h2[2])]) }
    }
}
fn pad(b: ([f64;3],[f64;3]), s: f64) -> ([f64;3],[f64;3]) { let ((l,h), p) = (b, 0.05*s); ([l[0]-p,l[1]-p,l[2]-p],[h[0]+p,h[1]+p,h[2]+p]) }

/// Build every op of (a, b) both orders; classify each result; check volume; return the results.
fn every_op(name: &str, a: &AtRestBody<f64>, ca: &Csg, b: &AtRestBody<f64>, cb: &Csg, pose: Pose, scale: f64, wrong: &mut Vec<String>, vols: &mut Vec<String>, refusals: &mut Vec<String>) -> Vec<(String, AtRestBody<f64>, Csg)> {
    let mut out = Vec::new();
    for (lab, op, x, y, cx, cy) in [
        ("A∪B", BooleanOp::Union, a, b, ca, cb), ("B∪A", BooleanOp::Union, b, a, cb, ca),
        ("A∩B", BooleanOp::Intersect, a, b, ca, cb), ("B∩A", BooleanOp::Intersect, b, a, cb, ca),
        ("A∖B", BooleanOp::Subtract, a, b, ca, cb), ("B∖A", BooleanOp::Subtract, b, a, cb, ca),
    ] {
        let full = format!("{name} {lab}");
        let c = csg(op, cx.clone(), cy.clone());
        match run(op, x, y) {
            Err(e) => { println!("REFUSED {full}: {e}"); refusals.push(format!("{full}: {e}")); }
            Ok(None) => {
                let (lo, hi) = pad(bbox(&c), scale);
                let o = volume(&c, lo, hi, 300);
                let line = format!("EMPTY {full}: oracle volume {o:.3e}");
                println!("{line}");
                if o > 1e-6 * scale.powi(3) { wrong.push(line); }
            }
            Ok(Some(r)) => {
                let (lo, hi) = pad(bbox(&c), scale);
                let t = classify(&full, &r, &c, pose, lo, hi, scale, 0x9e3779b97f4a7c15 ^ full.len() as u64);
                wrong.extend(t.wrong.iter().cloned());
                vol_check(&full, &r, &c, lo, hi, vols);
                out.push((full, r, c));
            }
        }
    }
    out
}

fn finish(wrong: &[String], vols: &[String]) {
    let mism: Vec<_> = vols.iter().filter(|l| l.contains("MISMATCH")).collect();
    println!("SUMMARY wrong {} vol-mismatch {}", wrong.len(), mism.len());
    assert!(wrong.is_empty() && mism.is_empty(), "wrong: {wrong:#?}\nvol: {mism:#?}");
}

fn poses() -> Vec<Pose> {
    vec![Pose::id(), Pose { axis: Vec3::new(0.3, 0.8, -0.5), angle: 0.7 }, Pose { axis: Vec3::new(1.0, 0.0, 0.2), angle: std::f64::consts::FRAC_PI_2 + 0.01 }]
}

// ---------------------------------------------------------------- probes

/// P1: lenses of other radii/separations/directions, scales, poses; every op both orders;
/// then each result reused against a nested / crossing / disjoint ball.
#[test]
fn p1_lens_family() {
    let (mut wrong, mut vols, mut refusals) = (Vec::new(), Vec::new(), Vec::new());
    let configs: &[(f64, f64, [f64; 3])] = &[
        (1.0, 1.0, [1.4, 0.0, 0.0]),
        (1.0, 0.6, [0.9, 0.5, 0.3]),
        (1.0, 0.3, [0.2, 0.95, 0.0]),   // small ball over A's pole
        (0.7, 1.0, [0.3, -1.2, 0.4]),
    ];
    for &scale in &[1.0, 1e-3, 1e3] {
        if scale > 1.0 && eps() < 1e-10 { println!("SKIP scale {scale}: at ε {} a plain revolved ball of this radius does not finish (1e-15 relative)", eps()); continue; }
        for (ci, &(r1, r2, dc)) in configs.iter().enumerate() {
            for (pi, &pose) in poses().iter().enumerate() {
                if scale != 1.0 && pi == 2 { continue; }
                let c1 = Vec3::new(0.1, 0.2, -0.1) * scale;
                let c2 = c1 + Vec3::new(dc[0], dc[1], dc[2]) * scale;
                let (a, b) = (ball(r1 * scale, c1, pose), ball(r2 * scale, c2, pose));
                let (ca, cb) = (Ball(c1, r1 * scale), Ball(c2, r2 * scale));
                let name = format!("lens c{ci} s{scale:e} p{pi}");
                let res = every_op(&name, &a, &ca, &b, &cb, pose, scale, &mut wrong, &mut vols, &mut refusals);
                if scale == 1.0 && pi < 2 {
                    // reuse: each result against a ball crossing the seam circle and one nested at the lens.
                    let mid = c1 + (c2 - c1) * 0.5;
                    for (rn, (rlab, rb, rc)) in res.iter().enumerate() {
                        if rn % 2 == 1 { continue; } // one order suffices for reuse
                        let s = Ball(mid + Vec3::new(0.0, 0.0, 0.3), 0.35);
                        let sb = if let Ball(c, r) = s { ball(r, c, pose) } else { unreachable!() };
                        every_op(&format!("{rlab} ×S"), rb, rc, &sb, &s, pose, scale, &mut wrong, &mut vols, &mut refusals);
                    }
                }
            }
        }
    }
    println!("REFUSALS {}: {refusals:#?}", refusals.len());
    finish(&wrong, &vols);
}

/// P2: three balls — collinear (middle face an annulus between two rings) and a
/// triangle cluster (faces with vertices where circles cross), posed, then reused.
#[test]
fn p2_three_balls() {
    let (mut wrong, mut vols, mut refusals) = (Vec::new(), Vec::new(), Vec::new());
    for (pi, &pose) in poses().iter().enumerate().take(2) {
        for (kind, cs) in [
            ("chain", [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.4, 0.1, 0.0), Vec3::new(2.8, 0.0, 0.2)]),
            ("triangle", [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.2, 0.2, 0.1), Vec3::new(0.5, 1.1, -0.2)]),
            ("triangle-flat", [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.3, 0.0, 0.0), Vec3::new(0.65, 0.0, 1.1)]),
        ] {
            let bs: Vec<_> = cs.iter().map(|c| ball(1.0, *c, pose)).collect();
            let os: Vec<_> = cs.iter().map(|c| Ball(*c, 1.0)).collect();
            let name = format!("{kind} p{pi}");
            match run(BooleanOp::Union, &bs[0], &bs[1]) {
                Ok(Some(ab)) => {
                    let cab = u(os[0].clone(), os[1].clone());
                    let res = every_op(&name, &ab, &cab, &bs[2], &os[2], pose, 1.0, &mut wrong, &mut vols, &mut refusals);
                    // reuse twice: each result against a small ball at the centroid and one crossing.
                    let cen = (cs[0] + cs[1] + cs[2]) / 3.0;
                    for (rlab, rb, rc) in res.iter().step_by(2) {
                        for (sl, s) in [("nest", Ball(cen, 0.15)), ("cross", Ball(cs[1] + Vec3::new(0.0, 0.9, 0.0), 0.3))] {
                            let sb = if let Ball(c, r) = &s { ball(*r, *c, pose) } else { unreachable!() };
                            every_op(&format!("{rlab} ×{sl}"), rb, rc, &sb, &s, pose, 1.0, &mut wrong, &mut vols, &mut refusals);
                        }
                    }
                }
                other => { println!("REFUSED {name} A∪B: {other:?}", other = other.err()); }
            }
        }
    }
    println!("REFUSALS {}: {refusals:#?}", refusals.len());
    finish(&wrong, &vols);
}

/// P3: ball against boxes leaving reflex vertices: a box corner inside the ball
/// (three planes meet inside), the pole-strut pose, a slab across a pole, posed.
#[test]
fn p3_ball_box_reflex() {
    let (mut wrong, mut vols, mut refusals) = (Vec::new(), Vec::new(), Vec::new());
    for (pi, &pose) in poses().iter().enumerate() {
        for (kind, lo, hi) in [
            ("corner-in", [0.3, 0.2, 0.1], [2.0, 2.0, 2.0]),
            ("strut", [-2.0, -2.0, -2.0], [0.25, 2.0, 0.0]),
            ("pole-slab", [-0.3, 0.6, -2.0], [0.2, 2.0, 2.0]),
            ("edge-through", [-0.2, -2.0, 0.4], [2.0, 2.0, 2.0]),
        ] {
            let a = ball(1.0, Vec3::new(0.0, 0.0, 0.0), pose);
            let b = boxb(lo, hi, pose);
            let name = format!("{kind} p{pi}");
            let res = every_op(&name, &a, &Ball(Vec3::new(0.0, 0.0, 0.0), 1.0), &b, &Boxx(lo, hi), pose, 1.0, &mut wrong, &mut vols, &mut refusals);
            if pi < 2 {
                for (rlab, rb, rc) in res.iter().step_by(2) {
                    let s = Ball(Vec3::new(0.35, 0.25, 0.15), 0.3);
                    let sb = if let Ball(c, r) = &s { ball(*r, *c, pose) } else { unreachable!() };
                    every_op(&format!("{rlab} ×S"), rb, rc, &sb, &s, pose, 1.0, &mut wrong, &mut vols, &mut refusals);
                }
            }
        }
    }
    println!("REFUSALS {}: {refusals:#?}", refusals.len());
    finish(&wrong, &vols);
}

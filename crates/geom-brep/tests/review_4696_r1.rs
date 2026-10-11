//! Review probes for PR 4696 (cone vector area). An analytic oracle on a
//! general frame (tilted axis, off-origin apex), independent of the PR's
//! Gauss–Legendre chart oracle and of `A⃗`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::cast_precision_loss, clippy::many_single_char_names)]

use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign_loops, curved_face_loops,
};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

#[derive(Clone, Copy, Debug)]
struct Cone {
    a: [f64; 3],
    k: [f64; 3],
    e1: [f64; 3],
    e2: [f64; 3],
    alpha: f64,
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}
fn add(a: [f64; 3], b: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]]
}
fn p<T: Real>(x: [f64; 3]) -> Point3<T> {
    Point3::new(T::from_f64(x[0]), T::from_f64(x[1]), T::from_f64(x[2]))
}
fn v<T: Real>(x: [f64; 3]) -> Vec3<T> {
    Vec3::new(T::from_f64(x[0]), T::from_f64(x[1]), T::from_f64(x[2]))
}

impl Cone {
    fn new(a: [f64; 3], k: [f64; 3], alpha: f64) -> Self {
        let k = unit(k);
        let t = if k[0].abs() < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] };
        let e1 = unit(add(t, k, -dot(t, k)));
        let e2 = cross(k, e1);
        Cone { a, k, e1, e2, alpha }
    }
    fn surface<T: Real>(&self) -> Surface<T> {
        Surface::Cone {
            apex: p(self.a),
            axis: v(self.k),
            half_angle: T::from_f64(self.alpha),
            u_ref: v(self.e1),
        }
    }
    fn rho(&self, u: f64) -> [f64; 3] {
        let (s, c) = u.sin_cos();
        [
            c * self.e1[0] + s * self.e2[0],
            c * self.e1[1] + s * self.e2[1],
            c * self.e1[2] + s * self.e2[2],
        ]
    }
    fn dir(&self, u: f64) -> [f64; 3] {
        let (s, c) = self.alpha.sin_cos();
        add(
            [s * self.rho(u)[0], s * self.rho(u)[1], s * self.rho(u)[2]],
            self.k,
            c,
        )
    }
    fn rim<T: Real>(&self, vv: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
        let (s, c) = self.alpha.sin_cos();
        let sg = vv.signum();
        topo::edge(
            Curve3::Circle {
                center: p(add(self.a, self.k, vv * c)),
                axis: v(self.k),
                radius: T::from_f64(vv.abs() * s),
                u_ref: v([sg * self.e1[0], sg * self.e1[1], sg * self.e1[2]]),
            },
            u0,
            u1,
            a,
            b,
        )
    }
    fn generator<T: Real>(&self, u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
        topo::edge(
            Curve3::Line {
                origin: p(self.a),
                dir: v(self.dir(u)),
            },
            t0,
            t1,
            a,
            b,
        )
    }
    /// Chart polygon (rectilinear in (u, v)) → loop.
    fn chart_loop<T: Real>(&self, verts: &[(f64, f64)]) -> Vec<LoopEdge<T>> {
        let n = verts.len();
        let id = |i: usize| {
            if verts[i % n].1 == 0.0 {
                n as u32
            } else {
                (i % n) as u32
            }
        };
        let mut edges = Vec::new();
        for i in 0..n {
            let ((ua, va), (ub, vb)) = (verts[i], verts[(i + 1) % n]);
            if va == vb && va == 0.0 {
                continue;
            }
            edges.push(if va == vb {
                self.rim(va, ua, ub, id(i), id(i + 1))
            } else {
                assert_eq!(ua, ub);
                self.generator(ua, va, vb, id(i), id(i + 1))
            });
        }
        edges
    }
    /// Analytic (flux, area) of the chart rectangle, chart-ccw orientation.
    fn rect(&self, (u0, u1): (f64, f64), (v0, v1): (f64, f64)) -> (f64, f64) {
        let (s, c) = self.alpha.sin_cos();
        let area = s * (u1 - u0) * (v1 * v1.abs() - v0 * v0.abs()) / 2.0;
        let (ae1, ae2, ak) = (dot(self.a, self.e1), dot(self.a, self.e2), dot(self.a, self.k));
        let ang = c * (ae1 * (u1.sin() - u0.sin()) - ae2 * (u1.cos() - u0.cos())) - s * ak * (u1 - u0);
        // p_u × p_v = v s (c ρ − s k) when k × e1 = e2.
        let flux = s * (v1 * v1 - v0 * v0) / 2.0 * ang;
        (flux, area)
    }
}

/// Signed chart area orientation of a polygon.
fn orient(verts: &[(f64, f64)]) -> f64 {
    let n = verts.len();
    (0..n)
        .map(|i| verts[i].0 * verts[(i + 1) % n].1 - verts[(i + 1) % n].0 * verts[i].1)
        .sum::<f64>()
        .signum()
}

fn measure<T: geom_core::Decide>(
    cone: &Cone,
    loops: &[Vec<LoopEdge<T>>],
    sense: bool,
) -> Result<(T, T), PropsError> {
    let l: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&cone.surface::<T>(), &l, sense, band()).map(|f| (f.flux, f.area))
}

struct Lcg(u64);
impl Lcg {
    fn f(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f()
    }
}

/// A random staircase face: columns over u in [u0, u0+span], each column
/// [v_lo, v_hi_i] (bottom flat at vb, tops stepped) — notched/stepped
/// walls; vb = 0 makes an apex sector-cap.
fn staircase(rng: &mut Lcg, vb: f64, span: f64) -> (Vec<(f64, f64)>, Vec<((f64, f64), (f64, f64))>) {
    let u0 = rng.r(-PI, PI);
    let ncol = 1 + (rng.f() * 4.0) as usize;
    let mut cuts = vec![0.0];
    for _ in 1..ncol {
        cuts.push(rng.r(0.05, 0.95));
    }
    cuts.push(1.0);
    cuts.sort_by(f64::total_cmp);
    let tops: Vec<f64> = (0..ncol).map(|_| vb + rng.r(0.3, 2.0)).collect();
    let mut verts = vec![(u0, vb), (u0 + span, vb)];
    let mut rects = vec![];
    for i in (0..ncol).rev() {
        let (ua, ub) = (u0 + span * cuts[i], u0 + span * cuts[i + 1]);
        verts.push((ub, tops[i]));
        verts.push((ua, tops[i]));
        rects.push(((ua, ub), (vb, tops[i])));
    }
    // dedupe consecutive vertical-collinear
    let mut out: Vec<(f64, f64)> = vec![];
    for q in verts {
        if out.last() != Some(&q) {
            out.push(q);
        }
    }
    (out, rects)
}

fn neg(verts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    verts.iter().map(|&(u, w)| (u, -w)).collect()
}
fn rev(verts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    verts.iter().rev().copied().collect()
}

fn random_cone(rng: &mut Lcg, alpha: f64) -> Cone {
    Cone::new(
        [rng.r(-3.0, 3.0), rng.r(-3.0, 3.0), rng.r(-3.0, 3.0)],
        [rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0)],
        alpha,
    )
}

/// Claim 1, f64 and Interval: random stepped walls, holed walls and apex
/// sectors on random tilted cones, both nappes, both traversals.
#[test]
fn fuzz_vector_area_against_analytic_oracle() {
    let mut rng = Lcg(0x4696);
    let mut worst = 0.0_f64;
    let mut iv_worst_width = 0.0_f64;
    let mut cases = 0;
    let mut refused = 0;
    for &alpha in &[1e-3, 0.05, 0.5, 1.2, PI / 2.0 - 1e-3] {
        for trial in 0..60 {
            let cone = random_cone(&mut rng, alpha);
            let vb = if trial % 3 == 0 { 0.0 } else { rng.r(0.2, 1.5) };
            let span = if trial % 5 == 0 { TAU } else { rng.r(0.3, 5.5) };
            if vb == 0.0 && span == TAU {
                continue; // full seam cap: generator pair coincide; covered elsewhere
            }
            let (outer, rects) = staircase(&mut rng, vb, span);
            // optional hole inside the first rect
            let with_hole = trial % 4 == 1 && vb > 0.0;
            let (ua, ub) = rects[0].0;
            let (va_, vb_) = rects[0].1;
            let hole = vec![
                (ua + 0.3 * (ub - ua), va_ + 0.2 * (vb_ - va_)),
                (ua + 0.6 * (ub - ua), va_ + 0.2 * (vb_ - va_)),
                (ua + 0.6 * (ub - ua), va_ + 0.5 * (vb_ - va_)),
                (ua + 0.3 * (ub - ua), va_ + 0.5 * (vb_ - va_)),
            ];
            for nappe in [1.0, -1.0] {
                let (o, h) = if nappe > 0.0 {
                    (outer.clone(), hole.clone())
                } else {
                    (neg(&outer), neg(&hole))
                };
                let (mut fl, mut ar) = (0.0, 0.0);
                for &((u0, u1), (v0, v1)) in &rects {
                    let (f, a) = if nappe > 0.0 {
                        cone.rect((u0, u1), (v0, v1))
                    } else {
                        cone.rect((u0, u1), (-v1, -v0))
                    };
                    fl += f;
                    ar += a;
                }
                if with_hole {
                    let hr = (
                        (h[0].0, h[1].0),
                        if nappe > 0.0 { (h[0].1, h[2].1) } else { (h[2].1, h[0].1) },
                    );
                    let (f, a) = cone.rect(hr.0, hr.1);
                    fl -= f;
                    ar -= a;
                }
                for fwd in [true, false] {
                    let oo = if fwd { o.clone() } else { rev(&o) };
                    let hh = if fwd ^ (orient(&o) == orient(&h)) { h.clone() } else { rev(&h) };
                    // hole wound against the outer
                    assert!(orient(&hh) != orient(&oo));
                    let want_flux = orient(&oo) * fl;
                    let mut polys = vec![oo.clone()];
                    if with_hole {
                        polys.push(hh.clone());
                    }
                    let loops: Vec<Vec<LoopEdge<f64>>> =
                        polys.iter().map(|q| cone.chart_loop(q)).collect();
                    let scale = ar.max(1.0) * (1.0 + dot(cone.a, cone.a).sqrt());
                    let (f, a) = match measure(&cone, &loops, true) {
                        Ok(x) => x,
                        Err(e) => {
                            eprintln!("F64 REFUSED alpha {alpha} trial {trial} nappe {nappe} fwd {fwd}: {e:?}\n{oo:?}");
                            refused += 1;
                            continue;
                        }
                    };
                    let err = ((f - want_flux).abs()).max((a - ar).abs()) / scale;
                    worst = worst.max(err);
                    assert!(
                        err < 1e-9,
                        "alpha {alpha} trial {trial} nappe {nappe} fwd {fwd}: got ({f}, {a}) want ({want_flux}, {ar})"
                    );
                    let l: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
                    let side = boundary_material_sign_loops(&cone.surface::<f64>(), &l, band());
                    let want_side = if orient(&oo) > 0.0 {
                        geom_core::Sign::Positive
                    } else {
                        geom_core::Sign::Negative
                    };
                    assert_eq!(side.ok(), Some(MaterialSign::Encoded(want_side)));
                    // Interval
                    let iloops: Vec<Vec<LoopEdge<Interval>>> =
                        polys.iter().map(|q| cone.chart_loop(q)).collect();
                    match measure(&cone, &iloops, true) {
                        Ok((fi, ai)) => {
                            let slack = 1e-12 * scale;
                            assert!(
                                fi.lo() - slack <= want_flux && want_flux <= fi.hi() + slack,
                                "interval flux [{}, {}] misses {want_flux}",
                                fi.lo(),
                                fi.hi()
                            );
                            assert!(
                                ai.lo() - slack <= ar && ar <= ai.hi() + slack,
                                "interval area [{}, {}] misses {ar}",
                                ai.lo(),
                                ai.hi()
                            );
                            iv_worst_width = iv_worst_width.max((ai.hi() - ai.lo()) / scale);
                        }
                        Err(e) => panic!(
                            "INTERVAL alpha {alpha} trial {trial} nappe {nappe} fwd {fwd}: refused {e:?} (f64 measured)"
                        ),
                    }
                    cases += 1;
                }
            }
        }
    }
    eprintln!("refused {refused} cases {cases} worst rel err {worst:e}, interval worst rel width {iv_worst_width:e}");
}

/// A full plane-section ellipse cap (tilted plane), both nappes, oracle
/// a 1-D periodic trapezoid over u of the closed-form v-integral.
#[test]
fn ellipse_cap_against_oracle() {
    let mut rng = Lcg(77);
    for &alpha in &[0.05, 0.5, 1.2] {
        for _ in 0..10 {
            let cone = random_cone(&mut rng, alpha);
            let (s, c) = alpha.sin_cos();
            let beta = rng.r(0.0, 0.9) * (PI / 2.0 - alpha);
            for h in [rng.r(0.5, 2.0), -rng.r(0.5, 2.0)] {
                let m = unit(add(cone.k, cone.e1, beta.tan()));
                let ve = |u: f64| h * beta.cos() / (s * beta.sin() * u.cos() + c * beta.cos());
                let pt = |u: f64| add(cone.a, cone.dir(u), ve(u));
                let (p0, ppi) = (pt(0.0), pt(PI));
                let cen = add(p0, add(ppi, p0, -1.0), 0.5);
                let d = add(p0, ppi, -1.0);
                let major = dot(d, d).sqrt() / 2.0;
                let mdir = unit(d);
                let ca = add(cen, cone.a, -1.0);
                let minor = ((dot(ca, cone.k) / c).powi(2) - dot(ca, ca)).sqrt();
                let ell = Curve3::Ellipse {
                    center: p::<f64>(cen),
                    axis: v(m),
                    major,
                    minor,
                    u_ref: v(mdir),
                };
                // check on cone
                for t in [0.3, 1.7, 4.0] {
                    let q = ell.eval(t);
                    let w = [q.x - cone.a[0], q.y - cone.a[1], q.z - cone.a[2]];
                    let ax = dot(w, cone.k);
                    let rad = (dot(w, w) - ax * ax).sqrt();
                    assert!((rad - ax.abs() * alpha.tan()).abs() < 1e-9, "not on cone");
                }
                // orientation: du/dt at t=0
                let q0 = ell.eval(0.0);
                let q1 = ell.eval(1e-4);
                let az = |q: Point3<f64>| {
                    let w = [q.x - cone.a[0], q.y - cone.a[1], q.z - cone.a[2]];
                    let sg = dot(w, cone.k).signum();
                    (sg * dot(w, cone.e2)).atan2(sg * dot(w, cone.e1))
                };
                let dudt = (az(q1) - az(q0)).signum();
                let n = 4000;
                let (ae1, ae2, ak) = (dot(cone.a, cone.e1), dot(cone.a, cone.e2), dot(cone.a, cone.k));
                let (mut fl, mut ar) = (0.0, 0.0);
                for i in 0..n {
                    let u = TAU * i as f64 / n as f64;
                    let w = TAU / n as f64;
                    let vv = ve(u);
                    ar += w * s * vv * vv / 2.0;
                    fl += w * vv * vv / 2.0 * s * (c * (ae1 * u.cos() + ae2 * u.sin()) - s * ak);
                }
                let want_flux = -dudt * fl;
                for fwd in [true, false] {
                    let e = topo::edge(ell.clone(), if fwd { 0.0 } else { TAU }, if fwd { TAU } else { 0.0 }, 0, 0);
                    let sgn = if fwd { 1.0 } else { -1.0 };
                    let (f, a) = measure(&cone, &[vec![e]], true).unwrap();
                    let scale = ar.max(1.0) * (1.0 + dot(cone.a, cone.a).sqrt());
                    assert!((a - ar).abs() < 1e-9 * scale, "area {a} vs {ar}");
                    assert!((f - sgn * want_flux).abs() < 1e-9 * scale, "flux {f} vs {}", sgn * want_flux);
                }
            }
        }
    }
}

/// Claim 3: a loop reaching the apex sweeps between 2π and 3π — a
/// spiral boundary that covers part of the nappe twice. Is it refused?
#[test]
fn apex_loop_sweeping_more_than_a_turn() {
    let cone = Cone::new([0.2, -0.1, -1.0], [0.0, 0.0, 1.0], 0.5);
    // apex → out at u=0 to v=1 → rim v=1 from 0 to 4 → up generator at u=4
    // to v=1.5 → rim v=1.5 from 4 to 2π+1 → back down to apex at u=1.
    let u_end = TAU + 1.0;
    let verts = [(0.0, 0.0), (0.0, 1.0), (4.0, 1.0), (4.0, 1.5), (u_end, 1.5), (u_end, 0.0)];
    let loops = vec![cone.chart_loop::<f64>(&verts)];
    let got = measure(&cone, &loops, true);
    let honest = cone.rect((0.0, 4.0), (0.0, 1.0)).1 + cone.rect((4.0, u_end), (0.0, 1.5)).1;
    let cap15 = cone.rect((0.0, TAU), (0.0, 1.5)).1;
    eprintln!("spiral: got {got:?}; chart-region area {honest} (cap at 1.5 = {cap15})");
    assert!(got.is_err(), "a loop sweeping 2π+1 about the axis measured: {got:?}");
}

/// The same with one rim: a sector of 2π + 1 (the rim retraces its
/// first radian) — a seam cap with a duplicated arc.
#[test]
fn apex_sector_of_more_than_a_turn() {
    let cone = Cone::new([0.2, -0.1, -1.0], [0.0, 0.0, 1.0], 0.5);
    let verts = [(0.0, 0.0), (0.0, 1.0), (TAU + 1.0, 1.0), (TAU + 1.0, 0.0)];
    let loops = vec![cone.chart_loop::<f64>(&verts)];
    let got = measure(&cone, &loops, true);
    eprintln!("sector 2π+1: {got:?} (cap {:?})", cone.rect((0.0, TAU), (0.0, 1.0)).1);
    assert!(got.is_err(), "a sector of 2π+1 measured: {got:?}");
}

/// Claim 3: a contractible loop stated twice (each joint closes).
#[test]
fn contractible_loop_stated_twice() {
    let cone = Cone::new([0.2, -0.1, -1.0], [0.0, 0.0, 1.0], 0.5);
    let q = [(0.0, 1.0), (2.0, 1.0), (2.0, 2.0), (0.0, 2.0)];
    let mut l = cone.chart_loop::<f64>(&q);
    l.extend(cone.chart_loop::<f64>(&q));
    let got = measure(&cone, &[l], true);
    eprintln!("twice: {got:?} once area {}", cone.rect((0.0, 2.0), (1.0, 2.0)).1);
    assert!(got.is_err(), "a rectangle stated twice measured: {got:?}");
}

/// A sector stated twice (through the apex twice), sweep 2·1.2 < 3π.
#[test]
fn apex_sector_stated_twice() {
    let cone = Cone::new([0.2, -0.1, -1.0], [0.0, 0.0, 1.0], 0.5);
    let q = [(0.0, 0.0), (1.2, 0.0), (1.2, 1.0), (0.0, 1.0)];
    let mut l = cone.chart_loop::<f64>(&q);
    l.extend(cone.chart_loop::<f64>(&q));
    let got = measure(&cone, &[l], true);
    eprintln!("sector twice: {got:?} once area {}", cone.rect((0.0, 1.2), (0.0, 1.0)).1);
    assert!(got.is_err(), "a sector stated twice measured: {got:?}");
}

/// Claim 4: two overlapping holes whose sum is under the region.
#[test]
fn overlapping_holes() {
    let cone = Cone::new([0.2, -0.1, -1.0], [0.0, 0.0, 1.0], 0.5);
    let o = [(0.0, 1.0), (3.0, 1.0), (3.0, 2.0), (0.0, 2.0)];
    let h1 = rev(&[(0.5, 1.2), (1.5, 1.2), (1.5, 1.8), (0.5, 1.8)]);
    let h2 = rev(&[(1.0, 1.2), (2.0, 1.2), (2.0, 1.8), (1.0, 1.8)]);
    let loops = vec![cone.chart_loop::<f64>(&o), cone.chart_loop(&h1), cone.chart_loop(&h2)];
    let got = measure(&cone, &loops, true);
    let true_area = cone.rect((0.0, 3.0), (1.0, 2.0)).1 - cone.rect((0.5, 2.0), (1.2, 1.8)).1;
    eprintln!("overlapping holes: {got:?} region area {true_area}");
    // and sum larger than the region
    let h3 = rev(&[(0.1, 1.05), (2.9, 1.05), (2.9, 1.95), (0.1, 1.95)]);
    let h4 = rev(&[(0.2, 1.1), (2.8, 1.1), (2.8, 1.9), (0.2, 1.9)]);
    let loops = vec![cone.chart_loop::<f64>(&o), cone.chart_loop(&h3), cone.chart_loop(&h4)];
    for sense in [true, false] {
        let got = measure(&cone, &loops, sense);
        eprintln!("holes summing past the region, sense {sense}: {got:?}");
        assert!(got.is_err(), "holes summing past the region measured: {got:?}");
    }
    // ring wound with the face
    let loops = vec![cone.chart_loop::<f64>(&o), cone.chart_loop(&rev(&h1))];
    let got = measure(&cone, &loops, true);
    eprintln!("ring with face: {got:?}");
    assert!(matches!(got, Err(PropsError::RingWoundWithFace)));
    // ring that winds the axis (band as outer + ring)
    let outer = vec![cone.rim::<f64>(2.0, 0.0, TAU, 0, 0)];
    let ring = vec![cone.rim::<f64>(1.0, TAU, 0.0, 1, 1)];
    let got = measure(&cone, &[outer, ring], true);
    eprintln!("winding ring: {got:?}");
    assert!(got.is_err());
}

/// Claim 2: boundaries within the band of the apex; generator through it.
#[test]
fn near_apex_nappe_rows() {
    let cone = Cone::new([0.2, -0.1, -1.0], [0.3, 0.1, 1.0], 0.5);
    let eps = crate::shared::tol::band().zero();
    for d in [0.0, 0.3 * eps, 3.0 * eps, 30.0 * eps, 1e3 * eps] {
        for sgn in [1.0, -1.0] {
            // sector whose near edge is a tiny rim at v = sgn*d (d=0 → apex)
            let q = [(0.0, sgn * d), (1.0, sgn * d), (1.0, sgn * 1.0), (0.0, sgn * 1.0)];
            let got = measure(&cone, &[cone.chart_loop::<f64>(&q)], true);
            let want = cone.rect((0.0, 1.0), if sgn > 0.0 { (d, 1.0) } else { (-1.0, -d) }).1;
            eprintln!("near apex d={d:e} sgn {sgn}: {got:?} want area {want}");
            // a sliver past the apex: v from −d to 1 (crossing)
            let q = [(0.0, -sgn * d), (0.0, sgn * 1.0), (1.0, sgn * 1.0), (1.0, -sgn * d)];
            let got = measure(&cone, &[cone.chart_loop::<f64>(&q)], true);
            eprintln!("  crossing by {d:e}: {got:?}");
            if d > 10.0 * eps {
                assert!(matches!(got, Err(PropsError::NappeSpanning)), "crossing by {d:e} measured {got:?}");
            }
            if d > 0.0 {
                // the in-band crossing must not measure silently either
                assert!(got.is_err() || d < eps, "crossing by {d:e} measured {got:?}");
            }
        }
    }
}

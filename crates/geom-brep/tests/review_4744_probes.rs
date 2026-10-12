//! Review probes for PR 4744 (FLUX cone tail): the five-span-point
//! ellipse incidence decide under tolerance.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::many_single_char_names
)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};
use geom_core::{Decide, Interval, Real};

const TAU: f64 = core::f64::consts::TAU;
const PI: f64 = core::f64::consts::PI;
type V = [f64; 3];
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn add(a: V, b: V, s: f64) -> V {
    [a[0] + s * b[0], a[1] + s * b[1], a[2] + s * b[2]]
}
fn unit(a: V) -> V {
    let n = dot(a, a).sqrt();
    [a[0] / n, a[1] / n, a[2] / n]
}

/// Ellipse parameters: center, normal, major, minor, u_ref.
#[derive(Clone, Copy, Debug)]
struct El {
    c: V,
    n: V,
    a: f64,
    b: f64,
    u: V,
}
impl El {
    fn eval(&self, t: f64) -> V {
        let v = cross(self.n, self.u);
        add(add(self.c, self.u, self.a * t.cos()), v, self.b * t.sin())
    }
    /// Perturbed by 8 params: dc(3), da, db, rotation w(3).
    fn perturb(&self, d: &[f64; 8]) -> El {
        let w = [d[5], d[6], d[7]];
        let n = unit(add(self.n, cross(w, self.n), 1.0));
        let u0 = add(self.u, cross(w, self.u), 1.0);
        let u = unit(add(u0, n, -dot(u0, n)));
        El {
            c: add(self.c, [d[0], d[1], d[2]], 1.0),
            n,
            a: self.a + d[3],
            b: self.b + d[4],
            u,
        }
    }
    fn curve<T: Real>(&self) -> Curve3<T> {
        Curve3::Ellipse {
            center: p3(self.c[0], self.c[1], self.c[2]),
            axis: v3(self.n[0], self.n[1], self.n[2]),
            major: T::from_f64(self.a),
            minor: T::from_f64(self.b),
            u_ref: v3(self.u[0], self.u[1], self.u[2]),
        }
    }
}

/// Cone: apex 0, axis z, half-angle alpha. Exact distance (double cone).
fn elev(alpha: f64, p: V) -> f64 {
    let (s, c) = alpha.sin_cos();
    (p[0] * p[0] + p[1] * p[1]).sqrt() * c - p[2].abs() * s
}

/// The section by a plane tilted `tau` toward y through (0,0,h0): the
/// fixture's closed form. Near vertex at t = 0, far vertex at t = π.
fn section(alpha: f64, tau: f64, h0: f64) -> El {
    let (s, co) = alpha.sin_cos();
    let a = [0.0, 0.0, 1.0];
    let e1 = [1.0, 0.0, 0.0];
    let e2 = [0.0, 1.0, 0.0];
    let normal = add(add([0.0; 3], a, tau.cos()), e2, tau.sin());
    let g = |u: f64| add(add(add([0.0; 3], e1, s * u.cos()), e2, s * u.sin()), a, co);
    let vertex = |u: f64| {
        let g = g(u);
        add([0.0; 3], g, h0 * tau.cos() / dot(normal, g))
    };
    let (p, q) = (
        vertex(-core::f64::consts::FRAC_PI_2),
        vertex(core::f64::consts::FRAC_PI_2),
    );
    let center = add(add([0.0; 3], p, 0.5), q, 0.5);
    let radius = center[2] * alpha.tan();
    let off = center[1];
    let half = add(p, q, -1.0);
    El {
        c: center,
        n: normal,
        a: 0.5 * dot(half, half).sqrt(),
        b: ((radius - off) * (radius + off)).sqrt(),
        u: unit(half),
    }
}

/// Solve the small dense system `m x = r` (Gaussian elimination, pivoting).
fn solve<const N: usize>(mut m: [[f64; N]; N], mut r: [f64; N]) -> [f64; N] {
    for i in 0..N {
        let p = (i..N)
            .max_by(|&x, &y| m[x][i].abs().total_cmp(&m[y][i].abs()))
            .unwrap();
        m.swap(i, p);
        r.swap(i, p);
        for k in i + 1..N {
            let f = m[k][i] / m[i][i];
            for j in i..N {
                m[k][j] -= f * m[i][j];
            }
            r[k] -= f * r[i];
        }
    }
    let mut x = [0.0; N];
    for i in (0..N).rev() {
        let mut s = r[i];
        for j in i + 1..N {
            s -= m[i][j] * x[j];
        }
        x[i] = s / m[i][i];
    }
    x
}

/// The gradient of the elevation at `t` with respect to the 8 params.
fn grad(alpha: f64, e: &El, t: f64) -> [f64; 8] {
    let h = 1e-6;
    let mut g = [0.0; 8];
    for k in 0..8 {
        let mut d = [0.0; 8];
        d[k] = h;
        let up = elev(alpha, e.perturb(&d).eval(t));
        d[k] = -h;
        let dn = elev(alpha, e.perturb(&d).eval(t));
        g[k] = (up - dn) / (2.0 * h);
    }
    g
}

/// The worst off-cone ellipse whose five span midpoints are each `level`
/// off the cone: the linearized maximum of the elevation over the span
/// subject to `|elev(t_i)| <= level`. Returns the perturbed ellipse.
fn worst(alpha: f64, e: &El, t0: f64, t1: f64, level: f64) -> (El, f64) {
    let nodes: [f64; 5] = core::array::from_fn(|i| t0 + (t1 - t0) * (i as f64 + 0.5) / 5.0);
    let jn: [[f64; 8]; 5] = core::array::from_fn(|i| grad(alpha, e, nodes[i]));
    let gram: [[f64; 5]; 5] = core::array::from_fn(|i| {
        core::array::from_fn(|j| (0..8).map(|k| jn[i][k] * jn[j][k]).sum())
    });
    let mut best = (0.0, [0.0; 5], 0.0);
    for i in 0..=2000 {
        let t = t0 + (t1 - t0) * f64::from(i) / 2000.0;
        let jt = grad(alpha, e, t);
        let rhs: [f64; 5] = core::array::from_fn(|i| (0..8).map(|k| jn[i][k] * jt[k]).sum());
        let c = solve(gram, rhs);
        let amp: f64 = c.iter().map(|x| x.abs()).sum();
        if amp > best.0 {
            best = (amp, c, t);
        }
    }
    let target: [f64; 5] = core::array::from_fn(|i| level * best.1[i].signum());
    let y = solve(gram, target);
    let d: [f64; 8] = core::array::from_fn(|k| (0..5).map(|i| jn[i][k] * y[i]).sum());
    (e.perturb(&d), best.2)
}

fn verdict<T: Decide>(e: &El, t0: f64, t1: f64) -> Result<(), PropsError> {
    verdict_at::<T>(ALPHA, e, t0, t1)
}
fn verdict_at<T: Decide>(alpha: f64, e: &El, t0: f64, t1: f64) -> Result<(), PropsError> {
    let edges: Vec<LoopEdge<T>> = vec![topo::edge(e.curve::<T>(), t0, t1, 0, 0)];
    let s: Surface<T> = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: T::from_f64(alpha),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    curved_face_loops(&s, &[&edges], true, band()).map(|_| ())
}
const ALPHA: f64 = 0.5;

fn max_elev(e: &El, t0: f64, t1: f64) -> f64 {
    (0..=20000)
        .map(|i| elev(ALPHA, e.eval(t0 + (t1 - t0) * f64::from(i) / 20000.0)).abs())
        .fold(0.0, f64::max)
}

/// **Falsifies claim 1.** A closed ellipse ring, off the cone by more
/// than K·ε (definitely off: it should refuse `OffSurface`), whose five
/// span midpoints are each within ε, is answered `Ok` — the five-point
/// argument does not bound the arc under tolerance once the elevation
/// varies with distance from the apex along a high-eccentricity section.
#[test]
fn an_off_cone_ellipse_ring_passes_the_five_point_decide() {
    let b = band();
    let mut report = Vec::new();
    let mut passed_off = 0;
    for ratio in [100.0, 1000.0] {
        // Far/near axial-height ratio: cos(tau-a)/cos(tau+a) = ratio.
        let (mut lo, mut hi) = (0.0, core::f64::consts::FRAC_PI_2 - ALPHA);
        for _ in 0..200 {
            let m = 0.5 * (lo + hi);
            if (m - ALPHA).cos() / (m + ALPHA).cos() > ratio {
                hi = m
            } else {
                lo = m
            }
        }
        let tau = 0.5 * (lo + hi);
        let e = section(ALPHA, tau, 1.0);
        report.push(format!("ratio {ratio}: {e:?}"));
        for (t0, t1) in [
            (0.0, TAU),
            (PI, PI + TAU),
            (PI + 0.3, PI + 0.3 + TAU),
            (PI - 0.3, PI - 0.3 + TAU),
            (0.5, 0.5 + TAU),
        ] {
            assert!(max_elev(&e, t0, t1) < 1e-11, "the section is on the cone");
            let (p, tstar) = worst(ALPHA, &e, t0, t1, 0.9 * b.zero());
            let nodes: Vec<f64> = (0..5)
                .map(|i| elev(ALPHA, p.eval(t0 + (t1 - t0) * (f64::from(i) + 0.5) / 5.0)))
                .collect();
            let m = max_elev(&p, t0, t1);
            let v64 = verdict::<f64>(&p, t0, t1);
            let vi = verdict::<Interval>(&p, t0, t1);
            if m > b.escalate() && v64.is_ok() {
                passed_off += 1;
            }
            report.push(format!(
            "span [{t0:.3},{t1:.3}] worst t* {tstar:.3}: nodes {:?} max elev {m:.3e} ({:.1} eps; escalate {:.1e}) f64 {v64:?} interval {vi:?}",
            nodes.iter().map(|x| format!("{x:.2e}")).collect::<Vec<_>>(), m / b.zero(), b.escalate()
        ));
        }
    }
    println!("{}", report.join("\n"));
    assert_eq!(
        passed_off,
        0,
        "definitely-off ellipse rings answered Ok:\n{}",
        report.join("\n")
    );
}

/// Filed row check (`fluxtail/planar-face-reads-arc-spans-undecided`):
/// a hand-built full circle stored over `[0, 2τ]` answers twice the disc.
#[test]
fn planar_face_counts_a_circle_past_a_period_twice() {
    let c = Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let one = geom_brep::props::planar_face(
        p3(0.0, 0.0, 0.0),
        &[vec![topo::edge(c.clone(), 0.0, TAU, 0, 0)]],
    )
    .unwrap();
    let two = geom_brep::props::planar_face(
        p3(0.0, 0.0, 0.0),
        &[vec![topo::edge(c, 0.0, 2.0 * TAU, 0, 0)]],
    )
    .unwrap();
    println!(
        "planar area one turn {:e}, two turns {:e}",
        one.area, two.area
    );
    assert!((two.area - 2.0 * one.area).abs() < 1e-12);
}

/// Filed row check (`flux/iso-parse-refuses-a-circle-within-band-...`):
/// a unit cylinder's rim tilted by 1e-5 is within ε of the cylinder
/// (`r·(1 − cos θ) = 5e-11`) and refuses `OffSurface` by
/// `props_rim_axis_parallel`.
#[test]
fn iso_parse_refuses_a_circle_within_band_of_the_cylinder() {
    let th: f64 = 1e-5;
    let c = Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(th.sin(), 0.0, th.cos()),
        radius: 1.0,
        u_ref: v3(th.cos(), 0.0, -th.sin()),
    };
    let dist = (0..=4096)
        .map(|i| {
            let p = c.eval(TAU * f64::from(i) / 4096.0);
            ((p.x * p.x + p.y * p.y).sqrt() - 1.0).abs()
        })
        .fold(0.0, f64::max);
    let s: Surface<f64> = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let edges = vec![topo::edge(c, 0.0, TAU, 0, 0)];
    let r = curved_face_loops(&s, &[&edges], true, band());
    println!("true distance {dist:e}; verdict {r:?}");
    assert!(dist < band().zero());
    assert_eq!(
        r.map(|_| ()),
        Err(PropsError::OffSurface {
            what: "props_rim_axis_parallel"
        })
    );
}

/// Claim 1, other half: genuinely on-cone sections (to f64 rounding)
/// across eccentricity, half-angle and span are never refused by the
/// ellipse incidence decide (other decides escalate at huge reach).
#[test]
fn on_cone_sections_answer() {
    let mut bad = Vec::new();
    for alpha in [1e-3, 0.5, core::f64::consts::FRAC_PI_2 - 1e-3] {
        for ratio in [1.5, 10.0, 1e3, 1e4] {
            let (mut lo, mut hi) = (0.0, core::f64::consts::FRAC_PI_2 - alpha);
            for _ in 0..200 {
                let m = 0.5 * (lo + hi);
                if (m - alpha).cos() / (m + alpha).cos() > ratio {
                    hi = m
                } else {
                    lo = m
                }
            }
            let tau = 0.5 * (lo + hi);
            for h0 in [1.0, 50.0] {
                let e = section(alpha, tau, h0);
                for (t0, t1) in [
                    (0.0, TAU),
                    (0.1, 0.1 + 1e-3),
                    (PI - 0.05, PI + 0.05),
                    (1.0, 1.0 + TAU),
                ] {
                    let m = (0..=4000)
                        .map(|i| elev(alpha, e.eval(t0 + (t1 - t0) * f64::from(i) / 4000.0)).abs())
                        .fold(0.0, f64::max);
                    let a = verdict_at::<f64>(alpha, &e, t0, t1);
                    let b = verdict_at::<Interval>(alpha, &e, t0, t1);
                    let incid = |r: &Result<(), PropsError>| {
                        format!("{r:?}").contains("props_cone_ellipse_on_cone")
                    };
                    if incid(&a) || incid(&b) {
                        bad.push(format!("alpha {alpha} ratio {ratio} h0 {h0} span [{t0},{t1}] stored-elev {m:e}: f64 {a:?} iv {b:?}"));
                    }
                }
            }
        }
    }
    println!("{}", bad.join("\n"));
    assert!(bad.is_empty(), "{} on-cone sections refused", bad.len());
}

/// How far the answer of the off-cone ring of the main probe (ratio
/// 1000, span [π, 3π]) moves from the true section's.
#[test]
fn the_passing_off_cone_ring_moves_the_answer() {
    let ratio = 1000.0;
    let (mut lo, mut hi) = (0.0, core::f64::consts::FRAC_PI_2 - ALPHA);
    for _ in 0..200 {
        let m = 0.5 * (lo + hi);
        if (m - ALPHA).cos() / (m + ALPHA).cos() > ratio {
            hi = m
        } else {
            lo = m
        }
    }
    let e = section(ALPHA, 0.5 * (lo + hi), 1.0);
    let (t0, t1) = (PI, PI + TAU);
    let (p, _) = worst(ALPHA, &e, t0, t1, 0.9 * band().zero());
    let s: Surface<f64> = Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: ALPHA,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let ans = |x: &El| {
        let ed = vec![topo::edge(x.curve::<f64>(), t0, t1, 0, 0)];
        curved_face_loops(&s, &[&ed], true, band()).unwrap()
    };
    let (a, b) = (ans(&e), ans(&p));
    println!(
        "true area {:e} flux {:e}; off-cone ring area {:e} flux {:e}; d_area {:e} d_flux {:e}; max elev {:e}; perimeter-ish 4a {:e}",
        a.area,
        a.flux,
        b.area,
        b.flux,
        b.area - a.area,
        b.flux - a.flux,
        max_elev(&p, t0, t1),
        4.0 * e.a
    );
}

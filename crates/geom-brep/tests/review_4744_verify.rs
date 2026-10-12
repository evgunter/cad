//! Verifier probes for PR 4744 @ b160b20c: the 128-piece ellipse bound.
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

const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;
const ALPHAS: [f64; 3] = [1e-3, 0.5, HALF_PI - 1e-3];

fn max_elev_at(alpha: f64, e: &El, t0: f64, t1: f64) -> f64 {
    let n = 40000;
    (0..=n)
        .map(|i| elev(alpha, e.eval(t0 + (t1 - t0) * f64::from(i) / f64::from(n))).abs())
        .fold(0.0, f64::max)
}
fn reach(e: &El, t0: f64, t1: f64) -> f64 {
    (0..=2000)
        .map(|i| dot(e.eval(t0 + (t1 - t0) * f64::from(i) / 2000.0), e.eval(t0 + (t1 - t0) * f64::from(i) / 2000.0)).sqrt())
        .fold(0.0, f64::max)
}
/// The tilt whose far vertex is `ratio` times as high as the near one.
fn tilt_for(alpha: f64, ratio: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, HALF_PI - alpha);
    for _ in 0..200 {
        let m = 0.5 * (lo + hi);
        if (m - alpha).cos() / (m + alpha).cos() > ratio {
            hi = m;
        } else {
            lo = m;
        }
    }
    0.5 * (lo + hi)
}
fn class(r: &Result<(), PropsError>) -> String {
    match r {
        Ok(()) => "ok".into(),
        Err(PropsError::OffSurface { what }) => format!("off:{what}"),
        Err(PropsError::Escalated { cause, .. }) => {
            let s = format!("{cause:?}");
            let n = s.find("props_").map(|i| s[i..].split(|c: char| !(c.is_alphanumeric() || c == '_')).next().unwrap().to_string());
            format!("esc:{}", n.unwrap_or(s))
        }
        Err(e) => {
            let s = format!("{e:?}");
            format!("other:{}", &s[..s.len().min(60)])
        }
    }
}
fn incidence_passed(c: &str) -> bool {
    !(c.contains("ellipse_on_cone") || c.contains("ellipse_nappe"))
}
const SPANS: [(f64, f64); 8] = [
    (0.0, TAU),
    (PI, PI + TAU),
    (-0.005, 0.005),
    (PI - 0.005, PI + 0.005),
    (-0.3, 0.3),
    (PI - 0.3, PI + 0.3),
    (0.005, TAU - 0.005),
    (PI + 0.005, PI + TAU - 0.005),
];

/// Off-cone ellipses at 1.05 and 2 escalation bands off: none may answer
/// incidence. Also records the largest true elevation among passes.
#[test]
fn verify_soundness_sweep() {
    let b = band();
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut rnd = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    };
    let mut bad = Vec::new();
    let (mut n, mut worst_pass, mut escal) = (0, 0.0f64, 0);
    for alpha in ALPHAS {
        for ratio in [1.5, 100.0, 1e3, 1e4] {
            let e = section(alpha, tilt_for(alpha, ratio), 1.0);
            for (t0, t1) in SPANS {
                let mut dirs: Vec<[f64; 8]> = (0..8).map(|k| core::array::from_fn(|j| if j == k { 1.0 } else { 0.0 })).collect();
                for _ in 0..4 {
                    dirs.push(core::array::from_fn(|_| rnd()));
                }
                // the reviewer's five-node construction, as a direction
                let (p, _) = worst(alpha, &e, t0, t1, 1e-9);
                let mut d = [0.0; 8];
                d[0] = p.c[0] - e.c[0];
                d[1] = p.c[1] - e.c[1];
                d[2] = p.c[2] - e.c[2];
                d[3] = p.a - e.a;
                d[4] = p.b - e.b;
                let rot = cross(e.n, p.n);
                d[5] = rot[0];
                d[6] = rot[1];
                d[7] = rot[2];
                dirs.push(d);
                let base = max_elev_at(alpha, &e, t0, t1);
                for dir in dirs {
                    let s0 = 1e-7 * reach(&e, t0, t1) / dir.iter().map(|x| x * x).sum::<f64>().sqrt();
                    let m0 = max_elev_at(alpha, &e.perturb(&dir.map(|x| x * s0)), t0, t1);
                    if m0 == 0.0 {
                        continue;
                    }
                    for target in [0.5 * b.zero(), 1.05 * b.escalate(), 2.0 * b.escalate()] {
                        let s = s0 * target / m0;
                        let p = e.perturb(&dir.map(|x| x * s));
                        let m = max_elev_at(alpha, &p, t0, t1);
                        for (lane, r) in [("f64", verdict_at::<f64>(alpha, &p, t0, t1)), ("int", verdict_at::<Interval>(alpha, &p, t0, t1))] {
                            n += 1;
                            let c = class(&r);
                            if c.contains("esc:props_cone_ellipse") {
                                escal += 1;
                            }
                            if incidence_passed(&c) {
                                worst_pass = worst_pass.max(m / b.zero());
                                if m > b.zero() {
                                    println!("OVERZERO a={alpha} ratio={ratio} span=[{t0},{t1}] true={:.3}zero base={:.2}zero reach={:.2e} {lane} {c}", m / b.zero(), base / b.zero(), reach(&e, t0, t1));
                                }
                                if m > b.escalate() {
                                    bad.push(format!("a={alpha} ratio={ratio} span=[{t0},{t1}] true={:.2}zero base={:.1e} {lane} {c}", m / b.zero(), base));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("SOUND n={n} incidence-escalations={escal} max true elevation among incidence passes = {worst_pass:.3} zero; violations {}", bad.len());
    for l in bad.iter().take(30) {
        println!("VIOL {l}");
    }
    assert!(bad.is_empty());
}

/// Genuine sections: which escalate or refuse, with reach.
#[test]
fn verify_on_cone_sweep() {
    let b = band();
    let mut refused = Vec::new();
    let mut esc = 0;
    let mut tot = 0;
    for alpha in ALPHAS {
        for ratio in [1.5, 10.0, 100.0, 1e3, 1e4] {
            for h0 in [1.0, 50.0] {
                let e = section(alpha, tilt_for(alpha, ratio), h0);
                for (name, t0, t1) in [("full", 0.0, TAU), ("near", -0.1, 0.1), ("far", PI - 0.1, PI + 0.1), ("centred", HALF_PI - 1.0, HALF_PI + 1.0), ("full@pi", PI, PI + TAU)] {
                    let err = max_elev_at(alpha, &e, t0, t1);
                    let r = reach(&e, t0, t1);
                    let c64 = class(&verdict_at::<f64>(alpha, &e, t0, t1));
                    let ci = class(&verdict_at::<Interval>(alpha, &e, t0, t1));
                    tot += 1;
                    let flag = if !incidence_passed(&c64) || !incidence_passed(&ci) { esc += 1; "**" } else { "  " };
                    println!("ONCONE {flag} a={alpha:.4} ratio={ratio:e} h0={h0} {name:8} reach={r:.2e} constr_err={:.2e}zero 64u*reach/zero={:.1} f64={c64} int={ci}", err / b.zero(), 64.0 * f64::EPSILON * r / b.zero());
                    if (c64.starts_with("off:props_cone_ellipse") || ci.starts_with("off:props_cone_ellipse")) && err < 0.1 * b.zero() {
                        refused.push(format!("a={alpha} ratio={ratio} h0={h0} {name} {c64} {ci}"));
                    }
                }
            }
        }
    }
    println!("ONCONE total={tot} not-passing-incidence={esc} refused={}", refused.len());
    assert!(refused.is_empty(), "{refused:#?}");
}

/// The Interval Ok/not-Ok boundary of off-cone ellipses, in units of the
/// true elevation over zero: the bound is sound iff every boundary ratio
/// is at most 1. Run at eps = 1e-6 (true elevation measured far above
/// f64 rounding).
#[test]
fn verify_threshold_ratio() {
    let b = band();
    let fine = |alpha: f64, e: &El, t0: f64, t1: f64| {
        let n = 400_000;
        (0..=n)
            .map(|i| elev(alpha, e.eval(t0 + (t1 - t0) * f64::from(i) / f64::from(n))).abs())
            .fold(0.0, f64::max)
    };
    let (mut lo_r, mut hi_r) = (f64::MAX, 0.0f64);
    let mut over = Vec::new();
    for alpha in ALPHAS {
        for ratio in [1.5, 10.0, 100.0] {
            let e = section(alpha, tilt_for(alpha, ratio), 1.0);
            for (t0, t1) in [(0.0, TAU), (PI, PI + TAU), (0.4, 1.9), (PI - 0.3, PI + 0.3), (0.003, TAU - 0.003)] {
                for k in 0..8 {
                    let dir: [f64; 8] = core::array::from_fn(|j| if j == k { 1.0 } else if j == (k + 3) % 8 { 0.37 } else { 0.0 });
                    let s0 = 1e-6;
                    let m0 = fine(alpha, &e.perturb(&dir.map(|x| x * s0)), t0, t1);
                    if m0 < 1e-12 {
                        continue;
                    }
                    let unit = s0 / m0; // scale per metre of true elevation
                    let ok = |s: f64| incidence_passed(&class(&verdict_at::<Interval>(alpha, &e.perturb(&dir.map(|x| x * s)), t0, t1)));
                    let (mut a, mut z) = (0.0, 4.0 * b.zero() * unit);
                    if !ok(0.0) || ok(z) {
                        continue;
                    }
                    for _ in 0..40 {
                        let m = 0.5 * (a + z);
                        if ok(m) { a = m } else { z = m }
                    }
                    let r = fine(alpha, &e.perturb(&dir.map(|x| x * a)), t0, t1) / b.zero();
                    lo_r = lo_r.min(r);
                    hi_r = hi_r.max(r);
                    if r > 1.0 {
                        over.push(format!("a={alpha} ratio={ratio} span=[{t0},{t1}] dir={k} boundary true={r:.6} zero"));
                    }
                }
            }
        }
    }
    println!("THRESH boundary true/zero in [{lo_r:.4}, {hi_r:.6}]; over 1: {}", over.len());
    for l in &over { println!("THRESH OVER {l}"); }
    assert!(over.is_empty());
}

//! **An independent volume and area for a body of whole NURBS patches
//! and planar caps** — the oracle a scene pins a quadrature face's
//! certified bracket against, where the closed form the scene knows
//! belongs to the continuum shape and not to the fitted body.
//!
//! It shares nothing with the kernel's quadrature or evaluation: it
//! reads each wall's knots, weights and control net, evaluates the
//! rational surface and its first partials by its own Cox–de Boor
//! recurrence (The NURBS Book, A2.3), and integrates by tensor
//! Gauss–Legendre per knot span, where the integrand is smooth.
//!
//! - Volume is the divergence form `V = ⅓ ∮ (x − c)·n dA` — on a wall
//!   `∫∫ (S − c)·(S_u × S_v) du dv`, on a planar cap `((o − c)·n)·A`
//!   with the cap's area `A` handed in by the caller. A closed shell's
//!   `∮ n dA` is zero, so the result does not depend on `c`, and
//!   [`measure`] reads it about two points and reports both: a wrong
//!   cap area, a missing face or a wall that is not whole shows as a
//!   disagreement.
//! - Area is `∫∫ |S_u × S_v|` over the walls plus the caps' areas.
//!
//! Every wall must be a whole patch (its face spans its surface's
//! domain), which a loft's or a sweep's lateral faces are.

use pncad::geom::{NurbsSurface, Surface};
use pncad::topo::Body;

/// Gauss–Legendre nodes and weights on `[-1, 1]`.
fn gauss(n: usize) -> Vec<(f64, f64)> {
    (0..n)
        .map(|i| {
            let mut x = (core::f64::consts::PI * (i as f64 + 0.75) / (n as f64 + 0.5)).cos();
            let mut dp = 0.0;
            for _ in 0..100 {
                let (mut p0, mut p1) = (1.0, x);
                for k in 2..=n {
                    let p2 = ((2 * k - 1) as f64 * x * p1 - (k - 1) as f64 * p0) / k as f64;
                    (p0, p1) = (p1, p2);
                }
                dp = n as f64 * (x * p1 - p0) / (x * x - 1.0);
                let dx = p1 / dp;
                x -= dx;
                if dx.abs() < 1e-16 {
                    break;
                }
            }
            (x, 2.0 / ((1.0 - x * x) * dp * dp))
        })
        .collect()
}

/// The `p + 1` basis functions alive on span `s` and their first
/// derivatives at `t` (The NURBS Book, A2.3 with `n = 1`).
fn basis(k: &[f64], p: usize, s: usize, t: f64) -> (Vec<f64>, Vec<f64>) {
    let mut ndu = vec![vec![0.0; p + 1]; p + 1];
    let (mut left, mut right) = (vec![0.0; p + 1], vec![0.0; p + 1]);
    ndu[0][0] = 1.0;
    for j in 1..=p {
        left[j] = t - k[s + 1 - j];
        right[j] = k[s + j] - t;
        let mut saved = 0.0;
        for r in 0..j {
            ndu[j][r] = right[r + 1] + left[j - r];
            let temp = ndu[r][j - 1] / ndu[j][r];
            ndu[r][j] = saved + right[r + 1] * temp;
            saved = left[j - r] * temp;
        }
        ndu[j][j] = saved;
    }
    let n = (0..=p).map(|j| ndu[j][p]).collect();
    let d = (0..=p)
        .map(|r| {
            if p == 0 {
                return 0.0;
            }
            let mut v = 0.0;
            if r >= 1 {
                v += ndu[r - 1][p - 1] / ndu[p][r - 1];
            }
            if r < p {
                v -= ndu[r][p - 1] / ndu[p][r];
            }
            v * p as f64
        })
        .collect();
    (n, d)
}

/// `(∫∫ (S − c)·(S_u × S_v), ∫∫ |S_u × S_v|)` over the whole of `net`
/// for each `c`, at `q` Gauss points a span a direction.
fn integrals(net: &NurbsSurface<f64>, q: usize, cs: &[[f64; 3]]) -> (Vec<f64>, f64) {
    let (ku, kv) = (net.knot_values_u().values(), net.knot_values_v().values());
    let (pu, pv) = (net.knot_values_u().degree(), net.knot_values_v().degree());
    let stride = net.control_counts().1;
    let (cp, w) = (net.control(), net.weights());
    let g = gauss(q);
    let spans = |k: &[f64], p: usize| -> Vec<usize> {
        (p..k.len() - p - 1).filter(|&s| k[s + 1] > k[s]).collect()
    };
    let (mut flux, mut area) = (vec![0.0; cs.len()], 0.0);
    for su in spans(ku, pu) {
        for sv in spans(kv, pv) {
            let (a, b) = (ku[su], ku[su + 1]);
            let (c, d) = (kv[sv], kv[sv + 1]);
            let jac = 0.25 * (b - a) * (d - c);
            for &(xu, wu) in &g {
                for &(xv, wv) in &g {
                    let u = 0.5 * (a + b) + 0.5 * (b - a) * xu;
                    let v = 0.5 * (c + d) + 0.5 * (d - c) * xv;
                    let (nu, du) = basis(ku, pu, su, u);
                    let (nv, dv) = basis(kv, pv, sv, v);
                    let (mut h, mut hu, mut hv) = ([0.0; 4], [0.0; 4], [0.0; 4]);
                    for i in 0..=pu {
                        for j in 0..=pv {
                            let idx = (su - pu + i) * stride + (sv - pv + j);
                            let (p, wt) = (cp[idx], w[idx]);
                            let p = [p.x, p.y, p.z];
                            let hw = [p[0] * wt, p[1] * wt, p[2] * wt, wt];
                            for k in 0..4 {
                                h[k] += nu[i] * nv[j] * hw[k];
                                hu[k] += du[i] * nv[j] * hw[k];
                                hv[k] += nu[i] * dv[j] * hw[k];
                            }
                        }
                    }
                    let s: [f64; 3] = core::array::from_fn(|k| h[k] / h[3]);
                    let s_u: [f64; 3] = core::array::from_fn(|k| (hu[k] - hu[3] * s[k]) / h[3]);
                    let s_v: [f64; 3] = core::array::from_fn(|k| (hv[k] - hv[3] * s[k]) / h[3]);
                    let n = [
                        s_u[1] * s_v[2] - s_u[2] * s_v[1],
                        s_u[2] * s_v[0] - s_u[0] * s_v[2],
                        s_u[0] * s_v[1] - s_u[1] * s_v[0],
                    ];
                    let wq = wu * wv * jac;
                    for (f, c) in flux.iter_mut().zip(cs) {
                        *f += wq
                            * ((s[0] - c[0]) * n[0] + (s[1] - c[1]) * n[1] + (s[2] - c[2]) * n[2]);
                    }
                    area += wq * (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
                }
            }
        }
    }
    (flux, area)
}

/// What [`measure`] read off a body.
#[derive(Debug)]
pub struct Measured {
    /// The volume about each reference point, in the order given.
    pub volume: Vec<f64>,
    pub area: f64,
}

/// `body`'s volume about each of `cs` and its area, at `q` Gauss points
/// a span a direction. Each planar face's area is `cap_area(origin,
/// normal)` of its plane; any surface but a NURBS wall or a plane is a
/// panic.
pub fn measure(
    body: &Body<f64>,
    q: usize,
    cs: &[[f64; 3]],
    cap_area: impl Fn([f64; 3], [f64; 3]) -> f64,
) -> Measured {
    let mut volume = vec![0.0; cs.len()];
    let mut area = 0.0;
    for (_, face) in body.faces() {
        let (_, surface) = body
            .surfaces()
            .find(|(k, _)| *k == face.surface)
            .expect("a face's surface is in its body");
        let sense = if face.sense { 1.0 } else { -1.0 };
        match surface {
            Surface::Nurbs(net) => {
                let (flux, a) = integrals(net, q, cs);
                for (v, f) in volume.iter_mut().zip(flux) {
                    *v += sense * f / 3.0;
                }
                area += a;
            }
            Surface::Plane { origin, normal, .. } => {
                let (o, n) = (
                    [origin.x, origin.y, origin.z],
                    [normal.x, normal.y, normal.z],
                );
                let a = cap_area(o, n);
                for (v, c) in volume.iter_mut().zip(cs) {
                    let lever = (o[0] - c[0]) * n[0] + (o[1] - c[1]) * n[1] + (o[2] - c[2]) * n[2];
                    *v += sense * a * lever / 3.0;
                }
                area += a;
            }
            other => panic!("the oracle reads NURBS walls and planar caps, not {other:?}"),
        }
    }
    Measured { volume, area }
}

//! CERT-N1 R2 probe fixtures: adversarial described NURBS (rational,
//! extreme weight ratios, degree >= 5, interior multiplicities up to
//! p - 1, closed-shaped nets, a 2-D curve) plus the parameter sets that
//! hit knot values and span boundaries. f64 only, so the dump probe
//! compiles at the red-first commit too.

#![allow(dead_code, clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3};

/// A deterministic pseudo-random stream (LCG), so the fixtures are
/// stable across runs and commits.
pub struct Lcg(pub u64);
impl Lcg {
    pub fn next_f64(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    pub fn coord(&mut self) -> f64 {
        self.next_f64() * 8.0 - 4.0
    }
}

fn kv(knots: &[f64], p: usize) -> KnotVector {
    KnotVector::clamped(knots.to_vec(), p).unwrap()
}

pub fn curves3() -> Vec<(&'static str, NurbsCurve3<f64>)> {
    let mut rng = Lcg(0x5eed_1234);
    let mut out = Vec::new();

    // (a) rational quadratic quarter circle, radius 2.
    out.push((
        "quarter_circle",
        NurbsCurve3::new(
            kv(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2),
            vec![
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(2.0, 2.0, 0.0),
                Point3::new(0.0, 2.0, 0.0),
            ],
            vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
        )
        .unwrap(),
    ));

    // (b) degree 5, interior multiplicities 1, 2, 3, 4 (= p - 1), weights
    // spanning 1e-6 .. 1e6.
    {
        let p = 5;
        let mut knots = vec![0.0; 6];
        knots.push(0.2);
        knots.extend([0.4; 2]);
        knots.extend([0.6; 3]);
        knots.extend([0.8; 4]);
        knots.extend([1.0; 6]);
        let n = knots.len() - p - 1;
        let control = (0..n)
            .map(|_| Point3::new(rng.coord(), rng.coord(), rng.coord()))
            .collect();
        let weights = (0..n)
            .map(|i| match i % 5 {
                0 => 1e-6,
                1 => 1e6,
                2 => 1.0,
                3 => 3e3,
                _ => 2e-4,
            })
            .collect();
        out.push(("deg5_mult_to_4_extreme_w", NurbsCurve3::new(kv(&knots, p), control, weights).unwrap()));
    }

    // (c) degree 7, one interior knot of multiplicity 6 (= p - 1), weight
    // ratio 1e8.
    {
        let p = 7;
        let mut knots = vec![0.0; 8];
        knots.extend([0.5; 6]);
        knots.extend([1.0; 8]);
        let n = knots.len() - p - 1;
        let control = (0..n)
            .map(|_| Point3::new(rng.coord(), rng.coord(), rng.coord()))
            .collect();
        let weights = (0..n).map(|i| if i % 2 == 0 { 1e-4 } else { 1e4 }).collect();
        out.push(("deg7_mult6_ratio_1e8", NurbsCurve3::new(kv(&knots, p), control, weights).unwrap()));
    }

    // (d) closed-shaped cubic: first control point == last.
    {
        let knots = [0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0];
        let first = Point3::new(1.0, 0.5, -0.25);
        let mut control: Vec<Point3<f64>> = (0..6)
            .map(|_| Point3::new(rng.coord(), rng.coord(), rng.coord()))
            .collect();
        control[0] = first;
        control.push(first);
        let weights = vec![1.0, 2.0, 0.5, 1.0, 3.0, 0.7, 1.0];
        out.push(("closed_cubic", NurbsCurve3::new(kv(&knots, 3), control, weights).unwrap()));
    }

    // (e) degree-1 polyline (every interior knot is a C0 corner).
    out.push((
        "polyline",
        NurbsCurve3::new(
            kv(&[0.0, 0.0, 0.3, 0.7, 1.0, 1.0], 1),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 2.0, 0.0),
                Point3::new(3.0, -1.0, 2.0),
                Point3::new(4.0, 0.0, 1.0),
            ],
            vec![1.0, 0.5, 4.0, 1.0],
        )
        .unwrap(),
    ));
    out
}

pub fn curves2() -> Vec<(&'static str, NurbsCurve2<f64>)> {
    let mut rng = Lcg(0xabcd_ef01);
    let p = 4;
    let mut knots = vec![0.0; 5];
    knots.push(0.3);
    knots.extend([0.7; 3]);
    knots.extend([1.0; 5]);
    let n = knots.len() - p - 1;
    let control = (0..n).map(|_| Point2::new(rng.coord(), rng.coord())).collect();
    let weights = (0..n).map(|i| if i % 3 == 0 { 1e-5 } else { 1e3 }).collect();
    vec![("deg4_2d_mult3", NurbsCurve2::new(kv(&knots, p), control, weights).unwrap())]
}

pub fn surfaces() -> Vec<(&'static str, NurbsSurface<f64>)> {
    let mut rng = Lcg(0x0fed_cba9);
    let mut out = Vec::new();
    // (a) degree (3, 2), u interior multiplicity 2 (= p - 1), extreme
    // weights.
    {
        let ku = kv(&[0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0], 3);
        let kvv = kv(&[0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2);
        let (nu, nv) = (6, 4);
        let control = (0..nu * nv)
            .map(|k| {
                let (i, j) = (k / nv, k % nv);
                Point3::new(i as f64 + 0.3 * rng.coord(), j as f64 + 0.3 * rng.coord(), rng.coord())
            })
            .collect();
        let weights = (0..nu * nv)
            .map(|k| match k % 4 {
                0 => 1e-5,
                1 => 1e5,
                2 => 1.0,
                _ => 7.0,
            })
            .collect();
        out.push(("deg32_mult2_extreme_w", NurbsSurface::new(ku, kvv, control, weights).unwrap()));
    }
    // (b) closed-shaped in u: first u-row == last u-row, degree (2, 1).
    {
        let ku = kv(&[0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2);
        let kvv = kv(&[0.0, 0.0, 1.0, 1.0], 1);
        let (nu, nv) = (4, 2);
        let mut control: Vec<Point3<f64>> = (0..nu * nv)
            .map(|_| Point3::new(rng.coord(), rng.coord(), rng.coord()))
            .collect();
        for j in 0..nv {
            control[(nu - 1) * nv + j] = control[j];
        }
        let weights = vec![1.0, 1.0, 0.25, 0.25, 3.0, 3.0, 1.0, 1.0];
        out.push(("closed_u_deg21", NurbsSurface::new(ku, kvv, control, weights).unwrap()));
    }
    out
}

/// Parameters for a curve: every knot value, every span midpoint and
/// quarter points, plus a few extras.
pub fn params(k: &KnotVector) -> Vec<f64> {
    let mut ps: Vec<f64> = k.knots().to_vec();
    let (lo, hi) = k.domain();
    for w in k.knots().windows(2) {
        if w[1] > w[0] {
            ps.push(0.5 * (w[0] + w[1]));
            ps.push(0.75 * w[0] + 0.25 * w[1]);
            ps.push(0.25 * w[0] + 0.75 * w[1]);
        }
    }
    ps.push(lo + 1e-9 * (hi - lo));
    ps.push(hi - 1e-9 * (hi - lo));
    ps.push(lo + 0.123456789 * (hi - lo));
    ps.sort_by(f64::total_cmp);
    ps.dedup();
    ps
}

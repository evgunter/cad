//! The test oracles the boolean's conic rows share — the conic doors'
//! rows, the subdivision's guard rows and the clearance rung's rows: a
//! random unit vector, the ellipse and torus fixtures, the TRUE signed
//! distance from a sphere, a wall or a torus, a function's crossings and
//! extremes along a parameter (each refined, so a pair of crossings
//! inside one sample cell is not missed), and a door's certified roots
//! matched against them.

use geom_core::{Point3, Vec3};
use test_utils::fuzz;

/// A uniformly drawn unit vector, away from the degenerate short ones.
pub(super) fn unit(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if v.norm() > 0.2 && v.norm() < 1.0 {
            return v.normalize();
        }
    }
}

/// The ring torus `(center, axis, big, small)`, its axis normalized and
/// its `u_ref` any unit vector square to it — the one torus fixture the
/// conic rows build.
pub(super) fn torus(
    center: Point3<f64>,
    axis: Vec3<f64>,
    big: f64,
    small: f64,
) -> geom::Surface<f64> {
    let axis = axis.normalize();
    geom::Surface::Torus {
        center,
        axis,
        major_radius: big,
        minor_radius: small,
        u_ref: axis.orthonormal_basis().0,
    }
}

/// The true signed distance from a sphere, a cylinder wall or a torus
/// of the point whose offset from a given point is `from(anchor)`, read
/// from the surface's own STORED anchor (its centre or origin). A
/// caller far from the origin hands an offset it computes as
/// `(a − anchor) + …` from stored values a few metres apart, which is
/// exact, rather than a point it rounded at the scale of its
/// coordinates.
pub(super) fn distance_from_anchor(
    s: &geom::Surface<f64>,
    from: impl Fn(Point3<f64>) -> Vec3<f64>,
) -> f64 {
    match *s {
        geom::Surface::Sphere { center, radius, .. } => from(center).norm() - radius,
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let q = from(origin);
            (q - axis * q.dot(axis)).norm() - radius
        }
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let q = from(center);
            let h = q.dot(axis);
            let rho = (q - axis * h).norm();
            (rho - major_radius).hypot(h) - minor_radius
        }
        _ => unreachable!("the oracle reads spheres, walls and tori"),
    }
}

/// The true signed distance of `p` from a sphere, a wall or a torus.
pub(super) fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    distance_from_anchor(s, |anchor| p - anchor)
}

/// How many times consecutive samples change sign.
pub(super) fn sign_changes(samples: &[f64]) -> usize {
    samples
        .windows(2)
        .filter(|w| (w[0] < 0.0) != (w[1] < 0.0))
        .count()
}

/// The ellipse `c + a·û cos θ + b·v̂ sin θ` on the unit normal `n`, its
/// `û` the part of `u` square to `n`.
pub(super) fn ellipse(c: [f64; 3], n: [f64; 3], u: [f64; 3], a: f64, b: f64) -> geom::Curve3<f64> {
    let n = Vec3::from_array(n).normalize();
    let u = Vec3::from_array(u);
    geom::Curve3::Ellipse {
        center: Point3::from_array(c),
        axis: n,
        major: a,
        minor: b,
        u_ref: (u - n * u.dot(n)).normalize(),
    }
}

/// The samples [`crossings`] and [`extremes`] take per call.
const SAMPLES: u32 = 20_000;

/// `f`'s extremum in `[a, b]` by golden section, a minimum when `low`.
fn golden(f: &impl Fn(f64) -> f64, (mut a, mut b): (f64, f64), low: bool) -> f64 {
    let g = (5f64.sqrt() - 1.0) / 2.0;
    for _ in 0..100 {
        let (x1, x2) = (b - g * (b - a), a + g * (b - a));
        if (f(x1) < f(x2)) == low {
            b = x2;
        } else {
            a = x1;
        }
    }
    (a + b) / 2.0
}

/// `f`'s root in `[a, b]`, whose ends `f` reads on opposite sides,
/// bisected to the bit.
fn bisect(f: &impl Fn(f64) -> f64, (mut a, mut b): (f64, f64)) -> f64 {
    for _ in 0..80 {
        let m = (a + b) / 2.0;
        if (f(m) < 0.0) == (f(a) < 0.0) {
            a = m;
        } else {
            b = m;
        }
    }
    (a + b) / 2.0
}

/// The sign changes of `f` on `[t0, t1]`, each bisected to the bit. A
/// pair inside one sample cell is found too: at every sampled local
/// extremum of `|f|`, the extremum is refined by golden section, and one
/// that crosses zero is bisected on both sides.
pub(super) fn crossings(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> Vec<f64> {
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(SAMPLES);
    let v: Vec<f64> = (0..=SAMPLES).map(|k| f(at(k))).collect();
    let mut out = Vec::new();
    for k in 0..SAMPLES {
        let (i, j) = (k as usize, k as usize + 1);
        if (v[i] < 0.0) != (v[j] < 0.0) {
            out.push(bisect(&f, (at(k), at(k + 1))));
        } else if k > 0
            && (v[i - 1] < 0.0) == (v[i] < 0.0)
            && v[i].abs() <= v[i - 1].abs()
            && v[i].abs() <= v[j].abs()
        {
            // `f` keeps its sign through both cells: its extremum near
            // `at(k)` may still cross zero and come back.
            let low = v[i] > 0.0;
            let x = golden(&f, (at(k - 1), at(k + 1)), low);
            if (f(x) < 0.0) != (v[i] < 0.0) {
                out.push(bisect(&f, (at(k - 1), x)));
                out.push(bisect(&f, (x, at(k + 1))));
            }
        }
    }
    out.sort_by(f64::total_cmp);
    out
}

/// `f`'s least and greatest values on `[t0, t1]`, every sampled local
/// extremum refined by golden section.
pub(super) fn extremes(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> (f64, f64) {
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(SAMPLES);
    let v: Vec<f64> = (0..=SAMPLES).map(|k| f(at(k))).collect();
    let (mut lo, mut hi) = (v[0].min(v[SAMPLES as usize]), v[0].max(v[SAMPLES as usize]));
    for k in 1..SAMPLES {
        let (p, q, s) = (v[k as usize - 1], v[k as usize], v[k as usize + 1]);
        if q <= p && q <= s {
            lo = lo.min(f(golden(&f, (at(k - 1), at(k + 1)), true)).min(q));
        }
        if q >= p && q >= s {
            hi = hi.max(f(golden(&f, (at(k - 1), at(k + 1)), false)).max(q));
        }
    }
    (lo, hi)
}

/// The crossings of the true distance from `s` along `e` on `[t0, t1]`.
pub(super) fn oracle(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> Vec<f64> {
    crossings(|t| distance(s, e.eval(t)), t0, t1)
}

/// `door`'s answer on `[t0, t0 + 1]` is certified roots matching the
/// oracle in number and place, every one on the surface and within `π`
/// of the arc's midpoint.
#[allow(clippy::panic)]
pub(super) fn assert_matches_oracle(
    label: &str,
    (e, s): (&geom::Curve3<f64>, &geom::Surface<f64>),
    t0: f64,
    want: usize,
    door: impl Fn(
        &geom::Curve3<f64>,
        &geom::Surface<f64>,
        f64,
        f64,
    ) -> super::circle_roots::CircleRoots<f64>,
) {
    use core::f64::consts::PI;
    let t1 = t0 + 1.0;
    let super::circle_roots::CircleRoots::Certified { count, thetas } = door(e, s, t0, t1) else {
        panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
    };
    assert_eq!(count, want, "{label}: the certified count");
    let mid = (t0 + t1) / 2.0;
    let mut got = thetas[..count].to_vec();
    for &t in &got {
        assert!((t - mid).abs() <= PI, "{label}: {t} within π of {mid}");
        let off = distance(s, e.eval(t)).abs();
        assert!(off < 1e-12, "{label}: root {t} lies {off} off the surface");
    }
    got.sort_by(f64::total_cmp);
    let truth = oracle(e, s, mid - PI, mid + PI);
    assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
    for (a, b) in got.iter().zip(&truth) {
        assert!(
            (a - b).abs() < 1e-9,
            "{label}: root {a} vs the oracle's {b}"
        );
    }
}

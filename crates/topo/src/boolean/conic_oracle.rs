//! The test oracles the boolean's conic rows share — the conic doors'
//! rows, the subdivision's guard rows and the clearance rung's rows: a
//! random unit vector, the ellipse and torus fixtures, the TRUE signed
//! distance from a sphere, a wall or a torus, a function's crossings and
//! extremes along a parameter (each refined, so a pair of crossings
//! inside one sample cell is not missed), and a door's certified roots
//! matched against them.

use geom_core::{Point3, Vec3};
use test_utils::fuzz;

pub(super) mod exact;

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

/// The ellipse `c + a·û cos θ + b·v̂ sin θ` on the unit normal `n`, its
/// `û` the part of `u` square to `n`.
pub(super) fn ellipse(c: [f64; 3], n: [f64; 3], u: [f64; 3], a: f64, b: f64) -> geom::Curve3<f64> {
    let n = Vec3::from_array(n).normalize();
    let u = Vec3::from_array(u);
    geom::Curve3::Ellipse {
        center: Point3::from_array(c),
        axis: n,
        semi_u: a,
        semi_v: b,
        u_ref: (u - n * u.dot(n)).normalize(),
    }
}

/// The samples [`crossings`] and [`extremes`] take per call.
const SAMPLES: u32 = 20_000;

/// `f`'s extremum in `[a, b]` by golden section, a minimum when `low`.
fn golden(f: &impl Fn(f64) -> f64, (mut a, mut b): (f64, f64), low: bool) -> f64 {
    let g = geom_core::GOLDEN_SECTION;
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

/// The samples about sample `k` of `v` (its one neighbour at either end)
/// when they keep its sign and `|v|` is least at `k`, a tie going to the
/// earlier sample so one extremum is searched once: the bracket in which
/// `f`'s extremum near there may cross zero and come back.
fn trough(v: &[f64], k: usize) -> Option<(usize, usize)> {
    let (a, b) = (k.saturating_sub(1), (k + 1).min(v.len() - 1));
    let kept = |j: usize| (v[j] < 0.0) == (v[k] < 0.0);
    let before = a == k || (kept(a) && v[k].abs() < v[a].abs());
    let after = b == k || (kept(b) && v[k].abs() <= v[b].abs());
    (before && after).then_some((a, b))
}

/// The sign changes of `f` on `[t0, t1]`, each bisected to the bit. A
/// pair inside one sample cell is found too, the first and last cells
/// included: at every sampled local extremum of `|f|`, the extremum is
/// refined by golden section, and one that crosses zero is bisected on
/// both sides.
pub(super) fn crossings(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> Vec<f64> {
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / f64::from(SAMPLES);
    let v: Vec<f64> = (0..=SAMPLES as usize).map(|k| f(at(k))).collect();
    let mut out: Vec<f64> = (0..SAMPLES as usize)
        .filter(|&k| (v[k] < 0.0) != (v[k + 1] < 0.0))
        .map(|k| bisect(&f, (at(k), at(k + 1))))
        .collect();
    for k in 0..v.len() {
        let Some((a, b)) = trough(&v, k) else {
            continue;
        };
        let x = golden(&f, (at(a), at(b)), v[k] > 0.0);
        if (f(x) < 0.0) != (v[k] < 0.0) {
            out.push(bisect(&f, (at(a), x)));
            out.push(bisect(&f, (x, at(b))));
        }
    }
    out.sort_by(f64::total_cmp);
    out
}

/// `f`'s least and greatest values on `[t0, t1]`, every sampled local
/// extremum refined by golden section, the first and last cells' included.
pub(super) fn extremes(f: impl Fn(f64) -> f64, t0: f64, t1: f64) -> (f64, f64) {
    let at = |k: usize| t0 + (t1 - t0) * k as f64 / f64::from(SAMPLES);
    let v: Vec<f64> = (0..=SAMPLES as usize).map(|k| f(at(k))).collect();
    let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
    for (k, &q) in v.iter().enumerate() {
        let (a, b) = (k.saturating_sub(1), (k + 1).min(v.len() - 1));
        lo = lo.min(q);
        hi = hi.max(q);
        if q <= v[a] && q <= v[b] {
            lo = lo.min(f(golden(&f, (at(a), at(b)), true)));
        }
        if q >= v[a] && q >= v[b] {
            hi = hi.max(f(golden(&f, (at(a), at(b)), false)));
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
    ) -> Result<super::circle_roots::CircleRoots<f64>, geom_brep::roots::Unsettled>,
) {
    use core::f64::consts::PI;
    let t1 = t0 + 1.0;
    let Ok(super::circle_roots::CircleRoots::Certified(found)) = door(e, s, t0, t1) else {
        panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
    };
    let (count, thetas) = (found.count(), found.roots());
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

/// **The first and last sample cells are searched like the rest.**
/// `cos(t − s) − cos w` crosses zero at `s ± w` and peaks at `s`; with
/// the pair a fifth of a cell wide and `s` inside the first or the last
/// cell, no sample sees the pair's sign, so only the refined extremum
/// finds it. On an arc and on a whole turn, at three places in each end
/// cell: both crossings, to the root's own rounding, and the peak `1`
/// to a few ulps, where an unrefined end cell reads it `1e-10` low.
#[test]
fn the_end_cells_are_searched() {
    use core::f64::consts::PI;
    for (t0, t1) in [(0.0, 1.0), (-PI, PI), (2.5, 2.5 + 2.0 * PI)] {
        let cell = (t1 - t0) / f64::from(SAMPLES);
        let w = 0.1 * cell;
        for frac in [0.3, 0.5, 0.7] {
            for s in [t0 + frac * cell, t1 - frac * cell] {
                let label = format!("[{t0}, {t1}], the pair about {s}");
                let got = crossings(|t| (t - s).cos() - w.cos(), t0, t1);
                assert_eq!(got.len(), 2, "{label}: {got:?}");
                for (g, want) in got.iter().zip([s - w, s + w]) {
                    assert!(
                        (g - want).abs() < 1e-9,
                        "{label}: a crossing {g} for {want}"
                    );
                }
                let (_, hi) = extremes(|t| (t - s).cos(), t0, t1);
                assert!(hi >= 1.0 - 1e-15, "{label}: the peak reads {hi}");
                let (lo, _) = extremes(|t| -(t - s).cos(), t0, t1);
                assert!(lo <= -1.0 + 1e-15, "{label}: the trough reads {lo}");
            }
        }
    }
}

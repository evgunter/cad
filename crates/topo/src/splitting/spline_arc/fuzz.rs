//! The counterexample-SEARCH half of the spline crossing row: splines
//! nobody chose — degree 1 to 4, polynomial and rational, a doubled
//! interior knot, a window ending on a knot — and points and rays at
//! random, against a dense polyline of the curve.

// Gated to the code it tests: the row, the driver, the restriction it
// reads its nets through, the carrier it evaluates, and the band and
// arithmetic it decides in.
test_utils::gated_to![
    "crates/topo/src/splitting/spline_arc.rs",
    "crates/topo/src/splitting/pieced_arc.rs",
    "crates/geom-core/src/bisect.rs",
    "crates/geom-core/src/spline/",
    "crates/geom-brep/src/pcurve_cache/projected.rs",
    "crates/geom/src/curves/nurbs.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/linalg/",
];

use super::*;
use crate::splitting::pieced_arc::{ArcHit, ArcRows};
use geom_core::{Interval, Tol, Vec3};
use test_utils::fuzz::Rng;

const ROWS: ArcRows = ArcRows {
    end: "fuzz_spline_end",
    clear: "fuzz_spline_clear",
    on: "fuzz_spline_on",
    leaf: "fuzz_spline_leaf",
};

/// The plane the curves lie in.
const Z: f64 = 0.7;

/// A random clamped spline on `[0, 1]` in the plane `z = Z`, its control
/// points in `[−2, 2]²`, rational on odd cases, with a doubled interior
/// knot on every third; and its interior knots.
fn spline(rng: &mut Rng, case: usize) -> (geom::NurbsCurve3<f64>, Vec<f64>) {
    loop {
        let p = 1 + rng.below(4);
        let mut inner: Vec<f64> = (0..rng.below(4)).map(|_| rng.range(0.05, 0.95)).collect();
        if case.is_multiple_of(3)
            && let Some(&v) = inner.first()
        {
            inner.push(v);
        }
        inner.sort_by(f64::total_cmp);
        let knots: Vec<f64> = core::iter::repeat_n(0.0, p + 1)
            .chain(inner.iter().copied())
            .chain(core::iter::repeat_n(1.0, p + 1))
            .collect();
        let count = knots.len() - p - 1;
        let control = (0..count)
            .map(|_| Point3::new(rng.range(-2.0, 2.0), rng.range(-2.0, 2.0), Z))
            .collect();
        let weights = (0..count)
            .map(|_| {
                if case.is_multiple_of(2) {
                    1.0
                } else {
                    0.1 + 6.0 * rng.unit() * rng.unit()
                }
            })
            .collect();
        let Ok(kv) = geom_core::spline::KnotVector::clamped(knots, p) else {
            continue;
        };
        if let Ok(c) = geom::NurbsCurve3::new(kv, control, weights) {
            return (c, inner);
        }
    }
}

/// The arc `(t0, t1)` of `c` as a polyline whose every chord is at most
/// `2·10⁻⁴` long: a fixed parameter step misses a narrow excursion on a
/// fast rational span, where the arc moves far for a small step.
fn polyline(c: &geom::NurbsCurve3<f64>, (t0, t1): (f64, f64)) -> Vec<(f64, f64)> {
    let at = |t: f64| {
        let x = c.eval(t);
        (x.x, x.y)
    };
    let mut out = vec![at(t0)];
    let mut stack = vec![(t0, t1)];
    while let Some((a, b)) = stack.pop() {
        let (pa, pb) = (at(a), at(b));
        let m = 0.5 * (a + b);
        let pm = at(m);
        let long = |p: (f64, f64), q: (f64, f64)| (p.0 - q.0).hypot(p.1 - q.1) > 2e-4;
        if (long(pa, pm) || long(pm, pb)) && a < m && m < b {
            stack.push((m, b));
            stack.push((a, m));
        } else {
            out.push(pm);
            out.push(pb);
        }
    }
    out
}

/// **Every ray the row counts crosses the arc as often as a dense
/// polyline of it says.** Points within `1e-3` of the arc, and rays the
/// polyline cannot read reliably (a near-tangent turn within `1e-3` of
/// the ray line ahead, a crossing within `1e-3` of `q` along the ray, an
/// end within `1e-3` of the line), are not asked. The row may refuse the
/// boundary reading or abandon a ray; it may not miscount one.
fn random_splines<T: Decide>(lane: &str, cases: usize) {
    let band = Band::linear(Tol::witness()).unwrap();
    let f = T::from_f64;
    let mut rng = test_utils::fuzz::start(&format!("spline_arcs_{lane}"));
    let (mut asked, mut abandoned, mut refused) = (0usize, 0usize, 0usize);
    let mut wrong = Vec::new();
    for case in 0..cases {
        let (c, inner) = spline(&mut rng, case);
        let t0 = if case.is_multiple_of(4) {
            0.0
        } else {
            rng.range(0.0, 0.3)
        };
        let t1 = match inner.last() {
            Some(&k) if case.is_multiple_of(5) && k > t0 => k,
            _ => rng.range(0.7, 1.0),
        };
        let Some(k) = SplineArc::new(&Arc::new(c.map_scalar(f)), (f(t0), f(t1))) else {
            panic!(
                "{lane} case {case}: a forward window refused; {}",
                test_utils::fuzz::replay()
            );
        };
        let pts = polyline(&c, (t0, t1));
        for _ in 0..40 {
            let q = (rng.range(-2.5, 2.5), rng.range(-2.5, 2.5));
            let a = rng.range(0.0, core::f64::consts::TAU);
            let (dx, dy) = (a.cos(), a.sin());
            let s = |(x, y): (f64, f64)| (y - q.1) * dx - (x - q.0) * dy;
            let g = |(x, y): (f64, f64)| (x - q.0) * dx + (y - q.1) * dy;
            let gap = pts
                .windows(2)
                .map(|w| {
                    let ((ax, ay), (bx, by)) = (w[0], w[1]);
                    let (ex, ey) = (bx - ax, by - ay);
                    let l = ex * ex + ey * ey;
                    let t = if l > 0.0 {
                        (((q.0 - ax) * ex + (q.1 - ay) * ey) / l).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };
                    (q.0 - ax - t * ex).hypot(q.1 - ay - t * ey)
                })
                .fold(f64::INFINITY, f64::min);
            let sv: Vec<f64> = pts.iter().map(|&p| s(p)).collect();
            let fragile = gap < 1e-3
                || (1..sv.len() - 1).any(|i| {
                    (sv[i] - sv[i - 1]) * (sv[i + 1] - sv[i]) <= 0.0
                        && sv[i].abs() < 1e-3
                        && g(pts[i]) > -1e-3
                })
                || pts
                    .windows(2)
                    .any(|w| (s(w[0]) > 0.0) != (s(w[1]) > 0.0) && g(w[0]).abs() < 1e-3)
                || sv[0].abs() < 1e-3
                || sv[sv.len() - 1].abs() < 1e-3;
            if fragile {
                continue;
            }
            let want = pts
                .windows(2)
                .filter(|w| (s(w[0]) > 0.0) != (s(w[1]) > 0.0) && g(w[0]) > 0.0)
                .count();
            let at = Point3::new(f(q.0), f(q.1), f(Z));
            if k.contact(at, ROWS, band) != Ok(ArcHit::Off) {
                refused += 1;
                continue;
            }
            let d = Vec3::new(f(dx), f(dy), f(0.0));
            let side = Vec3::new(f(0.0), f(0.0), f(1.0)).cross(d);
            asked += 1;
            match k.crossings(at, d, side, band) {
                None => abandoned += 1,
                Some(got) if got != want => wrong.push(format!(
                    "case {case} t {t0}..{t1} q {q:?} angle {a}: {got} crossings, the polyline {want}"
                )),
                Some(_) => {}
            }
        }
    }
    println!("{lane}: {asked} asked, {abandoned} abandoned, {refused} not read off");
    assert!(
        wrong.is_empty(),
        "{lane}: {} miscounted, first {:?}; {}",
        wrong.len(),
        wrong.first(),
        test_utils::fuzz::replay()
    );
}

#[test]
fn random_splines_at_f64() {
    random_splines::<f64>("f64", test_utils::fuzz::scaled(30));
}

#[test]
fn random_splines_at_interval() {
    random_splines::<Interval>("Interval", test_utils::fuzz::scaled(6));
}

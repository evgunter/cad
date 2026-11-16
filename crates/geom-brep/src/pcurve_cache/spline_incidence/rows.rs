//! **The spline chart's floor is a lower bound, and the door ends by
//! it.** A counterexample search compares each sample's floor with a
//! chart point's distance, enclosed at `Interval` (an upper bound on the
//! sample's distance from the chart), over rational and polynomial
//! patches of degree one to four with up to four interior knots of any
//! multiplicity, and near-degenerate nets: a row collapsed to a pole, a
//! net flattened to a micron, coincident neighbours, weights four
//! decades apart. Built with `PCERT_RADIAL_PAIR_MUTANT=spline-floor-net`
//! (the floor read as the nearest control point's distance, no hull) it
//! is red. The fixed rows read the door on a rational ring wall: a
//! circle on it, off it, and at a band-scale distance from it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom-brep/src/pcurve_cache/spline_incidence.rs"];

use geom::{Curve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Tol, Vec3};
use std::sync::Arc;

use super::{Net, Read};
use crate::PcurveCertifyError;

fn knots(rng: &mut test_utils::fuzz::Rng, p: usize) -> KnotVector {
    let mut interior: Vec<f64> = (0..rng.below(5)).map(|_| rng.range(0.05, 0.95)).collect();
    interior.sort_by(f64::total_cmp);
    let mut v = vec![0.0; p + 1];
    for x in interior {
        v.extend(core::iter::repeat_n(x, 1 + rng.below(p)));
    }
    v.extend(core::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(v, p).unwrap()
}

/// A random patch (module docs), its mode named for the message.
fn patch(rng: &mut test_utils::fuzz::Rng) -> (NurbsSurface<f64>, &'static str) {
    let (pu, pv) = (1 + rng.below(4), 1 + rng.below(4));
    let (ku, kv) = (knots(rng, pu), knots(rng, pv));
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let scale = 10f64.powf(rng.range(-2.0, 3.0));
    let offset = Vec3::new(
        rng.range(-5.0, 5.0),
        rng.range(-5.0, 5.0),
        rng.range(-5.0, 5.0),
    ) * scale;
    let mode = ["smooth", "folded", "pole", "flat", "coincident"][rng.below(5)];
    let mut control = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            let (x, y) = (i as f64 / nu as f64, j as f64 / nv as f64);
            let z = match mode {
                "folded" => rng.range(-1.0, 1.0),
                "flat" => rng.range(-1.0, 1.0) * 1e-6,
                _ => 0.3 * (3.0 * x).sin() * (2.0 * y).cos() + rng.range(-0.05, 0.05),
            };
            let (x, y) = match mode {
                "folded" => (x + rng.range(-0.5, 0.5), y + rng.range(-0.5, 0.5)),
                "pole" if i == 0 => (0.0, 0.5),
                _ => (x, y),
            };
            control.push(Point3::new(x * scale, y * scale, z * scale) + offset);
        }
    }
    if mode == "coincident" {
        for i in (1..nu).step_by(2) {
            for j in 0..nv {
                control[i * nv + j] = control[(i - 1) * nv + j];
            }
        }
    }
    let weights = match rng.below(3) {
        0 => vec![1.0; nu * nv],
        1 => (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-1.0, 1.0)))
            .collect(),
        _ => (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-2.0, 2.0)))
            .collect(),
    };
    (NurbsSurface::new(ku, kv, control, weights).unwrap(), mode)
}

/// The run's band: the search reads where the kernel does.
fn run_band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The band of the tightest ε the kernel runs at (the `1e-12` row),
/// where an `f64` floor's rounding is a band's size: the families that
/// pin the `f64` charge read here, whatever the run's ε.
fn tight_band() -> Band {
    Band::new(1e-12, 1e-11).unwrap()
}

fn lift(x: f64) -> Interval {
    <Interval as geom_core::Real>::from_f64(x)
}

/// A unit normal of `chart` at `(u, v)` by central differences, the
/// direction a sample is moved off a chart point along.
fn normal(chart: &NurbsSurface<f64>, u: f64, v: f64) -> Vec3<f64> {
    let ((u0, u1), (v0, v1)) = (
        chart.knot_values_u().domain(),
        chart.knot_values_v().domain(),
    );
    let h = 1e-6;
    let du = chart.eval((u + h).min(u1), v) - chart.eval((u - h).max(u0), v);
    let dv = chart.eval(u, (v + h).min(v1)) - chart.eval(u, (v - h).max(v0));
    let n = du.cross(dv);
    if n.norm() > 0.0 && n.norm().is_finite() {
        n.normalize()
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    }
}

/// What one family's samples read, and how far the worst `Off` floor
/// passed the sample's enclosed distance from a chart point (an upper
/// bound on its distance from the chart): a counterexample where past.
#[derive(Debug, Default)]
struct Tally {
    off: usize,
    on: usize,
    unsettled: usize,
    bad: usize,
}

impl Tally {
    fn read(&mut self, net: &Net<'_, f64>, q: Point3<f64>, upper: f64, band: Band) -> Read<f64> {
        let read = net.read(q, band);
        match read {
            Read::Off(floor) => {
                self.off += 1;
                if floor > upper {
                    self.bad += 1;
                }
            }
            Read::On => self.on += 1,
            Read::Unsettled(_) => self.unsettled += 1,
        }
        read
    }
}

/// **No sample is read off by more than a chart point's distance from
/// it** (module docs), at `f64` and at `Interval`, at the run's band. A
/// sample is a chart point at a random parameter, moved by `0`, along
/// the normal by one to a thousand escalations, or by `1e-10` to `10`
/// times the patch's reach in a random direction.
#[test]
fn a_samples_floor_is_below_a_chart_points_distance() {
    let mut rng = test_utils::fuzz::start("spline-chart-floor");
    let band = run_band();
    let (mut f64s, mut intervals) = (Tally::default(), Tally::default());
    for case in 0..test_utils::fuzz::scaled(400) {
        let (chart, mode) = patch(&mut rng);
        let lifted = chart.map_scalar(lift);
        let ((u0, u1), (v0, v1)) = (
            chart.knot_values_u().domain(),
            chart.knot_values_v().domain(),
        );
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let foot = chart.eval(u, v);
        let reach = (foot - Point3::origin()).norm().max(1.0);
        let q = match case % 4 {
            0 => foot,
            1 => foot + normal(&chart, u, v) * (band.escalate() * 10f64.powf(rng.range(0.0, 3.0))),
            _ => {
                let dir = Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                );
                foot + dir.normalize() * (10f64.powf(rng.range(-10.0, 1.0)) * reach)
            }
        };
        let qi = q.map(lift);
        let upper = (qi - lifted.eval(lift(u), lift(v))).norm().hi();
        let read = f64s.read(
            &Net::of(&chart, band).expect("the patch cuts"),
            q,
            upper,
            band,
        );
        let lifted_read = Net::of(&lifted, band)
            .expect("the patch cuts")
            .read(qi, band);
        if let Read::Off(floor) = lifted_read {
            intervals.off += 1;
            if floor.lo() > upper {
                intervals.bad += 1;
            }
        }
        assert!(
            f64s.bad == 0 && intervals.bad == 0,
            "case {case} ({mode}): q {q:?} at most {upper:e} from the chart reads off past it \
             at f64 {:?} or Interval: {}",
            matches!(read, Read::Off(_)),
            test_utils::fuzz::replay()
        );
    }
    eprintln!("[spline-chart-floor] f64 {f64s:?}, Interval {intervals:?}");
}

/// **A sample on the chart reads on it**: every sample that is a chart
/// point of a patch near the origin reads `On` at the run's band, the
/// walk's dive reaching a point within the zero band. Red where the walk
/// gives up its search for that point too soon, or drops the cell that
/// holds it.
#[test]
fn a_sample_on_the_chart_reads_on_it() {
    let mut rng = test_utils::fuzz::start("spline-chart-on");
    let band = run_band();
    for case in 0..test_utils::fuzz::scaled(200) {
        let (chart, mode) = near_patch(&mut rng);
        let ((u0, u1), (v0, v1)) = (
            chart.knot_values_u().domain(),
            chart.knot_values_v().domain(),
        );
        let q = chart.eval(rng.range(u0, u1), rng.range(v0, v1));
        let read = Net::of(&chart, band).expect("the patch cuts").read(q, band);
        assert!(
            matches!(read, Read::On),
            "case {case} ({mode}): a chart point does not read on the chart: {}",
            test_utils::fuzz::replay()
        );
    }
}

/// [`patch`] at a scale of a tenth to ten, placed within ten of the
/// origin, its weights within a decade of one another: a chart whose own
/// evaluation rounds far inside the zero band. Not the coincident mode:
/// two equal rows of unequal weights make a sheet a band thin over a
/// width, which the walk's visit cap stops on unsettled
/// (`work/pcert/the-spline-incidence-read-dives-where-its-floors-already-decide.md`).
fn near_patch(rng: &mut test_utils::fuzz::Rng) -> (NurbsSurface<f64>, &'static str) {
    let (chart, mode) = loop {
        let (chart, mode) = patch(rng);
        if mode != "coincident" {
            break (chart, mode);
        }
    };
    let (lo, hi) = super::shadow_box(chart.control());
    let size = (0..3)
        .map(|k| (hi[k] - lo[k]).powi(2))
        .sum::<f64>()
        .sqrt()
        .max(1e-300);
    let f = 10f64.powf(rng.range(-1.0, 1.0)) / size;
    let o = Vec3::new(
        rng.range(-10.0, 10.0),
        rng.range(-10.0, 10.0),
        rng.range(-10.0, 10.0),
    );
    let control = chart
        .control()
        .iter()
        .map(|p| Point3::new((p.x - lo[0]) * f, (p.y - lo[1]) * f, (p.z - lo[2]) * f) + o)
        .collect();
    let weights = chart
        .weights()
        .iter()
        .map(|_| 10f64.powf(rng.range(-0.5, 0.5)))
        .collect();
    let chart = NurbsSurface::new(
        chart.knot_values_u().clone(),
        chart.knot_values_v().clone(),
        control,
        weights,
    )
    .unwrap();
    (chart, mode)
}

/// **The `f64` charge holds where its rounding is a band's size**, at
/// the tight band: a sample a few escalations off the corner of a far,
/// large, nearly flat patch, along the outward diagonal, where the box
/// gap is the distance (fifteen hundred of them); a sample off a straight
/// boundary row of a knotted rational patch, weights up to eight decades
/// apart, where the box gap of the cell holding the foot is the distance
/// and the cut's insertions round the row (a thousand); and a chart
/// point of a patch `1e3` to `1e6` across (a hundred). No `Off` floor
/// passes a chart point's enclosed distance.
///
/// Built with `PCERT_RADIAL_PAIR_MUTANT=spline-charge-dropped` (no
/// `f64` charge) the corner and boundary-row families are red.
#[test]
fn the_f64_charge_holds_where_its_rounding_is_a_bands_size() {
    let band = tight_band();
    let corner = corner_family(&mut test_utils::fuzz::start("spline-charge-corner"), band);
    let row = row_family(&mut test_utils::fuzz::start("spline-charge-row"), band);
    let big = big_family(&mut test_utils::fuzz::start("spline-charge-big"), band);
    eprintln!("[spline-charge] corner {corner:?}, row {row:?}, big {big:?}");
    assert!(
        corner.bad == 0 && row.bad == 0 && big.bad == 0,
        "an f64 floor passes a chart point's distance: corner {corner:?}, row {row:?}, big \
         {big:?}; {}",
        test_utils::fuzz::replay()
    );
}

/// The corner family ([`the_f64_charge_holds_where_its_rounding_is_a_bands_size`]).
fn corner_family(rng: &mut test_utils::fuzz::Rng, band: Band) -> Tally {
    let mut tally = Tally::default();
    for _ in 0..test_utils::fuzz::scaled(1500) {
        let p = 1 + rng.below(3);
        let k = KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap();
        let n = p + 1;
        let scale = 10f64.powf(rng.range(2.0, 5.0));
        let o = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        ) * 10f64.powf(rng.range(0.0, 5.0));
        let mut control = Vec::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                control.push(
                    Point3::new(
                        (i as f64 / p as f64 + rng.range(0.0, 0.01)) * scale,
                        (j as f64 / p as f64 + rng.range(0.0, 0.01)) * scale,
                        0.0,
                    ) + o,
                );
            }
        }
        // The corner is the box's.
        control[0] = Point3::new(o.x - 1e-3 * scale, o.y - 1e-3 * scale, o.z);
        let spread = rng.range(0.0, 3.0);
        let weights = (0..n * n)
            .map(|_| 10f64.powf(rng.range(-spread, spread)))
            .collect();
        let corner = control[0];
        let chart = NurbsSurface::new(k.clone(), k, control, weights).unwrap();
        let q = corner
            + Vec3::new(-1.0, -1.0, 1.0).normalize() * (band.escalate() * rng.range(1.5, 20.0));
        let upper = (q.map(lift) - chart.map_scalar(lift).eval(lift(0.0), lift(0.0)))
            .norm()
            .hi();
        tally.read(&Net::of(&chart, band).unwrap(), q, upper, band);
    }
    tally
}

/// The boundary-row family ([`the_f64_charge_holds_where_its_rounding_is_a_bands_size`]).
fn row_family(rng: &mut test_utils::fuzz::Rng, band: Band) -> Tally {
    let mut tally = Tally::default();
    for _ in 0..test_utils::fuzz::scaled(1000) {
        let (pu, pv) = (2 + rng.below(3), 1 + rng.below(3));
        let (ku, kv) = (knots(rng, pu), knots(rng, pv));
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let scale = 10f64.powf(rng.range(0.0, 4.0));
        let o = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        ) * 10f64.powf(rng.range(0.0, 4.0));
        let mut control = Vec::with_capacity(nu * nv);
        for i in 0..nu {
            for j in 0..nv {
                let x = (i as f64 / nu as f64) * scale;
                let (y, z) = if j == 0 {
                    (0.0, 0.0)
                } else {
                    ((j as f64 / nv as f64) * scale, rng.range(-0.3, 0.3) * scale)
                };
                control.push(Point3::new(x, y, z) + o);
            }
        }
        let spread = [1.0, 2.0, 4.0, 8.0][rng.below(4)];
        let weights = (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-spread, spread)))
            .collect();
        let chart = NurbsSurface::new(ku, kv, control, weights).unwrap();
        let u = rng.range(0.0, 1.0);
        let q = chart.eval(u, 0.0)
            + Vec3::new(0.0, -1.0, 0.0) * (band.escalate() * rng.range(1.5, 50.0));
        let upper = (q.map(lift) - chart.map_scalar(lift).eval(lift(u), lift(0.0)))
            .norm()
            .hi();
        tally.read(&Net::of(&chart, band).unwrap(), q, upper, band);
    }
    tally
}

/// The big-patch family ([`the_f64_charge_holds_where_its_rounding_is_a_bands_size`]):
/// the sample is the middle of the chart point's enclosure, so its
/// distance is at most the enclosure's reach.
fn big_family(rng: &mut test_utils::fuzz::Rng, band: Band) -> Tally {
    let mut tally = Tally::default();
    for _ in 0..test_utils::fuzz::scaled(100) {
        let (chart, _) = patch(rng);
        let s = 10f64.powf(rng.range(3.0, 6.0));
        let (lo, hi) = super::shadow_box(chart.control());
        let f = s
            / (0..3)
                .map(|k| (hi[k] - lo[k]).powi(2))
                .sum::<f64>()
                .sqrt()
                .max(1e-300);
        let control = chart
            .control()
            .iter()
            .map(|p| Point3::new(p.x * f, p.y * f, p.z * f))
            .collect();
        let chart = NurbsSurface::new(
            chart.knot_values_u().clone(),
            chart.knot_values_v().clone(),
            control,
            chart.weights().to_vec(),
        )
        .unwrap();
        let ((u0, u1), (v0, v1)) = (
            chart.knot_values_u().domain(),
            chart.knot_values_v().domain(),
        );
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let at = chart.map_scalar(lift).eval(lift(u), lift(v));
        let mid = |x: Interval| 0.5 * (x.lo() + x.hi());
        let q = Point3::new(mid(at.x), mid(at.y), mid(at.z));
        let upper = (q.map(lift) - at).norm().hi();
        tally.read(&Net::of(&chart, band).unwrap(), q, upper, band);
    }
    tally
}

/// **The cut's `f64` rounding bound holds the cut's drift**: over
/// Bézier patches of degree one to five, weights up to eight decades
/// apart, placed up to `1e5` of their size away, each point of a cell
/// quartered at `f64` along a random descent up to thirty levels deep
/// lies within its tracked bound ([`super::point_rounding`]) of the same
/// descent in double-double arithmetic (a reference exact to about
/// `1e-30` of the coordinates).
#[test]
fn the_cut_rounding_bound_holds_the_cut_drift() {
    let worst = cut_drift(&mut test_utils::fuzz::start("spline-cut-drift"));
    eprintln!("[spline-cut-drift] worst drift over bound {worst}");
}

/// **The cut's bound is within a factor of four of a drift it bounds**,
/// on [`the_cut_rounding_bound_holds_the_cut_drift`]'s samples at one
/// seed: some point drifts past a quarter of its bound, so a bound cut
/// to a quarter is passed, and built with
/// `PCERT_RADIAL_PAIR_MUTANT=spline-bound-undercut` (every point's bound
/// cut to a quarter) both rows are red. The seed is a fixture
/// identifier (discipline §8, shape 3): over its four hundred patches the
/// worst ratio is about `0.55`, as it is at every seed sampled.
#[test]
fn the_cut_rounding_bound_comes_within_a_quarter_of_the_drift() {
    let worst = cut_drift(&mut test_utils::fuzz::pinned(
        "spline-cut-drift-tight",
        0x5eed_0005,
    ));
    assert!(
        worst > 0.25,
        "the bound's worst ratio to the drift is {worst}: an under-cut to a quarter would pass"
    );
}

/// The drift search's samples ([`the_cut_rounding_bound_holds_the_cut_drift`]):
/// the worst ratio of a drift to its bound, every drift within it.
fn cut_drift(rng: &mut test_utils::fuzz::Rng) -> f64 {
    let band = tight_band();
    let (mut worst, mut points) = (0.0f64, 0usize);
    for case in 0..test_utils::fuzz::scaled(400) {
        let (pu, pv) = (1 + rng.below(5), 1 + rng.below(5));
        let bezier = |p: usize| {
            KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap()
        };
        let (nu, nv) = (pu + 1, pv + 1);
        let scale = 10f64.powf(rng.range(-2.0, 3.0));
        let o = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        ) * scale
            * [0.0, 10.0, 1e3, 1e5][rng.below(4)];
        let control: Vec<Point3<f64>> = (0..nu * nv)
            .map(|_| {
                Point3::origin()
                    + Vec3::new(
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                        rng.range(-1.0, 1.0),
                    ) * scale
                    + o
            })
            .collect();
        let decades = [0.0, 2.0, 4.0, 8.0][rng.below(4)];
        let weights: Vec<f64> = (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-decades / 2.0, decades / 2.0)))
            .collect();
        let chart =
            NurbsSurface::new(bezier(pu), bezier(pv), control.clone(), weights.clone()).unwrap();
        let net = Net::of(&chart, band).unwrap();
        // The reference: the same net about the same centre, in
        // double-double.
        let c = [net.centre.x, net.centre.y, net.centre.z];
        let mut reference: Vec<[Dd; 4]> = control
            .iter()
            .zip(&weights)
            .map(|(p, &w)| {
                let w = Dd::of(w);
                let at = |x: f64, c: f64| Dd::of(x).sub(Dd::of(c)).mul(w);
                [at(p.x, c[0]), at(p.y, c[1]), at(p.z, c[2]), w]
            })
            .collect();
        let mut cell = net.cells[0].clone();
        for _ in 0..rng.below(31) {
            let (left, right) = (rng.below(2) == 0, rng.below(2) == 0);
            cell = descend(&cell, pu, pv, left, right);
            reference = dd_descend(&reference, pu, pv, left, right);
        }
        for (got, exact) in cell.net.iter().zip(&reference) {
            let bound = super::point_rounding(got);
            let at = super::point(got.0);
            for (k, x) in [at.x, at.y, at.z].into_iter().enumerate() {
                let drift = Dd::of(x).sub(exact[k].div(exact[3])).abs_hi();
                points += 1;
                worst = worst.max(drift / bound[k]);
                assert!(
                    drift <= bound[k],
                    "case {case}: a point drifts {drift:e} past its bound {:e}: {}",
                    bound[k],
                    test_utils::fuzz::replay()
                );
            }
        }
    }
    let _ = points;
    worst
}

/// A double-double: an unevaluated sum `hi + lo`, `|lo| ≤ ulp(hi)/2`.
#[derive(Clone, Copy, Debug)]
struct Dd(f64, f64);

impl Dd {
    fn of(x: f64) -> Self {
        Self(x, 0.0)
    }

    fn two_sum(a: f64, b: f64) -> Self {
        let s = a + b;
        let bb = s - a;
        Self(s, (a - (s - bb)) + (b - bb))
    }

    fn add(self, o: Self) -> Self {
        let s = Self::two_sum(self.0, o.0);
        Self::two_sum(s.0, s.1 + self.1 + o.1)
    }

    fn sub(self, o: Self) -> Self {
        self.add(Self(-o.0, -o.1))
    }

    fn mul(self, o: Self) -> Self {
        let p = self.0 * o.0;
        let e = self.0.mul_add(o.0, -p);
        Self::two_sum(p, e + self.0 * o.1 + self.1 * o.0)
    }

    fn half(self) -> Self {
        Self(self.0 * 0.5, self.1 * 0.5)
    }

    fn div(self, o: Self) -> Self {
        let q = self.0 / o.0;
        let r = self.sub(Self::of(q).mul(o));
        Self::two_sum(q, r.0 / o.0)
    }

    fn value(self) -> f64 {
        self.0 + self.1
    }

    fn abs_hi(self) -> f64 {
        (self.0 + self.1).abs()
    }
}

/// [`descend`] on a double-double net: the same halvings by de
/// Casteljau.
fn dd_descend(net: &[[Dd; 4]], pu: usize, pv: usize, left: bool, right: bool) -> Vec<[Dd; 4]> {
    let halve = |net: &[[Dd; 4]], along_u: bool, keep_left: bool| {
        let (n, m) = if along_u { (pu, pv) } else { (pv, pu) };
        let at = |i: usize, k: usize| {
            if along_u {
                i * (pv + 1) + k
            } else {
                k * (pv + 1) + i
            }
        };
        let mut out = net.to_vec();
        for k in 0..=m {
            let mut row: Vec<[Dd; 4]> = (0..=n).map(|i| net[at(i, k)]).collect();
            for level in 0..=n {
                if keep_left {
                    out[at(level, k)] = row[0];
                } else {
                    out[at(n - level, k)] = row[n - level];
                }
                for i in 0..n - level {
                    let (a, b) = (row[i], row[i + 1]);
                    row[i] = core::array::from_fn(|c| a[c].add(b[c].sub(a[c]).half()));
                }
            }
        }
        out
    };
    halve(&halve(net, true, left), false, !right)
}

/// **The cut's `f64` rounding bound holds a knotted cut's drift**: over
/// knotted rational patches of degree one to four, any interior knots
/// of any multiplicity, weights up to eight decades apart, placed up to
/// `1e5` of their size away, each point of each Bézier cell the
/// insertions cut, and of a cell then halved along a random descent up
/// to twelve levels, lies within its tracked bound
/// ([`super::point_rounding`]) of the same cut by Boehm's insertion in
/// double-double ([`dd_refine`]). So the insertion branch of
/// [`super::blend`], its ratio's rounding and its product's, meets a
/// reference. Built with `PCERT_RADIAL_PAIR_MUTANT=spline-product-uncharged`
/// (the product by the ratio uncharged) the row stays green: that drops
/// one unit of the five a ratio's blend charges on `|y − x|·t`, and a
/// drift reaches past the rest only where all five roundings meet at
/// their worst. Under it the worst drift found is `0.81` of the bound,
/// at twenty times this row's effort.
#[test]
fn the_cut_rounding_bound_holds_a_knotted_cuts_drift() {
    let worst = knotted_drift(&mut test_utils::fuzz::start("spline-knotted-drift"));
    eprintln!("[spline-knotted-drift] worst drift over bound {worst}");
}

/// A knotted patch for [`knotted_drift`] and
/// [`a_floor_is_below_the_exact_cells_hull`]: the chart, its net, and
/// the net's exact Bézier cells about the same centre.
fn knotted(rng: &mut test_utils::fuzz::Rng) -> (NurbsSurface<f64>, Vec<Vec<[Dd; 4]>>) {
    let (pu, pv) = (1 + rng.below(4), 1 + rng.below(4));
    let (ku, kv) = (knots(rng, pu), knots(rng, pv));
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let scale = 10f64.powf(rng.range(-2.0, 3.0));
    let o = Vec3::new(
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
        rng.range(-1.0, 1.0),
    ) * scale
        * [0.0, 10.0, 1e3, 1e5][rng.below(4)];
    let control: Vec<Point3<f64>> = (0..nu * nv)
        .map(|_| {
            Point3::origin()
                + Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                ) * scale
                + o
        })
        .collect();
    let decades = [0.0, 2.0, 4.0, 8.0][rng.below(4)];
    let weights: Vec<f64> = (0..nu * nv)
        .map(|_| 10f64.powf(rng.range(-decades / 2.0, decades / 2.0)))
        .collect();
    let chart = NurbsSurface::new(ku, kv, control.clone(), weights.clone()).unwrap();
    let net = Net::of(&chart, tight_band()).unwrap();
    let c = [net.centre.x, net.centre.y, net.centre.z];
    let at = |k: usize| {
        let w = Dd::of(weights[k]);
        let moved = |x: f64, c: f64| Dd::of(x).sub(Dd::of(c)).mul(w);
        let p = control[k];
        [moved(p.x, c[0]), moved(p.y, c[1]), moved(p.z, c[2]), w]
    };
    let (u, v) = (
        chart.knot_values_u().values(),
        chart.knot_values_v().values(),
    );
    let rows: Vec<Vec<[Dd; 4]>> = (0..nu)
        .map(|i| dd_refine(v, pv, (0..nv).map(|j| at(i * nv + j)).collect()))
        .collect();
    let columns: Vec<Vec<[Dd; 4]>> = (0..rows[0].len())
        .map(|j| dd_refine(u, pu, rows.iter().map(|r| r[j]).collect()))
        .collect();
    let (cu, cv) = ((columns[0].len() - 1) / pu, (columns.len() - 1) / pv);
    assert_eq!(
        net.cells.len(),
        cu * cv,
        "the cut's cells are the reference's"
    );
    let mut cells = Vec::with_capacity(cu * cv);
    for i in 0..cu {
        for j in 0..cv {
            let mut cell = Vec::with_capacity((pu + 1) * (pv + 1));
            for a in 0..=pu {
                for b in 0..=pv {
                    cell.push(columns[j * pv + b][i * pu + a]);
                }
            }
            cells.push(cell);
        }
    }
    (chart, cells)
}

/// The knotted drift search ([`the_cut_rounding_bound_holds_a_knotted_cuts_drift`]):
/// the worst ratio of a drift to its bound, every drift within it.
fn knotted_drift(rng: &mut test_utils::fuzz::Rng) -> f64 {
    let mut worst = 0.0f64;
    for case in 0..test_utils::fuzz::scaled(400) {
        let (chart, exact) = knotted(rng);
        let net = Net::of(&chart, tight_band()).unwrap();
        let (pu, pv) = (net.pu, net.pv);
        for (cell, reference) in net.cells.iter().zip(exact) {
            let (mut cell, mut reference) = (cell.clone(), reference);
            for _ in 0..rng.below(13) {
                let (left, right) = (rng.below(2) == 0, rng.below(2) == 0);
                cell = descend(&cell, pu, pv, left, right);
                reference = dd_descend(&reference, pu, pv, left, right);
            }
            for (got, exact) in cell.net.iter().zip(&reference) {
                let bound = super::point_rounding(got);
                let at = super::point(got.0);
                for (k, x) in [at.x, at.y, at.z].into_iter().enumerate() {
                    let drift = Dd::of(x).sub(exact[k].div(exact[3])).abs_hi();
                    worst = worst.max(drift / bound[k]);
                    assert!(
                        drift <= bound[k],
                        "case {case}: a knotted cut's point drifts {drift:e} past its bound {:e}: {}",
                        bound[k],
                        test_utils::fuzz::replay()
                    );
                }
            }
        }
    }
    worst
}

/// **A cell's `f64` floor is below its exact hull's**: on
/// [`knotted`]'s cells, a sample placed past the computed box along an
/// axis by less than the cell's cut charge ([`super::cut_rounding`]) has
/// a floor ([`super::Net::floor`]) no larger than the larger of its
/// distances to the exact control points' box and to their slab along
/// the computed slab's direction, either of which bounds its distance
/// from the exact cell. So the floor's subtraction of the cut term is
/// pinned: built with `PCERT_RADIAL_PAIR_MUTANT=spline-cut-uncharged`
/// the row is red.
#[test]
fn a_floor_is_below_the_exact_cells_hull() {
    let rng = &mut test_utils::fuzz::start("spline-floor-hull");
    let mut placed = 0usize;
    for case in 0..test_utils::fuzz::scaled(400) {
        let (chart, exact) = knotted(rng);
        let net = Net::of(&chart, tight_band()).unwrap();
        for (cell, reference) in net.cells.iter().zip(&exact) {
            let cut = super::cut_rounding(cell);
            if !cut.is_finite() || cut == 0.0 {
                continue;
            }
            let points: Vec<Point3<f64>> = cell.net.iter().map(|&(h, _)| super::point(h)).collect();
            let axis = rng.below(3);
            let high = rng.below(2) == 0;
            let coordinate = |p: &Point3<f64>| [p.x, p.y, p.z][axis];
            let edge = points.iter().map(coordinate).fold(
                if high {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                },
                |a, b| {
                    if high { a.max(b) } else { a.min(b) }
                },
            );
            let mid = |k: usize| {
                let (lo, hi) =
                    points
                        .iter()
                        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
                            let x = [p.x, p.y, p.z][k];
                            (lo.min(x), hi.max(x))
                        });
                0.5 * (lo + hi)
            };
            let past = cut * rng.range(0.05, 0.95);
            let mut q = [mid(0), mid(1), mid(2)];
            q[axis] = if high { edge + past } else { edge - past };
            let q = Point3::new(q[0], q[1], q[2]);
            let floor = net.floor(cell, q);
            let exact_points: Vec<[Dd; 3]> = reference
                .iter()
                .map(|h| [h[0].div(h[3]), h[1].div(h[3]), h[2].div(h[3])])
                .collect();
            let gap = |lo: Dd, hi: Dd, x: Dd| lo.sub(x).value().max(x.sub(hi).value()).max(0.0);
            let least = |a: Dd, b: Dd| if b.sub(a).value() < 0.0 { b } else { a };
            let most = |a: Dd, b: Dd| if b.sub(a).value() > 0.0 { b } else { a };
            let bounds = |k: usize| {
                exact_points
                    .iter()
                    .fold((exact_points[0][k], exact_points[0][k]), |(lo, hi), p| {
                        (least(lo, p[k]), most(hi, p[k]))
                    })
            };
            let qs = [Dd::of(q.x), Dd::of(q.y), Dd::of(q.z)];
            let box_gap = (0..3)
                .map(|k| {
                    let (lo, hi) = bounds(k);
                    gap(lo, hi, qs[k]).powi(2)
                })
                .sum::<f64>()
                .sqrt();
            let corner = |a: usize, b: usize| points[a * (net.pv + 1) + b];
            let n = (corner(net.pu, net.pv) - corner(0, 0))
                .cross(corner(0, net.pv) - corner(net.pu, 0));
            let slab_gap = if n.norm() > 0.0 && n.norm().is_finite() {
                let height = |p: [Dd; 3]| {
                    p[0].mul(Dd::of(n.x))
                        .add(p[1].mul(Dd::of(n.y)))
                        .add(p[2].mul(Dd::of(n.z)))
                };
                let heights: Vec<Dd> = exact_points.iter().map(|&p| height(p)).collect();
                let lo = heights.iter().copied().fold(heights[0], least);
                let hi = heights.iter().copied().fold(heights[0], most);
                // `|n|` rounds by a unit or two; the gap is read on the
                // longer side of it.
                gap(lo, hi, height(qs)) / (n.norm() * (1.0 - 4.0 * f64::EPSILON))
            } else {
                0.0
            };
            let exact_gap = box_gap.max(slab_gap);
            placed += 1;
            assert!(
                floor <= exact_gap,
                "case {case}: a floor {floor:e} past the exact hull's {exact_gap:e} \
                 (cut charge {cut:e}, placed {past:e} past the box): {}",
                test_utils::fuzz::replay()
            );
        }
    }
    assert!(
        placed > 0,
        "no cell had a cut charge to place a sample within"
    );
}

/// Boehm's insertion in double-double: `row`'s homogeneous points on the
/// knots `knots` of degree `p`, each interior knot raised to
/// multiplicity `p`, so the row is its Bézier segments' nets end to end.
/// Each ratio is formed from the exact knots, so the row is exact to
/// about `1e-30` of its coordinates.
fn dd_refine(knots: &[f64], p: usize, row: Vec<[Dd; 4]>) -> Vec<[Dd; 4]> {
    let (mut knots, mut row) = (knots.to_vec(), row);
    let (first, last) = (knots[0], knots[knots.len() - 1]);
    let mut interior: Vec<f64> = knots
        .iter()
        .copied()
        .filter(|&x| x != first && x != last)
        .collect();
    interior.dedup();
    for x in interior {
        while knots.iter().filter(|&&k| k == x).count() < p {
            let s = knots.iter().filter(|&&k| k == x).count();
            let k = knots.iter().rposition(|&u| u <= x).unwrap();
            let mut out = Vec::with_capacity(row.len() + 1);
            for i in 0..=row.len() {
                if i + p <= k {
                    out.push(row[i]);
                } else if i + s > k {
                    out.push(row[i - 1]);
                } else {
                    let a = Dd::of(x)
                        .sub(Dd::of(knots[i]))
                        .div(Dd::of(knots[i + p]).sub(Dd::of(knots[i])));
                    let (lo, hi) = (row[i - 1], row[i]);
                    out.push(core::array::from_fn(|c| lo[c].add(hi[c].sub(lo[c]).mul(a))));
                }
            }
            knots.insert(k + 1, x);
            row = out;
        }
    }
    row
}

/// The quarter of `cell` its two halvings pick, `u` then `v`.
fn descend<T: geom_core::Decide>(
    cell: &super::Cell<T>,
    pu: usize,
    pv: usize,
    left: bool,
    right: bool,
) -> super::Cell<T> {
    let (l, r) = cell.split(pu, pv, true);
    let (l, r) = if left { l } else { r }.split(pu, pv, false);
    if right { r } else { l }
}

/// The unit cylinder `z ∈ [0, 1]` over `[0, 1.2]` rad from `x̂`, in two
/// rational quadratic sub-arcs.
fn ring<T: geom_core::Real>() -> Surface<T> {
    let h = 1.2 / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = j as f64 * h / 2.0;
        let (r, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for z in [0.0, 1.0] {
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// A circle about the ring's axis at `z = 0.5`, of radius `r`, over the
/// wall's turn less a tenth at each end.
fn read<T: geom_core::Decide>(r: f64) -> Result<(), PcurveCertifyError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let c = |x: f64| T::from_f64(x);
    let circle = Curve3::circle(
        Point3::new(c(0.0), c(0.0), c(0.5)),
        Vec3::new(c(0.0), c(0.0), c(1.0)),
        c(r),
        Vec3::new(c(1.0), c(0.0), c(0.0)),
    );
    crate::read_edge_incidence(&circle, c(0.1), c(1.1), &ring::<T>(), band)
}

fn the_door_reads_a_circle_against_a_ring_wall<T: geom_core::Decide>() {
    let band = Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        read::<T>(1.0),
        Ok(()),
        "a circle on the wall, at {}",
        T::NAME
    );
    let off = read::<T>(1.0 + 1e-3);
    assert!(
        matches!(
            off.as_ref().err().and_then(crate::IncidenceRead::of),
            Some(crate::IncidenceRead::Off(_))
        ) && off.as_ref().err().and_then(|e| e.chart_carrier_kinds())
            == Some((geom::SurfaceKind::Nurbs, geom::CurveKind::Circle)),
        "a circle a millimetre off the wall, at {}: {off:?}",
        T::NAME
    );
    let near = read::<T>(1.0 + 0.5 * (band.zero() + band.escalate()));
    assert!(
        matches!(
            near.as_ref().err().and_then(crate::IncidenceRead::of),
            Some(crate::IncidenceRead::Undecided(_))
        ),
        "a circle at a band-scale distance from the wall, at {}: {near:?}",
        T::NAME
    );
}

#[test]
fn the_door_reads_a_circle_against_a_ring_wall_at_f64() {
    the_door_reads_a_circle_against_a_ring_wall::<f64>();
}

#[test]
fn the_door_reads_a_circle_against_a_ring_wall_at_interval() {
    the_door_reads_a_circle_against_a_ring_wall::<Interval>();
}

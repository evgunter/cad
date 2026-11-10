//! **An arc rim's certificate pays its weight margin**: a chart column
//! whose sub-arc weight is `cos(h/2) + δ`, `δ` inside the band, misses
//! the circle by up to [`super::arc_rim_weight_miss`], and the envelope
//! must hold that miss. The miss is read here off the geometry itself,
//! `|S(P(t)) − C(t)|` sampled densely, never off the bound's formula.
//! Every row holds at `f64` and at `Interval`, at every ε of the run;
//! `iso_arc_weight_search` sweeps the same check, and
//! `scripts/oracles/iso_arc_weight_miss_mpmath.py` re-derives the miss
//! of its dumped rows at 50 digits.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT` naming `arc-rim-weight-unpaid`
//! (the payment dropped), every row here and the search go red.
//!
//! The rows' wall: a circle of radius `R` about the origin in `z = 0`,
//! and a wall of `n` uniform rational-quadratic sub-arcs of angle `h` in
//! `u`, linear in `v`, at given held end weights and NORMALIZED interior
//! weights; it is the circle exactly where each interior weight is
//! `cos(h/2)`.

use geom::{Curve3, NurbsSurface, Surface};
use geom_core::spline::{KnotVector, Knots, SpanLocate};
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};

use super::harmonic_sup_rows::{band, zero};
use super::{Pcurve, PcurveCache};
use crate::pick_arc_rim;

const HEIGHT: f64 = 0.01;

#[derive(Debug, Clone)]
pub(super) struct ArcRow {
    pub(super) radius: f64,
    /// The sub-arc angle.
    pub(super) h: f64,
    /// The held weights at the `n + 1` joints.
    pub(super) ends: Vec<f64>,
    /// The `n` interior weights at unit end weights.
    pub(super) normal: Vec<f64>,
}

#[derive(Debug)]
pub(super) enum Verdict {
    /// Certified, with the envelope's upper end.
    Certified {
        envelope: f64,
    },
    Refused(String),
    NoPick(String),
}

impl ArcRow {
    pub(super) fn uniform(radius: f64, h: f64, ends: &[f64], normal: &[f64]) -> Self {
        assert_eq!(
            ends.len(),
            normal.len() + 1,
            "one interior weight per sub-arc"
        );
        Self {
            radius,
            h,
            ends: ends.to_vec(),
            normal: normal.to_vec(),
        }
    }

    /// A row from its held weights `e₀, w₀, e₁, w₁, …, eₙ`.
    pub(super) fn held(radius: f64, h: f64, weights: &[f64]) -> Self {
        let ends: Vec<f64> = weights.iter().step_by(2).copied().collect();
        let normal = (0..ends.len() - 1)
            .map(|k| weights[2 * k + 1] / (ends[k] * ends[k + 1]).sqrt())
            .collect::<Vec<_>>();
        Self::uniform(radius, h, &ends, &normal)
    }

    /// One quarter sub-arc at held weights `w`.
    pub(super) fn quarter(radius: f64, w: [f64; 3]) -> Self {
        Self::held(radius, core::f64::consts::FRAC_PI_2, &w)
    }

    /// Two quarter sub-arcs at held weights `w`.
    pub(super) fn half(radius: f64, w: [f64; 5]) -> Self {
        Self::held(radius, core::f64::consts::FRAC_PI_2, &w)
    }

    pub(super) fn spans(&self) -> usize {
        self.normal.len()
    }

    pub(super) fn angle(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let n = self.spans() as f64;
        self.h * n
    }

    /// The column's control points in the plane and its held weights.
    pub(super) fn column(&self) -> (Vec<[f64; 2]>, Vec<f64>) {
        let (r, h) = (self.radius, self.h);
        let c = (h / 2.0).cos();
        let mut pts = vec![[r, 0.0]];
        let mut w = vec![self.ends[0]];
        for k in 0..self.spans() {
            #[allow(clippy::cast_precision_loss)]
            let base = h * k as f64;
            let mid = base + h / 2.0;
            pts.push([r / c * mid.cos(), r / c * mid.sin()]);
            w.push(self.normal[k] * (self.ends[k] * self.ends[k + 1]).sqrt());
            pts.push([r * (base + h).cos(), r * (base + h).sin()]);
            w.push(self.ends[k + 1]);
        }
        (pts, w)
    }

    fn carrier<T: Real>(&self) -> Curve3<T> {
        let f = T::from_f64;
        Curve3::circle(
            Point3::new(f(0.0), f(0.0), f(0.0)),
            Vec3::new(f(0.0), f(0.0), f(1.0)),
            f(self.radius),
            Vec3::new(f(1.0), f(0.0), f(0.0)),
        )
    }

    pub(super) fn wall<T: SpanLocate>(&self) -> Surface<T> {
        let f = T::from_f64;
        let (pts, w) = self.column();
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for (p, wk) in pts.iter().zip(&w) {
            for z in [0.0, HEIGHT] {
                control.push(Point3::new(f(p[0]), f(p[1]), f(z)));
                weights.push(f(*wk));
            }
        }
        let n = self.spans();
        let mut ku = vec![0.0, 0.0, 0.0];
        for k in 1..n {
            #[allow(clippy::cast_precision_loss)]
            let t = k as f64 / n as f64;
            ku.extend([t, t]);
        }
        ku.extend([1.0, 1.0, 1.0]);
        let ku = KnotVector::clamped(ku, 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        Surface::Nurbs(std::sync::Arc::new(
            NurbsSurface::new(
                Knots::from(ku).lift(f),
                Knots::from(kv).lift(f),
                control,
                weights,
            )
            .unwrap(),
        ))
    }

    /// The arc rim's certificate at `T`, at the run's band.
    pub(super) fn certify<T: Decide + Bounds>(&self) -> Verdict {
        let band = band();
        let f = T::from_f64;
        let (carrier, wall) = (self.carrier::<T>(), self.wall::<T>());
        let angle = f(self.angle());
        match pick_arc_rim(&carrier, (f(0.0), angle), &wall, band) {
            Err(e) => Verdict::NoPick(format!("{e:?}")),
            Ok(None) => Verdict::NoPick("none".into()),
            Ok(Some(image)) => {
                match PcurveCache::certify(image, f(0.0), angle, &carrier, &wall, band) {
                    Ok(c) => Verdict::Certified {
                        envelope: c.certificate().envelope.hi(),
                    },
                    Err(e) => Verdict::Refused(format!("{e:?}")),
                }
            }
        }
    }

    /// `sup |S(P(t)) − C(t)|` over the arc at `f64`, sampled at 4001
    /// parameters and refined about the worst: a lower bound on the
    /// miss, read off the image and the wall rather than any formula.
    pub(super) fn sampled_miss(&self) -> f64 {
        let (carrier, wall) = (self.carrier::<f64>(), self.wall::<f64>());
        let angle = self.angle();
        let image: Pcurve<f64> = pick_arc_rim(&carrier, (0.0, angle), &wall, band())
            .expect("pick decided")
            .expect("an arc rim");
        let miss = |t: f64| {
            let p = image.eval(t);
            (wall.eval(p.x, p.y) - carrier.eval(t)).norm()
        };
        const N: usize = 4000;
        #[allow(clippy::cast_precision_loss)]
        let at = |i: usize| angle * i as f64 / N as f64;
        let (mut best, mut arg) = (0.0_f64, 0);
        for i in 0..=N {
            let m = miss(at(i));
            if m > best {
                (best, arg) = (m, i);
            }
        }
        let (mut lo, mut hi) = (at(arg.saturating_sub(1)), at((arg + 1).min(N)));
        for _ in 0..60 {
            let (m1, m2) = (lo + (hi - lo) * 0.382, lo + (hi - lo) * 0.618);
            if miss(m1) < miss(m2) {
                lo = m1;
            } else {
                hi = m2;
            }
        }
        best.max(miss(0.5 * (lo + hi)))
    }

    /// The rounding the sampled miss carries: an `f64` read of a point
    /// at the radius's scale, a few units in the last place of it,
    /// amplified by the map's Möbius skew `max(λ, 1/λ)`. The oracle
    /// re-derives the dumped rows exactly, without it.
    pub(super) fn eval_rounding(&self) -> f64 {
        let skew = self
            .ends
            .windows(2)
            .map(|e| (e[0] / e[1]).sqrt().max((e[1] / e[0]).sqrt()))
            .fold(1.0_f64, f64::max);
        16.0 * f64::EPSILON * self.radius * skew
    }
}

const R: f64 = 0.005;

fn assert_holds<T: Decide + Bounds>(row: &ArcRow, label: &str) -> f64 {
    match row.certify::<T>() {
        Verdict::Certified { envelope } => {
            let miss = row.sampled_miss();
            assert!(
                envelope >= miss - row.eval_rounding(),
                "{label}: envelope {envelope:e} under the sampled miss {miss:e} ({row:?})"
            );
            envelope
        }
        other => panic!("{label}: expected a certificate, got {other:?} ({row:?})"),
    }
}

/// The reviewer's one-span quarter at weights `(1, 1 + δ, 2)`, with `δ`
/// a tenth of the band in metres, and the same at one end weight. The
/// miss is radial at the middle and its sup is `0.27·R·δ_w`.
#[test]
fn a_near_circle_quarter_pays_its_weight() {
    let d = zero() / (10.0 * R);
    for (label, row) in [
        (
            "held ends (1, 1+δ, 2)",
            ArcRow::quarter(R, [1.0, 1.0 + d, 2.0]),
        ),
        (
            "one end weight",
            ArcRow::quarter(R, [1.0, core::f64::consts::FRAC_1_SQRT_2 + d, 1.0]),
        ),
    ] {
        let env = assert_holds::<f64>(&row, label);
        let miss = row.sampled_miss();
        assert!(miss > 0.0, "{label}: the row is off the circle");
        assert!(
            env <= 2.0 * miss + 1e-15,
            "{label}: envelope {env:e} far over the miss {miss:e}"
        );
        assert_holds::<Interval>(&row, label);
    }
}

/// The reviewer's two-span half at ends `(1, 2, 1)`, the second span's
/// weight off by `δ`: each sub-arc's margin is paid, not the first's.
#[test]
fn a_near_circle_second_span_pays_its_weight() {
    let d = zero() / (10.0 * R);
    let row = ArcRow::half(R, [1.0, 1.0, 2.0, 1.0 + d, 1.0]);
    assert_holds::<f64>(&row, "half, second span off");
    assert_holds::<Interval>(&row, "half, second span off");
}

/// Sub-arcs past 129°, where the radius alone under-reads the miss: a
/// 150° sub-arc costs 2.27·R·δ at its middle, a 170° one 9.6·R·δ.
#[test]
fn a_wide_sub_arc_pays_past_the_radius() {
    for deg in [150.0_f64, 170.0] {
        let h = deg.to_radians();
        let c = (h / 2.0).cos();
        let d = zero() / (40.0 * R);
        let row = ArcRow::uniform(R, h, &[1.0, 1.0], &[c + d]);
        let env = assert_holds::<f64>(&row, "wide sub-arc");
        assert!(
            row.sampled_miss() > R * d,
            "{deg}°: the miss is over the radius's reading"
        );
        assert!(env > R * d, "{deg}°: the envelope reads past the radius");
        assert_holds::<Interval>(&row, "wide sub-arc");
    }
}

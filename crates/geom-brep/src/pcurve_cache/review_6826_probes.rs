//! Review probes for PR 6826 (`read_net_bound`): a counterexample search
//! of the bounded read against an exact distance on all five analytic
//! charts, with narrow excursions between the samples beside a cone's
//! apex, through a sphere's poles and across a torus's inner equator.
//!
//! Each draw's exact supremum of the net's distance from the chart is
//! taken by a dense walk refined by golden section per knot span. The
//! probe then asserts the two directions:
//! - `Ok` never with a supremum past the band's escalation (unsound);
//! - `Off` never with a supremum under the band's zero (a false Off).
//!
//! Tallies (certified in band, off in band, lever past the band) are
//! printed, not asserted.

test_utils::gated_to![
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/edge_nurbs.rs",
];

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Interval, Point3, Real, Vec3};
use test_utils::fuzz;

use super::spline_door_rows::band;
use crate::{IncidenceRead, PcurveCertifyError};

const ALPHA: f64 = 0.3;
const BIG: f64 = 2.0;
const SMALL: f64 = 0.5;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Chart {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

impl Chart {
    const ALL: [Self; 5] = [
        Self::Plane,
        Self::Cylinder,
        Self::Cone,
        Self::Sphere,
        Self::Torus,
    ];

    fn surface(self) -> Surface<f64> {
        let (o, z, x) = (Point3::origin(), Vec3::unit_z(), Vec3::unit_x());
        match self {
            Self::Plane => Surface::Plane {
                origin: o,
                normal: z,
                u_ref: x,
            },
            Self::Cylinder => Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            Self::Cone => Surface::Cone {
                apex: o,
                axis: z,
                half_angle: ALPHA,
                u_ref: x,
            },
            Self::Sphere => Surface::Sphere {
                center: o,
                axis: z,
                radius: BIG,
                u_ref: x,
            },
            Self::Torus => Surface::Torus {
                center: o,
                axis: z,
                major_radius: BIG,
                minor_radius: SMALL,
                u_ref: x,
            },
        }
    }

    /// The exact distance of `p` from the chart (the cone double).
    fn distance(self, p: Point3<f64>) -> f64 {
        let rho = p.x.hypot(p.y);
        match self {
            Self::Plane => p.z.abs(),
            Self::Cylinder => (rho - 1.0).abs(),
            Self::Cone => {
                let (s, c) = ALPHA.sin_cos();
                let ray = |cz: f64| {
                    // The ray (s, cz)·σ, σ ≥ 0, in the meridian half-plane.
                    let sigma = (rho * s + p.z * cz).max(0.0);
                    (rho - s * sigma).hypot(p.z - cz * sigma)
                };
                ray(c).min(ray(-c))
            }
            Self::Sphere => (p.x.hypot(p.y).hypot(p.z) - BIG).abs(),
            Self::Torus => ((rho - BIG).hypot(p.z) - SMALL).abs(),
        }
    }

    /// A point on the chart along a base path at `s ∈ [0, 1]` of length
    /// scale `len`, and the chart's unit normal there. The paths pass the
    /// charts' hard places: a cone's ruling through the apex (onto the
    /// other nappe), a sphere's great circle through the north pole, a
    /// torus's tube circle across the inner equator.
    fn path(self, s: f64, len: f64, phase: f64) -> (Point3<f64>, Vec3<f64>) {
        let a = (s - 0.5) * len;
        match self {
            Self::Plane => {
                let (dx, dy) = phase.sin_cos();
                (
                    Point3::new(0.1 + dx * a, 0.2 + dy * a + 0.3 * a * a, 0.0),
                    Vec3::unit_z(),
                )
            }
            Self::Cylinder => {
                let u = phase + a;
                let (sn, cs) = u.sin_cos();
                (Point3::new(cs, sn, 0.7 * a), Vec3::new(cs, sn, 0.0))
            }
            Self::Cone => {
                let (sa, ca) = ALPHA.sin_cos();
                let (sp, cp) = phase.sin_cos();
                // The ruling σ ↦ (s σ cosφ, s σ sinφ, c σ), σ ∈ R: through the apex.
                let sigma = a;
                let n = if sigma >= 0.0 {
                    Vec3::new(ca * cp, ca * sp, -sa)
                } else {
                    Vec3::new(-ca * cp, -ca * sp, -sa)
                };
                (Point3::new(sa * sigma * cp, sa * sigma * sp, ca * sigma), n)
            }
            Self::Sphere => {
                let (sp, cp) = phase.sin_cos();
                // The great circle through the pole: θ = π/2 + a.
                let th = std::f64::consts::FRAC_PI_2 + a;
                let (st, ct) = th.sin_cos();
                let n = Vec3::new(ct * cp, ct * sp, st);
                (Point3::origin() + n * BIG, n)
            }
            Self::Torus => {
                let (sp, cp) = phase.sin_cos();
                // The tube circle across the inner equator: ψ = π + a.
                let psi = std::f64::consts::PI + a;
                let (sq, cq) = psi.sin_cos();
                let radial = Vec3::new(cp, sp, 0.0);
                let n = radial * cq + Vec3::unit_z() * sq;
                (Point3::origin() + radial * BIG + n * SMALL, n)
            }
        }
    }
}

/// A drawn net: knots, controls, weights, degree, window.
#[derive(Clone, Debug)]
struct Draw {
    chart: Chart,
    degree: usize,
    knots: Vec<f64>,
    control: Vec<Point3<f64>>,
    weights: Vec<f64>,
    window: (f64, f64),
}

impl Draw {
    fn net(&self) -> NurbsCurve3<f64> {
        NurbsCurve3::new(
            KnotVector::clamped(self.knots.clone(), self.degree).unwrap(),
            self.control.clone(),
            self.weights.clone(),
        )
        .unwrap()
    }

    /// The exact supremum of the distance over the window, and where.
    fn sup(&self) -> (f64, f64) {
        let net = self.net();
        let f = |t: f64| self.chart.distance(net.eval(t));
        let mut cuts: Vec<f64> = self
            .knots
            .iter()
            .copied()
            .filter(|&k| k > self.window.0 && k < self.window.1)
            .collect();
        cuts.push(self.window.0);
        cuts.push(self.window.1);
        cuts.sort_by(f64::total_cmp);
        cuts.dedup();
        let mut best = (0.0f64, self.window.0);
        let g = 0.5 * (5f64.sqrt() - 1.0);
        for w in cuts.windows(2) {
            let (lo, hi) = (w[0], w[1]);
            let n = 1500u32;
            let at = |i: u32| lo + (hi - lo) * f64::from(i) / f64::from(n);
            let vals: Vec<f64> = (0..=n).map(|i| f(at(i))).collect();
            // Every local maximum of the walk, the best eight refined.
            let mut peaks: Vec<(f64, u32)> = (0..=n)
                .filter(|&i| {
                    let i = i as usize;
                    (i == 0 || vals[i] >= vals[i - 1])
                        && (i == n as usize || vals[i] >= vals[i + 1])
                })
                .map(|i| (vals[i as usize], i))
                .collect();
            peaks.sort_by(|a, b| b.0.total_cmp(&a.0));
            for &(v, i) in peaks.iter().take(8) {
                if v > best.0 {
                    best = (v, at(i));
                }
                let (mut a, mut b) = (at(i.saturating_sub(1)), at((i + 1).min(n)));
                for _ in 0..100 {
                    let c = b - g * (b - a);
                    let d = a + g * (b - a);
                    if f(c) > f(d) {
                        b = d;
                    } else {
                        a = c;
                    }
                }
                let m = 0.5 * (a + b);
                if f(m) > best.0 {
                    best = (f(m), m);
                }
            }
        }
        best
    }
}

/// A drawn net along the chart's hard path: `count` controls on the
/// chart at their Greville abscissae over a path of length scale `len`,
/// then up to three controls pushed along the normal (or a random
/// direction) by a log-uniform height, with narrow knot spans about them.
fn draw(g: &mut fuzz::Rng, chart: Chart) -> Draw {
    let degree = 1 + g.below(3);
    let len = g.log_uniform(1e-4, 2.0);
    let phase = g.range(0.0, std::f64::consts::TAU);
    let mut interior: Vec<f64> = (0..g.below(40))
        .map(|_| g.range(1e-6, 1.0 - 1e-6))
        .collect();
    // A narrow cluster between the samples (between i/64 and (i+1)/64).
    let at = (g.below(64) as f64 + g.range(0.2, 0.8)) / 64.0;
    let w = g.log_uniform(1e-6, 2e-3);
    interior.extend([at - w, at, at + w].iter().filter(|&&k| k > 0.0 && k < 1.0));
    interior.sort_by(f64::total_cmp);
    interior.dedup();
    let mut knots = vec![0.0; degree + 1];
    knots.extend(&interior);
    knots.extend(vec![1.0; degree + 1]);
    let count = knots.len() - degree - 1;
    #[allow(clippy::cast_precision_loss)]
    let greville: Vec<f64> = (0..count)
        .map(|i| knots[i + 1..=i + degree].iter().sum::<f64>() / degree as f64)
        .collect();
    let mut control: Vec<Point3<f64>> = greville
        .iter()
        .map(|&s| chart.path(s, len, phase).0)
        .collect();
    let eps = band().zero();
    for _ in 0..1 + g.below(3) {
        // Bias toward the narrow cluster.
        let j = if g.below(2) == 0 {
            greville
                .iter()
                .enumerate()
                .min_by(|a, b| (a.1 - at).abs().total_cmp(&(b.1 - at).abs()))
                .unwrap()
                .0
        } else {
            g.below(count)
        };
        let h = g.log_uniform(1e-3 * eps, 1e6 * eps) * if g.below(2) == 0 { 1.0 } else { -1.0 };
        let dir = if g.below(4) == 0 {
            let v = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
            v / v.norm()
        } else {
            chart.path(greville[j], len, phase).1
        };
        control[j] = control[j] + dir * h;
    }
    let weights = if g.below(3) == 0 {
        (0..count).map(|_| g.range(0.5, 2.0)).collect()
    } else {
        vec![1.0; count]
    };
    let window = if g.below(2) == 0 {
        (0.0, 1.0)
    } else {
        let (x, y) = (g.unit(), g.unit());
        (x.min(y), x.max(y).max(x.min(y) + 1e-3).min(1.0))
    };
    Draw {
        chart,
        degree,
        knots,
        control,
        weights,
        window,
    }
}

type ReadFn = fn(&Surface<f64>, &NurbsCurve3<f64>, (f64, f64)) -> Result<(), PcurveCertifyError>;

macro_rules! read_at {
    ($t:ty) => {
        |surface: &Surface<f64>, net: &NurbsCurve3<f64>, (t0, t1): (f64, f64)| {
            let c = <$t as Real>::from_f64;
            let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(c)));
            let surface = surface.map_scalar(c);
            let span = (c(t0), c(t1));
            super::read_net_bound(&carrier, span, &surface, band(), || {
                super::limb2_offset_sup(&carrier, span, &surface, band())
            })
        }
    };
}

const READS: [(&str, ReadFn); 2] = [("f64", read_at!(f64)), ("Interval", read_at!(Interval))];

#[derive(Debug, Default)]
struct Tally {
    draws: usize,
    ok: usize,
    off: usize,
    lever: usize,
    other: usize,
    unsound: Vec<String>,
    false_off: Vec<String>,
    ok_in_band: usize,
    off_in_band: usize,
    lever_past: usize,
    lever_on: usize,
    poisoned: usize,
    poisoned_in_band: usize,
}

fn run(chart: Chart, g: &mut fuzz::Rng, n: usize, t: &mut Tally) {
    let b = band();
    for _ in 0..n {
        let d = draw(g, chart);
        let (sup, at) = d.sup();
        for (name, read) in READS {
            let got = read(&chart.surface(), &d.net(), d.window);
            t.draws += 1;
            let what = match &got {
                Ok(()) => "ok",
                Err(PcurveCertifyError::Incidence(r)) => match r.read() {
                    IncidenceRead::Off(_) => "off",
                    IncidenceRead::Undecided(c) if c.margin.is_invalid() => "poisoned",
                    IncidenceRead::Undecided(_) => "lever",
                    IncidenceRead::AtPose(_) => "other",
                },
                Err(_) => "other",
            };
            let line = || {
                format!(
                    "{chart:?} {name} sup {sup:e} at t={at} (zero {:e}, esc {:e}) -> {got:?}; draw {d:?}",
                    b.zero(),
                    b.escalate()
                )
            };
            match what {
                "ok" => {
                    t.ok += 1;
                    if sup > b.escalate() * (1.0 + 1e-6) {
                        t.unsound.push(line());
                    } else if sup > b.zero() * 1.01 {
                        t.ok_in_band += 1;
                    }
                }
                "off" => {
                    t.off += 1;
                    if sup < b.zero() * 0.99 {
                        t.false_off.push(line());
                    } else if sup < b.escalate() * 0.99 {
                        t.off_in_band += 1;
                        eprintln!("OFF-IN-BAND {}", line());
                    }
                }
                "lever" => {
                    t.lever += 1;
                    if sup > 4.0 * b.escalate() {
                        t.lever_past += 1;
                    }
                    if sup < 0.01 * b.zero() {
                        t.lever_on += 1;
                    }
                }
                "poisoned" => {
                    t.lever += 1;
                    t.poisoned += 1;
                    if sup < b.escalate() {
                        t.poisoned_in_band += 1;
                    }
                    if t.poisoned <= 2 {
                        eprintln!("POISONED {}", line());
                    }
                }
                _ => {
                    t.other += 1;
                    eprintln!("OTHER {}", line());
                }
            }
        }
    }
}

#[test]
fn review_6826_no_certificate_past_the_band_and_no_false_off() {
    let n: usize = std::env::var("REVIEW6826_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let mut g = fuzz::start("review_6826_sup");
    let mut bad = Vec::new();
    let only = std::env::var("REVIEW6826_CHART").ok();
    for chart in Chart::ALL {
        if only.as_deref().is_some_and(|o| o != format!("{chart:?}")) {
            continue;
        }
        let mut t = Tally::default();
        run(chart, &mut g, n, &mut t);
        eprintln!(
            "{chart:?}: draws {} ok {} off {} lever {} other {} | ok-in-band {} off-in-band {} \
             lever-past-4esc {} lever-on {} poisoned {} (on or in band {}) | UNSOUND {} FALSE-OFF {}",
            t.draws,
            t.ok,
            t.off,
            t.lever,
            t.other,
            t.ok_in_band,
            t.off_in_band,
            t.lever_past,
            t.lever_on,
            t.poisoned,
            t.poisoned_in_band,
            t.unsound.len(),
            t.false_off.len()
        );
        for l in t.unsound.iter().chain(&t.false_off).take(4) {
            eprintln!("  {l}");
        }
        bad.extend(t.unsound);
        bad.extend(t.false_off);
    }
    assert!(
        bad.is_empty(),
        "{} bad reads; {}",
        bad.len(),
        fuzz::replay()
    );
}

/// A ruling of the cone through its apex onto the other nappe, on the
/// (double) chart exactly, read over a window whose middle lies on one
/// nappe: does the composite read the far nappe's half off?
#[test]
fn review_6826_a_ruling_through_the_apex() {
    let (sa, ca) = ALPHA.sin_cos();
    for (lo, hi) in [(-0.2, 1.0), (-1.0, 1.0), (-1.0, 0.2), (-0.01, 1.0)] {
        let at = |s: f64| Point3::new(sa * s, 0.0, ca * s);
        let net = NurbsCurve3::new(
            KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap(),
            vec![at(lo), at(hi)],
            vec![1.0, 1.0],
        )
        .unwrap();
        for (name, read) in READS {
            let got = read(&Chart::Cone.surface(), &net, (lo, hi));
            eprintln!("apex ruling [{lo}, {hi}] {name}: {got:?}");
        }
    }
}

/// `NoOffsetBound` at `Interval`: `net_bound_refusal`'s straddle to `+∞`
/// is read in the scalar, whose `+∞` poisons the margin, so the read is
/// definite (`IncidenceRead::is_definite`) and ends as a defect. The
/// lane's `a_bound_that_does_not_read_ends_in_the_lever` runs at `f64`
/// only. Red by design.
#[test]
fn review_6826_no_offset_bound_at_interval_ends_in_the_lever() {
    use crate::edge_nurbs::AnalyticRung3Refusal as R;
    let surface = Chart::Cone.surface().map_scalar(Interval::from_f64);
    let (sa, ca) = ALPHA.sin_cos();
    let net = NurbsCurve3::new(
        KnotVector::clamped(vec![-1.0, -1.0, 1.0, 1.0], 1).unwrap(),
        vec![Point3::new(-sa, 0.0, -ca), Point3::new(sa, 0.0, ca)],
        vec![1.0, 1.0],
    )
    .unwrap();
    let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(Interval::from_f64)));
    let got = super::net_bound_refusal(
        R::NoOffsetBound {
            operand: surface.kind(),
            why: "a probe's",
        },
        &surface,
        &carrier,
        band(),
    );
    let read = match &got {
        PcurveCertifyError::Incidence(r) => Some(r.read()),
        _ => None,
    };
    assert!(
        matches!(read, Some(IncidenceRead::Undecided(c)) if !c.margin.is_invalid()),
        "NoOffsetBound at Interval ends in the lever with a margin: {got:?}"
    );
    // And the whole read of the ruling through the apex, on the chart.
    let span = (Interval::from_f64(-1.0), Interval::from_f64(1.0));
    let whole = super::read_net_bound(&carrier, span, &surface, band(), || {
        super::limb2_offset_sup(&carrier, span, &surface, band())
    });
    let definite =
        matches!(&whole, Err(PcurveCertifyError::Incidence(r)) if r.read().is_definite());
    assert!(
        !definite,
        "a ruling on the cone through its apex reads definite (a defect ending): {whole:?}"
    );
}

/// The search's two "in-band" Offs (seed 2, before the oracle walked
/// densely): a net along a cone's ruling past the apex, whose distance
/// peaks in a narrow kink beside the apex at `3.28e-8`, past the band at
/// ε 1e-9. The sampled read's `Off` (`2.97e-8` at `t = 1/2`) is right;
/// the first oracle missed the kink.
#[test]
fn review_6826_the_sampled_reads_in_band_off_beside_the_apex() {
    let d = Draw {
        chart: Chart::Cone,
        degree: 3,
        knots: vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.570_479_133_683_228_7,
            0.570_498_281_354_917_6,
            0.570_517_429_026_606_6,
            0.658_801_287_841_901_3,
            0.921_530_276_901_245_4,
            1.0,
            1.0,
            1.0,
            1.0,
        ],
        control: vec![
            Point3::new(
                -0.001_372_801_398_689_696_6,
                0.000_678_521_296_401_320_9,
                -0.004_950_377_030_488_926,
            ),
            Point3::new(
                -0.000_850_698_363_593_947_9,
                0.000_420_466_468_830_223,
                -0.003_067_652_497_316_474_6,
            ),
            Point3::new(
                -0.000_328_577_804_531_215_4,
                0.000_162_403_029_857_113_52,
                -0.001_184_864_772_014_612_7,
            ),
            Point3::new(
                0.000_193_540_911_243_972_9,
                -9.571_647_622_423_621e-5,
                0.000_698_001_985_665_741_7,
            ),
            Point3::new(
                0.000_274_392_796_341_376_45,
                -0.000_135_621_478_878_474_1,
                0.000_989_471_454_236_923,
            ),
            Point3::new(
                0.000_595_657_605_988_599_4,
                -0.000_294_409_935_342_765_57,
                0.002_147_965_272_716_412,
            ),
            Point3::new(
                0.000_988_718_318_416_776_6,
                -0.000_488_684_259_666_554_7,
                0.003_565_366_446_334_185_7,
            ),
            Point3::new(
                0.001_300_985_834_939_854_4,
                -0.000_643_025_710_904_507_9,
                0.004_691_407_220_610_993,
            ),
            Point3::new(
                0.001_372_801_398_689_696_6,
                -0.000_678_521_296_401_320_9,
                0.004_950_377_030_488_926,
            ),
        ],
        weights: vec![1.0; 9],
        window: (0.0, 1.0),
    };
    let net = d.net();
    let worst = (0..=64)
        .map(|i| {
            let t = f64::from(i) / 64.0;
            (Chart::Cone.distance(net.eval(t)), t)
        })
        .fold((0.0, 0.0), |a, b| if b.0 > a.0 { b } else { a });
    let (sup, at) = d.sup();
    eprintln!("exact sup {sup:e} at {at}; worst sample {worst:?}");
    for (name, read) in READS {
        let got = read(&Chart::Cone.surface(), &net, d.window);
        eprintln!("{name}: {got:?}");
        let c = <f64 as Real>::from_f64;
        let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(c)));
        let sampled = super::read_incidence(&carrier, 0.0, 1.0, &Chart::Cone.surface(), band());
        eprintln!("  sampled alone (f64): {sampled:?}");
    }
    assert!(sup > band().escalate(), "the oracle reads it past the band");
}

/// The witness polyline on the plane `z = 0` along `y = 0.3`, its vertex
/// at `t = 1/128` pushed `push` (a vector), stated by the straight
/// ruling as an `IsoLine`.
macro_rules! plane_certify {
    ($t:ty, $push:expr, $general:expr) => {{
        use crate::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, NurbsLane, SurfaceKey};
        use geom_core::{Point2, Vec2};
        let c = <$t as Real>::from_f64;
        let push: Vec3<f64> = $push;
        let (a, b) = (1.0 / 256.0, 1.0 / 128.0);
        let ts = [0.0, b - a, b, b + a, 1.0];
        let control: Vec<Point3<f64>> = ts
            .iter()
            .map(|&t| {
                let p = Point3::new(t, 0.3, 0.0);
                if t == b { p + push } else { p }
            })
            .collect();
        let net = NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, b - a, b, b + a, 1.0, 1.0], 1).unwrap(),
            control,
            vec![1.0; 5],
        )
        .unwrap();
        let mut table: slotmap::SlotMap<SurfaceKey, Surface<$t>> = slotmap::SlotMap::with_key();
        let key = table.insert(Chart::Plane.surface().map_scalar(c));
        let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(c)));
        let (start, end) = (carrier.eval(c(0.0)), carrier.eval(c(1.0)));
        let image = if $general {
            let line = geom::NurbsCurve2::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point2::new(0.0, 0.3), Point2::new(1.0, 0.3)],
                vec![1.0, 1.0],
            )
            .unwrap();
            super::Pcurve::General(Arc::new(line.map_scalar(c)))
        } else {
            super::Pcurve::IsoLine {
                p0: Point2::new(c(0.0), c(0.3)),
                pl: Vec2::new(c(1.0), c(0.0)),
            }
        };
        EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::chart_image(key, image),
                carrier,
                param_start: c(0.0),
                param_end: c(1.0),
            },
            start,
            end,
            |k| table.get(k).cloned(),
            geom_core::test_support::fixture_lever(<$t as Real>::one()),
            band(),
            NurbsLane::certified(),
        )
        .map(|_| ())
    }};
}

/// Claim 7, the new P1 reproduced: the witness pushed 1 mm (and 4× the
/// escalation) **within** the plane, stated by the straight ruling,
/// certifies at both scalars and through both images. The same push
/// along the normal ends off (this PR's fix). Red by design while the P1
/// is open.
#[test]
fn review_6826_an_in_plane_wiggle_between_the_samples_certifies() {
    let mut certified = Vec::new();
    for h in [1e-3, 4.0 * band().escalate()] {
        for general in [false, true] {
            let inplane = [
                ("f64", plane_certify!(f64, Vec3::new(0.0, h, 0.0), general)),
                (
                    "Interval",
                    plane_certify!(Interval, Vec3::new(0.0, h, 0.0), general),
                ),
            ];
            let normal = [
                ("f64", plane_certify!(f64, Vec3::new(0.0, 0.0, h), general)),
                (
                    "Interval",
                    plane_certify!(Interval, Vec3::new(0.0, 0.0, h), general),
                ),
            ];
            for ((name, got), (_, off)) in inplane.into_iter().zip(normal) {
                eprintln!(
                    "h {h:e} general {general} {name}: in-plane {got:?}; normal {:?}",
                    off.as_ref()
                        .err()
                        .and_then(crate::CertifyError::incidence_read)
                );
                assert!(
                    matches!(
                        off.as_ref()
                            .err()
                            .and_then(crate::CertifyError::incidence_read),
                        Some(IncidenceRead::Off(_))
                    ),
                    "the normal push ends off"
                );
                if got.is_ok() {
                    certified.push(format!("h {h:e} general {general} {name}"));
                }
            }
        }
    }
    assert!(
        certified.is_empty(),
        "in-plane wiggles certified: {certified:#?}"
    );
}

/// Claim 3: the endings of the witness' refusal (off) and of an in-band
/// bump (lever) at a build, at rest and at import, and the poisoned
/// `NoOffsetBound` read at `Interval` (a defect story for an in-band
/// input).
#[test]
fn review_6826_endings_at_the_three_doors() {
    use crate::recourse::{ReadAt, Reading};
    let doors = [
        ("build", ReadAt::Run(Reading::Build)),
        ("rest", ReadAt::Run(Reading::AtRest)),
        (
            "import",
            ReadAt::File(geom_core::predicate::FileCoincidence::new(1e-6)),
        ),
    ];
    let off = plane_certify!(f64, Vec3::new(0.0, 0.0, 1e-3), false).unwrap_err();
    let mid = 0.5 * (band().zero() + band().escalate());
    let lever = plane_certify!(f64, Vec3::new(0.0, 0.0, mid), false).unwrap_err();
    for (what, e) in [("off", &off), ("lever", &lever)] {
        for (door, at) in doors {
            eprintln!("{what} at {door}: {:?}", e.ending(at));
        }
    }
    // The ruling through the cone's apex, on the chart, at Interval.
    let (sa, ca) = ALPHA.sin_cos();
    let net = NurbsCurve3::new(
        KnotVector::clamped(vec![-1.0, -1.0, 1.0, 1.0], 1).unwrap(),
        vec![Point3::new(-sa, 0.0, -ca), Point3::new(sa, 0.0, ca)],
        vec![1.0, 1.0],
    )
    .unwrap();
    if let Err(PcurveCertifyError::Incidence(r)) =
        (READS[1].1)(&Chart::Cone.surface(), &net, (-1.0, 1.0))
    {
        for (door, at) in doors {
            eprintln!(
                "apex ruling (on the chart) at Interval, at {door}: {}",
                r.read().ending(at)
            );
        }
    }
}

/// Claim 6: the bounded read's cost on a degree-3 net with forty random
/// interior knots along a ruling, its peak at 0, 0.9, 1.1 and 3 × the
/// band's zero, against the sampled read alone. Run in release; prints
/// the median of seven per cell.
#[test]
fn review_6826_cost_of_a_knot_rich_read() {
    let mut g = fuzz::pinned("review_6826_cost", 7);
    let mut interior: Vec<f64> = (0..40).map(|_| g.range(1e-3, 1.0 - 1e-3)).collect();
    interior.sort_by(f64::total_cmp);
    let degree = 3;
    let mut knots = vec![0.0; degree + 1];
    knots.extend(&interior);
    knots.extend(vec![1.0; degree + 1]);
    let count = knots.len() - degree - 1;
    #[allow(clippy::cast_precision_loss)]
    let greville: Vec<f64> = (0..count)
        .map(|i| knots[i + 1..=i + degree].iter().sum::<f64>() / degree as f64)
        .collect();
    let (sa, ca) = ALPHA.sin_cos();
    let charts: [(Chart, [f64; 3], [f64; 3], [f64; 3]); 3] = [
        (
            Chart::Plane,
            [0.0, 0.3, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
        ),
        (
            Chart::Cylinder,
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        ),
        (Chart::Cone, [sa, 0.0, ca], [sa, 0.0, ca], [ca, 0.0, -sa]),
    ];
    let bumped = count / 2;
    let median = |mut v: Vec<f64>| {
        v.sort_by(f64::total_cmp);
        v[v.len() / 2]
    };
    for (chart, foot, dir, n) in charts {
        for k in [0.0, 0.9, 1.1, 3.0] {
            // The bumped control's basis peak is at most 1, so its height
            // is k·zero over that peak, read off a unit net first.
            let make = |h: f64| {
                let control: Vec<Point3<f64>> = greville
                    .iter()
                    .enumerate()
                    .map(|(i, &t)| {
                        let h = if i == bumped { h } else { 0.0 };
                        Point3::new(
                            foot[0] + dir[0] * t + n[0] * h,
                            foot[1] + dir[1] * t + n[1] * h,
                            foot[2] + dir[2] * t + n[2] * h,
                        )
                    })
                    .collect();
                NurbsCurve3::new(
                    KnotVector::clamped(knots.clone(), degree).unwrap(),
                    control,
                    vec![1.0; count],
                )
                .unwrap()
            };
            let unit = make(1.0);
            let peak = (0..=1 << 14)
                .map(|i| chart.distance(unit.eval(f64::from(i) / f64::from(1u32 << 14))))
                .fold(0.0, f64::max);
            for (name, read) in READS {
                let mut bounded = Vec::new();
                let mut sampled = Vec::new();
                let mut last = None;
                for rep in 0..7 {
                    // A distinct window per repetition and scalar, so limb
                    // 2's memo never serves a round: each read is cold.
                    let net = make(k * band().zero() / peak);
                    let hi = 1.0 - f64::from(rep + if name == "f64" { 0 } else { 100 }) * 1e-12;
                    let (hits, _) = super::limb2_memo::counts();
                    let clock = std::time::Instant::now();
                    last = Some(read(&chart.surface(), &net, (0.0, hi)));
                    bounded.push(clock.elapsed().as_secs_f64() * 1e3);
                    assert_eq!(
                        super::limb2_memo::counts().0,
                        hits,
                        "a cold read hit the memo"
                    );
                    let carrier = Curve3::Nurbs(Arc::new(net.clone()));
                    let clock = std::time::Instant::now();
                    let _ = super::read_incidence(&carrier, 0.0, 1.0, &chart.surface(), band());
                    sampled.push(clock.elapsed().as_secs_f64() * 1e3);
                }
                let what = match last.unwrap() {
                    Ok(()) => "ok".to_string(),
                    Err(PcurveCertifyError::Incidence(r)) => {
                        format!("{:?}", r.read()).chars().take(12).collect()
                    }
                    Err(e) => format!("{e:?}").chars().take(20).collect(),
                };
                eprintln!(
                    "COST {chart:?} {k}×zero {name}: bounded (cold) {:.3} ms, sampled {:.3} ms ({what})",
                    median(bounded),
                    median(sampled)
                );
            }
        }
    }
}

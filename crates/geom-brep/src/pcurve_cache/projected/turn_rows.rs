//! **Check 2's period gate agrees with the spelling it replaced** (the
//! PRED charter's agreement test, `geom_core::period`'s turn rule): the
//! sweep's headroom `(τ − extent)` at the chart's sup arm, `Positive`
//! and `Zero` admitted, `Negative` refused. The old spelling is an
//! oracle here, bit for bit and generic over the scalar as the shipped
//! code was, run against the shipped gate ([`ChannelRate::turn`]) on a
//! full turn and `τ ± k·e/rate` for `k ∈ {0.5, 3, 20}` at both band edges
//! `e`, at an exact rate (a cylinder's azimuth, a torus's tube) and at
//! an eccentric bounded one (inf `1`, sup `16`), each multiple read
//! against both rates; and the inf arms per chart kind.

use core::f64::consts::TAU;

use geom::Surface;
use geom_core::k_stats::decide;
use geom_core::period::{Bounded, Exact, Extent, Turn};
use geom_core::{Band, Decide, InfSpeed, Margin, Point3, Sign, SupSpeed, Tol, Vec3};

use super::ChannelRate;

/// What the gate reached.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Admitted,
    Exceeded,
    Escalated,
}

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness band")
}

/// **The oracle**: check 2's gate as it was spelled, at the sup arm.
fn old_gate<T: Decide>(extent: T, arm: SupSpeed<T>, band: Band) -> Outcome {
    match decide(
        "pcurve_azimuth_period",
        Margin::metered_sup(T::tau() - extent, arm),
        band,
    ) {
        Ok(Sign::Positive | Sign::Zero) => Outcome::Admitted,
        Ok(Sign::Negative) => Outcome::Exceeded,
        Err(_) => Outcome::Escalated,
    }
}

fn new_gate(extent: f64, rate: ChannelRate<f64>, band: Band) -> Outcome {
    let extent = Extent::hull([0.0, extent]).expect("two values");
    match rate.turn(extent, band) {
        Ok(Turn::Short(_) | Turn::Whole(_)) => Outcome::Admitted,
        Ok(Turn::Over(_)) => Outcome::Exceeded,
        Err(_) => Outcome::Escalated,
    }
}

/// The multiples `k·e/ε`, both senses (positive is past a turn).
fn multiples(band: Band) -> Vec<f64> {
    let mut out = vec![0.0];
    for edge in [band.zero(), band.escalate()] {
        for k in [0.5, 3.0, 20.0] {
            out.extend([
                k * (edge / band.zero()).round(),
                -k * (edge / band.zero()).round(),
            ]);
        }
    }
    out
}

/// **At an exact rate the gate is the old one, bit for bit.**
#[test]
fn an_exact_channel_agrees_with_the_old_gate() {
    let band = band();
    let eps = band.zero();
    for r in [2.0, 0.3] {
        for k in multiples(band) {
            let extent = TAU + k * eps / r;
            assert_eq!(
                new_gate(extent, ChannelRate::Exact(Exact::new(r)), band),
                old_gate(extent, SupSpeed::new(r), band),
                "r = {r}, k = {k}"
            );
        }
    }
}

/// **At an eccentric bounded rate the gate agrees with the old one, but
/// for the listed deltas.** The old gate read only the sup.
///
/// **The expected deltas**, each listed by the rate its multiple is of:
/// - where the inf reads the defect in the band (`±3`, `±5` of the inf's
///   length; `±20`, `±30` of the sup's), the old gate decided it at the
///   sup — refused past a turn, admitted short of one — and the gate
///   now escalates: neither reading is a sound verdict past the band;
/// - where the inf reads an overshoot within the zero band and the sup
///   past it (`0.9` of the inf's length), the old gate refused, and the
///   gate now escalates on the straddle. The short twin is admitted, as
///   it was: a headroom the sup reads definitely positive is a sweep
///   short of a turn.
#[test]
fn a_bounded_channel_agrees_with_the_old_gate() {
    let band = band();
    let eps = band.zero();
    let (inf, sup) = (1.0, 16.0);
    let rate = ChannelRate::Bounded(Bounded::new(InfSpeed::new(inf), SupSpeed::new(sup)));
    let deltas = [
        (inf, 3.0),
        (inf, -3.0),
        (inf, 5.0),
        (inf, -5.0),
        (inf, 0.9),
        (sup, 20.0),
        (sup, -20.0),
        (sup, 30.0),
        (sup, -30.0),
    ];
    let mut seen = 0;
    for of in [inf, sup] {
        for k in multiples(band).into_iter().chain([0.9, -0.9]) {
            let extent = TAU + k * eps / of;
            let old = old_gate(extent, SupSpeed::new(sup), band);
            let new = new_gate(extent, rate, band);
            if deltas.contains(&(of, k)) {
                seen += 1;
                assert_ne!(old, Outcome::Escalated, "{of}, {k}: the listed delta");
                assert_eq!(new, Outcome::Escalated, "{of}, {k}: the listed delta");
            } else {
                assert_eq!(new, old, "of {of}, k = {k}: unlisted");
            }
        }
    }
    assert_eq!(seen, deltas.len(), "every listed delta is a case");
}

/// **The inf arm per chart kind** (`azimuth_lever_inf`): a cylinder's
/// radius; a sphere's `r·cos` at the window's widest latitude, zero on
/// the pole; a ring torus's `R − r`, a spindle's zero; a cone's least
/// `|v|·sin α`, zero across the apex.
#[test]
fn the_azimuth_inf_arm_is_a_floor_per_kind() {
    let o = Point3::origin();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let inf = |s: &Surface<f64>, lo: f64, hi: f64| super::super::azimuth_lever_inf(s, lo, hi).get();
    let cylinder = Surface::Cylinder {
        origin: o,
        axis: z,
        radius: 2.0,
        u_ref: x,
    };
    assert_eq!(inf(&cylinder, -5.0, 5.0), 2.0, "a cylinder");
    let sphere = Surface::Sphere {
        center: o,
        radius: 2.0,
        axis: z,
        u_ref: x,
    };
    assert_eq!(
        inf(&sphere, -0.3, 0.1),
        2.0 * 0.3_f64.cos(),
        "a sphere band"
    );
    assert!(
        inf(&sphere, -0.2, core::f64::consts::FRAC_PI_2) < 1e-15,
        "a sphere's pole"
    );
    assert!(inf(&sphere, 0.0, 7.0) < 1e-15, "past the pole");
    let torus = |major_radius, minor_radius| Surface::Torus {
        center: o,
        axis: z,
        major_radius,
        minor_radius,
        u_ref: x,
    };
    assert_eq!(inf(&torus(3.0, 1.0), -1.0, 1.0), 2.0, "a ring torus");
    assert_eq!(inf(&torus(1.0, 3.0), -1.0, 1.0), 0.0, "a spindle torus");
    let cone = Surface::Cone {
        apex: o,
        axis: z,
        half_angle: 0.5,
        u_ref: x,
    };
    assert_eq!(inf(&cone, 1.0, 2.0), 0.5_f64.sin(), "a cone off its apex");
    assert_eq!(
        inf(&cone, -2.0, -1.5),
        1.5 * 0.5_f64.sin(),
        "the other nappe"
    );
    assert_eq!(inf(&cone, -1.0, 2.0), 0.0, "across the apex");
}

/// REVIEW PROBE (PR 5710, claim 3): `azimuth_lever_inf` is a floor on
/// the true azimuth speed over its window, at f64 (a seeded sweep of
/// windows, each sampled densely) and at Interval (point and widened
/// windows), on the sphere, the torus and the cone.
#[test]
fn review_probe_the_inf_arm_is_a_floor_at_f64_and_interval() {
    use geom_core::{Bounds, Interval, Real};
    let o = Point3::origin();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let half = core::f64::consts::FRAC_PI_2;
    let kinds: [(Surface<f64>, fn(f64) -> f64, (f64, f64)); 3] = [
        (
            Surface::Sphere {
                center: o,
                radius: 2.0,
                axis: z,
                u_ref: x,
            },
            |v: f64| 2.0 * v.cos().abs(),
            (-half, half),
        ),
        (
            Surface::Torus {
                center: o,
                axis: z,
                major_radius: 3.0,
                minor_radius: 1.0,
                u_ref: x,
            },
            |v: f64| (3.0 + v.cos()).abs(),
            (-4.0, 4.0),
        ),
        (
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: 0.5,
                u_ref: x,
            },
            |v: f64| v.abs() * 0.5_f64.sin(),
            (-3.0, 3.0),
        ),
    ];
    let mut seed: u64 = 0x5710;
    let mut next = || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for (s, speed, (lo, hi)) in &kinds {
        for _ in 0..400 {
            let a = lo + (hi - lo) * next();
            let b = lo + (hi - lo) * next();
            let (v0, v1) = (a.min(b), a.max(b));
            let inf = super::super::azimuth_lever_inf(s, v0, v1).get();
            let min = (0..=2000)
                .map(|i| speed(v0 + (v1 - v0) * f64::from(i) / 2000.0))
                .fold(f64::INFINITY, f64::min);
            assert!(inf >= 0.0, "{:?} [{v0}, {v1}]: {inf}", s.kind());
            assert!(
                inf <= min * (1.0 + 1e-12) + 1e-15,
                "{:?} [{v0}, {v1}]: inf {inf} > {min}",
                s.kind()
            );
        }
        // Interval: a point window and a widened one.
        let f = Interval::from_f64;
        let si: Surface<Interval> = match *s {
            Surface::Sphere { .. } => Surface::Sphere {
                center: Point3::new(f(0.0), f(0.0), f(0.0)),
                radius: f(2.0),
                axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
                u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
            },
            Surface::Torus { .. } => Surface::Torus {
                center: Point3::new(f(0.0), f(0.0), f(0.0)),
                axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
                major_radius: f(3.0),
                minor_radius: f(1.0),
                u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
            },
            _ => Surface::Cone {
                apex: Point3::new(f(0.0), f(0.0), f(0.0)),
                axis: Vec3::new(f(0.0), f(0.0), f(1.0)),
                half_angle: f(0.5),
                u_ref: Vec3::new(f(1.0), f(0.0), f(0.0)),
            },
        };
        for (v0, v1) in [
            (-0.2, half),
            (0.0, 1.0),
            (1.0, 1.2),
            (-1.0, -0.5),
            (-0.3, 0.3),
        ] {
            for w in [0.0, 1e-6] {
                let lo_v = Interval::from_bounds(v0 - w, v0 + w);
                let hi_v = Interval::from_bounds(v1 - w, v1 + w);
                let inf = super::super::azimuth_lever_inf(&si, lo_v, hi_v).get();
                let min = (0..=2000)
                    .map(|i| speed((v0 - w) + ((v1 + w) - (v0 - w)) * f64::from(i) / 2000.0))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    inf.lo() >= 0.0 && inf.lo() <= min + 1e-15,
                    "{:?} Interval [{v0}, {v1}] w {w}: inf {:?} vs {min}",
                    s.kind(),
                    (inf.lo(), inf.hi())
                );
                assert!(!inf.lo().is_nan(), "not poison");
            }
        }
        let _ = Interval::tau();
    }
}

/// REVIEW PROBE (PR 5710, claim 3): at the eccentric pair, a headroom the
/// sup reads definitely positive is admitted when the inf reads it Zero
/// (the straddle) but escalates when the inf reads it in the band, though
/// both put the sweep definitely short of a turn.
#[test]
fn review_probe_a_sup_positive_headroom_admits_or_escalates_by_the_inf() {
    let band = band();
    let eps = band.zero();
    let rate = ChannelRate::Bounded(Bounded::new(InfSpeed::new(1.0), SupSpeed::new(16.0)));
    let short = |k: f64| new_gate(TAU - k * eps, rate, band);
    // sup reads 16·k·ε: definitely positive (past 10·ε) for every k here.
    assert_eq!(
        short(0.9),
        Outcome::Admitted,
        "inf Zero, sup positive: straddle admitted"
    );
    assert_eq!(
        short(3.0),
        Outcome::Escalated,
        "inf in band, sup positive: escalated"
    );
    assert_eq!(
        short(5.0),
        Outcome::Escalated,
        "inf in band, sup positive: escalated"
    );
}

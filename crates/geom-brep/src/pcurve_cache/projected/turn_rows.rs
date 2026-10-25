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

//! M8-F67 blinded-review probes (unit r1): the typed-margin fold-in
//! attacked where the shipped twins do not reach.
//!
//! - The arc-length lower-bound claim re-derived and EXECUTED for a
//!   RATIONAL carrier and a multi-span non-rational carrier (the
//!   shipped twins only exercise a degree-1 unit-weight segment).
//! - Poison and backwards-span behaviour driven END-TO-END through
//!   the certify gate and both pcurve lanes (fitted and iso), not
//!   just the `param_rate_gate` unit.
//! - The pcurve gate's own reparametrization invariance, sampled at
//!   `Probe` through the iso lane.
//!
//! **CI EXECUTES THIS SUITE.** It is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`, so every assertion below is a gate and a red here
//! fails the merge. By hand:
//! `cargo test -p geom-brep --features probe --test all -- m8_f67_r1_probes::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

use crate::shared::fixture;
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    CertifyError, ChartWindow, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, Pcurve, PcurveCache,
    PcurveCertifyError, PcurveCheck,
};
use geom_core::k_stats::{self, Probe};
use geom_core::spline::KnotVector;
use geom_core::{MarginDiag, Point2, Point3, Vec2, Vec3};
use slotmap::SlotMap;

/// Chord-sum arc length of a `Curve3<f64>` over `[d0, d1]` — a LOWER
/// bound on the true arc length at every sample count, so a metered
/// comparand at or below it is certainly a lower bound too.
fn chord_sum(c: &Curve3<f64>, d0: f64, d1: f64, n: usize) -> f64 {
    let mut acc = 0.0;
    let mut prev = c.eval(d0);
    for i in 1..=n {
        #[allow(clippy::cast_precision_loss)]
        let t = d0 + (d1 - d0) * (i as f64) / (n as f64);
        let p = c.eval(t);
        acc += prev.distance(p);
        prev = p;
    }
    acc
}

/// The rational quarter circle of radius 2 in the `z = 0` plane,
/// authored over the knot domain `[0, s]` — the same locus at every
/// `s`, so `s` is the reparametrization knob.
fn quarter_circle(s: f64) -> Curve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, s, s, s], 2).expect("knots");
    let n = NurbsCurve3::new(
        k,
        vec![
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
        ],
        vec![1.0, FRAC_1_SQRT_2, 1.0],
    )
    .expect("the rational arc builds");
    Curve3::Nurbs(Arc::new(n))
}

/// A curved multi-span non-rational degree-2 net over `[0, 2s]`.
fn wiggle(s: f64) -> Curve3<f64> {
    let k =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, s, 2.0 * s, 2.0 * s, 2.0 * s], 2).expect("knots");
    let n = NurbsCurve3::new(
        k,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, -0.5, 0.5),
            Point3::new(3.0, 0.5, 0.0),
        ],
        vec![1.0; 4],
    )
    .expect("the net builds");
    Curve3::Nurbs(Arc::new(n))
}

fn speed_bound(c: &Curve3<f64>) -> f64 {
    let Curve3::Nurbs(n) = c else {
        panic!("the probes' carriers are nets by construction")
    };
    n.speed_lower_bound()
}

/// CLAIM 1, rational arm: `domain × speed_lower_bound` is a genuine
/// arc-length lower bound, and the product is invariant under the
/// `t → 2t` reparametrization while the bare rate splits by 2.
#[test]
fn rational_metered_length_is_an_arc_length_lower_bound_and_reparam_invariant() {
    let c1 = quarter_circle(1.0);
    let b1 = speed_bound(&c1);
    assert!(b1 > 0.0, "a healthy rational arc states a positive bound");
    let metered = 1.0 * b1;
    let chords = chord_sum(&c1, 0.0, 1.0, 4096);
    // chord_sum underestimates the true arc length (pi here), so this
    // is the strict test of the lower-bound claim.
    assert!(
        metered <= chords,
        "domain x bound {metered:e} exceeds the chord-sum arc length {chords:e}"
    );
    let c2 = quarter_circle(2.0);
    let b2 = speed_bound(&c2);
    let metered2 = 2.0 * b2;
    assert!(
        ((metered - metered2) / metered).abs() < 1e-14,
        "the metered length must be reparametrization-invariant: {metered:e} vs {metered2:e}"
    );
    assert!(
        (b1 / b2 - 2.0).abs() < 1e-12,
        "the bare rate must split the twins: {b1:e} vs {b2:e}"
    );
}

/// CLAIM 1, non-rational multi-span arm: same statement for a curved
/// two-span degree-2 net.
#[test]
fn multispan_metered_length_is_an_arc_length_lower_bound_and_reparam_invariant() {
    let c1 = wiggle(1.0);
    let b1 = speed_bound(&c1);
    assert!(b1 > 0.0, "a healthy net states a positive bound");
    let metered = 2.0 * b1;
    let chords = chord_sum(&c1, 0.0, 2.0, 4096);
    assert!(
        metered <= chords,
        "domain x bound {metered:e} exceeds the chord-sum arc length {chords:e}"
    );
    let c2 = wiggle(2.0);
    let metered2 = 4.0 * speed_bound(&c2);
    assert!(
        ((metered - metered2) / metered).abs() < 1e-14,
        "the metered length must be reparametrization-invariant: {metered:e} vs {metered2:e}"
    );
}

// ---- End-to-end poison / backwards-span probes (f64 lane). ----

fn axis_planes_f64() -> (Surface<f64>, Surface<f64>) {
    (
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            u_ref: Vec3::new(0.0, 1.0, 0.0),
        },
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
    )
}

fn degenerate_net() -> Curve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("knots");
    let p = Point3::new(0.0, 0.0, 1.0);
    let n = NurbsCurve3::new(k, vec![p, p], vec![1.0, 1.0]).expect("the degenerate net builds");
    Curve3::Nurbs(Arc::new(n))
}

fn axis_segment_f64(length: f64) -> Curve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("knots");
    let n = NurbsCurve3::new(
        k,
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, length)],
        vec![1.0, 1.0],
    )
    .expect("the carrier builds");
    Curve3::Nurbs(Arc::new(n))
}

fn certify_axis(carrier: Curve3<f64>, t0: f64, t1: f64) -> Result<EdgeCurve<f64>, CertifyError> {
    let (px, py) = axis_planes_f64();
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let s1 = arena.insert(px);
    let s2 = arena.insert(py);
    let witness = carrier.eval((t0 + t1) * 0.5);
    let (start, end) = (carrier.eval(t0), carrier.eval(t1));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
        carrier,
        param_start: t0,
        param_end: t1,
    };
    EdgeCurve::certify(spec, start, end, |k| arena.get(k).cloned(), band())
}

/// CLAIM 2, certify gate: a poison meter escalates `Invalid` naming
/// `nurbs_span_meter` — never a silent pass, never `IntervalNotForward`.
#[test]
fn certify_poison_meter_escalates_invalid_at_the_meter() {
    match certify_axis(degenerate_net(), 0.0, 1.0) {
        Err(CertifyError::Escalated { cause, .. }) => {
            assert_eq!(cause.predicate, Some("nurbs_span_meter"), "{cause:?}");
            assert!(matches!(cause.margin, MarginDiag::Invalid), "{cause:?}");
        }
        other => panic!("a poison meter must escalate at the meter: {other:?}"),
    }
}

/// CLAIM 2, certify gate: a backwards span on a HEALTHY carrier stays
/// `IntervalNotForward` — the two failure modes remain distinct.
#[test]
fn certify_backwards_span_stays_interval_not_forward() {
    match certify_axis(axis_segment_f64(1.0), 1.0, 0.0) {
        Err(CertifyError::IntervalNotForward) => {}
        other => panic!("a backwards span must stay IntervalNotForward: {other:?}"),
    }
}

/// The rational quarter-cylinder wall (the m7_8 fixture, verbatim).
fn quarter_cylinder_wall() -> Surface<f64> {
    Surface::Nurbs(Arc::new(fixture::quarter_cylinder_wall()))
}

fn window() -> ChartWindow<f64> {
    ChartWindow {
        u_min: -10.0,
        u_max: 10.0,
        v_min: -10.0,
        v_max: 10.0,
    }
}

/// CLAIM 2, iso lane: the poison meter refuses AT THE METER
/// (`pcurve_interval_meter`, `Invalid`), end-to-end through
/// `PcurveCache::certify`, not just at the helper.
#[test]
fn iso_lane_poison_meter_escalates_invalid_end_to_end() {
    let iso = Pcurve::IsoLine {
        p0: Point2::new(0.0, 0.0),
        pl: Vec2::new(0.0, 1.0),
    };
    let got = PcurveCache::certify(
        iso,
        0.0,
        1.0,
        &degenerate_net(),
        &quarter_cylinder_wall(),
        window(),
        band(),
    );
    match got {
        Err(PcurveCertifyError::Escalated {
            check: PcurveCheck::ParamSpan,
            cause,
            ..
        }) => {
            assert_eq!(cause.predicate, Some("pcurve_interval_meter"), "{cause:?}");
            assert!(matches!(cause.margin, MarginDiag::Invalid), "{cause:?}");
        }
        other => panic!("the iso lane must refuse a poison meter at the meter: {other:?}"),
    }
}

/// CLAIM 2, iso lane: a healthy carrier with a backwards span stays
/// `IntervalNotForward` — distinguishable from the poison refusal.
#[test]
fn iso_lane_backwards_span_stays_interval_not_forward() {
    let iso = Pcurve::IsoLine {
        p0: Point2::new(0.0, 0.0),
        pl: Vec2::new(0.0, 1.0),
    };
    let got = PcurveCache::certify(
        iso,
        1.0,
        0.0,
        &axis_segment_f64(1.0),
        &quarter_cylinder_wall(),
        window(),
        band(),
    );
    match got {
        Err(PcurveCertifyError::IntervalNotForward) => {}
        other => panic!("a backwards span must stay IntervalNotForward: {other:?}"),
    }
}

/// CLAIM 2, fitted lane: the poison meter refuses at the meter through
/// `PcurveCache::certify_fitted` (check 1 is passed with a mate so
/// check 2's gate is the deciding site).
#[test]
fn fitted_lane_poison_meter_escalates_invalid_end_to_end() {
    let ik = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("image knots");
    let image = Arc::new(
        NurbsCurve2::new(
            ik,
            vec![Point2::new(0.0, 0.0), Point2::new(0.0, 1.0)],
            vec![1.0, 1.0],
        )
        .expect("the image builds"),
    );
    let (px, _) = axis_planes_f64();
    let wall = quarter_cylinder_wall();
    let got = PcurveCache::certify_fitted(
        image,
        0.0,
        1.0,
        &degenerate_net(),
        &wall,
        Some(&px),
        window(),
        band(),
    );
    match got {
        Err(PcurveCertifyError::Escalated {
            check: PcurveCheck::ParamSpan,
            cause,
            ..
        }) => {
            assert_eq!(cause.predicate, Some("pcurve_interval_meter"), "{cause:?}");
            assert!(matches!(cause.margin, MarginDiag::Invalid), "{cause:?}");
        }
        other => panic!("the fitted lane must refuse a poison meter at the meter: {other:?}"),
    }
}

// ---- The pcurve gate's own reparametrization twin, at Probe. ----

fn probe_segment(length: f64, domain: f64) -> Curve3<Probe> {
    let k = KnotVector::clamped(vec![0.0, 0.0, domain, domain], 1).expect("knots");
    let n = NurbsCurve3::new(
        k,
        vec![
            Point3::new(Probe(0.0), Probe(0.0), Probe(0.0)),
            Point3::new(Probe(0.0), Probe(0.0), Probe(length)),
        ],
        vec![1.0, 1.0],
    )
    .expect("the carrier builds");
    Curve3::Nurbs(Arc::new(n))
}

/// CLAIM 1 extension to the F6 sites: the iso lane's
/// `pcurve_interval_meter` AND `pcurve_interval_forward` margins are
/// both invariant under `t → 2t`.
#[test]
fn iso_lane_gate_and_span_margins_are_reparametrization_invariant() {
    let wall: Surface<Probe> = {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("u knots");
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("v knots");
        let control: Vec<Point3<Probe>> = [
            (1.0, 0.0, 0.0),
            (1.0, 0.0, 1.0),
            (1.0, 1.0, 0.0),
            (1.0, 1.0, 1.0),
            (0.0, 1.0, 0.0),
            (0.0, 1.0, 1.0),
        ]
        .iter()
        .map(|&(x, y, z)| Point3::new(Probe(x), Probe(y), Probe(z)))
        .collect();
        let w = FRAC_1_SQRT_2;
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).expect("wall"),
        ))
    };
    let win = ChartWindow {
        u_min: Probe(-10.0),
        u_max: Probe(10.0),
        v_min: Probe(-10.0),
        v_max: Probe(10.0),
    };
    let margins = |domain: f64| {
        let iso = Pcurve::IsoLine {
            p0: Point2::new(Probe(0.0), Probe(0.0)),
            pl: Vec2::new(Probe(0.0), Probe(1.0)),
        };
        k_stats::start_recording();
        // The certification may fail past check 2 (the carrier is not
        // on the wall); the gate and span samples land first, which is
        // all this row reads.
        let _ = PcurveCache::certify(
            iso,
            Probe(0.0),
            Probe(domain),
            &probe_segment(1.0, domain),
            &wall,
            win,
            band(),
        );
        let samples = k_stats::take_samples();
        let get = |name: &str| {
            samples
                .iter()
                .find(|s| s.predicate == name)
                .unwrap_or_else(|| panic!("{name} must fire"))
                .margin
        };
        (get("pcurve_interval_meter"), get("pcurve_interval_forward"))
    };
    let (gate1, fwd1) = margins(1.0);
    let (gate2, fwd2) = margins(2.0);
    assert!(
        (gate1 - gate2).abs() < 1e-15,
        "the gate margin must be invariant: {gate1:e} vs {gate2:e}"
    );
    assert!(
        (fwd1 - fwd2).abs() < 1e-15,
        "the forward margin must be invariant: {fwd1:e} vs {fwd2:e}"
    );
    assert!(
        (gate1 - 1.0).abs() < 1e-15 && (fwd1 - 1.0).abs() < 1e-15,
        "both margins ARE the 1 m subtended length: {gate1:e}, {fwd1:e}"
    );
}

//! **An INTERIOR column of a described NURBS chart certifies as an
//! exact `Pcurve::IsoLine`** — the seam class's control-difference
//! hull over the de Boor collapse (`geom_brep::interior_iso_u`) rather
//! than a boundary row's copy, at the certifier's own door.
//!
//! Every row runs at BOTH scalar lanes through one generic body: `f64`
//! always, `Interval` under the feature. The fixtures are static
//! witnesses at millimetre scale, chosen so the collapse is genuine
//! arithmetic (a `u*` strictly between knots on a degree-2 net: three
//! live rows, none a copy) rather than the Kronecker row a knot would
//! select. The carrier is never taken from the extractor under test:
//! the polynomial row comes from knot insertion (evaluation-invariant
//! in ℝ), the rational one from the chart's own evaluator.
//!
//! # ε posture
//!
//! Each certification asserts a DEFINITE outcome at every cell of the
//! ε table: certified with `envelope ≤ ε` AND `envelope < 1e-12`
//! absolutely, so no cell passes on a loose ε; the structural refusal
//! is typed and ε-independent.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::Curve3;
use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{ChartWindow, EnvelopeStatement, Pcurve, PcurveCache, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Point2, Point3, Real, Vec2};
use std::sync::Arc;

/// A degree-2 × degree-1 polynomial chart, `u` knots at thirds, bowed
/// in `y` so no column is a straight copy of another, ~3 mm across.
fn polynomial_chart() -> NurbsSurface<f64> {
    let ku =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::unit_segment(1);
    let bow = [0.0, 0.7e-3, 1.1e-3, 0.4e-3, 0.0];
    let mut control = Vec::new();
    for (i, y) in bow.iter().enumerate() {
        let x = 0.75e-3 * i as f64;
        for z in [0.0, 2.0e-3] {
            control.push(Point3::new(x, *y + 0.1 * z, z));
        }
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap()
}

/// dm1's cylinder wall (`imported_chart_arc_rim.rs`'s shape): a
/// full-period rational-quadratic cylinder as one patch, weights
/// `1, ½, 1, …` along `u` and constant along `v` — the separable
/// case (b) every arc-profile wall this kernel builds is in.
fn imported_wall(perturb: Option<usize>) -> NurbsSurface<f64> {
    const R: f64 = 0.005;
    const H: f64 = 0.01;
    let tau = core::f64::consts::TAU;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for k in 0..7 {
        let on = k % 2 == 0;
        let theta = tau * (k as f64) / 6.0;
        let rr = if on { R } else { 2.0 * R };
        let w = if on { 1.0 } else { 0.5 };
        for v in [0.0, H] {
            control.push(Point3::new(rr * theta.cos(), rr * theta.sin(), v));
            weights.push(w);
        }
    }
    if let Some(index) = perturb {
        weights[index] = 0.9;
    }
    let ku = KnotVector::clamped(
        vec![
            0.0,
            0.0,
            0.0,
            1.7320508075689,
            1.7320508075689,
            3.4641016151377,
            3.4641016151377,
            5.1961524227066,
            5.1961524227066,
            5.1961524227066,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

/// The surface lifted to `T` (control points through `from_f64`;
/// weights and knots are `f64` structure and stay so).
fn lift_surface<T: Real>(s: &NurbsSurface<f64>) -> NurbsSurface<T> {
    let control = s
        .control()
        .iter()
        .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
        .collect();
    NurbsSurface::new(
        s.knots_u().clone(),
        s.knots_v().clone(),
        control,
        s.weights().to_vec(),
    )
    .unwrap()
}

/// The curve lifted to `T`, as [`lift_surface`].
fn lift_curve<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve3<T> {
    let control = c
        .control()
        .iter()
        .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
        .collect();
    NurbsCurve3::new(c.knots().clone(), control, c.weights().to_vec()).unwrap()
}

/// `certify` of the column `u = u_star` traversing the whole `v`
/// domain, carrier and chart lifted to `T`.
fn certify_column<T: Decide>(
    chart: &NurbsSurface<f64>,
    carrier: &NurbsCurve3<f64>,
    u_star: f64,
) -> Result<PcurveCache<T>, PcurveCertifyError> {
    let (u0, u1) = chart.knots_u().domain();
    let (v0, v1) = chart.knots_v().domain();
    let f = T::from_f64;
    PcurveCache::certify(
        Pcurve::IsoLine {
            p0: Point2::new(f(u_star), f(v0)),
            pl: Vec2::new(T::zero(), f(v1 - v0)),
        },
        T::zero(),
        T::one(),
        &Curve3::Nurbs(Arc::new(lift_curve::<T>(carrier))),
        &Surface::Nurbs(Arc::new(lift_surface::<T>(chart))),
        ChartWindow {
            u_min: f(u0),
            u_max: f(u1),
            v_min: f(v0),
            v_max: f(v1),
        },
        band(),
    )
}

/// The row asserts the certificate's shape and its two bounds: inside
/// the run's ε (the gate) and inside `1e-12` absolutely (so no cell
/// passes on a loose ε). At the interval lane the envelope's upper end
/// is what is bounded.
fn assert_certified<T: Decide + Bounds>(row: &str, cache: &PcurveCache<T>) {
    let cert = cache.certificate();
    assert_eq!(
        cert.statement,
        EnvelopeStatement::MapResidualIsoHull,
        "{row}: the seam class's hull statement"
    );
    let hi = cert.envelope.hi();
    assert!(
        hi <= eps(),
        "{row}: envelope {hi:e} m inside ε = {:e}",
        eps()
    );
    assert!(
        hi < 1e-12,
        "{row}: envelope {hi:e} m is tight, not merely inside ε"
    );
    assert!(
        matches!(cache.pcurve(), Pcurve::IsoLine { .. }),
        "{row}: the certified image is the exact iso class: {:?}",
        cache.pcurve()
    );
}

/// **A1 — a genuine collapse on a polynomial chart.** The carrier is
/// `S(0.5, ·)`, derived WITHOUT the extractor: inserting `u = 0.5` to
/// multiplicity 2 (= the degree) exposes it as a control row of the
/// refined net, the one at Greville abscissa `0.5`.
fn a1_body<T: Decide + Bounds>(row: &str) {
    let chart = polynomial_chart();
    let refined = chart
        .insert_knot_u(0.5, 2)
        .expect("knot insertion to multiplicity p");
    let (_, nv) = refined.control_counts();
    let kn = refined.knots_u().knots();
    let p = refined.knots_u().degree();
    let index = (0..refined.control_counts().0)
        .find(|i| kn[i + 1..=i + p].iter().sum::<f64>() / p as f64 == 0.5)
        .expect("a control row sits at Greville abscissa 0.5");
    let carrier = NurbsCurve3::new(
        refined.knots_v().clone(),
        refined.control()[index * nv..(index + 1) * nv].to_vec(),
        refined.weights()[index * nv..(index + 1) * nv].to_vec(),
    )
    .unwrap();
    // The witness is a witness: the exposed row IS the column, by the
    // chart's own evaluator, and it is NOT a row of the original net.
    for i in 0..=4 {
        let v = f64::from(i) / 4.0;
        assert!(carrier.eval(v).distance(chart.eval(0.5, v)) < 1e-15);
    }
    let (nu, _) = chart.control_counts();
    let nearest = (0..nu)
        .map(|i| carrier.control()[0].distance(chart.control()[i * nv]))
        .fold(f64::INFINITY, f64::min);
    assert!(
        nearest > 1e-4,
        "no control row of the net is this column: {nearest:e} m"
    );
    // (the collapse at 0.5 weighs rows 1..=3 as 1/8, 3/4, 1/8: ~1.4e-4 m
    // from the nearest row, so a row-copy mutant cannot pass)
    let cache = certify_column::<T>(&chart, &carrier, 0.5).unwrap_or_else(|e| {
        panic!("{row}: an interior column of a polynomial chart certifies exactly: {e}")
    });
    assert_certified(row, &cache);
    println!("{row}: envelope {:e} m", cache.certificate().envelope.hi());
}

#[test]
fn a1_an_interior_column_of_a_polynomial_chart_certifies() {
    a1_body::<f64>("A1 f64");
}

#[cfg(feature = "interval")]
#[test]
fn a1_an_interior_column_of_a_polynomial_chart_certifies_at_interval() {
    a1_body::<geom_core::Interval>("A1 interval");
}

/// **A2 — the rational case (b).** dm1's wall at `u* = √3/2`, the
/// middle of the first sub-arc, where the weight-½ row is fully live;
/// the carrier is the meridian segment, a degree-1 polynomial from
/// `S(u*, 0)` to `S(u*, H)` by the chart's own evaluator. Only a
/// collapse that carries the weights through `λ` lands on the
/// cylinder; one that does not misses it by O(R·(1 − w)).
fn a2_body<T: Decide + Bounds>(row: &str) {
    let chart = imported_wall(None);
    let u_star = 3f64.sqrt() / 2.0;
    let carrier = NurbsCurve3::new(
        chart.knots_v().clone(),
        vec![chart.eval(u_star, 0.0), chart.eval(u_star, 1.0)],
        vec![1.0; 2],
    )
    .unwrap();
    let r = carrier.control()[0].x.hypot(carrier.control()[0].y);
    assert!(
        (r - 0.005).abs() < 1e-15,
        "the witness sits on the cylinder: {r}"
    );
    let cache = certify_column::<T>(&chart, &carrier, u_star).unwrap_or_else(|e| {
        panic!("{row}: an interior column of a case-(b) rational chart certifies: {e}")
    });
    assert_certified(row, &cache);
    println!("{row}: envelope {:e} m", cache.certificate().envelope.hi());
}

#[test]
fn a2_an_interior_column_of_a_separable_rational_chart_certifies() {
    a2_body::<f64>("A2 f64");
}

#[cfg(feature = "interval")]
#[test]
fn a2_an_interior_column_of_a_separable_rational_chart_certifies_at_interval() {
    a2_body::<geom_core::Interval>("A2 interval");
}

/// **A2b — a non-separable net refuses typed.** One weight of a row
/// NOT live at `u*` is perturbed (so the column itself, and the
/// schedule residual, are untouched): the net now varies along both
/// parameters, no carrier can share the collapsed row's space, and the
/// refusal names the weight net — structural, the same at every ε.
fn a2b_body<T: Decide + Bounds>(row: &str) {
    let chart = imported_wall(Some(11)); // row 5, column 1: `u ∈ [2√3, 3√3]`, far from u*
    let u_star = 3f64.sqrt() / 2.0;
    let carrier = NurbsCurve3::new(
        chart.knots_v().clone(),
        vec![chart.eval(u_star, 0.0), chart.eval(u_star, 1.0)],
        vec![1.0; 2],
    )
    .unwrap();
    let refusal = certify_column::<T>(&chart, &carrier, u_star)
        .err()
        .unwrap_or_else(|| panic!("{row}: a non-separable weight net has no exact class"));
    let PcurveCertifyError::IsoUnsupported { what } = refusal else {
        panic!("{row}: the refusal is the typed class refusal, not a residual: {refusal:?}")
    };
    assert!(
        what.contains("weight net varies in both directions"),
        "{row}: the refusal names the weight net: {what}"
    );
}

#[test]
fn a2b_a_non_separable_weight_net_refuses_typed() {
    a2b_body::<f64>("A2b f64");
}

#[cfg(feature = "interval")]
#[test]
fn a2b_a_non_separable_weight_net_refuses_typed_at_interval() {
    a2b_body::<geom_core::Interval>("A2b interval");
}

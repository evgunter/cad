//! **The interior-column collapse under the review's own fixtures** —
//! rows adopted from the TRIM-1 dual review (R1's `r1_c1_*`, `r1_c2_*`
//! and `r1_c3_*` probes, authorship preserved; the out-of-domain rows
//! are the fix pass's, on R1's chart).
//!
//! The fixture is a degree-3 × degree-2 chart with interior knots in
//! both directions and a weight net that can be made polynomial,
//! constant along `u` (case (a): a rational profile in `v`), constant
//! along `v` (case (b), unit and non-unit) or varying both ways. What
//! `interior_iso_column.rs` states at the certifier's door on the
//! unit's own fixtures, these rows state on nets the unit did not
//! build — in particular the case-(a) arm, which the unit's fixtures
//! reach only with unit weights.
//!
//! # ε posture
//!
//! Every certification asserts a definite outcome at every cell; the
//! soundness rows assert `envelope ≥ dense residual` wherever the door
//! certifies, and that it certifies at least once.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::Curve3;
use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{
    EnvelopeStatement, IsoRowError, Pcurve, PcurveCache, PcurveCertifyError, boundary_iso_u,
    interior_iso_u,
};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Point2, Point3, Real, Vec2};
use std::sync::Arc;

/// The weight net: polynomial, constant along `u` (case (a)), constant
/// along `v` (case (b), unit or non-unit), or varying both ways.
#[derive(Clone, Copy)]
enum Net {
    Poly,
    CaseA,
    CaseB,
    CaseBNonUnit,
    Both,
}

/// Degree 3 in `u` (knots at thirds, 6 columns) × degree 2 in `v` (one
/// interior knot at 0.5, 4 rows), `scale` metres per 0.6 mm of the
/// original (R1's chart at `scale = 1.0`), bowed in `y` and `z`.
fn cubic_chart(net: Net, scale: f64) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let (nu, nv) = (6usize, 4usize);
    let bow_u = [0.0, 0.7e-3, 1.1e-3, 0.9e-3, 0.4e-3, 0.0];
    let bow_v = [0.0, 0.3e-3, -0.2e-3, 0.0];
    let wu = [2.0, 1.0, 2.0, 1.5, 2.0, 2.0];
    let wv = [1.0, 0.7, 0.9, 1.0];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let x = 0.6e-3 * i as f64;
            let z = 0.8e-3 * j as f64;
            let p = Point3::new(x, bow_u[i] + bow_v[j] + 0.05 * z, z + 0.1 * bow_u[i]);
            control.push(Point3::new(p.x * scale, p.y * scale, p.z * scale));
            weights.push(match net {
                Net::Poly => 1.0,
                Net::CaseA => wv[j],
                Net::CaseB => {
                    if i % 2 == 0 {
                        1.0
                    } else {
                        0.5
                    }
                }
                Net::CaseBNonUnit => wu[i],
                Net::Both => wv[j] * wu[i],
            });
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

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

fn lift_curve<T: Real>(c: &NurbsCurve3<f64>) -> NurbsCurve3<T> {
    let control = c
        .control()
        .iter()
        .map(|p| Point3::new(T::from_f64(p.x), T::from_f64(p.y), T::from_f64(p.z)))
        .collect();
    NurbsCurve3::new(c.knots().clone(), control, c.weights().to_vec()).unwrap()
}

/// The row `u = u*` exposed by knot insertion to multiplicity `p` —
/// the independent witness (evaluation-invariant in ℝ, rational-aware).
fn inserted_row(chart: &NurbsSurface<f64>, u_star: f64) -> NurbsCurve3<f64> {
    let p = chart.knots_u().degree();
    let have = chart
        .knots_u()
        .multiplicity_of(u_star)
        .map_or(0, |(s, _)| s);
    let refined = chart.insert_knot_u(u_star, p - have).unwrap();
    let (_, nv) = refined.control_counts();
    let kn = refined.knots_u().knots();
    let index = (0..refined.control_counts().0)
        .find(|i| (kn[i + 1..=i + p].iter().sum::<f64>() / p as f64 - u_star).abs() < 1e-15)
        .expect("a control row at the Greville abscissa");
    NurbsCurve3::new(
        refined.knots_v().clone(),
        refined.control()[index * nv..(index + 1) * nv].to_vec(),
        refined.weights()[index * nv..(index + 1) * nv].to_vec(),
    )
    .unwrap()
}

/// `certify` of the line `p0 + pl·t`, `t ∈ [0, 1]`, against the chart's
/// whole domain as the window.
fn certify_line<T: Decide>(
    chart: &NurbsSurface<f64>,
    carrier: &NurbsCurve3<f64>,
    p0: (f64, f64),
    pl: (f64, f64),
) -> Result<PcurveCache<T>, PcurveCertifyError> {
    let (u0, u1) = chart.knots_u().domain();
    let (v0, v1) = chart.knots_v().domain();
    let f = T::from_f64;
    PcurveCache::certify(
        Pcurve::IsoLine {
            p0: Point2::new(f(p0.0), f(p0.1)),
            pl: Vec2::new(f(pl.0), f(pl.1)),
        },
        T::zero(),
        T::one(),
        &Curve3::Nurbs(Arc::new(lift_curve::<T>(carrier))),
        &Surface::Nurbs(Arc::new(lift_surface::<T>(chart))),
        geom_brep::ChartWindow {
            u_min: f(u0),
            u_max: f(u1),
            v_min: f(v0),
            v_max: f(v1),
        },
        band(),
    )
}

/// Dense sup of `|S(u(t), v(t)) − C(t)|` over `t ∈ [0, 1]`.
fn dense_residual(
    chart: &NurbsSurface<f64>,
    carrier: &NurbsCurve3<f64>,
    p0: (f64, f64),
    pl: (f64, f64),
) -> f64 {
    (0..=2000)
        .map(|i| {
            let t = f64::from(i) / 2000.0;
            chart
                .eval(p0.0 + pl.0 * t, p0.1 + pl.1 * t)
                .distance(carrier.eval(t))
        })
        .fold(0.0, f64::max)
}

const NETS: [(&str, Net); 4] = [
    ("poly", Net::Poly),
    ("caseA", Net::CaseA),
    ("caseB", Net::CaseB),
    ("caseB-nonunit", Net::CaseBNonUnit),
];

// ------------------------------------------------------------------ C1

/// The extractor's row IS `S(u*, ·)` at f64 on every separable net —
/// mid-span, ON an interior knot, near the ends — and at both ends it
/// is the boundary copy bit for bit; a net varying both ways refuses.
#[test]
fn r1_c1_collapse_matches_evaluation_f64() {
    for (name, net) in NETS {
        let s = cubic_chart(net, 1.0);
        for u in [0.5, 1.0 / 3.0, 2.0 / 3.0, 0.1, 0.999_999, 1e-7, 0.0, 1.0] {
            let c = interior_iso_u(&s, u).unwrap_or_else(|e| panic!("{name} u={u}: {e}"));
            let mut worst = 0.0f64;
            for i in 0..=64 {
                let v = f64::from(i) / 64.0;
                worst = worst.max(c.eval(v).distance(s.eval(u, v)));
            }
            assert!(worst < 1e-16, "{name} u* = {u}: worst {worst:e}");
        }
        for (u, end) in [(0.0, false), (1.0, true)] {
            let a = interior_iso_u(&s, u).unwrap();
            let b = boundary_iso_u(&s, end).unwrap();
            let bit = a.control().iter().zip(b.control()).all(|(p, q)| {
                p.x.to_bits() == q.x.to_bits()
                    && p.y.to_bits() == q.y.to_bits()
                    && p.z.to_bits() == q.z.to_bits()
            });
            assert!(
                bit,
                "{name}: the domain-end collapse is the boundary row's control bitwise"
            );
            // Case (b) wraps its row `1.0` where the boundary copy
            // carries the net's own (constant) row weights — the same
            // curve; elsewhere the weights are the copy's, bitwise.
            assert!(
                matches!(net, Net::CaseB | Net::CaseBNonUnit) || a.weights() == b.weights(),
                "{name}: the domain-end collapse carries the boundary row's weights"
            );
        }
    }
    let s = cubic_chart(Net::Both, 1.0);
    let e = interior_iso_u(&s, 0.5).expect_err("both-varying refuses");
    assert!(
        matches!(
            e,
            IsoRowError::WeightsNotSeparable {
                control_counts: (6, 4)
            }
        ),
        "{e}"
    );
}

/// At `Interval` the row's enclosures meet the surface's own and
/// contain the f64 truth, at degenerate `u*` (mid-span, ON the interior
/// knot — the straddle, near an end) and at a WIDE `u*` straddling the
/// knot, where the row must enclose `S(u, v)` for every `u` in it.
#[cfg(feature = "interval")]
#[test]
fn r1_c1_collapse_encloses_evaluation_interval() {
    use geom_core::Interval;
    for (name, net) in NETS {
        let s = cubic_chart(net, 1.0);
        let si = lift_surface::<Interval>(&s);
        for u in [0.5, 1.0 / 3.0, 2.0 / 3.0, 1e-7, 0.999_999] {
            let c = interior_iso_u(&si, Interval::from_bounds(u, u)).unwrap();
            for i in 0..=64 {
                let v = f64::from(i) / 64.0;
                let pc = c.eval(Interval::from_bounds(v, v));
                let ps = si.eval(Interval::from_bounds(u, u), Interval::from_bounds(v, v));
                let pf = s.eval(u, v);
                for (a, b, f) in [(pc.x, ps.x, pf.x), (pc.y, ps.y, pf.y), (pc.z, ps.z, pf.z)] {
                    assert!(
                        a.lo() <= b.hi() && b.lo() <= a.hi(),
                        "{name} u*={u} v={v}: row enclosure [{}, {}] misses the surface's [{}, {}]",
                        a.lo(),
                        a.hi(),
                        b.lo(),
                        b.hi()
                    );
                    assert!(
                        a.lo() - 5e-18 <= f && f <= a.hi() + 5e-18,
                        "{name} u*={u} v={v}: f64 point {f} outside the row enclosure [{}, {}]",
                        a.lo(),
                        a.hi()
                    );
                }
            }
        }
        let c = interior_iso_u(&si, Interval::from_bounds(0.30, 0.36)).unwrap();
        let mut worst_out = 0.0f64;
        for u in [0.30, 0.32, 1.0 / 3.0, 0.34, 0.36] {
            for i in 0..=32 {
                let v = f64::from(i) / 32.0;
                let pc = c.eval(Interval::from_bounds(v, v));
                let pf = s.eval(u, v);
                for (a, f) in [(pc.x, pf.x), (pc.y, pf.y), (pc.z, pf.z)] {
                    worst_out = worst_out.max((a.lo() - f).max(f - a.hi()).max(0.0));
                }
            }
        }
        assert!(
            worst_out <= 5e-18,
            "{name}: the straddling hull does not enclose S(u, v): {worst_out:e}"
        );
    }
}

/// A degree-2 chart with a DOUBLE interior `u` knot (a C⁰ crease with
/// a derivative jump) and a narrow `u*` straddling it at the interval
/// scalar: the per-span hull must enclose `S(u, v)` on both sides — a
/// first-span-only extractor extrapolates the wrong polynomial across
/// the crease.
#[cfg(feature = "interval")]
#[test]
fn r1_c1_straddle_at_a_crease_interval() {
    use geom_core::Interval;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ys = [0.0, 1.0e-3, 0.0, -1.0e-3, 0.0];
    let mut control = Vec::new();
    for (i, y) in ys.iter().enumerate() {
        for z in [0.0, 1.0e-3] {
            control.push(Point3::new(0.5e-3 * i as f64, *y, z));
        }
    }
    let s = NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap();
    let si = lift_surface::<Interval>(&s);
    for d in [1e-6, 1e-9, 1e-12] {
        let c = interior_iso_u(&si, Interval::from_bounds(0.5 - d, 0.5 + d)).unwrap();
        let mut worst_out = 0.0f64;
        for u in [0.5 - d, 0.5, 0.5 + d] {
            for i in 0..=8 {
                let v = f64::from(i) / 8.0;
                let pc = c.eval(Interval::from_bounds(v, v));
                let pf = s.eval(u, v);
                for (a, f) in [(pc.x, pf.x), (pc.y, pf.y), (pc.z, pf.z)] {
                    worst_out = worst_out.max((a.lo() - f).max(f - a.hi()).max(0.0));
                }
            }
        }
        assert!(
            worst_out <= 5e-18,
            "d = {d:e}: the straddling hull does not enclose S(u, v) across the crease: {worst_out:e}"
        );
    }
}

// ------------------------------------------------------------------ C2

/// Case (a) at the certifier's door, with row-0 weights VARYING along
/// `v`: the inserted row certifies; a constant non-unit carrier, a
/// proportional carrier and a refined-knot carrier refuse (the shared
/// space is bitwise, not up to a factor); case (b) non-unit accepts any
/// constant carrier and refuses a varying one; a both-varying net
/// refuses at the extractor.
fn c2_body<T: Decide + Bounds>(lane: &str) {
    let s = cubic_chart(Net::CaseA, 1.0);
    let u = 0.5;
    let row = inserted_row(&s, u);
    assert_eq!(
        row.weights(),
        &[1.0, 0.7, 0.9, 1.0],
        "the inserted row carries row 0's weights"
    );
    let (v0, v1) = s.knots_v().domain();
    let ok = certify_line::<T>(&s, &row, (u, v0), (0.0, v1 - v0))
        .unwrap_or_else(|e| panic!("{lane}: case (a) with varying row-0 weights certifies: {e}"));
    assert_eq!(
        ok.certificate().statement,
        EnvelopeStatement::MapResidualIsoHull
    );
    let env = ok.certificate().envelope.hi();
    assert!(env < 1e-12, "{lane}: case (a) envelope {env:e}");
    // A constant non-unit carrier against varying `W_j`: a different
    // curve, so check 3 sees it before the shared-space check does —
    // either refusal is sound.
    let c2 = NurbsCurve3::new(row.knots().clone(), row.control().to_vec(), vec![2.0; 4]).unwrap();
    let e = certify_line::<T>(&s, &c2, (u, v0), (0.0, v1 - v0))
        .expect_err("a constant carrier against varying W refuses");
    assert!(
        matches!(
            e,
            PcurveCertifyError::IsoUnsupported { .. } | PcurveCertifyError::ResidualExceeded { .. }
        ),
        "{lane}: {e:?}"
    );
    // A proportional (2·W) carrier IS the same curve in ℝ, and the
    // shared-space check is deliberately bitwise: refused typed.
    let c3 = NurbsCurve3::new(
        row.knots().clone(),
        row.control().to_vec(),
        row.weights().iter().map(|w| 2.0 * w).collect(),
    )
    .unwrap();
    let e = certify_line::<T>(&s, &c3, (u, v0), (0.0, v1 - v0))
        .expect_err("a proportional-weight carrier refuses: the shared space is bitwise");
    assert!(
        matches!(e, PcurveCertifyError::IsoUnsupported { what } if what.contains("one spline space")),
        "{lane}: {e:?}"
    );
    // A carrier in a knot-insertion refinement of the row's `v` space:
    // the same curve in ℝ, a different space, refused typed.
    let refined_chart = s.insert_knot_v(0.25, 1).unwrap();
    let c4 = inserted_row(&refined_chart, u);
    for i in 0..=8 {
        let v = f64::from(i) / 8.0;
        assert!(
            c4.eval(v).distance(s.eval(u, v)) < 1e-17,
            "the refined carrier is the same curve"
        );
    }
    let e = certify_line::<T>(&s, &c4, (u, v0), (0.0, v1 - v0))
        .expect_err("a refined-knot carrier refuses");
    assert!(
        matches!(e, PcurveCertifyError::IsoUnsupported { what } if what.contains("one spline space")),
        "{lane}: {e:?}"
    );
    // Case (b) non-unit: any constant carrier certifies; a varying one
    // refuses.
    let sb = cubic_chart(Net::CaseBNonUnit, 1.0);
    let rowb = inserted_row(&sb, u);
    let poly =
        NurbsCurve3::new(rowb.knots().clone(), rowb.control().to_vec(), vec![3.0; 4]).unwrap();
    let ok = certify_line::<T>(&sb, &poly, (u, v0), (0.0, v1 - v0)).unwrap_or_else(|e| {
        panic!("{lane}: case (b) non-unit chart, constant[3] carrier certifies: {e}")
    });
    assert!(ok.certificate().envelope.hi() < 1e-12);
    let vary = NurbsCurve3::new(
        rowb.knots().clone(),
        rowb.control().to_vec(),
        vec![1.0, 2.0, 1.0, 1.0],
    )
    .unwrap();
    let e = certify_line::<T>(&sb, &vary, (u, v0), (0.0, v1 - v0))
        .expect_err("a varying carrier against a polynomial row refuses");
    assert!(
        matches!(
            e,
            PcurveCertifyError::IsoUnsupported { .. } | PcurveCertifyError::ResidualExceeded { .. }
        ),
        "{lane}: {e:?}"
    );
    // A both-varying net: perturb a weight on a row NOT live at
    // `u* = 0.5` (rows 1..=4 are), so the carrier stays the true curve,
    // check 3 passes, and the refusal is the extractor's.
    let mut wts = s.weights().to_vec();
    wts[5 * 4 + 1] = 0.95;
    let sboth = NurbsSurface::new(
        s.knots_u().clone(),
        s.knots_v().clone(),
        s.control().to_vec(),
        wts,
    )
    .unwrap();
    let e = certify_line::<T>(&sboth, &row, (u, v0), (0.0, v1 - v0))
        .expect_err("a both-varying net refuses");
    assert!(
        matches!(e, PcurveCertifyError::IsoUnsupported { what } if what.contains("weight net varies in both directions")),
        "{lane}: {e:?}"
    );
}

#[test]
fn r1_c2_shared_space_f64() {
    c2_body::<f64>("f64");
}

#[cfg(feature = "interval")]
#[test]
fn r1_c2_shared_space_interval() {
    c2_body::<geom_core::Interval>("interval");
}

// ------------------------------------------------------------------ C3

/// Soundness of the envelope against a dense residual on both routes:
/// a `u_start` inside the boundary band with the boundary row as
/// carrier, and a DRIFTING seam (`pl.x ≠ 0`) with the inserted row as
/// carrier — the interior route's drift-only slack is what keeps the
/// second sound. Wherever the door certifies, `envelope ≥ residual`;
/// the smallest drift certifies at every ε, so the term is exercised.
fn c3_body<T: Decide + Bounds>(lane: &str) {
    let s = cubic_chart(Net::CaseA, 1.0);
    let (v0, v1) = s.knots_v().domain();
    let boundary = boundary_iso_u(&s, false).unwrap();
    for delta in [1e-9, 1e-7, 3e-6, 1e-4] {
        let (p0, pl) = ((delta, v0), (0.0, v1 - v0));
        let res = dense_residual(&s, &boundary, p0, pl);
        if let Ok(c) = certify_line::<T>(&s, &boundary, p0, pl) {
            let env = c.certificate().envelope.hi();
            assert!(
                env >= res,
                "{lane}: UNSOUND envelope {env:e} < residual {res:e} at u_start = {delta:e}"
            );
        }
    }
    let row = inserted_row(&s, 0.5);
    let mut certified = 0;
    for eta in [1e-12, 1e-9, 1e-7, 1e-5] {
        let (p0, pl) = ((0.5, v0), (eta, v1 - v0));
        let res = dense_residual(&s, &row, p0, pl);
        match certify_line::<T>(&s, &row, p0, pl) {
            Ok(c) => {
                certified += 1;
                let env = c.certificate().envelope.hi();
                assert!(
                    env >= res,
                    "{lane}: UNSOUND envelope {env:e} < residual {res:e} at drift {eta:e}"
                );
                assert!(
                    c.certificate().statement == EnvelopeStatement::MapResidualIsoHull && env > 0.0,
                    "{lane}: the drift is paid by the interior route's slack: {env:e}"
                );
            }
            Err(e) => assert!(
                eta > 1e-12,
                "{lane}: the smallest drift certifies at every ε: {e}"
            ),
        }
    }
    assert!(certified >= 1, "{lane}: the drift slack was exercised");
    // The interior route with a WRONG carrier (the boundary row claimed
    // beyond the band): certified or not, never unsound.
    for delta in [1e-6, 1e-5, 1e-3] {
        let (p0, pl) = ((delta, v0), (0.0, v1 - v0));
        let res = dense_residual(&s, &boundary, p0, pl);
        if let Ok(c) = certify_line::<T>(&s, &boundary, p0, pl) {
            let env = c.certificate().envelope.hi();
            assert!(
                env >= res,
                "{lane}: UNSOUND at u = {delta:e}: {env:e} < {res:e}"
            );
        }
    }
}

#[test]
fn r1_c3_slack_soundness_f64() {
    c3_body::<f64>("f64");
}

#[cfg(feature = "interval")]
#[test]
fn r1_c3_slack_soundness_interval() {
    c3_body::<geom_core::Interval>("interval");
}

// ------------------------------------------------------ out-of-domain u*

/// An `IsoLine` whose fixed `u` lies OUTSIDE the chart's `u` domain
/// refuses typed at the seam class's domain decide — even when the
/// carrier is the span's polynomial extension bit for bit and the
/// window is the pcurve's own box (check 5 vacuous, as on the mint
/// path). The chart is R1's at ×10 so that `1e-3` off the domain is
/// definitely outside at every ε the matrix draws.
fn out_of_domain_body<T: Decide + Bounds>(lane: &str) {
    for (name, net) in [
        ("poly", Net::Poly),
        ("caseA", Net::CaseA),
        ("caseB", Net::CaseB),
    ] {
        let s = cubic_chart(net, 10.0);
        let (v0, v1) = s.knots_v().domain();
        for u in [-0.2, 1.2, -1e-3, 1.0 + 1e-3] {
            let row = interior_iso_u(&s, u).unwrap();
            let f = T::from_f64;
            let pc = Pcurve::IsoLine {
                p0: Point2::new(f(u), f(v0)),
                pl: Vec2::new(T::zero(), f(v1 - v0)),
            };
            let window = pc.chart_box(T::zero(), T::one());
            let r = PcurveCache::certify(
                pc,
                T::zero(),
                T::one(),
                &Curve3::Nurbs(Arc::new(lift_curve::<T>(&row))),
                &Surface::Nurbs(Arc::new(lift_surface::<T>(&s))),
                window,
                band(),
            );
            let Err(PcurveCertifyError::IsoUnsupported { what }) = r else {
                panic!("{lane} {name} u* = {u}: a column outside the domain refuses typed: {r:?}")
            };
            assert!(
                what.contains("outside the chart's u domain"),
                "{lane} {name} u* = {u}: the refusal names the domain: {what}"
            );
        }
    }
}

#[test]
fn out_of_domain_u_star_refuses_typed_f64() {
    out_of_domain_body::<f64>("f64");
}

#[cfg(feature = "interval")]
#[test]
fn out_of_domain_u_star_refuses_typed_interval() {
    out_of_domain_body::<geom_core::Interval>("interval");
}

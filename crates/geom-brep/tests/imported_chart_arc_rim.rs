//! **The IMPORTED-chart arc-rim lane, at the certifier's own door**
//! (#327; PR #391 review MINOR-1: the five imported-chart pcurve fixes
//! rode a single wild fixture and had no unit test in the crates they
//! change).
//!
//! M8-3's arc-rim class was written for charts the kernel BUILDS: the
//! unit square `[0,1]²`, sub-arc knots at exact `k/m`, and the rim's
//! increasing carrier parameter running with increasing chart `u`. A
//! chart the kernel IMPORTS satisfies none of those by default. This
//! suite states the three generalizations as executable facts, on a
//! chart shaped exactly like dm1's cylinder wall — `u ∈ [0, 3√3]`,
//! interior knots at `√3` and `2√3` each rounded on its own, so they
//! miss exact thirds of the domain by ~2·10⁻¹⁴.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{ChartWindow, Pcurve, PcurveCache};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Vec2, Vec3};
use geom_core::Tol;

const R: f64 = 0.005;
const H: f64 = 0.01;

/// dm1's wall shape: a full-period rational-quadratic cylinder as ONE
/// patch, degree 2 × 1, `u ∈ [0, 3√3]` with the file's own rounded
/// interior knots, `v ∈ [0, 1]`.
fn imported_wall() -> Surface<f64> {
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
    // The file's printed knots, each rounded independently — NOT exact
    // thirds of the domain. This is the input the bitwise knot check
    // used to refuse.
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
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, weights).unwrap(),
    ))
}

fn rim(axis_z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, axis_z),
        radius: R,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn breaks() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0], 1).unwrap()
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn window() -> ChartWindow<f64> {
    ChartWindow {
        u_min: 0.0,
        u_max: 5.1961524227066,
        v_min: 0.0,
        v_max: 1.0,
    }
}

/// The FORWARD rim on an imported chart: `p0 = (u₀, v)` and the whole
/// displacement `pd = (u₁ − u₀, 0)`. Under the pre-#327 code every one
/// of `side_of`, the expected sub-arc knots and `slack_affine` was
/// written against a literal `0`/`1`, so this certification asked
/// about an interior column and refused.
#[test]
fn a_forward_arc_rim_certifies_on_a_non_unit_chart() {
    let tau = core::f64::consts::TAU;
    let cache = PcurveCache::certify(
        Pcurve::IsoArc {
            p0: Point2::new(0.0, 0.0),
            pd: Vec2::new(5.1961524227066, 0.0),
            t0: 0.0,
            angle: tau,
            breaks: breaks(),
        },
        0.0,
        tau,
        &rim(1.0),
        &imported_wall(),
        window(),
        band(),
    )
    .expect("a full-period rim on its own imported wall must certify");
    // The metered knot slack is real but tiny: the file's knots miss
    // exact thirds by ~2e-14 and the chart's u-stretch is ~6e-3 m per
    // unit, so it contributes ~1e-16 m — paid, not assumed away.
    assert!(
        cache.certificate().envelope < 1e-9,
        "envelope {:e}",
        cache.certificate().envelope
    );
}

/// The REVERSED rim: the same wall, a carrier that winds the other way,
/// `p0 = (u₁, v)` and `pd = (u₀ − u₁, 0)`. A cylinder's two rims
/// routinely wind oppositely in a file, so exactly one of them is this
/// case. Before the u-direction pick the mint emitted the forward image
/// for it — which does not refuse, it traverses the wall BACKWARDS —
/// and the certifier compared its control list in the wrong order.
#[test]
fn a_reversed_arc_rim_certifies_on_a_non_unit_chart() {
    let tau = core::f64::consts::TAU;
    let cache = PcurveCache::certify(
        Pcurve::IsoArc {
            p0: Point2::new(5.1961524227066, 0.0),
            pd: Vec2::new(-5.1961524227066, 0.0),
            t0: 0.0,
            angle: tau,
            breaks: breaks(),
        },
        0.0,
        tau,
        &rim(-1.0),
        &imported_wall(),
        window(),
        band(),
    )
    .expect("a reversed full-period rim must certify too");
    assert!(cache.certificate().envelope < 1e-9);
}

/// And the generalization did not become a licence: a rim placed on an
/// INTERIOR column of the same chart (`u = 1`, which is exactly the
/// value the old code treated as a boundary) still refuses typed.
#[test]
fn an_interior_column_still_refuses() {
    let tau = core::f64::consts::TAU;
    let outcome = PcurveCache::certify(
        Pcurve::IsoArc {
            p0: Point2::new(1.0, 0.0),
            pd: Vec2::new(5.1961524227066, 0.0),
            t0: 0.0,
            angle: tau,
            breaks: breaks(),
        },
        0.0,
        tau,
        &rim(1.0),
        &imported_wall(),
        window(),
        band(),
    );
    assert!(
        outcome.is_err(),
        "u = 1 is an interior column of a [0, 3√3] chart, not a boundary"
    );
}

/// The SEAM class on the same imported chart (M8-4): the boundary
/// column an `Intersection` seam's chart image lands on is a knot-domain
/// end, so a `u = 0` image over the chart's own boundary row certifies —
/// and the `u = 1` restatement of it, which a `[0, 1]` literal would
/// have called a boundary, is an INTERIOR column and refuses typed.
#[test]
fn a_seam_column_certifies_on_a_non_unit_chart() {
    let wall = imported_wall();
    let Surface::Nurbs(ref payload) = wall else {
        panic!("the imported wall is a described NURBS chart")
    };
    let column = geom_brep::boundary_iso_u(payload.as_ref(), false)
        .expect("the chart's own start column re-wraps as a curve");
    let carrier = Curve3::Nurbs(std::sync::Arc::new(column));
    let seam = |u: f64| {
        PcurveCache::certify(
            Pcurve::IsoLine {
                p0: Point2::new(u, 0.0),
                pl: Vec2::new(0.0, 1.0),
            },
            0.0,
            1.0,
            &carrier,
            &wall,
            window(),
            band(),
        )
    };
    let cache = seam(0.0).expect("the chart's own boundary column certifies as a seam");
    assert!(
        cache.certificate().envelope < 1e-9,
        "envelope {:e}",
        cache.certificate().envelope
    );
    // `u = 1` is an INTERIOR column of a `[0, 3√3]` chart AND a
    // different locus from the carrier, so it refuses TYPED — at the
    // residual, which measures the distance between the two loci before
    // the boundary question is reached.
    let refusal = seam(1.0).expect_err("u = 1 is no boundary of this chart");
    let shown = format!("{refusal:?}");
    assert!(
        shown.contains("MapResidual") || shown.contains("INTERIOR"),
        "the refusal is typed and names its check: {shown}"
    );
}

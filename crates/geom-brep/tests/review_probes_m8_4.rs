//! **M8-4 blinded-review probes — the certification boundary behind
//! the Intersection-arm pick.**
//!
//! The mint's residency pick is a single interior probe (t0 + span/4)
//! against each of four candidate images. These rows attack the
//! BACKSTOP: whatever the pick chooses, `run_iso_checks` must refuse
//! any image whose carrier is not the chart's own boundary row within
//! tolerance — a carrier doctored to COINCIDE with the image at the
//! pick's own probe point included.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{ChartWindow, Pcurve, PcurveCache};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point2, Point3, Vec2};

const R: f64 = 0.005;
const H: f64 = 0.01;
const WIDE: f64 = 5.1961524227066;

/// dm1's wall shape (the imported-chart suite's fixture): a
/// full-period rational-quadratic cylinder as ONE patch, closed in
/// `u`, `u ∈ [0, 3√3]`, `v ∈ [0, 1]`.
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
    let ku = KnotVector::clamped(
        vec![
            0.0,
            0.0,
            0.0,
            1.7320508075689,
            1.7320508075689,
            3.4641016151377,
            3.4641016151377,
            WIDE,
            WIDE,
            WIDE,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, weights).unwrap(),
    ))
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn window() -> ChartWindow<f64> {
    ChartWindow {
        u_min: 0.0,
        u_max: WIDE,
        v_min: 0.0,
        v_max: 1.0,
    }
}

fn certify_seam(u: f64, carrier: &Curve3<f64>) -> Result<(), String> {
    PcurveCache::certify(
        Pcurve::IsoLine {
            p0: Point2::new(u, 0.0),
            pl: Vec2::new(0.0, 1.0),
        },
        0.0,
        1.0,
        carrier,
        &imported_wall(),
        window(),
        band(),
    )
    .map(|_| ())
    .map_err(|e| format!("{e:?}"))
}

fn start_column() -> geom::NurbsCurve3<f64> {
    let Surface::Nurbs(p) = imported_wall() else {
        panic!("the imported wall is a described NURBS chart")
    };
    geom_brep::boundary_iso_u(p.as_ref(), false).unwrap()
}

/// **P-A: a carrier doctored to coincide with the image at the mint
/// pick's own probe point (t = 0.25) must NOT certify.** cp0 moves
/// +1e-3·ẑ and cp1 −3e-3·ẑ, so 0.75·δ0 + 0.25·δ1 = 0 exactly: the
/// pick's single-probe discrimination is blind to this carrier, and
/// only the certification (schedule residuals + control hull) stands
/// between it and a silent mint of a non-boundary locus.
#[test]
fn probe_a_pick_point_coincidence_cannot_certify() {
    let b = start_column();
    let mut control = b.control().to_vec();
    control[0].z += 1e-3;
    control[1].z -= 3e-3;
    let doctored = Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(b.knots().clone(), control, b.weights().to_vec()).unwrap(),
    ));
    // The coincidence is real: at t = 0.25 the doctored carrier sits
    // on the candidate image exactly.
    let on_image = imported_wall().eval(0.0, 0.25);
    assert!(
        doctored.eval(0.25).distance(on_image) < 1e-15,
        "the doctored carrier passes through the probe point"
    );
    let refusal = certify_seam(0.0, &doctored)
        .expect_err("a carrier off the boundary row everywhere but the probe point certifies?!");
    assert!(
        refusal.contains("MapResidual") || refusal.contains("Envelope"),
        "the refusal is the residual/hull's own: {refusal}"
    );
    println!("P-A refusal: {refusal}");
}

/// **P-B: a carrier that IS the boundary column geometrically but in a
/// reparameterized spline space refuses TYPED**, never a silent
/// certificate through a hull that would compare nothing.
#[test]
fn probe_b_reparameterized_column_refuses_typed() {
    let b = start_column();
    let stretched = KnotVector::clamped(vec![0.0, 0.0, 2.0, 2.0], 1).unwrap();
    let carrier = Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(stretched, b.control().to_vec(), b.weights().to_vec()).unwrap(),
    ));
    let refusal = certify_seam(0.0, &carrier).expect_err("a foreign spline space certifies?!");
    assert!(
        refusal.contains("not the chart's own boundary row") || refusal.contains("MapResidual"),
        "the refusal names its class: {refusal}"
    );
    println!("P-B refusal: {refusal}");
}

/// **P-C: the CLOSED chart's double residency is benign.** The wall is
/// a full-period cylinder, so `u = 0` and `u = 3√3` are the SAME 3D
/// locus; both columns must certify over the shared carrier — the
/// mint's fixed candidate order then picks `u = 0` deterministically
/// (D9), and neither choice is wrong.
#[test]
fn probe_c_closed_chart_both_columns_certify() {
    let carrier = Curve3::Nurbs(std::sync::Arc::new(start_column()));
    certify_seam(0.0, &carrier).expect("the start column certifies");
    certify_seam(WIDE, &carrier).expect("the end column of a closed chart is the same locus");
}

/// **P-D: a DIAGONAL image refuses typed at the certifier** — the
/// (true, true) arm of the envelope classifier, the excluded class's
/// pin at the geom-brep door.
#[test]
fn probe_d_diagonal_image_refuses_typed() {
    let carrier = Curve3::Nurbs(std::sync::Arc::new(start_column()));
    let refusal = PcurveCache::certify(
        Pcurve::IsoLine {
            p0: Point2::new(0.0, 0.0),
            pl: Vec2::new(1.0, 1.0),
        },
        0.0,
        1.0,
        &carrier,
        &imported_wall(),
        window(),
        band(),
    )
    .map(|_| ())
    .expect_err("a diagonal image certifies?!");
    let shown = format!("{refusal:?}");
    assert!(
        shown.contains("DIAGONAL") || shown.contains("MapResidual"),
        "the refusal names the class: {shown}"
    );
    println!("P-D refusal: {shown}");
}

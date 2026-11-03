//! **An `Intersection` circle on a spline chart whose arc-rim
//! certificate reads in band takes its `General` image**, as one the
//! certificate refuses does: the certificate decides which lane the
//! circle takes (D3), so in band it records nothing and the circle goes
//! on to the general lane, whose own certificate does not rest on it.
//!
//! The fixture is the unit cylinder sheet over `[0, TURN] × [0, 1]`
//! (`topo::test_support::cyl_wall_sheet_keyed`), its rims the circles
//! cut by the planes at each height and described as that
//! `Intersection`, with the analytic cylinder then swapped for a
//! rational ring net in two sub-arcs whose `v = 0` row is the bottom
//! rim, its edges' descriptions left naming the same key. Bending that
//! row's second sub-arc inside the band leaves the pick's probe (the
//! edge's ends and a quarter in) on it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};
use std::sync::Arc;
use topo::test_support::{CylFrame, CylKey, cyl_wall_sheet_keyed};
use topo::{AtRestPolicy, Body, HalfEdgeKey};

/// The sheet's turn.
const TURN: f64 = 1.2;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A distance inside the band: past `zero()`, short of `escalate()`.
fn in_band() -> f64 {
    let b = band();
    0.5 * (b.zero() + b.escalate())
}

/// The unit cylinder `z ∈ [0, 1]` through `TURN` from `x̂`, in two
/// rational quadratic sub-arcs, the second's middle control point pushed
/// `push` off the axis: its `v = 0` row parts from the unit circle by
/// `push / (1 + cos(TURN/4))` at that sub-arc's middle.
fn ring(push: f64) -> NurbsSurface<f64> {
    let m = 2;
    let h = TURN / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * m {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        let r = reach * if j == 3 { 1.0 + push } else { 1.0 };
        for z in [0.0, 1.0] {
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// The sheet at `T` with its cylinder swapped for [`ring`]`(push)`, and
/// the bottom rim's half-edge on the wall.
fn sheet<T: AtRestPolicy + geom_core::Bounds>(push: f64) -> (Body<T>, HalfEdgeKey) {
    let mut body = Body::<T>::new();
    let (wall, cyl) = cyl_wall_sheet_keyed(
        &mut body,
        CylFrame::canonical(1.0),
        CylKey::Bare,
        (0.0, TURN),
        (0.0, 1.0),
        tol(),
    );
    let net = Surface::Nurbs(Arc::new(ring(push).map_scalar(T::from_f64)));
    let body = body.with_surface_moved_for_tests(cyl, net).unwrap();
    let bottom = body
        .half_edges()
        .map(|(k, _)| k)
        .filter(|&h| body.face_of_half_edge(h) == Some(wall))
        .find(|&h| {
            let edge = body.get_half_edge(h).unwrap().edge;
            body.get_curve_geom(body.get_edge(edge).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .is_some_and(|c| {
                    matches!(c.carrier(), geom::Curve3::Circle(..))
                        && c.carrier().eval(c.params().0).z.lo() < 0.5
                })
        })
        .expect("the bottom rim bounds the wall");
    (body, bottom)
}

fn rim_image_on_the_ring<T: AtRestPolicy + geom_core::Bounds>() {
    // The control: on the unbent ring the rim's image is its row.
    let (body, bottom) = sheet::<T>(0.0);
    let got = topo::pcurve_of(&body, bottom, band());
    assert!(
        matches!(got, Ok(Pcurve::IsoArc { .. })),
        "on the true ring the rim is its row, at {}: {got:?}",
        T::NAME
    );

    // The row bent inside the band: the pick shows it, and the
    // certificate reads the bend in band.
    let push = in_band() * (1.0 + (TURN / 4.0).cos());
    let (body, bottom) = sheet::<T>(push);
    let surface = body
        .face_surface(body.face_of_half_edge(bottom).unwrap())
        .unwrap()
        .clone();
    let edge = body.get_half_edge(bottom).unwrap().edge;
    let curve = body
        .get_curve_geom(body.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .clone();
    let (t0, t1) = curve.params();
    let row = geom_brep::pick_arc_rim(curve.carrier(), (t0, t1 - t0), &surface, band())
        .expect("the pick decides")
        .expect("the probe shows the bent row");
    let certified = geom_brep::PcurveCache::certify(row, t0, t1, curve.carrier(), &surface, band());
    assert!(
        matches!(certified, Err(PcurveCertifyError::Escalated { .. })),
        "the row's certificate reads the bend in band, at {}: {:?}",
        T::NAME,
        certified.as_ref().map(|_| ())
    );
    let got = topo::pcurve_of(&body, bottom, band());
    assert!(
        matches!(got, Ok(Pcurve::General(_))),
        "the circle takes its General image, at {}: {got:?}",
        T::NAME
    );
}

#[test]
fn an_in_band_rim_certificate_takes_the_general_image_at_f64() {
    rim_image_on_the_ring::<f64>();
}

#[test]
fn an_in_band_rim_certificate_takes_the_general_image_at_interval() {
    rim_image_on_the_ring::<geom_core::Interval>();
}

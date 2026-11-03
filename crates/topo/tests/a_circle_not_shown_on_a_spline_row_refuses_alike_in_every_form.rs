//! **A circle on a spline chart that no row holds refuses "not yet",
//! alike in every description form.** Its exact image is the arc rim's
//! row, picked at the probe and read by the row's certificate; both
//! are a lane choice (D3), and a circle on a spline chart has no
//! general lane (the mint's `General` producer is degree 1, so a
//! circle's image refuses at its map wherever its foot path bends).
//! A circle the pick does not show, or whose row the certificate
//! refuses or reads in band, takes the same disposition whatever form
//! its description takes; in band, that is `NoGeneralLane`, recording
//! nothing.
//!
//! The fixture is the unit cylinder sheet over `[0, TURN] × [0, 1]`
//! (`topo::test_support::cyl_wall_sheet_keyed`), its rims the circles
//! cut by the planes at each height and described as that
//! `Intersection`, or restated through the scaffolding door, with the
//! analytic cylinder then swapped for a rational ring net in two
//! sub-arcs whose `v = 0` row is the bottom rim, its edges'
//! descriptions left naming the same key. Bending that row's second
//! sub-arc leaves the pick's probe (the edge's ends and a quarter in)
//! on it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::{Pcurve, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};
use std::sync::Arc;
use topo::construction::Construct;
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

/// The description form the bottom rim is minted in.
#[derive(Clone, Copy, Debug)]
enum Form {
    /// The cylinder cut by the rim's plane, as the sheet builds it.
    Intersection,
    /// The same circle restated through the scaffolding door, the
    /// rotation of its start about the axis.
    Scaffold,
}

/// The sheet at `T`, its bottom rim described in `form`, with its
/// cylinder swapped for [`ring`]`(push)`, and the bottom rim's
/// half-edge on the wall.
fn sheet<T: AtRestPolicy + geom_core::Bounds>(push: f64, form: Form) -> (Body<T>, HalfEdgeKey) {
    let mut body = Body::<T>::new();
    let (wall, cyl) = cyl_wall_sheet_keyed(
        &mut body,
        CylFrame::canonical(1.0),
        CylKey::Bare,
        (0.0, TURN),
        (0.0, 1.0),
        tol(),
    );
    let bottom = bottom_rim(&body, wall);
    if let Form::Scaffold = form {
        let edge = body.get_half_edge(bottom).unwrap().edge;
        let curve = body
            .get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .clone();
        let (t0, t1) = curve.params();
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(curve.carrier().clone(), t0, t1)
            .expect("the rim is a circle");
        body.set_edge_curve(edge, spec, tol())
            .expect("the rim restates through the scaffolding door");
    }
    let net = Surface::Nurbs(Arc::new(ring(push).map_scalar(T::from_f64)));
    let body = body.with_surface_moved_for_tests(cyl, net).unwrap();
    (body, bottom)
}

/// The bottom rim's half-edge on `wall`.
fn bottom_rim<T: AtRestPolicy + geom_core::Bounds>(
    body: &Body<T>,
    wall: topo::FaceKey,
) -> HalfEdgeKey {
    body.half_edges()
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
        .expect("the bottom rim bounds the wall")
}

/// What the mint derives for the bottom rim on a ring bent by `push`,
/// in `form`.
fn derived<T: AtRestPolicy + geom_core::Bounds>(
    push: f64,
    form: Form,
) -> Result<Pcurve<T>, topo::PcurveMintError> {
    let (body, bottom) = sheet::<T>(push, form);
    topo::pcurve_of(&body, bottom, band())
}

/// The row's certificate on a ring bent by `push`, the pick asserted to
/// show the row.
fn row_certificate<T: AtRestPolicy + geom_core::Bounds>(
    push: f64,
) -> Result<(), PcurveCertifyError> {
    let (body, bottom) = sheet::<T>(push, Form::Intersection);
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
        .expect("the probe shows the row");
    geom_brep::PcurveCache::certify(row, t0, t1, curve.carrier(), &surface, band()).map(|_| ())
}

fn rim_image_on_the_ring<T: AtRestPolicy + geom_core::Bounds>() {
    let no_lane = |got: &Result<Pcurve<T>, topo::PcurveMintError>| {
        matches!(
            got,
            Err(topo::PcurveMintError::Certify {
                error: PcurveCertifyError::NoGeneralLane {
                    chart: geom::SurfaceKind::Nurbs,
                    carrier: geom::CurveKind::Circle,
                    gap: geom_brep::LaneGap::Unwritten,
                },
                ..
            })
        )
    };
    let in_band_push = in_band() * (1.0 + (TURN / 4.0).cos());
    assert!(
        matches!(
            row_certificate::<T>(in_band_push),
            Err(PcurveCertifyError::Escalated { .. })
        ),
        "the bent row's certificate reads the bend in band, at {}",
        T::NAME
    );
    assert!(
        matches!(
            row_certificate::<T>(OFF_PUSH),
            Err(PcurveCertifyError::ResidualExceeded { .. })
        ),
        "the bent row's certificate refuses the bend, at {}",
        T::NAME
    );
    for form in [Form::Intersection, Form::Scaffold] {
        // The control: on the true ring the rim's image is its row.
        let got = derived::<T>(0.0, form);
        assert!(
            matches!(got, Ok(Pcurve::IsoArc { .. })),
            "{form:?}: on the true ring the rim is its row, at {}: {got:?}",
            T::NAME
        );
        // Bent in band: the certificate reads the row in band, and the
        // circle refuses "not yet".
        let near = derived::<T>(in_band_push, form);
        assert!(
            no_lane(&near),
            "{form:?}: a row read in band refuses \"not yet\", at {}: {near:?}",
            T::NAME
        );
    }
    // Bent past the band: the certificate refuses the row, and every form
    // takes the same disposition. What that disposition ends as is a
    // spline chart's missing incidence test's
    // (`work/pcert/a-spline-chart-has-no-incidence-test-so-a-lane-choice-refusal-cannot-tell-off-from-uncovered.md`),
    // so this row pins only that the forms agree.
    assert_eq!(
        derived::<T>(OFF_PUSH, Form::Intersection).err(),
        derived::<T>(OFF_PUSH, Form::Scaffold).err(),
        "a refused row ends alike in every form, at {}",
        T::NAME
    );
}

/// A bend the row's certificate refuses definitely.
const OFF_PUSH: f64 = 1e-3;

#[test]
fn a_rim_not_shown_refuses_alike_in_every_form_at_f64() {
    rim_image_on_the_ring::<f64>();
}

#[test]
fn a_rim_not_shown_refuses_alike_in_every_form_at_interval() {
    rim_image_on_the_ring::<geom_core::Interval>();
}

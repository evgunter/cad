//! **`Body::split_edge` carries a fitted-grade pcurve row** — a
//! `General` one the mint stores and a `Fitted` one a caller attaches —
//! on a SPLINE chart, at `f64` and at `Interval`.
//!
//! A fitted-grade image is a 2-D NURBS on the carrier's own parameter,
//! and a split keeps the parent's carrier and cuts only its interval
//! (`EdgeCurve::split_specs`). So each child's row is the parent's
//! image over the child's sub-interval, re-certified through the door
//! that stated it, and tier 3 reads the split body clean with no
//! caller mint.
//!
//! The fixture is `m8_4_intersection_iso`'s interior-column body: the
//! offset square prism with its flat wall restated as the plane it is,
//! the bowed wall re-charted on a `u`-widened net so the plane × wall
//! seam is an INTERIOR column, which no exact class reaches and the
//! mint stores as U2's `General` image, certified against the pair.
//! Built at each scalar through the public loft; the keys are read off
//! the `f64` build, which the `T` build reproduces (D9).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::approx::band;
use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, PcurveCache};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point2, Point3, Real, Tol, Vec3};
use profile::test_support::bulge_loop;
use std::sync::Arc;
use topo::{AtRestPolicy, Body, EdgeKey, FaceKey, FaceSurface, HalfEdgeKey, Pcurve, SurfaceKey};

/// `m8_4_intersection_iso`'s `INTERIOR_COLUMN_SCALE`: small enough that
/// the seam's certified sup fits inside every ε the matrix draws.
const SCALE: f64 = 1.0 / 1024.0;

/// The offset square prism: exactly-planar `y = ±SCALE` walls, bowed
/// `x = ±SCALE` walls.
fn prism<T: AtRestPolicy>() -> Body<T> {
    let square = || -> sweep::Section {
        let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
        vec![bulge_loop(vec![
            v(-SCALE, -SCALE),
            v(SCALE, -SCALE),
            v(SCALE, SCALE),
            v(-SCALE, SCALE),
        ])]
    };
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5 * SCALE, 0.0, SCALE)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0 * SCALE)),
    ];
    sweep::loft_body::<T>(&[square(), square(), square()], &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

fn he_surface<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> SurfaceKey {
    let lp = body
        .get_loop(body.get_half_edge(he).unwrap().parent_loop)
        .unwrap();
    body.get_face(lp.face).unwrap().surface
}

fn face_on<T: Real>(body: &Body<T>, surface: SurfaceKey) -> FaceKey {
    body.faces()
        .find(|(_, f)| f.surface == surface)
        .expect("the surface has a face")
        .0
}

/// The plane × bowed-wall seam on the `f64` build, as
/// `(edge, flat face, bowed face, half-edge on the bowed side)`.
fn seam_keys() -> (EdgeKey, FaceKey, FaceKey, HalfEdgeKey) {
    let body = prism::<f64>();
    let flat = |k| {
        matches!(body.get_surface(k), Some(Surface::Nurbs(n))
            if n.control().iter().all(|p| p.y == -SCALE))
    };
    let bowed = |k| {
        matches!(body.get_surface(k), Some(Surface::Nurbs(n))
            if n.control().iter().any(|p| p.y != -SCALE)
                && n.control().iter().any(|p| p.x.abs() == SCALE))
    };
    for (ek, e) in body.edges() {
        let spline = matches!(
            body.get_curve_geom(e.curve),
            Some(topo::CurveGeom::Certified(c)) if matches!(c.carrier(), Curve3::Nurbs(_))
        );
        let (sp, sm) = (he_surface(&body, e.he_plus), he_surface(&body, e.he_minus));
        if spline && flat(sp) && bowed(sm) {
            return (ek, face_on(&body, sp), face_on(&body, sm), e.he_minus);
        }
        if spline && flat(sm) && bowed(sp) {
            return (ek, face_on(&body, sm), face_on(&body, sp), e.he_plus);
        }
    }
    panic!("the offset square prism has a flat-wall/bowed-wall seam")
}

fn seam_carrier<T: Real>(body: &Body<T>, edge: EdgeKey) -> (Curve3<T>, T, T) {
    let Some(topo::CurveGeom::Certified(c)) =
        body.get_curve_geom(body.get_edge(edge).unwrap().curve)
    else {
        panic!("the seam's carrier is certified")
    };
    let (t0, t1) = c.params();
    (c.carrier().clone(), t0, t1)
}

/// Describes the seam as the intersection of `s1` and `s2`.
fn describe<T: AtRestPolicy>(body: &mut Body<T>, edge: EdgeKey, s1: SurfaceKey, s2: SurfaceKey) {
    let (carrier, t0, t1) = seam_carrier(body, edge);
    body.set_edge_curve(
        edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: carrier.eval((t0 + t1) * T::from_f64(0.5)),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        Tol::witness(),
    )
    .expect("the seam attaches at every ε this matrix draws");
}

/// The degree-1 `u` net continued one column each way: the face then
/// occupies `u ∈ [1, 2]` of a `[0, 3]` chart, and its seams are
/// interior columns.
fn widened<T: Real>(n: &NurbsSurface<T>) -> Surface<T> {
    let (nu, nv) = n.control_counts();
    assert_eq!((nu, n.knots_u().degree()), (2, 1), "the loft wall's u span");
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1).unwrap();
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for i in 0..4 {
        for j in 0..nv {
            let (a, b) = (n.control()[j], n.control()[nv + j]);
            control.push(match i {
                0 => a + (a - b),
                1 => a,
                2 => b,
                _ => b + (b - a),
            });
            weights.push(n.weights()[if i <= 1 { j } else { nv + j }]);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, n.knots_v().clone(), control, weights).unwrap(),
    ))
}

/// The interior-column body at rest at `T`: the seam's half on the
/// widened chart, its edge, and the chart's key.
fn general_seam_at_rest<T: AtRestPolicy>() -> (Body<T>, HalfEdgeKey, EdgeKey, SurfaceKey) {
    let (edge, flat_face, bowed_face, he) = seam_keys();
    let mut body = prism::<T>();
    let plane = body
        .set_face_surface_unvouched_for_tests(
            flat_face,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin: Point3::new(T::zero(), T::from_f64(-SCALE), T::zero()),
                    normal: Vec3::new(T::zero(), -T::one(), T::zero()),
                    u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
                },
                sense: true,
            },
        )
        .expect("the exactly-planar wall restates as a plane");
    let bowed = body.get_face(bowed_face).unwrap().surface;
    describe(&mut body, edge, plane, bowed);
    let Some(Surface::Nurbs(net)) = body.get_surface(bowed).cloned() else {
        panic!("the bowed wall is a described NURBS chart")
    };
    let chart = body
        .set_face_surface_unvouched_for_tests(
            bowed_face,
            FaceSurface::New {
                surface: widened(&net),
                sense: true,
            },
        )
        .expect("the wall takes its widened chart");
    // The description must name the chart the face now carries, or the
    // mint finds no mate.
    describe(&mut body, edge, plane, chart);
    topo::mint_pcurves(&mut body, Tol::witness()).expect("the trimmed chart mints at rest");
    assert_eq!(
        he_surface(&body, he),
        chart,
        "the f64 build's keys are the T build's"
    );
    (body, he, edge, chart)
}

/// Splits `edge` at its midpoint and asserts what the split owes the
/// fitted-grade row `parent_half` holds on `chart`: both halves on the
/// chart keep the parent's image, of the same variant, over their own
/// sub-intervals, re-certified at the fitted grade, and tier 3 reads the
/// split body clean.
fn split_carries<T: AtRestPolicy>(
    mut body: Body<T>,
    parent_half: HalfEdgeKey,
    edge: EdgeKey,
    chart: SurfaceKey,
) {
    let parent = body
        .pcurve(parent_half)
        .expect("the parent carries its row");
    let kind = match parent.pcurve() {
        Pcurve::General(_) => "General",
        Pcurve::Fitted(_) => "Fitted",
        other => panic!("the parent's row is fitted-grade: {other:?}"),
    };
    let row = |p: &PcurveCache<T>| {
        let variant = match p.pcurve() {
            Pcurve::General(_) => "General",
            Pcurve::Fitted(_) => "Fitted",
            _ => "other",
        };
        (variant, format!("{:?}", p.pcurve()))
    };
    let image = row(parent);
    let (t0, t1) = parent.params();
    let t = (t0 + t1) * T::from_f64(0.5);
    let created = body
        .split_edge(edge, t, Tol::witness())
        .unwrap_or_else(|e| panic!("{kind} @ {}: the split refused: {e:?}", T::NAME));
    let mut spans = Vec::new();
    for ek in [edge, created.new_edge] {
        let e = body.get_edge(ek).unwrap();
        for h in [e.he_plus, e.he_minus] {
            if he_surface(&body, h) != chart {
                continue;
            }
            let cache = body.pcurve(h).unwrap_or_else(|| {
                panic!("{kind} @ {}: {h:?} is rowless after the split", T::NAME)
            });
            assert_eq!(
                row(cache),
                image,
                "{kind} @ {}: {h:?} keeps the parent's image",
                T::NAME
            );
            assert!(
                cache.certificate().ssi().is_some(),
                "{kind} @ {}: {h:?} is re-certified at the fitted grade",
                T::NAME
            );
            spans.push(format!("{:?}", cache.params()));
        }
    }
    spans.sort();
    let mut want = vec![format!("{:?}", (t0, t)), format!("{:?}", (t, t1))];
    want.sort();
    assert_eq!(
        spans,
        want,
        "{kind} @ {}: each half's own sub-interval",
        T::NAME
    );
    let findings = topo::pcurves::validate_pcurves(&body, band());
    assert!(
        findings.is_empty(),
        "{kind} @ {}: tier 3 after the split: {findings:?}",
        T::NAME
    );
}

/// The minted `General` row, split.
fn general_row<T: AtRestPolicy>() {
    let (body, he, edge, chart) = general_seam_at_rest::<T>();
    assert!(
        matches!(body.pcurve(he).unwrap().pcurve(), Pcurve::General(_)),
        "the mint stores the interior column's General image"
    );
    split_carries(body, he, edge, chart);
}

/// The same image stated through `certify_fitted` and attached as a
/// caller would, split.
fn fitted_row<T: AtRestPolicy>() {
    let (mut body, he, edge, chart) = general_seam_at_rest::<T>();
    let Pcurve::General(image) = body.pcurve(he).unwrap().pcurve().clone() else {
        panic!("the mint stores the interior column's General image")
    };
    let (carrier, t0, t1) = seam_carrier(&body, edge);
    let Some(topo::CurveGeom::Certified(c)) =
        body.get_curve_geom(body.get_edge(edge).unwrap().curve)
    else {
        panic!("the seam's carrier is certified")
    };
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
        panic!("the seam is described as an intersection")
    };
    let mate = body
        .get_surface(if s1 == chart { s2 } else { s1 })
        .cloned()
        .expect("the seam names its plane mate");
    let fitted = PcurveCache::certify_fitted(
        image,
        t0,
        t1,
        &carrier,
        body.get_surface(chart).unwrap(),
        Some(&mate),
        band(),
        T::fitted_lane().expect("the scalar holds the fitted door"),
    )
    .expect("the image certifies as a Fitted row");
    body.attach_pcurve(he, fitted);
    let findings = topo::pcurves::validate_pcurves(&body, band());
    assert!(
        findings.is_empty(),
        "the attached Fitted row is at rest: {findings:?}"
    );
    split_carries(body, he, edge, chart);
}

#[test]
fn a_split_carries_the_general_row_to_each_child() {
    general_row::<f64>();
}

#[test]
fn a_split_carries_an_attached_fitted_row_to_each_child() {
    fitted_row::<f64>();
}

#[test]
fn a_split_carries_the_general_row_to_each_child_at_the_interval_scalar() {
    general_row::<geom_core::Interval>();
}

#[test]
fn a_split_carries_an_attached_fitted_row_to_each_child_at_the_interval_scalar() {
    fitted_row::<geom_core::Interval>();
}

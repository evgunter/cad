//! **A cap rim restated on its pair mints the wall's pcurve an
//! `Intersection` of it would.** `topo::pcurves`' iso derivation keys
//! its arms on the carrier, never on the description's form: an
//! `OnPair` rim takes the line-rim schedule on a line and the
//! seam-column schedule on any other carrier, as an `Intersection` does.
//! Each row lofts a polygon prism, restates a cap rim as an `OnPair` of
//! the cap's plane and the wall (certified through the plane × NURBS
//! lane's incidence), and reads the wall's image again with the stored
//! one detached. An arc rim takes the arc-rim arm, its incidence
//! certified through the very `IsoArc` the mint stored.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;
use topo::construction::Construct;

use geom::{Curve3, NurbsCurve3};
use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec, SurfacePair};
use geom_core::{Band, Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::{Body, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// A prism whose profile's second edge has `bulge` (0 a line, 1 a
/// semicircle), lofted between two copies.
fn prism(bulge: f64) -> Body<f64> {
    let v = |x: f64, y: f64, b: f64| (Point2::new(x, y), b);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, bulge),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::loft_body::<f64>(&[profile(), profile()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body
}

/// Every cap rim's half-edge on its wall, with its stored image: the
/// wall-side halves of the edges stated in a cap's chart.
fn wall_rims(body: &Body<f64>) -> Vec<(HalfEdgeKey, Pcurve<f64>)> {
    body.pcurves()
        .filter(|&(h, _)| {
            let edge = body.get_half_edge(h).unwrap().edge;
            let curve = body.get_edge(edge).unwrap().curve;
            let face = body.face_of_half_edge(h).unwrap();
            let wall = body.get_face(face).unwrap().surface;
            matches!(
                body.get_curve_geom(curve).and_then(topo::CurveGeom::certified).map(|c| c.description()),
                Some(EdgeDescription::Chart(c))
                    if c.surface != wall
                        && matches!(body.get_surface(c.surface), Some(geom::Surface::Plane { .. }))
            )
        })
        .map(|(h, c)| (h, c.pcurve().clone()))
        .collect()
}

/// Restates the edge under `h` on the pair of its two faces' surfaces,
/// as an `OnPair` or (`intersection`) an `Intersection`, its carrier
/// `carrier` (or its own).
fn restate(
    body: &mut Body<f64>,
    h: HalfEdgeKey,
    carrier: Option<(Curve3<f64>, f64, f64)>,
    intersection: bool,
) {
    let he = body.get_half_edge(h).unwrap().clone();
    let edge = body.get_edge(he.edge).unwrap().clone();
    let [a, b] = [edge.he_plus, edge.he_minus].map(|x| {
        body.get_face(body.face_of_half_edge(x).unwrap())
            .unwrap()
            .surface
    });
    let mut spec: EdgeCurveSpec<f64> = body
        .get_curve_geom(edge.curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .restated_spec();
    if let Some((c, t0, t1)) = carrier {
        (spec.carrier, spec.param_start, spec.param_end) = (c, t0, t1);
    }
    let pair = SurfacePair::new(a, b);
    spec.description = if intersection {
        EdgeDescriptionSpec::Intersection {
            pair,
            selector: geom_brep::CurveSelector::Witness(
                spec.carrier.eval(0.5 * (spec.param_start + spec.param_end)),
            ),
        }
    } else {
        EdgeDescriptionSpec::on_pair(pair)
    };
    body.set_edge_curve(he.edge, spec, tol())
        .expect("the rim certifies on its pair");
}

/// The wall's image of `h`, derived again with the stored one detached.
fn derived(body: &Body<f64>, h: HalfEdgeKey) -> String {
    let mut fresh = body.clone();
    fresh.detach_pcurve(h);
    let image =
        topo::pcurve_of(&fresh, h, Band::linear(tol()).unwrap()).expect("the image derives");
    format!("{image:?}")
}

/// A line rim on its pair takes the line-rim schedule: the same
/// `IsoLine` it minted in the cap's chart.
#[test]
fn a_line_rim_on_its_pair_mints_its_iso_line() {
    let mut body = prism(0.0);
    let rims = wall_rims(&body);
    assert!(!rims.is_empty(), "the walls' cap rims");
    for (h, stored) in rims {
        assert!(matches!(stored, Pcurve::IsoLine { .. }), "{stored:?}");
        restate(&mut body, h, None, false);
        assert_eq!(
            derived(&body, h),
            format!("{stored:?}"),
            "the line rim's image on its pair"
        );
    }
}

/// The same line carried as a degree-1 spline takes the seam-column
/// schedule, the arm for any other carrier, and derives what an
/// `Intersection` of it derives (a net outside the column's spline
/// space falls through to the derived general image on both).
#[test]
fn a_spline_carried_rim_on_its_pair_derives_what_its_intersection_does() {
    let mut on_pair = prism(0.0);
    let (h, _) = wall_rims(&on_pair).into_iter().next().expect("a rim");
    let edge = on_pair.get_half_edge(h).unwrap().edge;
    let curve = on_pair.get_edge(edge).unwrap().curve;
    let spec = on_pair
        .get_curve_geom(curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .restated_spec();
    let (t0, t1) = (spec.param_start, spec.param_end);
    let (p, q) = (spec.carrier.eval(t0), spec.carrier.eval(t1));
    let k = geom_core::spline::KnotVector::clamped(vec![t0, t0, t1, t1], 1).unwrap();
    let spline = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(k, vec![p, q], vec![1.0, 1.0]).unwrap(),
    ));
    let mut intersection = on_pair.clone();
    restate(&mut on_pair, h, Some((spline.clone(), t0, t1)), false);
    restate(&mut intersection, h, Some((spline, t0, t1)), true);
    assert_eq!(
        derived(&on_pair, h),
        derived(&intersection, h),
        "the pair's image is the intersection's"
    );
}

/// **An arc rim on its pair takes the arc-rim arm, and its certificate is
/// the stored image's.** Restated as an `OnPair`, the rim's incidence on
/// the wall is certified through the `IsoArc` the mint stored for it
/// (topo hands it to certification as stated), and re-derives that same
/// image; certified through the builder's candidate instead, with nothing
/// stated, it holds to the same bound.
#[test]
fn an_arc_rim_on_its_pair_mints_its_iso_arc_and_certifies_through_it() {
    let mut body = prism(1.0);
    let rims: Vec<_> = wall_rims(&body)
        .into_iter()
        .filter(|(_, p)| matches!(p, Pcurve::IsoArc { .. }))
        .collect();
    assert_eq!(rims.len(), 2, "the arc wall's two rims");
    for (h, stored) in rims {
        restate(&mut body, h, None, false);
        assert_eq!(
            derived(&body, h),
            format!("{stored:?}"),
            "the arc rim's image on its pair"
        );
        let edge = body.get_half_edge(h).unwrap().edge;
        let curve = body.get_edge(edge).unwrap().curve;
        let certified = body
            .get_curve_geom(curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .clone();
        let spec = certified.restated_spec();
        let (start, end) = (
            spec.carrier.eval(spec.param_start),
            spec.carrier.eval(spec.param_end),
        );
        let surfaces = |k| body.get_surface(k).cloned();
        let built = geom_brep::EdgeCurve::certify(
            spec,
            start,
            end,
            surfaces,
            1.0,
            Band::linear(tol()).unwrap(),
            geom_brep::NurbsLane::certified(),
        )
        .expect("the builder's image certifies too");
        assert_eq!(
            built.certificate().max_residual,
            certified.certificate().max_residual,
            "the stored image and the builder's certify to one bound"
        );
    }
}

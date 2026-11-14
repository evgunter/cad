use super::derive_general_image;
use geom::{NurbsCurve3, NurbsSurface};
use geom_brep::PcurveCertifyError;
use geom_core::spline::KnotVector;
use geom_core::{Dual64, Point3, Real};

#[test]
fn a_dual_general_image_refuses_before_any_check_and_says_only_that() {
    let lift = Dual64::from_f64;
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let chart = NurbsSurface::new(
        geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
        geom_core::spline::Knots::from(kv.clone()).lift(geom_core::Real::from_f64),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ],
        vec![geom_core::Real::one(); 4],
    )
    .unwrap()
    .map_scalar(lift);
    let carrier = NurbsCurve3::new(
        geom_core::spline::Knots::from(kv).lift(geom_core::Real::from_f64),
        vec![Point3::new(0.5, 0.0, 0.0), Point3::new(0.5, 1.0, 0.0)],
        vec![geom_core::Real::one(); 2],
    )
    .unwrap()
    .map_scalar(lift);
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(carrier));
    let params = (Dual64::from_f64(0.0), Dual64::from_f64(1.0));
    let err = derive_general_image(
        &carrier,
        params,
        &chart,
        geom_core::Band::linear(geom_core::Tol::witness()).unwrap(),
    )
    .unwrap_err();
    let PcurveCertifyError::FittedLaneUnsupported { scalar: "dual" } = err else {
        panic!("a dual derives no general image, and says so by name: {err:?}");
    };
    let text = err.render(geom_brep::recourse::Reading::Build);
    assert!(
        text.contains("dual scalar")
            && text.contains("certification rights")
            && !text.contains("check"),
        "the refusal names the dual and who holds the door, and claims no check ran: {text}"
    );
}

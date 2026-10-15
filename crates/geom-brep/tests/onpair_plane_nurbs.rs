//! **`OnPair` on a plane × spline pair**: incidence on both surfaces
//! through the plane × NURBS lane's limbs 1 and 2
//! ([`geom_brep::plane_nurbs_incidence`]), and nothing about how they
//! meet. The plane side is its closed-form residual at each sample; the
//! wall side the foot residual and the composite sup of
//! `|S(P(t)) − C(t)|` over the chart image.
//!
//! The fixture is M7-8's: the rational quarter-cylinder wall and a plane
//! through its `u = 0` ruling. A transverse plane certifies the ruling
//! as an `OnPair`, and so does the TANGENT plane `x = 1`, which the
//! `Intersection` lane refuses `NotTransverse`: an `OnPair` states the
//! carrier lies on both, and leaves the meeting to tier 3. An `Approx`
//! wall is metered against its fit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture::{quarter_cylinder_wall, segment, transverse_plane};
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec,
    PlaneNurbsRefusal, SurfacePair,
};
use geom_core::test_support::upper;
use geom_core::{Point3, Tol, Vec3};
use slotmap::SlotMap;

/// The `x = 1` plane, tangent to [`quarter_cylinder_wall`] along its
/// `u = 0` ruling.
fn tangent_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    }
}

/// The ruling, `(1, 0, z)` for `z ∈ [0, 1]`, displaced by `off` along
/// `x`: off the wall, and on the `y = 0` plane.
fn ruling(off: f64) -> NurbsCurve3<f64> {
    segment(
        Point3::new(1.0 + off, 0.0, 0.0),
        Point3::new(1.0 + off, 0.0, 1.0),
    )
}

/// Certifies `carrier` as an `OnPair` of `plane` and `wall` through the
/// door, with `lane`.
fn on_pair(
    plane: Surface<f64>,
    wall: Surface<f64>,
    carrier: NurbsCurve3<f64>,
    lane: Option<geom_brep::NurbsLane<f64>>,
) -> Result<EdgeCurve<f64>, CertifyError> {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(plane), arena.insert(wall));
    let ends = (carrier.eval(0.0), carrier.eval(1.0));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::on_pair(SurfacePair::new(s1, s2)),
        carrier: Curve3::Nurbs(Arc::new(carrier)),
        param_start: 0.0,
        param_end: 1.0,
    };
    EdgeCurve::certify_via(
        spec,
        ends.0,
        ends.1,
        move |k| arena.get(k).cloned(),
        band(),
        lane,
    )
}

fn wall() -> Surface<f64> {
    Surface::Nurbs(Arc::new(quarter_cylinder_wall()))
}

fn assert_on_pair(edge: &EdgeCurve<f64>, what: &str) {
    assert!(
        matches!(edge.description(), EdgeDescription::OnPair { .. }),
        "{what}: {:?}",
        edge.description()
    );
    assert!(
        edge.certificate().max_residual <= Tol::witness().get().eps,
        "{what}: the recorded residual is within ε: {:e}",
        edge.certificate().max_residual
    );
}

/// The ruling on the transverse plane certifies as an `OnPair`, its
/// recorded residual the lane's certified sup.
#[test]
fn a_ruling_on_a_transverse_plane_certifies_on_the_pair() {
    let edge = on_pair(
        transverse_plane(),
        wall(),
        ruling(0.0),
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the ruling lies on both");
    assert_on_pair(&edge, "the transverse plane");
}

/// **The smooth seam.** The tangent plane meets the wall along the
/// same ruling with parallel normals: the `Intersection` lane refuses it
/// `NotTransverse` (`m7_8_plane_nurbs_edge`), and the pair still carries
/// it, incidence being all an `OnPair` states.
#[test]
fn a_ruling_on_the_tangent_plane_certifies_on_the_pair() {
    let edge = on_pair(
        tangent_plane(),
        wall(),
        ruling(0.0),
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the ruling lies on both");
    assert_on_pair(&edge, "the tangent plane");
}

/// **The wall-side falsifier.** The ruling moved to `x = 1 + off` stays
/// exactly on the `y = 0` plane and leaves only the wall, so the lane's
/// foot residual alone refuses it, with the measured miss.
#[test]
fn a_carrier_off_the_wall_refuses_with_the_measured_miss() {
    let off = 1e3 * Tol::witness().get().eps;
    match on_pair(
        transverse_plane(),
        wall(),
        ruling(off),
        Some(geom_brep::NurbsLane::certified()),
    ) {
        Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { margin, .. })) => {
            let value = upper(margin);
            assert!(value >= 0.5 * off, "the measured miss: {value:e}");
        }
        other => panic!("a carrier {off:e} m off the wall must refuse typed: {other:?}"),
    }
}

/// With no lane in hand the pair refuses typed, before any check of the
/// edge, as the plane × NURBS `Intersection` does.
#[test]
fn the_pair_with_no_lane_refuses_typed() {
    assert!(matches!(
        on_pair(transverse_plane(), wall(), ruling(0.0), None),
        Err(CertifyError::NurbsLaneNotSupplied)
    ));
}

/// **An `Approx` wall is metered against its fit.** The flat patch
/// `x = 0` over `y, z ∈ [0, 1]` offset by `d`: its fit is the plane
/// `x = ±d` itself, and its `y = 0` side lies on the `y = 0` plane.
#[test]
fn a_side_of_an_approx_wall_certifies_on_the_pair() {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let flat = NurbsSurface::new(
        k.clone(),
        k,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    let approx =
        geom_brep::offset_fit::approx_offset_surface_at(Arc::new(flat), 0.05, 1e-6, band())
            .expect("the flat patch's offset fits");
    let side = segment(approx.eval(0.0, 0.0), approx.eval(0.0, 1.0));
    assert_eq!(
        (side.eval(0.0).y, side.eval(1.0).y),
        (0.0, 0.0),
        "the side lies on the y = 0 plane"
    );
    let edge = on_pair(
        transverse_plane(),
        approx,
        side,
        Some(geom_brep::NurbsLane::certified()),
    )
    .expect("the side lies on both");
    assert_on_pair(&edge, "the Approx wall");
}

/// Two spline charts have no incidence lane: the pair refuses as before.
#[test]
fn two_spline_charts_still_refuse_the_pair() {
    assert!(matches!(
        on_pair(
            wall(),
            Surface::Nurbs(Arc::new(quarter_cylinder_wall())),
            ruling(0.0),
            Some(geom_brep::NurbsLane::certified()),
        ),
        Err(CertifyError::Unimplemented)
    ));
}

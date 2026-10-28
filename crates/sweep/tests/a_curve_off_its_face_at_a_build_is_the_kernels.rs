//! **A curve that does not lie on the face its description names is
//! the kernel's defect at a build, declared or not** (D3, D4 ¶1 (iv)):
//! every curve-and-face pair a build door holds is the kernel's own, so a
//! definite miss there is a contradiction the kernel built, and at rest
//! the stored culprit's. Who placed the carrier (a declaration on the
//! description) is provenance and moves no ending.
//!
//! The body is a revolved hemisphere: a quarter disc about the `y` axis,
//! whose arc sweeps the sphere and leaves its meridian seam. That seam
//! is re-described through `set_edge_curve` as the straight chord
//! between its two vertices, described as a chart image of the sphere:
//! the sphere holds no line, so the mint reads it off its chart
//! (`CarrierOffChart`) before any sample is metered, and the
//! declaration's own source is never read (a dummy stands in). At rest
//! the same refusal is read through the at-rest door's window, the
//! validator's finding that carries it. Read at `f64` and at `Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{
    CertCheck, CertifyError, EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, MappedSource,
};
use geom_core::{Affine3, Bounds, Decide, Interval, Point2, Real, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use sweep::{Revolution, RevolveAxis};
use topo::construction::Construct;
use topo::{AtRestPolicy, Body, CurveGeom, EdgeKey, EulerOpError, ValidationError};

/// The unit quarter disc revolved a full turn about `y`.
fn hemisphere<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    let tol = Tol::witness();
    let quarter = bulge_loop(vec![
        (Point2::new(f(0.0), f(0.0)), f(0.0)),
        (
            Point2::new(f(1.0), f(0.0)),
            f((core::f64::consts::PI / 8.0).tan()),
        ),
        (Point2::new(f(0.0), f(1.0)), f(0.0)),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![quarter])
        .validate(tol)
        .expect("the quarter disc validates");
    let axis = RevolveAxis {
        origin: Point2::new(f(0.0), f(0.0)),
        dir: Vec2::new(f(0.0), f(1.0)),
    };
    sweep::revolve(&profile, axis, Revolution::Full, tol)
        .expect("the quarter disc revolves")
        .body
}

/// The sphere's meridian seam: a circle whose axis is horizontal.
fn seam<T: Decide + Bounds>(body: &Body<T>) -> EdgeKey {
    body.edges()
        .find(|(_, e)| {
            body.get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)
                .is_some_and(|c| match c.carrier() {
                    Curve3::Circle { axis, .. } => axis.y.hi().abs() < 0.5,
                    _ => false,
                })
        })
        .map(|(k, _)| k)
        .expect("the hemisphere's meridian seam")
}

/// The sphere's surface key.
fn sphere<T: Decide>(body: &Body<T>) -> geom_brep::SurfaceKey {
    body.faces()
        .map(|(_, f)| f.surface)
        .find(|&k| matches!(body.get_surface(k), Some(Surface::Sphere { .. })))
        .expect("the hemisphere's sphere")
}

fn dummy<T: Real>() -> MappedCurve<T> {
    MappedCurve::whole(MappedSource::ExtrudedPoint {
        point: Point2::new(T::zero(), T::zero()),
        place: Affine3::identity(),
        vec: Vec3::new(T::zero(), T::zero(), T::one()),
    })
}

/// The refusal `set_edge_curve` gives the seam's chord described on the
/// sphere, declared or not.
fn refused<T: Decide + AtRestPolicy + Bounds>(declared: bool) -> CertifyError {
    let mut body = hemisphere::<T>();
    let edge = seam(&body);
    let chart = sphere(&body);
    let curve = body
        .get_edge(edge)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap()
        .clone();
    let (t0, t1) = curve.params();
    let (start, end) = (curve.carrier().eval(t0), curve.carrier().eval(t1));
    let len = start.distance(end);
    let description = EdgeDescriptionSpec::chart(chart);
    let spec = EdgeCurveSpec {
        description: if declared {
            description.declared_by(dummy())
        } else {
            description
        },
        carrier: Curve3::Line {
            origin: start,
            dir: (end - start) / len,
        },
        param_start: T::zero(),
        param_end: len,
    };
    match body.set_edge_curve(edge, spec, Tol::witness()) {
        Err(EulerOpError::Certification { error }) => error,
        other => panic!(
            "{}: a chord off its sphere refuses at the mint: {other:?}",
            T::NAME
        ),
    }
}

fn check<T: Decide + AtRestPolicy + Bounds>() {
    let lane = T::NAME;
    let declared = refused::<T>(true);
    let derived = refused::<T>(false);
    for refusal in [&declared, &derived] {
        assert!(
            matches!(
                refusal,
                CertifyError::CarrierOffChart {
                    margin: Some(geom_brep::OffMargin::Past(_)),
                    ..
                }
            ),
            "{lane}: {refusal:?}"
        );
        assert_eq!(
            refusal.decision().map(|(check, _)| check),
            Some(CertCheck::ChartIncidence),
            "{lane}"
        );
    }
    let ending = declared.ending(Reading::Build);
    assert_eq!(
        ending.as_deref(),
        Some(geom_core::KERNEL_DEFECT_ENDING),
        "{lane}"
    );
    assert_eq!(
        ending,
        derived.ending(Reading::Build),
        "{lane}: who placed the carrier moves no ending"
    );
    // At rest, through the at-rest door: the validator's finding that
    // carries the same refusal names the stored culprit.
    for refusal in [declared, derived] {
        let at_rest = ValidationError::EdgeCertification {
            edge: EdgeKey::default(),
            error: refusal,
        }
        .to_string();
        assert!(
            at_rest.contains(topo::test_support::incidence_reasons::OFF)
                && at_rest.ends_with(geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
            "{lane}: {at_rest}"
        );
    }
}

#[test]
fn a_curve_off_its_face_at_a_build_is_the_kernels_at_f64() {
    check::<f64>();
}

#[test]
fn a_curve_off_its_face_at_a_build_is_the_kernels_at_interval() {
    check::<Interval>();
}

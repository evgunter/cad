//! **A stored face moved off its edges ends as their incidence read at
//! rest** (D3, D4 ¶1 (iv)), through the real door, [`AtRestBody::validate`].
//! Each row builds a body, moves one face's surface in place
//! ([`Body::with_surface_moved_for_tests`]: the face and its edges keep
//! naming it, so the edges are read against it rather than reported as
//! described on a surface their face left), and validates it:
//!
//! - a box's side plane moved along its normal, which the planar tier
//!   reads at its boundary and the pcurve tier at each half-edge;
//! - a cylinder's radius grown;
//! - a hemisphere's radius grown.
//!
//! Each is moved `0.5 m`, a definite miss, and to the middle of the band.
//! A definite miss ends in the stored culprit (the kernel or the file); a
//! read in band ends in the incidence lever, never in a defect and never
//! in the kernel-limit recourse. No pcurve row ends as its image's
//! residual (`MapResidual`): the read comes before any meter. The edge
//! certificate's two-surface residual (`SurfaceResidual`) is select's
//! ground and is not read here
//! (`work/select/a-two-surface-residual-meters-a-moved-face-as-the-descriptions-miss.md`).
//!
//! The planar tier reads the moved side's four corners and four edges,
//! and every one of those findings ends as its read too.
//!
//! Read at `f64` and at `Interval`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::recourse::INCIDENCE_UNSHOWN;
use geom_brep::{IncidenceRead, PcurveCertifyError};
use geom_core::{Bounds, CertifiedBounds, Decide, Interval, Point2, Tol, Vec2, Vec3};
use profile::SketchPlane;
use profile::test_support::bulge_loop;
use sweep::test_support::extruded;
use topo::{AtRestBody, AtRestPolicy, Body, PcurveMintError, ValidationError};

/// The unit disc, extruded 2 m.
fn cylinder<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(f(1.0), f(0.0)), f(1.0)),
            (Point2::new(f(-1.0), f(0.0)), f(1.0)),
        ])],
        f(2.0),
        Tol::witness(),
    )
}

/// The unit square, extruded 2 m.
fn cube<T: Decide + AtRestPolicy>() -> Body<T> {
    let f = T::from_f64;
    let corner = |x: f64, y: f64| (Point2::new(f(x), f(y)), f(0.0));
    extruded(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            corner(0.0, 0.0),
            corner(1.0, 0.0),
            corner(1.0, 1.0),
            corner(0.0, 1.0),
        ])],
        f(2.0),
        Tol::witness(),
    )
}

/// A unit quarter disc revolved about its straight side: a hemisphere.
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
    let profile = profile::Profile::new(SketchPlane::xy(), vec![quarter])
        .validate(tol)
        .unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(f(0.0), f(0.0)),
        dir: Vec2::new(f(0.0), f(1.0)),
    };
    sweep::revolve(&profile, axis, sweep::Revolution::Full, tol)
        .unwrap()
        .body
}

/// `body` with the first face whose surface `pick` selects moved by
/// `moved`, validated at rest: its findings.
fn findings<T: Decide + AtRestPolicy + Bounds + CertifiedBounds>(
    body: Body<T>,
    pick: impl Fn(&Surface<T>) -> bool,
    moved: impl Fn(Surface<T>) -> Surface<T>,
) -> Vec<ValidationError> {
    let key = body
        .faces()
        .map(|(_, f)| f.surface)
        .find(|&s| body.get_surface(s).is_some_and(&pick))
        .expect("the row's face");
    let surface = moved(body.get_surface(key).unwrap().clone());
    let body = body.with_surface_moved_for_tests(key, surface).unwrap();
    match AtRestBody::validate(body, Tol::witness()) {
        Ok(_) => panic!("{}: a moved face validates", T::NAME),
        Err(errors) => errors,
    }
}

/// How a row was moved: past the band, or to its middle.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Miss {
    Definite,
    InBand,
}

/// The incidence reads among `errors` ([`ValidationError::incidence_read`],
/// the one map), each with its rendered finding. Asserts no pcurve row
/// ends as its image's residual.
fn incidence_findings(what: &str, errors: &[ValidationError]) -> Vec<(IncidenceRead, String)> {
    let mut reads = Vec::new();
    for e in errors {
        if let ValidationError::Pcurve {
            finding: PcurveMintError::Certify { error, .. },
        } = e
        {
            assert!(
                !matches!(
                    error,
                    PcurveCertifyError::ResidualExceeded {
                        check: geom_brep::PcurveCheck::MapResidual,
                        ..
                    }
                ),
                "{what}: a stored row ends as its image's residual: {e:?}"
            );
        }
        if let Some(read) = e.incidence_read() {
            reads.push((read, e.to_string()));
        }
    }
    reads
}

fn check<T: Decide + AtRestPolicy + Bounds + CertifiedBounds>() {
    let lane = T::NAME;
    let f = T::from_f64;
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let mid = 0.5 * (band.zero() + band.escalate());
    for (miss, d) in [(Miss::Definite, 0.5), (Miss::InBand, mid)] {
        let rows: [(&str, Vec<ValidationError>); 3] = [
            (
                "a box's side plane",
                findings(
                    cube::<T>(),
                    |s| matches!(s, Surface::Plane { normal, .. } if normal.x.hi() > 0.5),
                    |s| match s {
                        Surface::Plane {
                            origin,
                            normal,
                            u_ref,
                        } => Surface::Plane {
                            origin: origin + Vec3::new(f(d), f(0.0), f(0.0)),
                            normal,
                            u_ref,
                        },
                        _ => unreachable!("picked a plane"),
                    },
                ),
            ),
            (
                "a cylinder's radius",
                findings(
                    cylinder::<T>(),
                    |s| matches!(s, Surface::Cylinder { .. }),
                    |s| match s {
                        Surface::Cylinder {
                            origin,
                            axis,
                            radius,
                            u_ref,
                        } => Surface::Cylinder {
                            origin,
                            axis,
                            radius: radius + f(d),
                            u_ref,
                        },
                        _ => unreachable!("picked a cylinder"),
                    },
                ),
            ),
            (
                "a hemisphere's radius",
                findings(
                    hemisphere::<T>(),
                    |s| matches!(s, Surface::Sphere { .. }),
                    |s| match s {
                        Surface::Sphere {
                            center,
                            radius,
                            axis,
                            u_ref,
                        } => Surface::Sphere {
                            center,
                            radius: radius + f(d),
                            axis,
                            u_ref,
                        },
                        _ => unreachable!("picked a sphere"),
                    },
                ),
            ),
        ];
        for (face, errors) in rows {
            let what = format!("{lane}, {face} moved {miss:?} ({d:e} m)");
            let reads = incidence_findings(&what, &errors);
            assert!(
                reads.len() >= 2,
                "{what}: the moved face's edges are read: {errors:#?}"
            );
            for (read, text) in &reads {
                assert_eq!(
                    read.is_definite(),
                    miss == Miss::Definite,
                    "{what}: the read is as definite as the move: {text}"
                );
                match miss {
                    Miss::Definite => assert!(
                        text.ends_with(geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
                        "{what}: a definite miss is the stored culprit: {text}"
                    ),
                    Miss::InBand => assert!(
                        text.ends_with(INCIDENCE_UNSHOWN.lever)
                            && !text.contains(geom_core::KERNEL_LIMIT_RECOURSE),
                        "{what}: a read in band ends in the incidence lever: {text}"
                    ),
                }
            }
            if face == "a box's side plane" {
                let count =
                    |is: fn(&ValidationError) -> bool| errors.iter().filter(|e| is(e)).count();
                assert_eq!(
                    count(|e| matches!(
                        e,
                        ValidationError::PlanarBoundaryResidual { .. }
                            | ValidationError::PlanarBoundaryEscalated { .. }
                    )),
                    4,
                    "{what}: the planar tier reads each of the side's four edges"
                );
                assert_eq!(
                    count(|e| matches!(
                        e,
                        ValidationError::PlanarFaceResidual { .. }
                            | ValidationError::PlanarFaceEscalated { .. }
                    )),
                    4,
                    "{what}: the planar tier reads each of the side's four corners"
                );
            }
        }
    }
}

#[test]
fn a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest_at_f64() {
    check::<f64>();
}

#[test]
fn a_face_moved_off_its_edges_ends_as_their_incidence_read_at_rest_at_interval() {
    check::<Interval>();
}

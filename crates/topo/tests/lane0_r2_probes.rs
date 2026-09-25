//! **The offset-fit seam at every scalar, through the public doors** —
//! the one-`Approx`-face seed body driven at `f64`, `Dual64`,
//! `Sym<f64>` and, behind their features, the telemetry probe and the
//! interval scalar.
//!
//! One scalar answers the seam and four do not, and the rows say what
//! that costs at each public door: the transform door and the offset
//! mint reach the seam and refuse by their own typed variant, naming
//! the scalar's lane; the two validators do NOT reach it on this
//! subject — an `mvfs` seed carries an empty loop and tier 2 refuses
//! first — which is recorded here as an assertion rather than left as
//! a thing a reader would have to run the suite to learn.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{ApproxSurface, NurbsSurface, Surface};
use geom_core::{Affine3, Band, Decide, Point3, Real, Tol, Vec3};
use topo::{Body, ContactRecords, FaceKey, FaceSurface};

/// The bowed patch the head's fixture uses, copied so the base can
/// build it too.
fn bowed_patch() -> NurbsSurface<f64> {
    const BOW: f64 = 1.5e-5;
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
            control.push(Point3::new(
                u,
                v,
                BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v,
            ));
        }
    }
    NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
}

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn bowed_offset_approx<T: Real>() -> ApproxSurface<T> {
    let minted = geom_brep::approx_offset_surface(Arc::new(bowed_patch()), 0.05, tol(), band())
        .expect("the bowed patch's offset fits at the run's eps");
    let Surface::Approx(approx) = minted else {
        panic!("the mint door produces Surface::Approx");
    };
    approx.map_scalar(T::from_f64)
}

fn seed_with<T: Decide>(surface: Surface<T>) -> (Body<T>, FaceKey) {
    let mut body = Body::<T>::new();
    let created = body
        .mvfs(Point3::new(T::zero(), T::zero(), T::zero()))
        .expect("mvfs has no preconditions");
    body.set_face_surface(created.face, FaceSurface::New(surface))
        .expect("the seed face takes a fresh surface");
    (body, created.face)
}

fn approx_seed<T: Decide>() -> (Body<T>, FaceKey) {
    seed_with(Surface::Approx(Arc::new(bowed_offset_approx::<T>())))
}

fn nurbs_seed<T: Decide>() -> (Body<T>, FaceKey) {
    seed_with(Surface::Nurbs(Arc::new(
        bowed_patch().map_scalar(T::from_f64),
    )))
}

fn turned<T: Real>() -> Affine3<T> {
    Affine3::rotation_about_axis(
        Point3::new(T::zero(), T::zero(), T::zero()),
        Vec3::new(T::zero(), T::zero(), T::from_f64(1.0)),
        T::from_f64(0.7),
    )
}

/// The public doors at one scalar. `lane` is the name the transform
/// door reports for it, or `None` where the scalar answers the seam.
fn doors_at<T: geom_core::Bounds + topo::AtRestPolicy>(label: &str, lane: Option<&str>) {
    let (body, face) = approx_seed::<T>();

    // The two validators never reach the offset-fit door on this
    // subject, at any scalar: the seed's empty loop is a tier-2
    // refusal and the walk stops there. A row that expected an
    // offset-fit refusal here would be asserting something the public
    // validator cannot produce on an `mvfs` seed.
    for (door, r) in [
        (
            "validate_geometric_structural",
            topo::validate_geometric_structural(&body, tol()),
        ),
        (
            "validate_pseudomanifold_structural",
            topo::validate_pseudomanifold_structural(&body, &ContactRecords::default(), tol()),
        ),
    ] {
        let Err(errors) = r else {
            panic!("{label} {door}: the seed body is not tier-2 valid");
        };
        assert!(
            errors
                .iter()
                .all(|e| matches!(e, topo::ValidationError::ScaffoldingEmptyLoop { .. })),
            "{label} {door}: tier 2 refuses this subject before check 1 runs, so any other \
             finding means the walk changed: {errors:?}"
        );
    }

    // The transform door DOES reach the seam.
    match topo::transform_rigid(&body, &turned::<T>(), tol()) {
        Ok(_) => assert!(
            lane.is_none(),
            "{label}: this scalar has no fit, so the map may not carry the certificate"
        ),
        Err(topo::TransformError::ApproxLaneUnsupported { lane: named }) => {
            let expected = lane.unwrap_or_else(|| {
                panic!("{label}: this scalar answers the seam, so the map must re-derive")
            });
            assert_eq!(
                named, expected,
                "{label}: the refusal names the scalar's lane"
            );
        }
        Err(other) => panic!("{label} transform_rigid: {other:?}"),
    }

    // So does the offset mint, on a NURBS-faced seed.
    let (mut nurbs, nface) = nurbs_seed::<T>();
    let minted = topo::replace_faces_offset(&mut nurbs, &[nface], T::from_f64(0.05), band(), tol());
    match minted {
        Err(topo::ReplaceFaceError::ApproxLaneUnsupported { face: f }) => {
            assert!(
                lane.is_some(),
                "{label}: this scalar answers the seam, so the mint must not report its absence"
            );
            assert_eq!(
                f, nface,
                "{label}: the refusal names the face it could not mint"
            );
        }
        other => assert!(
            lane.is_none(),
            "{label}: a scalar with no fit must report the absence, got {other:?}"
        ),
    }

    let _ = face;
}

/// The certified doors, for the scalars that may form them at all.
fn certified_doors_at<T: geom_core::CertifiedBounds + topo::AtRestPolicy>(label: &str) {
    let (body, _) = approx_seed::<T>();
    for (door, r) in [
        ("validate_geometric", topo::validate_geometric(&body, tol())),
        (
            "validate_pseudomanifold",
            topo::validate_pseudomanifold(&body, &ContactRecords::default(), tol()),
        ),
    ] {
        let Err(errors) = r else {
            panic!("{label} {door}: the seed body is not tier-2 valid");
        };
        assert!(
            errors
                .iter()
                .all(|e| matches!(e, topo::ValidationError::ScaffoldingEmptyLoop { .. })),
            "{label} {door}: tier 2 refuses this subject before check 1 runs: {errors:?}"
        );
    }
}

#[test]
fn the_f64_seam_answers_the_public_doors() {
    doors_at::<f64>("f64", None);
    certified_doors_at::<f64>("f64");
}

#[test]
fn the_dual_tier_has_no_fit_at_the_public_doors() {
    doors_at::<geom_core::Dual64>("dual64", Some("dual"));
}

#[test]
fn the_symbolic_tier_has_no_fit_at_the_public_doors() {
    doors_at::<geom_core::Sym<f64>>("sym<f64>", Some("symbolic"));
    certified_doors_at::<geom_core::Sym<f64>>("sym<f64>");
}

#[cfg(feature = "probe")]
#[test]
fn the_probe_has_no_fit_at_the_public_doors() {
    doors_at::<geom_core::Probe>("probe", Some("telemetry probe"));
    certified_doors_at::<geom_core::Probe>("probe");
}

#[test]
fn the_interval_scalar_has_no_fit_at_the_public_doors() {
    doors_at::<geom_core::interval::Interval>("interval", Some("interval"));
    certified_doors_at::<geom_core::interval::Interval>("interval");
}

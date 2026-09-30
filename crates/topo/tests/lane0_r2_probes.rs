//! **The offset-fit seam at every scalar, through the public doors** —
//! the one-`Approx`-face seed body driven at `f64`, `Dual64`,
//! `Sym<f64>` and, behind their features, the telemetry probe and the
//! interval scalar.
//!
//! One scalar answers the seam and four do not, and the rows say what
//! that costs at each public door: the transform door and the offset
//! mint reach the seam and refuse by their own typed variant, naming
//! the scalar; the two validators do NOT reach it on this
//! subject — an `mvfs` seed carries an empty loop and tier 2 refuses
//! first — which is recorded here as an assertion rather than left as
//! a thing a reader would have to run the suite to learn.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{ApproxSurface, NurbsSurface, Surface};
use geom_core::{Affine3, Decide, Point3, Real, Tol, Vec3};
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

fn bowed_offset_approx<T: Real>() -> ApproxSurface<T> {
    let approx = geom_brep::approx_offset_surface(Arc::new(bowed_patch()), 0.05, tol())
        .expect("the bowed patch's offset fits at the run's eps");
    approx.map_scalar(T::from_f64)
}

fn seed_with<T: Decide>(surface: Surface<T>) -> (Body<T>, FaceKey) {
    let mut body = Body::<T>::new();
    let created = body
        .mvfs(Point3::new(T::zero(), T::zero(), T::zero()), true)
        .expect("mvfs has no preconditions");
    body.set_face_surface(
        created.face,
        FaceSurface::New {
            surface,
            sense: true,
        },
    )
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
/// door and the mint report for it, or `None` where the scalar answers
/// the seam.
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
        Err(topo::TransformError::ApproxLaneUnsupported { scalar: named }) => {
            let expected = lane.unwrap_or_else(|| {
                panic!("{label}: this scalar answers the seam, so the map must re-derive")
            });
            assert_eq!(named, expected, "{label}: the refusal names the scalar");
        }
        Err(other) => panic!("{label} transform_rigid: {other:?}"),
    }

    // So does the offset mint, on a NURBS-faced seed.
    let (mut nurbs, nface) = nurbs_seed::<T>();
    let minted = topo::replace_faces_offset(&mut nurbs, &[nface], T::from_f64(0.05), tol());
    match minted {
        Err(topo::ReplaceFaceError::ApproxLaneUnsupported { face: f, scalar }) => {
            let expected = lane.unwrap_or_else(|| {
                panic!(
                    "{label}: this scalar answers the seam, so the mint must not report its absence"
                )
            });
            assert_eq!(
                (f, scalar),
                (nface, expected),
                "{label}: the refusal names the face it could not mint and the scalar"
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

/// **The offset-fit refusals' replay list is the policy's answer.**
/// `geom_brep::OFFSET_FIT_DOOR_HOLDERS` is what the three refusals
/// render, and geom-brep cannot read the policy, so its membership is
/// pinned here: exactly the scalars whose `offset_fit_lane()` answers
/// `Some`, the probe among them in a build that has it.
#[test]
fn the_offset_fit_replay_list_is_the_policys_holders() {
    fn holds<T: topo::AtRestPolicy>(held: &mut Vec<&'static str>) {
        if T::offset_fit_lane().is_some() {
            held.push(T::NAME);
        }
    }
    let mut held = Vec::new();
    holds::<f64>(&mut held);
    holds::<geom_core::interval::Interval>(&mut held);
    holds::<geom_core::Sym<f64>>(&mut held);
    holds::<geom_core::Sym<geom_core::interval::Interval>>(&mut held);
    holds::<geom_core::Dual64>(&mut held);
    #[cfg(feature = "probe")]
    holds::<geom_core::Probe>(&mut held);
    held.sort_unstable();
    held.dedup();
    let mut listed = geom_brep::OFFSET_FIT_DOOR_HOLDERS.to_vec();
    listed.sort_unstable();
    assert_eq!(
        listed, held,
        "the offset-fit refusals' replay list must name exactly the scalars whose policy \
         holds the door"
    );
}

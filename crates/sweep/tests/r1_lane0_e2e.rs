//! **The offset-fit door through the PUBLIC doors** — `validate_geometric`,
//! `validate_pseudomanifold`, `transform_rigid` and `replace_faces_offset`
//! over a real box body whose top cap carries a certified `Approx`
//! surface, at `f64` and, the same subject lifted, at `Interval` and at
//! the telemetry probe.
//!
//! The rows in `topo`'s own source name the door by hand, because their
//! subject is the PARAMETER. These take the seam
//! (`topo::AtRestPolicy::offset_fit_lane`) as production takes it: at
//! `f64` the `Approx` arms never fire and the mapped certificate is
//! `geom-brep`'s own measurement of the mapped pair, bit for bit; at
//! every other scalar each door refuses by its own typed variant with
//! the text that says WHICH absence it is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::Surface;
use geom_core::{Affine3, Tol, Vec3};
use topo::FaceSurface;

#[allow(unused_imports)]
use crate::common::approx::{band, box_with_approx_cap, planar_patch, pulled_back};

fn rigid_f64() -> Affine3<f64> {
    Affine3::translation(Vec3::new(1.0, 0.0, 0.0))
}

/// **At `f64` the absence arm never fires, and the map's certificate is
/// the free certifier's.**
///
/// The fixture refuses for its own reason — its cap's boundary is a
/// neighbour's chart image, so mass properties have no stored pcurve
/// cache to quadrature — and that refusal is not this unit's. What the
/// row holds is the two things that are: no door on the walk reports
/// `ApproxLaneUnsupported` or `ApproxCertification` when the seam
/// answers, and the surface `transform_rigid` produces carries
/// `geom_brep::certify_offset_over_at`'s measurement of the mapped
/// pair, limb for limb by bits — the free function, not the door
/// re-asked.
#[test]
fn the_f64_seam_answers_every_public_door() {
    let d = 0.05;
    let (body, face) = box_with_approx_cap(d, 1e-9);
    let Some(Surface::Approx(a)) = body.get_surface(body.get_face(face).unwrap().surface) else {
        panic!("the cap wears the approximating surface");
    };
    let stored = a.tolerance();

    let contacts = topo::boolean::ContactRecords::default();
    for (door, r) in [
        (
            "validate_geometric",
            topo::validate_geometric(&body, Tol::witness()),
        ),
        (
            "validate_pseudomanifold",
            topo::validate_pseudomanifold(&body, &contacts, Tol::witness()),
        ),
        (
            "validate_geometric_structural",
            topo::validate_geometric_structural(&body, Tol::witness()),
        ),
        (
            "validate_pseudomanifold_certified",
            topo::validate_pseudomanifold_certified(&body, &contacts, Tol::witness()),
        ),
    ] {
        if let Err(errors) = r {
            for e in &errors {
                assert!(
                    !matches!(
                        e,
                        topo::ValidationError::ApproxLaneUnsupported { .. }
                            | topo::ValidationError::ApproxCertification { .. }
                    ),
                    "{door}: the `f64` seam hands the pass a door, so no offset-fit arm may \
                     fire: {e:?}"
                );
            }
        }
    }

    let moved = topo::transform_rigid(&body, &rigid_f64(), Tol::witness())
        .expect("a rigid map of a certified fit re-certifies at the same tolerance");
    let Some(Surface::Approx(m)) = moved.get_surface(moved.get_face(face).unwrap().surface) else {
        panic!("the mapped cap is still approximating");
    };
    assert_eq!(
        m.tolerance().to_bits(),
        stored.to_bits(),
        "the map carries the surface's own claim, not the run's"
    );
    let spec = m.spec();
    let geom::SurfaceDescription::Offset { base, d: dm } = &spec.description;
    let free =
        geom_brep::certify_offset_over_at(base, &spec.fit, *dm, spec.window, m.tolerance(), band())
            .expect("`geom-brep`'s certifier measures the mapped pair");
    let got = m.certificate();
    for (name, x, y) in [
        ("distance", got.distance, free.distance),
        ("on_locus_max", got.on_locus_max, free.on_locus_max),
        ("hull_sup", got.hull_sup, free.hull_sup),
        ("normal_floor", got.normal_floor, free.normal_floor),
        ("curvature_reach", got.curvature_reach, free.curvature_reach),
    ] {
        assert_eq!(
            x.to_bits(),
            y.to_bits(),
            "{name}: the public door's mapped certificate is not the free certifier's \
             measurement of the same pair"
        );
    }

    // The offset door over the whole body: the mint that produces an
    // `Approx` for a kind not closed under offset. The fixture's cap
    // boundary refuses first, and that refusal is named so a change to
    // the LANE absence cannot hide behind it.
    let (mut fresh, cap) = box_with_approx_cap(d, 1e-9);
    fresh
        .set_face_surface(
            cap,
            FaceSurface::New(Surface::Nurbs(Arc::new(planar_patch(1.0)))),
        )
        .expect("the cap takes a NURBS surface");
    match topo::replace_faces_offset(&mut fresh, &[cap], 0.05, band(), Tol::witness()) {
        Ok(()) => {}
        Err(topo::ReplaceFaceError::FittedBoundaryUnsupported { .. }) => {}
        other => panic!("the `f64` mint must not report the lane's absence: {other:?}"),
    }
    let (mut single, scap) = box_with_approx_cap(d, 1e-9);
    single
        .set_face_surface(
            scap,
            FaceSurface::New(Surface::Nurbs(Arc::new(planar_patch(1.0)))),
        )
        .expect("the cap takes a NURBS surface");
    match topo::replace_face_offset(&mut single, scap, 0.05, band(), Tol::witness()) {
        Ok(()) => {}
        Err(topo::ReplaceFaceError::FittedBoundaryUnsupported { .. }) => {}
        other => panic!("the single-face `f64` mint must not report the lane's absence: {other:?}"),
    }
}

/// The certifying interval scalar: every door refuses by its own typed
/// variant, and the text says the absence is the DERIVATION's.
#[cfg(feature = "interval")]
#[test]
fn the_interval_seam_refuses_at_every_public_door() {
    use geom_core::{Bounds, Interval, Real};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};

    let approx_f64 = geom_brep::approx_offset_surface_at(
        Arc::new(pulled_back(&planar_patch(1.0), 0.05)),
        0.05,
        1e-9,
        band(),
    )
    .expect("a planar patch's offset fits");
    let Surface::Approx(a) = &approx_f64 else {
        panic!("the door mints the variant")
    };
    let lifted = a.map_scalar(Interval::from_f64);

    let iv = Interval::from_f64;
    let v = |x: f64, y: f64| ProfileVertex::new(geom_core::Point2::new(iv(x), iv(y)), iv(0.0));
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance(iv(1.0)),
        Tol::witness(),
    )
    .expect("a square prism extrudes at Interval")
    .body;
    let face = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if Bounds::lo(normal.z) > 0.5 && Bounds::lo(origin.z) > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the extruded box has a top cap");
    body.set_face_surface(face, FaceSurface::New(Surface::Approx(Arc::new(lifted))))
        .expect("the attach-layer door accepts a live face");

    let contacts = topo::boolean::ContactRecords::default();
    for (door, r) in [
        (
            "validate_geometric",
            topo::validate_geometric(&body, Tol::witness()),
        ),
        (
            "validate_pseudomanifold",
            topo::validate_pseudomanifold(&body, &contacts, Tol::witness()),
        ),
    ] {
        let Err(errors) = r else {
            panic!("{door}: the interval scalar has no fit, so the Approx face must be reported");
        };
        let found = errors.iter().find(
            |e| matches!(e, topo::ValidationError::ApproxLaneUnsupported { face: f } if *f == face),
        );
        let found =
            found.unwrap_or_else(|| panic!("{door}: the face must be reported: {errors:?}"));
        assert!(
            found.to_string().contains("no re-derivation lane"),
            "{door}: the text says which absence this is; got {found}"
        );
    }

    match topo::transform_rigid(
        &body,
        &Affine3::translation(geom_core::Vec3::new(iv(1.0), iv(0.0), iv(0.0))),
        Tol::witness(),
    ) {
        Err(topo::TransformError::ApproxLaneUnsupported { lane }) => {
            assert_eq!(lane, "interval", "the refusal names the scalar's own lane");
        }
        other => panic!("the map must refuse rather than carry the certificate: {other:?}"),
    }
}

/// The mint's absence path through the PUBLIC offset door at a scalar with
/// no fit: a NURBS cap, offset at `Interval`.
#[cfg(feature = "interval")]
#[test]
fn the_interval_mint_refuses_through_the_public_offset_door() {
    use geom_core::{Bounds, Interval, Real};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};

    let iv = Interval::from_f64;
    let v = |x: f64, y: f64| ProfileVertex::new(geom_core::Point2::new(iv(x), iv(y)), iv(0.0));
    let lp = ProfileLoop::new(vec![v(0.0, 0.0), v(2.0, 0.0), v(2.0, 2.0), v(0.0, 2.0)]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a square is a valid profile");
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance(iv(1.0)),
        Tol::witness(),
    )
    .expect("a square prism extrudes at Interval")
    .body;
    let face = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if Bounds::lo(normal.z) > 0.5 && Bounds::lo(origin.z) > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the extruded box has a top cap");
    let nurbs = planar_patch(1.0).map_scalar(Interval::from_f64);
    body.set_face_surface(face, FaceSurface::New(Surface::Nurbs(Arc::new(nurbs))))
        .expect("the attach-layer door accepts a live face");
    match topo::replace_faces_offset(
        &mut body,
        &[face],
        iv(0.05),
        geom_core::Band::linear(Tol::witness()).unwrap(),
        Tol::witness(),
    ) {
        Err(topo::ReplaceFaceError::ApproxLaneUnsupported { face: f }) => {
            assert_eq!(f, face, "the refusal names the face it could not mint");
        }
        other => panic!("the mint must refuse rather than fall back: {other:?}"),
    }
}

/// The same two absence paths at the telemetry probe scalar, through the
/// public doors. The probe IS `f64` with a sink attached, and it still
/// has no fit: the derivation is written at `f64` the TYPE.
#[cfg(feature = "probe")]
#[test]
fn the_probe_seam_refuses_at_the_map_and_the_mint() {
    use geom_core::{Probe, Real};

    let approx_f64 = geom_brep::approx_offset_surface_at(
        Arc::new(pulled_back(&planar_patch(1.0), 0.05)),
        0.05,
        1e-9,
        band(),
    )
    .expect("a planar patch's offset fits");
    let Surface::Approx(a) = &approx_f64 else {
        panic!("the door mints the variant")
    };
    let lifted = a.map_scalar(Probe::from_f64);

    let mut body = topo::Body::<Probe>::new();
    let c = body
        .mvfs(geom_core::Point3::new(
            Probe::zero(),
            Probe::zero(),
            Probe::zero(),
        ))
        .unwrap();
    body.set_face_surface(c.face, FaceSurface::New(Surface::Approx(Arc::new(lifted))))
        .unwrap();

    match topo::transform_rigid(
        &body,
        &Affine3::translation(geom_core::Vec3::new(
            Probe::from_f64(1.0),
            Probe::zero(),
            Probe::zero(),
        )),
        Tol::witness(),
    ) {
        Err(topo::TransformError::ApproxLaneUnsupported { lane }) => {
            assert_eq!(lane, "telemetry probe");
        }
        other => panic!("the map must refuse at the probe: {other:?}"),
    }

    let nurbs = planar_patch(1.0).map_scalar(Probe::from_f64);
    let mut b2 = topo::Body::<Probe>::new();
    let c2 = b2
        .mvfs(geom_core::Point3::new(
            Probe::zero(),
            Probe::zero(),
            Probe::zero(),
        ))
        .unwrap();
    b2.set_face_surface(c2.face, FaceSurface::New(Surface::Nurbs(Arc::new(nurbs))))
        .unwrap();
    match topo::replace_faces_offset(
        &mut b2,
        &[c2.face],
        Probe::from_f64(0.05),
        geom_core::Band::linear(Tol::witness()).unwrap(),
        Tol::witness(),
    ) {
        Err(topo::ReplaceFaceError::ApproxLaneUnsupported { face }) => {
            assert_eq!(face, c2.face);
        }
        other => panic!("the mint must refuse at the probe: {other:?}"),
    }
}

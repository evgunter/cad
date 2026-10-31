//! `transform_rigid` acceptance (M4 PR 2, spec D3's Transform clause):
//! key stability, exact dyadic translation, rotation validity through
//! re-certification, composition against booleans, and refusal doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use topo::construction::Construct;

use std::f64::consts::FRAC_PI_2;

use common::{brick, finished};
use geom_core::Tol;
use geom_core::{Affine3, Point3, Vec3};
use topo::{
    Body, BooleanResult, mass_properties, transform_rigid, validate, validate_closed,
    validate_geometric,
};

fn tiers_ok(b: &Body<f64>) {
    assert_eq!(validate(b), Ok(()));
    assert_eq!(validate_closed(b), Ok(()));
    assert_eq!(validate_geometric(b, Tol::witness()), Ok(()));
}

#[test]
fn translation_is_exact_and_key_stable() {
    let b = brick((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), Tol::witness());
    let keys: Vec<_> = b.points().map(|(k, _)| k).collect();
    let t = transform_rigid(
        &b,
        &geom_core::RigidMotion::translation(Vec3::new(0.25, -1.5, 8.0)).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    tiers_ok(&t);
    // Same keys, exactly translated coordinates (dyadic in, dyadic out).
    for k in keys {
        let p0 = *b.get_point(k).unwrap();
        let p1 = *t.get_point(k).unwrap();
        assert_eq!((p1.x, p1.y, p1.z), (p0.x + 0.25, p0.y - 1.5, p0.z + 8.0));
    }
    let (m0, m1) = (
        mass_properties(
            &topo::test_support::finished("the measured body", b.clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap(),
        mass_properties(
            &topo::test_support::finished("the measured body", t.clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap(),
    );
    assert_eq!(m0.enclosure.volume.to_bits(), m1.enclosure.volume.to_bits());
    assert_eq!(
        m0.enclosure.surface_area.to_bits(),
        m1.enclosure.surface_area.to_bits()
    );
}

#[test]
fn zero_angle_rotation_is_exact_identity() {
    let b = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let map =
        Affine3::rotation_about_axis(Point3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
    let t = transform_rigid(
        &b,
        &geom_core::test_support::rigid(&map, Tol::witness()),
        Tol::witness(),
    )
    .unwrap();
    for (k, p0) in b.points() {
        let p1 = t.get_point(k).unwrap();
        assert_eq!(
            (p0.x.to_bits(), p0.y.to_bits(), p0.z.to_bits()),
            (p1.x.to_bits(), p1.y.to_bits(), p1.z.to_bits())
        );
    }
}

#[test]
fn quarter_turn_recertifies_and_preserves_mass_properties_close() {
    let b = brick((0.0, 2.0), (0.0, 1.0), (0.0, 0.5), Tol::witness());
    let map = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        FRAC_PI_2,
    );
    let t = transform_rigid(
        &b,
        &geom_core::test_support::rigid(&map, Tol::witness()),
        Tol::witness(),
    )
    .unwrap();
    tiers_ok(&t);
    let (m0, m1) = (
        mass_properties(
            &topo::test_support::finished("the measured body", b.clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap(),
        mass_properties(
            &topo::test_support::finished("the measured body", t.clone(), Tol::witness()),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .unwrap(),
    );
    assert!((m0.enclosure.volume - m1.enclosure.volume).abs() < 1e-12);
    assert!((m0.enclosure.surface_area - m1.enclosure.surface_area).abs() < 1e-12);
}

#[test]
fn transformed_tool_subtracts_exactly() {
    // The M3 pocket pattern: translate a dyadic tool onto a face and
    // subtract — the moved body composes with the boolean pipeline.
    let base = finished(
        "base",
        brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let tool = brick(
        (-0.125, 0.125),
        (-0.125, 0.125),
        (0.0, 0.25),
        Tol::witness(),
    );
    // Place the tool so it embeds in the top face: pocket at (1, 1).
    let map = Affine3::translation(Vec3::new(1.0, 1.0, 1.75));
    let placed = transform_rigid(
        &tool,
        &geom_core::test_support::rigid(&map, Tol::witness()),
        Tol::witness(),
    )
    .unwrap();
    tiers_ok(&placed);
    let placed = finished("the placed tool", placed, Tol::witness());
    let out = match topo::subtract(&base, &placed, Tol::witness()).unwrap() {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("nonempty subtract"),
    };
    tiers_ok(&out);
    let vol = mass_properties(
        &out,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap()
    .met()
    .expect("the pinned resolution is met")
    .volume;
    assert_eq!(vol, 8.0 - 0.25 * 0.25 * 0.25);
}

#[test]
fn non_rigid_maps_are_refused_at_the_door() {
    // Uniform scale: affinely self-consistent on planar bodies (re-
    // certification alone would PASS it) — the decided rigidity door
    // is what refuses it.
    let scale = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
        ),
        Vec3::zero(),
    );
    // Shear: unit columns, non-orthogonal pair.
    let shear = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.5, 1.0, 0.0).normalize(),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    );
    // Reflection: orthonormal but determinant −1.
    let mirror = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    );
    for (name, m) in [("scale", scale), ("shear", shear), ("mirror", mirror)] {
        match geom_core::RigidMotion::check_rigid(
            &m,
            geom_core::Band::linear(Tol::witness()).unwrap(),
        ) {
            Err(geom_core::RigidMotionError::NotRigid { .. }) => {}
            other => panic!("{name}: expected NotRigid refusal, got {other:?}"),
        }
    }
}

// ------------------------------------------------------------------ //
// The M7-8 class: an `Intersection` between a plane and a DESCRIBED   //
// NURBS wall — the one carrier class that certifies only through the  //
// plane × NURBS lane, which `f64`'s policy holds.                     //
// ------------------------------------------------------------------ //

/// **The M7-8 cube moves through the plain door at `f64`**, and every
/// carrier re-certifies at rest after the map, the four wall edges
/// included.
#[test]
fn the_m7_8_cubes_carriers_certify_at_rest_before_and_after_a_move() {
    use crate::fixture::m7_8::{edge_findings, m7_8_cube, m7_8_edges};
    let body = m7_8_cube::<f64>();
    assert_eq!(
        edge_findings(&body),
        0,
        "the certified at-rest door re-derives every certificate"
    );
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0));
    let moved = transform_rigid(
        &body,
        &geom_core::test_support::rigid(&map, Tol::witness()),
        Tol::witness(),
    )
    .expect("an M7-8 body moves at a scalar that holds the lane");
    assert_eq!(m7_8_edges(&moved), 4, "the class survives the map");
    assert_eq!(
        edge_findings(&moved),
        0,
        "and re-derives every certificate after the map"
    );
    assert_eq!(validate_closed(&moved), Ok(()));
    let before: Vec<_> = body.points().map(|(k, _)| k).collect();
    let after: Vec<_> = moved.points().map(|(k, _)| k).collect();
    assert_eq!(before, after, "point keys are stable across the map");
}

/// **Moving a body mid-edit refuses by the edit's lever**: a lone
/// `mev_null` on a fresh face leaves a null-scaffold edge, which only
/// finishing that edit removes, so the map ends in that recourse and
/// not in a kernel defect.
#[test]
fn moving_a_body_mid_edit_asks_to_finish_the_edit() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
    body.set_face_surface(
        seed.face,
        topo::construction::FaceSurface::New {
            surface: geom::Surface::Plane {
                origin: Point3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            sense: true,
        },
    )
    .unwrap();
    body.mev_null(
        topo::construction::MevSite::Lone {
            r#loop: seed.r#loop,
        },
        topo::construction::NewVertexSide::Above,
    )
    .unwrap();
    let err = transform_rigid(
        &body,
        &geom_core::RigidMotion::translation(Vec3::new(1.0, 0.0, 0.0)).unwrap(),
        Tol::witness(),
    )
    .unwrap_err();
    assert!(
        matches!(err, topo::TransformError::NullScaffold { .. }),
        "{err:?}"
    );
    assert_eq!(
        err.to_string(),
        "the body still carries what an edit left behind, which the map cannot move. \
         Recourse: finish that edit first",
        "a build"
    );
    let topo::TransformError::NullScaffold { edge } = &err else {
        unreachable!()
    };
    let stored = format!(
        "edge {edge:?} carries a transient null-scaffold curve, which a body at rest never does. "
    );
    assert_eq!(
        err.render(geom_brep::recourse::Reading::AtRest),
        format!("{stored}{}", geom_core::KERNEL_OR_FILE_DEFECT_ENDING),
        "at rest"
    );
    assert_eq!(
        err.render(geom_core::FileCoincidence::new(1e-6)),
        format!("{stored}{}", geom_core::DAMAGED_FILE_ENDING),
        "the import door"
    );
}

//! `transform_rigid` acceptance (M4 PR 2, spec D3's Transform clause):
//! key stability, exact dyadic translation, rotation validity through
//! re-certification, composition against booleans, and refusal doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::f64::consts::FRAC_PI_2;

use common::prism_z;
use geom_core::Tol;
use geom_core::{Affine3, Point3, Vec3};
use topo::{
    Body, BooleanResult, mass_properties, transform_rigid, validate, validate_closed,
    validate_geometric,
};

/// A dyadic brick `[x0,x1]×[y0,y1]×[0,h]` (the M3 fixture builder).
fn brick(x: (f64, f64), y: (f64, f64), h: f64) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], 0.0, h).body
}

fn tiers_ok(b: &Body<f64>) {
    assert_eq!(validate(b), Ok(()));
    assert_eq!(validate_closed(b), Ok(()));
    assert_eq!(validate_geometric(b, Tol::witness()), Ok(()));
}

#[test]
fn translation_is_exact_and_key_stable() {
    let b = brick((0.0, 2.0), (0.0, 1.0), 0.5);
    let keys: Vec<_> = b.points().map(|(k, _)| k).collect();
    let t = transform_rigid(
        &b,
        &Affine3::translation(Vec3::new(0.25, -1.5, 8.0)),
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
        mass_properties(&b, Tol::witness()).unwrap(),
        mass_properties(&t, Tol::witness()).unwrap(),
    );
    assert_eq!(m0.volume.to_bits(), m1.volume.to_bits());
    assert_eq!(m0.surface_area.to_bits(), m1.surface_area.to_bits());
}

#[test]
fn zero_angle_rotation_is_exact_identity() {
    let b = brick((0.0, 1.0), (0.0, 1.0), 1.0);
    let map =
        Affine3::rotation_about_axis(Point3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
    let t = transform_rigid(&b, &map, Tol::witness()).unwrap();
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
    let b = brick((0.0, 2.0), (0.0, 1.0), 0.5);
    let map = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        FRAC_PI_2,
    );
    let t = transform_rigid(&b, &map, Tol::witness()).unwrap();
    tiers_ok(&t);
    let (m0, m1) = (
        mass_properties(&b, Tol::witness()).unwrap(),
        mass_properties(&t, Tol::witness()).unwrap(),
    );
    assert!((m0.volume - m1.volume).abs() < 1e-12);
    assert!((m0.surface_area - m1.surface_area).abs() < 1e-12);
}

#[test]
fn transformed_tool_subtracts_exactly() {
    // The M3 pocket pattern: translate a dyadic tool onto a face and
    // subtract — the moved body composes with the boolean pipeline.
    let base = brick((0.0, 2.0), (0.0, 2.0), 2.0);
    let tool = brick((-0.125, 0.125), (-0.125, 0.125), 0.25);
    // Place the tool so it embeds in the top face: pocket at (1, 1).
    let map = Affine3::translation(Vec3::new(1.0, 1.0, 1.75));
    let placed = transform_rigid(&tool, &map, Tol::witness()).unwrap();
    tiers_ok(&placed);
    let out = match topo::subtract_with(
        &base,
        &placed,
        &common::flush_declarations(&base, &placed),
        Tol::witness(),
    )
    .unwrap()
    {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("nonempty subtract"),
    };
    tiers_ok(&out);
    let vol = mass_properties(&out, Tol::witness()).unwrap().volume;
    assert_eq!(vol, 8.0 - 0.25 * 0.25 * 0.25);
}

#[test]
fn non_rigid_maps_are_refused_at_the_door() {
    let b = brick((0.0, 1.0), (0.0, 1.0), 1.0);
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
        match transform_rigid(&b, &m, Tol::witness()) {
            Err(topo::TransformError::NotRigid { .. }) => {}
            other => panic!("{name}: expected NotRigid refusal, got {other:?}"),
        }
    }
}

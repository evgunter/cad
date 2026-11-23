//! **A section arm decides a circle's radius before it states the
//! circle.** `geom::Circle::new` panics on a radius that is negative or
//! not finite, so an arm handed a surface outside its convention, or one
//! whose radius has no finite value, refuses typed first.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::intersect::{
    PlaneCylinderSection, PlaneSphereSection, SectionError, plane_cylinder_section,
    plane_sphere_section,
};
use geom_brep::{Reach, SectionRadius};
use geom_core::{Point3, Vec3};

fn cylinder_z(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        frame: geom::Frame::new(Point3::new(1.0, 2.0, 3.0), Vec3::unit_z(), Vec3::unit_x()),
        radius,
    }
}

fn plane(normal: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Plane {
        frame: geom::Frame::new(Point3::new(0.0, 0.0, 7.0), normal, u_ref),
    }
}

/// The plane square to the axis reads `‖axis×normal‖·r = −0` on a
/// cylinder of radius `−1`, which is the rim lane, and the rim is stated
/// at the stored radius.
///
/// Mutant: `plane_cylinder_classified` without its `pc_cylinder_radius`
/// guard panics in `Circle::new` at the first radius.
#[test]
fn a_plane_cylinder_rim_decides_the_cylinders_radius() {
    let reach = Reach::fixture(Point3::new(1.0, 2.0, 3.0), 1.0);
    let cuts = [
        ("square", plane(Vec3::unit_z(), Vec3::unit_x())),
        (
            "tilted",
            plane(Vec3::new(0.6, 0.0, 0.8), Vec3::new(0.8, 0.0, -0.6)),
        ),
        ("along", plane(Vec3::unit_x(), Vec3::unit_y())),
    ];
    for (pose, cut) in &cuts {
        for radius in [-1.0, 0.0] {
            let got = plane_cylinder_section(cut, &cylinder_z(radius), &reach, band());
            assert!(
                matches!(got, Err(SectionError::DegenerateOperand { .. })),
                "{pose}, radius {radius}: a radius decided not positive refuses as the \
                 operand's: {got:?}"
            );
        }
        let got = plane_cylinder_section(cut, &cylinder_z(f64::NAN), &reach, band());
        assert!(
            matches!(
                got,
                Err(SectionError::RadiusEscalated {
                    radius: SectionRadius::Cylinder,
                    ..
                })
            ),
            "{pose}: a poisoned radius is the radius guard's escalation: {got:?}"
        );
    }
    let got = plane_cylinder_section(&cuts[0].1, &cylinder_z(2.0), &reach, band());
    let Ok(PlaneCylinderSection::Rim(geom::Curve3::Circle(rim))) = got else {
        panic!("the control: a square cut of a cylinder is its rim: {got:?}");
    };
    assert_eq!(rim.radius().to_bits(), 2.0_f64.to_bits());
}

/// `ps_center_gap` decides `r − |s|` positive at an infinite radius, and
/// at `r = 1e200` the product under the root is past the float range.
///
/// Mutant: `plane_sphere_section_read` without the read of the radius'
/// width panics in `Circle::new: radius inf` at both.
#[test]
fn a_plane_sphere_circle_with_no_finite_radius_escalates() {
    let cut = plane(Vec3::unit_z(), Vec3::unit_x());
    let sphere = |radius: f64| Surface::Sphere {
        frame: geom::Frame::new(Point3::new(0.0, 0.0, 6.5), Vec3::unit_y(), Vec3::unit_x()),
        radius,
    };
    for radius in [f64::INFINITY, 1e200] {
        let got = plane_sphere_section(&cut, &sphere(radius), band());
        assert!(
            matches!(got, Err(SectionError::Escalated(_))),
            "radius {radius:e}: a circle with no finite radius is a poisoned read: {got:?}"
        );
    }
    let got = plane_sphere_section(&cut, &sphere(1.0), band());
    let Ok(PlaneSphereSection::Circle(geom::Curve3::Circle(circle))) = got else {
        panic!("the control: a plane half a radius off the centre cuts a circle: {got:?}");
    };
    assert_eq!(circle.radius().to_bits(), 0.75_f64.sqrt().to_bits());
}

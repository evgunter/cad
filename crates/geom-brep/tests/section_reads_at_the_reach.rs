//! **The section classifiers read their gap at the reach.** The
//! axis-in-plane rows of `plane_cylinder_section` and the parallel rows
//! of `cylinder_cylinder_section` read their gap at the feet of the
//! consumed extent's centre and lever the tilt from there, so a verdict
//! does not move with where a cylinder's origin is stored or with which
//! operand comes first, and the tangent-locus witness reads the same
//! rows the same way.

#![allow(clippy::panic)]

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::intersect::{
    EqualCylinderSection, PlaneCylinderSection, RadiusEvidence, cylinder_cylinder_section,
    plane_cylinder_section,
};
use geom_brep::{ExtentBall, TangentLocus, tangent_locus};
use geom_core::{Point3, Vec3};

/// Where along its axis each cylinder's origin is stored, in metres
/// from the point the fixture names.
const STORED: [f64; 5] = [0.0, 10.0, -10.0, 1000.0, -1000.0];

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// The radius-1 cylinder whose axis passes `through`, along `axis`
/// (unit), its origin stored `along` metres from there.
fn cylinder(through: Point3<f64>, axis: Vec3<f64>, along: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: through + axis * along,
        axis,
        radius: 1.0,
        u_ref: Vec3::unit_z().cross(axis).normalize(),
    }
}

/// A tilt of half the zero band off `x`, toward `toward`.
fn tilted(toward: Vec3<f64>) -> Vec3<f64> {
    let theta = 0.5 * band().zero();
    Vec3::unit_x() * theta.cos() + toward * theta.sin()
}

/// **Plane × cylinder, wherever the origin is stored.** A unit cylinder
/// resting on `z = 0` along `x`, tilted half the zero band upward, read
/// over a ball of radius ½ about the touch point at `x = 0`: the tilt
/// levered from the axis's foot (1.5 m) reads `0.75·zero`, the gap at
/// that foot reads one radius, and the section is the tangent ruling at
/// every stored origin. Read at a stored origin 1000 m back, the axis
/// stood `500·zero` low and the section answered `ParallelLines`.
///
/// The witness reads the same rows: it mints in both operand orders.
#[test]
fn plane_cylinder_reads_one_verdict_at_every_stored_origin() {
    let reach = ExtentBall::new(Point3::origin(), 0.5);
    let axis = tilted(Vec3::unit_z());
    for along in STORED {
        let cyl = cylinder(Point3::new(0.0, 0.0, 1.0), axis, along);
        match plane_cylinder_section(&plane(), &cyl, reach, band()) {
            Ok(PlaneCylinderSection::TangentLine(geom::Curve3::Line { origin, .. })) => assert!(
                origin.x.abs() < 1.0 && origin.z.abs() < band().zero(),
                "stored {along} m along: the ruling is stated beside the reach, on the plane: \
                 {origin:?}"
            ),
            other => panic!("stored {along} m along: the section is the tangent ruling: {other:?}"),
        }
        for (label, a, b) in [
            ("plane, cyl", &plane(), &cyl),
            ("cyl, plane", &cyl, &plane()),
        ] {
            match tangent_locus(a, b, reach, band()) {
                Ok(TangentLocus::Line { .. }) => {}
                other => panic!("stored {along} m along, ({label}): the witness mints: {other:?}"),
            }
        }
    }
}

/// **Two parallel cylinders, in either order, wherever either origin
/// is stored.** Radius-1 cylinders with axes 2 apart at `x = 0`, the
/// second tilted half the zero band toward the first, read over a unit
/// ball about the first's axis: the gap between the feet reads 2 and
/// the tilt is levered from the nearer foot (1 m), so the section is the
/// tangent ruling and the witness mints, in both orders and at every
/// stored origin. With the second's origin stored 1000 m along, the
/// stored-origin reading answered differently by order, and the
/// witness, levering from the second operand's foot, escalated in one
/// order and minted in the other.
#[test]
fn equal_cylinders_read_one_verdict_in_either_order_at_every_stored_origin() {
    let reach = ExtentBall::new(Point3::origin(), 1.0);
    let toward = tilted(-Vec3::unit_y());
    for along1 in STORED {
        for along2 in STORED {
            let c1 = cylinder(Point3::origin(), Vec3::unit_x(), along1);
            let c2 = cylinder(Point3::new(0.0, 2.0, 0.0), toward, along2);
            for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
                let at = format!("stored {along1} m, {along2} m along, ({label})");
                match cylinder_cylinder_section(a, b, RadiusEvidence::Declared, reach, band()) {
                    Ok(EqualCylinderSection::TangentLine(geom::Curve3::Line {
                        origin, ..
                    })) => {
                        assert!(
                            (origin - Point3::new(0.0, 1.0, 0.0)).norm() < 1e-6,
                            "{at}: the ruling is stated beside the reach: {origin:?}"
                        );
                    }
                    other => panic!("{at}: the section is the tangent ruling: {other:?}"),
                }
                match tangent_locus(a, b, reach, band()) {
                    Ok(TangentLocus::Line { .. }) => {}
                    other => panic!("{at}: the witness mints: {other:?}"),
                }
            }
        }
    }
}

/// **The tilt is levered from the foot nearer the reach, in either
/// order.** The same pair read over a unit ball about a point 5 m from
/// the first axis on the far side from the second: the nearer foot is
/// the first's at 6 m, the second's is at 8 m, and the half-zero tilt
/// reads `3·zero` from either, in band. Both orders escalate on the
/// axis row; neither reads it from a different lever than the other.
#[test]
fn the_tilt_lever_does_not_depend_on_operand_order() {
    let reach = ExtentBall::new(Point3::new(0.0, -5.0, 0.0), 1.0);
    let c1 = cylinder(Point3::origin(), Vec3::unit_x(), 1000.0);
    let c2 = cylinder(Point3::new(0.0, 2.0, 0.0), tilted(-Vec3::unit_y()), -1000.0);
    for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
        match cylinder_cylinder_section(a, b, RadiusEvidence::Declared, reach, band()) {
            Err(geom_brep::SectionError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("cc_axes_parallel"),
                "({label}): the section escalates on the axis row"
            ),
            other => panic!("({label}): the tilt levered across the reach is in band: {other:?}"),
        }
        match tangent_locus(a, b, reach, band()) {
            Err(geom_brep::TangentLocusError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("cc_axes_parallel"),
                "({label}): the witness escalates on the same row"
            ),
            other => panic!("({label}): the witness reads the same lever: {other:?}"),
        }
    }
}

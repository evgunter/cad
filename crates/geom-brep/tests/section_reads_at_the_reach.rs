//! **The section classifiers read their gap at the reach.** The
//! axis-in-plane rows of `plane_cylinder_section` and the parallel rows
//! of `cylinder_cylinder_section` read their gap at the feet of the
//! consumed extent's centre and lever the tilt from there, so a verdict
//! does not move with where a cylinder's origin is stored or with which
//! operand comes first, and the tangent-locus witness reads the same
//! rows the same way. The cone × cylinder arm reads its pose at the
//! apex, against the cylinder's own axis.

#![allow(clippy::panic)]

use crate::shared::tol::band;
use geom::Surface;
use geom_brep::intersect::{
    ConeCylinderSection, EqualCylinderSection, PlaneCylinderSection, RadiusEvidence,
    cone_cylinder_section, cylinder_cylinder_section, plane_cylinder_section,
};
use geom_brep::{ExtentBall, Reach, TangentLocus, tangent_locus};
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
        match plane_cylinder_section(&plane(), &cyl, &Reach::Ball(reach), band()) {
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
                match cylinder_cylinder_section(a, b, &Reach::Ball(reach), band()) {
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
        match cylinder_cylinder_section(a, b, &Reach::Ball(reach), band()) {
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

/// **The tilt is levered from the NEARER foot, not the farther.** The
/// pair and the ball of [`the_tilt_lever_does_not_depend_on_operand_order`]
/// (feet levers 6 m and 8 m), tilted `zero/7`: from the nearer foot the
/// tilt reads `6/7·zero`, inside the zero band, so the section is the
/// tangent ruling and the witness mints, in both orders. Pivoted on the
/// farther foot it reads `8/7·zero`, in the band, and both escalate.
/// The row above guards the other side: a lever cut to the ball's
/// radius reads its tilt as Zero.
#[test]
fn the_tilt_is_levered_from_the_nearer_foot() {
    let reach = ExtentBall::new(Point3::new(0.0, -5.0, 0.0), 1.0);
    let theta = band().zero() / 7.0;
    let toward = Vec3::unit_x() * theta.cos() - Vec3::unit_y() * theta.sin();
    let c1 = cylinder(Point3::origin(), Vec3::unit_x(), 1000.0);
    let c2 = cylinder(Point3::new(0.0, 2.0, 0.0), toward, -1000.0);
    for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
        match cylinder_cylinder_section(a, b, &Reach::Ball(reach), band()) {
            Ok(EqualCylinderSection::TangentLine(_)) => {}
            other => panic!("({label}): from the nearer foot the tilt is Zero: {other:?}"),
        }
        match tangent_locus(a, b, reach, band()) {
            Ok(TangentLocus::Line { .. }) => {}
            other => panic!("({label}): the witness reads the same lever: {other:?}"),
        }
    }
}

/// **A measured lever reaches at least to the pivot's distance from
/// where it was measured.** Two nearly coaxial unit cylinders (axes half
/// a metre apart, the second tilted half the zero band), read at a
/// point 10 m off both axes with a measured length of 1. The pivot, the
/// point's foot on an axis, stands 10 m from where the length was
/// measured, so the lever is 10: the tilt reads `5·zero`, in the band,
/// and both orders escalate. Levered at the bare length, the tilt read
/// Zero and the walls' two rulings were served.
#[test]
fn a_measured_lever_reaches_the_pivots_distance() {
    let reach = Reach::Measured {
        at: Point3::new(0.0, -10.0, 0.0),
        lever: 1.0,
    };
    let c1 = cylinder(Point3::origin(), Vec3::unit_x(), 0.0);
    let c2 = cylinder(Point3::new(0.0, 0.5, 0.0), tilted(Vec3::unit_y()), 0.0);
    for (label, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
        match cylinder_cylinder_section(a, b, &reach, band()) {
            Err(geom_brep::SectionError::Escalated(d)) => assert_eq!(
                d.predicate,
                Some("cc_axes_parallel"),
                "({label}): the section escalates on the axis row"
            ),
            other => panic!("({label}): the tilt levered at the pivot is in band: {other:?}"),
        }
    }
}

/// **Cone × cylinder is read at the apex, wherever the cylinder's origin
/// is stored.** A half-angle-0.5 cone along `x` with its apex at the
/// origin, and a radius-¼ cylinder whose axis passes through the apex
/// tilted half the zero band toward `y`, read over a unit extent from
/// the apex: the tilt reads `0.5·zero`, the apex stands on the
/// cylinder's axis, and the arm mints its two coaxial circles at
/// `±R·cot α` at every stored origin. Read at a stored origin 1000 m
/// along, the axes stood `500·zero` apart and the arm refused the pose
/// as parallel and off the axis.
#[test]
fn cone_cylinder_reads_one_verdict_at_every_stored_origin() {
    let alpha: f64 = 0.5;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_x(),
        half_angle: alpha,
        u_ref: Vec3::unit_y(),
    };
    let axis = tilted(Vec3::unit_y());
    let big_r = 0.25;
    let station = big_r / alpha.tan();
    for along in STORED {
        let cyl = Surface::Cylinder {
            origin: Point3::origin() + axis * along,
            axis,
            radius: big_r,
            u_ref: Vec3::unit_z(),
        };
        match cone_cylinder_section(&cone, &cyl, 1.0, band()) {
            Ok(ConeCylinderSection::CoaxialCircles { c1, c2 }) => {
                for (c, want) in [(c1, station), (c2, -station)] {
                    let geom::Curve3::Circle { center, .. } = c else {
                        panic!("stored {along} m along: a circle");
                    };
                    assert!(
                        (center - Point3::new(want, 0.0, 0.0)).norm() < 1e-12,
                        "stored {along} m along: a circle at the station {want}: {center:?}"
                    );
                }
            }
            other => panic!("stored {along} m along: the coaxial circles: {other:?}"),
        }
    }
}

/// The plane×cylinder section of the wall of radius `r` about `z` and
/// the plane through `through` of normal `normal`, read over `reach`.
fn wall_cut(
    r: f64,
    through: Point3<f64>,
    normal: Vec3<f64>,
    reach: &Reach<f64>,
) -> Result<PlaneCylinderSection<f64>, geom_brep::SectionError> {
    let wall = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    };
    let cut = Surface::Plane {
        origin: through,
        normal,
        u_ref: Vec3::unit_x(),
    };
    plane_cylinder_section(&cut, &wall, reach, band())
}

fn escalated(got: &Result<PlaneCylinderSection<f64>, geom_brep::SectionError>) -> bool {
    matches!(got, Err(geom_brep::SectionError::Escalated(d)) if d.predicate == Some("pc_axis_plane_parallel"))
}

fn bounded(got: &Result<PlaneCylinderSection<f64>, geom_brep::SectionError>) -> bool {
    matches!(
        got,
        Ok(PlaneCylinderSection::Rim(_) | PlaneCylinderSection::TiltedEllipse(_))
    )
}

/// **A tilt's turn across the wall is levered at the face's reach
/// across it.** The unit wall about `z` read at `(1, 0, 0)` over a face
/// that reaches nothing along the axis and `x = 1` across the wall
/// (`Reach::Face`), cut by planes through the `x` axis (no gap, so the
/// hinge stands on the foot) whose normals lean `β` off `y` toward `z`.
/// The plane turns about the hinge by `β`, which moves the face by
/// `(1 − cos β)·x`:
/// - the plane `z = 0` and planes tilted 30° and 60° off it are bounded
///   cuts, where a lever along the axis alone reads Zero and mints
///   rulings;
/// - `(1 − cos β)·x = 1.5·Kε` is definite (an ellipse): at half the
///   reach across it reads `0.75·Kε` and escalates;
/// - `(1 − cos β)·x = 0.6·Kε` escalates: at twice the reach across it
///   reads `1.2·Kε` and serves an ellipse.
#[test]
fn a_tilts_turn_across_the_wall_is_levered_at_the_faces_reach_across() {
    let at = Point3::new(1.0, 0.0, 0.0);
    let reach = Reach::Face {
        at,
        below: 0.0,
        above: 0.0,
        across: 1.0,
    };
    let leaning = |c: f64| Vec3::new(0.0, (1.0 - c * c).sqrt(), c);
    for degrees in [90.0_f64, 60.0, 30.0] {
        let got = wall_cut(1.0, at, leaning(degrees.to_radians().sin()), &reach);
        assert!(bounded(&got), "{degrees}°: a bounded cut, got {got:?}");
    }
    let kk = band().escalate();
    // `1 − cos β = m` at `sin β = √(m·(2 − m))`.
    let sine = |m: f64| (m * (2.0 - m)).sqrt();
    let definite = wall_cut(1.0, at, leaning(sine(1.5 * kk)), &reach);
    assert!(
        bounded(&definite),
        "1.5·Kε across the wall: an ellipse, got {definite:?}"
    );
    let in_band = wall_cut(1.0, at, leaning(sine(0.6 * kk)), &reach);
    assert!(
        escalated(&in_band),
        "0.6·Kε across the wall: in the band, got {in_band:?}"
    );
}

/// **A tangent cut is levered from its rulings' hinge.** A wall of radius
/// `r = 10¹²·ε` (1 km at the witness ε) read at `(r, 0, 0)` over a face
/// `e = 10⁴·ε` long (its lone ruling, 10 µm), and the plane through that
/// point tilted so the axis meets it at `sin β = k·ε/e`. The face leaves
/// the plane by at most `k·ε`, but the rulings the axis-in-plane lane
/// mints stand on the hinge through the foot's projection along the
/// plane's normal, `r·sin β` up the axis and `r·(1 − cos² β)` in from the
/// wall: the cut there is not the face's ruling, and levered from the
/// vertex's foot alone the lane read the gap at `r·cos β` and minted two
/// rulings `r·sin β` off the face (5–9.5 cm at the witness ε). Levered
/// from the hinge's station the tilt reads `r·sin² β = 10⁴·k²·ε`,
/// definite, and the cut is the exact ellipse. A tilt whose `r·sin² β`
/// is `0.6·Kε` escalates.
#[test]
fn a_tangent_cut_is_levered_from_its_rulings_hinge() {
    let eps = band().zero();
    let (r, e) = (1e12 * eps, 1e4 * eps);
    let at = Point3::new(r, 0.0, 0.0);
    let reach = Reach::Face {
        at,
        below: 0.0,
        above: e,
        across: e,
    };
    let tilted = |c: f64| Vec3::new((1.0 - c * c).sqrt(), 0.0, c);
    for k in [0.5, 0.95] {
        let got = wall_cut(r, at, tilted(k * band().zero() / e), &reach);
        assert!(bounded(&got), "k = {k}: the exact ellipse, got {got:?}");
    }
    let got = wall_cut(r, at, tilted((0.6 * band().escalate() / r).sqrt()), &reach);
    assert!(
        escalated(&got),
        "0.6·Kε at the hinge: in the band, got {got:?}"
    );
}

/// **A tangent cut turns about its rulings' hinge, not the foot.** The
/// unit wall about `z` read at `(1, 0, 0)` over a face `10·ε` long (its
/// lone ruling), cut by the plane through that point tilted to
/// `sin β = c`, `c² = 0.6·ε`. The hinge stands `r·c` up the axis from
/// the foot and on the plane, so the face moves by `c` times its axial
/// distance from the hinge's station, `r·c² = 0.6·ε`, and the turn moves
/// it by next to nothing; beside the gap the plane reads at the foot,
/// `r·(1 − cos β) = 0.3·ε`, the sum is Zero, the tangent ruling. Turned
/// about the foot, which stands `r` across the wall from the face, the
/// turn read `(1 − cos β)·r = 0.3·ε` more and escalated a cut whose
/// face it holds within the band.
#[test]
fn a_tangent_cut_turns_about_its_rulings_hinge_not_its_foot() {
    let eps = band().zero();
    let at = Point3::new(1.0, 0.0, 0.0);
    let e = 10.0 * eps;
    let reach = Reach::Face {
        at,
        below: 0.0,
        above: e,
        across: e,
    };
    let c = (0.6 * eps).sqrt();
    let got = wall_cut(1.0, at, Vec3::new((1.0 - c * c).sqrt(), 0.0, c), &reach);
    assert!(
        matches!(got, Ok(PlaneCylinderSection::TangentLine(_))),
        "the tangent ruling, got {got:?}"
    );
}

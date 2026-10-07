//! **A position and a tilt are decided as one sum.** Each section row
//! that serves a verdict on a position datum (a gap, an axis distance,
//! an apex's stand-off) beside a tilt its routing row admitted decides
//! `|datum| + tilt·lever` against the band, never the two one at a
//! time: each just inside the zero band sums to nearly twice it.
//!
//! Every pose below reads `0.6·zero` on each term: each row alone is
//! Zero, so a classifier deciding them one at a time served the
//! tangency or the coincidence, and the sum, `1.2·zero`, escalates
//! (in the band at the run's K; past it at a K under 1.2, where the
//! floor row's Zero escalates through its gate). At `0.4·zero` each,
//! the sum is inside the zero band and the verdict is served.
//!
//! Each cylinder is a quarter of the reach's lever, so a sum levered at
//! the radius would read the tilt at a quarter and serve.

#![allow(clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::intersect::{
    ConeCylinderSection, EqualCylinderSection, PlaneConeSection, PlaneCylinderSection,
    PlaneTorusSection, RadiusEvidence, cone_cylinder_section, cylinder_cylinder_section,
    plane_cone_section, plane_cylinder_section, plane_torus_section,
};
use geom_brep::{ExtentBall, Reach, SectionError, TangentLocus, TangentLocusError, tangent_locus};
use geom_core::{Point3, Vec3};

/// Each term's reading where the sum escalates, and where it is served,
/// as fractions of the zero band.
const SPLIT: f64 = 0.6;
const INSIDE: f64 = 0.4;

/// The cylinders' radius.
const R: f64 = 0.25;

fn zero() -> f64 {
    band().zero()
}

fn escalated_on<T: std::fmt::Debug>(got: &Result<T, SectionError>, row: &str) -> bool {
    let floor = format!("{row}_floor");
    matches!(got, Err(SectionError::Escalated(d))
        if d.predicate == Some(row) || d.predicate == Some(floor.as_str()))
}

/// The unit vector `cos θ·from + sin θ·toward` (both unit, ⊥).
fn turned(from: Vec3<f64>, toward: Vec3<f64>, theta: f64) -> Vec3<f64> {
    from * theta.cos() + toward * theta.sin()
}

/// The plane `z = 0` and a radius-`R` cylinder along `x`, tilted `θ`
/// toward `z`, its axis through `(0, 0, R + lift)`; read over a ball of
/// radius ½ about the origin. The tilt is levered from the foot of the
/// ball's centre, `R` above it, out to the ball's far side: `0.75`.
fn plane_and_wall(tilt: f64, lift: f64) -> Result<PlaneCylinderSection<f64>, SectionError> {
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let axis = turned(Vec3::unit_x(), Vec3::unit_z(), tilt / 0.75);
    let wall = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, R + lift),
        axis,
        radius: R,
        u_ref: Vec3::unit_y(),
    };
    let reach = Reach::Ball(ExtentBall::new(Point3::origin(), 0.5));
    plane_cylinder_section(&plane, &wall, &reach, band())
}

/// **Plane × cylinder.** The wall stands `lift` off the plane at the
/// foot, and the tilt moves it by its levered reading across the reach.
/// Served at `0.4 + 0.4`; escalated at `0.6 + 0.6`, where the tilt row
/// and the gap row each read Zero and the ruling was minted. A gap
/// definitely under the radius (`K·zero + 0.3·zero` deep) beside the
/// split tilt escalates too: the wall reads inside the band at the
/// reach's far side, where the two rulings were served.
#[test]
fn plane_cylinder_decides_the_gap_and_the_tilt_as_one_sum() {
    let z = zero();
    let inside = plane_and_wall(INSIDE * z, INSIDE * z);
    assert!(
        matches!(inside, Ok(PlaneCylinderSection::TangentLine(_))),
        "0.4 + 0.4: the tangent ruling, got {inside:?}"
    );
    let split = plane_and_wall(SPLIT * z, SPLIT * z);
    assert!(
        escalated_on(&split, "pc_parallel_gap"),
        "0.6 + 0.6: escalates on the gap read across the reach, got {split:?}"
    );
    let deep = plane_and_wall(SPLIT * z, -(band().escalate() + 0.3 * z));
    assert!(
        escalated_on(&deep, "pc_parallel_gap"),
        "a definite gap less the split tilt is in band: escalates, got {deep:?}"
    );
    let clear = plane_and_wall(SPLIT * z, -(band().escalate() + 0.7 * z));
    assert!(
        matches!(clear, Ok(PlaneCylinderSection::ParallelLines { .. })),
        "a gap clearing the band by more than the tilt: two rulings, got {clear:?}"
    );
}

/// Two radius-`R` cylinders: the first along `x` through the origin, the
/// second through `(0, 2R + apart, 0)` tilted `θ` toward `z`; read over
/// a unit ball about the point between them, `(0, R, 0)`. The tilt is
/// levered from the nearer foot: `R + 1`.
fn two_walls(tilt: f64, apart: f64, radius2: f64) -> (Surface<f64>, Surface<f64>, ExtentBall<f64>) {
    let c1 = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_x(),
        radius: R,
        u_ref: Vec3::unit_y(),
    };
    let c2 = Surface::Cylinder {
        origin: Point3::new(0.0, R + radius2 + apart, 0.0),
        axis: turned(Vec3::unit_x(), Vec3::unit_z(), tilt / (R + 1.0)),
        radius: radius2,
        u_ref: Vec3::unit_y(),
    };
    (c1, c2, ExtentBall::new(Point3::new(0.0, R, 0.0), 1.0))
}

/// **Cylinder × cylinder, and the witness.** The walls stand `apart`
/// at the feet, and the tilt moves them by its levered reading. Served
/// at `0.4 + 0.4`, the section's tangent ruling and the witness's, in
/// either order; escalated at `0.6 + 0.6`, where `cc_axes_parallel`
/// and `cc_parallel_gap` each read Zero and both minted.
#[test]
fn cylinder_pair_decides_the_gap_and_the_tilt_as_one_sum() {
    let z = zero();
    for (label, tilt, apart, served) in [
        ("0.4 + 0.4", INSIDE * z, INSIDE * z, true),
        ("0.6 + 0.6", SPLIT * z, SPLIT * z, false),
    ] {
        let (c1, c2, ball) = two_walls(tilt, apart, R);
        for (order, a, b) in [("c1, c2", &c1, &c2), ("c2, c1", &c2, &c1)] {
            let got = cylinder_cylinder_section(
                a,
                b,
                RadiusEvidence::Declared,
                &Reach::Ball(ball),
                band(),
            );
            let witness = tangent_locus(a, b, ball, band());
            if served {
                assert!(
                    matches!(got, Ok(EqualCylinderSection::TangentLine(_))),
                    "{label} ({order}): the tangent ruling, got {got:?}"
                );
                assert!(
                    matches!(witness, Ok(TangentLocus::Line { .. })),
                    "{label} ({order}): the witness mints, got {witness:?}"
                );
            } else {
                assert!(
                    escalated_on(&got, "cc_parallel_gap"),
                    "{label} ({order}): escalates on the gap, got {got:?}"
                );
                assert!(
                    matches!(&witness, Err(TangentLocusError::Escalated(d))
                        if d.predicate.is_some_and(|p| p.starts_with("cc_parallel_gap"))),
                    "{label} ({order}): the witness escalates on the same row, got {witness:?}"
                );
            }
        }
    }
}

/// **The internal tangency the witness reads alone.** A radius-`R`
/// cylinder resting inside one of radius `3R`, their axes `2R + apart`
/// apart: the internal gap `|r1 − r2| − d` reads `−apart` at the feet.
/// Served at `0.4 + 0.4`; escalated at `0.6 + 0.6`, where
/// `tangent_locus_internal_gap` read Zero beside a Zero tilt and minted.
#[test]
fn the_witness_reads_the_internal_gap_and_the_tilt_as_one_sum() {
    let z = zero();
    let (c1, c2, ball) = two_walls(INSIDE * z, INSIDE * z - 2.0 * R, 3.0 * R);
    let got = tangent_locus(&c1, &c2, ball, band());
    assert!(
        matches!(got, Ok(TangentLocus::Line { .. })),
        "0.4 + 0.4: the internal generator, got {got:?}"
    );
    let (c1, c2, ball) = two_walls(SPLIT * z, SPLIT * z - 2.0 * R, 3.0 * R);
    let got = tangent_locus(&c1, &c2, ball, band());
    assert!(
        matches!(&got, Err(TangentLocusError::Escalated(d))
            if d.predicate.is_some_and(|p| p.starts_with("tangent_locus_internal_gap"))),
        "0.6 + 0.6: escalates on the internal gap, got {got:?}"
    );
}

/// The cone of half-angle ½ along `x` with its apex at the origin, and
/// a radius-`R` cylinder tilted `θ` toward `z` whose axis passes `off`
/// from the apex along `y`, read over a unit extent from the apex.
fn cone_and_wall(tilt: f64, off: f64) -> Result<ConeCylinderSection<f64>, SectionError> {
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_x(),
        half_angle: 0.5,
        u_ref: Vec3::unit_y(),
    };
    let wall = Surface::Cylinder {
        origin: Point3::new(0.0, off, 0.0),
        axis: turned(Vec3::unit_x(), Vec3::unit_z(), tilt),
        radius: R,
        u_ref: Vec3::unit_y(),
    };
    cone_cylinder_section(&cone, &wall, 1.0, band())
}

/// **Cone × cylinder.** A coaxial circle at `R` stands off the cylinder
/// by the apex's distance from its axis plus the tilt levered over the
/// extent. Served at `0.4 + 0.4`; escalated at `0.6 + 0.6`, where
/// `coc_axes_parallel` and `coc_coaxial` each read Zero and the circles
/// were minted.
#[test]
fn cone_cylinder_decides_the_offset_and_the_tilt_as_one_sum() {
    let z = zero();
    let inside = cone_and_wall(INSIDE * z, INSIDE * z);
    assert!(
        matches!(inside, Ok(ConeCylinderSection::CoaxialCircles { .. })),
        "0.4 + 0.4: the coaxial circles, got {inside:?}"
    );
    let split = cone_and_wall(SPLIT * z, SPLIT * z);
    assert!(
        escalated_on(&split, "coc_coaxial"),
        "0.6 + 0.6: escalates on the offset read across the extent, got {split:?}"
    );
}

/// The ring torus `R = 1`, `r = R` about `z` at the origin.
fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.0,
        minor_radius: R,
        u_ref: Vec3::unit_x(),
    }
}

/// **Plane × torus, both lanes.** A plane `lift` past the tube's cap,
/// its normal tilted off the axis so the tilt reads its share at the
/// circle's radius 1: the tangent circle at `0.4 + 0.4`, escalated at
/// `0.6 + 0.6`. A plane `lift` off the axis, its normal tilted toward
/// the axis so the tilt reads its share over an extent of 2: the
/// meridian circles at `0.4 + 0.4`, escalated at `0.6 + 0.6`.
#[test]
fn plane_torus_decides_each_gap_and_its_tilt_as_one_sum() {
    let z = zero();
    let cap = |share: f64| {
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, R + share * z),
            normal: turned(Vec3::unit_z(), Vec3::unit_x(), share * z),
            u_ref: Vec3::unit_y(),
        };
        plane_torus_section(&plane, &torus(), 2.0, band())
    };
    let meridian = |share: f64| {
        let plane = Surface::Plane {
            origin: Point3::new(0.0, share * z, 0.0),
            normal: turned(Vec3::unit_y(), Vec3::unit_z(), share * z / 2.0),
            u_ref: Vec3::unit_x(),
        };
        plane_torus_section(&plane, &torus(), 2.0, band())
    };
    let got = cap(INSIDE);
    assert!(
        matches!(got, Ok(PlaneTorusSection::TangentCircle(_))),
        "cap, 0.4 + 0.4: the tangent circle, got {got:?}"
    );
    let got = cap(SPLIT);
    assert!(
        escalated_on(&got, "pt_cap_gap"),
        "cap, 0.6 + 0.6: escalates, got {got:?}"
    );
    let got = meridian(INSIDE);
    assert!(
        matches!(got, Ok(PlaneTorusSection::MeridianCircles { .. })),
        "meridian, 0.4 + 0.4: the meridian circles, got {got:?}"
    );
    let got = meridian(SPLIT);
    assert!(
        escalated_on(&got, "pt_axis_plane_gap"),
        "meridian, 0.6 + 0.6: escalates, got {got:?}"
    );
}

/// **Plane × cone's apex lane.** The cone of half-angle ½ along `z`
/// with its apex at the origin, and a plane `share·zero` off the apex
/// whose normal leans off the tangent-generator pose by `share·zero`,
/// so the discriminant reads `share·zero` over a unit extent: the
/// tangent generator at `0.4 + 0.4`, escalated at `0.6 + 0.6`, where
/// `pn_apex_on_plane` and `pn_apex_section` each read Zero.
#[test]
fn plane_cone_decides_the_apex_gap_and_the_discriminant_as_one_sum() {
    let z = zero();
    let alpha: f64 = 0.5;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    };
    let at = |share: f64| {
        let beta = core::f64::consts::FRAC_PI_2 - alpha + share * z;
        let normal = turned(Vec3::unit_z(), Vec3::unit_x(), beta);
        let plane = Surface::Plane {
            origin: Point3::origin() + normal * (share * z),
            normal,
            u_ref: Vec3::unit_y(),
        };
        plane_cone_section(&plane, &cone, 1.0, band())
    };
    let got = at(INSIDE);
    assert!(
        matches!(
            got,
            Ok(PlaneConeSection::ApexTangentLine(Curve3::Line { .. }))
        ),
        "0.4 + 0.4: the tangent generator, got {got:?}"
    );
    let got = at(SPLIT);
    assert!(
        escalated_on(&got, "pn_apex_section"),
        "0.6 + 0.6: escalates, got {got:?}"
    );
}

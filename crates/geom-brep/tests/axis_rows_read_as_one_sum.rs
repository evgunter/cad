//! **A position and a tilt are decided as one sum.** Each section row
//! that serves a verdict on a position datum (a gap, an axis distance,
//! an apex's stand-off) beside a term its routing row admitted (a tilt
//! levered at the reach, or the apex gap) reads that datum across the
//! consumed region (`decide_across` in `geom_brep::intersect`): the
//! zero side on `|datum| + swing`, each definite side on the datum shrunk
//! toward zero by the swing (the row's `_floor`).
//!
//! Every row poses its geometry so that the swing moves the served
//! object at first order: the tilt in the plane of the offset, the
//! consumed travel nearly the whole lever (the cylinders are a tenth of
//! a metre against a reach of two). So each pose's terms are real:
//!
//! - **split**, `0.9 + 0.9` zero bands: each row alone reads Zero, main
//!   served the tangency, coincidence or circles, and the served object
//!   stands about `1.7` bands off across the region; head escalates;
//! - **inside**, `0.4 + 0.4`: served;
//! - **floor**, a datum `K·zero + 0.3·zero` past the flip beside a swing
//!   of `0.6`: main served the definite side, a consumed point stands
//!   inside the band, and head escalates on the floor; at `K·zero +
//!   0.9·zero` the definite side is served.
//!
//! Where the split sum `1.8` reaches the escalation edge (a run's K at
//! or under 1.8), the far row decides it definite and the floor's Zero
//! escalates through its gate, so the split rows expect that name.

#![allow(clippy::panic, clippy::expect_used)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::intersect::{
    ConeCylinderSection, EqualCylinderSection, PlaneConeSection, PlaneCylinderSection,
    PlaneTorusSection, cone_cylinder_section, cylinder_cylinder_section, plane_cone_section,
    plane_cylinder_section, plane_torus_section,
};
use geom_brep::{ExtentBall, Reach, SectionError, TangentLocus, TangentLocusError, tangent_locus};
use geom_core::{Band, Point3, Vec3};

/// Each term's share of the zero band where the sum escalates, and where
/// it is served.
const SPLIT: f64 = 0.9;
const INSIDE: f64 = 0.4;
/// A floor pose's swing, and its datum past the escalation edge where
/// the floor escalates and where it is served.
const SWING: f64 = 0.6;
const FLOOR_IN: f64 = 0.3;
const FLOOR_OUT: f64 = 0.9;

/// The cylinders' radius, and the reach's radius about a point a radius
/// off the axis: the tilt's lever is `R + RHO`, its travel `RHO`.
const R: f64 = 0.1;
const RHO: f64 = 2.0;
const LEVER: f64 = R + RHO;

fn zero() -> f64 {
    band().zero()
}

/// The datum of a floor pose, `sign·(K + past)` zero bands.
fn past_edge(sign: f64, past: f64) -> f64 {
    sign * (band().escalate() + past * zero())
}

/// The row a split pose escalates on: the far row while its sum is in
/// the band, else the floor's gate.
fn split_row(row: &str) -> String {
    if 2.0 * SPLIT * zero() < band().escalate() {
        row.to_string()
    } else {
        format!("{row}_floor")
    }
}

fn escalated_on<T: std::fmt::Debug>(got: &Result<T, SectionError>, row: &str) -> bool {
    matches!(got, Err(SectionError::Escalated(d)) if d.predicate == Some(row))
}

fn witness_escalated_on(got: &Result<TangentLocus<f64>, TangentLocusError>, row: &str) -> bool {
    matches!(got, Err(TangentLocusError::Escalated(d)) if d.predicate == Some(row))
}

/// The unit vector `cos θ·from + sin θ·toward` (both unit, ⊥).
fn turned(from: Vec3<f64>, toward: Vec3<f64>, theta: f64) -> Vec3<f64> {
    from * theta.cos() + toward * theta.sin()
}

/// The plane `z = 0` and a radius-`R` cylinder along `x` tilted up, its
/// axis through `(0, 0, R + lift)` (the gap row's datum `−lift`), read
/// over the ball of radius `RHO` about the origin: the tilt reads
/// `tilt` at its lever `LEVER`, and lifts the far end of the reach by
/// `tilt·RHO/LEVER`.
fn plane_and_wall(
    tilt: f64,
    lift: f64,
    band: Band,
) -> Result<PlaneCylinderSection<f64>, SectionError> {
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let wall = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, R + lift),
        axis: turned(Vec3::unit_x(), Vec3::unit_z(), tilt / LEVER),
        radius: R,
        u_ref: Vec3::unit_y(),
    };
    let reach = Reach::Ball(ExtentBall::new(Point3::origin(), RHO));
    plane_cylinder_section(&plane, &wall, &reach, band)
}

/// **Plane × cylinder.** At `0.9 + 0.9` the wall stands `1.76` bands off
/// the plane at the reach's far end, where main minted the tangent
/// ruling. Both definite sides: a gap `K + 0.3` bands into the wall or
/// clear of it beside a `0.6` swing escalates on the floor (main served
/// the two rulings or nothing), `K + 0.9` serves them.
#[test]
fn plane_cylinder_decides_the_gap_and_the_tilt_as_one_sum() {
    let (z, b) = (zero(), band());
    let got = plane_and_wall(INSIDE * z, INSIDE * z, b);
    assert!(
        matches!(got, Ok(PlaneCylinderSection::TangentLine(_))),
        "inside: the tangent ruling, got {got:?}"
    );
    let got = plane_and_wall(SPLIT * z, SPLIT * z, b);
    assert!(
        escalated_on(&got, &split_row("pc_parallel_gap")),
        "split: escalates, got {got:?}"
    );
    for (side, label) in [(-1.0, "into the wall"), (1.0, "clear of it")] {
        let got = plane_and_wall(SWING * z, past_edge(side, FLOOR_IN), b);
        assert!(
            escalated_on(&got, "pc_parallel_gap_floor"),
            "{label}, K + 0.3 beside 0.6: escalates on the floor, got {got:?}"
        );
        let got = plane_and_wall(SWING * z, past_edge(side, FLOOR_OUT), b);
        let served = if side < 0.0 {
            matches!(got, Ok(PlaneCylinderSection::ParallelLines { .. }))
        } else {
            matches!(got, Ok(PlaneCylinderSection::Empty))
        };
        assert!(served, "{label}, K + 0.9 beside 0.6: served, got {got:?}");
    }
}

/// **A straddle escalates through the floor's gate.** At the run's K a
/// swing the routing row admits (under one band) cannot carry a datum
/// from inside the zero band past the escalation edge, so this row
/// pins its own band, `K = 1.2`, where it can: a datum of `0.7` beside a
/// swing of `0.6` reads `1.3` far (definite) and `0.1` on the floor
/// (Zero), and the pose is neither verdict across the reach.
#[test]
fn a_straddling_reach_escalates_on_the_floor() {
    let z = zero();
    let narrow = Band::new(z, 1.2 * z).expect("a well-ordered band");
    let got = plane_and_wall(0.6 * z, -0.7 * z, narrow);
    assert!(
        escalated_on(&got, "pc_parallel_gap_floor"),
        "a straddle escalates, got {got:?}"
    );
}

/// Two cylinders along `x`-ish: the first of radius `R` through the
/// origin, the second of radius `r2` through `(0, R + r2 + apart, 0)`,
/// turned in the plane of their offset by the tilt that reads `tilt` at
/// the lever `LEVER`; read over the ball of radius `RHO` about
/// `(0, R, 0)`.
fn two_walls(tilt: f64, apart: f64, r2: f64) -> (Surface<f64>, Surface<f64>, ExtentBall<f64>) {
    let c1 = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_x(),
        radius: R,
        u_ref: Vec3::unit_y(),
    };
    let c2 = Surface::Cylinder {
        origin: Point3::new(0.0, R + r2 + apart, 0.0),
        axis: turned(Vec3::unit_x(), Vec3::unit_y(), tilt / LEVER),
        radius: r2,
        u_ref: Vec3::unit_z(),
    };
    (c1, c2, ExtentBall::new(Point3::new(0.0, R, 0.0), RHO))
}

fn pair(
    c1: &Surface<f64>,
    c2: &Surface<f64>,
    ball: ExtentBall<f64>,
) -> Result<EqualCylinderSection<f64>, SectionError> {
    cylinder_cylinder_section(c1, c2, &Reach::Ball(ball), band())
}

/// **Cylinder × cylinder, and the witness, in either order.** At
/// `0.9 + 0.9` the walls stand `1.76` bands apart at the reach's far end
/// (the section's ruling, midway, `1.3` off the far wall; the witness's,
/// on the first wall, `1.76`), where main minted both. Both definite
/// sides escalate on the floor at `K + 0.3` and serve at `K + 0.9`.
#[test]
fn cylinder_pair_decides_the_gap_and_the_tilt_as_one_sum() {
    let z = zero();
    let orders = |(c1, c2, ball): (Surface<f64>, Surface<f64>, ExtentBall<f64>)| {
        [
            (pair(&c1, &c2, ball), tangent_locus(&c1, &c2, ball, band())),
            (pair(&c2, &c1, ball), tangent_locus(&c2, &c1, ball, band())),
        ]
    };
    for (got, witness) in orders(two_walls(INSIDE * z, INSIDE * z, R)) {
        assert!(
            matches!(got, Ok(EqualCylinderSection::TangentLine(_))),
            "inside: the tangent ruling, got {got:?}"
        );
        assert!(
            matches!(witness, Ok(TangentLocus::Line { .. })),
            "inside: the witness mints, got {witness:?}"
        );
    }
    let row = split_row("cc_parallel_gap");
    for (got, witness) in orders(two_walls(SPLIT * z, SPLIT * z, R)) {
        assert!(escalated_on(&got, &row), "split: escalates, got {got:?}");
        assert!(
            witness_escalated_on(&witness, &row),
            "split: the witness escalates on the same row, got {witness:?}"
        );
    }
    for side in [-1.0, 1.0] {
        for (got, witness) in orders(two_walls(SWING * z, past_edge(side, FLOOR_IN), R)) {
            assert!(
                escalated_on(&got, "cc_parallel_gap_floor"),
                "{side}: K + 0.3 beside 0.6 escalates on the floor, got {got:?}"
            );
            assert!(
                witness_escalated_on(&witness, "cc_parallel_gap_floor"),
                "{side}: the witness too, got {witness:?}"
            );
        }
        for (got, _) in orders(two_walls(SWING * z, past_edge(side, FLOOR_OUT), R)) {
            let served = if side < 0.0 {
                matches!(got, Ok(EqualCylinderSection::ParallelLines { .. }))
            } else {
                matches!(got, Ok(EqualCylinderSection::Empty))
            };
            assert!(served, "{side}: K + 0.9 beside 0.6 is served, got {got:?}");
        }
    }
}

/// **The coaxial row.** Two radius-`R` cylinders whose axes stand `off`
/// apart, the second turned in the plane of the offset: at `0.9 + 0.9`
/// they stand `1.76` bands apart at the reach's far end, where main
/// called them one surface. `K + 0.3` apart beside `0.6` escalates on
/// the floor (main read them apart and served two rulings); `K + 0.9`
/// serves the rulings.
#[test]
fn cylinder_pair_decides_the_coaxial_offset_and_the_tilt_as_one_sum() {
    let z = zero();
    let coaxial = |tilt: f64, off: f64| {
        let (c1, c2, ball) = two_walls(tilt, off - 2.0 * R, R);
        pair(&c1, &c2, ball)
    };
    let got = coaxial(INSIDE * z, INSIDE * z);
    assert!(
        matches!(got, Err(SectionError::CoincidentSurfaces)),
        "inside: one surface, got {got:?}"
    );
    let got = coaxial(SPLIT * z, SPLIT * z);
    assert!(
        escalated_on(&got, &split_row("cc_coaxial")),
        "split: escalates, got {got:?}"
    );
    let got = coaxial(SWING * z, past_edge(1.0, FLOOR_IN));
    assert!(
        escalated_on(&got, "cc_coaxial_floor"),
        "K + 0.3 beside 0.6: escalates on the floor, got {got:?}"
    );
    let got = coaxial(SWING * z, past_edge(1.0, FLOOR_OUT));
    assert!(
        matches!(got, Ok(EqualCylinderSection::ParallelLines { .. })),
        "K + 0.9 beside 0.6: apart, two rulings, got {got:?}"
    );
}

/// **The internal tangency the witness reads alone.** A radius-`R`
/// cylinder resting inside one of radius `3R` (`|r1 − r2| = 2R`), their
/// axes `2R + apart` apart: at `0.9 + 0.9` the walls stand `1.76` bands
/// off tangency at the reach's far end, where main minted the internal
/// generator. `K + 0.3` past either side escalates on the floor, and
/// `K + 0.9` refuses the tangency on the side it is past.
#[test]
fn the_witness_reads_the_internal_gap_and_the_tilt_as_one_sum() {
    let z = zero();
    let inner = |tilt: f64, apart: f64| {
        let (c1, c2, ball) = two_walls(tilt, apart - 2.0 * R, 3.0 * R);
        tangent_locus(&c1, &c2, ball, band())
    };
    let got = inner(INSIDE * z, INSIDE * z);
    assert!(
        matches!(got, Ok(TangentLocus::Line { .. })),
        "inside: the internal generator, got {got:?}"
    );
    let got = inner(SPLIT * z, SPLIT * z);
    assert!(
        witness_escalated_on(&got, &split_row("tangent_locus_internal_gap")),
        "split: escalates, got {got:?}"
    );
    for (side, apart) in [(1.0, false), (-1.0, true)] {
        let got = inner(SWING * z, past_edge(side, FLOOR_IN));
        assert!(
            witness_escalated_on(&got, "tangent_locus_internal_gap_floor"),
            "{side}: K + 0.3 beside 0.6 escalates on the floor, got {got:?}"
        );
        let got = inner(SWING * z, past_edge(side, FLOOR_OUT));
        assert!(
            matches!(got, Err(TangentLocusError::NotTangent { apart: a, .. }) if a == apart),
            "{side}: K + 0.9 beside 0.6 refuses the tangency, got {got:?}"
        );
    }
}

/// The cone along `x` with its apex at the origin, its half-angle set so
/// that a radius-`R` circle stands at station `0.9`, and a radius-`R`
/// cylinder turned toward `y` by the tilt that reads `tilt` over a unit
/// extent, its axis passing `off` from the apex along `y`.
fn cone_and_wall(tilt: f64, off: f64) -> Result<ConeCylinderSection<f64>, SectionError> {
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_x(),
        half_angle: (R / 0.9).atan(),
        u_ref: Vec3::unit_y(),
    };
    let wall = Surface::Cylinder {
        origin: Point3::new(0.0, off, 0.0),
        axis: turned(Vec3::unit_x(), Vec3::unit_y(), tilt),
        radius: R,
        u_ref: Vec3::unit_z(),
    };
    cone_cylinder_section(&cone, &wall, 1.0, band())
}

/// **Cone × cylinder.** At `0.8 + 0.8` a circle at station `0.9` stands
/// `0.8 + 0.72` bands off the cylinder, where main minted both circles.
/// `K + 0.3` off the axis beside `0.6` escalates on the floor (main
/// routed it to the general rung as parallel and off the axis), and
/// `K + 0.9` routes.
#[test]
fn cone_cylinder_decides_the_offset_and_the_tilt_as_one_sum() {
    let z = zero();
    let got = cone_and_wall(INSIDE * z, INSIDE * z);
    assert!(
        matches!(got, Ok(ConeCylinderSection::CoaxialCircles { .. })),
        "inside: the coaxial circles, got {got:?}"
    );
    let row = if 1.6 * z < band().escalate() {
        "coc_coaxial"
    } else {
        "coc_coaxial_floor"
    };
    let got = cone_and_wall(0.8 * z, 0.8 * z);
    assert!(escalated_on(&got, row), "0.8 + 0.8: escalates, got {got:?}");
    let got = cone_and_wall(SWING * z, past_edge(1.0, FLOOR_IN));
    assert!(
        escalated_on(&got, "coc_coaxial_floor"),
        "K + 0.3 beside 0.6: escalates on the floor, got {got:?}"
    );
    let got = cone_and_wall(SWING * z, past_edge(1.0, FLOOR_OUT));
    assert!(
        matches!(got, Err(SectionError::RoutesToGeneralRung { .. })),
        "K + 0.9 beside 0.6: parallel and off the axis, got {got:?}"
    );
}

/// The ring torus `R = 1`, `r = 4R` about `z` at the origin.
fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.0,
        minor_radius: 4.0 * R,
        u_ref: Vec3::unit_x(),
    }
}

/// **Plane × torus, the cap lane.** A plane `lift` past the tube's crest,
/// its normal tilted off the axis so the tilt reads `tilt` at the crest
/// circle's radius 1: at `0.9 + 0.9` the plane stands `1.8` bands off
/// the crest on one side, where main minted the tangent circle. Both
/// definite sides escalate on the floor at `K + 0.3` and serve at
/// `K + 0.9`.
#[test]
fn plane_torus_decides_the_cap_gap_and_the_tilt_as_one_sum() {
    let z = zero();
    let cap = |tilt: f64, lift: f64| {
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 4.0 * R + lift),
            normal: turned(Vec3::unit_z(), Vec3::unit_x(), tilt),
            u_ref: Vec3::unit_y(),
        };
        plane_torus_section(&plane, &torus(), 2.0, band())
    };
    let got = cap(INSIDE * z, INSIDE * z);
    assert!(
        matches!(got, Ok(PlaneTorusSection::TangentCircle(_))),
        "inside: the tangent circle, got {got:?}"
    );
    let got = cap(SPLIT * z, SPLIT * z);
    assert!(
        escalated_on(&got, &split_row("pt_cap_gap")),
        "split: escalates, got {got:?}"
    );
    for side in [-1.0, 1.0] {
        let got = cap(SWING * z, past_edge(side, FLOOR_IN));
        assert!(
            escalated_on(&got, "pt_cap_gap_floor"),
            "{side}: K + 0.3 beside 0.6 escalates on the floor, got {got:?}"
        );
        let got = cap(SWING * z, past_edge(side, FLOOR_OUT));
        let served = if side < 0.0 {
            matches!(got, Ok(PlaneTorusSection::ConcentricCircles { .. }))
        } else {
            matches!(got, Ok(PlaneTorusSection::Empty))
        };
        assert!(served, "{side}: K + 0.9 beside 0.6 is served, got {got:?}");
    }
}

/// A plane parallel-ish to the torus's axis, `d` off its centre along
/// `y`, its normal tilted toward the axis so the tilt reads `tilt` over
/// the extent 2.
fn axial_cut(tilt: f64, d: f64) -> Result<PlaneTorusSection<f64>, SectionError> {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, d, 0.0),
        normal: turned(Vec3::unit_y(), Vec3::unit_z(), tilt / 2.0),
        u_ref: Vec3::unit_x(),
    };
    plane_torus_section(&plane, &torus(), 2.0, band())
}

/// **Plane × torus, the axis-in-plane lane.** The meridian circles are
/// minted parallel to the plane, `d` off it, and the tilt turns them
/// about their centres' radial line, along the torus: `pt_axis_plane_gap`
/// reads `d` alone, and `0.9 + 0.9` is served. The two-oval row is
/// summed: an oval plane `K + 0.3` short of the inner equator beside a
/// `0.6` tilt escalates on the floor (main minted the two ovals), and
/// `K + 0.9` short mints them.
#[test]
fn plane_torus_axis_in_plane_reads_each_row_as_it_moves() {
    let z = zero();
    let got = axial_cut(SPLIT * z, SPLIT * z);
    assert!(
        matches!(got, Ok(PlaneTorusSection::MeridianCircles { .. })),
        "split: the meridian circles, got {got:?}"
    );
    let short_of_equator = |past: f64| (1.0 - 4.0 * R) - past_edge(1.0, past);
    let got = axial_cut(SWING * z, short_of_equator(FLOOR_IN));
    assert!(
        escalated_on(&got, "pt_spiric_two_ovals_floor"),
        "K + 0.3 beside 0.6: escalates on the floor, got {got:?}"
    );
    let got = axial_cut(SWING * z, short_of_equator(FLOOR_OUT));
    assert!(
        matches!(got, Ok(PlaneTorusSection::SpiricOvals { .. })),
        "K + 0.9 beside 0.6: the two ovals, got {got:?}"
    );
}

/// **Plane × cone's apex lane.** The cone of half-angle ½ along `z` with
/// its apex at the origin, and a plane `gap` off the apex whose normal
/// leans off the tangent-generator pose so the discriminant reads
/// `lean` over a unit extent: at `0.9 + 0.9` the tangent generator
/// stands `1.8` bands off the plane at the extent, where main minted it.
/// `K + 0.3` either side beside a `0.6` gap escalates on the floor (main
/// served the line pair or the apex point), and `K + 0.9` serves them.
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
    let at = |gap: f64, lean: f64| {
        let beta = core::f64::consts::FRAC_PI_2 - alpha + lean;
        let normal = turned(Vec3::unit_z(), Vec3::unit_x(), beta);
        let plane = Surface::Plane {
            origin: Point3::origin() + normal * gap,
            normal,
            u_ref: Vec3::unit_y(),
        };
        plane_cone_section(&plane, &cone, 1.0, band())
    };
    let got = at(INSIDE * z, INSIDE * z);
    assert!(
        matches!(
            got,
            Ok(PlaneConeSection::ApexTangentLine(Curve3::Line { .. }))
        ),
        "inside: the tangent generator, got {got:?}"
    );
    let got = at(SPLIT * z, SPLIT * z);
    assert!(
        escalated_on(&got, &split_row("pn_apex_section")),
        "split: escalates, got {got:?}"
    );
    for side in [-1.0, 1.0] {
        let got = at(SWING * z, past_edge(side, FLOOR_IN));
        assert!(
            escalated_on(&got, "pn_apex_section_floor"),
            "{side}: K + 0.3 beside 0.6 escalates on the floor, got {got:?}"
        );
        let got = at(SWING * z, past_edge(side, FLOOR_OUT));
        let served = if side > 0.0 {
            matches!(got, Ok(PlaneConeSection::ApexLinePair { .. }))
        } else {
            matches!(got, Ok(PlaneConeSection::ApexPoint(_)))
        };
        assert!(served, "{side}: K + 0.9 beside 0.6 is served, got {got:?}");
    }
}

//! **The C5 table asked about a POSE** (`route_pose`), and the aperture
//! division guards of the plane×cone arm.
//!
//! [`route`] answers per KIND pair, and every implemented arm below is
//! configuration-scoped. These rows hold `route_pose` to the arm it
//! asks, pose by pose: a pose the arm classifies keeps `implemented`, a
//! pose the arm routes to the general rung loses it and carries the
//! arm's own grounds, and an in-band pose escalates. The cylinder×
//! cylinder rows hold the one-sided reading: asked with the most
//! permissive evidence, the answer refuses only what no evidence would
//! admit.
//!
//! The aperture rows hold plane×cone to the file's BAND posture on the
//! cone's convention `α ∈ (0, π/2)` (the module docs; the posture itself
//! is the open row `the-tube-and-radius-guards-decide-on-the-band-where-
//! check-1-reads-lo`). Each was red before the guard: a cone at the
//! stored right angle minted an axis-normal circle `h·tan α ≈ 3e16`
//! metres wide, and a cone closed to `1e-300` answered its apex lane's
//! tangent generator. Both answers are the correctly rounded values for
//! the datums as stored; what the rows pin is the posture, which refuses
//! a cone whose aperture the band cannot tell from its convention's end.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::Surface;
use geom_brep::intersect::{
    PlaneConeSection, SectionError, SurfaceKind, plane_cone_section, route, route_pose,
};
use geom_core::{Point3, Vec3};

const PI_6: f64 = core::f64::consts::FRAC_PI_6;

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let normal = normal.normalize();
    let seed = if normal.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    Surface::Plane {
        origin,
        normal,
        u_ref: (seed - normal * seed.dot(normal)).normalize(),
    }
}

/// The `z`-axis cone, apex at `(0, 0, 1)`.
fn cone_z(half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        half_angle,
        u_ref: Vec3::unit_x(),
    }
}

fn cyl(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> Surface<f64> {
    let axis = axis.normalize();
    let seed = if axis.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref: (seed - axis * seed.dot(axis)).normalize(),
    }
}

/// The `y`-axis ring torus at the origin, `R = 2`, `r = 0.5`.
fn torus_y() -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_y(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// `route_pose` in both argument orders: the table is symmetric and so
/// must its pose reading be.
fn served(a: &Surface<f64>, b: &Surface<f64>) -> bool {
    let ab = route_pose(a, b, 4.0, band()).expect("the pose decides");
    let ba = route_pose(b, a, 4.0, band()).expect("the pose decides");
    assert_eq!(ab.implemented, ba.implemented, "asymmetric: {a:?} / {b:?}");
    assert_eq!(ab.rung, route(SurfaceKind::of(a), SurfaceKind::of(b)).rung);
    ab.implemented
}

/// **The configuration-scoped pairs, pose by pose.** Each pair's served
/// poses keep the kind-level answer and each refused pose loses it. A
/// gate reading the kind pair answers `true` on every row, so every
/// `false` below is a pose that gate admitted.
#[test]
fn each_scoped_arm_serves_its_own_poses_and_refuses_the_rest() {
    let cone = cone_z(PI_6);
    let apex = Point3::new(0.0, 0.0, 1.0);
    let tor = torus_y();
    let cases: Vec<(&str, Surface<f64>, &Surface<f64>, bool)> = vec![
        // plane × cone
        (
            "plane through the apex",
            plane(apex, Vec3::unit_x()),
            &cone,
            true,
        ),
        (
            "axis-normal plane",
            plane(Point3::new(0.0, 0.0, 3.0), Vec3::unit_z()),
            &cone,
            true,
        ),
        (
            "axis-parallel plane off the apex (a hyperbola)",
            plane(Point3::new(0.05, 0.0, 0.0), Vec3::unit_x()),
            &cone,
            false,
        ),
        (
            "tilted plane off the apex (an ellipse)",
            plane(Point3::new(0.0, 0.0, 3.0), Vec3::new(0.3, 0.0, 1.0)),
            &cone,
            false,
        ),
        // plane × torus
        (
            "axis-containing plane",
            plane(Point3::new(0.0, 0.0, 0.0), Vec3::unit_x()),
            &tor,
            true,
        ),
        (
            "axis-normal plane",
            plane(Point3::new(0.0, 0.2, 0.0), Vec3::unit_y()),
            &tor,
            true,
        ),
        (
            "axis-parallel plane off the axis (a spiric)",
            plane(Point3::new(0.05, 0.0, 0.0), Vec3::unit_x()),
            &tor,
            false,
        ),
        (
            "tilted plane",
            plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.3, 1.0, 0.0)),
            &tor,
            false,
        ),
    ];
    for (what, a, b, want) in &cases {
        assert_eq!(served(a, b), *want, "{what}");
    }
    // cone × cylinder: the cone first and second.
    let z = Vec3::unit_z();
    for (what, c, want) in [
        ("coaxial", cyl(apex, z, 0.5), true),
        (
            "coaxial, cylinder origin elsewhere on the axis",
            cyl(Point3::new(0.0, 0.0, -2.0), -z, 0.5),
            true,
        ),
        (
            "parallel but offset",
            cyl(Point3::new(0.1, 0.0, 1.0), z, 0.5),
            false,
        ),
        ("tilted", cyl(apex, Vec3::new(0.2, 0.0, 1.0), 0.5), false),
    ] {
        assert_eq!(served(&cone, &c), want, "cone x cylinder, {what}");
    }
}

/// **The refused pose carries the ARM's grounds**, not the kind pair's
/// note: a consumer that surfaces `note` must say why THIS pose is not
/// served, and the kind-level note says what the arm does serve.
#[test]
fn a_refused_pose_carries_the_arms_own_grounds() {
    let cone = cone_z(PI_6);
    let hyperbola = plane(Point3::new(0.05, 0.0, 0.0), Vec3::unit_x());
    let posed = route_pose(&hyperbola, &cone, 4.0, band()).unwrap();
    let kind = route(SurfaceKind::Plane, SurfaceKind::Cone);
    assert!(!posed.implemented);
    assert_ne!(posed.note, kind.note);
    let Err(SectionError::RoutesToGeneralRung { why, .. }) =
        plane_cone_section(&hyperbola, &cone, 4.0, band())
    else {
        panic!("the arm refuses this pose");
    };
    assert_eq!(posed.note, why);
}

/// **Cylinder × cylinder, read one-sidedly.** Asked with the most
/// permissive evidence: unequal radii and skew axes are refused under
/// every evidence, so they are not served; the equal-radius crossing
/// and the parallel pair are served given evidence, so the question
/// does not refuse them.
#[test]
fn cylinder_pairs_are_refused_only_where_no_evidence_admits_them() {
    let x = Vec3::unit_x();
    let z = Vec3::unit_z();
    let o = Point3::new(0.0, 0.0, 0.0);
    let base = cyl(o, z, 0.5);
    for (what, c, want) in [
        ("equal radii, crossing axes", cyl(o, x, 0.5), true),
        (
            "equal radii, parallel axes",
            cyl(Point3::new(0.7, 0.0, 0.0), z, 0.5),
            true,
        ),
        ("unequal radii, crossing axes", cyl(o, x, 0.55), false),
        (
            "equal radii, skew axes",
            cyl(Point3::new(0.3, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.5),
            false,
        ),
    ] {
        assert_eq!(served(&base, &c), want, "{what}");
    }
}

/// **Every pose of the unscoped implemented pairs is served**, and an
/// unimplemented kind pair answers the table unchanged — including a
/// pose that WOULD be the easy one (a coaxial cone×sphere).
#[test]
fn unscoped_pairs_answer_the_kind_table() {
    let cone = cone_z(PI_6);
    let tilted = plane(Point3::new(0.0, 0.0, 3.0), Vec3::new(0.3, 0.0, 1.0));
    let s = sphere(Point3::new(0.2, 0.1, 0.0), 1.0);
    let c = cyl(Point3::new(0.3, 0.0, 0.0), Vec3::new(0.1, 0.2, 1.0), 0.4);
    assert!(served(&tilted, &s));
    assert!(served(&tilted, &c));
    assert!(served(&c, &s));
    assert!(served(&s, &sphere(Point3::new(1.0, 0.0, 0.0), 0.7)));
    assert!(!served(&cone, &sphere(Point3::new(0.0, 0.0, -1.0), 0.5)));
    assert!(!served(&cone, &cone_z(0.4)));
}

/// **An in-band pose escalates**; it is neither served nor refused. The
/// axis-parallel plane `3ε` off a cone's apex is the apex lane's
/// neighbour at this ε.
#[test]
fn an_in_band_pose_escalates() {
    let cone = cone_z(PI_6);
    let near = plane(Point3::new(3.0 * eps(), 0.0, 0.0), Vec3::unit_x());
    let err = route_pose(&near, &cone, 4.0, band()).expect_err("in band");
    assert!(matches!(err, SectionError::Escalated(_)), "{err:?}");
}

/// **The axis-normal lane's `cos α` clause, decided on the band.** At
/// the stored right angle `cos α` is `6e-17`, inside the band, and the
/// arm used to mint a "circle" of radius `h·tan α ≈ 3e16` from it. Red
/// before the guard: `Ok(AxisNormalCircle)`.
#[test]
fn a_right_angled_cone_refuses_the_axis_normal_division() {
    let cone = cone_z(core::f64::consts::FRAC_PI_2);
    let cut = plane(Point3::new(0.0, 0.0, 3.0), Vec3::unit_z());
    let got = plane_cone_section(&cut, &cone, 1.0, band());
    assert!(
        matches!(got, Err(SectionError::DegenerateOperand { .. })),
        "the cos alpha division must be decided first, got {got:?}"
    );
    // The in-band twin escalates on the guard's own name.
    let cone = cone_z((3.0 * eps()).acos());
    let Err(SectionError::Escalated(diag)) = plane_cone_section(&cut, &cone, 1.0, band()) else {
        panic!("the in-band aperture must escalate");
    };
    assert_eq!(diag.predicate, Some("pn_aperture_cos"));
}

/// **The apex lane's `sin α` clause, decided on the band.** A cone
/// closed to within ε of its axis, cut by a plane through its apex,
/// used to answer the tangent-generator lane. Red before the guard:
/// `Ok(ApexTangentLine)`.
#[test]
fn a_closed_cone_refuses_the_apex_lane_division() {
    let cone = cone_z(1e-300);
    let through = plane(Point3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1e-17));
    let got = plane_cone_section(&through, &cone, 1.0, band());
    assert!(
        matches!(got, Err(SectionError::DegenerateOperand { .. })),
        "the sin alpha division must be decided first, got {got:?}"
    );
    let cone = cone_z((3.0 * eps()).asin());
    let Err(SectionError::Escalated(diag)) = plane_cone_section(&through, &cone, 1.0, band())
    else {
        panic!("the in-band aperture must escalate");
    };
    assert_eq!(diag.predicate, Some("pn_aperture_sin"));
    // A cone well inside its convention keeps its lanes.
    assert!(matches!(
        plane_cone_section(&through, &cone_z(PI_6), 1.0, band()),
        Ok(PlaneConeSection::ApexLinePair { .. })
    ));
}

/// **The pose is read over the caller's reach.** A cylinder tilted off
/// a cone's axis by `ε/10` is coaxial to within the band over a unit
/// reach, and definitely tilted over a reach of a thousand: the same
/// two surfaces are served for an edge near the apex and refused for
/// one far from it. A `route_pose` that dropped its `extent` for a
/// constant answers both the same.
#[test]
fn the_pose_is_read_over_the_callers_reach() {
    let cone = cone_z(PI_6);
    let tilt = 0.1 * eps();
    let c = cyl(Point3::new(0.0, 0.0, 1.0), Vec3::new(tilt, 0.0, 1.0), 0.5);
    let near = route_pose(&cone, &c, 1.0, band()).expect("decides at a unit reach");
    let far = route_pose(&cone, &c, 1e3, band()).expect("decides at a long reach");
    assert!(near.implemented, "within the band over a unit reach");
    assert!(!far.implemented, "definitely tilted over a long reach");
}

/// **A pose the arm cannot classify is not served.** The aperture
/// guards run ahead of plane×cone's pose trileans, so a cone whose
/// `cos α` is inside the band refuses before the tilt is ever read. A
/// gate that read that refusal as "served" would admit the tilted plane
/// — which the same arm refuses at any classifiable aperture — only
/// because the operand is degenerate.
#[test]
fn an_unclassified_pose_is_not_served() {
    let cone = cone_z((0.2 * eps()).acos());
    let tilted = plane(Point3::new(0.0, 0.0, 3.0), Vec3::new(0.3, 0.0, 1.0));
    let p = route_pose(&tilted, &cone, 1.0, band()).unwrap();
    assert!(!p.implemented, "an operand guard classified no pose");
    let control = route_pose(&tilted, &cone_z(0.5), 1.0, band()).unwrap();
    assert!(
        !control.implemented,
        "and the classifiable twin is refused too"
    );
}

/// **The aperture guard is levered by the arm's extent.** `sin α = 20ε`
/// over a unit reach is definitely open; over a reach of `0.01` it is
/// inside the band. A guard that dropped its lever answers both alike.
#[test]
fn the_aperture_guard_is_levered_by_extent() {
    let cone = cone_z(20.0 * eps());
    let through = plane(Point3::new(0.0, 0.0, 1.0), Vec3::unit_x());
    assert!(plane_cone_section(&through, &cone, 1.0, band()).is_ok());
    assert!(matches!(
        plane_cone_section(&through, &cone, 0.01, band()),
        Err(SectionError::DegenerateOperand { .. })
    ));
}

//! **The curve × face pose's rows** (`super::pose`): each class read
//! through [`super::cross`] on every arm that answers it, each rung's
//! escalation pinned to its rung, and a touch's arc against the true
//! distance. Every pose is built at the band it is read at; the cases in
//! the band's gap are aimed at its middle.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

use super::pose::{Pose, PoseRefusal, PoseRung};
use super::{CrossWhy, Crossings, cross, pose};
use crate::curve_pair::Span;

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// A depth in the middle of the band's escalation gap.
fn gap() -> f64 {
    (band().zero() + band().escalate()) / 2.0
}

fn plane(z: f64) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: Point3::new(0.3, -0.2, z),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn wall(r: f64) -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

fn sphere(r: f64) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        center: Point3::origin(),
        radius: r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// The cone of half-angle `π/4` with its apex at the origin, up `z`.
fn cone() -> geom::Surface<f64> {
    geom::Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: Vec3::unit_x(),
    }
}

/// The ring torus `(2, 0.7)` about `z`.
fn torus() -> geom::Surface<f64> {
    geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.7,
        u_ref: Vec3::unit_x(),
    }
}

/// The circle about `(0, 0, z)` square to `z`, of radius `r`.
fn rim(r: f64, z: f64) -> Curve3<f64> {
    Curve3::circle(Point3::new(0.0, 0.0, z), Vec3::unit_z(), r, Vec3::unit_x())
}

fn line(origin: [f64; 3], dir: [f64; 3]) -> Curve3<f64> {
    Curve3::line(Point3::from_array(origin), Vec3::from_array(dir))
}

/// The span of `c` over the whole turn, or over `(t0, t1)` of a line.
fn whole(c: &Curve3<f64>) -> Span<'_, f64> {
    match c {
        Curve3::Line(_) => Span::new(c, (-3.0, 3.0)),
        _ => Span::new(c, (0.0, TAU)),
    }
}

fn read(
    c: &Curve3<f64>,
    s: &geom::Surface<f64>,
) -> Result<Crossings<f64>, super::CrossRefusal<f64>> {
    cross(&whole(c), s, band())
}

/// The class `cross` answered, as a word, or the rung it escalated on.
fn class(c: &Curve3<f64>, s: &geom::Surface<f64>) -> String {
    match read(c, s) {
        Ok(found) => match found.pose {
            Pose::Lies => "Lies".into(),
            Pose::Apart => "Apart".into(),
            Pose::Tangent(_) => "Tangent".into(),
            Pose::Through(_) => "Through".into(),
            Pose::Isolated(_) => format!("Isolated({})", found.meetings.len()),
        },
        Err(refusal) => match refusal.windows[0].why {
            CrossWhy::Pose(PoseRefusal::InBand { rung, .. }) => format!("InBand({rung:?})"),
            other => format!("{other:?}"),
        },
    }
}

/// **`Lies`, on every arm that has it**: a circle in a plane, a wall's
/// ruling (its drift and offset both in the zero band), a coaxial circle
/// on a wall and on a sphere, a torus's rim and meridian, a cone's
/// generator.
#[test]
fn every_arm_reads_a_span_lying_on_its_surface() {
    let r = (0.7_f64.powi(2) + 0.0_f64).sqrt();
    let meridian = Curve3::circle(
        Point3::new(2.0, 0.0, 0.0),
        Vec3::unit_y(),
        0.7,
        Vec3::unit_x(),
    );
    let cases: [(&str, Curve3<f64>, geom::Surface<f64>); 7] = [
        ("a circle in a plane", rim(1.0, 0.0), plane(0.0)),
        (
            "a wall's ruling",
            line([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            wall(1.0),
        ),
        ("a rim of a wall", rim(1.0, 0.4), wall(1.0)),
        ("a coaxial circle on a sphere", rim(0.6, 0.8), sphere(1.0)),
        ("a torus's outer rim", rim(2.0 + r, 0.0), torus()),
        ("a torus's meridian", meridian, torus()),
        (
            "a cone's generator",
            line([0.0, 0.0, 0.0], [1.0, 0.0, 1.0]),
            cone(),
        ),
    ];
    for (label, c, s) in cases {
        assert_eq!(class(&c, &s), "Lies", "{label}");
    }
}

/// **`Apart`, on every arm**: a conic clear of a plane, a line clear of a
/// wall, a sphere and a cone, a circle clear of a wall and of a torus,
/// and a line parallel to a wall's axis off it, both through `cross`.
#[test]
fn every_arm_reads_a_span_apart_from_its_surface() {
    let cases: [(&str, Curve3<f64>, geom::Surface<f64>); 6] = [
        ("a circle clear of a plane", rim(1.0, 0.0), plane(2.0)),
        (
            "a line past a wall",
            line([3.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            wall(1.0),
        ),
        (
            "a parallel line off a wall",
            line([3.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            wall(1.0),
        ),
        (
            "a line past a sphere",
            line([0.0, 3.0, 0.0], [1.0, 0.0, 0.0]),
            sphere(1.0),
        ),
        ("a circle inside a wall", rim(0.5, 0.0), wall(1.0)),
        ("a circle in a torus's hole", rim(0.5, 0.0), torus()),
    ];
    for (label, c, s) in cases {
        assert_eq!(class(&c, &s), "Apart", "{label}");
    }
}

/// **Each rung escalates on itself**: a circle in a plane at an offset in
/// the gap (`PlaneParallel`), a wall's ruling at an offset in the gap
/// (`AxisParallel`), a rim of a wall and of a torus at a residual in the
/// gap (`Coaxial`), a line grazing a wall in the gap (`Discriminant`), a
/// near-circular ellipse on a sphere whose second harmonic is in the gap
/// and whose points read in band (`NotLies`), and a Villarceau circle,
/// which lies on its torus and which no rung of the pose reads as lying
/// (`NotLies`).
#[test]
fn each_rung_escalates_on_itself() {
    let g = gap();
    let delta = 3.0 * band().zero();
    let ellipse = Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        1.0 + delta,
        1.0 - delta,
    ));
    let (big, small) = (2.0_f64, 0.7_f64);
    let tilt = (small / big).asin();
    let lean = Vec3::new(0.0, -1.0, 0.0) * tilt.cos() + Vec3::unit_z() * tilt.sin();
    let villarceau = Curve3::circle(
        Point3::new(small, 0.0, 0.0),
        Vec3::unit_x().cross(lean),
        big,
        Vec3::unit_x(),
    );
    let cases: [(&str, Curve3<f64>, geom::Surface<f64>, PoseRung); 7] = [
        (
            "a circle a gap off its plane",
            rim(1.0, g),
            plane(0.0),
            PoseRung::PlaneParallel,
        ),
        (
            "a ruling a gap off its wall",
            line([1.0 + g, 0.0, 0.0], [0.0, 0.0, 1.0]),
            wall(1.0),
            PoseRung::AxisParallel,
        ),
        (
            "a rim a gap off its wall",
            rim(1.0 + g, 0.3),
            wall(1.0),
            PoseRung::Coaxial,
        ),
        (
            "a rim a gap off a torus",
            rim(2.7 + g, 0.0),
            torus(),
            PoseRung::Coaxial,
        ),
        (
            "a line grazing a wall in the gap",
            line([1.0 - g, -2.0, 0.3], [0.0, 1.0, 0.0]),
            wall(1.0),
            PoseRung::Discriminant,
        ),
        (
            "a near-circle on a sphere",
            ellipse,
            sphere(1.0),
            PoseRung::NotLies,
        ),
        (
            "a Villarceau circle",
            villarceau,
            torus(),
            PoseRung::NotLies,
        ),
    ];
    for (label, c, s, rung) in cases {
        assert_eq!(class(&c, &s), format!("InBand({rung:?})"), "{label}");
    }
}

/// **`Through` on both arms that have a singular point**: a line through
/// a cone's apex and a circle through it, each answering the apex as the
/// pose's datum, not as a meeting.
#[test]
fn a_span_through_a_cones_apex_is_through() {
    let tilted = Curve3::circle(
        Point3::new(0.5, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.5,
        Vec3::new(-1.0, 0.0, 0.0),
    );
    for (label, c) in [
        ("a line", line([-1.0, 0.0, 0.5], [1.0, 0.0, -0.5])),
        ("a circle", tilted),
    ] {
        let found = read(&c, &cone()).unwrap_or_else(|r| panic!("{label}: {r:?}"));
        let Pose::Through(apex) = found.pose else {
            panic!("{label}: {:?}", found.pose);
        };
        assert!(
            apex.point.equals_exactly(Point3::origin()),
            "{label}: the apex"
        );
        assert!(
            found
                .meetings
                .iter()
                .all(|m| (m.point - Point3::origin()).norm() > band().escalate()),
            "{label}: the apex is no meeting: {:?}",
            found.meetings
        );
    }
}

/// **A higher-order contact escalates on `Kappa`**: a unit circle tilted
/// so its swing `R` across a plane is `1.5Kε` grazes the plane, a belly
/// in the zero band and a swing past it. Over the whole turn its
/// relative curvature's sagitta, `κ·L²/2`, is definite: a touch. Over a
/// span of `0.6` rad about the touch it reads in the gap.
#[test]
fn a_higher_order_contact_escalates_on_kappa() {
    let swing = 1.5 * band().escalate();
    let axis = Vec3::new(0.0, swing, (1.0 - swing * swing).sqrt());
    let c = Curve3::circle(Point3::new(0.0, 0.0, swing), axis, 1.0, Vec3::unit_x());
    let Ok(Pose::Tangent(touch)) = pose(&whole(&c), &plane(0.0), band()) else {
        panic!("the whole turn touches");
    };
    let near = Span::new(&c, (touch.at - 0.3, touch.at + 0.3));
    match pose(&near, &plane(0.0), band()) {
        Err(PoseRefusal::InBand {
            rung: PoseRung::Kappa,
            ..
        }) => {}
        other => panic!("0.6 rad about the touch: {other:?}"),
    }
}

/// A true distance from a surface.
type Distance = fn(&geom::Surface<f64>, Point3<f64>) -> f64;

/// **A touch's arc holds every span point within the band of the
/// surface**: a circle grazing a plane, a line grazing a sphere and a
/// wall, each read at the true
/// distance on a fine sampling of the span about the touch.
#[test]
fn a_touchs_arc_holds_the_in_band_arc() {
    let sin = 1e-3_f64;
    let axis = Vec3::new(0.0, sin, (1.0 - sin * sin).sqrt());
    let cases: [(&str, Curve3<f64>, geom::Surface<f64>, Distance); 3] = [
        (
            "a circle on a plane",
            Curve3::circle(Point3::new(0.0, 0.0, sin), axis, 1.0, Vec3::unit_x()),
            plane(0.0),
            |_, p| p.z,
        ),
        (
            "a line on a sphere",
            line([0.0, -2.0, 1.0], [0.0, 1.0, 0.0]),
            sphere(1.0),
            super::conic_oracle::distance,
        ),
        (
            "a line on a wall",
            line([1.0, -2.0, 0.3], [0.0, 1.0, 0.0]),
            wall(1.0),
            super::conic_oracle::distance,
        ),
    ];
    for (label, c, s, distance) in cases {
        let span = whole(&c);
        let got = pose(&span, &s, band());
        let Ok(Pose::Tangent(touch)) = got else {
            panic!("{label}: {got:?}");
        };
        let (t0, t1) = span.params();
        let n = 200_000;
        for k in 0..=n {
            let t = t0 + (t1 - t0) * f64::from(k) / f64::from(n);
            let near = |x: f64| {
                if matches!(c, Curve3::Line(_)) {
                    x
                } else {
                    geom_core::period::Period::angle().near(x, touch.at)
                }
            };
            let t = near(t);
            if (t - touch.at).abs() > PI - 1e-9 {
                continue;
            }
            let d = distance(&s, c.eval(t)).abs();
            if d <= band().zero() {
                assert!(
                    touch.arc.0 <= t && t <= touch.arc.1,
                    "{label}: {t} is {d:e} off the surface, outside the arc {:?}",
                    touch.arc
                );
            }
        }
    }
}

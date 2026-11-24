//! **The curve × face pose's rows** (`super::pose`): each class read
//! through [`super::cross()`] on every arm that answers it, each rung's
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
        frame: geom::Frame::new(Point3::new(0.3, -0.2, z), Vec3::unit_z(), Vec3::unit_x()),
    }
}

fn wall(r: f64) -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: r,
    }
}

fn sphere(r: f64) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: r,
    }
}

/// The cone of half-angle `π/4` with its apex at the origin, up `z`.
fn cone() -> geom::Surface<f64> {
    geom::Surface::Cone {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        half_angle: core::f64::consts::FRAC_PI_4,
    }
}

/// The ring torus `(2, 0.7)` about `z`.
fn torus() -> geom::Surface<f64> {
    geom::Surface::Torus {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        major_radius: 2.0,
        minor_radius: 0.7,
    }
}

/// The circle about `(0, 0, z)` square to `z`, of radius `r`.
fn rim(r: f64, z: f64) -> Curve3<f64> {
    Curve3::circle(Point3::new(0.0, 0.0, z), Vec3::unit_z(), r, Vec3::unit_x())
}

fn line(origin: [f64; 3], dir: [f64; 3]) -> Curve3<f64> {
    Curve3::line(
        Point3::from_array(origin),
        geom_core::UnitVec3::squared(Vec3::from_array(dir)),
    )
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

/// **A higher-order contact escalates on `Kappa`, whatever the span**:
/// `κ` is levered by the carrier's own scale at the touch, never the
/// span's length. A line tangent to a unit wall at a slope `s` off its
/// axis has `κ = s²/r`, its sagitta over the wall's radius `s²r/2`: at
/// `s = 1e-4` that is `5e-9`, in the gap, and it escalates over the
/// whole run and over `0.6` about the touch alike (levered by the run of
/// `6`, it would read definite); at `s = 1e-3` it is a definite touch
/// over both. A conic against a plane is never so: past the parallel
/// gate its sagitta over its diameter is its whole swing `2R`, past `Kε`.
#[test]
fn a_higher_order_contact_escalates_on_kappa() {
    for (slope, touches) in [(1e-4_f64, false), (1e-3, true)] {
        let c = line([1.0, 0.0, 0.0], [0.0, slope, (1.0 - slope * slope).sqrt()]);
        for span in [whole(&c), Span::new(&c, (-0.3, 0.3))] {
            let got = pose(&span, &wall(1.0), band());
            let label = format!("slope {slope} over {:?}", span.params());
            if touches {
                assert!(matches!(got, Ok(Pose::Tangent(_))), "{label}: {got:?}");
            } else {
                assert!(
                    matches!(
                        got,
                        Err(PoseRefusal::InBand {
                            rung: PoseRung::Kappa,
                            ..
                        })
                    ),
                    "{label}: {got:?}"
                );
            }
        }
    }
}

/// **A short span far from a touch is read by the span itself**
/// (`Kappa` only where it decides): a unit hoop tangent to a plane at
/// `θ = π/2`, read over `±1e-5` about `θ = 0, 1, 2`, stands clear of the
/// touch's reach: `Apart`, never `Kappa` in band over the span's short
/// length; the plane door's old entry answers no root. A line tangent to a
/// wall, read over a metre 4 m from the touch, is apart too.
#[test]
fn a_short_span_far_from_a_touch_is_apart() {
    let hoop = Curve3::circle(
        Point3::new(0.0, 0.0, -1.0),
        Vec3::unit_x(),
        1.0,
        Vec3::unit_z(),
    );
    let touch = hoop.eval(PI / 2.0);
    assert!((touch.y + 1.0).abs() < 1e-15, "the hoop's touch: {touch:?}");
    let floor = |y: f64| geom::Surface::Plane {
        frame: geom::Frame::new(Point3::new(0.0, y, 0.0), Vec3::unit_y(), Vec3::unit_x()),
    };
    for mid in [0.0, 1.0, 2.0] {
        let span = Span::new(&hoop, (mid - 1e-5, mid + 1e-5));
        let got = cross(&span, &floor(touch.y), band());
        assert!(
            matches!(
                got,
                Ok(Crossings {
                    pose: Pose::Apart,
                    ..
                })
            ),
            "about θ = {mid}: {got:?}"
        );
        let old = super::plane_crossing::plane_crossing_lane(
            &hoop,
            mid - 1e-5,
            mid + 1e-5,
            Point3::new(0.0, touch.y, 0.0),
            Vec3::unit_y(),
            band(),
        );
        use super::plane_crossing::{ConicPlaneMeet, PlaneCrossingLane};
        let no_root = match &old {
            PlaneCrossingLane::Conic(ConicPlaneMeet::Miss) => true,
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(roots))) => roots.is_empty(),
            _ => false,
        };
        assert!(no_root, "the old entry about θ = {mid}: {old:?}");
    }
    let l = line([1.0, -5.0, 0.0], [0.0, 1.0, 0.0]);
    let got = cross(&Span::new(&l, (8.5, 9.5)), &wall(1.0), band());
    assert!(
        matches!(
            got,
            Ok(Crossings {
                pose: Pose::Apart,
                ..
            })
        ),
        "a line tangent to a wall, read 4 m from the touch: {got:?}"
    );
}

/// **A touch just past the span whose reach falls inside it is a
/// window, never nothing**: a unit hoop poking `0.5e-9` through a plane
/// at `θ = 0`, read over `(2e-5, 1)`, has a true root at `3.16e-5`,
/// inside the span; a line just inside a wall the same way. `cross`
/// answers no meeting it cannot place, but a window over the touch's
/// reach into the span.
#[test]
fn a_touch_reaching_into_the_span_is_a_window() {
    let hoop = Curve3::circle(Point3::origin(), Vec3::unit_y(), 1.0, Vec3::unit_z());
    let poke = geom::Surface::Plane {
        frame: geom::Frame::new(
            Point3::new(0.0, 0.0, 1.0 - 0.5e-9),
            Vec3::unit_z(),
            Vec3::unit_x(),
        ),
    };
    let l = line([1.0 - 0.5e-9, -1.0, 0.0], [0.0, 1.0, 0.0]);
    for (label, span, s) in [
        ("hoop", Span::new(&hoop, (2e-5, 1.0)), poke),
        ("line", Span::new(&l, (1.0 + 2e-5, 2.0)), wall(1.0)),
    ] {
        let got = cross(&span, &s, band());
        let Err(refusal) = &got else {
            panic!("{label}: a window, got {got:?}");
        };
        assert!(
            refusal.found.is_empty()
                && refusal.windows.len() == 1
                && refusal.windows[0].window.0 <= span.params().0 + 1e-12
                && refusal.windows[0].window.1 >= span.params().0 + 3.2e-5,
            "{label}: the touch's reach into the span, got {got:?}"
        );
    }
}

/// A true distance from a surface.
type Distance = fn(&geom::Surface<f64>, Point3<f64>) -> f64;

/// **A touch's arc holds every span point within the band of the
/// surface**: a circle grazing a plane (and one whose swing across it is
/// `3Kε`, its arc a quarter radian wide), a line grazing a sphere and a
/// wall, each read at the true distance on a fine sampling of the span
/// about the touch.
#[test]
fn a_touchs_arc_holds_the_in_band_arc() {
    let sin = 1e-3_f64;
    let axis = Vec3::new(0.0, sin, (1.0 - sin * sin).sqrt());
    // A swing of `3Kε` across the plane: a definite touch whose arc is
    // wide, a quarter radian, where a first harmonic falls slower than its
    // quadratic and the arc's `π/2` is owed.
    let wide = 3.0 * band().escalate();
    let wide_axis = Vec3::new(0.0, wide, (1.0 - wide * wide).sqrt());
    let cases: [(&str, Curve3<f64>, geom::Surface<f64>, Distance); 4] = [
        (
            "a circle on a plane",
            Curve3::circle(Point3::new(0.0, 0.0, sin), axis, 1.0, Vec3::unit_x()),
            plane(0.0),
            |_, p| p.z,
        ),
        (
            "a circle swinging 3Kε across a plane",
            Curve3::circle(Point3::new(0.0, 0.0, wide), wide_axis, 1.0, Vec3::unit_x()),
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

/// The cone of half-angle `alpha` with its apex at the origin, up `z`.
fn cone_of(alpha: f64) -> geom::Surface<f64> {
    geom::Surface::Cone {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        half_angle: alpha,
    }
}

/// **A narrow cone's `F` is no residual** (`|res| ≤ |F| / floor`, the
/// floor about `sin 2α`): a line from the apex `4e-8` off a generator of
/// a cone of half-angle `0.01`, its residual `−4e-8` at the run's far
/// end, four bands, reads its apex and its lead in the zero band. It
/// does not lie on the cone: it passes through the apex
/// (`curve_face_pose_cone_on_surface` reads its whole reach through the
/// floor, past the band). The door's old entry does not call it on the
/// carrier.
#[test]
fn a_line_off_a_narrow_cones_generator_does_not_lie_on_it() {
    let (alpha, off) = (0.01f64, 4e-8);
    let c = line([0.0; 3], [(alpha - off).sin(), 0.0, (alpha - off).cos()]);
    let span = Span::new(&c, (0.0, 1.0));
    let got = pose(&span, &cone_of(alpha), band());
    assert!(
        matches!(got, Ok(Pose::Through(_))),
        "through the apex: {got:?}"
    );
    let Curve3::Line(l) = c else { unreachable!() };
    let old = super::line_quadric::line_quadric_roots(l, (0.0, 1.0), &cone_of(alpha), band());
    assert!(
        !matches!(old, Ok(Some(Ok(super::circle_roots::CircleRoots::On(_))))),
        "not on the carrier: {old:?}"
    );
    // On the generator itself it lies, at both half-angles' ends.
    for alpha in [0.01f64, 1.56] {
        let g = line([0.0; 3], [alpha.sin(), 0.0, alpha.cos()]);
        let got = pose(&Span::new(&g, (0.0, 1.0)), &cone_of(alpha), band());
        assert!(matches!(got, Ok(Pose::Lies)), "α {alpha}: {got:?}");
    }
}

/// **A narrow cone's `F` extreme in the zero band is no graze**: a circle
/// at height `1` square to the axis of a cone of half-angle `0.01` (and
/// `0.003`), its centre `1e-3` off the axis, its residual's least value
/// `±3e-8`, three bands either way: `F`'s extreme reads in the zero band,
/// and through the floor (`curve_face_pose_cone_graze`) it is past it.
/// A line across the cone at that height, its least residual the same,
/// reads its depth `|G|/R` in the zero band, the same way. Never
/// `Tangent`: apart above, two crossings below, or a refusal.
#[test]
fn a_circle_by_a_narrow_cone_is_not_read_tangent_off_fs_extreme() {
    for alpha in [0.01f64, 0.003] {
        for least in [3e-8, -3e-8] {
            let (sin, cos) = alpha.sin_cos();
            let r = (sin + least) / cos + 1e-3;
            let c = Curve3::circle(
                Point3::new(1e-3, 0.0, 1.0),
                Vec3::unit_z(),
                r,
                Vec3::unit_x(),
            );
            // And a line across the axis's plane at height 1, its least
            // residual the same: its depth `|G|/R` in the zero band.
            let l = line([-0.1, (sin + least) / cos, 1.0], [1.0, 0.0, 0.0]);
            for (span, label) in [
                (whole(&c), format!("circle, α {alpha}, least {least:e}")),
                (
                    Span::new(&l, (0.0, 0.2)),
                    format!("line, α {alpha}, least {least:e}"),
                ),
            ] {
                let got = cross(&span, &cone_of(alpha), band());
                match &got {
                    Ok(found) => match found.pose {
                        Pose::Apart => assert!(least > 0.0, "{label}: {got:?}"),
                        Pose::Isolated(_) => {
                            assert!(least < 0.0 && found.meetings.len() == 2, "{label}: {got:?}");
                        }
                        _ => panic!("{label}: {got:?}"),
                    },
                    Err(refusal) => assert!(
                        matches!(
                            refusal.windows[0].why,
                            CrossWhy::Pose(PoseRefusal::InBand {
                                rung: PoseRung::Extreme | PoseRung::Discriminant,
                                ..
                            })
                        ),
                        "{label}: {got:?}"
                    ),
                }
            }
        }
    }
}

/// **A drift in the zero band still bounds the offset**
/// (`curve_face_pose_axis_offset` reads `at_mid ± drift/2`): a line along
/// a wall's axis, `0.8ε` off the wall at its middle and drifting `0.8ε`
/// over its run, reads its drift in the zero band, its middle in it too,
/// and its far end `1.2ε` off, in the gap: it refuses on the axis-parallel
/// rung, never `Lies`.
#[test]
fn an_axis_parallel_line_reads_its_offset_with_its_drift() {
    let eps = band().zero();
    let l = line([1.0 + 0.4 * eps, 0.0, 0.0], [0.8 * eps, 0.0, 1.0]);
    let got = pose(&Span::new(&l, (0.0, 1.0)), &wall(1.0), band());
    assert!(
        matches!(
            got,
            Err(PoseRefusal::InBand {
                rung: PoseRung::AxisParallel,
                ..
            })
        ),
        "{got:?}"
    );
}

/// **A circle parallel to a torus's axis in the plane of its tube's top
/// grazes it and refuses** (`super::pose`'s parallel arm): its plane's
/// depth into the tube reads in the zero band, and its extremes stand on
/// either side of the centre circle, one over the hole and one beyond:
/// two touches the band cannot call, never `Apart`.
#[test]
fn a_circle_grazing_a_tubes_top_across_its_centre_circle_refuses() {
    let c = Curve3::circle(
        Point3::new(2.0, 0.0, 0.7),
        Vec3::unit_z(),
        0.5,
        Vec3::unit_x(),
    );
    let got = pose(&whole(&c), &torus(), band());
    assert!(
        matches!(
            got,
            Err(PoseRefusal::InBand {
                rung: PoseRung::Contour,
                ..
            })
        ),
        "{got:?}"
    );
}

/// **A span no point of which reads off the surface is degenerate to the
/// doors' old entries** (`PoseRefusal::unsettled`): a Villarceau circle
/// of the torus escalates the pose's not-`Lies` rung in its zero band,
/// which the circle × torus door's old entry answers
/// `Unsettled::Degenerate`, not a graze's `Tangency`.
#[test]
fn a_villarceau_circle_is_degenerate_to_the_old_entry() {
    let (big, small) = (2.0_f64, 0.7_f64);
    let tilt = (small / big).asin();
    let lean = Vec3::new(0.0, -1.0, 0.0) * tilt.cos() + Vec3::unit_z() * tilt.sin();
    let villarceau = Curve3::circle(
        Point3::new(small, 0.0, 0.0),
        Vec3::unit_x().cross(lean),
        big,
        Vec3::unit_x(),
    );
    let got = super::circle_torus::circle_torus_roots(&villarceau, 0.0, TAU, &torus(), band());
    assert!(
        matches!(got, Ok(Err(crate::roots::Unsettled::Degenerate))),
        "{got:?}"
    );
}

/// **A pair the band cannot part is one window, neither found**
/// (`super::cross::place`): two roots `1e-12` apart along a line are
/// within the band of each other by their chord, so the placement keeps
/// neither as a meeting and refuses one window over both; the found
/// meetings sit outside every window.
#[test]
fn a_pair_the_band_cannot_part_is_one_window() {
    let l = line([0.0; 3], [1.0, 0.0, 0.0]);
    let span = Span::new(&l, (0.0, 1.0));
    let placed = super::cross::place(&span, &[0.5, 0.5 + 1e-12], band());
    assert!(
        placed.meetings.is_empty() && placed.refused.len() == 1,
        "{:?} {:?}",
        placed.meetings,
        placed.refused.iter().map(|r| r.0).collect::<Vec<_>>()
    );
    let (window, _) = placed.refused[0];
    assert!(window.0 <= 0.5 && window.1 >= 0.5 + 1e-12, "{window:?}");
}

/// **A touch past the span is read by its reach at `Kε`, and its window
/// is what that reach covers**: a unit hoop exactly tangent to a plane at
/// `θ = 0`, read over `(s, 1)` for `s` from `1e-4` to `2e-4`. Its points
/// from `s` to `√(2Kε) ≈ 1.41e-4` stand within `Kε` of the plane, in the
/// band's gap: none may read as decided. So the span is not `Apart` while
/// any is on it, and its window holds every one of them; no window is a
/// point.
#[test]
fn a_touch_past_the_span_is_windowed_by_its_reach() {
    let hoop = Curve3::circle(Point3::origin(), Vec3::unit_y(), 1.0, Vec3::unit_z());
    let top = geom::Surface::Plane {
        frame: geom::Frame::new(Point3::new(0.0, 0.0, 1.0), Vec3::unit_z(), Vec3::unit_x()),
    };
    for k in 0..=10 {
        let s = 1e-4 + 1e-5 * f64::from(k);
        let got = cross(&Span::new(&hoop, (s, 1.0)), &top, band());
        let gap: Vec<f64> = (0..=1000)
            .map(|i| s + (2e-4 - s) * f64::from(i) / 1000.0)
            .filter(|&t| 1.0 - t.cos() < band().escalate())
            .collect();
        match &got {
            Ok(found) => assert!(
                gap.is_empty() && found.meetings.is_empty(),
                "s = {s:e}: decided over points in the gap, {got:?}"
            ),
            Err(refusal) => {
                assert!(
                    refusal.windows.iter().all(|w| w.window.1 > w.window.0),
                    "s = {s:e}: a point window, {got:?}"
                );
                for t in gap {
                    assert!(
                        refusal
                            .windows
                            .iter()
                            .any(|w| w.window.0 <= t && t <= w.window.1),
                        "s = {s:e}: {t:e}, in the gap, outside every window: {got:?}"
                    );
                }
            }
        }
    }
}

/// The far-centred conic graze rows' readings: `(wrong, of them beyond
/// the escalation, definite)` of the poses read. Each pose is a graze
/// from [`crate::curve_pair::far_graze_rows::graze`] (an ellipse of
/// semi-axes `far` and `far/3`, held at its centre or on its arc, grazed
/// at any angle round the turn by a point in the box) against a plane
/// through that point, its normal the ellipse's in-plane normal there
/// tilted out of the ellipse's plane, its stored origin by the arc and its
/// gap within twice the band's escalation. Judged exactly on the stored
/// data ([`crate::curve_pair::far_graze_rows::judge`]).
fn far_centre_conic_grazes(far: f64, arc_held: bool) -> (usize, usize, usize) {
    use crate::curve_pair::far_graze_rows::{band, graze, judge};
    let band = band();
    let mut rng = test_utils::fuzz::start("curve_face::far_centre_conic_grazes");
    let (mut wrong, mut beyond, mut definite) = (0, 0, 0);
    for _ in 0..test_utils::fuzz::scaled(2000) {
        let g = graze(&mut rng, far, arc_held);
        let tilt = rng.range(-1.2, 1.2);
        let n = (g.outward * tilt.cos() + g.axis * tilt.sin()).normalize();
        let w = n.cross(g.along).normalize();
        let gap = rng.range(-2.0, 2.0) * band.escalate();
        let plane = geom::Frame::new(g.crest + n * gap + w * rng.range(-3.0, 3.0), n, w);
        let span = Span::new(&g.conic, (0.0, TAU));
        let read = match pose(&span, &geom::Surface::Plane { frame: plane }, band) {
            Ok(Pose::Isolated(_)) => Some(true),
            Ok(Pose::Apart) => Some(false),
            _ => None,
        };
        if let Some(meets) = read {
            definite += 1;
            let (w, b) = judge(g.stored, (plane.origin(), plane.axis()), meets, band);
            wrong += usize::from(w);
            beyond += usize::from(b);
        }
    }
    (wrong, beyond, definite)
}

/// **A graze of a far-centred conic is read wrong nowhere**: an ellipse
/// of semi-axes of the order `far`, held at its centre or on its arc,
/// whose height above a plane grazing it at any angle round the turn has
/// an amplitude and an offset each of the order `far` and a belly of the
/// band's order. The pose charges no rounding (`[simple.conditioned]`),
/// so no reading may be definite where the exact belly is not.
#[test]
fn a_graze_of_a_far_centred_conic_is_read_wrong_nowhere() {
    let got: Vec<_> = [false, true]
        .into_iter()
        .flat_map(|held| [1e4, 1e5, 1e6].map(|far| (held, far, far_centre_conic_grazes(far, held))))
        .collect();
    println!("(arc-held, far, (wrong, beyond the escalation, definite)): {got:?}");
    assert!(
        got.iter().all(|g| g.2.0 == 0),
        "read wrong: {got:?}; {}",
        test_utils::fuzz::replay()
    );
}

//! **The root doors a point touch leans on (S′)**, read at the run's
//! band: an edge standing within the band of the other carrier, on the
//! side a touch's parting edge stands (the module docs, "A touch"),
//! refuses or names its roots, and goes silent only on a clearance
//! decided past the band. A torus point touch reads line × torus inside
//! the solid (a plane's or a wall's straight edge), circle × torus (a
//! partner's rim) and circle × plane, sphere and wall (a torus face's
//! circles); the one-point touches of a sphere or a wall against a plane,
//! a sphere or a wall read line × sphere and line × wall (the reduction's
//! belly branch) and the circle doors. Each row reads one door the sweep
//! reads an edge by (`reduce.rs` `wall_crossing`), at gaps of `ε/100`,
//! `ε/2` and `2ε` from the other carrier, and, against vacuity, at
//! `3Kε`, where every door answers a decided miss.

use super::search_kit::band;
use core::f64::consts::TAU;
use geom::{Curve3, Frame, Line, Surface};
use geom_brep::curve_face::{MeetingKind, OnSpan, Pose, cross, pose};
use geom_brep::curve_pair::Span;
use geom_core::{Point3, UnitVec3, Vec3};

/// What a door answered of a span, as the sweep reads it: silent (a
/// miss, or roots none of which lies inside the span: no event), a root
/// inside the span (an event), or a refusal (a tangency, an escalation,
/// a limit), which keeps the door.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Read {
    Silent,
    Event,
    Refused,
}

/// What `cross` answered of `carrier`'s `span` against `s`: a refusal
/// (a window), the span on the carrier, or a touch keeps the door; a
/// crossing strictly inside the span is an event; anything else (`Apart`,
/// or meetings only at an end) is silent.
fn read_cross(carrier: &Curve3<f64>, span: (f64, f64), s: &Surface<f64>) -> Read {
    match cross(&Span::new(carrier, span), s, band()) {
        Err(_) => Read::Refused,
        Ok(got) if matches!(got.pose, Pose::Lies) => Read::Refused,
        Ok(got) => {
            if got
                .meetings
                .iter()
                .any(|m| matches!(m.kind, MeetingKind::Touch { .. }))
            {
                Read::Refused
            } else if got
                .meetings
                .iter()
                .any(|m| matches!(m.on_span, OnSpan::Inside { .. }))
            {
                Read::Event
            } else {
                Read::Silent
            }
        }
    }
}

/// The donut of the torus doors about `y`, `R = 2`, `r = 0.5`: its top
/// on the outer equator is `(0, 0, 2.5)`, and the tube circle there is
/// centred `(0, 0, 2)` in the plane `y = 0`.
fn donut() -> Surface<f64> {
    Surface::Torus {
        frame: Frame::new(Point3::origin(), Vec3::unit_y(), Vec3::unit_x()),
        major_radius: 2.0,
        minor_radius: 0.5,
    }
}

/// The unit sphere and the unit wall about `z` through the origin.
fn sphere() -> Surface<f64> {
    Surface::Sphere {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    }
}

fn wall() -> Surface<f64> {
    Surface::Cylinder {
        frame: Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    }
}

/// A line through `at` along `d`, read over `±half` about it.
fn line_read(at: Point3<f64>, d: Vec3<f64>, half: f64, s: &Surface<f64>) -> Read {
    let line = Line {
        origin: at - d.normalize() * half,
        dir: UnitVec3::squared(d),
    };
    read_cross(&Curve3::Line(line), (0.0, 2.0 * half), s)
}

/// A whole circle about `c`, normal `n`, of radius `rad`.
fn circle(c: Point3<f64>, n: Vec3<f64>, rad: f64) -> Curve3<f64> {
    let (u, _) = n.orthonormal_basis();
    Curve3::circle(c, n, rad, u)
}

/// Every door's reading of one pose, at gap `g`: `(door, read)`.
fn doors_at(g: f64) -> Vec<(&'static str, Read)> {
    let eps = band().zero();
    let (y, z) = (Vec3::unit_y(), Vec3::unit_z());
    let top = |depth: f64| Point3::new(0.0, 0.0, 2.5 - depth);
    // A segment wholly inside the tube below its top, shorter than the
    // spread of its roots there, `±√(r·g)` and more.
    let inside = 0.01 * eps.sqrt();
    let turn = (0.0, TAU);
    let circle_torus = |c: &Curve3<f64>| read_cross(c, turn, &donut());
    let conic_quadric = |c: &Curve3<f64>, s: &Surface<f64>| read_cross(c, turn, s);
    let mut out = vec![
        (
            "line × torus inside, along the axis",
            line_read(top(g), y, inside, &donut()),
        ),
        (
            "line × torus inside, generic",
            line_read(top(g), Vec3::new(1.0, 0.6, 0.0), inside, &donut()),
        ),
        (
            "line × torus inside, along the axis beside the inner equator",
            line_read(Point3::new(0.0, 0.0, 1.5 + g), y, inside, &donut()),
        ),
        (
            "circle × torus outside",
            circle_torus(&circle(Point3::new(0.0, 0.0, 2.8 + g), y, 0.3)),
        ),
        (
            "circle × torus inside",
            circle_torus(&circle(Point3::new(0.0, 0.0, 2.3 - g), y, 0.2)),
        ),
        (
            "line × sphere outside",
            line_read(
                Point3::new(1.0 + g, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.3),
                1.0,
                &sphere(),
            ),
        ),
        (
            "line × wall outside",
            line_read(
                Point3::new(1.0 + g, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.3),
                1.0,
                &wall(),
            ),
        ),
        (
            "line × sphere inside",
            line_read(
                Point3::new(1.0 - g, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.3),
                1.0,
                &sphere(),
            ),
        ),
        (
            "line × wall inside",
            line_read(
                Point3::new(1.0 - g, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.3),
                1.0,
                &wall(),
            ),
        ),
    ];
    for (door, at, s) in [
        ("circle × sphere outside", 1.3 + g, sphere()),
        ("circle × sphere inside", 0.7 - g, sphere()),
        ("circle × wall outside", 1.3 + g, wall()),
        ("circle × wall inside", 0.7 - g, wall()),
    ] {
        out.push((
            door,
            conic_quadric(&circle(Point3::new(at, 0.0, 0.0), z, 0.3), &s),
        ));
    }
    let plane = Surface::Plane {
        frame: Frame::new(Point3::new(0.0, 0.0, g), z, Vec3::unit_x()),
    };
    let arc = circle(Point3::new(0.0, 0.0, -0.3), y, 0.3);
    let span = geom_brep::curve_pair::Span::new(&arc, turn);
    out.push((
        "circle × plane",
        match pose::pose(&span, &plane, band()) {
            Ok(Pose::Apart) => Read::Silent,
            Ok(Pose::Tangent(_)) | Err(_) => Read::Refused,
            Ok(_) => Read::Event,
        },
    ));
    out
}

/// **Within the band, no door a touch at an extreme leans on goes
/// silent**: each answers a tangency, escalates, or names a root, at
/// gaps of `ε/100`, `ε/2` and `2ε` (the reach an extreme's parting edge
/// needs is the touch's margin, at most `2ε`); and past `3Kε` each
/// answers decided, a miss or a root. The line × torus door is read on
/// the solid's side only, where a torus point touch's parting line
/// stands (the module docs), along the axis beside both equators and in a
/// generic direction; line × sphere and line × wall on both sides, the
/// inside the side a one-point touch's parting line stands at sphere ×
/// plane or wall × plane.
/// Mutant: any door's silent answer read on a margin that is no
/// clearance in metres.
#[test]
fn within_the_band_no_door_an_extreme_leans_on_goes_silent() {
    let eps = band().zero();
    for g in [0.01 * eps, 0.5 * eps, 2.0 * eps] {
        for (door, read) in doors_at(g) {
            assert_ne!(read, Read::Silent, "{door} at a gap of {:.2}ε", g / eps);
        }
    }
    let far = 3.0 * band().escalate();
    for (door, read) in doors_at(far) {
        assert_ne!(read, Read::Refused, "{door} at 3Kε");
    }
}

/// **The line × torus door goes silent within the band outside the
/// solid** (`work/germ/the-line-torus-door-misses-silently-within-the-band.md`):
/// a line along the torus's axis `ε/2` beside the outer equator, outside
/// the tube, and one `ε/2` beside the inner equator, in the hole, each
/// answer a certified miss, decided on the quartic's discriminant, not on
/// a clearance. A touch at a point extreme does not lean on that side; a
/// touch at a saddle or along a parallel would, and their admission waits
/// on this row turning red.
#[test]
fn the_line_torus_door_misses_silently_outside_the_solid_within_the_band() {
    let eps = band().zero();
    for (side, at) in [
        ("the outer equator", Point3::new(0.0, 0.0, 2.5 + 0.5 * eps)),
        ("the inner equator", Point3::new(0.0, 0.0, 1.5 - 0.5 * eps)),
    ] {
        assert_eq!(
            line_read(at, Vec3::unit_y(), 1.0, &donut()),
            Read::Silent,
            "beside {side}"
        );
    }
}

/// **A segment inside the solid whose shallowest point stands 4ε–9ε deep
/// reads as no event** (`work/germ/a-segment-just-inside-a-torus-reads-no-event.md`):
/// a short segment `6ε` under the outer equator's top, in a generic
/// direction, answers two certified roots past its span. A touch at a
/// point extreme needs the door to see edges within `2ε` only.
#[test]
fn a_segment_just_inside_the_torus_past_2_eps_reads_no_event() {
    let eps = band().zero();
    let at = Point3::new(0.0, 0.0, 2.5 - 6.0 * eps);
    assert_eq!(
        line_read(at, Vec3::new(1.0, 0.6, 0.0), 0.01 * eps.sqrt(), &donut()),
        Read::Silent
    );
}

//! **The root doors a touch at an extreme leans on (S′)**, read at the
//! run's band: an edge standing within the band of the other carrier,
//! on the side a touch's parting edge stands (the module docs, "A
//! touch"), refuses or names its roots, and goes silent only on a
//! clearance decided past the band. Each row reads one door the sweep
//! reads an edge by (`reduce.rs` `wall_crossing`), at gaps of `ε/100`,
//! `ε/2` and `2ε` from the other carrier, and, against vacuity, at
//! `3Kε`, where every door answers a decided miss.

use super::search_kit::band;
use core::f64::consts::TAU;
use geom::{Curve3, Frame, Line, Surface};
use geom_brep::curve_face::{circle_torus, conic_quadric, line_quadric, pose};
use geom_brep::roots::Roots;
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

fn read_roots<C, E, U, const N: usize>(
    got: Result<Result<Roots<f64, N, C>, U>, E>,
    (t0, t1): (f64, f64),
) -> Read {
    match got {
        Ok(Ok(Roots::Miss)) => Read::Silent,
        Ok(Ok(Roots::Certified(found))) => {
            if found.roots().iter().any(|&t| t0 < t && t < t1) {
                Read::Event
            } else {
                Read::Silent
            }
        }
        Ok(Ok(Roots::On(_)) | Err(_)) | Err(_) => Read::Refused,
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
    let span = (0.0, 2.0 * half);
    match line_quadric::line_quadric_roots(line, span, s, band()) {
        Ok(Some(got)) => read_roots(Ok::<_, ()>(got), span),
        Ok(None) | Err(_) => Read::Refused,
    }
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
    let circle_torus = |c: &Curve3<f64>| {
        read_roots(
            circle_torus::circle_torus_roots(c, 0.0, TAU, &donut(), band()),
            turn,
        )
    };
    let conic_quadric = |c: &Curve3<f64>, s: &Surface<f64>| {
        read_roots(
            conic_quadric::conic_quadric_roots(c, 0.0, TAU, s, band()),
            turn,
        )
    };
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
            Ok(pose::Pose::Apart) => Read::Silent,
            Ok(pose::Pose::Tangent(_)) | Err(_) => Read::Refused,
            Ok(_) => Read::Event,
        },
    ));
    out
}

/// **Within the band, no door a touch at an extreme leans on goes
/// silent**: each answers a tangency, escalates, or names a root, at
/// gaps of `ε/100`, `ε/2` and `2ε` (the reach an extreme's parting edge
/// needs is the touch's margin, at most `2ε`); and past `3Kε` each
/// answers a decided miss. The line × torus door is read on the solid's
/// side only, where an extreme's parting line stands (the module docs).
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
        assert_eq!(read, Read::Silent, "{door} at 3Kε");
    }
}

/// **The line × torus door goes silent within the band outside the
/// solid** (`work/germ/the-line-torus-door-misses-silently-within-the-band.md`):
/// a line along the torus's axis `ε/100` above the outer equator's top
/// answers a certified miss, decided on the quartic's discriminant, not
/// on a clearance. A touch at an extreme does not lean on that side; a
/// touch at a saddle would, and its admission waits on this row turning
/// red.
#[test]
fn the_line_torus_door_misses_silently_outside_the_solid_within_the_band() {
    let eps = band().zero();
    let above = Point3::new(0.0, 0.0, 2.5 + 0.01 * eps);
    assert_eq!(
        line_read(above, Vec3::unit_y(), 1.0, &donut()),
        Read::Silent
    );
}

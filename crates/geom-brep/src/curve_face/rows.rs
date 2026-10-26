//! [`super::route`] on every kind pair, and [`super::boundary`] on a
//! quarter disc's boundary (two lines and a quarter arc in `z = 0`): a
//! shared run, a run longer than its boundary edge, a crossing, an end
//! on the rim, a far span, and a vertex in the band's gap.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI};

use geom::{Curve3, CurveKind, SurfaceKind};
use geom_core::{Band, Point3, Vec3};

use super::{Boundary, BoundaryRefusal, Door, boundary, route};
use crate::curve_pair::Span;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn line(origin: [f64; 3], dir: [f64; 3]) -> Curve3<f64> {
    let [x, y, z] = dir;
    Curve3::Line {
        origin: Point3::new(origin[0], origin[1], origin[2]),
        dir: Vec3::new(x, y, z).normalize(),
    }
}

/// The unit circle about the origin in `z = 0`, wound about `axis`.
fn unit_circle(axis: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::origin(),
        axis,
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

/// The quarter disc's boundary carriers, in walk order: the `x` axis
/// from the origin, the arc to `(0, 1)`, and the `y` axis back.
fn rim() -> [Curve3<f64>; 3] {
    [
        line([0.0; 3], [1.0, 0.0, 0.0]),
        unit_circle(Vec3::unit_z()),
        line([0.0, 1.0, 0.0], [0.0, -1.0, 0.0]),
    ]
}

/// [`boundary`] of `span` against [`rim`].
fn read(carrier: &Curve3<f64>, params: (f64, f64)) -> Result<Boundary<f64>, BoundaryRefusal> {
    let rim = rim();
    let spans = [(0.0, 1.0), (0.0, FRAC_PI_2), (0.0, 1.0)];
    let edges: Vec<_> = rim
        .iter()
        .zip(spans)
        .map(|(c, (t0, t1))| (c.eval(t0), Span::new(c, (t0, t1))))
        .collect();
    boundary(&Span::new(carrier, params), &edges, band())
}

/// **Every kind pair has a door**, and the ones not built say so. The
/// table is the claim: a door moved or built changes a row here.
#[test]
fn every_kind_pair_routes_to_its_door() {
    use CurveKind::{Circle, Ellipse, Line, Nurbs, Spiric};
    use SurfaceKind::{Approx, Cone, Cylinder, Plane, Sphere, Torus};
    let curves = [Line, Circle, Ellipse, Spiric, Nurbs];
    let surfaces = [
        Plane,
        Cylinder,
        Cone,
        Sphere,
        Torus,
        SurfaceKind::Nurbs,
        Approx,
    ];
    for c in curves {
        for s in surfaces {
            let want = match (c, s) {
                (Spiric, _) => (Door::Spiric, false),
                (_, SurfaceKind::Nurbs | Approx) => (Door::SplineFace, false),
                (Nurbs, _) => (Door::SplineComposite, false),
                (_, Plane) => (Door::PlaneCrossing, true),
                (Line, _) => (Door::LineRoots, true),
                (_, Torus) => (Door::ConicTorus, true),
                (_, Cylinder | Cone | Sphere) => (Door::ConicQuadric, true),
            };
            let got = route(c, s);
            assert_eq!((got.door, got.implemented), want, "{c:?} × {s:?}: {got:?}");
            assert!(!got.note.is_empty(), "{c:?} × {s:?} names its door");
        }
    }
}

/// **A span along a boundary edge is a shared run**, in either sense,
/// decided by the curve-pair reading alone.
#[test]
fn an_arc_on_the_rim_lies_along_it() {
    let along = read(&unit_circle(Vec3::unit_z()), (0.0, FRAC_PI_2));
    assert!(
        matches!(along, Ok(Boundary::Along { edge: 1 })),
        "same sense: {along:?}"
    );
    let reversed = read(&unit_circle(-Vec3::unit_z()), (-FRAC_PI_2, 0.0));
    assert!(
        matches!(reversed, Ok(Boundary::Along { edge: 1 })),
        "reversed: {reversed:?}"
    );
}

/// **The rest of the rim's circle is no run along the rim**: it shares
/// the arc's two ends and nothing between them.
#[test]
fn the_rest_of_the_rims_circle_does_not_lie_along_it() {
    let rest = read(&unit_circle(Vec3::unit_z()), (FRAC_PI_2, 2.0 * PI));
    assert!(matches!(rest, Ok(Boundary::Clear)), "{rest:?}");
}

/// **A run that ends inside the span meets the boundary at that end**,
/// the vertex where the boundary leaves the span's carrier.
#[test]
fn a_run_shorter_than_the_span_meets_at_its_vertex() {
    match read(&unit_circle(Vec3::unit_z()), (0.0, PI)).unwrap() {
        Boundary::Meets { t, point, edge } => {
            assert_eq!(edge, 2, "the vertex is the third edge's start");
            assert!((t - FRAC_PI_2).abs() < 1e-12, "at the corner: {t}");
            assert!((point - Point3::new(0.0, 1.0, 0.0)).norm() < 1e-12);
        }
        other => panic!("a run ending inside the span: {other:?}"),
    }
}

/// **A span crossing a boundary edge meets it there.**
#[test]
fn a_line_across_the_rim_meets_it() {
    let x = 0.75_f64.sqrt();
    match read(&line([0.5, 0.5, 0.0], [1.0, 0.0, 0.0]), (0.0, 1.5)).unwrap() {
        Boundary::Meets { t, edge, .. } => {
            assert_eq!(edge, 1, "the arc");
            assert!((t - (x - 0.5)).abs() < 1e-12, "at the arc: {t}");
        }
        other => panic!("a line across the arc: {other:?}"),
    }
}

/// **An end on the rim is no meeting inside the span**: the boundary
/// leaves the span's interior to the interior door, as it does a span
/// that comes nowhere near.
#[test]
fn an_end_on_the_rim_and_a_far_span_are_clear() {
    let rising = line([0.6, 0.8, 0.0], [0.0, 0.0, 1.0]);
    let end = read(&rising, (0.0, 1.0));
    assert!(
        matches!(end, Ok(Boundary::Clear)),
        "end on the rim: {end:?}"
    );
    let far = line([5.0, 5.0, 1.0], [1.0, 0.0, 0.0]);
    let far = read(&far, (0.0, 1.0));
    assert!(matches!(far, Ok(Boundary::Clear)), "far: {far:?}");
}

/// **A boundary vertex in the band's gap escalates, and never prunes**:
/// a line through the corner `(0, 1)` but for three zero bands, outside
/// the disc either way.
#[test]
fn a_corner_in_the_gap_escalates() {
    let off = 3.0 * ZERO * FRAC_1_SQRT_2;
    let grazing = line([-off, 1.0 + off, 0.0], [1.0, 1.0, 0.0]);
    match read(&grazing, (-1.0, 1.0)) {
        Err(BoundaryRefusal::Escalated(_)) => {}
        other => panic!("a corner in the gap: {other:?}"),
    }
}

/// **A boundary vertex within the band of a thin ellipse is read on it.**
/// An ellipse of semi-axes `a/b` from 100 to 1000, and a boundary edge
/// starting half the band's zero off it along its in-plane normal, at
/// three parameters across the span, and leaving along that normal. Read
/// at the ellipse's foot the vertex is on it; read at the angle of its
/// unit-coordinate image (`param_near`) its gap is about `(1 + a/b)` times
/// that, which the band decides off.
#[test]
fn a_vertex_in_the_band_of_a_thin_ellipse_is_on_it() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let z = band.zero();
    for ratio in [100.0, 1e3] {
        let thin = Curve3::Ellipse {
            center: Point3::new(0.2, -0.1, 0.4),
            axis: Vec3::unit_z(),
            semi_u: ratio,
            semi_v: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let swept = Span::new(&thin, (0.5, 2.5));
        for t in [0.9, 1.3, 2.1] {
            let (p, d) = thin.ders1(t);
            let normal = Vec3::unit_z().cross(d).normalize();
            let vertex = p + normal * (0.5 * z);
            let leaving = Curve3::Line {
                origin: vertex,
                dir: normal,
            };
            let edges = [(vertex, Span::new(&leaving, (0.0, 1.0)))];
            match boundary(&swept, &edges, band) {
                Ok(Boundary::Meets { t: at, edge: 0, .. }) => {
                    assert!((at - t).abs() < 1e-6, "{ratio} at {t}: {at}");
                }
                other => panic!("{ratio} at {t}: {other:?}"),
            }
        }
    }
}

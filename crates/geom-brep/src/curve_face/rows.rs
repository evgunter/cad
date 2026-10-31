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
                (Spiric, SurfaceKind::Nurbs | Approx) => (Door::SplineFace, false),
                (_, SurfaceKind::Nurbs | Approx) => (Door::SplineFace, true),
                (Spiric, _) => (Door::Spiric, false),
                (Nurbs, Plane) => (Door::SplineComposite, true),
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
/// the disc either way. The meeting reading of the edge arriving there
/// reads its end in the gap of the line first (`curve_carrier_gap`), a
/// decision of the meeting.
#[test]
fn a_corner_in_the_gap_escalates() {
    let off = 3.0 * ZERO * FRAC_1_SQRT_2;
    let grazing = line([-off, 1.0 + off, 0.0], [1.0, 1.0, 0.0]);
    match read(&grazing, (-1.0, 1.0)) {
        Err(BoundaryRefusal::Meeting(diag)) => {
            assert_eq!(diag.predicate, Some("curve_carrier_gap"));
        }
        other => panic!("a corner in the gap: {other:?}"),
    }
}

/// **A boundary vertex within the band of a thin ellipse meets it.** An
/// ellipse of semi-axes `a/b` from 100 to 1000, and a boundary edge
/// starting half the band's zero off it along its in-plane normal, at
/// three parameters across the span, and leaving along that normal: the
/// pass reads a meeting there, at the vertex's own parameter. Both of its
/// readings find it (the vertex reading, and the meeting reading's end
/// candidate), each at the point's foot on the ellipse; an angle read as
/// the point's unit-coordinate image (`param_near`) would put it about
/// `(1 + a/b)` times further off, which the band decides off.
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

/// Review r2: zero-length-overlap poses are not runs.
mod review_r2 {
    use super::*;

    fn ellipse(axis: Vec3<f64>) -> Curve3<f64> {
        Curve3::Ellipse {
            center: Point3::origin(),
            axis,
            semi_u: 3.0,
            semi_v: 1.0,
            u_ref: Vec3::unit_x(),
        }
    }

    /// An elliptic arc's boundary: the arc, then its chord back.
    fn read_ellipse(
        carrier: &Curve3<f64>,
        params: (f64, f64),
        endless: bool,
    ) -> Result<Boundary<f64>, BoundaryRefusal> {
        let e = ellipse(Vec3::unit_z());
        let (a, b) = (e.eval(0.3), e.eval(2.0));
        let chord = line([b.x, b.y, b.z], [a.x - b.x, a.y - b.y, 0.0]);
        let edges = [
            (a, Span::new(&e, (0.3, 2.0))),
            (b, Span::new(&chord, (0.0, (a - b).norm()))),
        ];
        let mut s = Span::new(carrier, params);
        if endless {
            s = s.endless();
        }
        boundary(&s, &edges, band())
    }

    #[test]
    fn the_rest_of_an_ellipse_is_not_along_its_arc() {
        for axis in [Vec3::unit_z(), -Vec3::unit_z()] {
            let e = ellipse(axis);
            // The complement of (0.3, 2.0) on the same point set.
            let params = if axis.z > 0.0 {
                (2.0, 0.3 + 2.0 * PI)
            } else {
                (-0.3, 2.0 * PI - 2.0)
            };
            let got = read_ellipse(&e, params, false);
            assert!(matches!(got, Ok(Boundary::Clear)), "{axis:?}: {got:?}");
        }
    }

    #[test]
    fn an_elliptic_arc_on_its_own_rim_is_along() {
        let got = read_ellipse(&ellipse(Vec3::unit_z()), (0.3, 2.0), false);
        assert!(matches!(got, Ok(Boundary::Along { edge: 0 })), "{got:?}");
        let got = read_ellipse(&ellipse(Vec3::unit_z()), (0.5, 1.5), false);
        assert!(matches!(got, Ok(Boundary::Along { edge: 0 })), "{got:?}");
    }

    /// The whole ellipse, closed at 2.0 (a vertex of the rim): meets at
    /// the other rim vertex.
    #[test]
    fn a_closed_ellipse_through_the_rim_meets_at_its_far_vertex() {
        let got = read_ellipse(&ellipse(Vec3::unit_z()), (2.0, 2.0 + 2.0 * PI), false);
        assert!(
            matches!(got, Ok(Boundary::Meets { edge: 0, .. })),
            "{got:?}"
        );
        let got = read_ellipse(&ellipse(Vec3::unit_z()), (2.0, 2.0 + 2.0 * PI), true);
        assert!(
            matches!(got, Ok(Boundary::Meets { .. })),
            "endless: {got:?}"
        );
    }

    /// Collinear segments touching end to end share no run.
    #[test]
    fn collinear_segments_touching_end_to_end_are_clear() {
        let got = read(&line([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]), (0.0, 1.0));
        assert!(matches!(got, Ok(Boundary::Clear)), "{got:?}");
        let got = read(&line([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]), (0.0, 1.0));
        assert!(matches!(got, Ok(Boundary::Clear)), "{got:?}");
    }

    /// The rim's own circle as an endless span: it is the arc's carrier
    /// and passes the rim's two arc vertices inside itself.
    #[test]
    fn an_endless_circle_meets_the_rim_at_a_vertex() {
        let c = unit_circle(Vec3::unit_z());
        let got = read(&c, (0.0, 2.0 * PI));
        assert!(matches!(got, Ok(Boundary::Meets { .. })), "closed: {got:?}");
        let rim = rim();
        let spans = [(0.0, 1.0), (0.0, FRAC_PI_2), (0.0, 1.0)];
        let edges: Vec<_> = rim
            .iter()
            .zip(spans)
            .map(|(c, (t0, t1))| (c.eval(t0), Span::new(c, (t0, t1))))
            .collect();
        let got = boundary(&Span::new(&c, (0.0, 2.0 * PI)).endless(), &edges, band());
        assert!(
            matches!(got, Ok(Boundary::Meets { .. })),
            "endless: {got:?}"
        );
    }

    /// A line lying on the x axis rim edge but running past both ends
    /// meets at the corner it passes inside itself.
    #[test]
    fn a_line_longer_than_its_rim_edge_meets_at_the_far_corner() {
        let got = read(&line([-0.5, 0.0, 0.0], [1.0, 0.0, 0.0]), (0.0, 2.0));
        assert!(matches!(got, Ok(Boundary::Meets { .. })), "{got:?}");
    }

    /// A circle-as-ellipse (equal semi-axes) arc on the rim's arc.
    #[test]
    fn a_round_ellipse_on_the_rim_arc() {
        let round = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            semi_u: 1.0,
            semi_v: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let along = read(&round, (0.2, 1.2));
        let rest = read(&round, (FRAC_PI_2, 2.0 * PI));
        eprintln!("round along {along:?}; rest {rest:?}");
        assert!(!matches!(rest, Ok(Boundary::Along { .. })), "{rest:?}");
    }
}

/// **A vertex between two boundary edges the span runs along is found
/// by the vertex reading alone.** The boundary's rim is two arcs of one
/// circle, `[0, ½]` and `[½, π/2]`, and the span is the whole of
/// `[0, π/2]`: each arc shares a run with it, and the meeting reading
/// gives a shared pose no point, so only the second arc's start vertex,
/// read on the span, puts the joint at `t = ½`. Read `Along` without it,
/// and the joint vertex of the face's body is never recorded on the
/// span.
#[test]
fn a_joint_between_two_runs_meets_the_span() {
    let circle = unit_circle(Vec3::unit_z());
    let [x_axis, _, y_axis] = rim();
    let edges = [
        (x_axis.eval(0.0), Span::new(&x_axis, (0.0, 1.0))),
        (circle.eval(0.0), Span::new(&circle, (0.0, 0.5))),
        (circle.eval(0.5), Span::new(&circle, (0.5, FRAC_PI_2))),
        (y_axis.eval(0.0), Span::new(&y_axis, (0.0, 1.0))),
    ];
    match boundary(&Span::new(&circle, (0.0, FRAC_PI_2)), &edges, band()) {
        Ok(Boundary::Meets { t, edge: 2, .. }) => {
            assert!((t - 0.5).abs() < 1e-12, "at the joint: {t}");
        }
        other => panic!("two runs and their joint: {other:?}"),
    }
}

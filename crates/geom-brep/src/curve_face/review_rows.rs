//! Review rows (PR 5768): zero-length and partial overlaps against the
//! boundary pass, beyond the quarter disc's own rows.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Vec3};

use super::{Boundary, BoundaryRefusal, boundary};
use crate::curve_pair::Span;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

fn circle(axis: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::origin(),
        axis,
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

fn read_on(
    edges: &[(Curve3<f64>, (f64, f64))],
    carrier: &Curve3<f64>,
    params: (f64, f64),
) -> Result<Boundary<f64>, BoundaryRefusal> {
    let spans: Vec<_> = edges
        .iter()
        .map(|(c, (t0, t1))| (c.eval(*t0), Span::new(c, (*t0, *t1))))
        .collect();
    boundary(&Span::new(carrier, params), &spans, band())
}

/// A disc bounded by one closed circle edge, its vertex at angle 0.
fn disc() -> Vec<(Curve3<f64>, (f64, f64))> {
    vec![(circle(Vec3::unit_z()), (0.0, TAU))]
}

#[test]
fn closed_rim_arc_inside_is_along() {
    let r = read_on(&disc(), &circle(Vec3::unit_z()), (0.5, 1.5));
    assert!(matches!(r, Ok(Boundary::Along { edge: 0 })), "{r:?}");
}

#[test]
fn closed_rim_arc_across_its_vertex_meets_there() {
    let r = read_on(&disc(), &circle(Vec3::unit_z()), (-0.5, 0.5));
    match r {
        Ok(Boundary::Meets { t, edge: 0, .. }) => assert!(t.abs() < 1e-9, "{t}"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn closed_rim_whole_circle_is_along() {
    let r = read_on(&disc(), &circle(-Vec3::unit_z()), (0.0, TAU));
    assert!(matches!(r, Ok(Boundary::Along { edge: 0 })), "{r:?}");
}

/// A whole circle whose seam is elsewhere than the disc's vertex: the
/// vertex is inside the span, a meeting there.
#[test]
fn closed_rim_whole_circle_seamed_elsewhere_meets_at_the_vertex() {
    let r = read_on(&disc(), &circle(Vec3::unit_z()), (1.0, 1.0 + TAU));
    assert!(matches!(r, Ok(Boundary::Meets { edge: 0, .. })), "{r:?}");
}

fn quarter_ellipse() -> Vec<(Curve3<f64>, (f64, f64))> {
    let e = Curve3::Ellipse {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        semi_u: 3.0,
        semi_v: 1.0,
        u_ref: Vec3::unit_x(),
    };
    vec![
        (
            Curve3::Line {
                origin: Point3::origin(),
                dir: Vec3::unit_x(),
            },
            (0.0, 3.0),
        ),
        (e, (0.0, FRAC_PI_2)),
        (
            Curve3::Line {
                origin: Point3::new(0.0, 1.0, 0.0),
                dir: -Vec3::unit_y(),
            },
            (0.0, 1.0),
        ),
    ]
}

fn ellipse(axis: Vec3<f64>) -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::origin(),
        axis,
        semi_u: 3.0,
        semi_v: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

#[test]
fn the_rest_of_an_ellipse_is_clear() {
    let r = read_on(
        &quarter_ellipse(),
        &ellipse(Vec3::unit_z()),
        (FRAC_PI_2, TAU),
    );
    assert!(matches!(r, Ok(Boundary::Clear)), "{r:?}");
}

#[test]
fn an_ellipse_arc_reversed_lies_along() {
    let r = read_on(
        &quarter_ellipse(),
        &ellipse(-Vec3::unit_z()),
        (-FRAC_PI_2, 0.0),
    );
    assert!(matches!(r, Ok(Boundary::Along { edge: 1 })), "{r:?}");
}

#[test]
fn an_ellipse_sub_run_lies_along() {
    let r = read_on(&quarter_ellipse(), &ellipse(Vec3::unit_z()), (0.3, 1.1));
    assert!(matches!(r, Ok(Boundary::Along { edge: 1 })), "{r:?}");
}

#[test]
fn an_ellipse_run_past_the_corner_meets_there() {
    let r = read_on(&quarter_ellipse(), &ellipse(Vec3::unit_z()), (0.3, 2.0));
    match r {
        Ok(Boundary::Meets { t, edge: 2, .. }) => assert!((t - FRAC_PI_2).abs() < 1e-9, "{t}"),
        other => panic!("{other:?}"),
    }
}

fn rim() -> Vec<(Curve3<f64>, (f64, f64))> {
    vec![
        (
            Curve3::Line {
                origin: Point3::origin(),
                dir: Vec3::unit_x(),
            },
            (0.0, 1.0),
        ),
        (circle(Vec3::unit_z()), (0.0, FRAC_PI_2)),
        (
            Curve3::Line {
                origin: Point3::new(0.0, 1.0, 0.0),
                dir: -Vec3::unit_y(),
            },
            (0.0, 1.0),
        ),
    ]
}

#[test]
fn a_line_sub_run_lies_along_and_one_across_the_corner_meets() {
    let x = Curve3::Line {
        origin: Point3::origin(),
        dir: Vec3::unit_x(),
    };
    let r = read_on(&rim(), &x, (0.2, 0.7));
    assert!(matches!(r, Ok(Boundary::Along { edge: 0 })), "{r:?}");
    let r = read_on(&rim(), &x, (-0.5, 0.5));
    assert!(matches!(r, Ok(Boundary::Meets { edge: 0, .. })), "{r:?}");
}

/// Two arcs of one circle sharing a run shorter than the band's zero
/// at a corner: a touch at the corner, never a run.
#[test]
fn an_overlap_inside_the_zero_is_no_run() {
    let r = read_on(
        &rim(),
        &circle(Vec3::unit_z()),
        (FRAC_PI_2 - 0.3 * ZERO, PI),
    );
    assert!(
        !matches!(r, Ok(Boundary::Along { .. }) | Ok(Boundary::Meets { .. })),
        "{r:?}"
    );
}

/// A rational quadratic quarter circle along the rim's arc: the walk
/// reports no shared run, so the pass must refuse, never read Clear.
#[test]
fn a_spline_along_the_rim_refuses() {
    use geom::NurbsCurve3;
    use geom_core::spline::{KnotVector, Knots};
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let s = Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(
            Knots::from(knots),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            vec![1.0, FRAC_1_SQRT_2, 1.0],
        )
        .unwrap(),
    ));
    let r = read_on(&rim(), &s, (0.0, 1.0));
    assert!(r.is_err(), "{r:?}");
    // And a spline over a sub-run of the rim's arc.
    let r = read_on(&rim(), &s, (0.2, 0.6));
    assert!(r.is_err(), "sub-run: {r:?}");
}

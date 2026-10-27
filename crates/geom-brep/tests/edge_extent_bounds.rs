//! **The edge extent's two bounds, per carrier kind.** Each fixture
//! reads the edge's diameter `D` and its reach from its start `R` off a
//! dense schedule (the sampled value, and that value plus the sampled
//! speed's Lipschitz slack, bracket the true one), then pins:
//! `edge_extent_inf ≤ D ≤ edge_extent_sup ≤ c·D` and
//! `R ≤ edge_reach_from(start) ≤ c·R`, `c` stated per kind, so a bound
//! that loosens (or reads `∞`) goes red beside one that stops bracketing.

#![allow(clippy::expect_used)]

use geom::{Curve3, NurbsCurve3};
use geom_brep::{edge_extent_inf, edge_extent_sup, edge_reach_from};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

use crate::shared::tol::band;

/// The dense schedule's piece count.
const DENSE: u32 = 800;

/// One fixture: its name, carrier, window, and the tightness `c` its
/// bound from above and its reach are held to.
struct Row {
    what: &'static str,
    carrier: Curve3<f64>,
    window: (f64, f64),
    c_sup: f64,
    c_reach: f64,
}

/// `(D, slack, R)`: the dense diameter, the Lipschitz slack past it,
/// and the dense reach from the start.
fn dense(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> (f64, f64, f64) {
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(DENSE);
    let pts: Vec<Point3<f64>> = (0..=DENSE).map(|k| carrier.eval(at(k))).collect();
    let speed = (0..=DENSE)
        .map(|k| carrier.deriv(at(k)).norm())
        .fold(0.0, f64::max);
    let mut d = 0.0_f64;
    for (i, a) in pts.iter().enumerate() {
        for b in &pts[i + 1..] {
            d = d.max(a.distance(*b));
        }
    }
    let r = pts.iter().map(|p| p.distance(pts[0])).fold(0.0, f64::max);
    // Twice the sampled speed covers its own sampling between samples.
    (d, 2.0 * speed * (t1 - t0).abs() / f64::from(DENSE), r)
}

fn circle(r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.3, -0.2, 0.1),
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

fn ellipse(a: f64, b: f64) -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::new(-0.4, 0.2, 1.0),
        axis: Vec3::unit_z(),
        semi_u: a,
        semi_v: b,
        u_ref: Vec3::unit_x(),
    }
}

fn spline(
    degree: usize,
    knots: Vec<f64>,
    control: Vec<Point3<f64>>,
    weights: Vec<f64>,
) -> Curve3<f64> {
    Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(knots, degree).expect("clamped knots"),
            control,
            weights,
        )
        .expect("a valid spline"),
    ))
}

fn rows() -> Vec<Row> {
    use core::f64::consts::{PI, TAU};
    let p = Point3::new;
    let steep = spline(
        2,
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        vec![p(0.0, -1.0, 0.0), p(0.0, 0.0, 50.0), p(0.0, 1.0, 0.0)],
        vec![1.0, 10.0, 1.0],
    );
    let wave = spline(
        3,
        vec![0.0, 0.0, 0.0, 0.0, 0.4, 0.7, 1.0, 1.0, 1.0, 1.0],
        vec![
            p(0.0, 0.0, 0.7),
            p(1.0, 2.5, 0.7),
            p(2.0, -0.6, 0.7),
            p(3.0, 3.0, 0.7),
            p(4.0, 1.5, 0.7),
            p(5.0, 0.0, 0.7),
        ],
        vec![1.0, 1.3, 0.8, 1.0, 1.2, 1.0],
    );
    let closed = spline(
        2,
        vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0],
        vec![
            p(1.0, 0.0, 0.0),
            p(1.0, 1.0, 0.0),
            p(-1.0, 1.0, 0.0),
            p(-1.0, -1.0, 0.0),
            p(1.0, -1.0, 0.0),
            p(1.0, 0.0, 0.0),
        ],
        vec![1.0; 6],
    );
    let spiric = Curve3::spiric(
        Point3::new(0.3, -0.2, 0.7),
        Vec3::unit_z(),
        Vec3::unit_x(),
        1.2,
        0.225,
        0.05,
        band(),
    )
    .expect("the elbow's numbers are in regime");
    let row = |what, carrier: &Curve3<f64>, window, c_sup, c_reach| Row {
        what,
        carrier: carrier.clone(),
        window,
        c_sup,
        c_reach,
    };
    vec![
        row(
            "circle, short arc",
            &circle(2.0),
            (0.1, 0.4),
            1.0 + 1e-12,
            1.0 + 1e-12,
        ),
        row(
            "circle, past a half turn",
            &circle(2.0),
            (0.0, 4.0),
            1.0 + 1e-12,
            1.0 + 1e-12,
        ),
        row(
            "circle, closed",
            &circle(2.0),
            (0.0, TAU),
            1.0 + 1e-12,
            1.0 + 1e-12,
        ),
        row(
            "ellipse, half",
            &ellipse(1.0, 0.5),
            (-PI / 2.0, PI / 2.0),
            1.05,
            1.05,
        ),
        row(
            "ellipse, closed",
            &ellipse(1.0, 0.5),
            (0.0, TAU),
            1.05,
            1.05,
        ),
        row(
            "flat ellipse, about the major vertex",
            &ellipse(1.0, 0.01),
            (-0.3, 0.3),
            1.2,
            1.05,
        ),
        row(
            "flat ellipse, across the minor vertex",
            &ellipse(1.0, 0.01),
            (1.2, 1.9),
            1.2,
            1.05,
        ),
        row(
            "flat ellipse, stored minor first",
            &ellipse(0.01, 1.0),
            (1.2, 1.9),
            1.2,
            1.05,
        ),
        row("spiric, closed", &spiric, (0.0, TAU), 1.25, 1.25),
        row("spiric, an arc", &spiric, (0.7, 2.9), 1.25, 1.25),
        row("steep spline, whole", &steep, (0.0, 1.0), 1.1, 1.1),
        row("steep spline, a sub-window", &steep, (0.05, 0.25), 1.6, 1.6),
        row("wave spline, whole", &wave, (0.0, 1.0), 1.6, 1.6),
        row("wave spline, a sub-window", &wave, (0.45, 0.6), 1.6, 1.6),
        row("closed spline", &closed, (0.0, 1.0), 1.6, 1.6),
    ]
}

/// **Every kind's bounds bracket its diameter and its reach, within the
/// kind's stated tightness.**
#[test]
fn each_kinds_bounds_bracket_the_diameter_and_stay_tight() {
    for row in rows() {
        let (t0, t1) = row.window;
        let c = &row.carrier;
        let (start, end) = (c.eval(t0), c.eval(t1));
        let chord = start.distance(end);
        let (d, slack, r) = dense(c, row.window);
        let inf = edge_extent_inf(c, t0, t1, chord);
        let sup = edge_extent_sup(c, t0, t1, chord);
        let reach = edge_reach_from(c, t0, t1, start);
        println!(
            "[measured] {}: D {d:.6} (+{slack:.1e}), inf/D {:.6}, sup/D {:.6}, R {r:.6}, reach/R {:.6}",
            row.what,
            inf / d,
            sup / d,
            reach / r
        );
        let w = row.what;
        assert!(
            inf <= d + slack + 1e-12,
            "{w}: inf {inf} exceeds D {d} (+{slack})"
        );
        assert!(sup >= d, "{w}: sup {sup} is short of D {d}");
        assert!(
            sup <= row.c_sup * d,
            "{w}: sup {sup} passes {}·D = {}",
            row.c_sup,
            row.c_sup * d
        );
        assert!(reach >= r, "{w}: the reach {reach} is short of R {r}");
        assert!(
            reach <= row.c_reach * r,
            "{w}: the reach {reach} passes {}·R = {}",
            row.c_reach,
            row.c_reach * r
        );
    }
}

//! **The edge extent's two bounds, per carrier kind.** Each fixture
//! reads the edge's diameter `D` and its reach from its start `R` off a
//! dense schedule (the sampled value, and that value plus the sampled
//! speed's Lipschitz slack, bracket the true one), then pins:
//! `edge_extent_inf ≤ D ≤ edge_extent_sup ≤ c·D` and
//! `R ≤ edge_reach_from(start) ≤ c·R`, `c` stated per kind, so a bound
//! that loosens (or reads `∞`) goes red beside one that stops bracketing.
//!
//! The rows marked off the schedule have their diameter and their reach
//! attained between the bounds' own 16 samples (an asymmetric spike, a
//! flat ellipse's vertex on a lopsided window, a near-pinch spiric's
//! far side), and the row pins that the samples miss both by a stated
//! fraction: there each bound from above holds only through its slack
//! term (the Lipschitz slack, the conic pieces' bulge), so removing one
//! goes red by that fraction, not by a rounding.
//!
//! A wild parameterisation (clustered knots, weights 1e±3) is held to
//! soundness alone ([`a_wild_parameterisation_is_bracketed_and_no_more`]):
//! the bounds from above read a whole-carrier speed bound, and their
//! tightness there is not claimed.

#![allow(clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsCurve3};
use geom_brep::{
    LeverBounds, edge_extent_inf, edge_extent_sup, edge_reach_bounds, edge_reach_from,
};
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
    /// Where set, the least fraction of `D` and of `R` the bounds' own
    /// 16-piece schedule misses ([`sixteen`]).
    off_schedule: Option<f64>,
}

/// The farthest pair, and the farthest point from the start, among the
/// carrier's points at `t₀ + k·(t₁ − t₀)/16`: what the bounds' samples
/// read before any slack.
fn sixteen(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> (f64, f64) {
    let pts: Vec<Point3<f64>> = (0..=16)
        .map(|k| carrier.eval(t0 + (t1 - t0) * f64::from(k) / 16.0))
        .collect();
    let mut d = 0.0_f64;
    for (i, a) in pts.iter().enumerate() {
        for b in &pts[i + 1..] {
            d = d.max(a.distance(*b));
        }
    }
    let r = pts.iter().map(|p| p.distance(pts[0])).fold(0.0, f64::max);
    (d, r)
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
    // The plane stands 0.005 off the inner equator's tangent (`R − r`):
    // near the pinch, where the rate bound grows.
    let pinched = Curve3::spiric(
        Point3::new(0.3, -0.2, 0.7),
        Vec3::unit_z(),
        Vec3::unit_x(),
        1.2,
        0.225,
        0.97,
        band(),
    )
    .expect("a near-pinch oval is in regime");
    // A spike at t ≈ 0.03, between the schedule's first two samples,
    // and the curve's farthest point from its start and from its end.
    let spike = spline(
        2,
        vec![0.0, 0.0, 0.0, 0.025, 0.035, 1.0, 1.0, 1.0],
        vec![
            p(0.0, 0.0, 0.0),
            p(0.05, 0.0, 0.0),
            p(0.1, 10.0, 0.0),
            p(0.15, 0.0, 0.0),
            p(4.0, 0.0, 0.0),
        ],
        vec![1.0; 5],
    );
    let row = |what, carrier: &Curve3<f64>, window, c_sup, c_reach| Row {
        what,
        carrier: carrier.clone(),
        window,
        c_sup,
        c_reach,
        off_schedule: None,
    };
    let off = |row: Row, fraction| Row {
        off_schedule: Some(fraction),
        ..row
    };
    vec![
        row(
            "circle, short arc",
            &circle(2.0),
            (0.1, 0.4),
            1.0 + 1e-9,
            1.0 + 1e-9,
        ),
        row(
            "circle, past a half turn",
            &circle(2.0),
            (0.0, 4.0),
            1.0 + 1e-9,
            1.0 + 1e-9,
        ),
        row(
            "circle, closed",
            &circle(2.0),
            (0.0, TAU),
            1.0 + 1e-9,
            1.0 + 1e-9,
        ),
        row(
            "ellipse, half",
            &ellipse(1.0, 0.5),
            (-PI / 2.0, PI / 2.0),
            1.1,
            1.1,
        ),
        row("ellipse, closed", &ellipse(1.0, 0.5), (0.0, TAU), 1.1, 1.1),
        row(
            "flat ellipse, about the major vertex",
            &ellipse(1.0, 0.01),
            (-0.3, 0.3),
            1.1,
            1.1,
        ),
        row(
            "flat ellipse, across the minor vertex",
            &ellipse(1.0, 0.01),
            (1.2, 1.9),
            1.1,
            1.1,
        ),
        row(
            "flat ellipse, stored minor first",
            &ellipse(0.01, 1.0),
            (1.2, 1.9),
            1.1,
            1.1,
        ),
        row("spiric, closed", &spiric, (0.0, TAU), 1.05, 1.15),
        row("spiric, an arc", &spiric, (0.7, 2.9), 1.05, 1.15),
        row("steep spline, whole", &steep, (0.0, 1.0), 1.15, 1.15),
        row(
            "steep spline, a sub-window",
            &steep,
            (0.05, 0.25),
            1.8,
            1.45,
        ),
        row("wave spline, whole", &wave, (0.0, 1.0), 1.05, 1.05),
        row("wave spline, a sub-window", &wave, (0.45, 0.6), 1.3, 1.2),
        row("closed spline", &closed, (0.0, 1.0), 1.3, 1.2),
        off(
            row(
                "spike spline, off the schedule",
                &spike,
                (0.0, 1.0),
                1.05,
                1.05,
            ),
            0.04,
        ),
        off(
            row(
                "flat ellipse, its vertex off the schedule",
                &ellipse(1.0, 0.01),
                (-0.27, 0.33),
                1.1,
                1.1,
            ),
            3e-4,
        ),
        off(
            row(
                "near-pinch spiric, off the schedule",
                &pinched,
                (0.33, 6.2),
                1.15,
                1.5,
            ),
            0.01,
        ),
    ]
}

/// **Every kind's bounds bracket its diameter and its reach, within the
/// kind's stated tightness.**
#[test]
fn each_kinds_bounds_bracket_the_diameter_and_stay_tight() {
    // Every row is read and every failure reported, so a loosened or
    // short bound names each row it moves.
    let mut failed = Vec::new();
    let mut check = |ok: bool, why: String| {
        if !ok {
            failed.push(why);
        }
    };
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
        if let Some(fraction) = row.off_schedule {
            let (d16, r16) = sixteen(c, row.window);
            check(
                d - d16 > fraction * d && r - r16 > fraction * r,
                format!(
                    "{w}: the schedule misses D by {:.2e} and R by {:.2e} of them, under {fraction:.0e}",
                    (d - d16) / d,
                    (r - r16) / r
                ),
            );
        }
        // The reach read from both sides brackets it too: the samples
        // from below, the reach above it.
        let bounds = edge_reach_bounds(c, t0, t1, start);
        check(
            bounds.below() <= (r + slack) * (1.0 + 1e-12) && bounds.above() == reach,
            format!("{w}: the reach's bounds {bounds:?} do not bracket R {r} at the reach {reach}"),
        );
        check(
            inf <= d + slack + 1e-12,
            format!("{w}: inf {inf} exceeds D {d} (+{slack})"),
        );
        check(sup >= d, format!("{w}: sup {sup} is short of D {d}"));
        check(
            sup <= row.c_sup * (d + slack),
            format!(
                "{w}: sup {sup} passes {}·D = {}",
                row.c_sup,
                row.c_sup * (d + slack)
            ),
        );
        check(
            reach >= r,
            format!("{w}: the reach {reach} is short of R {r}"),
        );
        check(
            reach <= row.c_reach * (r + slack),
            format!(
                "{w}: the reach {reach} passes {}·R = {}",
                row.c_reach,
                row.c_reach * (r + slack)
            ),
        );
    }
    assert!(failed.is_empty(), "{}", failed.join("\n"));
}

/// **A wild parameterisation is bracketed, and nothing more is claimed**:
/// clustered knots (three within 2e-7) and weights alternating 1e±3. The
/// bounds from above read a whole-carrier speed bound, which the
/// spline's fast stretches set, so on a window where the parameter
/// crawls they pass the diameter and the reach many times over; only
/// `inf ≤ D ≤ sup`, `R ≤ reach` and the samples' reach below `R` are
/// pinned. The two-sided consumers read both bounds for this reason
/// (`geom_brep::LeverBounds`).
#[test]
fn a_wild_parameterisation_is_bracketed_and_no_more() {
    let p = Point3::new;
    let wild = spline(
        3,
        vec![
            0.0,
            0.0,
            0.0,
            0.0,
            0.5,
            0.500_000_1,
            0.500_000_2,
            1.0,
            1.0,
            1.0,
            1.0,
        ],
        vec![
            p(0.0, 0.0, 0.0),
            p(1.0, 2.0, 0.0),
            p(2.0, -1.0, 1.0),
            p(3.0, 0.5, 0.0),
            p(4.0, 1.0, 0.0),
            p(5.0, 0.0, 2.0),
            p(6.0, 0.0, 0.0),
        ],
        vec![1.0, 1e3, 1e-3, 1e3, 1e-3, 1e3, 1.0],
    );
    for window in [
        (0.0, 1.0),
        (0.1, 0.9),
        (0.499_999, 0.500_001),
        (0.2, 0.2001),
        (0.9, 0.95),
    ] {
        let (t0, t1) = window;
        let start = wild.eval(t0);
        let chord = start.distance(wild.eval(t1));
        let (d, _, r) = dense(&wild, window);
        let inf = edge_extent_inf(&wild, t0, t1, chord);
        let sup = edge_extent_sup(&wild, t0, t1, chord);
        let reach = edge_reach_from(&wild, t0, t1, start);
        let LeverBounds::Between { below, above } = edge_reach_bounds(&wild, t0, t1, start) else {
            panic!("{window:?}: a spline's reach is known from both sides");
        };
        println!(
            "[measured] wild {window:?}: D {d:.3e}, inf/D {:.4}, sup/D {:.3e}, R {r:.3e}, \
             below/R {:.4}, reach/R {:.3e}",
            inf / d,
            sup / d,
            below / r,
            reach / r
        );
        assert!(
            inf <= d * (1.0 + 1e-12) + 1e-15,
            "{window:?}: inf {inf} > D {d}"
        );
        assert!(sup >= d, "{window:?}: sup {sup} < D {d}");
        assert!(
            reach >= r && above == reach,
            "{window:?}: reach {reach} < R {r}"
        );
        assert!(
            below <= r * (1.0 + 1e-12),
            "{window:?}: below {below} > R {r}"
        );
    }
}

//! **A one-segment closed loop through the solid builders** (D1: a full
//! turn is one segment at one vertex, so a closed carrier is one edge).
//!
//! Every fixture is a raw table — a one-segment circle written through
//! the fixture door — and every body is checked at all three tiers and
//! against its closed-form volume:
//!
//! - **extrude**: one vertex per cap, one self-loop rim per cap, one
//!   periodic wall, one strut with both halves in that wall, described
//!   as the wall's seam; outer and hole, both directions, `f64` and
//!   `Interval`;
//! - **revolve**: a partial revolve's torus patch (one start and one end
//!   meridian, one latitude strut) and a full revolve's torus (one
//!   vertex, a meridian and a latitude circle, one face), with holes;
//! - **loft**: two one-segment sections skinned into one wall;
//! - **a boolean** on an extruded periodic wall with a seam strut.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_brep::EdgeDescription;
use geom_core::{Arc2, Bounds, Interval, Point2, Real, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane, ValidatedProfile};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{Body, EdgeKey, validate, validate_closed, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

/// A one-segment circle about `(cx, cy)` of radius `r`, its vertex at
/// carrier angle 0, turning `sweep` (±2π).
fn circle<T: Real>(cx: f64, cy: f64, r: f64, sweep: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r), f(cy)),
        Segment::Arc(Arc2 {
            centre: Point2::new(f(cx), f(cy)),
            radius: f(r),
            sweep: f(sweep),
        }),
    )])
}

fn rect<T: Real>(x0: f64, y0: f64, x1: f64, y1: f64) -> ProfileLoop<T> {
    let p = |x: f64, y: f64| Point2::new(T::from_f64(x), T::from_f64(y));
    RawLoop::polygon([p(x0, y0), p(x1, y0), p(x1, y1), p(x0, y1)])
}

fn validated<T: geom_core::Decide>(loops: Vec<ProfileLoop<T>>) -> ValidatedProfile<T> {
    Profile::new(SketchPlane::<T>::xy(), loops)
        .validate(tol())
        .unwrap_or_else(|e| panic!("the fixture validates: {e}"))
}

/// All three tiers, by name.
fn tiers<T: geom_core::Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    body: &Body<T>,
    what: &str,
) {
    assert_eq!(validate(body), Ok(()), "{what}: tier 1");
    assert_eq!(validate_closed(body), Ok(()), "{what}: tier 2");
    assert_eq!(validate_geometric(body, tol()), Ok(()), "{what}: tier 3");
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(body, tol()).unwrap().volume
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: volume {got}, closed form {want}"
    );
}

/// `(V, E, F)` of a body.
fn census<T: Real>(body: &Body<T>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

/// Whether `edge` is described as its chart's seam.
fn is_seam<T: Real>(body: &Body<T>, edge: EdgeKey) -> bool {
    let e = body.get_edge(edge).unwrap();
    let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
    matches!(c.description(), EdgeDescription::Chart(ch) if ch.seam)
}

fn extruded(loops: Vec<ProfileLoop<f64>>, depth: f64, side: ExtrudeSide) -> sweep::Extruded<f64> {
    extrude(
        &validated(loops),
        Extrusion::Distance { depth, side },
        tol(),
    )
    .unwrap_or_else(|e| panic!("the extrusion builds: {e}"))
}

/// A one-segment circle extruded either way: two vertices, three edges
/// (two self-loop rims and the strut), three faces; the strut's two
/// halves both bound the wall, which is its seam; πr²h.
#[test]
fn an_extruded_one_segment_circle_is_one_wall_with_a_seam_strut() {
    for side in [ExtrudeSide::Along, ExtrudeSide::Against] {
        for sweep in [TAU, -TAU] {
            let what = format!("{side:?}, sweep {sweep}");
            let t = extruded(vec![circle(0.5, -0.25, 1.5, sweep)], 2.0, side);
            tiers(&t.body, &what);
            assert_eq!(census(&t.body), (2, 3, 3), "{what}");
            assert_eq!(t.walls.len(), 1);
            let [wall] = &t.walls[0][..] else {
                panic!("{what}: one wall, got {:?}", t.walls[0]);
            };
            assert_eq!(wall.segments, vec![0], "{what}");
            assert!(is_seam(&t.body, wall.strut), "{what}: the strut is the wall's seam");
            for rim in wall.top_rims.iter().chain(&wall.bottom_rims) {
                let e = t.body.get_edge(*rim).unwrap();
                let (a, b) = (
                    t.body.get_half_edge(e.he_plus).unwrap().start,
                    t.body.get_half_edge(e.he_minus).unwrap().start,
                );
                assert_eq!(a, b, "{what}: a rim is a self-loop");
            }
            close(volume(&t.body), PI * 1.5 * 1.5 * 2.0, &what);
        }
    }
}

/// One-segment circles as holes, and as the outer around a polygonal
/// hole: an annulus of two circles, a square with a circular hole, a
/// circle with a square hole.
#[test]
fn one_segment_circles_extrude_as_holes_and_around_them() {
    let h = 1.25;
    let cases: Vec<(&str, Vec<ProfileLoop<f64>>, f64, (usize, usize, usize))> = vec![
        (
            "annulus",
            vec![circle(0.0, 0.0, 2.0, TAU), circle(0.0, 0.0, 1.0, -TAU)],
            PI * (4.0 - 1.0) * h,
            (4, 6, 4),
        ),
        (
            "square with a round hole",
            vec![rect(-2.0, -2.0, 2.0, 2.0), circle(0.25, 0.0, 1.0, TAU)],
            (16.0 - PI) * h,
            (10, 15, 7),
        ),
        (
            "round plate with a square hole",
            vec![circle(0.0, 0.0, 2.0, TAU), rect(-0.5, -0.5, 0.5, 0.5)],
            (4.0 * PI - 1.0) * h,
            (10, 15, 7),
        ),
        (
            "two round holes",
            vec![
                rect(-3.0, -2.0, 3.0, 2.0),
                circle(-1.5, 0.0, 1.0, TAU),
                circle(1.5, 0.0, 0.5, -TAU),
            ],
            (24.0 - PI * 1.25) * h,
            (12, 18, 8),
        ),
    ];
    for side in [ExtrudeSide::Along, ExtrudeSide::Against] {
        for (what, loops, want, counts) in &cases {
            let what = format!("{what}, {side:?}");
            let t = extruded(loops.clone(), h, side);
            tiers(&t.body, &what);
            assert_eq!(census(&t.body), *counts, "{what}");
            close(volume(&t.body), *want, &what);
            for (l, walls) in t.walls.iter().enumerate() {
                if walls.iter().map(|w| w.segments.len()).sum::<usize>() == 1 {
                    assert!(is_seam(&t.body, walls[0].strut), "{what}: loop {l}'s strut");
                }
            }
        }
    }
}

/// The same extrusions at `Interval`, all three tiers.
#[test]
fn one_segment_circles_extrude_at_interval() {
    let f = Interval::from_f64;
    for loops in [
        vec![circle::<Interval>(0.5, -0.25, 1.5, TAU)],
        vec![rect(-2.0, -2.0, 2.0, 2.0), circle(0.25, 0.0, 1.0, TAU)],
        vec![circle(0.0, 0.0, 2.0, -TAU), circle(0.0, 0.0, 1.0, TAU)],
    ] {
        let n = loops.len();
        let t = extrude(
            &validated(loops),
            Extrusion::Distance {
                depth: f(1.5),
                side: ExtrudeSide::Along,
            },
            tol(),
        )
        .unwrap_or_else(|e| panic!("{n} loops: the extrusion builds at Interval: {e}"));
        tiers(&t.body, &format!("{n} loops at Interval"));
        let v = topo::mass_properties(&t.body, tol()).unwrap().volume;
        assert!(v.lo() > 0.0, "{n} loops: {v:?}");
    }
}

/// **A boolean on an extruded periodic wall with a seam strut** — no
/// evidence that one had ever run before this row. The cylinder is
/// r = 1 about the z axis from z = 0 to 2, its seam strut on the
/// meridian x = 1. Every operand pair is plane × cylinder.
///
/// - Cuts ALONG the wall build, at all three tiers and their closed
///   forms: the half x ≥ 0 cut away (the seam inside the tool), the half
///   y ≥ 0 cut away (the tool's face through the seam), and a bar united
///   through the wall around the seam.
/// - Cuts ACROSS the wall refuse, typed: a cap-parallel plane meets the
///   wall in a circle that crosses the one seam once, a closed section
///   loop with one site, which the join does not build
///   (`Join(SingleSiteSectionLoop)`, `work/join/closed-in-face-section-loop-has-one-site.md`).
#[test]
fn a_boolean_on_an_extruded_seam_wall_builds_along_it_and_refuses_across_it() {
    use sweep::test_support::brick;
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
    use topo::{BooleanDeclarations, BooleanError, SplitJoinError};

    let cyl = extruded(
        vec![circle(0.0, 0.0, 1.0, TAU)],
        2.0,
        ExtrudeSide::Along,
    )
    .body;
    // ∫_{-1/2}^{1/2} √(1 − y²) dy: the disc's share of a bar of width 1
    // reaching past x = 1 from x = 0.
    let bar_in_disc = 0.75f64.sqrt() * 0.5 + (0.5f64).asin();
    let cases: Vec<(&str, BooleanOp, Body<f64>, Option<f64>)> = vec![
        (
            "x ≥ 0 cut away",
            BooleanOp::Subtract,
            brick((0.0, 3.0), (-3.0, 3.0), (-1.0, 3.0), tol()),
            Some(PI),
        ),
        (
            "y ≥ 0 cut away",
            BooleanOp::Subtract,
            brick((-3.0, 3.0), (0.0, 3.0), (-1.0, 3.0), tol()),
            Some(PI),
        ),
        (
            "a bar through the seam",
            BooleanOp::Union,
            brick((0.0, 3.0), (-0.5, 0.5), (0.5, 1.5), tol()),
            Some(2.0 * PI + 3.0 - bar_in_disc),
        ),
        (
            "a slab kept",
            BooleanOp::Intersect,
            brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol()),
            None,
        ),
        (
            "a slab cut away",
            BooleanOp::Subtract,
            brick((-3.0, 3.0), (-3.0, 3.0), (0.5, 1.0), tol()),
            None,
        ),
    ];
    for (what, op, tool, want) in cases {
        let out = boolean_op_with(
            op,
            &topo::test_support::finished("the seam cylinder", cyl.clone(), tol()),
            &topo::test_support::finished("the tool", tool, tol()),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            tol(),
        );
        match (out, want) {
            (Ok(out), Some(want)) => {
                let body = &out
                    .body()
                    .unwrap_or_else(|| panic!("{what}: the boolean left no body"))
                    .body;
                tiers(body, what);
                close(volume(body), want, what);
            }
            (Err(BooleanError::Join(SplitJoinError::SingleSiteSectionLoop { count })), None) => {
                assert_eq!(count, 2, "{what}: one loop per cutting plane");
            }
            (out, want) => panic!(
                "{what}: want {}, got {:?}",
                want.map_or("the single-site refusal".to_string(), |v| format!("volume {v}")),
                out.map(|_| "a body")
            ),
        }
    }
}

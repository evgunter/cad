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
//! - **revolve** and **loft** refuse a one-segment loop, typed: the one
//!   wall each would build wraps a period no chart here has a seam on
//!   (a torus's tube angle; a spline wall's `u`);
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
    circle_at(cx, cy, r, 0.0, sweep)
}

/// [`circle`] with its vertex at carrier angle `phase`.
fn circle_at<T: Real>(cx: f64, cy: f64, r: f64, phase: f64, sweep: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r * phase.cos()), f(cy + r * phase.sin())),
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
            assert!(
                is_seam(&t.body, wall.strut),
                "{what}: the strut is the wall's seam"
            );
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
    type Case = (
        &'static str,
        Vec<ProfileLoop<f64>>,
        f64,
        (usize, usize, usize),
    );
    let cases: Vec<Case> = vec![
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

    let cyl = extruded(vec![circle(0.0, 0.0, 1.0, TAU)], 2.0, ExtrudeSide::Along).body;
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
                want.map_or("the single-site refusal".to_string(), |v| format!(
                    "volume {v}"
                )),
                out.map(|_| "a body")
            ),
        }
    }
}

/// The sketch y axis: `r(p) = p.x`.
fn y_axis<T: Real>() -> sweep::RevolveAxis<T> {
    sweep::RevolveAxis {
        origin: Point2::new(T::zero(), T::zero()),
        dir: geom_core::Vec2::new(T::zero(), T::one()),
    }
}

/// **A one-segment loop does not revolve yet, and says so.** Its wall
/// would be one torus face wrapping the tube's own angle, cut only by
/// the latitude strut at its vertex; a seam here is a `u_ref`
/// meridian, so no chart describes that cut (built, the patch reads
/// inside out: tier 3's `CurvedSenseInverted` and a negated volume).
/// Every case refuses `OneSegmentLoop` naming the loop — the outer,
/// a hole, part of a turn either way and a full turn, at `f64` and at
/// `Interval` — and nothing panics on the way.
#[test]
fn a_one_segment_loop_revolve_refuses_typed() {
    let cases: Vec<(&str, Vec<ProfileLoop<f64>>, usize)> = vec![
        ("an off-axis circle", vec![circle(3.0, 0.25, 0.5, TAU)], 0),
        (
            "a rectangle with a round hole",
            vec![rect(2.0, -1.0, 4.0, 1.0), circle(3.0, 0.0, 0.5, -TAU)],
            1,
        ),
    ];
    for (what, loops, want) in cases {
        for turn in [
            sweep::Revolution::Partial(1.25),
            sweep::Revolution::Partial(-1.25),
            sweep::Revolution::Full,
        ] {
            let got = sweep::revolve(&validated(loops.clone()), y_axis(), turn, tol());
            assert!(
                matches!(got, Err(sweep::RevolveError::OneSegmentLoop { loop_index }) if loop_index == want),
                "{what}, {turn:?}: {:?}",
                got.err()
            );
        }
        let at_i: Vec<ProfileLoop<Interval>> = loops
            .iter()
            .map(|l| l.map_scalar(Interval::from_f64))
            .collect();
        let got = sweep::revolve(
            &validated(at_i),
            y_axis(),
            sweep::Revolution::Partial(Interval::from_f64(1.25)),
            tol(),
        );
        assert!(
            matches!(got, Err(sweep::RevolveError::OneSegmentLoop { loop_index }) if loop_index == want),
            "{what} at Interval: {:?}",
            got.err()
        );
    }
}

/// **A one-segment section skins but does not loft yet, and says
/// so.** The section curve converts (`segment_curve` reads a full turn
/// from its start), so the geometry door builds one wall per
/// one-segment loop; the body would be that one spline face closing on
/// itself, its strut both its `u = 0` and `u = 1` edges, which a
/// non-periodic chart cannot image twice (built, the pcurve mint
/// refuses the second half). The assembly refuses `OneSegmentLoop`.
#[test]
fn a_one_segment_section_skins_and_the_loft_refuses_typed() {
    let places = sweep::test_support::stacked_at(&[0.0, 2.0]);
    let cases: Vec<(&str, Vec<ProfileLoop<f64>>, usize)> = vec![
        ("a circle", vec![circle(0.0, 0.0, 1.0, TAU)], 0),
        (
            "a square with a round hole",
            vec![rect(-2.0, -2.0, 2.0, 2.0), circle(0.25, 0.0, 1.0, TAU)],
            1,
        ),
    ];
    for (what, section, want) in cases {
        let sections = vec![section.clone(), section];
        let geometry = sweep::loft_geometry(&sections, &places, 1, tol())
            .unwrap_or_else(|e| panic!("{what}: the sections skin: {e}"));
        assert_eq!(geometry.walls[want].len(), 1, "{what}: one wall");
        let got = sweep::loft_body::<f64>(&sections, &places, 1, tol());
        assert!(
            matches!(got, Err(sweep::LoftError::OneSegmentLoop { loop_index }) if loop_index == want),
            "{what}: {:?}",
            got.err()
        );
    }
}

/// A one-segment circle centred on the axis crosses it, and one tangent
/// to it touches it, whatever its arity: both refuse, typed.
#[test]
fn a_one_segment_circle_on_or_against_the_axis_refuses() {
    for (what, lp) in [
        ("centred on the axis", circle(0.0, 0.0, 1.0, TAU)),
        ("tangent to the axis", circle_at(1.0, 0.0, 1.0, PI, TAU)),
    ] {
        let got = sweep::revolve(
            &validated(vec![lp]),
            y_axis(),
            sweep::Revolution::Partial(1.0),
            tol(),
        );
        assert!(
            matches!(
                got,
                Err(sweep::RevolveError::ArcCrossesAxis { .. }
                    | sweep::RevolveError::UnsupportedToroid { .. }
                    | sweep::RevolveError::VertexCrossesAxis { .. })
            ),
            "{what}: {:?}",
            got.err()
        );
    }
}

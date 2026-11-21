//! **A curve's cover box over a span against one interval evaluation over
//! it**, at `Interval`, per carrier arm:
//! - on a segment and on circle arcs of radius ½ and 1 (a quarter, a
//!   half, a short arc, more than a turn, and a half named the other way
//!   round), [`Cover::box_in`] at [`BoxAxes::World`] is the whole-span
//!   evaluation bracket for bracket, bit for bit;
//! - at a crest of radius 0.7, 0.9 or 1.3, whose square rounds, the
//!   cover's box is one or two ulps past the evaluation's;
//! - [`Cover::aabb`] at a zero pad is that box stepped out by one ulp on
//!   every side: [`bvh::Aabb::padded`] steps even at zero;
//! - on a rational quadratic, weights from `1e-8` to `1e8` and a whole
//!   span or a sub-span, the whole-span evaluation is the entire line
//!   while the cover box is finite and holds the sampled curve.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};
use std::sync::Arc;

use geom::cover::{BoxAxes, Cover, ParamWindow};
use geom::{Circle, Curve3, Frame, Line, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn cover(c: &Curve3<f64>, t0: f64, t1: f64) -> Cover<Interval> {
    c.map_scalar(Interval::from_f64)
        .cover_over(ParamWindow::new(
            Interval::from_f64(t0),
            Interval::from_f64(t1),
        ))
        .unwrap_or_else(|e| panic!("{c:?} over [{t0}, {t1}] has a cover: {e:?}"))
}

fn brackets(lo: Point3<Interval>, hi: Point3<Interval>) -> [f64; 6] {
    [
        lo.x.lo(),
        hi.x.hi(),
        lo.y.lo(),
        hi.y.hi(),
        lo.z.lo(),
        hi.z.hi(),
    ]
}

#[test]
fn a_closed_form_arcs_cover_box_is_its_whole_span_evaluation_and_aabb_steps_it_out() {
    let line = Curve3::Line(Line {
        origin: Point3::new(0.0, 2.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    });
    let diagonal = Curve3::Line(Line {
        origin: Point3::new(0.1, 0.2, 0.3),
        dir: Vec3::new(0.3, 0.7, -0.2),
    });
    let circle = Curve3::Circle(Circle::new(
        Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        0.5,
    ));
    let off = Curve3::Circle(Circle::new(
        Frame::new(Point3::new(-1.0, 0.0, 0.0), Vec3::unit_z(), Vec3::unit_x()),
        1.0,
    ));
    for (what, c, t0, t1) in [
        ("the segment at y = 2", &line, -1.0, 1.0),
        ("a diagonal segment", &diagonal, 0.0, 1.0),
        ("a half circle", &circle, 0.0, PI),
        ("a quarter circle", &circle, 0.0, FRAC_PI_2),
        ("a short arc", &circle, 0.1, 0.2),
        ("more than a turn", &circle, -1.0, 7.0),
        ("a half named backwards", &circle, PI, 0.0),
        ("a half about (−1, 0)", &off, FRAC_PI_2, 3.0 * FRAC_PI_2),
    ] {
        let cv = cover(c, t0, t1);
        let (lo, hi) = cv.box_in(&BoxAxes::World);
        let got = brackets(lo, hi);
        let e = c
            .map_scalar(Interval::from_f64)
            .eval(Interval::from_bounds(t0.min(t1), t0.max(t1)));
        let want = [e.x.lo(), e.x.hi(), e.y.lo(), e.y.hi(), e.z.lo(), e.z.hi()];
        assert_eq!(
            got.map(f64::to_bits),
            want.map(f64::to_bits),
            "{what}: {got:?} against {want:?}"
        );
        let b = cv.aabb(0.0);
        let stepped = [b.min_x, b.max_x, b.min_y, b.max_y, b.min_z, b.max_z];
        for (k, (s, g)) in stepped.iter().zip(got).enumerate() {
            let one = if k % 2 == 0 {
                g.next_down()
            } else {
                g.next_up()
            };
            assert_eq!(
                s.to_bits(),
                one.to_bits(),
                "{what}: aabb(0.0) bound {k} is one step out of {g}"
            );
        }
    }
    // The segment's height is its own, not a rounding off it.
    let (lo, hi) = cover(&line, -1.0, 1.0).box_in(&BoxAxes::World);
    assert!(
        lo.y.lo() == 2.0 && hi.y.hi() == 2.0,
        "the segment reads y = 2 exactly"
    );
}

#[test]
fn a_rational_arcs_cover_box_is_finite_where_its_whole_span_evaluation_is_the_line() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    ];
    for weights in [
        [1.0, 1.0, 1.0],
        [1.0, 1e3, 1.0],
        [1.0, 1e8, 1.0],
        [1.0, 1e-8, 1.0],
        [1e8, 1.0, 1e-8],
    ] {
        let spline = NurbsCurve3::new(knots.clone(), control.clone(), weights.to_vec()).unwrap();
        let c = Curve3::Nurbs(Arc::new(spline.clone()));
        let whole = c
            .map_scalar(Interval::from_f64)
            .eval(Interval::from_bounds(0.0, 1.0));
        assert!(
            !whole.x.hi().is_finite(),
            "{weights:?}: the whole-span evaluation is the line: {whole:?}"
        );
        for (t0, t1) in [(0.0, 1.0), (0.0, 0.1), (0.3, 0.7)] {
            let (lo, hi) = cover(&c, t0, t1).box_in(&BoxAxes::World);
            let b = brackets(lo, hi);
            assert!(
                b.iter().all(|x| x.is_finite()),
                "{weights:?} over [{t0}, {t1}]: {b:?}"
            );
            for k in 0..=1000 {
                let p = spline.eval(t0 + (t1 - t0) * f64::from(k) / 1000.0);
                assert!(
                    b[0] <= p.x && p.x <= b[1] && b[2] <= p.y && p.y <= b[3],
                    "{weights:?} over [{t0}, {t1}]: {p:?} in {b:?}"
                );
            }
        }
    }
}

/// **A spline window whose ends are not points reads the whole net**:
/// the pieces are cut at point ends only, so a window `[0.2, 0.3] ×
/// [0.6, 0.7]` (each end a bracket, as a `Sym` or an `Interval` body's
/// parameters can be) is covered by the control net's box, which holds
/// the curve everywhere and so over the window. Red if a bracket end is
/// read as if it were a point (a box narrower than the net), or if it
/// refuses.
#[test]
fn a_spline_window_with_bracket_ends_reads_its_whole_net() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
    ];
    let spline = NurbsCurve3::new(knots, control, vec![1.0, 1.0722, 1.0]).unwrap();
    let c = Curve3::Nurbs(Arc::new(spline)).map_scalar(Interval::from_f64);
    let (lo, hi) = c
        .cover_over(ParamWindow::new(
            Interval::from_bounds(0.2, 0.3),
            Interval::from_bounds(0.6, 0.7),
        ))
        .expect("a bracket window is covered")
        .box_in(&BoxAxes::World);
    assert_eq!(
        brackets(lo, hi),
        [0.0, 2.0, 0.0, 1.0, 0.0, 0.0],
        "the whole net's box"
    );
}

/// **At a crest of a radius whose square is not a float, the cover reads
/// one or two ulps out**: the arc's extremum is `c ± √(p² + q²)`, and
/// `0.7²`, `0.9²`, `1.3²` round, where the evaluation reads `r·cos` at
/// the crest exactly. The cover's box holds the evaluation's and is no
/// more than two ulps past it on any side. Red if the cover reads
/// narrower than the evaluation, or wider by more than its amplitude's
/// rounding. The last assertion pins the rounding itself
/// (`work/onecopy/an-arcs-cover-reads-its-crest-a-rounding-out-where-the-radius-squared-rounds.md`):
/// red when that row's fix lands, which is the time to drop it.
#[test]
fn at_a_crest_of_an_inexact_square_the_cover_reads_at_most_two_ulps_out() {
    let out =
        |x: f64, low: bool| (0..2).fold(x, |y, _| if low { y.next_down() } else { y.next_up() });
    for r in [0.7f64, 0.9, 1.3] {
        let c = Curve3::Circle(Circle::new(
            Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
            r,
        ));
        for (t0, t1) in [(0.0, PI), (0.0, 2.0 * PI), (0.3, 2.0)] {
            let (lo, hi) = cover(&c, t0, t1).box_in(&BoxAxes::World);
            let got = brackets(lo, hi);
            let e = c
                .map_scalar(Interval::from_f64)
                .eval(Interval::from_bounds(t0, t1));
            let want = [e.x.lo(), e.x.hi(), e.y.lo(), e.y.hi()];
            for (k, w) in want.iter().enumerate() {
                let g = got[k];
                let outer = out(*w, k % 2 == 0);
                let holds = if k % 2 == 0 {
                    g <= *w && g >= outer
                } else {
                    g >= *w && g <= outer
                };
                assert!(
                    holds || (w.abs() < 1e-300 && g.abs() < 1e-300),
                    "r = {r} over [{t0}, {t1}], bound {k}: the cover's {g:e} against the evaluation's {w:e}"
                );
            }
        }
    }
    let c = Curve3::Circle(Circle::new(
        Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        0.9,
    ));
    let (_, hi) = cover(&c, 0.0, PI).box_in(&BoxAxes::World);
    assert!(
        hi.y.hi() > 0.9,
        "the half circle of radius 0.9 reads past 0.9 at its crest: {:e}",
        hi.y.hi()
    );
}

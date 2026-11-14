//! Reviewer probes (PR 6963): how far a table's accepted recorded
//! direction may sit from the unit its vertices imply.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::float_cmp)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SegmentDirections, SketchPlane};

/// A square of side `l` whose first director is `d`.
fn square(l: f64, d: Vec2<f64>) -> ProfileLoop<f64> {
    let line = |x: f64, y: f64| Some(SegmentDirections::Line(Vec2::new(x, y)));
    RawLoop::new_directed([
        (Point2::new(0.0, 0.0), Segment::Line, Some(SegmentDirections::Line(d))),
        (Point2::new(l, 0.0), Segment::Line, line(0.0, 1.0)),
        (Point2::new(l, l), Segment::Line, line(-1.0, 0.0)),
        (Point2::new(0.0, l), Segment::Line, line(0.0, -1.0)),
    ])
}

fn accepts(l: f64, d: Vec2<f64>, tol: Tol) -> Option<Vec2<f64>> {
    Profile::new(SketchPlane::xy(), vec![square(l, d)])
        .validate(tol)
        .ok()
        .map(|v| match v.loops()[0].segments()[0].directions {
            SegmentDirections::Line(d) => d,
            SegmentDirections::Arc { .. } => panic!(),
        })
}

/// The largest t in [0, hi] with `ok(t)`, by bisection (monotone assumed).
fn largest(hi: f64, ok: impl Fn(f64) -> bool) -> f64 {
    let (mut lo, mut hi) = (0.0, hi);
    for _ in 0..80 {
        let m = 0.5 * (lo + hi);
        if ok(m) { lo = m } else { hi = m }
    }
    lo
}

#[test]
fn review_accepted_director_slack() {
    {
        let tol = Tol::witness();
        let eps = tol.eps();
        let shortest = {
            // scan down for the smallest side that validates
            let mut l = 1.0;
            while l > 1e-30 && accepts(l * 0.5, Vec2::new(1.0, 0.0), tol).is_some() { l *= 0.5 }
            l
        };
        for l in [shortest, 4.0 * shortest, 10.0 * eps, 100.0 * eps, 1.0] {
            if accepts(l, Vec2::new(1.0, 0.0), tol).is_none() { println!("eps {eps:e} l {l:e}: square refuses"); continue; }
            // angular: d = (cos a, sin a)
            let ang = largest(1.0, |a| accepts(l, Vec2::new(a.cos(), a.sin()), tol).is_some());
            // length: d = (1 + s, 0)
            let len = largest(1.0, |s| accepts(l, Vec2::new(1.0 + s, 0.0), tol).is_some());
            let got = accepts(l, Vec2::new(ang.cos(), ang.sin()), tol).unwrap();
            println!(
                "eps {eps:e} side {l:e} (= {:.2} eps): accepted angle off {ang:e} rad ({:.3} deg), accepted |d|-1 up to {len:e}; stored verbatim: {:?}",
                l / eps, ang.to_degrees(), got
            );
        }
    }
}

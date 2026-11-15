//! Reviewer differential: the `f64` directions validation mints for
//! segments that record none, printed as bits, to be run at two commits
//! and compared. Shapes the unit's own bit-identity row does not use:
//! staircases through the table door, authored both ways round, signed
//! zeros in the coordinates, and arcs whose ends sit at their centre's
//! axis points.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::print_stdout
)]

use geom_core::{Arc2, Point2, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SegmentDirections, SketchPlane};

fn dump(what: &str, lp: ProfileLoop<f64>) {
    match Profile::new(SketchPlane::xy(), vec![lp]).validate(Tol::witness()) {
        Err(e) => println!("DIFF {what}: refused {e:?}"),
        Ok(vp) => {
            for (k, s) in vp.loops()[0].segments().iter().enumerate() {
                let b = match s.directions {
                    SegmentDirections::Line(d) => vec![d.x.to_bits(), d.y.to_bits()],
                    SegmentDirections::Arc { start, end } => vec![
                        start.x.to_bits(),
                        start.y.to_bits(),
                        end.x.to_bits(),
                        end.y.to_bits(),
                    ],
                };
                println!("DIFF {what} seg {k}: {b:016x?}");
            }
        }
    }
}

fn lines(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    RawLoop::new_directed(
        pts.iter()
            .map(|&(x, y)| (Point2::new(x, y), Segment::Line, None)),
    )
}

#[test]
fn review_f64_minted_directions_dump() {
    let mut seed = 0x2545_f491_4f6c_dd1du64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    // Staircases: an L and a notch, at scales, both ways round.
    for i in 0..60 {
        let s = 10f64.powf(6.0 * next() - 3.0);
        let (x0, y0) = ((next() - 0.5) * 8.0 * s, (next() - 0.5) * 8.0 * s);
        let (a, b, c, d) = (
            (0.2 + next()) * s,
            (0.2 + next()) * s,
            (0.2 + next()) * s,
            (0.2 + next()) * s,
        );
        let l = [
            (x0, y0),
            (x0 + a + c, y0),
            (x0 + a + c, y0 + b),
            (x0 + a, y0 + b),
            (x0 + a, y0 + b + d),
            (x0, y0 + b + d),
        ];
        dump(&format!("L{i}"), lines(&l));
        let mut r = l;
        r.reverse();
        dump(&format!("L{i}r"), lines(&r));
        // A slanted quadrilateral: no axis at all.
        let q = [(x0, y0), (x0 + a, y0 + 0.1 * b), (x0 + a + 0.2 * c, y0 + b), (x0 - 0.1 * a, y0 + b + 0.3 * d)];
        dump(&format!("Q{i}"), lines(&q));
    }
    // Signed zeros in the coordinates of an axis-aligned square.
    for (i, (zx0, zx1, zy0, zy1)) in [
        (0.0, 0.0, 0.0, 0.0),
        (-0.0, 0.0, 0.0, 0.0),
        (0.0, -0.0, 0.0, 0.0),
        (0.0, 0.0, -0.0, 0.0),
        (0.0, 0.0, 0.0, -0.0),
        (-0.0, -0.0, -0.0, -0.0),
    ]
    .into_iter()
    .enumerate()
    {
        // Left side on x = ±0, bottom on y = ±0, each end its own zero.
        let sq = [(zx0, zy0), (1.0, zy1), (1.0, 1.0), (zx1, 1.0)];
        dump(&format!("Z{i}"), lines(&sq));
        let mut r = sq;
        r.reverse();
        dump(&format!("Z{i}r"), lines(&r));
        let sq = [(zx0 - 1.0, zy0 - 1.0), (zx1, zy1 - 1.0), (zx0, zy1), (-1.0, zy0)];
        dump(&format!("Y{i}"), lines(&sq));
    }
    // A D: a half turn about `c` between its poles, closed by a chord;
    // and a stadium. The radials sit on the centre's axes.
    for i in 0..40 {
        let s = 10f64.powf(4.0 * next() - 2.0);
        let (cx, cy, r) = ((next() - 0.5) * 4.0 * s, (next() - 0.5) * 4.0 * s, (0.2 + next()) * s);
        let arc = |sweep: f64, cx: f64, cy: f64| {
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, cy),
                radius: r,
                sweep,
            })
        };
        let pi = std::f64::consts::PI;
        let d = RawLoop::new_directed([
            (Point2::new(cx, cy - r), arc(pi, cx, cy), None),
            (Point2::new(cx, cy + r), Segment::Line, None),
        ]);
        dump(&format!("D{i}"), d);
        let len = (0.5 + next()) * s;
        let st = RawLoop::new_directed([
            (Point2::new(cx, cy - r), Segment::Line, None),
            (Point2::new(cx + len, cy - r), arc(pi, cx + len, cy), None),
            (Point2::new(cx + len, cy + r), Segment::Line, None),
            (Point2::new(cx, cy + r), arc(pi, cx, cy), None),
        ]);
        dump(&format!("S{i}"), st);
        // East-west poles.
        let e = RawLoop::new_directed([
            (Point2::new(cx + r, cy), arc(pi, cx, cy), None),
            (Point2::new(cx - r, cy), Segment::Line, None),
        ]);
        dump(&format!("E{i}"), e);
    }
    // A D about a signed-zero centre.
    for (i, (cx, px)) in [(0.0, 0.0), (-0.0, 0.0), (0.0, -0.0), (-0.0, -0.0)]
        .into_iter()
        .enumerate()
    {
        let arc = Segment::Arc(Arc2 {
            centre: Point2::new(cx, 0.0),
            radius: 1.0,
            sweep: std::f64::consts::PI,
        });
        let d = RawLoop::new_directed([
            (Point2::new(px, -1.0), arc, None),
            (Point2::new(px, 1.0), Segment::Line, None),
        ]);
        dump(&format!("DZ{i}"), d);
    }
}

//! Design probe (not for main): SketchSegment restriction, endpoints
//! re-derived per split (current) vs a range on the authored segment.
#![allow(clippy::all)]
use geom_brep::{SketchSegment, SweepRange};
use geom_core::{Arc2, Bounds, Interval, Point2, Real};
fn iv(x: f64) -> Interval { Interval::from_f64(x) }
fn w(e: Interval) -> f64 { e.hi() - e.lo() }
fn pw(p: Point2<Interval>) -> f64 { w(p.x).max(w(p.y)) }
fn ranged(seg: &SketchSegment<Interval>, r: SweepRange<Interval>, s: Interval) -> Point2<Interval> {
    match *seg {
        SketchSegment::Line { a, b } => a.lerp(b, r.at(s)),
        SketchSegment::Arc { a, arc, .. } => arc.point_from(a, r.at(s)),
    }
}
fn main() {
    for (cx, cy) in [(0.0, 0.0), (1000.0, -700.0), (1e5, 3e4)] {
        let c = Point2::new(iv(cx), iv(cy));
        let arc = SketchSegment::Arc { a: Point2::new(iv(cx + 1.0), iv(cy)), b: Point2::new(iv(cx - 1.0), iv(cy)),
            arc: Arc2 { centre: c, radius: iv(1.0), sweep: iv(core::f64::consts::PI) } };
        let line = SketchSegment::Line { a: Point2::new(iv(cx + 1.0), iv(cy)), b: Point2::new(iv(cx - 0.5), iv(cy + 2.0)) };
        for (kind, seg) in [("arc", arc), ("line", line)] {
            for chain in ["(0,1/2)", "(0.3,0.7)", "(a,1)"] {
                let (mut cur, mut r) = (seg, SweepRange::<Interval>::whole());
                let mut out = vec![];
                let measure = |cur: &SketchSegment<Interval>, r: SweepRange<Interval>| {
                    let mut m = (0.0f64, 0.0f64);
                    for s in [0.0, 0.5, 1.0] { m.0 = m.0.max(pw(cur.eval(iv(s)))); m.1 = m.1.max(pw(ranged(&seg, r, iv(s)))); }
                    m
                };
                out.push(measure(&cur, r));
                for k in 0..64 {
                    let a = 0.37 + 0.011 * (((k * 7) % 5) as f64);
                    let (s0, s1) = match chain { "(0,1/2)" => (iv(0.0), iv(0.5)), "(0.3,0.7)" => (iv(0.3), iv(0.7)), _ => (iv(a), iv(1.0)) };
                    cur = cur.restrict(s0, s1);
                    r = r.restrict(s0, s1);
                    if [0usize, 7, 63].contains(&k) { out.push(measure(&cur, r)); }
                }
                let f = |i: usize| format!("{:.2e}/{:.2e}", out[i].0, out[i].1);
                println!("c=({cx},{cy}) {kind:<4} {chain:<10} N=0 {}  N=1 {}  N=8 {}  N=64 {}   (current/range)", f(0), f(1), f(2), f(3));
            }
        }
    }
}

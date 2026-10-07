//! **A pair whose carriers are tangent reads no contact only where its
//! segments are apart**: a counterexample search over line × arc and
//! arc × arc pairs whose carriers lie within ε of tangency, each segment
//! reaching a few `√(2rε)` either side of the tangency point, against
//! an independent oracle — the closed-form distance between the two
//! segments, from their endpoints, their common normals and their
//! crossings. No pair the oracle puts within ε may read "no contact".
//!
//! In a file of its own so the per-file test gate can skip it without
//! skipping `seg`'s deterministic pair rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/seg.rs",
    "crates/geom-core/src/k_stats.rs",
    "crates/geom-core/src/tolerance.rs",
];

use core::f64::consts::{PI, TAU};

use geom_core::{Arc2, Band, Decide, Interval, Point2, Tol};
use test_utils::fuzz;

use crate::Segment;
use crate::seg::{Consistency, PairOutcome, build_seg, pair_contacts};

type P = (f64, f64);

fn sub(p: P, q: P) -> P {
    (p.0 - q.0, p.1 - q.1)
}

fn len(v: P) -> f64 {
    v.0.hypot(v.1)
}

fn dist(p: P, q: P) -> f64 {
    len(sub(p, q))
}

/// A segment as the oracle reads it: a chord, or an arc on a circle
/// from a start angle through a signed sweep.
#[derive(Clone, Copy, Debug)]
enum Shape {
    Line {
        a: P,
        b: P,
    },
    Arc {
        c: P,
        r: f64,
        start: f64,
        sweep: f64,
    },
}

impl Shape {
    fn ends(self) -> [P; 2] {
        match self {
            Self::Line { a, b } => [a, b],
            Self::Arc { c, r, start, sweep } => [at(c, r, start), at(c, r, start + sweep)],
        }
    }
}

fn at(c: P, r: f64, t: f64) -> P {
    (c.0 + r * t.cos(), c.1 + r * t.sin())
}

/// Whether the circle point at polar angle `t` lies on the arc.
fn on_arc(start: f64, sweep: f64, t: f64) -> bool {
    let along = (t - start) * sweep.signum();
    along.rem_euclid(TAU) <= sweep.abs()
}

/// The distance from `q` to the shape.
fn to_shape(q: P, s: Shape) -> f64 {
    match s {
        Shape::Line { a, b } => {
            let d = sub(b, a);
            let t = ((q.0 - a.0) * d.0 + (q.1 - a.1) * d.1) / (d.0 * d.0 + d.1 * d.1);
            let t = t.clamp(0.0, 1.0);
            dist(q, (a.0 + t * d.0, a.1 + t * d.1))
        }
        Shape::Arc { c, r, start, sweep } => {
            let v = sub(q, c);
            let [e0, e1] = s.ends();
            let ends = dist(q, e0).min(dist(q, e1));
            if len(v) > 0.0 && on_arc(start, sweep, v.1.atan2(v.0)) {
                (len(v) - r).abs().min(ends)
            } else {
                ends
            }
        }
    }
}

/// The closed-form distance between two shapes, one of them an arc: the
/// least of each endpoint's distance to the other shape, the common
/// normals both shapes hold, and zero where they cross.
fn oracle(s1: Shape, s2: Shape) -> f64 {
    let mut best = f64::INFINITY;
    for (p, q) in [(s1, s2), (s2, s1)] {
        for e in p.ends() {
            best = best.min(to_shape(e, q));
        }
    }
    match (s1, s2) {
        (Shape::Line { a, b }, Shape::Arc { c, r, start, sweep })
        | (Shape::Arc { c, r, start, sweep }, Shape::Line { a, b }) => {
            let l = dist(a, b);
            let u = (sub(b, a).0 / l, sub(b, a).1 / l);
            let n = (-u.1, u.0);
            let tc = (c.0 - a.0) * u.0 + (c.1 - a.1) * u.1;
            let h = (c.0 - a.0) * n.0 + (c.1 - a.1) * n.1;
            let on_line = |t: f64| (0.0..=l).contains(&t);
            // Common normals: the circle points facing along the line's
            // normal, with their foot on the line.
            for side in [-1.0, 1.0] {
                let t = (side * n.1).atan2(side * n.0);
                if on_arc(start, sweep, t) && on_line(tc) {
                    best = best.min((h + side * r).abs());
                }
            }
            if h.abs() < r {
                let half = (r * r - h * h).sqrt();
                for t in [tc - half, tc + half] {
                    let x = (a.0 + t * u.0, a.1 + t * u.1);
                    if on_line(t) && on_arc(start, sweep, (x.1 - c.1).atan2(x.0 - c.0)) {
                        best = 0.0;
                    }
                }
            }
        }
        (
            Shape::Arc {
                c: c1,
                r: r1,
                start: a1,
                sweep: w1,
            },
            Shape::Arc {
                c: c2,
                r: r2,
                start: a2,
                sweep: w2,
            },
        ) => {
            let d = dist(c1, c2);
            let u = (sub(c2, c1).0 / d, sub(c2, c1).1 / d);
            let t = u.1.atan2(u.0);
            // Common normals: both circles' points on the centre line.
            for t1 in [t, t + PI] {
                for t2 in [t, t + PI] {
                    if on_arc(a1, w1, t1) && on_arc(a2, w2, t2) {
                        best = best.min(dist(at(c1, r1, t1), at(c2, r2, t2)));
                    }
                }
            }
            let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
            if r1 * r1 > x * x {
                let y = (r1 * r1 - x * x).sqrt();
                for s in [-1.0, 1.0] {
                    let p = (c1.0 + x * u.0 - s * y * u.1, c1.1 + x * u.1 + s * y * u.0);
                    if on_arc(a1, w1, (p.1 - c1.1).atan2(p.0 - c1.0))
                        && on_arc(a2, w2, (p.1 - c2.1).atan2(p.0 - c2.0))
                    {
                        best = 0.0;
                    }
                }
            }
        }
        (Shape::Line { .. }, Shape::Line { .. }) => unreachable!("no line × line draw"),
    }
    best
}

/// What the pair pass read.
enum Read {
    /// This many contacts (an overlap counts as one).
    Contacts(usize),
    /// An escalation.
    Escalated,
    /// A segment the build refused, which no draw counts.
    Unbuilt,
}

fn kernel<T: Decide>(s1: Shape, s2: Shape, band: Band) -> Read {
    let build = |s: Shape| {
        let pt = |p: P| Point2::new(T::from_f64(p.0), T::from_f64(p.1));
        let [a, b] = s.ends();
        let segment = match s {
            Shape::Line { .. } => Segment::Line,
            Shape::Arc { c, r, sweep, .. } => Segment::Arc(Arc2 {
                centre: pt(c),
                radius: T::from_f64(r),
                sweep: T::from_f64(sweep),
            }),
        };
        build_seg(pt(a), pt(b), segment, Consistency::Decide, band).ok()
    };
    let (Some(seg1), Some(seg2)) = (build(s1), build(s2)) else {
        return Read::Unbuilt;
    };
    match pair_contacts(&seg1, &seg2, band) {
        Ok(PairOutcome::Contacts(contacts)) => Read::Contacts(contacts.len()),
        Ok(PairOutcome::Overlap) => Read::Contacts(1),
        Err(_) => Read::Escalated,
    }
}

/// One draw: a circle, and a line or a second circle tangent to it to
/// within ε (externally or internally), each segment reaching up to
/// four `√(2rε)` either side of the tangency point.
fn draw(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape) {
    let c = (rng.range(-2.0, 2.0), rng.range(-2.0, 2.0));
    let r = 10f64.powf(rng.range(-1.0, 0.5));
    let toward = rng.range(0.0, TAU);
    let gap = eps * rng.range(-0.95, 0.95);
    let tangency = at(c, r, toward);
    let reach = (2.0 * r * eps).sqrt().max(k * eps);
    // An arc on the first circle around the tangency point.
    let arc_near = |rng: &mut fuzz::Rng, c: P, r: f64, toward: f64| {
        let from = toward + rng.range(-4.0, 4.0) * reach / r;
        let sweep = if rng.unit() < 0.5 {
            rng.range(-4.0, 4.0) * reach / r
        } else {
            rng.range(-3.0, 3.0)
        };
        Shape::Arc {
            c,
            r,
            start: from,
            sweep,
        }
    };
    let s1 = arc_near(rng, c, r, toward);
    let s2 = match rng.below(3) {
        0 => {
            // A line tangent to the circle at `tangency`, `gap` outside.
            let n = (toward.cos(), toward.sin());
            let u = (-n.1, n.0);
            let foot = (tangency.0 + gap * n.0, tangency.1 + gap * n.1);
            let t0 = rng.range(-4.0, 4.0) * reach;
            let t1 = if rng.unit() < 0.5 {
                rng.range(-4.0, 4.0) * reach
            } else {
                t0 + rng.range(-3.0, 3.0)
            };
            Shape::Line {
                a: (foot.0 + t0 * u.0, foot.1 + t0 * u.1),
                b: (foot.0 + t1 * u.0, foot.1 + t1 * u.1),
            }
        }
        external => {
            let r2 = 10f64.powf(rng.range(-1.0, 0.5));
            let (r2, d) = if external == 1 {
                (r2, r + r2 + gap)
            } else {
                // The smaller circle inside the larger.
                let r2 = r2.min(0.9 * r);
                (r2, r - r2 + gap)
            };
            let c2 = (c.0 + d * toward.cos(), c.1 + d * toward.sin());
            let back = if external == 1 { toward + PI } else { toward };
            arc_near(rng, c2, r2, back)
        }
    };
    (s1, s2)
}

/// Sweeps `n` draws at scalar `T`; returns the counterexamples, and the
/// draws that escalated.
fn sweep<T: Decide>(rng: &mut fuzz::Rng, n: usize) -> (Vec<String>, usize) {
    let t = Tol::witness().get();
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let (mut wrong, mut escalated) = (Vec::new(), 0);
    for _ in 0..n {
        let (s1, s2) = draw(rng, t.eps, t.k);
        match kernel::<T>(s1, s2, band) {
            Read::Contacts(0) => {
                let d = oracle(s1, s2);
                if d <= t.eps {
                    wrong.push(format!(
                        "{}: {s1:?} × {s2:?}: no contact at distance {d:e}",
                        core::any::type_name::<T>()
                    ));
                }
            }
            Read::Contacts(_) | Read::Unbuilt => {}
            Read::Escalated => escalated += 1,
        }
    }
    (wrong, escalated)
}

#[test]
fn a_near_tangent_pair_reads_no_contact_only_past_eps() {
    let mut rng = fuzz::start("seg_reach_fuzz::a_near_tangent_pair_reads_no_contact_only_past_eps");
    let (mut wrong, f64_escalated) = sweep::<f64>(&mut rng, fuzz::scaled(40_000));
    let (wrong_i, interval_escalated) = sweep::<Interval>(&mut rng, fuzz::scaled(10_000));
    println!("escalated: {f64_escalated} at f64, {interval_escalated} at Interval");
    wrong.extend(wrong_i);
    assert!(
        wrong.is_empty(),
        "{} pairs within eps read no contact, e.g.\n{}\n{}",
        wrong.len(),
        wrong[..wrong.len().min(5)].join("\n"),
        fuzz::replay()
    );
}

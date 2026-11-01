//! **A pair whose carriers nearly touch reads no contact only where its
//! segments are apart**: a counterexample search against an independent
//! oracle, the closed-form distance between the two segments from their
//! endpoints, their common normals and their crossings. No pair the
//! oracle puts within ε may read "no contact". A third of the draws are
//! line × arc and arc × arc pairs whose carriers lie within ε of
//! tangency or cross at a shallow angle just past the band, each
//! segment reaching a few times the stretch along which the carriers
//! stay within ε or sweeping up to 10⁻³ short of a full turn. Another
//! third end a line or an arc at an edge of the band off another
//! segment's carrier, leaving it at an angle as small as 10⁻⁶, which is
//! where an in-band end reading has to escalate rather than read clear.
//! The last third end two segments at an edge of the band either side
//! of their carriers' crossing, at angles down to 10⁻⁶, where a span
//! reads the crossing in band and the ends have to settle it. A second
//! search ([`rivals`]) puts the carriers' two crossings close together,
//! so an end beside one stands within ε of both carriers while the
//! segments meet at the other. Draws whose coordinates are too large
//! for the oracle's `f64` to resolve ε are counted and skipped. A third
//! ([`joint`]) judges two adjacent arcs at a joint on circles within ε
//! of internal tangency the way validation does, shared-vertex discount
//! included, against the arcs' crossings: none far from the joint may
//! read clear.
//!
//! In a file of its own so the per-file test gate can skip it without
//! skipping `seg`'s deterministic pair rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/profile/src/seg.rs",
    "crates/profile/src/validate.rs",
    "crates/geom-core/src/k_stats.rs",
    "crates/geom-core/src/tolerance.rs",
];

use core::f64::consts::{PI, TAU};

use geom_core::{Arc2, Band, Decide, Interval, Point2, Tol};
use test_utils::fuzz;

use crate::Segment;
use crate::seg::{Consistency, PairOutcome, SegKind, build_seg, pair_contacts};

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
                let half = ((r - h.abs()) * (r + h.abs())).sqrt();
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
            for p in crossings((c1, r1), (c2, r2)) {
                if on_arc(a1, w1, (p.1 - c1.1).atan2(p.0 - c1.0))
                    && on_arc(a2, w2, (p.1 - c2.1).atan2(p.0 - c2.0))
                {
                    best = 0.0;
                }
            }
        }
        // Two chords: the four endpoint distances, and a crossing, which
        // needs every side read past its rounding. An end within that of
        // the other carrier is as near the other chord as its distance
        // says, so the endpoint distances cover the rest.
        (Shape::Line { a, b }, Shape::Line { a: c, b: d }) => {
            let side = |p: P, q: P, x: P| {
                let s = (q.0 - p.0) * (x.1 - p.1) - (q.1 - p.1) * (x.0 - p.0);
                let rounding = 8.0 * f64::EPSILON * dist(p, q) * (dist(p, x) + len(p));
                if s.abs() > rounding { s.signum() } else { 0.0 }
            };
            if side(a, b, c) * side(a, b, d) < 0.0 && side(c, d, a) * side(c, d, b) < 0.0 {
                best = 0.0;
            }
        }
    }
    best
}

/// Two circles' crossings, from the smaller circle's centre, in
/// factored form: the radical line's offset `x` from that centre and
/// the half-chord `y` each lose to cancellation as a difference of
/// squares when the radii differ by orders of magnitude.
fn crossings((c1, r1): (P, f64), (c2, r2): (P, f64)) -> Vec<P> {
    let d = dist(c1, c2);
    let u = (sub(c2, c1).0 / d, sub(c2, c1).1 / d);
    let (small, rs, rl, towards) = if r1 <= r2 {
        (c1, r1, r2, u)
    } else {
        (c2, r2, r1, (-u.0, -u.1))
    };
    let (lens, inner) = (rs + rl - d, d + rs - rl);
    if !(lens > 0.0 && inner > 0.0) {
        return Vec::new();
    }
    let x = rs - lens * (rl + d - rs) / (2.0 * d);
    let y = (lens * inner * (d + rl - rs) * (d + rs + rl)).sqrt() / (2.0 * d);
    [-1.0, 1.0]
        .into_iter()
        .map(|s| {
            (
                small.0 + x * towards.0 - s * y * towards.1,
                small.1 + x * towards.1 + s * y * towards.0,
            )
        })
        .collect()
}

/// What the pair pass read.
enum Read {
    /// This many contacts (an overlap counts as one), between the two
    /// shapes as the build classified them: an arc whose sagitta is
    /// within ε is its chord.
    Contacts(usize, Shape, Shape),
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
        let seg = build_seg(pt(a), pt(b), segment, Consistency::Decide, band).ok()?;
        let read = match seg.kind {
            SegKind::Line => Shape::Line { a, b },
            SegKind::Arc(_) => s,
        };
        Some((seg, read))
    };
    let (Some((seg1, read1)), Some((seg2, read2))) = (build(s1), build(s2)) else {
        return Read::Unbuilt;
    };
    match pair_contacts(&seg1, &seg2, None, band) {
        Ok(PairOutcome::Contacts(contacts)) => Read::Contacts(contacts.len(), read1, read2),
        Ok(PairOutcome::Overlap) => Read::Contacts(1, read1, read2),
        Err(_) => Read::Escalated,
    }
}

/// One draw: a near-tangent or shallow-secant pair ([`near`]), a
/// segment ending at the edge of the band off another ([`band_edge`]),
/// or two segments ending at the edge of the band either side of their
/// carriers' crossing ([`crossing`]).
fn draw(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape) {
    match rng.below(3) {
        0 => near(rng, eps, k),
        1 => band_edge(rng, eps, k),
        _ => crossing(rng, eps, k),
    }
}

/// Two carriers, lines or arcs, crossing at a point at an angle from
/// 10⁻⁶ to one radian, each segment ending at an edge of the band
/// ([`edge`]) short of the crossing or past it along its own carrier
/// and running on from there either way, up to 10⁻³ short of a full
/// turn. A span reads such a crossing in band, and its stretch,
/// ε/sin φ, runs from inside the band to past the segment.
fn crossing(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape) {
    let x = (rng.range(-2.0, 2.0), rng.range(-2.0, 2.0));
    let psi = rng.range(0.0, TAU);
    let phi = 10f64.powf(rng.range(-6.0, 0.0)) * if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let mut through = |heading: f64| {
        let u = (heading.cos(), heading.sin());
        let along = edge(rng, eps, k);
        let back = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        if rng.unit() < 0.5 {
            let reach = back * 10f64.powf(rng.range(-2.0, 0.5));
            let at = |s: f64| (x.0 + s * u.0, x.1 + s * u.1);
            Shape::Line {
                a: at(along),
                b: at(along + reach),
            }
        } else {
            // Turning left (centre on the left, counter-clockwise) or
            // right; arc length `s` from the crossing turns it s/r.
            let r = 10f64.powf(rng.range(-1.0, 0.5));
            let turn = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
            let c = (x.0 - turn * r * u.1, x.1 + turn * r * u.0);
            let at_x = (x.1 - c.1).atan2(x.0 - c.0);
            Shape::Arc {
                c,
                r,
                start: at_x + turn * along / r,
                sweep: turn * back * sweep_of(rng, 1e-3).abs(),
            }
        }
    };
    (through(psi), through(psi + phi))
}

/// A signed sweep: a few times `small` (an arc local to one point), up
/// to three radians, or past a half turn up to 10⁻³ short of a full one.
fn sweep_of(rng: &mut fuzz::Rng, small: f64) -> f64 {
    let sign = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    sign * match rng.below(3) {
        0 => rng.range(0.0, 4.0) * small,
        1 => rng.range(0.0, 3.0),
        _ => TAU - 10f64.powf(rng.range(-3.0, 0.5)),
    }
}

/// A circle, and a line or a second circle tangent to it to within ε,
/// or cutting it by 1.5 to 6 Kε (externally, internally, or nearly
/// concentric inside it, the centres from Kε to 10⁻²·r apart), each
/// segment reaching up to four times the near stretch either side of
/// the closest point, or sweeping up to 10⁻³ short of a full turn.
fn near(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape) {
    let c = (rng.range(-2.0, 2.0), rng.range(-2.0, 2.0));
    let r = 10f64.powf(rng.range(-1.0, 0.5));
    let toward = rng.range(0.0, TAU);
    // Tangent within ε, or secant past the band at a shallow angle.
    let gap = if rng.unit() < 0.5 {
        eps * rng.range(-0.95, 0.95)
    } else {
        -k * eps * rng.range(1.5, 6.0)
    };
    let tangency = at(c, r, toward);
    let family = rng.below(4);
    // A nearly concentric second circle, its centre as little as Kε
    // off: the carriers stay within ε for an angle of about
    // √(2(ε − gap)/off) either side of the tangency, up to a radian.
    let off = (family == 3).then(|| {
        if rng.unit() < 0.5 {
            r * 10f64.powf(rng.range(-7.0, -2.0))
        } else {
            (-gap).max(0.0) + k * eps * 10f64.powf(rng.range(0.0, 3.0))
        }
    });
    let reach = match off {
        Some(off) => r * (2.0 * (eps - gap) / off).sqrt().min(1.0),
        None => (2.0 * r * (eps - gap)).sqrt(),
    }
    .max(k * eps);
    // An arc on the first circle around the tangency point.
    let arc_near = |rng: &mut fuzz::Rng, c: P, r: f64, toward: f64| Shape::Arc {
        c,
        r,
        start: toward + rng.range(-4.0, 4.0) * reach / r,
        sweep: sweep_of(rng, reach / r),
    };
    let s1 = arc_near(rng, c, r, toward);
    let s2 = match family {
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
        family => {
            let r2 = 10f64.powf(rng.range(-1.0, 0.5));
            let (r2, d) = match family {
                1 => (r2, r + r2 + gap),
                // The smaller circle inside the larger.
                2 => {
                    let r2 = r2.min(0.9 * r);
                    (r2, r - r2 + gap)
                }
                // Inside it, nearly concentric.
                _ => {
                    let off = off.expect("the concentric family draws an offset");
                    (r - off, off + gap)
                }
            };
            let c2 = (c.0 + d * toward.cos(), c.1 + d * toward.sin());
            let back = if family == 1 { toward + PI } else { toward };
            arc_near(rng, c2, r2, back)
        }
    };
    (s1, s2)
}

/// An offset at the edge of the band: ±{0.5, 0.999, 1, 1.001, 2}·ε or
/// ±{0.999, 1, 1.001, 2}·Kε.
fn edge(rng: &mut fuzz::Rng, eps: f64, k: f64) -> f64 {
    let at = [
        0.5,
        0.999,
        1.0,
        1.001,
        2.0,
        k * 0.999,
        k,
        k * 1.001,
        k * 2.0,
    ];
    let sign = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    sign * eps * at[rng.below(at.len())]
}

/// A line or an arc (up to 10⁻³ short of a full turn), and a line or an
/// arc that ends at an edge of the band off its carrier ([`edge`]),
/// over a foot that lies at an edge of the band off its end or inside
/// its span, and leaves that end at an angle as small as 10⁻⁶ to the
/// carrier, toward it or away.
fn band_edge(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape) {
    let o = (rng.range(-2.0, 2.0), rng.range(-2.0, 2.0));
    let length = 10f64.powf(rng.range(-1.0, 0.5));
    let psi = rng.range(0.0, TAU);
    // How far along the host from its start end the foot lies, in
    // meters: an edge of the band either side of that end, or inside.
    let at_edge = rng.unit() < 0.5;
    let (edge_along, inside) = (edge(rng, eps, k), rng.unit());
    // The host, the foot on its carrier, the carrier's unit normal and
    // unit tangent there.
    let (host, foot, normal, tangent) = if rng.unit() < 0.5 {
        let u = (psi.cos(), psi.sin());
        let t = if at_edge { edge_along } else { inside * length };
        let b = (o.0 + length * u.0, o.1 + length * u.1);
        let foot = (o.0 + t * u.0, o.1 + t * u.1);
        (Shape::Line { a: o, b }, foot, (-u.1, u.0), u)
    } else {
        let sweep = sweep_of(rng, 1.0);
        let turn = sweep.signum();
        let t = if at_edge {
            edge_along
        } else {
            inside * sweep.abs() * length
        };
        let angle = psi + turn * t / length;
        let radial = (angle.cos(), angle.sin());
        let host = Shape::Arc {
            c: o,
            r: length,
            start: psi,
            sweep,
        };
        let tangent = (-turn * radial.1, turn * radial.0);
        (host, at(o, length, angle), radial, tangent)
    };
    let off = edge(rng, eps, k);
    let end = (foot.0 + off * normal.0, foot.1 + off * normal.1);
    // The other segment leaves `end` at `phi` to the tangent, either way
    // along it, toward the carrier or away.
    let phi = 10f64.powf(rng.range(-6.0, 0.0)) * if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let back = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let (sin, cos) = phi.sin_cos();
    let dir = (
        back * (tangent.0 * cos - tangent.1 * sin),
        back * (tangent.0 * sin + tangent.1 * cos),
    );
    let reach = 10f64.powf(rng.range(-2.0, 0.5));
    let other = if rng.unit() < 0.5 {
        Shape::Line {
            a: end,
            b: (end.0 + reach * dir.0, end.1 + reach * dir.1),
        }
    } else {
        // Turning left (centre on the left, counter-clockwise) or right.
        let turn = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let c = (end.0 - turn * reach * dir.1, end.1 + turn * reach * dir.0);
        Shape::Arc {
            c,
            r: reach,
            start: (end.1 - c.1).atan2(end.0 - c.0),
            sweep: turn * sweep_of(rng, 1e-3).abs(),
        }
    };
    if rng.unit() < 0.5 {
        (host, other)
    } else {
        (other, host)
    }
}

/// The most the oracle's `f64` arithmetic can be off by on a pair: a
/// few ulps of the largest coordinate or radius it reads.
fn oracle_slack(s1: Shape, s2: Shape) -> f64 {
    let scale = [s1, s2]
        .into_iter()
        .map(|s| match s {
            Shape::Line { a, b } => a.0.abs().max(a.1.abs()).max(b.0.abs()).max(b.1.abs()),
            Shape::Arc { c, r, .. } => c.0.abs().max(c.1.abs()) + r,
        })
        .fold(0.0, f64::max);
    4.0 * f64::EPSILON * scale
}

/// Sweeps `n` draws from `draw` at scalar `T`; returns the
/// counterexamples, the draws that escalated, and the draws the oracle
/// could not resolve at ε (its slack past a quarter of ε), which no
/// draw counts.
fn sweep<T: Decide>(
    scalar: &str,
    rng: &mut fuzz::Rng,
    n: usize,
    draw: fn(&mut fuzz::Rng, f64, f64) -> (Shape, Shape),
) -> (Vec<String>, usize, usize) {
    let t = Tol::witness().get();
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let (mut wrong, mut escalated, mut unresolved) = (Vec::new(), 0, 0);
    for _ in 0..n {
        let (s1, s2) = draw(rng, t.eps, t.k);
        let slack = oracle_slack(s1, s2);
        if slack > 0.25 * t.eps {
            unresolved += 1;
            continue;
        }
        match kernel::<T>(s1, s2, band) {
            Read::Contacts(0, s1, s2) => {
                let d = oracle(s1, s2);
                if d + slack <= t.eps {
                    wrong.push(format!(
                        "{scalar}: {s1:?} × {s2:?}: no contact at distance {d:e}"
                    ));
                }
            }
            Read::Contacts(..) | Read::Unbuilt => {}
            Read::Escalated => escalated += 1,
        }
    }
    (wrong, escalated, unresolved)
}

/// Runs `draw` at `f64` and at `Interval` and asserts no pair the
/// oracle puts within ε read no contact.
fn search(label: &str, draw: fn(&mut fuzz::Rng, f64, f64) -> (Shape, Shape)) {
    let mut rng = fuzz::start(label);
    let (mut wrong, f64_escalated, f64_unresolved) =
        sweep::<f64>("f64", &mut rng, fuzz::scaled(40_000), draw);
    let (wrong_i, interval_escalated, interval_unresolved) =
        sweep::<Interval>("Interval", &mut rng, fuzz::scaled(10_000), draw);
    println!(
        "escalated: {f64_escalated} at f64, {interval_escalated} at Interval; past the \
         oracle's resolution: {f64_unresolved} at f64, {interval_unresolved} at Interval"
    );
    wrong.extend(wrong_i);
    assert!(
        wrong.is_empty(),
        "{} pairs within eps read no contact, e.g.\n{}\n{}",
        wrong.len(),
        wrong[..wrong.len().min(5)].join("\n"),
        fuzz::replay()
    );
}

#[test]
fn a_near_tangent_pair_reads_no_contact_only_past_eps() {
    search(
        "seg_reach_fuzz::a_near_tangent_pair_reads_no_contact_only_past_eps",
        draw,
    );
}

/// **A crossing near its rival reads no contact only where the
/// segments are apart.** Draws from [`rivals`]: the carriers' two
/// crossings as little as ε apart, so an end beside one stands within
/// ε of both carriers while the segments meet at the other.
#[test]
fn a_crossing_near_its_rival_reads_no_contact_only_past_eps() {
    search(
        "seg_reach_fuzz::a_crossing_near_its_rival_reads_no_contact_only_past_eps",
        rivals,
    );
}

/// A circle, and a line or a second circle, through two crossings `X`
/// and `Y`, from 0.5ε to 10³ε apart or anywhere along the circle, at
/// up to 300 from the origin, radii from 0.5Kε to 10⁴ (two circles'
/// within 10¹² of each other), and crossing angles up to a quarter
/// turn. Each segment's ends are drawn beside
/// `X`, beside `Y` (at an edge of the band, between the two, or up to
/// a tenth of the radius off), or anywhere, and an arc runs either way
/// between them, so it sweeps up to 10⁻³ short of a full turn. That
/// puts a segment's end beside one crossing while the pair meets at
/// the other, which an in-band span reading, compressed past 5.6 rad
/// or on a radius of a few Kε, can reach.
fn rivals(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape) {
    let signed = |rng: &mut fuzz::Rng, lo: f64, hi: f64| {
        10f64.powf(rng.range(lo, hi)) * if rng.unit() < 0.5 { -1.0 } else { 1.0 }
    };
    let x = (signed(rng, -1.0, 2.5), signed(rng, -1.0, 2.5));
    let radius = |rng: &mut fuzz::Rng, lo: f64, hi: f64| {
        10f64.powf(rng.range(lo.max(0.5 * k * eps).log10(), hi.min(1e4).log10()))
    };
    let r2 = radius(rng, 0.0, 1e4);
    let apart = if rng.unit() < 0.5 {
        (eps * 10f64.powf(rng.range(-0.3, 3.0))).min(1.998 * r2)
    } else {
        2.0 * r2 * rng.range(1e-3, 0.999)
    };
    let psi = rng.range(0.0, TAU);
    let u = (psi.cos(), psi.sin());
    let y = (x.0 + apart * u.0, x.1 + apart * u.1);
    // The circle of radius `r` through X and Y, its centre on the
    // chord's left or right.
    let circle = |rng: &mut fuzz::Rng, r: f64| {
        let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let off = side * (r * r - 0.25 * apart * apart).max(0.0).sqrt();
        (
            (0.5 * (x.0 + y.0) - off * u.1, 0.5 * (x.1 + y.1) + off * u.0),
            r,
        )
    };
    // An offset from an anchor along the carrier: an edge of the band,
    // a fraction of the gap between the crossings, or up to a tenth of
    // the radius.
    let offset = |rng: &mut fuzz::Rng, r: f64| match rng.below(3) {
        0 => edge(rng, eps, k),
        1 => apart * rng.range(-1.0, 1.0),
        _ => signed(rng, -4.0, -1.0) * r,
    };
    let on_circle = |rng: &mut fuzz::Rng, (c, r): (P, f64)| {
        let angle = |p: P| (p.1 - c.1).atan2(p.0 - c.0);
        let anchor = |rng: &mut fuzz::Rng| match rng.below(3) {
            0 => angle(x) + offset(rng, r) / r,
            1 => angle(y) + offset(rng, r) / r,
            _ => rng.range(0.0, TAU),
        };
        let (from, to) = (anchor(rng), anchor(rng));
        let turn = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let sweep = ((to - from) * turn).rem_euclid(TAU).clamp(1e-9, TAU - 1e-3);
        Shape::Arc {
            c,
            r,
            start: from,
            sweep: turn * sweep,
        }
    };
    let carrier2 = circle(rng, r2);
    let s2 = on_circle(rng, carrier2);
    let s1 = if rng.unit() < 0.5 {
        let r1 = radius(rng, (0.5 * apart * 1.0001).max(r2 * 1e-12), r2 * 1e12);
        let carrier1 = circle(rng, r1);
        on_circle(rng, carrier1)
    } else {
        let along = |rng: &mut fuzz::Rng| match rng.below(3) {
            0 => offset(rng, r2),
            1 => apart + offset(rng, r2),
            _ => signed(rng, -2.0, 2.5),
        };
        let (t0, t1) = (along(rng), along(rng));
        Shape::Line {
            a: (x.0 + t0 * u.0, x.1 + t0 * u.1),
            b: (x.0 + t1 * u.0, x.1 + t1 * u.1),
        }
    };
    if rng.unit() < 0.5 { (s1, s2) } else { (s2, s1) }
}

/// What validation's pair judgement read of a joint's two arcs.
enum Judged {
    /// No contact but the shared vertex's.
    Clear,
    /// A contact it refused, or an escalation.
    Refused,
    /// A segment the build refused, which no draw counts.
    Unbuilt,
}

/// Judges the arcs `s1` into the joint `v` and `s2` out of it as one
/// loop's adjacent pair (`validate::judge_pair`, the shared-vertex
/// discount included), the loop closed by a line.
fn judge_joint<T: Decide>(s1: Shape, s2: Shape, v: P, band: Band) -> Judged {
    use crate::{ProfileLoop, RawLoop};
    let pt = |p: P| Point2::new(T::from_f64(p.0), T::from_f64(p.1));
    let arc = |s: Shape| match s {
        Shape::Arc { c, r, sweep, .. } => Segment::Arc(Arc2 {
            centre: pt(c),
            radius: T::from_f64(r),
            sweep: T::from_f64(sweep),
        }),
        Shape::Line { .. } => Segment::Line,
    };
    let lp = ProfileLoop::new([
        (pt(s1.ends()[0]), arc(s1)),
        (pt(v), arc(s2)),
        (pt(s2.ends()[1]), Segment::Line),
    ]);
    let Ok(segs) = crate::validate::build_loop_segs(&lp, 0, Consistency::Decide, band) else {
        return Judged::Unbuilt;
    };
    match crate::validate::judge_pair(&[lp], &[segs], (0, 0), (0, 1), band) {
        Ok(()) => Judged::Clear,
        Err(_) => Judged::Refused,
    }
}

/// A joint on circles within the band of internal tangency: the second
/// circle's centre from Kε to a third of the radius off the first's,
/// its internal clearance within 1.2ε, and the joint on the first
/// circle at the tangency direction, at an edge of the band along it,
/// or anywhere along the stretch where the circles stay within ε.
/// The first arc arrives at the joint and the second leaves it either
/// way, at a cusp or a smooth joint, each reaching a few times that
/// stretch or up to 10⁻³ short of a full turn. Returns the two arcs and
/// the joint.
fn joint(rng: &mut fuzz::Rng, eps: f64, k: f64) -> (Shape, Shape, P) {
    let sign = |rng: &mut fuzz::Rng| if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let c = (rng.range(-2.0, 2.0), rng.range(-2.0, 2.0));
    let r = 10f64.powf(rng.range(-1.0, 0.5));
    let toward = rng.range(0.0, TAU);
    let d = if rng.unit() < 0.5 {
        r * 10f64.powf(rng.range(-7.0, -0.5))
    } else {
        k * eps * 10f64.powf(rng.range(0.0, 3.0))
    };
    let g = if rng.unit() < 0.5 {
        eps * rng.range(-1.2, 1.2)
    } else {
        eps * [0.0, 0.5, 0.999, 1.0][rng.below(4)] * sign(rng)
    };
    let (c2, r2) = ((c.0 + d * toward.cos(), c.1 + d * toward.sin()), r - d + g);
    // The circles stand |g − d(1 − cos θ)| apart at θ off the tangency
    // direction, within ε out to this angle.
    let stretch = (2.0 * (eps + g).max(0.0) / d).sqrt().min(PI);
    let off = match rng.below(4) {
        0 => 0.0,
        1 => edge(rng, eps, k) / r,
        2 => sign(rng) * stretch * rng.unit(),
        _ => sign(rng) * stretch * (1.0 + rng.range(-1e-3, 1e-3)),
    };
    let v = at(c, r, toward + off);
    let sweep = |rng: &mut fuzz::Rng| sweep_of(rng, stretch.max(k * eps / r));
    let w1 = sweep(rng);
    let s1 = Shape::Arc {
        c,
        r,
        start: toward + off - w1,
        sweep: w1,
    };
    let s2 = Shape::Arc {
        c: c2,
        r: r2,
        start: (v.1 - c2.1).atan2(v.0 - c2.0),
        sweep: sweep(rng),
    };
    (s1, s2, v)
}

/// **A joint's arcs read clear only where they cross, if at all,
/// within a tangency's reach of the joint**: a counterexample search
/// over [`joint`] at `f64` and at `Interval`. Carriers within ε of
/// internal tangency read as a tangency only where their chord's
/// sagitta is within ε, whose ends stand within √(2r·ε) of its foot,
/// and a cusp there is a tangent cusp. A sagitta within the band reads
/// clear only where the crossings and the tangency agree, which a cusp
/// holding a crossing does not. So a crossing of the two arcs that
/// stands farther from the joint than the joint does from the tangency
/// direction, plus 1.5·√(2r·ε) (that half-chord, with room for the two
/// circles' sagittas differing by g), is one the shared-vertex discount
/// must not absorb. One nearer the tangency is a tangent cusp's, which
/// `a-cusp-on-nested-near-concentric-carriers-validates-along-a-long-eps-stretch`
/// holds.
#[test]
fn a_joint_reads_clear_only_where_its_arcs_cross_near_it() {
    let t = Tol::witness().get();
    let band = Band::linear(Tol::witness()).expect("the run's band");
    let mut rng =
        fuzz::start("seg_reach_fuzz::a_joint_reads_clear_only_where_its_arcs_cross_near_it");
    let mut wrong = Vec::new();
    let (mut crossing, mut refused, mut unbuilt) = (0, 0, 0);
    for (scalar, n) in [
        ("f64", fuzz::scaled(20_000)),
        ("Interval", fuzz::scaled(5_000)),
    ] {
        for _ in 0..n {
            let (s1, s2, v) = joint(&mut rng, t.eps, t.k);
            let (
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
            ) = (s1, s2)
            else {
                unreachable!("a joint draws two arcs")
            };
            let tangency = at(c1, r1, (c2.1 - c1.1).atan2(c2.0 - c1.0));
            let reach = dist(v, tangency) + 1.5 * (2.0 * r1.max(r2) * t.eps).sqrt();
            let far = crossings((c1, r1), (c2, r2)).into_iter().find(|&p| {
                on_arc(a1, w1, (p.1 - c1.1).atan2(p.0 - c1.0))
                    && on_arc(a2, w2, (p.1 - c2.1).atan2(p.0 - c2.0))
                    && dist(p, v) > reach
            });
            let judged = if scalar == "f64" {
                judge_joint::<f64>(s1, s2, v, band)
            } else {
                judge_joint::<Interval>(s1, s2, v, band)
            };
            match judged {
                Judged::Unbuilt => unbuilt += 1,
                Judged::Refused => refused += 1,
                Judged::Clear => {}
            }
            if let (Some(p), Judged::Clear) = (far, &judged) {
                wrong.push(format!(
                    "{scalar}: {s1:?} × {s2:?} at {v:?}: clear, crossing at {p:?}, {:e} from the joint",
                    dist(p, v)
                ));
            }
            if far.is_some() && !matches!(judged, Judged::Unbuilt) {
                crossing += 1;
            }
        }
    }
    println!(
        "{crossing} joints crossing away from the joint; {refused} refused; {unbuilt} unbuilt"
    );
    assert!(
        wrong.is_empty(),
        "{} joints crossing away from the joint read clear, e.g.\n{}\n{}",
        wrong.len(),
        wrong[..wrong.len().min(5)].join("\n"),
        fuzz::replay()
    );
}

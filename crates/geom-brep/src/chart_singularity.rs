//! **Where a chart singularity lies against a face**: a sphere's pole or
//! a cone's apex read against the face's region, at one of its
//! vertices, inside one of its edges, in its interior, or outside its
//! closure ([`locate`]).
//!
//! Every chart singularity in a face's closure is a vertex of it (DESIGN
//! `[topology.tiers]`, tier 3). Two doors read that statement through
//! this module and no other reading: `step-import`'s normalization,
//! which makes each such point a vertex of the face it lies in, and
//! tier 3, which refuses a face where one is not. The reading is
//! form-neutral: it never asks whether a loop has a meridian, since a
//! tilted cap holds a pole with none and a meridian spur that stops
//! short of the pole has one and still holds it.
//!
//! **The singular points** are [`singular_points`]: a sphere's two poles
//! and a cone's apex, from the chart's own frame. Every other consumer
//! of a chart's singular set reads this list.
//!
//! **On the boundary.** A point is at a vertex of the face where its
//! distance to the vertex decides zero, and inside an edge where its
//! distance to the edge's nearest interior point does, both at the
//! caller's band ([`on_boundary`], `chart_singularity_on_boundary`); a
//! distance the band cannot call is undecided and carries its margin
//! ([`Unread::Boundary`]). Lines and circles are read in closed form;
//! any other carrier by samples, which can certify the point off it but
//! never on it ([`Unread::Carrier`]).
//!
//! **Inside the face.** Off the boundary, the point is in the face where
//! it lies on the face's side of every loop. A loop's side holding the
//! point is read off its walk through the chart ([`walk_loop`]): its
//! winding about the surface axis, and the signed area of its chart
//! image. A loop that winds separates the sphere's poles, the north on
//! its left when it winds positively, and rings the cone's apex, which
//! it holds on its left exactly when it winds positively on the
//! negative nappe; one that does not wind leaves the singular points on
//! the side its chart image does not enclose, which is its left exactly
//! when that image runs clockwise (`chart_singularity_loop_side`). The
//! face's side of each loop is its left under the outward normal, the
//! chart normal turned by the face's sense.

use geom::{ChartArm, ChartInverse, Curve3, Surface};
use geom_core::k_stats::{NonzeroSign, decide, decide_nonzero};
use geom_core::predicate::Band;
use geom_core::{Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, SizedPass};

use crate::props::LoopEdge;
use crate::recourse::{RefusedSide, SizedDecision};

/// **Whether a chart singularity lies on a face's boundary**, a sized
/// decision: the distance from the point to a vertex or an edge of the
/// face. Zero (on it) and a clear distance (off it) both pass; only a
/// distance the band cannot call refuses, and a smaller tolerance calls
/// any nonzero one. The one ending of every refusal of it, at import
/// and at rest.
pub const ON_BOUNDARY: SizedDecision = SizedDecision {
    lever: "move the face's boundary, or its vertex, onto that point or clear of it",
    size: "distance",
    passes: SizedPass::AnySign,
    side: RefusedSide::Lever,
    no_size: None,
};

/// Which singular point of its chart a point is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Singularity {
    /// A sphere's pole along its axis.
    North,
    /// A sphere's pole against its axis.
    South,
    /// A cone's apex.
    Apex,
}

/// **A chart's singular points**: a sphere's two poles, north first, and
/// a cone's apex, from the chart's own frame; none for any other kind.
pub fn singular_points<T: Real>(chart: &ChartInverse<T>) -> Vec<(Singularity, Point3<T>)> {
    let (anchor, axis) = (chart.anchor(), chart.axis());
    match chart.arm() {
        ChartArm::Sphere { radius } => vec![
            (Singularity::North, anchor + axis * radius),
            (Singularity::South, anchor - axis * radius),
        ],
        ChartArm::Cone { .. } => vec![(Singularity::Apex, anchor)],
        ChartArm::Plane | ChartArm::Cylinder { .. } | ChartArm::Torus { .. } => Vec::new(),
    }
}

/// Where a singular point lies against a face ([`locate`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lies<T> {
    /// At the traversal start of `loops[r#loop][edge]`: a vertex of the
    /// face, which the rule admits.
    AtVertex {
        /// The loop, in the order given.
        r#loop: usize,
        /// The edge whose traversal start it is.
        edge: usize,
    },
    /// Inside `loops[r#loop][edge]`, at its carrier parameter `t`.
    InsideEdge {
        /// The loop, in the order given.
        r#loop: usize,
        /// The edge.
        edge: usize,
        /// The carrier parameter of the edge's point nearest it.
        t: T,
    },
    /// In the face's interior.
    Interior,
    /// Outside the face's closure.
    Outside,
}

/// A reading that does not place the point ([`locate`]).
#[derive(Clone, Debug)]
pub enum Unread<T> {
    /// Its distance to a vertex or an edge is in band
    /// ([`on_boundary`]).
    Boundary {
        /// The escalation, carrying the margin.
        cause: Indeterminate,
        /// The distance.
        distance: T,
    },
    /// The side of a loop it lies on is in band
    /// ([`loop_holds_on_left`]).
    Side(Indeterminate),
    /// `loops[r#loop][edge]` rides a carrier this reading has no
    /// closed-form distance to (not a line or a circle), and its samples
    /// pass near enough to the point that they cannot place it off it.
    Carrier {
        /// The loop.
        r#loop: usize,
        /// The edge.
        edge: usize,
    },
    /// `loops[r#loop]`'s chart image does not state a winding about the
    /// surface axis: a sample reads no chart point away from every
    /// singular point, or the loop winds more than once.
    Loop {
        /// The loop.
        r#loop: usize,
    },
}

/// One singular point of the face's surface, located ([`locate`]).
#[derive(Clone, Debug)]
pub struct Located<T: Real> {
    /// Which singular point.
    pub singularity: Singularity,
    /// The point.
    pub point: Point3<T>,
    /// Where it lies.
    pub lies: Result<Lies<T>, Unread<T>>,
}

/// **Whether a distance `d` from a singular point to a vertex or edge
/// puts it on that feature**, at `band` (`chart_singularity_on_boundary`,
/// [`ON_BOUNDARY`]): on it where `d` decides zero, off it where it
/// decides clear.
///
/// # Errors
///
/// The escalation of a distance in the band.
pub fn on_boundary<T: Decide>(d: T, band: Band) -> Result<bool, Indeterminate> {
    decide("chart_singularity_on_boundary", Margin::of(d), band).map(|s| s == Sign::Zero)
}

/// **Every singular point of `surface` against the face** whose loops
/// are `loops` (any order; every loop closed, each edge with its stored
/// interval and traversal) and whose sense is `sense` (its outward normal
/// is the chart normal where `true`), on/off and side decided at `band`
/// (module docs). Empty for a surface with no singular point.
///
/// Each point is read in turn: at a vertex, else inside an edge (the
/// loops and edges in order, the first that decides on), else inside
/// the face or outside it. A reading that cannot place a point
/// ([`Unread`]) is that point's answer; the others are read regardless.
pub fn locate<T: Decide>(
    surface: &Surface<T>,
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    band: Band,
) -> Vec<Located<T>> {
    let Ok(chart) = ChartInverse::of(surface) else {
        return Vec::new();
    };
    let points = singular_points(&chart);
    if points.is_empty() {
        return Vec::new();
    }
    let mut walks: Vec<Option<Result<LoopWalk<T>, Unread<T>>>> = vec![None; loops.len()];
    points
        .iter()
        .map(|&(singularity, x)| Located {
            singularity,
            point: x,
            lies: locate_one(
                &chart,
                &points,
                loops,
                sense,
                singularity,
                x,
                band,
                &mut walks,
            ),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)] // one point's reading, with the face's shared walks
fn locate_one<T: Decide>(
    chart: &ChartInverse<T>,
    points: &[(Singularity, Point3<T>)],
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    singularity: Singularity,
    x: Point3<T>,
    band: Band,
    walks: &mut [Option<Result<LoopWalk<T>, Unread<T>>>],
) -> Result<Lies<T>, Unread<T>> {
    let boundary = |distance: T| {
        on_boundary(distance, band).map_err(|cause| Unread::Boundary { cause, distance })
    };
    // At a vertex: the nearest traversal start decides; a farther one
    // cannot decide zero where the nearest does not.
    let mut nearest: Option<T> = None;
    for edges in loops {
        for e in *edges {
            let d = (e.traversal_ends().0 - x).norm();
            nearest = Some(nearest.map_or(d, |n| n.min(d)));
        }
    }
    if let Some(d) = nearest
        && boundary(d)?
    {
        for (l, edges) in loops.iter().enumerate() {
            for (k, e) in edges.iter().enumerate() {
                if on_boundary((e.traversal_ends().0 - x).norm(), band).unwrap_or(false) {
                    return Ok(Lies::AtVertex { r#loop: l, edge: k });
                }
            }
        }
    }
    // Inside an edge.
    for (l, edges) in loops.iter().enumerate() {
        for (k, e) in edges.iter().enumerate() {
            match nearest_on_edge(e, x, band) {
                Near::Off => {}
                Near::On { t, d } => {
                    if boundary(d)? {
                        return Ok(Lies::InsideEdge {
                            r#loop: l,
                            edge: k,
                            t,
                        });
                    }
                }
                Near::Unread => return Err(Unread::Carrier { r#loop: l, edge: k }),
            }
        }
    }
    // Inside the face.
    Ok(
        if holds_with(chart, points, loops, sense, singularity, band, walks)? {
            Lies::Interior
        } else {
            Lies::Outside
        },
    )
}

/// Where a point lies against one edge's carrier.
enum Near<T> {
    /// Its nearest point is no interior point of the edge, or lies past
    /// the band's escalation.
    Off,
    /// Within reach of the edge's interior point at carrier parameter
    /// `t`, `d` away: on or off is [`on_boundary`]'s.
    On { t: T, d: T },
    /// A carrier read by samples, which pass too near to place it off.
    Unread,
}

/// **The point of `e` nearest `x`**: a line's projection, or a circle's
/// angle about its centre in its own frame brought to the turn at or
/// past `t0`, kept where it lies strictly inside the edge's interval
/// (its ends are vertices, which [`locate`] reads first). Any other
/// carrier is sampled and placed off only where every sample stands
/// clear of `x` by the band's escalation plus the widest step between
/// samples.
fn nearest_on_edge<T: Decide>(e: &LoopEdge<T>, x: Point3<T>, band: Band) -> Near<T> {
    let (t0, t1) = (e.t0, e.t1);
    let (t, speed) = match e.carrier {
        Curve3::Line { origin, dir } => ((x - origin).dot(dir) / dir.dot(dir), dir.norm()),
        Curve3::Circle {
            center,
            axis,
            u_ref,
            radius,
        } => {
            let w = x - center;
            let t = w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref));
            (t0 + (t - t0).reduce_periodic(T::tau()), radius)
        }
        _ => {
            let n = 256_u32;
            let mut best: Option<T> = None;
            let mut step = T::zero();
            let mut prev = e.carrier.eval(t0);
            for i in 0..=n {
                let s = T::from_f64(f64::from(i)) / T::from_f64(f64::from(n));
                let p = e.carrier.eval(t0 + (t1 - t0) * s);
                let d = (p - x).norm();
                best = Some(best.map_or(d, |b| b.min(d)));
                step = step.max((p - prev).norm());
                prev = p;
            }
            let clear = best.map(|b| {
                decide(
                    "chart_singularity_sampled_clear",
                    Margin::of(b - step),
                    band,
                )
            });
            return match clear {
                Some(Ok(Sign::Positive)) => Near::Off,
                _ => Near::Unread,
            };
        }
    };
    // Strictly inside the interval, an exact order (its ends are the
    // edge's vertices, read before).
    let inside = |m: T| {
        matches!(
            decide(
                "chart_singularity_in_span",
                Margin::of(m * speed),
                Band::least()
            ),
            Ok(Sign::Positive)
        )
    };
    if !(inside(t - t0) && inside(t1 - t)) {
        return Near::Off;
    }
    let d = (e.carrier.eval(t) - x).norm();
    match decide("chart_singularity_on_boundary", Margin::of(d), band) {
        Ok(Sign::Positive) => Near::Off,
        _ => Near::On { t, d },
    }
}

/// **One loop walked through its surface's chart**: the reading every
/// side of it is decided from ([`walk_loop`]).
#[derive(Clone, Copy, Debug)]
pub struct LoopWalk<T> {
    /// Its winding about the surface axis, in turns: `−1`, `0` or `1`.
    pub winding: i8,
    /// The signed area of its chart image, metric: positive where the
    /// image runs counter-clockwise. Read only for a loop that does not
    /// wind, whose image closes in the chart.
    pub area: T,
    /// The length of the polygon its samples make, an under-estimate of
    /// its own.
    pub length: T,
    /// The midpoint of its samples' chart `v` range: on a cone, a slant
    /// whose sign is the nappe a loop off the apex runs on, every sample
    /// being on it.
    pub mid_v: T,
}

/// **The walk of one loop through `chart`**, the reading its sides are
/// decided from ([`LoopWalk`]). Each edge is sampled along its traversal
/// and read through the chart inverse at `band`, a sample bisected where
/// the azimuth steps by more than a sixteenth of a turn; each step of
/// azimuth is taken as its principal angle, so the steps of a closed
/// loop sum to whole turns exactly. A sample on a singular point (one of
/// `points`, decided not clear of it at `band`) has no chart preimage
/// and is skipped: the step across it is read as the nearest
/// representative, which stands for a detour round that point on one
/// side or the other, and so leaves the side of every OTHER singular
/// point as it is. A corner contributes the step between the samples
/// either side of it and nothing else.
///
/// `None` where a sample reads no chart point away from every singular
/// point, fewer than three samples read, or the loop winds more than
/// once.
#[must_use]
pub fn walk_loop<T: Decide>(
    chart: &ChartInverse<T>,
    points: &[(Singularity, Point3<T>)],
    edges: &[LoopEdge<T>],
    band: Band,
) -> Option<LoopWalk<T>> {
    let wrap = |d: T| {
        let (s, c) = d.sin_cos();
        s.atan2(c)
    };
    let pi = T::pi();
    // The arm an azimuth step is levered by in its exact order: any
    // positive length scales the sign alone.
    let arm = |p: Point3<T>| (p - chart.anchor()).norm();
    let mut charted: Vec<(Point3<T>, Point2<T>)> = Vec::new();
    for e in edges {
        let (a, b) = if e.forward {
            (e.t0, e.t1)
        } else {
            (e.t1, e.t0)
        };
        let at = |t: T| -> Result<(Point3<T>, Option<Point2<T>>), ()> {
            let p = e.carrier.eval(t);
            let read = chart.point(p, band);
            if !read.singular {
                return Ok((p, Some(read.uv)));
            }
            let on_point = points.iter().any(|&(_, s)| {
                !matches!(
                    decide("chart_singularity_sample", Margin::of((s - p).norm()), band),
                    Ok(Sign::Positive)
                )
            });
            if on_point { Ok((p, None)) } else { Err(()) }
        };
        let n = 16_u32;
        let mut ts: Vec<(T, u32)> = (0..=n)
            .map(|i| {
                (
                    a + (b - a) * T::from_f64(f64::from(i)) / T::from_f64(f64::from(n)),
                    0,
                )
            })
            .collect();
        let mut read: Vec<(Point3<T>, Option<Point2<T>>)> = ts
            .iter()
            .map(|&(t, _)| at(t))
            .collect::<Result<_, _>>()
            .ok()?;
        // Bisect where the azimuth steps past a sixteenth of a turn, each
        // gap at most `DEPTH` times and the loop to a bounded count.
        const DEPTH: u32 = 50;
        loop {
            let mut refined = false;
            let mut i = 0;
            while i + 1 < ts.len() {
                let wide = match (read[i], read[i + 1]) {
                    ((p, Some(u)), (_, Some(v))) => matches!(
                        decide(
                            "chart_singularity_refine",
                            Margin::levered(wrap(v.x - u.x).abs() - pi / T::from_f64(8.0), arm(p)),
                            Band::least(),
                        ),
                        Ok(Sign::Positive)
                    ),
                    _ => false,
                };
                let depth = ts[i].1.max(ts[i + 1].1);
                if wide && depth < DEPTH {
                    let tm = (ts[i].0 + ts[i + 1].0) * T::from_f64(0.5);
                    ts.insert(i + 1, (tm, depth + 1));
                    read.insert(i + 1, at(tm).ok()?);
                    refined = true;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if !refined || ts.len() > 1 << 16 {
                break;
            }
        }
        // The end is the next edge's start.
        charted.extend(
            read[..read.len() - 1]
                .iter()
                .filter_map(|&(p, uv)| uv.map(|uv| (p, uv))),
        );
    }
    let m = charted.len();
    if m < 3 {
        return None;
    }
    // The lifted image, its total azimuth, its length and its mean v.
    let mut lifted = Vec::with_capacity(m);
    let mut total = T::zero();
    let mut length = T::zero();
    let (mut v_lo, mut v_hi) = (charted[0].1.y, charted[0].1.y);
    for (i, &(p, uv)) in charted.iter().enumerate() {
        (v_lo, v_hi) = (v_lo.min(uv.y), v_hi.max(uv.y));
        let (q, prev) = charted[(i + m - 1) % m];
        length = length + (p - q).norm();
        if i == 0 {
            lifted.push(uv);
            continue;
        }
        let step = wrap(uv.x - prev.x);
        total = total + step;
        lifted.push(Point2::new(lifted[i - 1].x + step, uv.y));
    }
    total = total + wrap(charted[0].1.x - charted[m - 1].1.x);
    // Whole turns by construction: read at the half turns, exactly.
    let past = |c: f64| {
        decide(
            "chart_singularity_winding",
            Margin::levered(total - pi * T::from_f64(c), length),
            Band::least(),
        )
    };
    let winding = match (past(1.0).ok()?, past(-1.0).ok()?) {
        (Sign::Positive, _) => match past(3.0).ok()? {
            Sign::Negative => 1,
            _ => return None,
        },
        (_, Sign::Negative) => match past(-3.0).ok()? {
            Sign::Positive => -1,
            _ => return None,
        },
        _ => 0,
    };
    // The metric area left of the image, `−∮ g(v) du` with `∂g/∂v` the
    // chart's area element over `du dv`: `R² sin v` on a sphere, and on a
    // cone, `sin α · v|v| / 2` (its element `|v| sin α`).
    let g = |v: T| match chart.arm() {
        ChartArm::Sphere { radius } => radius * radius * v.sin(),
        ChartArm::Cone { half_angle } => half_angle.sin() * v * v.abs() * T::from_f64(0.5),
        ChartArm::Plane | ChartArm::Cylinder { .. } | ChartArm::Torus { .. } => T::zero(),
    };
    // The closing step returns the lifted image to its start less the
    // turns it wound.
    let turns = match winding {
        1 => T::tau(),
        -1 => -T::tau(),
        _ => T::zero(),
    };
    let mut area = T::zero();
    for i in 0..m {
        let (p, q) = (lifted[i], lifted[(i + 1) % m]);
        let du = if i + 1 == m {
            q.x - p.x + turns
        } else {
            q.x - p.x
        };
        area = area - (g(p.y) + g(q.y)) * T::from_f64(0.5) * du;
    }
    Some(LoopWalk {
        winding,
        area,
        length,
        mid_v: (v_lo + v_hi) * T::from_f64(0.5),
    })
}

/// **Whether a loop's left side holds the singular point**, under the
/// chart normal (module docs; `chart_singularity_loop_side` for a loop
/// that does not wind, `chart_singularity_nappe` for a cone loop that
/// does).
///
/// # Errors
///
/// The escalation of either decision: an image of no area to read a
/// side off, or a loop on no decided nappe.
pub fn loop_holds_on_left<T: Decide>(
    walk: &LoopWalk<T>,
    singularity: Singularity,
    band: Band,
) -> Result<bool, Indeterminate> {
    match walk.winding {
        0 => {
            let side = decide_nonzero(
                "chart_singularity_loop_side",
                Margin::over_lever(walk.area, walk.length),
                band,
            )?;
            Ok(side == NonzeroSign::Negative)
        }
        w => match singularity {
            Singularity::North => Ok(w == 1),
            Singularity::South => Ok(w == -1),
            Singularity::Apex => Ok((w == 1) == (nappe(walk, band)? == NonzeroSign::Negative)),
        },
    }
}

/// **The nappe a cone loop off the apex runs on**: the sign of its
/// samples' slant (`chart_singularity_nappe`).
///
/// # Errors
///
/// The escalation of a slant range centred in band of the apex.
pub fn nappe<T: Decide>(walk: &LoopWalk<T>, band: Band) -> Result<NonzeroSign, Indeterminate> {
    decide_nonzero("chart_singularity_nappe", Margin::of(walk.mid_v), band)
}

/// **Whether the singular point lies on the face's side of every
/// loop** — in the face, for a point off its boundary (module docs).
/// A loop that places it on its other side settles it outside, whatever
/// any other loop reads.
///
/// # Errors
///
/// [`Unread::Loop`] for a loop whose walk states no winding, and
/// [`Unread::Side`] for a side the band cannot call, where no loop
/// settles the point outside.
pub fn holds<T: Decide>(
    surface: &Surface<T>,
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    singularity: Singularity,
    band: Band,
) -> Result<bool, Unread<T>> {
    let Ok(chart) = ChartInverse::of(surface) else {
        return Ok(false);
    };
    let points = singular_points(&chart);
    let mut walks = vec![None; loops.len()];
    holds_with(&chart, &points, loops, sense, singularity, band, &mut walks)
}

fn holds_with<T: Decide>(
    chart: &ChartInverse<T>,
    points: &[(Singularity, Point3<T>)],
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    singularity: Singularity,
    band: Band,
    walks: &mut [Option<Result<LoopWalk<T>, Unread<T>>>],
) -> Result<bool, Unread<T>> {
    let mut unread: Option<Unread<T>> = None;
    for (l, edges) in loops.iter().enumerate() {
        let walk = walks[l]
            .get_or_insert_with(|| {
                walk_loop(chart, points, edges, band).ok_or(Unread::Loop { r#loop: l })
            })
            .clone();
        let left =
            walk.and_then(|w| loop_holds_on_left(&w, singularity, band).map_err(Unread::Side));
        match left {
            Ok(left) if left != sense => return Ok(false),
            Ok(_) => {}
            Err(why) => {
                unread.get_or_insert(why);
            }
        }
    }
    unread.map_or(Ok(true), Err)
}

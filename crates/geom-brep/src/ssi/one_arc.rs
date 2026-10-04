//! **Limb 3's one-arc proof**: a tube's chain holds one arc of the locus,
//! spanning the carrier, and nothing else (C2). The proof is the same at
//! every door, a search's and an edge's at rest.
//!
//! Limb 3's enclosure, zero-free over a box, makes the solution set in
//! the box a graph over the slices transverse to the carrier: at most one
//! solution on each. That is less than one arc. A second arc beside the
//! first along the carrier, leaving through the box's sides, is a graph
//! too, and so is a carrier joining two arcs across a gap within ε of
//! both surfaces. This module proves the rest.
//!
//! **One piece per box.** Let `R` be a box cut to the region it is
//! searched over (the wall's knot rectangle; the ℝ³ slab where a search
//! clips to one); it is convex. Each connected piece of the solution set
//! in `R` is a graph over the slices, so it ends on `∂R` at two distinct
//! points. It cannot end inside `R`, where the zero set continues by the
//! implicit function theorem, and it cannot close up. A boundary holding
//! exactly two solutions, each a simple crossing of `∂R`, therefore holds
//! the ends of exactly one piece. A tangential touch of `∂R` is never
//! counted as a crossing: on an edge it fails the monotone test, at a
//! corner the joint rule refuses it, and on a face Krawczyk cannot
//! isolate it.
//!
//! **The side arm.** Along a side of the wall's domain that lies on the
//! plane within ε no boundary zero is simple, and no count is read. There
//! a chart box with an edge on that side holds the side's piece where the
//! boundary pass, reading that stretch of the side as it reads a whole
//! side, finds `|φ| ≤ ε` along it and no piece of it clear of the plane
//! (Ev's #3862 rule: `φ` one-signed along it and the wall moving further
//! that way inward, the exact empty answer), and the side's cover puts
//! every solution in the box within ε of the side ([`read_stretch`],
//! [`side_stretch`]). The piece is that stretch of the side, as a region
//! of the locus, not a curve. Two stretches of one side link where their
//! overlap reads within ε and no piece of it clear.
//!
//! **One arc, spanning the carrier.** Consecutive boxes whose pieces
//! share a solution hold one connected arc between them, and every
//! solution in the chain lies on it. The first box's piece meets the
//! slice through the carrier's start and the last box's the slice
//! through its end, so the arc runs from end to end of the carrier: a
//! carrier overrunning its arc refuses. An arc's end counts also where a
//! zero of the locus is certified within ε of the carrier's end on a
//! slice beside it; a side's where the end, moved across onto the side,
//! lands on its stretch at a point the boundary pass reads within ε of
//! the plane and not clear. [`Shortfall::Short`] is read only where that
//! is certified not so, and a reading that resolves neither is
//! [`Shortfall::Undecided`]. A side's end whose `|φ|` is not certified
//! within ε is such a reading. Where a box resolves no piece but an end
//! is certified unreached by its own box (by the side's reading where a
//! side holds that box's piece, by an arc's where none does), no chain
//! reaches that end, and the refusal is `Short`.
//!
//! **The walks.**
//! - Chart edges ([`boundary_zeros`]): each edge is cut into runs, each
//!   of one certified sign or monotone along the edge.
//! - ℝ³ faces ([`face_roots`]): each face is cut into pieces, each
//!   isolated by the Krawczyk operator of the pair restricted to it.
//!
//! Both cut at [`EXIT_CUT`] and stop at [`EXIT_DEPTH`] cuts or
//! [`EXIT_PIECES`] pieces. A walk that stops there proves nothing, and
//! says so ([`Shortfall::Undecided`]).

use geom::{NurbsCurve2, Surface};
use geom_core::{Band, Bounds, CertifiedBounds, Interval};

use super::ChartAxis;
use super::boundary::{ChartEnd, ChartSide, Reading, SIDES, cut_along, read_stretch, side_stretch};
use super::certify::ChartWindow;
use super::enclose::{Box3, NurbsBoxes, implicit_enclosure, implicit_gradient_enclosure};
use super::exhaust::UvRect;
use super::section::{SectionReader, sign};

/// The deepest a walk cuts one edge or face piece.
pub(crate) const EXIT_DEPTH: u32 = 40;

/// Where a walk cuts a piece, as a fraction of it from its low end. The
/// value is a choice, not a derivation: anything off the middle, because
/// a box is centred on the carrier, so the arc crosses its boundary at
/// the middle of a side as often as not, and a solution on a cut is one
/// no piece can isolate.
pub(crate) const EXIT_CUT: f64 = 0.5 - 1.0 / 128.0;

/// How wide across an edge the strip is that an edge piece's derivative
/// is read over, as a fraction of the piece's length. A choice: small
/// enough that the derivative's variation across the strip stays below
/// the slack a band-thin `φ` along the edge leaves, and the strip is
/// never under two ulps of the edge's coordinate.
pub(crate) const EXIT_STRIP: f64 = 1.0 / 1_073_741_824.0;

/// The most pieces one box's walk examines.
pub(crate) const EXIT_PIECES: u32 = 4096;

/// Why a chain was not proved one arc.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Shortfall {
    /// A box's boundary holds this many simple solutions, certified,
    /// other than two.
    Count(u32),
    /// Two consecutive boxes each hold one piece, but the reading that
    /// would show the pieces meet certifies no shared solution there.
    Unlinked,
    /// The chain's pieces do not reach an end of the carrier: the slice
    /// through that end is certified to hold no solution, and the
    /// carrier's end lies farther than ε from the locus.
    Short,
    /// A walk resolved no count, or a linking or end reading no sign.
    Undecided,
}

/// The point interval at `x`.
fn pt(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

/// The plane distance `φ = n·(S − p₀)` over a parameter rectangle.
fn phi_over<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    (n, p0): ([Interval; 3], [Interval; 3]),
    (u0, u1, v0, v1): (f64, f64, f64, f64),
) -> Interval {
    let b = boxes.rect_box(u0, u1, v0, v1);
    n[0] * (b.x - p0[0]) + n[1] * (b.y - p0[1]) + n[2] * (b.z - p0[2])
}

/// The intersection of two rectangles, `None` when it has no interior.
fn meet(a: UvRect, b: UvRect) -> Option<UvRect> {
    let u = (a.u.0.max(b.u.0), a.u.1.min(b.u.1));
    let v = (a.v.0.max(b.v.0), a.v.1.min(b.v.1));
    (u.0 < u.1 && v.0 < v.1).then_some(UvRect { u, v })
}

/// One resolved piece of a box's boundary, in walk order.
#[derive(Clone, Copy, Debug)]
enum Run {
    /// `φ` zero-free over the piece, of this sign.
    Constant(bool),
    /// `φ` monotone along the piece, so at most one zero on it, with the
    /// certified signs of its two ends where they have one.
    Monotone(Option<bool>, Option<bool>),
}

impl Run {
    /// The signs at the run's two ends, in walk order.
    fn ends(self) -> (Option<bool>, Option<bool>) {
        match self {
            Self::Constant(s) => (Some(s), Some(s)),
            Self::Monotone(a, b) => (a, b),
        }
    }

    /// The run walked the other way.
    fn reversed(self) -> Self {
        match self {
            Self::Constant(s) => Self::Constant(s),
            Self::Monotone(a, b) => Self::Monotone(b, a),
        }
    }
}

/// One edge of a box's boundary resolved into [`Run`]s, in the order of
/// its running coordinate.
///
/// Each piece is cut until one of two things holds:
/// - `φ` is zero-free over it. That is read by the mean-value form about
///   the piece's middle, which keeps the cancellation of `n·S` along the
///   piece that a box of `S` per coordinate loses, or else by that box.
/// - Its derivative along the edge is zero-free.
///
/// `None` when a piece resolves neither way within [`EXIT_DEPTH`] cuts,
/// or `pieces` passes [`EXIT_PIECES`].
fn edge_runs<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (along_u, c, (a, b)): (bool, f64, (f64, f64)),
    pieces: &mut u32,
) -> Option<Vec<Run>> {
    let n = plane.0;
    let piece = |s: f64, t: f64| if along_u { (s, t, c, c) } else { (c, c, s, t) };
    let sign_at = |s: f64| sign(phi_over(boxes, plane, piece(s, s)));
    let mut out = Vec::new();
    let mut stack = vec![(a, b, 0u32)];
    while let Some((s, t, depth)) = stack.pop() {
        *pieces += 1;
        if *pieces > EXIT_PIECES {
            return None;
        }
        // The derivative over a strip across the edge: of positive width,
        // since a net is cut only to a rectangle that has one and a strip
        // of none would read the whole span cell's, and thin beside the
        // piece, since the derivative along the edge varies across it and
        // a wide strip would read that variation into the slope.
        let w = (t - s) * EXIT_STRIP;
        let across = ((c - w).min(c.next_down()), (c + w).max(c.next_up()));
        let (r0, r1, r2, r3) = if along_u {
            (s, t, across.0, across.1)
        } else {
            (across.0, across.1, s, t)
        };
        let d = boxes.deriv_box(r0, r1, r2, r3, along_u);
        let slope = n[0] * d.x + n[1] * d.y + n[2] * d.z;
        // `[m − h, m + h]` holds `[s, t]`: `h` is the larger half,
        // rounded up.
        let m = 0.5 * (s + t);
        let h = (m - s).max(t - m).next_up();
        let mean = phi_over(boxes, plane, piece(m, m)) + slope * Interval::from_bounds(-h, h);
        if let Some(s) = sign(mean).or_else(|| sign(phi_over(boxes, plane, piece(s, t)))) {
            out.push(Run::Constant(s));
            continue;
        }
        if sign(slope).is_some() {
            out.push(Run::Monotone(sign_at(s), sign_at(t)));
            continue;
        }
        if depth >= EXIT_DEPTH {
            return None;
        }
        let cut = s + EXIT_CUT * (t - s);
        // Popped low half first, so `out` runs in the edge's order.
        stack.push((cut, t, depth + 1));
        stack.push((s, cut, depth + 1));
    }
    Some(out)
}

/// How many zeros `φ` has on the boundary of `rect`, each certified a
/// simple crossing, walked once around as [`Run`]s.
///
/// - Between two points of certified sign, one run holds one zero where
///   the signs differ and none where they agree.
/// - Two monotone runs meeting at a point of no certified sign (a zero on
///   a cut or a corner) hold exactly one zero where the outer signs
///   differ.
/// - Outer signs that agree around such a point could hold none or two:
///   a touch at a corner, which is no crossing. That is `None`, as are
///   two such points in a row, and an edge [`edge_runs`] does not
///   resolve.
pub(crate) fn boundary_zeros<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    rect: UvRect,
) -> Option<u32> {
    let ((u0, u1), (v0, v1)) = (rect.u, rect.v);
    let mut pieces = 0u32;
    // Counter-clockwise: the top and left edges are walked backwards.
    let mut runs = Vec::new();
    for (edge, back) in [
        ((true, v0, (u0, u1)), false),
        ((false, u1, (v0, v1)), false),
        ((true, v1, (u0, u1)), true),
        ((false, u0, (v0, v1)), true),
    ] {
        let mut r = edge_runs(boxes, plane, edge, &mut pieces)?;
        if back {
            r = r.into_iter().rev().map(Run::reversed).collect();
        }
        runs.extend(r);
    }
    // The sign at each joint (after run `j`): either neighbour's word.
    let k = runs.len();
    let joints: Vec<Option<bool>> = (0..k)
        .map(|j| runs[j].ends().1.or(runs[(j + 1) % k].ends().0))
        .collect();
    let start = joints.iter().position(Option::is_some)?;
    let mut zeros = 0u32;
    let mut from = joints[start]?;
    let mut unknown = 0u32;
    for step in 1..=k {
        let j = (start + step) % k;
        let Some(to) = joints[j] else {
            unknown += 1;
            if unknown > 1 {
                return None;
            }
            continue;
        };
        match (unknown, from == to) {
            (_, false) => zeros += 1,
            (0, true) => {}
            (_, true) => return None,
        }
        from = to;
        unknown = 0;
    }
    Some(zeros)
}

/// The ends of the line through `p` along `e`, cut to `rect`, each
/// clamped into it, so the segment between them lies in `rect`
/// (convex). `None` where the line misses the interior of `rect`.
fn slice(p: (f64, f64), e: (f64, f64), rect: UvRect) -> Option<[(f64, f64); 2]> {
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for (pc, ec, (r0, r1)) in [(p.0, e.0, rect.u), (p.1, e.1, rect.v)] {
        if ec == 0.0 {
            if !(r0..=r1).contains(&pc) {
                return None;
            }
            continue;
        }
        let (a, b) = ((r0 - pc) / ec, (r1 - pc) / ec);
        lo = lo.max(a.min(b));
        hi = hi.min(a.max(b));
    }
    if lo.partial_cmp(&hi) != Some(core::cmp::Ordering::Less) {
        return None;
    }
    let at = |t: f64| {
        (
            (p.0 + t * e.0).clamp(rect.u.0, rect.u.1),
            (p.1 + t * e.1).clamp(rect.v.0, rect.v.1),
        )
    };
    Some([at(lo), at(hi)])
}

/// The certified sign of `φ` at a chart point.
fn sign_at<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (u, v): (f64, f64),
) -> Option<bool> {
    sign(phi_over(boxes, plane, (u, u, v, v)))
}

/// Whether `φ` changes sign between the two ends of the line through
/// `p` along `e`, cut to `rect` ([`slice`]). `Some(true)` puts a zero on
/// the segment between them, which lies in `rect`. `Some(false)`: both
/// ends are certified of one sign. `None`: an end has no certified
/// sign, or the line misses the interior of `rect`.
pub(crate) fn holds_zero<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    p: (f64, f64),
    e: (f64, f64),
    rect: UvRect,
) -> Option<bool> {
    let [a, b] = slice(p, e, rect)?;
    Some(sign_at(boxes, plane, a)? != sign_at(boxes, plane, b)?)
}

/// What one window, cut to the wall's domain, holds: exactly one piece
/// of the locus, of one of two kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Piece {
    /// Two simple solutions on its boundary ([`boundary_zeros`]): the
    /// ends of one arc.
    Crossing,
    /// A stretch of its boundary on this side of the wall's domain that
    /// the boundary pass reads within ε of the plane with no piece of it
    /// clear, with every solution in the window within ε of the side by
    /// the side's cover ([`side_stretch`]).
    Side(ChartSide),
}

/// The coordinate `side` fixes, of `r`'s edge on that side.
fn edge_at(side: ChartSide, r: UvRect) -> f64 {
    let (lo, hi) = match side.fixed {
        ChartAxis::U => r.u,
        ChartAxis::V => r.v,
    };
    if side.end == ChartEnd::Low { lo } else { hi }
}

/// The coordinate `side` fixes, of a chart point, and the one along it.
fn split((u, v): (f64, f64), side: ChartSide) -> (f64, f64) {
    match side.fixed {
        ChartAxis::U => (u, v),
        ChartAxis::V => (v, u),
    }
}

/// Whether `r` has an edge on `side` of `domain`.
fn on_side(r: UvRect, domain: UvRect, side: ChartSide) -> bool {
    edge_at(side, r) == edge_at(side, domain)
}

/// The reader of `φ` along each side of the wall's domain, in [`SIDES`]
/// order: the boundary pass's row of the net for that side
/// ([`super::boundary::side_row`]), its control points enclosed
/// ([`SectionReader`]); `None` where the row or its
/// Bernstein form is refused, and that side holds no piece.
fn side_readers<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    (n, p0): ([Interval; 3], [Interval; 3]),
) -> [Option<SectionReader>; 4] {
    SIDES.map(|side| {
        let row = super::boundary::side_row(boxes.surface(), side).ok()?;
        let control: Vec<[Interval; 3]> = row
            .control()
            .iter()
            .map(|c| [c.x, c.y, c.z].map(Interval::from_certified))
            .collect();
        SectionReader::of(row.knots(), &control, row.weights(), (p0, n))
    })
}

/// The reader of `side`, from [`side_readers`].
fn reader_of(readers: &[Option<SectionReader>; 4], side: ChartSide) -> Option<&SectionReader> {
    SIDES
        .iter()
        .zip(readers)
        .find_map(|(s, r)| (*s == side).then_some(r.as_ref()).flatten())
}

/// The side a window's piece lies along: one of its edges on a side of
/// the domain, whose stretch the boundary pass reads within ε and no
/// piece of it clear, with the side's cover holding every zero of the
/// window within ε of the side ([`side_stretch`]); `None` where none
/// does.
fn side_piece<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (domain, readers): (UvRect, &[Option<SectionReader>; 4]),
    r: UvRect,
    band: Band,
) -> Option<ChartSide> {
    SIDES.into_iter().find(|&side| {
        on_side(r, domain, side)
            && reader_of(readers, side)
                .is_some_and(|reader| side_stretch(boxes, plane.0, reader, (side, domain), r, band))
    })
}

/// The one piece a window holds: the side cover, or else its
/// boundary's two simple zeros. The cover is read first: along a side
/// within ε of the plane no zero on it is simple, and the walk spends
/// its pieces finding that out.
///
/// # Errors
///
/// [`Shortfall::Count`] for a count other than two with no side cover,
/// [`Shortfall::Undecided`] for a walk that resolved none.
fn piece<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    sides: (UvRect, &[Option<SectionReader>; 4]),
    r: UvRect,
    band: Band,
) -> Result<Piece, Shortfall> {
    if let Some(side) = side_piece(boxes, plane, sides, r, band) {
        return Ok(Piece::Side(side));
    }
    match boundary_zeros(boxes, plane, r) {
        Some(2) => Ok(Piece::Crossing),
        Some(n) => Err(Shortfall::Count(n)),
        None => Err(Shortfall::Undecided),
    }
}

/// A reading that should certify a solution.
fn found(reading: Option<bool>, missing: Shortfall) -> Result<(), Shortfall> {
    match reading {
        Some(true) => Ok(()),
        Some(false) => Err(missing),
        None => Err(Shortfall::Undecided),
    }
}

/// The Euclidean norm of a vector of three non-negative bounds, as an
/// enclosure.
fn norm3(c: [f64; 3]) -> Interval {
    use geom_core::Real as _;
    let [x, y, z] = c.map(pt);
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

/// How far any point of `a` can lie from any point of `b`, rounded up;
/// `∞` where a side is not certified.
fn farthest(a: Box3, b: Box3) -> f64 {
    if ![a.x, a.y, a.z, b.x, b.y, b.z]
        .iter()
        .all(|i| i.is_certified())
    {
        return f64::INFINITY;
    }
    let d = |x: Interval, y: Interval| (x.hi() - y.lo()).max(y.hi() - x.lo()).next_up();
    norm3([d(a.x, b.x), d(a.y, b.y), d(a.z, b.z)]).hi()
}

/// How near any point of `a` can lie to any point of `b`, rounded down;
/// `0` where a side is not certified.
fn nearest(a: Box3, b: Box3) -> f64 {
    if ![a.x, a.y, a.z, b.x, b.y, b.z]
        .iter()
        .all(|i| i.is_certified())
    {
        return 0.0;
    }
    let g = |x: Interval, y: Interval| (x.lo() - y.hi()).max(y.lo() - x.hi()).next_down().max(0.0);
    norm3([g(a.x, b.x), g(a.y, b.y), g(a.z, b.z)]).lo()
}

/// What a reading of the solutions near a carrier's end found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Near {
    /// A solution within ε of the end, certified.
    Found,
    /// None, certified: the reading's chord holds no sign change, or the
    /// part of it that does lies wholly farther than ε from the end.
    Far,
    /// The reading resolved neither.
    Unknown,
}

/// Whether the chord of `r` through `p` along `e` holds a solution
/// within `eps` metres of `end`: its ends take certified opposite signs,
/// and the chord is cut at [`EXIT_CUT`], keeping the sign change, until
/// the wall over the part kept lies within `eps` of `end`
/// ([`Near::Found`]), or wholly farther ([`Near::Far`]). A chord whose
/// wall lies wholly farther than `eps` from `end`, or with both ends
/// certified of one sign, is [`Near::Far`]; an unsigned point,
/// a missed slice, a cut below `f64` resolution or the halvings spent
/// are [`Near::Unknown`].
fn crossing_near<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    r: UvRect,
    (p, e): ((f64, f64), (f64, f64)),
    (end, eps): (Box3, f64),
) -> Near {
    let Some([a, b]) = slice(p, e, r) else {
        return Near::Unknown;
    };
    // The wall over the whole chord farther than `eps` from the end: no
    // solution on it is near, whatever its signs.
    let chord = boxes.rect_box(a.0.min(b.0), a.0.max(b.0), a.1.min(b.1), a.1.max(b.1));
    if nearest(chord, end) > eps {
        return Near::Far;
    }
    let (Some(sa), Some(sb)) = (sign_at(boxes, plane, a), sign_at(boxes, plane, b)) else {
        return Near::Unknown;
    };
    if sa == sb {
        return Near::Far;
    }
    let (mut lo, mut hi) = (a, b);
    for _ in 0..NEAR_HALVINGS {
        let seg = boxes.rect_box(
            lo.0.min(hi.0),
            lo.0.max(hi.0),
            lo.1.min(hi.1),
            lo.1.max(hi.1),
        );
        if farthest(seg, end) <= eps {
            return Near::Found;
        }
        if nearest(seg, end) > eps {
            return Near::Far;
        }
        // Off the middle, as the walks cut: a chord about a carrier on
        // the locus has it at its middle.
        let mid = (
            lo.0 + EXIT_CUT * (hi.0 - lo.0),
            lo.1 + EXIT_CUT * (hi.1 - lo.1),
        );
        if mid == lo || mid == hi {
            return Near::Unknown;
        }
        match sign_at(boxes, plane, mid) {
            Some(s) if s == sa => lo = mid,
            Some(_) => hi = mid,
            None => return Near::Unknown,
        }
    }
    Near::Unknown
}

/// Whether a solution of the window `r` lies within `eps` metres of the
/// carrier's end `end`, its chart point `q`: on the slice through `q`
/// along `e`, or on a slice through a point `q + δ·τ` beside it, `τ`
/// pointing back along the carrier and `δ` halved towards `q`
/// ([`crossing_near`]). The shifted slices read an end at a crossing of
/// the wall's domain side or corner, where the slice through `q` itself
/// leaves the domain at once. A shifted slice that misses the window is
/// not read: it holds none of the window's solutions. [`Near::Far`] only
/// where every slice read is.
fn near_end<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (q, e, tau): ((f64, f64), (f64, f64), (f64, f64)),
    r: UvRect,
    end: (Box3, f64),
) -> Near {
    let mut all = crossing_near(boxes, plane, r, (q, e), end);
    if all == Near::Found {
        return all;
    }
    let mut d = (r.u.1 - r.u.0).max(r.v.1 - r.v.0);
    for _ in 0..NEAR_HALVINGS {
        let p = (q.0 + d * tau.0, q.1 + d * tau.1);
        if p == q {
            return all;
        }
        d *= 0.5;
        // A slice that misses the window holds none of its solutions.
        if slice(p, e, r).is_none() {
            continue;
        }
        match crossing_near(boxes, plane, r, (p, e), end) {
            Near::Found => return Near::Found,
            Near::Unknown => all = Near::Unknown,
            Near::Far => {}
        }
    }
    Near::Unknown
}

/// The most halvings [`crossing_near`] and [`near_end`] take: past this
/// many an interval is below an `f64`'s resolution of any chord it could
/// be cut from.
const NEAR_HALVINGS: u32 = 1100;

/// A carrier's end, as the slice through it reads it.
#[derive(Clone, Copy, Debug)]
struct CarrierEnd {
    /// The slice's direction in the chart.
    e: (f64, f64),
    /// A chart direction pointing back along the carrier from its end:
    /// its tangent at its end span's middle.
    tau: (f64, f64),
    /// The carrier's point there, enclosed.
    at: Box3,
}

/// **The window's piece reaches the carrier's end.**
/// - A side's piece reaches it where the end's chart point, moved
///   straight across onto the side, lands on the window's stretch, at a
///   point the boundary pass reads not clear ([`read_stretch`]). The
///   slice through the end is not read: at a corner of the domain it
///   leaves the window at once.
/// - An arc reaches it where the slice holds a solution in the window
///   ([`holds_zero`]), or the carrier's end lies within ε of a solution
///   in the window ([`near_end`]).
///
/// [`Shortfall::Short`] only where that is certified not so: an arc's
/// end certified far ([`end_reading`]), a side's end off its stretch or
/// on a point the pass reads clear. A reading that resolves neither is
/// [`Shortfall::Undecided`]: an arc's end read neither way, a side's end
/// whose `φ` is refused, or whose `|φ|` is not certified within ε
/// ([`Reading::Beyond`], which is no certificate that it is beyond).
fn reaches_end<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (readers, eps): (&[Option<SectionReader>; 4], f64),
    (piece, r): (Piece, UvRect),
    q: (f64, f64),
    end: CarrierEnd,
) -> Result<(), Shortfall> {
    match piece {
        Piece::Side(side) => {
            let t = split(q, side).1;
            let (a, b) = super::boundary::along(side, r);
            if !(a <= t && t <= b) {
                return Err(Shortfall::Short);
            }
            let reader = reader_of(readers, side).ok_or(Shortfall::Undecided)?;
            match read_stretch(
                boxes,
                plane.0,
                reader,
                (side, cut_along(side, r, (t, t))),
                eps,
            ) {
                Reading::Within(_) => Ok(()),
                Reading::Clear => Err(Shortfall::Short),
                Reading::Refused | Reading::Beyond => Err(Shortfall::Undecided),
            }
        }
        Piece::Crossing => match end_reading(boxes, plane, r, q, end, eps) {
            Near::Found => Ok(()),
            Near::Far => Err(Shortfall::Short),
            Near::Unknown => Err(Shortfall::Undecided),
        },
    }
}

/// How the window `r` reads the carrier's end `q` by an arc's standard:
/// [`Near::Found`] where the slice through the end holds a solution
/// ([`holds_zero`]) or one lies within ε of the end ([`near_end`]);
/// [`Near::Far`] where the slice is certified to hold none and every
/// near reading is certified far; else [`Near::Unknown`].
fn end_reading<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    r: UvRect,
    q: (f64, f64),
    end: CarrierEnd,
    eps: f64,
) -> Near {
    match holds_zero(boxes, plane, q, end.e, r) {
        Some(true) => Near::Found,
        reading => match (
            near_end(boxes, plane, (q, end.e, end.tau), r, (end.at, eps)),
            reading,
        ) {
            (Near::Found, _) => Near::Found,
            (Near::Far, Some(false)) => Near::Far,
            _ => Near::Unknown,
        },
    }
}

/// Whether the window `r`'s solutions are certified far from the
/// carrier's end `q` ([`end_reading`]).
fn end_far<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    r: UvRect,
    q: (f64, f64),
    end: CarrierEnd,
    eps: f64,
) -> bool {
    end_reading(boxes, plane, r, q, end, eps) == Near::Far
}

/// **Consecutive windows hold one piece between them**, read on the
/// slice through the knot `p` they share, cut to their overlap.
/// - An arc on either side: a solution there lies on it, and on the
///   other's piece (an arc's, or within ε of a side's stretch, as every
///   solution of the side's window is).
/// - Two stretches of one side: they overlap along it, and the boundary
///   pass reads no piece of the overlap clear ([`read_stretch`]).
fn link<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (domain, readers, eps): (UvRect, &[Option<SectionReader>; 4], f64),
    (a, b): (Piece, Piece),
    overlap: UvRect,
    (p, e): ((f64, f64), (f64, f64)),
) -> Result<(), Shortfall> {
    match (a, b) {
        (Piece::Side(s), Piece::Side(t)) => {
            if s != t || !on_side(overlap, domain, s) {
                return Err(Shortfall::Unlinked);
            }
            let reader = reader_of(readers, s).ok_or(Shortfall::Undecided)?;
            match read_stretch(boxes, plane.0, reader, (s, overlap), eps) {
                Reading::Within(_) => Ok(()),
                Reading::Clear => Err(Shortfall::Unlinked),
                Reading::Refused | Reading::Beyond => Err(Shortfall::Undecided),
            }
        }
        _ => found(holds_zero(boxes, plane, p, e, overlap), Shortfall::Unlinked),
    }
}

/// **The chart chain's solution set is one arc, and it spans the
/// carrier** (module docs).
///
/// - **Hypothesis:** a zero-free `∂φ/∂e⊥` over every window, each
///   clipped to the wall's `domain` as every enclosure over it is.
/// - **Each window:** it holds one piece ([`piece`]).
/// - **Consecutive windows:** their pieces meet ([`link`]).
/// - **The carrier's ends:** the first window's piece reaches the
///   carrier's start, the last window's its end ([`reaches_end`]).
///
/// # Errors
///
/// The first [`Shortfall`] met: windows in order, then links in order,
/// then the start and the end.
pub(crate) fn one_arc<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (domain, band): (UvRect, Band),
    pcurve: &NurbsCurve2<T>,
    windows: &[(ChartWindow, (f64, f64))],
    ends: [Box3; 2],
) -> Result<(), Shortfall> {
    let at = |t: f64| {
        let p = pcurve.span_at(t).eval_in_span(T::from_f64(t));
        (0.5 * (p.x.lo() + p.x.hi()), 0.5 * (p.y.lo() + p.y.hi()))
    };
    let clipped = windows
        .iter()
        .map(|(w, _)| meet(w.rect, domain))
        .collect::<Option<Vec<UvRect>>>()
        .ok_or(Shortfall::Undecided)?;
    let (Some(first), Some(last)) = (windows.first(), windows.last()) else {
        return Err(Shortfall::Undecided);
    };
    let n = windows.len() - 1;
    // The tangent at a span's middle is `e⊥` turned back: `(e.1, −e.0)`.
    let back = |e: (f64, f64), sense: f64| (sense * e.1, -sense * e.0);
    let carrier_ends = [
        (
            0,
            at(first.0.ends.0),
            CarrierEnd {
                e: first.1,
                tau: back(first.1, 1.0),
                at: ends[0],
            },
        ),
        (
            n,
            at(last.0.ends.1),
            CarrierEnd {
                e: last.1,
                tau: back(last.1, -1.0),
                at: ends[1],
            },
        ),
    ];
    let readers = side_readers(boxes, plane);
    let piece_at = |k: usize| piece(boxes, plane, (domain, &readers), clipped[k], band);
    let pieces = (0..clipped.len()).map(piece_at).collect::<Result<Vec<Piece>, Shortfall>>();
    // A window that resolves no piece leaves the chain undecided, unless
    // an end of the carrier is certified unreached by its window: by the
    // side's reading where a side holds the window's piece, and by an
    // arc's where none does. No chain reaches that end, whatever the
    // undecided window holds.
    let unreached = |&(k, q, end): &(usize, (f64, f64), CarrierEnd)| match piece_at(k) {
        Ok(p @ Piece::Side(_)) => {
            reaches_end(boxes, plane, (&readers, band.zero()), (p, clipped[k]), q, end)
                == Err(Shortfall::Short)
        }
        _ => end_far(boxes, plane, clipped[k], q, end, band.zero()),
    };
    let pieces = match pieces {
        Err(Shortfall::Undecided) if carrier_ends.iter().any(unreached) => {
            return Err(Shortfall::Short);
        }
        other => other?,
    };
    for k in 1..windows.len() {
        let overlap = meet(clipped[k - 1], clipped[k]).ok_or(Shortfall::Undecided)?;
        let (w, e) = windows[k - 1];
        link(
            boxes,
            plane,
            (domain, &readers, band.zero()),
            (pieces[k - 1], pieces[k]),
            overlap,
            (at(w.ends.1), e),
        )?;
    }
    for (k, q, end) in carrier_ends {
        reaches_end(
            boxes,
            plane,
            (&readers, band.zero()),
            (pieces[k], clipped[k]),
            q,
            end,
        )?;
    }
    Ok(())
}

/// A box's side along axis `i` (0, 1, 2 for x, y, z).
fn side(b: Box3, i: usize) -> Interval {
    [b.x, b.y, b.z][i]
}

/// `b` with its side along axis `i` replaced.
fn with_side(b: Box3, i: usize, s: Interval) -> Box3 {
    let mut v = [b.x, b.y, b.z];
    v[i] = s;
    Box3 {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

/// The solutions of `f₁ = f₂ = 0` on the boundary of `r`, each in a box
/// that holds exactly one ([`face_walk`] on each of its six faces).
///
/// `None` when a piece resolves no way within [`EXIT_DEPTH`] cuts, or the
/// walk passes [`EXIT_PIECES`].
pub(crate) fn face_roots<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    r: Box3,
) -> Option<Vec<Box3>> {
    let mut roots = Vec::new();
    let mut pieces = 0u32;
    for k in 0..3 {
        for c in [side(r, k).lo(), side(r, k).hi()] {
            face_walk(s1, s2, (with_side(r, k, pt(c)), k), &mut roots, &mut pieces)?;
        }
    }
    Some(roots)
}

/// The solutions on the face `x` held at axis `k`, pushed onto `roots`.
/// Each face piece is treated one of four ways:
/// - dropped where `f₁` or `f₂` is zero-free over it;
/// - kept where the Krawczyk operator of the pair restricted to the face
///   maps it strictly into its own interior (one solution, enclosed by
///   the image);
/// - dropped where the image misses it;
/// - cut otherwise.
///
/// `None` when a piece resolves no way within [`EXIT_DEPTH`] cuts, or
/// `pieces` passes [`EXIT_PIECES`].
fn face_walk<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    (x, k): (Box3, usize),
    roots: &mut Vec<Box3>,
    pieces: &mut u32,
) -> Option<()> {
    let (i, j) = ((k + 1) % 3, (k + 2) % 3);
    let mut stack = vec![(x, 0u32)];
    while let Some((x, depth)) = stack.pop() {
        *pieces += 1;
        if *pieces > EXIT_PIECES {
            return None;
        }
        if sign(implicit_enclosure(s1, x)).is_some() || sign(implicit_enclosure(s2, x)).is_some() {
            continue;
        }
        match krawczyk(s1, s2, x, (i, j)) {
            Some(Krawczyk::One(root)) => {
                roots.push(root);
                continue;
            }
            Some(Krawczyk::None) => continue,
            None => {}
        }
        if depth >= EXIT_DEPTH {
            return None;
        }
        let span = |a: usize| side(x, a).hi() - side(x, a).lo();
        let a = if span(i) >= span(j) { i } else { j };
        let s = side(x, a);
        let cut = s.lo() + EXIT_CUT * (s.hi() - s.lo());
        stack.push((
            with_side(x, a, Interval::from_bounds(cut, s.hi())),
            depth + 1,
        ));
        stack.push((
            with_side(x, a, Interval::from_bounds(s.lo(), cut)),
            depth + 1,
        ));
    }
    Some(())
}

/// What the Krawczyk test proved over a face piece.
enum Krawczyk {
    /// Exactly one solution, inside this box.
    One(Box3),
    /// No solution.
    None,
}

/// The Krawczyk operator of `(f₁, f₂)` over the face piece `x`, in its
/// two free axes `(i, j)`: `K = m − Y·F(m) + (I − Y·J(x))·(x − m)`, with
/// `Y` the inverse of `J`'s midpoint. `Y` only preconditions; any `Y`
/// keeps the test sound.
///
/// - `K` strictly inside `x` proves exactly one solution in `x`, and that
///   it lies in `K`.
/// - `K` missing `x` proves none.
/// - `None` when it proves neither.
fn krawczyk<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    x: Box3,
    (i, j): (usize, usize),
) -> Option<Krawczyk> {
    let (xi, xj) = (side(x, i), side(x, j));
    let (mi, mj) = (0.5 * (xi.lo() + xi.hi()), 0.5 * (xj.lo() + xj.hi()));
    let m = with_side(with_side(x, i, pt(mi)), j, pt(mj));
    let (f1, f2) = (implicit_enclosure(s1, m), implicit_enclosure(s2, m));
    let (g1, g2) = (
        implicit_gradient_enclosure(s1, x),
        implicit_gradient_enclosure(s2, x),
    );
    let jac = [[g1[i], g1[j]], [g2[i], g2[j]]];
    let mid = |v: Interval| 0.5 * (v.lo() + v.hi());
    let (a, b, c, d) = (
        mid(jac[0][0]),
        mid(jac[0][1]),
        mid(jac[1][0]),
        mid(jac[1][1]),
    );
    let det = a * d - b * c;
    if !det.is_finite() || det == 0.0 {
        return None;
    }
    let y = [[d / det, -b / det], [-c / det, a / det]].map(|row| row.map(pt));
    let (di, dj) = (xi - pt(mi), xj - pt(mj));
    let row = |r: usize, m: f64| {
        let yf = y[r][0] * f1 + y[r][1] * f2;
        let ci = pt(if r == 0 { 1.0 } else { 0.0 }) - (y[r][0] * jac[0][0] + y[r][1] * jac[1][0]);
        let cj = pt(if r == 1 { 1.0 } else { 0.0 }) - (y[r][0] * jac[0][1] + y[r][1] * jac[1][1]);
        pt(m) - yf + ci * di + cj * dj
    };
    let (ki, kj) = (row(0, mi), row(1, mj));
    if !(ki.is_certified() && kj.is_certified()) {
        return None;
    }
    let inside = |k: Interval, s: Interval| k.lo() > s.lo() && k.hi() < s.hi();
    let misses = |k: Interval, s: Interval| k.hi() < s.lo() || k.lo() > s.hi();
    if inside(ki, xi) && inside(kj, xj) {
        Some(Krawczyk::One(with_side(with_side(x, i, ki), j, kj)))
    } else if misses(ki, xi) || misses(kj, xj) {
        Some(Krawczyk::None)
    } else {
        None
    }
}

/// **The box `r`'s piece reaches the carrier's end `at`**, `k` the
/// axis the carrier runs most along there.
/// - The slice of `r` through the end across axis `k` holds a solution
///   (Krawczyk on that face).
/// - Or a box about the end of diameter `eps`, cut to `r`, holds one on
///   its boundary: the locus within ε of the carrier's end.
///
/// [`Shortfall::Short`] only where both are certified not so: the
/// slice's walk resolved with no root, and the box about the end is
/// disjoint from `r` or its walk resolved with none. An end whose slice
/// lies outside `r` along `k` (a carrier leaving a search's slab, which
/// a search does not hand over) is [`Shortfall::Undecided`].
fn r3_reaches_end<T: CertifiedBounds>(
    (s1, s2): (&Surface<T>, &Surface<T>),
    r: Box3,
    (at, k): (Box3, usize),
    eps: f64,
) -> Result<(), Shortfall> {
    if ![at.x, at.y, at.z, r.x, r.y, r.z]
        .iter()
        .all(|i| i.is_certified())
    {
        return Err(Shortfall::Undecided);
    }
    let mid = |i: Interval| 0.5 * (i.lo() + i.hi());
    let c = mid(side(at, k));
    if !(side(r, k).lo() <= c && c <= side(r, k).hi()) {
        return Err(Shortfall::Undecided);
    }
    let mut roots = Vec::new();
    let sliced = face_walk(s1, s2, (with_side(r, k, pt(c)), k), &mut roots, &mut 0);
    if !roots.is_empty() {
        return Ok(());
    }
    // A half-width of eps/(2√3) less the end's own width keeps the box
    // and the end inside a ball of diameter eps.
    let h = 0.5 * eps / 3f64.sqrt();
    let about = |i: Interval| {
        let (m, h) = (mid(i), h - 0.5 * (i.hi() - i.lo()));
        (h > 0.0).then(|| Interval::from_bounds(m - h, m + h))
    };
    let ball = (|| {
        Some(Box3 {
            x: about(at.x)?,
            y: about(at.y)?,
            z: about(at.z)?,
        })
    })();
    // Disjoint along some axis: no solution of `r` lies in the ball.
    let apart = |b: Box3| {
        [(b.x, r.x), (b.y, r.y), (b.z, r.z)]
            .iter()
            .any(|(p, q)| p.hi() < q.lo() || q.hi() < p.lo())
    };
    let near = match ball {
        None => Near::Unknown,
        Some(b) if apart(b) => Near::Far,
        Some(b) => match b.intersection(r).and_then(|b| face_roots(s1, s2, b)) {
            Some(v) if !v.is_empty() => Near::Found,
            Some(_) => Near::Far,
            None => Near::Unknown,
        },
    };
    match (near, sliced) {
        (Near::Found, _) => Ok(()),
        (Near::Far, Some(())) => Err(Shortfall::Short),
        _ => Err(Shortfall::Undecided),
    }
}

/// The axis a direction runs most along.
pub(crate) fn dominant_axis(e: [f64; 3]) -> usize {
    let a = e.map(f64::abs);
    if a[0] >= a[1] && a[0] >= a[2] {
        0
    } else if a[1] >= a[2] {
        1
    } else {
        2
    }
}

/// **The ℝ³ chain's solution set is one arc, and it spans the
/// carrier** (module docs).
///
/// - **Hypothesis:** a zero-free `(∇f₁ × ∇f₂)·e` over every box, each
///   cut to the searched `slab` where a search clips to one.
/// - **Each box:** exactly two simple solutions on its boundary
///   ([`face_roots`]).
/// - **Consecutive boxes:** they share their piece where a boundary
///   solution of either lies in the other, which then holds it too.
/// - **The carrier's ends:** the first box's piece reaches the carrier's
///   start, the last box's its end ([`r3_reaches_end`]); `ends` holds
///   each end's point and the axis the carrier runs most along there.
///
/// # Errors
///
/// The first [`Shortfall`] met, boxes in order, then links in order,
/// then the start and the end.
pub(crate) fn one_arc_r3<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    (boxes, slab): (&[Box3], Option<Box3>),
    ends: [(Box3, usize); 2],
    eps: f64,
) -> Result<(), Shortfall> {
    let cut = boxes
        .iter()
        .map(|b| slab.map_or(Some(*b), |s| b.intersection(s)))
        .collect::<Option<Vec<Box3>>>()
        .ok_or(Shortfall::Undecided)?;
    let mut roots = Vec::with_capacity(cut.len());
    for r in &cut {
        match face_roots(s1, s2, *r) {
            Some(v) if v.len() == 2 => roots.push(v),
            #[allow(clippy::cast_possible_truncation)]
            Some(v) => return Err(Shortfall::Count(v.len() as u32)),
            None => return Err(Shortfall::Undecided),
        }
    }
    (1..cut.len()).try_for_each(|k| {
        let shared = roots[k - 1].iter().any(|q| q.contained_in(cut[k]))
            || roots[k].iter().any(|q| q.contained_in(cut[k - 1]));
        if shared {
            Ok(())
        } else {
            Err(Shortfall::Unlinked)
        }
    })?;
    let (Some(&first), Some(&last)) = (cut.first(), cut.last()) else {
        return Err(Shortfall::Undecided);
    };
    r3_reaches_end((s1, s2), first, ends[0], eps)?;
    r3_reaches_end((s1, s2), last, ends[1], eps)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! Each row here is the one a removed or loosened arm of the proof
    //! turns red; the arm it guards is named in its doc.

    use geom::{NurbsCurve2, NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Interval, Point2, Point3, Vec3};

    use super::super::ChartAxis;
    use super::super::certify::chart_tube_windows;
    use super::super::enclose::{Box3, NurbsBoxes};
    use super::super::exhaust::UvRect;
    use super::{
        Krawczyk, Shortfall, boundary_zeros, edge_runs, face_roots, krawczyk, one_arc, one_arc_r3,
        pt,
    };

    /// A biquadratic graph `z = g(x) + h(y)` over `[0, 1]²` from the
    /// Bernstein coefficients of `g` and `h`, with `(u, v) = (x, y)`.
    fn biq(g: [f64; 3], h: [f64; 3]) -> NurbsSurface<f64> {
        let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = (0..9)
            .map(|i| {
                Point3::new(
                    0.5 * f64::from(i / 3),
                    0.5 * f64::from(i % 3),
                    g[(i / 3) as usize] + h[(i % 3) as usize],
                )
            })
            .collect();
        NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
    }

    /// The plane `z = 0`, crossed into certification arithmetic.
    fn ground() -> ([Interval; 3], [Interval; 3]) {
        (
            [0.0, 0.0, 1.0].map(Interval::from_certified),
            [0.0, 0.0, 0.0].map(Interval::from_certified),
        )
    }

    fn rect(u: (f64, f64), v: (f64, f64)) -> UvRect {
        UvRect { u, v }
    }

    /// The reviewer's fold (`ssi_limb3_one_arc.rs`), at ε = 1e-9, over
    /// its own `[0, 1]²` chart: `u` runs `x ∈ [−1.5w, 1.8w]`, `v` runs
    /// `y ∈ [0, L]`. Two arcs, each from a `v` side to the low `u` side;
    /// `u0` is the chart's `x = 0`.
    fn fold() -> (NurbsSurface<f64>, f64) {
        let beta = 1e-9;
        let c = 80.0 * beta;
        let a = 0.28 * c * c / beta;
        let w = beta / c;
        let (x0, x1) = (-1.5 * w, 1.8 * w);
        let g = |x: f64| c * x + a * x * x;
        let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0), g(x1)];
        let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.6 * w, 1.2 * w]);
        let control = (0..9)
            .map(|i| {
                Point3::new(
                    xs[i / 3],
                    ys[i % 3],
                    gb[i / 3] + [0.0, 2.0 * beta, 0.0][i % 3],
                )
            })
            .collect();
        let wall = NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap();
        (wall, -x0 / (x1 - x0))
    }

    /// The straight pcurve `u = u0` across the fold's gap, in `spans`
    /// equal spans.
    fn across(u0: f64, spans: u32) -> NurbsCurve2<f64> {
        let mut knots = vec![0.0];
        knots.extend((0..=spans).map(|k| f64::from(k) / f64::from(spans)));
        knots.push(1.0);
        let pts = (0..=spans)
            .map(|k| Point2::new(u0, f64::from(k) / f64::from(spans)))
            .collect();
        NurbsCurve2::new(
            KnotVector::clamped(knots, 1).unwrap(),
            pts,
            vec![1.0; spans as usize + 1],
        )
        .unwrap()
    }

    /// The fold's run band, at its ε.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// The wall's points at a pcurve's two ends, the carrier's ends.
    fn ends_on(wall: &NurbsSurface<f64>, pc: &NurbsCurve2<f64>) -> [Box3; 2] {
        let (t0, t1) = pc.domain();
        [t0, t1].map(|t| {
            let q = pc.eval(t);
            let p = wall.eval(q.x, q.y);
            bx((p.x, p.x), (p.y, p.y), (p.z, p.z))
        })
    }

    /// The probe's windows with their `e⊥`, for a straight pcurve along
    /// `v`: `e⊥ = (−1, 0)`.
    fn windows(pc: &NurbsCurve2<f64>, pad: (f64, f64)) -> Vec<(super::ChartWindow, (f64, f64))> {
        chart_tube_windows(pc, pad)
            .unwrap()
            .into_iter()
            .map(|w| (w, (-1.0, 0.0)))
            .collect()
    }

    /// **The count is two, not "some" or "at least two".** The fold's
    /// whole wall holds both arcs; its boundary holds their four ends,
    /// and a window over it is refused by its count. Red under
    /// `boundary_zeros(..) == Some(2)` loosened to `.is_some()` or
    /// `>= Some(2)`: the lone window then reaches its ends, and the
    /// refusal changes cause.
    #[test]
    fn a_window_holding_two_arcs_is_refused_by_its_count_of_four() {
        let (wall, u0) = fold();
        let boxes = NurbsBoxes::new(&wall);
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        assert_eq!(boundary_zeros(&boxes, ground(), whole), Some(4));
        let pc = across(u0, 1);
        let found = one_arc(
            &boxes,
            ground(),
            (whole, band()),
            &pc,
            &windows(&pc, (3.0, 8.0)),
            ends_on(&wall, &pc),
        );
        assert_eq!(found, Err(Shortfall::Count(4)));
    }

    /// **Consecutive windows must share their piece.** Cut the same
    /// pcurve into two spans with a pad across the gap that keeps each
    /// window on one arc: the lower window holds one arc's ends, the
    /// upper the other's, two each, and no zero joins them across the
    /// gap. Red under the chart link forced true.
    #[test]
    fn two_windows_on_two_arcs_do_not_link() {
        let (wall, u0) = fold();
        let boxes = NurbsBoxes::new(&wall);
        let pc = across(u0, 2);
        let ws = windows(&pc, (3.0, 0.1));
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        for (w, _) in &ws {
            let r = super::meet(w.rect, whole).unwrap();
            assert_eq!(
                boundary_zeros(&boxes, ground(), r),
                Some(2),
                "FIXTURE: window {r:?} holds one arc's two ends"
            );
        }
        assert_eq!(
            one_arc(
                &boxes,
                ground(),
                (whole, band()),
                &pc,
                &ws,
                ends_on(&wall, &pc)
            ),
            Err(Shortfall::Unlinked)
        );
    }

    /// **A touch at a corner is no crossing.** `z = 0.5 − x + y` meets
    /// `z = 0` in a line through the corner `(0.5, 0)` of `[0, 0.5]²`,
    /// and `φ ≥ 0` on both edges there: the corner's runs have agreeing
    /// outer signs, which could hold none or two. Red under that arm
    /// counted as none, and under an unsigned run end read as signed.
    #[test]
    fn a_touch_at_a_corner_resolves_no_count() {
        let wall = biq([0.5, 0.0, -0.5], [0.0, 0.5, 1.0]);
        let boxes = NurbsBoxes::new(&wall);
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 0.5), (0.0, 0.5))),
            None
        );
        // The control: the same line crossing a box's sides is two.
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.25, 0.75), (0.0, 0.5))),
            Some(2)
        );
    }

    /// **A touch inside an edge is no crossing.** `z = (x − ½)² + y − 0.2`
    /// touches the edge `y = 0.2` of `[0, 1] × [0.2, 1]` at `x = ½` from
    /// outside. No piece there is zero-free or monotone. Red under a run
    /// marked monotone with no slope proof.
    #[test]
    fn a_touch_inside_an_edge_resolves_no_count() {
        let wall = biq([0.25, -0.25, 0.25], [-0.2, 0.3, 0.8]);
        let boxes = NurbsBoxes::new(&wall);
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.2, 1.0))),
            None
        );
        // The touched edge itself refuses, rather than walking past it.
        assert!(
            edge_runs(&boxes, ground(), (true, 0.2, (0.0, 1.0)), &mut 0).is_none(),
            "the touched edge resolved"
        );
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.1, 1.0))),
            Some(2),
            "the control: the edge below cuts the parabola twice"
        );
    }

    fn bx(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Box3 {
        Box3 {
            x: Interval::from_bounds(x.0, x.1),
            y: Interval::from_bounds(y.0, y.1),
            z: Interval::from_bounds(z.0, z.1),
        }
    }

    fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
        let u = if normal.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let u = (u - normal * u.dot(normal)) / (u - normal * u.dot(normal)).norm();
        Surface::Plane {
            origin,
            normal,
            u_ref: u,
        }
    }

    /// **A face graze is no crossing.** The unit sphere meets `z = 0` in
    /// the unit circle, which touches the face `x = 1` of
    /// `[0.5, 1] × [−0.3, 0.3] × [−0.1, 0.1]` at `(1, 0, 0)`; Krawczyk
    /// cannot isolate a tangential root. Red under the face walk's depth
    /// cut dropping its piece.
    #[test]
    fn a_face_graze_resolves_no_count() {
        let sphere = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ground = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let graze = bx((0.5, 1.0), (-0.3, 0.3), (-0.1, 0.1));
        assert_eq!(face_roots(&sphere, &ground, graze).map(|v| v.len()), None);
        let cut = bx((0.5, 0.9), (0.3, 0.9), (-0.1, 0.1));
        assert_eq!(
            face_roots(&sphere, &ground, cut).map(|v| v.len()),
            Some(2),
            "the control: the circle crosses two faces"
        );
    }

    /// **A solution on a box edge is counted on neither face.** The line
    /// `y = ½, x − z = 1` meets `[0, 1]³` only at `(1, ½, 0)`, on the edge
    /// where the faces `x = 1` and `z = 0` meet. Each face's Krawczyk
    /// image straddles that edge, never inside a piece, so the walk
    /// resolves no count. Red under the face walk's depth cut dropping
    /// its piece, which would count none.
    #[test]
    fn a_solution_on_a_box_edge_is_counted_on_neither_face() {
        let p1 = plane(Point3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let n = Vec3::new(1.0, 0.0, -1.0);
        let p2 = plane(Point3::new(1.0, 0.0, 0.0), n / n.norm());
        let unit = bx((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
        assert_eq!(face_roots(&p1, &p2, unit).map(|v| v.len()), None);
    }

    /// **Consecutive ℝ³ boxes must share their piece.** The circle of
    /// radius 1 at `z = √8` where a unit cylinder meets a radius-3 sphere,
    /// cut by the slab `x ≤ cos 5°, y ≤ sin 11°` into a long arc and a
    /// short one (5° to 11°). Two overlapping boxes each hold one arc's
    /// piece, two boundary solutions each, and no solution of either lies
    /// in the other. Red under the ℝ³ link forced true.
    #[test]
    fn two_boxes_on_two_arcs_do_not_link() {
        let cylinder = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let sphere = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 3.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let z0 = 8.0f64.sqrt();
        let slab = bx(
            (-3.0, 5.0f64.to_radians().cos()),
            (-3.0, 11.0f64.to_radians().sin()),
            (z0 - 1.0, z0 + 1.0),
        );
        let long = bx((0.9, 1.1), (-0.25, -0.05), (z0 - 0.1, z0 + 0.1));
        let short = bx((0.9, 1.1), (-0.06, 0.3), (z0 - 0.1, z0 + 0.1));
        for b in [long, short] {
            assert_eq!(
                face_roots(&cylinder, &sphere, b.intersection(slab).unwrap()).map(|v| v.len()),
                Some(2),
                "FIXTURE: each box holds one arc's piece"
            );
        }
        assert_eq!(
            one_arc_r3(
                &cylinder,
                &sphere,
                (&[long, short], Some(slab)),
                [(long, 1), (short, 1)],
                1e-9
            ),
            Err(Shortfall::Unlinked)
        );
    }

    /// **Krawczyk accepts only an image strictly inside its piece.** The
    /// planes `x = ½` and `y = ¼` meet in a line along `z`; over the face
    /// piece `[½, 1] × [0, 1] × {0}` every reading is exact, and the image
    /// is the point `(½, ¼)` on the piece's edge. Closed containment
    /// proves a solution exists, not that it is the piece's alone: the
    /// neighbouring piece would claim it too. Red under a non-strict
    /// acceptance.
    #[test]
    fn krawczyk_does_not_accept_an_image_on_its_pieces_edge() {
        let p1 = plane(Point3::new(0.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let p2 = plane(Point3::new(0.0, 0.25, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let piece = bx((0.5, 1.0), (0.0, 1.0), (0.0, 0.0));
        assert!(
            krawczyk(&p1, &p2, piece, (0, 1)).is_none(),
            "an image on the piece's edge was accepted"
        );
        let inside = bx((0.25, 1.0), (0.0, 1.0), (0.0, 0.0));
        assert!(
            matches!(krawczyk(&p1, &p2, inside, (0, 1)), Some(Krawczyk::One(_))),
            "the control: a piece holding the solution inside accepts it"
        );
    }

    /// A wall of degree 1 in `u` with a knot at `u = ½`, so `C0` across
    /// it, and biquadratic `y`: `z = g + h` from the Bernstein ordinates
    /// of each.
    fn kinked(g: [f64; 3], h: [f64; 3]) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = (0..9)
            .map(|i| {
                Point3::new(
                    0.5 * f64::from(i / 3),
                    0.5 * f64::from(i % 3),
                    g[(i / 3) as usize] + h[(i % 3) as usize],
                )
            })
            .collect();
        NurbsSurface::new(ku, kv, control, vec![1.0; 9]).unwrap()
    }

    /// **A kink that dips through an edge hides two crossings.** On a
    /// `C0` wall, `z = 0.1·|x − ½| − δ − (y − 0.2)` meets the edge
    /// `y = 0.2` of `[0, 1] × [0.2, 1]` twice, `20δ` apart about the kink.
    /// At δ = 1e-14 the piece holding both is unresolved at the depth
    /// cut, well under the piece cap since the kink's slopes are ±0.1,
    /// and the walk refuses. Red under the edge walk's depth cut
    /// dropping its piece, which counts the boundary's other two zeros
    /// as one arc's ends. (The verifier's fixture.)
    #[test]
    fn a_kink_dipping_through_an_edge_resolves_no_count() {
        let delta = 1e-14;
        let wall = kinked([0.05 - delta, -delta, 0.05 - delta], [0.2, -0.3, -0.8]);
        let boxes = NurbsBoxes::new(&wall);
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.2, 1.0))),
            None
        );
    }

    /// **A kink touching an edge is no crossing.** The same `C0` wall
    /// with `z = 0.1·|x − ½| + (y − 0.2)(0.6 − y)` touches the edge
    /// `y = 0.2` at the kink from above. Red under the edge walk's depth
    /// cut dropping its piece. (The verifier's fixture.)
    #[test]
    fn a_kink_touching_an_edge_resolves_no_count() {
        let wall = kinked([0.05, 0.0, 0.05], [-0.12, 0.28, -0.32]);
        let boxes = NurbsBoxes::new(&wall);
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.1, 1.0))),
            Some(4),
            "the control: the edge below cuts the V twice, the arc twice"
        );
        assert_eq!(
            boundary_zeros(&boxes, ground(), rect((0.0, 1.0), (0.2, 1.0))),
            None
        );
    }

    /// A wall `z = k·x + h(y)` over `[0, 1]²`, `(u, v) = (x, y)`, linear
    /// in `u` and in `v` a quadratic B-spline on the knots
    /// `0, 0, 0, ½, ¾, 1, 1, 1` with coefficients `h` (its Greville
    /// abscissae carry `y = v`).
    fn knotted(k: f64, h: [f64; 5]) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.75, 1.0, 1.0, 1.0], 2).unwrap();
        let ys = [0.0, 0.25, 0.625, 0.875, 1.0];
        let control = (0..10)
            .map(|i| {
                let x = f64::from(i / 5);
                Point3::new(x, ys[(i % 5) as usize], k * x + h[(i % 5) as usize])
            })
            .collect();
        NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap()
    }

    /// A polyline pcurve through `pts`, one span per segment.
    fn polyline(pts: &[(f64, f64)]) -> NurbsCurve2<f64> {
        let n = pts.len() - 1;
        let mut knots = vec![0.0];
        knots.extend((0..=n).map(|k| k as f64 / n as f64));
        knots.push(1.0);
        NurbsCurve2::new(
            KnotVector::clamped(knots, 1).unwrap(),
            pts.iter().map(|&(u, v)| Point2::new(u, v)).collect(),
            vec![1.0; n + 1],
        )
        .unwrap()
    }

    /// Each span's window at `pad` with its own `e⊥`.
    fn spans(
        pc: &NurbsCurve2<f64>,
        pts: &[(f64, f64)],
        pad: f64,
    ) -> Vec<(super::ChartWindow, (f64, f64))> {
        chart_tube_windows(pc, (pad, pad))
            .unwrap()
            .into_iter()
            .zip(pts.windows(2))
            .map(|(w, p)| {
                let (tx, ty) = (p[1].0 - p[0].0, p[1].1 - p[0].1);
                let n = tx.hypot(ty);
                (w, (-ty / n, tx / n))
            })
            .collect()
    }

    /// A coarse run band, so a unit-scale wall reads a side within it.
    fn coarse() -> Band {
        Band::new(1e-3, 1e-2).unwrap()
    }

    /// **A window on a side within ε of the plane holds the side's piece.**
    /// `z = x` meets `z = 0` in the side `u = 0` itself: along it `φ` is
    /// zero, so no boundary zero is simple, and the walk resolves no
    /// count. The side cover holds: every zero of a window on that side
    /// lies on it. Red under the side-cover arm removed (the walk's
    /// `Undecided` then speaks).
    #[test]
    fn a_window_on_a_side_within_eps_of_the_plane_holds_the_sides_piece() {
        let wall = knotted(1.0, [0.0; 5]);
        let boxes = NurbsBoxes::new(&wall);
        let pts = [(0.0, 0.0), (0.0, 0.5), (0.0, 1.0)];
        let pc = polyline(&pts);
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        let ws = spans(&pc, &pts, 0.01);
        for (w, _) in &ws {
            let r = super::meet(w.rect, whole).unwrap();
            assert_eq!(
                boundary_zeros(&boxes, ground(), r),
                None,
                "FIXTURE: window {r:?}'s boundary walk resolves no count"
            );
        }
        assert_eq!(
            one_arc(
                &boxes,
                ground(),
                (whole, coarse()),
                &pc,
                &ws,
                ends_on(&wall, &pc)
            ),
            Ok(())
        );
    }

    /// The locus `x = −h(y)` of [`knotted`] at `k = 1` runs along the side
    /// `u = 0` for `y ≤ ½` and, for the `h` given, leaves it: the pcurve
    /// along that side to its knot at `y = 0.55`, then to `(0.3, 1)`
    /// where `h(1) = −0.3` puts the locus. The lower window is the
    /// side's (its stretch within ε, ε = 10⁻³), the upper a crossing
    /// window.
    fn side_then_crossing(h: [f64; 5]) -> Result<(), Shortfall> {
        let wall = knotted(1.0, h);
        let boxes = NurbsBoxes::new(&wall);
        let pts = [(0.0, 0.0), (0.0, 0.55), (0.3, 1.0)];
        let pc = polyline(&pts);
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        let ws = spans(&pc, &pts, 0.01);
        let r: Vec<UvRect> = ws
            .iter()
            .map(|(w, _)| super::meet(w.rect, whole).unwrap())
            .collect();
        let readers = super::side_readers(&boxes, ground());
        if h[3] < 0.0 {
            assert_eq!(
                super::piece(&boxes, ground(), (whole, &readers), r[0], coarse()),
                Ok(super::Piece::Side(super::SIDES[0])),
                "FIXTURE: the lower window is the side's"
            );
            assert_eq!(
                super::piece(&boxes, ground(), (whole, &readers), r[1], coarse()),
                Ok(super::Piece::Crossing),
                "FIXTURE: the upper window is a crossing window"
            );
        }
        one_arc(
            &boxes,
            ground(),
            (whole, coarse()),
            &pc,
            &ws,
            ends_on(&wall, &pc),
        )
    }

    /// **A side's window and a crossing window link through a solution
    /// they share.** With `h ≤ 0` the locus leaves the side inward and
    /// runs on to `(0.3, 1)`: one arc, and the slice through the shared
    /// knot holds a zero in the overlap. Where `h` rises above zero past
    /// `y = ½` the locus leaves the domain there and comes back through
    /// the side near `y = 0.8`: the lower window's stretch reads clear
    /// past `y = ½`, so it holds no side's piece, and the chain refuses.
    #[test]
    fn a_sides_window_and_a_crossing_window_link_through_a_shared_solution() {
        assert_eq!(side_then_crossing([0.0, 0.0, 0.0, -0.01, -0.3]), Ok(()));
        assert!(side_then_crossing([0.0, 0.0, 0.0, 0.01, -0.3]).is_err());
    }

    /// **A link or an end that the boundary pass reads clear does not
    /// hold.** `z = x + ½ε`, ε = 10⁻³: along the side `u = 0` the plane
    /// distance is `½ε > 0`, within the band, and the wall rises away from
    /// the plane inward, so the side is clear of it (the exact empty
    /// answer), and no slice holds a zero. A side's stretch linked to an arc, two stretches of the side
    /// linked, two stretches of different sides linked, and a side's
    /// piece reaching an end on the side or off it, all refuse. Red under
    /// the side-to-arc link forced, the side-to-side link forced, and the
    /// side's end check forced.
    #[test]
    fn a_link_or_an_end_the_boundary_pass_reads_clear_does_not_hold() {
        let wall = knotted(1.0, [5e-4; 5]);
        let boxes = NurbsBoxes::new(&wall);
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        let readers = super::side_readers(&boxes, ground());
        let (u_low, v_low) = (super::SIDES[0], super::SIDES[2]);
        assert_eq!((u_low.fixed, v_low.fixed), (ChartAxis::U, ChartAxis::V));
        let overlap = rect((0.0, 0.1), (0.4, 0.6));
        let (p, e) = ((0.05, 0.5), (1.0, 0.0));
        let link = |a, b| {
            super::link(
                &boxes,
                ground(),
                (whole, &readers, 1e-3),
                (a, b),
                overlap,
                (p, e),
            )
        };
        let (side, other, arc) = (
            super::Piece::Side(u_low),
            super::Piece::Side(v_low),
            super::Piece::Crossing,
        );
        assert_eq!(link(side, arc), Err(Shortfall::Unlinked));
        assert_eq!(link(arc, side), Err(Shortfall::Unlinked));
        assert_eq!(link(side, side), Err(Shortfall::Unlinked));
        assert_eq!(link(side, other), Err(Shortfall::Unlinked));
        let end = |q: (f64, f64)| {
            super::reaches_end(
                &boxes,
                ground(),
                (&readers, 1e-3),
                (side, overlap),
                q,
                super::CarrierEnd {
                    e: (1.0, 0.0),
                    tau: (0.0, -1.0),
                    at: Box3 {
                        x: pt(0.0),
                        y: pt(q.1),
                        z: pt(0.0),
                    },
                },
            )
        };
        assert_eq!(
            end((0.05, 0.5)),
            Err(Shortfall::Short),
            "an end on the side"
        );
        let flush = knotted(1.0, [0.0; 5]);
        let fb = NurbsBoxes::new(&flush);
        let fr = super::side_readers(&fb, ground());
        assert_eq!(
            super::link(
                &fb,
                ground(),
                (whole, &fr, 1e-3),
                (side, side),
                overlap,
                (p, e)
            ),
            Ok(()),
            "the control: a flush side's stretches link"
        );
        assert_eq!(
            super::reaches_end(
                &fb,
                ground(),
                (&fr, 1e-3),
                (side, overlap),
                (0.05, 0.5),
                super::CarrierEnd {
                    e: (1.0, 0.0),
                    tau: (0.0, -1.0),
                    at: Box3 {
                        x: pt(0.0),
                        y: pt(0.5),
                        z: pt(0.0),
                    },
                },
            ),
            Ok(()),
            "the control: a flush side's piece reaches an end on it"
        );
        assert_eq!(
            super::reaches_end(
                &fb,
                ground(),
                (&fr, 1e-3),
                (side, overlap),
                (0.0, 0.7),
                super::CarrierEnd {
                    e: (1.0, 0.0),
                    tau: (0.0, -1.0),
                    at: Box3 {
                        x: pt(0.0),
                        y: pt(0.7),
                        z: pt(0.0),
                    },
                },
            ),
            Err(Shortfall::Short),
            "an end past the window's stretch of the side"
        );
    }

    /// **The pieces must reach both ends of the carrier.** `z = x − 0.2 −
    /// 0.6y` meets `z = 0` in the line from `(0.2, 0)` to `(0.8, 1)`;
    /// the pcurve `u = 0.2` follows it from `v = 0` and runs on to
    /// `v = 1` after the line has left its window through the side
    /// `u = 0.45`. The window holds one arc, two boundary zeros, but the
    /// slice through the overrun end holds none and no solution lies
    /// within ε of it. Declared both ways, the overrun end is the last,
    /// then the first: red under either end's check removed.
    #[test]
    fn a_piece_short_of_either_end_of_the_carrier_refuses() {
        let wall = biq([-0.2, 0.3, 0.8], [0.0, -0.3, -0.6]);
        let boxes = NurbsBoxes::new(&wall);
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        for pts in [[(0.2, 0.0), (0.2, 1.0)], [(0.2, 1.0), (0.2, 0.0)]] {
            let pc = polyline(&pts);
            let ws = spans(&pc, &pts, 0.25);
            let r = super::meet(ws[0].0.rect, whole).unwrap();
            assert_eq!(
                boundary_zeros(&boxes, ground(), r),
                Some(2),
                "FIXTURE: the window holds one arc"
            );
            assert_eq!(
                one_arc(
                    &boxes,
                    ground(),
                    (whole, band()),
                    &pc,
                    &ws,
                    ends_on(&wall, &pc)
                ),
                Err(Shortfall::Short),
                "the pcurve {pts:?}"
            );
        }
        // The control: a carrier that stops on the locus.
        let pts = [(0.2, 0.0), (0.45, 5.0 / 12.0)];
        let pc = polyline(&pts);
        let ws = spans(&pc, &pts, 0.25);
        assert_eq!(
            one_arc(
                &boxes,
                ground(),
                (whole, band()),
                &pc,
                &ws,
                ends_on(&wall, &pc)
            ),
            Ok(())
        );
    }

    /// **In ℝ³ too, the piece must reach both ends of the carrier.** The
    /// planes `y = ½x` and `z = 0` meet in a line that enters the box
    /// `[0.1, 1] × [0, 0.3] × [−0.1, 0.1]` through its face `x = 0.1` and
    /// leaves through `y = 0.3` at `x = 0.6`. A carrier from the entry
    /// to `x = 0.9`, its end's slice `x = 0.9` holding no solution in the
    /// box, refuses, either way round: red under either end's check
    /// removed.
    #[test]
    fn an_r3_piece_short_of_either_end_of_the_carrier_refuses() {
        let n = Vec3::new(-0.5, 1.0, 0.0);
        let p1 = plane(Point3::new(0.0, 0.0, 0.0), n / n.norm());
        let p2 = plane(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let b = bx((0.1, 1.0), (0.0, 0.3), (-0.1, 0.1));
        assert_eq!(
            face_roots(&p1, &p2, b).map(|v| v.len()),
            Some(2),
            "FIXTURE: the box holds one piece"
        );
        let at = |x: f64, y: f64| (bx((x, x), (y, y), (0.0, 0.0)), 0);
        let (entry, overrun) = (at(0.1, 0.05), at(0.9, 0.05));
        for ends in [[entry, overrun], [overrun, entry]] {
            assert_eq!(
                one_arc_r3(&p1, &p2, (&[b], None), ends, 1e-9),
                Err(Shortfall::Short)
            );
        }
        assert_eq!(
            one_arc_r3(&p1, &p2, (&[b], None), [entry, at(0.5, 0.25)], 1e-9),
            Ok(()),
            "the control: a carrier stopping on the locus"
        );
    }

    /// **A side within the band holds the side's piece only where its
    /// cover is within ε.** `z = 10⁻³x + h(y)` with `h` zero at both ends
    /// of the side `u = 0` and up to `3·10⁻⁴` below between: no piece of
    /// the side reads clear and `|φ| ≤ ε` along it (ε = 10⁻³), but the
    /// locus runs `|h|/10⁻³` deep, from the side into the wall and back.
    /// The side's cover reaches past ε, so the window is no side's, and
    /// its walk refuses. Red under limb 3 taking a side within the band
    /// as its piece without the cover's verdict.
    #[test]
    fn a_side_within_the_band_whose_cover_reaches_past_eps_holds_no_piece() {
        let b = 1e-4;
        let wall = knotted(1e-3, [0.0, -2.0 * b, -b, -3.0 * b, 0.0]);
        let boxes = NurbsBoxes::new(&wall);
        let whole = rect((0.0, 1.0), (0.0, 1.0));
        let pts = [(0.1, 0.0), (0.1, 1.0)];
        let pc = polyline(&pts);
        let ws = spans(&pc, &pts, 0.2);
        let r = super::meet(ws[0].0.rect, whole).unwrap();
        let side = super::SIDES[0];
        let readers = super::side_readers(&boxes, ground());
        let reader = readers[0].as_ref().unwrap();
        assert!(
            matches!(
                super::read_stretch(&boxes, ground().0, reader, (side, r), coarse().zero()),
                super::Reading::Within(_)
            ),
            "FIXTURE: the side lies within the band of the plane, no piece of it clear"
        );
        assert_eq!(
            super::side_piece(&boxes, ground(), (whole, &readers), r, coarse()),
            None
        );
        assert!(
            one_arc(
                &boxes,
                ground(),
                (whole, coarse()),
                &pc,
                &ws,
                ends_on(&wall, &pc)
            )
            .is_err(),
            "the window holding two arcs certified"
        );
    }
}

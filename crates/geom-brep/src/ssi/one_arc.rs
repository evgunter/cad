//! **Limb 3's one-arc proof**: a tube's chain holds the traced arc and
//! nothing else (C2).
//!
//! Limb 3's enclosure, zero-free over a box, makes the solution set in
//! the box a graph over the slices transverse to the carrier: at most one
//! solution on each. That is less than one arc. A second arc beside the
//! first along the carrier, leaving through the box's sides, is a graph
//! too, and a search banks every cell inside a tube as accounted
//! ([`super::exhaust`]). This module proves the rest.
//!
//! **The argument.** Let `R` be a box cut to the region the search covers
//! (the wall's knot rectangle, the ℝ³ slab); it is convex. Each
//! connected piece of the solution set in `R` is a graph over the
//! slices, so it ends on `∂R` at two distinct points. It cannot end
//! inside `R`, where the zero set continues by the implicit function
//! theorem, and it cannot close up. A boundary holding exactly two
//! solutions, each a simple crossing of `∂R`, therefore holds the ends of
//! exactly one piece. A tangential touch of `∂R` is never counted as a
//! crossing: on an edge it fails the monotone test, at a corner the
//! joint rule refuses it, and on a face Krawczyk cannot isolate it.
//! Consecutive boxes whose pieces share a solution hold one connected
//! arc between them, and every solution in the chain lies on it.
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
use geom_core::{Bounds, CertifiedBounds, Interval, Real};

use super::certify::ChartWindow;
use super::enclose::{Box3, NurbsBoxes, implicit_enclosure, implicit_gradient_enclosure};
use super::exhaust::UvRect;
use super::section::sign;

/// The deepest a walk cuts one edge or face piece.
pub(crate) const EXIT_DEPTH: u32 = 40;

/// Where a walk cuts a piece, as a fraction of it from its low end. The
/// value is a choice, not a derivation: anything off the middle, because
/// a box is centred on the carrier, so the arc crosses its boundary at
/// the middle of a side as often as not, and a solution on a cut is one
/// no piece can isolate.
pub(crate) const EXIT_CUT: f64 = 0.5 - 1.0 / 128.0;

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
    /// A walk resolved no count, or a linking reading no sign.
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
pub(crate) fn meet(a: UvRect, b: UvRect) -> Option<UvRect> {
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
        // The derivative over a strip as wide across the edge as the
        // piece is long: a net is cut only to a rectangle of positive
        // width, so a strip of none would read the whole span cell's.
        let w = t - s;
        let (r0, r1, r2, r3) = if along_u {
            (s, t, c - w, c + w)
        } else {
            (c - w, c + w, s, t)
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

/// Whether `φ` changes sign between the two ends of the line through
/// `p` along `e`, cut to `rect`. `Some(true)` puts a zero on the
/// segment between them, which lies in `rect` (convex). `Some(false)`:
/// both ends are certified of one sign. `None`: an end has no certified
/// sign, or the line misses the interior of `rect`.
pub(crate) fn holds_zero<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    p: (f64, f64),
    e: (f64, f64),
    rect: UvRect,
) -> Option<bool> {
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
    // Clamped into the rectangle, so the segment between them is in it.
    let at = |t: f64| {
        let (u, v) = (
            (p.0 + t * e.0).clamp(rect.u.0, rect.u.1),
            (p.1 + t * e.1).clamp(rect.v.0, rect.v.1),
        );
        sign(phi_over(boxes, plane, (u, u, v, v)))
    };
    Some(at(lo)? != at(hi)?)
}

/// **The chart chain's solution set is the one arc** (module docs).
///
/// - **Hypothesis:** a zero-free `∂φ/∂e⊥` over every window, each
///   clipped to the wall's `domain` as every enclosure over it is.
/// - **Each window:** its boundary holds exactly two simple zeros
///   ([`boundary_zeros`]).
/// - **Consecutive windows:** they share a zero inside their overlap, on
///   the line along the first window's `e⊥` through the knot they
///   share.
/// - **A lone window:** it holds a zero on that line through its span's
///   middle.
///
/// # Errors
///
/// The first [`Shortfall`] met, windows in order.
pub(crate) fn one_arc<T: CertifiedBounds + Bounds + Real>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    domain: UvRect,
    pcurve: &NurbsCurve2<T>,
    windows: &[(ChartWindow, (f64, f64))],
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
    for r in &clipped {
        match boundary_zeros(boxes, plane, *r) {
            Some(2) => {}
            Some(n) => return Err(Shortfall::Count(n)),
            None => return Err(Shortfall::Undecided),
        }
    }
    let linked = |found: Option<bool>| match found {
        Some(true) => Ok(()),
        Some(false) => Err(Shortfall::Unlinked),
        None => Err(Shortfall::Undecided),
    };
    match windows {
        [(w, e)] => linked(holds_zero(boxes, plane, at(w.mid), *e, clipped[0])),
        _ => windows
            .windows(2)
            .zip(clipped.windows(2))
            .try_for_each(|(w, r)| {
                let overlap = meet(r[0], r[1]).ok_or(Shortfall::Undecided)?;
                linked(holds_zero(boxes, plane, at(w[0].0.ends.1), w[0].1, overlap))
            }),
    }
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
/// that holds exactly one.
///
/// Each face piece is treated one of four ways:
/// - dropped where `f₁` or `f₂` is zero-free over it;
/// - kept where the Krawczyk operator of the pair restricted to the face
///   maps it strictly into its own interior (one solution, enclosed by
///   the image);
/// - dropped where the image misses it;
/// - cut otherwise.
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
        let (i, j) = ((k + 1) % 3, (k + 2) % 3);
        for c in [side(r, k).lo(), side(r, k).hi()] {
            let mut stack = vec![(with_side(r, k, pt(c)), 0u32)];
            while let Some((x, depth)) = stack.pop() {
                pieces += 1;
                if pieces > EXIT_PIECES {
                    return None;
                }
                if sign(implicit_enclosure(s1, x)).is_some()
                    || sign(implicit_enclosure(s2, x)).is_some()
                {
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
        }
    }
    Some(roots)
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

/// **The ℝ³ chain's solution set is the one arc** (module docs).
///
/// - **Hypothesis:** a zero-free `(∇f₁ × ∇f₂)·e` over every box, each
///   cut to the searched `slab`.
/// - **Each box:** exactly two simple solutions on its boundary
///   ([`face_roots`]).
/// - **Consecutive boxes:** they share their piece where a boundary
///   solution of either lies in the other, which then holds it too.
///
/// # Errors
///
/// The first [`Shortfall`] met, boxes in order, then links in order.
pub(crate) fn one_arc_r3<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    boxes: &[Box3],
    slab: Box3,
) -> Result<(), Shortfall> {
    let cut = boxes
        .iter()
        .map(|b| b.intersection(slab))
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
    })
}

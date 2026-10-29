//! **The NURBS × NURBS lane's boxes in ℝ⁴**: the state `(u₁, v₁, u₂, v₂)`
//! on `F = S₁(u₁, v₁) − S₂(u₂, v₂) = 0`, three equations in four
//! unknowns, and the walks that isolate its solutions on a box's
//! boundary.
//!
//! A box is a window on each chart. Its boundary is eight faces, each a
//! three-dimensional box with one coordinate held, on which `F = 0` is
//! three equations in three unknowns. A face is cut into pieces, each
//! treated one of four ways ([`face_walk`]):
//!
//! - dropped where a component of `F`, enclosed over the piece by the
//!   two charts' first-order boxes ([`NurbsBoxes::rect_box`]), is
//!   zero-free;
//! - kept where the Krawczyk operator of `F` restricted to the face maps
//!   the piece strictly into its own interior: exactly one solution,
//!   enclosed by the image;
//! - dropped where the image misses the piece;
//! - cut otherwise, at [`EXIT_CUT`] of its widest side in metres.
//!
//! The same walk serves the boundary pass, whose box is the product of
//! the two knot rectangles and whose roots are the crossings, and limb
//! 3's one-arc proof over the two-chart tube ([`one_arc_pair`]), the
//! analogue of [`super::one_arc::one_arc_r3`] one dimension up.

use std::rc::Rc;

use geom::{NurbsCurve2, NurbsSurface};
use geom_core::interval::certification::Certification;
use geom_core::{Bounds, CertifiedBounds, Interval, Point2};

use super::enclose::{Box3, NurbsBoxes, UvWindow, refused_box};
use super::exhaust::UvRect;
use super::one_arc::{EXIT_CUT, EXIT_DEPTH, Shortfall};
use super::section::sign;
use crate::krawczyk::{det3, inverse3};

/// A box in the ℝ⁴ state space: `[lo, hi]` per coordinate, the first
/// chart's `u, v` then the second's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Box4(pub(crate) [(f64, f64); 4]);

impl Box4 {
    /// The product of `a`'s and `b`'s knot rectangles: the whole state
    /// space of their section.
    pub(crate) fn of_charts<T: CertifiedBounds>(a: &NurbsSurface<T>, b: &NurbsSurface<T>) -> Self {
        Self([
            a.knots_u_f64_interim().domain(),
            a.knots_v_f64_interim().domain(),
            b.knots_u_f64_interim().domain(),
            b.knots_v_f64_interim().domain(),
        ])
    }

    /// The box holding the single state `x`.
    pub(crate) fn at(x: [f64; 4]) -> Self {
        Self(x.map(|c| (c, c)))
    }

    /// The box with coordinate `k` held at `c`.
    fn holding(self, k: usize, c: f64) -> Self {
        let mut s = self.0;
        s[k] = (c, c);
        Self(s)
    }

    /// Whether every side is ordered and finite.
    fn is_box(self) -> bool {
        self.0
            .iter()
            .all(|&(a, b)| a.is_finite() && b.is_finite() && a <= b)
    }

    /// The meet of two boxes, `None` where it is empty.
    pub(crate) fn meet(self, o: Self) -> Option<Self> {
        let mut s = self.0;
        for (k, side) in s.iter_mut().enumerate() {
            *side = (side.0.max(o.0[k].0), side.1.min(o.0[k].1));
            if side.0 > side.1 {
                return None;
            }
        }
        Some(Self(s))
    }

    /// The least box holding both; a `NaN` side in either poisons that
    /// side (`f64::min` and `max` would drop it).
    fn hull(self, o: Self) -> Self {
        let pick = |p: f64, q: f64, f: fn(f64, f64) -> f64| {
            if p.is_nan() || q.is_nan() {
                f64::NAN
            } else {
                f(p, q)
            }
        };
        Self(core::array::from_fn(|k| {
            (
                pick(self.0[k].0, o.0[k].0, f64::min),
                pick(self.0[k].1, o.0[k].1, f64::max),
            )
        }))
    }

    /// Whether `self` lies inside `o`, sides included.
    pub(crate) fn contained_in(self, o: Self) -> bool {
        (0..4).all(|k| o.0[k].0 <= self.0[k].0 && self.0[k].1 <= o.0[k].1)
    }

    /// Whether the state `x` lies in the box, sides included.
    pub(crate) fn holds(self, x: &[f64; 4]) -> bool {
        (0..4).all(|k| self.0[k].0 <= x[k] && x[k] <= self.0[k].1)
    }

    /// The box padded by `pad[k]` along each coordinate.
    pub(crate) fn pad(self, pad: [f64; 4]) -> Self {
        Self(core::array::from_fn(|k| {
            (self.0[k].0 - pad[k], self.0[k].1 + pad[k])
        }))
    }

    /// The first chart's window, `None` where it names no region.
    fn window_a(self) -> Option<UvWindow> {
        UvWindow::new(self.0[0].0, self.0[0].1, self.0[1].0, self.0[1].1)
    }

    /// The second chart's window.
    fn window_b(self) -> Option<UvWindow> {
        UvWindow::new(self.0[2].0, self.0[2].1, self.0[3].0, self.0[3].1)
    }
}

/// The two charts of a NURBS × NURBS pair, boxed, with each coordinate's
/// certified chart speed in metres per parameter unit: what a width in
/// state units is read in metres by.
pub(crate) struct PairBoxes<'a, T: CertifiedBounds> {
    a: Rc<NurbsBoxes<'a, T>>,
    b: Rc<NurbsBoxes<'a, T>>,
    /// The chart speeds `(u₁, v₁, u₂, v₂)`.
    pub(crate) speeds: [f64; 4],
}

/// How [`PairBoxes`] reads a wall's derivative box over a box.
#[derive(Clone, Copy)]
enum DerivRead {
    /// Each touched cell's net cut to the box ([`NurbsBoxes::deriv_box`]).
    Cut,
    /// Over the closed box, through the cells it has width in
    /// ([`NurbsBoxes::deriv_box_closed`]).
    Closed,
}

impl<'a, T: CertifiedBounds> PairBoxes<'a, T> {
    /// The pair over the two operands' own cells (the enclosures they
    /// hold, shared), with its four chart speeds.
    pub(crate) fn new(
        a: &super::ChartedNurbs<'a, T>,
        b: &super::ChartedNurbs<'a, T>,
        speeds: [f64; 4],
    ) -> Self {
        Self {
            a: Rc::clone(a.own_boxes()),
            b: Rc::clone(b.own_boxes()),
            speeds,
        }
    }

    /// An enclosure of `F = S₁ − S₂` over `x`, per component.
    pub(crate) fn residual(&self, x: Box4) -> [Interval; 3] {
        let a = x
            .window_a()
            .map_or_else(refused_box, |w| self.a.rect_box(w));
        let b = x
            .window_b()
            .map_or_else(refused_box, |w| self.b.rect_box(w));
        [a.x - b.x, a.y - b.y, a.z - b.z]
    }

    /// An enclosure of `F`'s Jacobian over `x`: column `j` is `∂F/∂xⱼ`,
    /// rows the three components.
    pub(crate) fn jacobian(&self, x: Box4) -> [[Interval; 4]; 3] {
        self.jacobian_read(x, DerivRead::Cut)
    }

    /// The Jacobian over the closed box `x`, read only through the span
    /// cells `x` has width in ([`NurbsBoxes::deriv_box_closed`]).
    pub(crate) fn jacobian_closed(&self, x: Box4) -> [[Interval; 4]; 3] {
        self.jacobian_read(x, DerivRead::Closed)
    }

    /// Whether both walls' first partials are continuous inside `x`
    /// ([`NurbsBoxes::first_order_continuous_in`]).
    pub(crate) fn first_order_continuous_in(&self, x: Box4) -> bool {
        x.window_a()
            .is_some_and(|w| self.a.first_order_continuous_in(w))
            && x.window_b()
                .is_some_and(|w| self.b.first_order_continuous_in(w))
    }

    /// The Jacobian, each wall's derivative box read as `how` says.
    fn jacobian_read(&self, x: Box4, how: DerivRead) -> [[Interval; 4]; 3] {
        let col = |b: Box3, s: f64| {
            [
                b.x * Interval::point(s),
                b.y * Interval::point(s),
                b.z * Interval::point(s),
            ]
        };
        let read = |n: &NurbsBoxes<'_, T>, w: Option<UvWindow>, u: bool| {
            w.map_or_else(refused_box, |w| match how {
                DerivRead::Cut => n.deriv_box(w, u),
                DerivRead::Closed => n.deriv_box_closed(w, u),
            })
        };
        let (wa, wb) = (x.window_a(), x.window_b());
        let cols = [
            col(read(&self.a, wa, true), 1.0),
            col(read(&self.a, wa, false), 1.0),
            col(read(&self.b, wb, true), -1.0),
            col(read(&self.b, wb, false), -1.0),
        ];
        core::array::from_fn(|r| core::array::from_fn(|c| cols[c][r]))
    }

    /// The widest side of `x` in metres, and its coordinate.
    fn widest(&self, x: Box4, free: &[usize]) -> usize {
        heaviest(free, |k| (x.0[k].1 - x.0[k].0) * self.speeds[k])
    }
}

/// The 3×3 minor of a 3×4 interval matrix with column `skip` deleted.
pub(crate) fn minor(j: &[[Interval; 4]; 3], skip: usize) -> [[Interval; 3]; 3] {
    let cols: Vec<usize> = (0..4).filter(|&c| c != skip).collect();
    core::array::from_fn(|r| core::array::from_fn(|c| j[r][cols[c]]))
}

/// What the Krawczyk test proved over a face piece.
enum Krawczyk {
    /// Exactly one solution, inside this box.
    One(Box4),
    /// No solution.
    None,
}

/// The Krawczyk operator of `F` over the face piece `x`, in its three
/// free coordinates: `K = m − Y·F(m) + (I − Y·J(x))·(x − m)`, `Y` the
/// inverse of `J`'s midpoint. `Y` only preconditions; any `Y` keeps the
/// test sound.
fn krawczyk<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    x: Box4,
    free: [usize; 3],
) -> Option<Krawczyk> {
    let jac = pair.jacobian(x);
    if mean_value_excludes(pair, x, free, &jac) {
        return Some(Krawczyk::None);
    }
    krawczyk_with(pair, x, free, jac)
}

/// Whether the mean-value form `F(m) + J(x)·(x − m)` over the face piece
/// `x` excludes zero along one of five fixed directions: the three
/// coordinate axes, and each chart's normal at the piece's middle. Along
/// a chart's normal its own columns of `J` are tangent, so they enter
/// the form only through their variation over the piece: a side row
/// passing near the other chart, nearly tangent to it, is cleared at
/// the size its gap and the charts' curvature set, where separate boxes
/// of the two charts would need pieces as narrow as the gap.
fn mean_value_excludes<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    x: Box4,
    free: [usize; 3],
    jac: &[[Interval; 4]; 3],
) -> bool {
    let mid = |s: (f64, f64)| 0.5 * (s.0 + s.1);
    let mut m = x;
    for &k in &free {
        m = m.holding(k, mid(x.0[k]));
    }
    let f = pair.residual(m);
    let col = |c: usize| [0, 1, 2].map(|r| 0.5 * (jac[r][c].lo() + jac[r][c].hi()));
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let directions = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        cross(col(0), col(1)),
        cross(col(2), col(3)),
    ];
    directions.iter().any(|n| {
        if !n.iter().all(|v| v.is_finite()) {
            return false;
        }
        let dot = |v: [Interval; 3]| {
            (0..3).fold(Interval::point(0.0), |acc, r| {
                acc + Interval::point(n[r]) * v[r]
            })
        };
        let v = free.iter().fold(dot(f), |acc, &k| {
            acc + dot([jac[0][k], jac[1][k], jac[2][k]])
                * (Interval::from_bounds(x.0[k].0, x.0[k].1) - Interval::point(mid(x.0[k])))
        });
        sign(v).is_some()
    })
}

/// [`krawczyk`] with `jac` an enclosure of `F`'s Jacobian over `x`.
fn krawczyk_with<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    x: Box4,
    free: [usize; 3],
    jac: [[Interval; 4]; 3],
) -> Option<Krawczyk> {
    let k_box = image(pair, x, free, jac)?;
    let inside = free
        .iter()
        .all(|&k| k_box.0[k].0 > x.0[k].0 && k_box.0[k].1 < x.0[k].1);
    let misses = free
        .iter()
        .any(|&k| k_box.0[k].1 < x.0[k].0 || k_box.0[k].0 > x.0[k].1);
    if inside {
        Some(Krawczyk::One(k_box))
    } else if misses {
        Some(Krawczyk::None)
    } else {
        None
    }
}

/// The Krawczyk operator's image of the face piece `x` ([`krawczyk`]):
/// every solution in `x` lies in it. `None` where `J`'s midpoint is
/// singular or the image is not certified.
fn image<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    x: Box4,
    free: [usize; 3],
    jac: [[Interval; 4]; 3],
) -> Option<Box4> {
    let mid = |s: (f64, f64)| 0.5 * (s.0 + s.1);
    let mut m = x;
    for &k in &free {
        m = m.holding(k, mid(x.0[k]));
    }
    let f = pair.residual(m);
    let j: [[Interval; 3]; 3] = core::array::from_fn(|r| core::array::from_fn(|c| jac[r][free[c]]));
    let jm: [[f64; 3]; 3] = j.map(|row| row.map(|v| 0.5 * (v.lo() + v.hi())));
    let y = inverse3(jm)?;
    let d: [Interval; 3] =
        free.map(|k| Interval::from_bounds(x.0[k].0, x.0[k].1) - Interval::point(mid(x.0[k])));
    let mut k_box = x;
    for r in 0..3 {
        let mut v = Interval::point(mid(x.0[free[r]]));
        for c in 0..3 {
            v = v - Interval::point(y[r][c]) * f[c];
        }
        for c in 0..3 {
            let mut yj = Interval::point(if r == c { 1.0 } else { 0.0 });
            for q in 0..3 {
                yj = yj - Interval::point(y[r][q]) * j[q][c];
            }
            v = v + yj * d[c];
        }
        if !v.is_certified() {
            return None;
        }
        k_box.0[free[r]] = (v.lo(), v.hi());
    }
    Some(k_box)
}

/// The most times [`narrow`] applies the operator.
const NARROWINGS: u32 = 64;

/// The root box `q` on the face held at `k`, narrowed: met with its
/// Krawczyk image ([`image`]) until the meet stops shrinking. The image
/// holds every solution in `q`, so each meet holds the one `q` does.
fn narrow<T: CertifiedBounds>(pair: &PairBoxes<'_, T>, (q, k): (Box4, usize)) -> Box4 {
    let free = free_of(k);
    let mut q = q;
    for _ in 0..NARROWINGS {
        let Some(next) = image(pair, q, free, pair.jacobian(q)).and_then(|i| i.meet(q)) else {
            return q;
        };
        if next == q {
            return q;
        }
        q = next;
    }
    q
}

/// The three coordinates a face held at `k` leaves free.
fn free_of(k: usize) -> [usize; 3] {
    match k {
        0 => [1, 2, 3],
        1 => [0, 2, 3],
        2 => [0, 1, 3],
        _ => [0, 1, 2],
    }
}

/// The solutions on the face `x` held at coordinate `k`, pushed onto
/// `roots` (module docs).
///
/// # Errors
///
/// The piece that resolved no way within [`EXIT_DEPTH`] cuts; `None`
/// where `pieces` passes `budget` or `x` is no box.
fn face_walk<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    (x, k): (Box4, usize),
    roots: &mut Vec<Box4>,
    (pieces, budget): (&mut u32, u32),
) -> Result<(), Option<Box4>> {
    let free = free_of(k);
    if !x.is_box() {
        return Err(None);
    }
    let mut stack = vec![(x, 0u32)];
    while let Some((x, depth)) = stack.pop() {
        *pieces += 1;
        if *pieces > budget {
            return Err(None);
        }
        if pair.residual(x).iter().any(|c| sign(*c).is_some()) {
            continue;
        }
        match krawczyk(pair, x, free) {
            Some(Krawczyk::One(root)) => {
                roots.push(root);
                continue;
            }
            Some(Krawczyk::None) => continue,
            None => {}
        }
        if depth >= EXIT_DEPTH {
            return Err(Some(x));
        }
        let a = pair.widest(x, &free);
        let (lo, hi) = x.0[a];
        let cut = lo + EXIT_CUT * (hi - lo);
        let (mut l, mut h) = (x, x);
        l.0[a] = (lo, cut);
        h.0[a] = (cut, hi);
        stack.push((h, depth + 1));
        stack.push((l, depth + 1));
    }
    Ok(())
}

/// The most pieces one box's walk examines in a tube's one-arc proof.
pub(crate) const PAIR_EXIT_PIECES: u32 = 4096;

/// The most pieces the boundary pass's walk of the domain box examines:
/// eight faces of the whole product, each read from its full extent.
pub(crate) const PAIR_BOUNDARY_PIECES: u32 = 1 << 18;

/// The solutions of `F = 0` on the boundary of `r`, each in a box that
/// holds exactly one ([`face_walk`] on each of its eight faces), with the
/// coordinate of the face each lies on.
///
/// `None` when a piece resolves no way within [`EXIT_DEPTH`] cuts, or the
/// walk passes `budget` pieces.
pub(crate) fn face_roots<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    r: Box4,
    budget: u32,
) -> Option<Vec<(Box4, usize, bool)>> {
    walk_faces(pair, r, budget).ok()
}

/// [`face_roots`], with the piece its walk stopped in where it stopped
/// at [`EXIT_DEPTH`] ([`face_walk`]).
fn walk_faces<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    r: Box4,
    budget: u32,
) -> Result<Vec<(Box4, usize, bool)>, Option<Box4>> {
    let mut out = Vec::new();
    let mut pieces = 0u32;
    for k in 0..4 {
        for (high, c) in [(false, r.0[k].0), (true, r.0[k].1)] {
            let mut roots = Vec::new();
            face_walk(
                pair,
                (r.holding(k, c), k),
                &mut roots,
                (&mut pieces, budget),
            )?;
            out.extend(roots.into_iter().map(|q| (q, k, high)));
        }
    }
    Ok(out)
}

/// The near reading of an end ([`reaches_end`]).
enum Near {
    Found,
    Far,
    Unknown,
}

/// **The box `r`'s piece reaches the carrier's end `at`**, `k` the
/// coordinate the carrier's state runs most along there, in metres.
/// - One of `r`'s own boundary solutions, `roots`, each narrowed
///   ([`narrow`]), lies in the box about the end ([`end_box`]), its
///   half-width `eps/8` metres at each axis's chart speed.
///   This is the reading of an end on a side of the domain: the slice's
///   solution there lies on the slice's own boundary, where no Krawczyk
///   test isolates it.
/// - Or the slice of `r` through the end across `k` holds a solution.
/// - Or the box about the end, cut to `r`, holds one on its boundary.
///
/// The box is r3's ball of diameter `eps` (`one_arc::r3_reaches_end`,
/// half-width `eps/(2√3)` less the end's width) read in chart units: a
/// chart's speed bounds its surface's motion along each of its two
/// axes, so a point of the box lands within `eps/8 + eps/8 = eps/4` of
/// the end's image on either chart, inside that ball with room. The end
/// is a point, so no width of its own is taken off.
///
/// [`Shortfall::Short`] only where the slice and the box about the end
/// are both certified to hold none (a root of `r` in the box lies on
/// the box's boundary too, so that covers the first reading); an end
/// whose slice lies outside `r` along `k`, or that no reading resolves,
/// is [`Shortfall::Undecided`], which [`one_arc_pair`] may rename by the
/// carrier's shape.
fn reaches_end<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    (r, roots): (Box4, &[(Box4, usize, bool)]),
    (at, k): ([f64; 4], usize),
    eps: f64,
) -> Result<(), Shortfall> {
    let ball = end_box(pair, at, eps);
    if roots
        .iter()
        .any(|&(q, face, _)| narrow(pair, (q, face)).contained_in(ball))
    {
        return Ok(());
    }
    let c = at[k];
    if !(r.0[k].0 <= c && c <= r.0[k].1) {
        return Err(Shortfall::Undecided);
    }
    let mut on_slice = Vec::new();
    let sliced = face_walk(
        pair,
        (r.holding(k, c), k),
        &mut on_slice,
        (&mut 0, PAIR_EXIT_PIECES),
    )
    .ok();
    if !on_slice.is_empty() {
        return Ok(());
    }
    let near = match ball.meet(r) {
        None => Near::Far,
        Some(b) => match face_roots(pair, b, PAIR_EXIT_PIECES) {
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

/// The coordinate a state-space direction `e` runs most along, in
/// metres at the pair's chart speeds.
pub(crate) fn dominant<T: CertifiedBounds>(pair: &PairBoxes<'_, T>, e: [f64; 4]) -> usize {
    heaviest(&ALL_AXES, |k| (e[k] * pair.speeds[k]).abs())
}

/// The four axes of the state.
pub(crate) const ALL_AXES: [usize; 4] = [0, 1, 2, 3];

/// The axis of `axes` (non-empty) with the largest weight `w`, the
/// first on a tie: the widest side to split, or the axis a direction
/// runs most along, each in metres at its chart's speed.
pub(crate) fn heaviest(axes: &[usize], w: impl Fn(usize) -> f64) -> usize {
    axes.iter()
        .copied()
        .fold(axes[0], |best, k| if w(k) > w(best) { k } else { best })
}

/// **The two-chart chain's solution set is one arc, and it spans the
/// carrier** — [`super::one_arc::one_arc_r3`]'s proof in ℝ⁴:
///
/// - **Hypothesis:** the graph margin is zero-free over every box, each
///   cut to the product of the two knot rectangles, `domain`.
/// - **Each box:** exactly two simple solutions on its boundary
///   ([`face_roots`]).
/// - **Consecutive boxes:** they share their piece where a boundary
///   solution of either lies in the other, which then holds it too.
/// - **The carrier's ends:** the first box's piece reaches the carrier's
///   start, the last box's its end ([`reaches_end`]); `ends` holds each
///   end's state and the coordinate the carrier runs most along there.
///
/// A walk or end reading that resolves nothing, at a box or an end, is
/// [`Shortfall::Undecided`], renamed after the fact where the carrier's
/// shape accounts for it. Nothing else is renamed:
/// - [`Shortfall::AlongSide`] where `along` names the chart whose image
///   lies along a side of its domain ([`along_side`], read by the
///   caller);
/// - else [`Shortfall::EndOnSides`] where the failure is an end's: its
///   reading in [`reaches_end`], or a box walk that stopped in a piece
///   meeting that end's box ([`end_box`]), the end lying on two or more
///   of `domain`'s sides ([`sides_at`]), that end's count alone.
///
/// A box missing `domain` stays unnamed.
///
/// # Errors
///
/// The first [`Shortfall`] met, boxes in order, then links in order,
/// then the start and the end.
pub(crate) fn one_arc_pair<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    (boxes, domain, along): (&[Box4], Box4, Option<u32>),
    ends: [([f64; 4], usize); 2],
    eps: f64,
) -> Result<(), Shortfall> {
    let end_boxes = [0, boxes.len().saturating_sub(1)];
    // `failed` is the end the failure is that end's, if any.
    let named = |failed: Option<usize>| {
        if let Some(k) = along {
            return Shortfall::AlongSide(k);
        }
        match failed.map(|j| sides_at(pair, domain, ends[j].0, eps)) {
            Some(on) if on >= 2 => Shortfall::EndOnSides(on),
            _ => Shortfall::Undecided,
        }
    };
    let cut = boxes
        .iter()
        .map(|b| b.meet(domain))
        .collect::<Option<Vec<Box4>>>()
        .ok_or(Shortfall::Undecided)?;
    let mut roots = Vec::with_capacity(cut.len());
    for (i, r) in cut.iter().enumerate() {
        match walk_faces(pair, *r, PAIR_EXIT_PIECES) {
            Ok(v) if v.len() == 2 => roots.push(v),
            #[allow(clippy::cast_possible_truncation)]
            Ok(v) => return Err(Shortfall::Count(v.len() as u32)),
            Err(stopped) => {
                let at_end = |j: usize| {
                    end_boxes[j] == i
                        && stopped.is_some_and(|p| p.meet(end_box(pair, ends[j].0, eps)).is_some())
                };
                return Err(named((0..2).find(|&j| at_end(j))));
            }
        }
    }
    (1..cut.len()).try_for_each(|k| {
        let shared = roots[k - 1].iter().any(|q| q.0.contained_in(cut[k]))
            || roots[k].iter().any(|q| q.0.contained_in(cut[k - 1]));
        if shared {
            Ok(())
        } else {
            Err(Shortfall::Unlinked)
        }
    })?;
    let (Some(&first), Some(&last), Some(r0), Some(r1)) =
        (cut.first(), cut.last(), roots.first(), roots.last())
    else {
        return Err(Shortfall::Undecided);
    };
    let read = |j: usize, s: Shortfall| match s {
        Shortfall::Undecided => named(Some(j)),
        s => s,
    };
    reaches_end(pair, (first, r0), ends[0], eps).map_err(|s| read(0, s))?;
    reaches_end(pair, (last, r1), ends[1], eps).map_err(|s| read(1, s))
}

/// The half-width, in chart units, of the box about a carrier's end
/// that [`reaches_end`] looks in: `eps/8` metres at the axis's chart
/// `speed`. `0` where that is not finite. It bounds the chart IMAGE's
/// distance from the end, not the locus's.
pub(crate) fn end_reach(eps: f64, speed: f64) -> f64 {
    let h = eps / 8.0 / speed;
    if h.is_finite() { h } else { 0.0 }
}

/// The box about the state `x` [`reaches_end`] looks in ([`end_reach`]
/// on each axis).
fn end_box<T: CertifiedBounds>(pair: &PairBoxes<'_, T>, x: [f64; 4], eps: f64) -> Box4 {
    Box4::at(x).pad(core::array::from_fn(|k| end_reach(eps, pair.speeds[k])))
}

/// How many sides of `domain` the state `x` lies on, each read as the
/// side meeting [`end_box`] about it.
#[allow(clippy::cast_possible_truncation)]
fn sides_at<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    domain: Box4,
    x: [f64; 4],
    eps: f64,
) -> u32 {
    let near = |k: usize, side: f64| (x[k] - side).abs() <= end_reach(eps, pair.speeds[k]);
    (0..4)
        .filter(|&k| near(k, domain.0[k].0) || near(k, domain.0[k].1))
        .count() as u32
}

/// Whether the chart image `pcurve` lies along a side of `domain`: every
/// control point's `u` within `within.0` of one `u` side, or every one's
/// `v` within `within.1` of one `v` side, each bracket whole. The weights
/// are positive, so the image lies in its control points' hull, that
/// near the side. `within` is [`end_reach`] on each axis. This places
/// the IMAGE, not the locus.
///
/// A control whose bracket does not certify is not along any side: the
/// answer moves a refusal from `Undecided` to the sign-certain
/// `AlongSide`, so it rests only on certified controls.
pub(crate) fn along_side<T: CertifiedBounds>(
    pcurve: &NurbsCurve2<T>,
    domain: UvRect,
    within: (f64, f64),
) -> bool {
    let on = |read: fn(&Point2<T>) -> T, (lo, hi): (f64, f64), h: f64| {
        [lo, hi].into_iter().any(|side| {
            pcurve.control().iter().all(|p| {
                read(p)
                    .certified_bracket()
                    .is_some_and(|(lo, hi)| (lo - side).abs() <= h && (hi - side).abs() <= h)
            })
        })
    };
    on(|p| p.x, domain.u, within.0) || on(|p| p.y, domain.v, within.1)
}

/// The graph margin over a box: the determinant of `F`'s Jacobian, each
/// column scaled to metres per metre by its chart speed, stacked over
/// the unit direction `e` in those scaled coordinates. Zero-free, the
/// solution set in the box is a graph over `e`: two solutions `x`, `y`
/// in one slice would give a nonzero `y − x ⟂ e` orthogonal to each
/// row of `J` at some point of the segment, so the determinant would
/// vanish at the rows' mean-value points, which the enclosure, valid at
/// every point of the box independently per entry, excludes.
///
/// Scaling a column by a positive constant scales the determinant and
/// moves no zero, so the speeds only make the reading a sine: two
/// orthonormal charts crossing at angle θ read `√2·sin θ` along the
/// locus.
pub(crate) fn graph_margin<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    x: Box4,
    e: [f64; 4],
) -> Interval {
    margin_of(pair, pair.jacobian(x), e)
}

/// [`graph_margin`] of a Jacobian `j` read over a box.
fn margin_of<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    mut j: [[Interval; 4]; 3],
    e: [f64; 4],
) -> Interval {
    for row in &mut j {
        for (c, v) in row.iter_mut().enumerate() {
            *v = *v * Interval::point(1.0 / pair.speeds[c]);
        }
    }
    (0..4).fold(Interval::point(0.0), |acc, c| {
        let sgn = if c % 2 == 0 { -1.0 } else { 1.0 };
        acc + Interval::point(sgn * e[c]) * det3(minor(&j, c))
    })
}

/// The pieces a row's span is first cut into ([`row_crossing`]).
const ROW_PIECES: usize = 8;

/// How many times [`row_crossing`] may halve a piece before it stops.
const ROW_DEPTH: u32 = 10;

/// The share of a chart's domain a row's box is widened by across the row
/// ([`row_crossing`]).
const ROW_SLIVER: f64 = 1.0 / (1u64 << 40) as f64;

/// `raw`, a direction in the state's chart coordinates, scaled to metres
/// by each coordinate's chart `speeds` and made a unit vector: the axis a
/// [`graph_margin`] is read across. `None` for a zero or non-finite one.
pub(crate) fn scaled_unit(raw: [f64; 4], speeds: [f64; 4]) -> Option<[f64; 4]> {
    let scaled: [f64; 4] = core::array::from_fn(|k| raw[k] * speeds[k]);
    let n = scaled.iter().map(|v| v.powi(2)).sum::<f64>().sqrt();
    (n.is_finite() && n > 0.0).then(|| scaled.map(|v| v / n))
}

/// Why [`row_crossing`] gave no reading: wall `0` or `1` has no chart
/// speeds to scale its Jacobian by.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RowUnreadable(pub(crate) usize, pub(crate) super::ChartSpeedRefusal);

/// What [`row_crossing`] read over a row's cover.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum RowCrossing {
    /// Two zero-free pieces read opposite signs with both walls' first
    /// partials continuous over the stretch between them: the walls'
    /// tangent planes coincide at a state of the row there.
    Turns,
    /// Every piece read zero-free, with no turn: the least graph margin
    /// over the cover.
    Clear(f64),
    /// A piece, halved [`ROW_DEPTH`] times, still read no sign, and no
    /// turn was read.
    Unread,
}

/// **Two walls' crossing along a shared row, over its whole span**: the
/// [`graph_margin`] read over a cover of `[t0, t1]`, ends and joins
/// included. `read(lo, hi)` gives a piece's state box (holding both row
/// images over `[lo, hi]`) and the images' chord across it, in chart
/// coordinates.
///
/// A zero-free graph margin over a box says the pair's Jacobian has rank
/// three at every state in it: the two tangent planes differ there. A
/// piece that is not zero-free is halved, up to [`ROW_DEPTH`] times. Each
/// piece is read over its closed box through each wall's spans that the
/// box has width in ([`PairBoxes::jacobian_closed`]), so a piece ending
/// on a knot reads that knot through its own span's polynomial, and the
/// pieces' boxes meet: every state of the row images as evaluated is
/// read. A `NaN` in a box or a chord poisons its piece, which then reads
/// no sign.
///
/// **Opposite signs.** With the images exact, `F` vanishes along the row,
/// so the row's tangent `d` lies in the Jacobian's kernel. Write `n` for the Jacobian's
/// generalized cross product (scaled as the margin's): continuous wherever
/// the first partials are, zero exactly where the rank drops, and along
/// the kernel otherwise, so `n = λ·d`. The margin at a row state is `n·e`.
/// The caller states that each image moves along one chart coordinate,
/// strictly monotonically with nonzero speed, and that a chord component
/// is zero or of its coordinate's sign; so with `σ` the coordinates' sign
/// pattern, `d·σ > 0` and `d·e > 0` on every piece, and the margin's sign
/// there is `λ`'s, and `n·σ`'s. Two zero-free pieces of opposite signs,
/// with both walls' first partials continuous inside the hull of the boxes
/// from one to the other ([`PairBoxes::first_order_continuous_in`]), give
/// the continuous `n·σ` both signs between them: it is zero at a state of
/// the row there, where `λ`, and so the rank, drops and the tangent
/// planes coincide. Across a knot of full multiplicity the partials jump,
/// and a sign change there says nothing. Where the images hold only to
/// their certified residual, a turn may read a crossing at an angle of
/// about that residual as an exact zero, a refusal either way.
///
/// The caller's `read` gives the states of two images
/// `description::row_of` admits; nothing here checks it.
///
/// A box with no width across the row is widened into the domain when its
/// fixed coordinate is a domain end, so no net is cut to a point (a box
/// of no width would read its whole cell); elsewhere it is read as it
/// stands, a sound and looser reading.
pub(crate) fn row_crossing<T: CertifiedBounds>(
    (a, b): (&NurbsSurface<T>, &NurbsSurface<T>),
    (t0, t1): (f64, f64),
    read: impl Fn(f64, f64) -> (Box4, [f64; 4]),
) -> Result<RowCrossing, RowUnreadable> {
    let ca = super::ChartedNurbs::mint(a).map_err(|e| unreadable(0, e))?;
    let cb = super::ChartedNurbs::mint(b).map_err(|e| unreadable(1, e))?;
    let (sa, sb) = (ca.speeds(), cb.speeds());
    let pair = PairBoxes::new(&ca, &cb, [sa.u.get(), sa.v.get(), sb.u.get(), sb.v.get()]);
    let domain = Box4::of_charts(a, b);
    #[allow(clippy::cast_precision_loss)]
    let mut work: std::collections::VecDeque<(f64, f64, u32)> = (0..ROW_PIECES)
        .map(|k| {
            let at = |k: usize| t0 + (t1 - t0) * (k as f64 / ROW_PIECES as f64);
            (at(k), at(k + 1), 0)
        })
        .collect();
    // Each read piece: its start, its box, and its margin's sign where it
    // is zero-free (`None` for a piece read at depth with no sign).
    let mut pieces: Vec<(f64, Box4, Option<bool>)> = Vec::new();
    let mut least = f64::INFINITY;
    while let Some((lo, hi, depth)) = work.pop_front() {
        let (x, chord) = read(lo, hi);
        let x = Box4(core::array::from_fn(|k| {
            let ((l, h), (d0, d1)) = (x.0[k], domain.0[k]);
            let w = (d1 - d0) * ROW_SLIVER;
            if l < h {
                (l, h)
            } else if l == d0 {
                (l, (l + w).max(l.next_up()))
            } else if l == d1 {
                ((l - w).min(l.next_down()), l)
            } else {
                (l, h)
            }
        }));
        let margin = scaled_unit(chord, pair.speeds)
            .map(|e| margin_of(&pair, pair.jacobian_closed(x), e))
            .and_then(|i| {
                let m = super::enclose::tube_margin(i)?;
                (m > 0.0).then_some((m, i.lo() > 0.0))
            });
        match margin {
            Some((m, positive)) => {
                least = least.min(m);
                pieces.push((lo, x, Some(positive)));
            }
            None if depth < ROW_DEPTH => {
                let half = lo * 0.5 + hi * 0.5;
                work.push_back((lo, half, depth + 1));
                work.push_back((half, hi, depth + 1));
            }
            None => pieces.push((lo, x, None)),
        }
    }
    pieces.sort_by(|p, q| p.0.total_cmp(&q.0));
    let signed: Vec<usize> = (0..pieces.len())
        .filter(|&k| pieces[k].2.is_some())
        .collect();
    let turns = signed.windows(2).any(|w| {
        let (i, j) = (w[0], w[1]);
        pieces[i].2 != pieces[j].2
            && pair.first_order_continuous_in(
                pieces[i..=j].iter().fold(pieces[i].1, |h, p| h.hull(p.1)),
            )
    });
    Ok(if turns {
        RowCrossing::Turns
    } else if pieces.iter().any(|p| p.2.is_none()) {
        RowCrossing::Unread
    } else {
        RowCrossing::Clear(least)
    })
}

/// Wall `wall`'s chart speeds refused.
fn unreadable(wall: usize, e: super::SsiError) -> RowUnreadable {
    match e {
        super::SsiError::ChartSpeed(r) => RowUnreadable(wall, r),
        other => unreachable!("a chart's mint refuses only its speeds: {other:?}"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{CertifiedBounds, Interval, Point3, Real};

    use super::super::ChartedNurbs;
    use super::super::one_arc::Shortfall;
    use super::{Box4, PAIR_EXIT_PIECES, PairBoxes, face_roots, narrow, one_arc_pair};

    /// The bilinear patch through `corner(i, j)` at `(u, v) = (i, j)`.
    fn bilinear(corner: impl Fn(f64, f64) -> [f64; 3]) -> NurbsSurface<f64> {
        let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .map(|(i, j)| {
                let [x, y, z] = corner(i, j);
                Point3::new(x, y, z)
            })
            .to_vec();
        NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap()
    }

    /// The pair over two charts, at their own speeds.
    fn boxed<'a, T: CertifiedBounds>(
        a: &'a NurbsSurface<T>,
        b: &'a NurbsSurface<T>,
    ) -> PairBoxes<'a, T> {
        let (na, nb) = (
            ChartedNurbs::mint(a).unwrap(),
            ChartedNurbs::mint(b).unwrap(),
        );
        let (sa, sb) = (na.speeds(), nb.speeds());
        PairBoxes::new(&na, &nb, [sa.u.get(), sa.v.get(), sb.u.get(), sb.v.get()])
    }

    /// [`one_arc_pair`] over the one box that is the two charts' whole
    /// domain.
    fn spans<T: CertifiedBounds>(
        (a, b): (&NurbsSurface<T>, &NurbsSurface<T>),
        ends: [([f64; 4], usize); 2],
        eps: f64,
    ) -> Result<(), Shortfall> {
        let domain = Box4::of_charts(a, b);
        one_arc_pair(&boxed(a, b), (&[domain], domain, None), ends, eps)
    }

    /// The walls `x = 0`, `(u, v) ↦ (0, u, v)`, and `y = ½`,
    /// `(u, v) ↦ (u − ½, ½, 2v − ½)`, crossing on `x = 0, y = ½` from the
    /// first's `v = 0` side to its `v = 1` side: the state
    /// `(½, z, ½, (z + ½)/2)`. In metres the locus runs along `v₁` and
    /// `v₂` alike, so either is its axis at an end.
    fn walls() -> (NurbsSurface<f64>, NurbsSurface<f64>) {
        (
            bilinear(|i, j| [0.0, i, j]),
            bilinear(|i, j| [i - 0.5, 0.5, 2.0 * j - 0.5]),
        )
    }

    /// **An end on a side of the domain is reached by the box's own root
    /// there.** Read across `v₂`, the slice through either end meets the
    /// locus on its own edge `v₁ = 0` or `v₁ = 1`, where no Krawczyk test
    /// isolates it; the box's boundary root on that side, inside the box
    /// about the end, is what reaches it, at every ε, on the `f64` lane
    /// and on the `Interval` lane, whose chart midpoint is an enclosure.
    #[test]
    fn an_end_on_a_domain_side_is_reached_by_the_root_there() {
        let (a, b) = walls();
        let (ia, ib) = (
            a.map_scalar(<Interval as Real>::from_f64),
            b.map_scalar(<Interval as Real>::from_f64),
        );
        let ends = [([0.5, 0.0, 0.5, 0.25], 3), ([0.5, 1.0, 0.5, 0.75], 3)];
        for eps in [1e-6, 1e-9, 1e-12] {
            assert_eq!(
                spans((&a, &b), ends, eps),
                Ok(()),
                "the arc spans both sides at ε = {eps:e}, f64"
            );
            assert_eq!(
                spans((&ia, &ib), ends, eps),
                Ok(()),
                "the arc spans both sides at ε = {eps:e}, Interval"
            );
        }
        // A start a tenth past the first's `v = 0` side: no root of the
        // box lies near it, and the box about it misses the domain.
        let past = [([0.5, -0.1, 0.5, 0.2], 3), ends[1]];
        assert_eq!(
            spans((&a, &b), past, 1e-9),
            Err(Shortfall::Short),
            "a carrier overrunning the arc refuses as short"
        );
    }

    /// **An end at a corner of a chart refuses by name.** The walls
    /// `(u, v) ↦ (0, u, v)` and `(u, v) ↦ (u − ½, 2v − ½, 2v − ½)` cross on
    /// the first's diagonal `u₁ = v₁`, from its corner `(0, 0)` to its
    /// corner `(1, 1)`: each end's root lies on two faces of the box at
    /// once, where neither face's walk isolates it. The walk is undecided
    /// at every ε, and names the two sides the end lies on.
    #[test]
    fn an_end_at_a_charts_corner_refuses_by_its_sides() {
        let (a, b) = (
            bilinear(|i, j| [0.0, i, j]),
            bilinear(|i, j| [i - 0.5, 2.0 * j - 0.5, 2.0 * j - 0.5]),
        );
        let ends = [([0.0, 0.0, 0.5, 0.25], 1), ([1.0, 1.0, 0.5, 0.75], 1)];
        for eps in [1e-6, 1e-9, 1e-12] {
            assert_eq!(
                spans((&a, &b), ends, eps),
                Err(Shortfall::EndOnSides(2)),
                "the start lies on the first chart's u and v sides at ε = {eps:e}"
            );
        }
    }

    /// **An end's refusal counts that end's sides alone.** The walls
    /// `(u, v) ↦ (0, u, v)` and `(u, v) ↦ (u − ½, 2v − 1, 2v − 1)` cross on
    /// the first's diagonal, from its corner `(0, 0)`, where the second is
    /// at `(½, ½)`, to its corner `(1, 1)`, where the second is on its
    /// `v = 1` side: the start lies on two domain sides, the end on three.
    /// Over one box both ends are that box's, and the walk stops first at
    /// the start's corner, on the face `u₁ = 0`: it names the start's two
    /// sides, not the end's three.
    #[test]
    fn an_end_refusal_counts_that_ends_sides_alone() {
        let (a, b) = (
            bilinear(|i, j| [0.0, i, j]),
            bilinear(|i, j| [i - 0.5, 2.0 * j - 1.0, 2.0 * j - 1.0]),
        );
        let ends = [([0.0, 0.0, 0.5, 0.5], 1), ([1.0, 1.0, 0.5, 1.0], 1)];
        for eps in [1e-6, 1e-9, 1e-12] {
            assert_eq!(
                spans((&a, &b), ends, eps),
                Err(Shortfall::EndOnSides(2)),
                "the start's two sides at ε = {eps:e}"
            );
        }
    }

    /// **An image off a side by twice the end box is not along it.** A
    /// polyline at `v = d` over `[0, 1]²`, at unit speed: along the
    /// `v = 0` side for `d` inside the end box `ε/8` ([`end_reach`]),
    /// not for `d = 2·ε/8`. Red under an end box loosened to `ε`.
    #[test]
    fn an_image_off_a_side_by_twice_the_end_box_is_not_along_it() {
        use geom::NurbsCurve2;
        use geom_core::Point2;
        let domain = super::UvRect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        };
        let at = |d: f64| {
            NurbsCurve2::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point2::new(0.2, d), Point2::new(0.8, d)],
                vec![1.0; 2],
            )
            .unwrap()
        };
        for eps in [1e-6, 1e-9, 1e-12] {
            let within = (super::end_reach(eps, 1.0), super::end_reach(eps, 1.0));
            for (d, along) in [(0.0, true), (eps / 16.0, true), (2.0 * eps / 8.0, false)] {
                assert_eq!(
                    super::along_side(&at(d), domain, within),
                    along,
                    "v = {d:e} at ε = {eps:e}"
                );
            }
        }
    }

    /// **A control that does not certify places no image along a side.**
    /// The polyline at `v = 0` lifted to intervals: along the side with
    /// point controls, not with a refused point bracket on the side for
    /// one control's `v`. Red under a read of the bracket's ends, which
    /// would move an `Undecided` refusal to the sign-certain `AlongSide`.
    #[test]
    fn a_refused_control_is_not_along_a_side() {
        use geom::NurbsCurve2;
        use geom_core::{Interval, Point2};
        let domain = super::UvRect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        };
        // The second control's `v` is marked by a value no other datum
        // takes, and lifted to `v`; every other datum to its point.
        const MARK: f64 = 0.5;
        let line = NurbsCurve2::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point2::new(0.2, 0.0), Point2::new(0.8, MARK)],
            vec![1.0; 2],
        )
        .unwrap();
        let at = |v: Interval| {
            line.map_scalar(|x| {
                if x == MARK {
                    v
                } else {
                    Interval::from_bounds(x, x)
                }
            })
        };
        let within = (super::end_reach(1e-9, 1.0), super::end_reach(1e-9, 1.0));
        assert!(super::along_side(
            &at(Interval::from_bounds(0.0, 0.0)),
            domain,
            within
        ));
        let refused = geom_core::test_support::refused_point(0.0);
        assert!(!super::along_side(&at(refused), domain, within));
    }

    /// **A carrier along a chart's side refuses by name, and only where
    /// the walk is undecided.** The wall `(u, v) ↦ (0, u, v)` and the
    /// floor `(u, v) ↦ (2v − 1, u, 0)` meet along the wall's `v = 0` row,
    /// the state `(s, 0, s, ½)`: a face of the box holds the whole locus.
    /// Told the wall's image lies along a side, the undecided walk names
    /// it; told nothing, it stays undecided. The crossing of
    /// [`an_end_on_a_domain_side_is_reached_by_the_root_there`]'s walls,
    /// told the same, still proves its arc: a proof is never named away.
    #[test]
    fn a_carrier_along_a_charts_side_refuses_by_name() {
        let (wall, floor) = (
            bilinear(|i, j| [0.0, i, j]),
            bilinear(|i, j| [2.0 * j - 1.0, i, 0.0]),
        );
        let domain = Box4::of_charts(&wall, &floor);
        let ends = [([0.25, 0.0, 0.25, 0.5], 0), ([0.75, 0.0, 0.75, 0.5], 0)];
        for eps in [1e-6, 1e-9, 1e-12] {
            let walk =
                |along| one_arc_pair(&boxed(&wall, &floor), (&[domain], domain, along), ends, eps);
            assert_eq!(
                walk(None),
                Err(Shortfall::Undecided),
                "unnamed at ε = {eps:e}"
            );
            assert_eq!(
                walk(Some(0)),
                Err(Shortfall::AlongSide(0)),
                "named at ε = {eps:e}"
            );
        }
        let (a, b) = walls();
        let domain = Box4::of_charts(&a, &b);
        let ends = [([0.5, 0.0, 0.5, 0.25], 3), ([0.5, 1.0, 0.5, 0.75], 3)];
        assert_eq!(
            one_arc_pair(&boxed(&a, &b), (&[domain], domain, Some(0)), ends, 1e-9),
            Ok(()),
            "a proved arc stays proved"
        );
    }

    /// **A boundary root is narrowed to the arithmetic's width.** The
    /// trough `z = (x − ½)²` cut by the plane `z = 0.04 + 0.05·y`, the
    /// plane's chart sheared so that no root coordinate is a round
    /// number: the face walk accepts a root's piece wherever the
    /// Krawczyk image falls inside it, and over a curved pair that image
    /// is wide; narrowing meets it with its own image until it holds
    /// still, every root kept.
    #[test]
    fn a_boundary_root_is_narrowed_to_the_arithmetics_width() {
        let k = |(a, b): (f64, f64)| KnotVector::clamped(vec![a, a, a, b, b, b], 2).unwrap();
        let net = |(t0, t1): (f64, f64), at: &dyn Fn(f64, f64) -> [f64; 3]| {
            let t = |n: usize| t0 + 0.5 * (t1 - t0) * f64::from(u8::try_from(n).unwrap());
            let control = (0..9)
                .map(|n| {
                    let [x, y, z] = at(t(n / 3), t(n % 3));
                    Point3::new(x, y, z)
                })
                .collect();
            NurbsSurface::new(k((t0, t1)), k((t0, t1)), control, vec![1.0; 9]).unwrap()
        };
        // The trough's net: `z` at its three columns is `(x − ½)²`'s
        // Bernstein form.
        let trough = net((0.0, 1.0), &|x, y| {
            [x, y, [0.25, -0.25, 0.25][(2.0 * x).round() as usize]]
        });
        let plane = net((-0.2, 1.2), &|u, v| {
            let (x, y) = (u + 0.1 * v, v + 0.1 * u);
            [x, y, 0.04 + 0.05 * y]
        });
        let pair = boxed(&trough, &plane);
        let roots = face_roots(&pair, Box4::of_charts(&trough, &plane), PAIR_EXIT_PIECES)
            .expect("the domain's boundary resolves");
        assert_eq!(roots.len(), 4, "two branches, each crossing two sides");
        let metres = |q: Box4| {
            (0..4)
                .map(|i| (q.0[i].1 - q.0[i].0) * pair.speeds[i])
                .fold(0.0, f64::max)
        };
        let mut widest = 0.0f64;
        for &(q, face, _) in &roots {
            let n = narrow(&pair, (q, face));
            assert!(
                n.contained_in(q),
                "the narrowed root {n:?} lies in its box {q:?}"
            );
            assert!(metres(n) <= 1e-13, "{q:?} narrows only to {n:?}");
            widest = widest.max(metres(q));
        }
        assert!(widest > 1e-9, "a root the walk left wide: {widest:e} m");
    }
}

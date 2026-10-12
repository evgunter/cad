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

use geom::NurbsSurface;
use geom_core::interval::certification::Certification;
use geom_core::{Bounds, CertifiedBounds, Interval};

use super::enclose::{Box3, NurbsBoxes, UvWindow, refused_box};
use super::one_arc::{EXIT_CUT, EXIT_DEPTH, Shortfall};
use super::section::sign;

/// A box in the ℝ⁴ state space: `[lo, hi]` per coordinate, the first
/// chart's `u, v` then the second's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Box4(pub(crate) [(f64, f64); 4]);

impl Box4 {
    /// The product of `a`'s and `b`'s knot rectangles: the whole state
    /// space of their section.
    pub(crate) fn of_charts<T: CertifiedBounds>(a: &NurbsSurface<T>, b: &NurbsSurface<T>) -> Self {
        Self([
            a.knots_u().domain(),
            a.knots_v().domain(),
            b.knots_u().domain(),
            b.knots_v().domain(),
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
    a: NurbsBoxes<'a, T>,
    b: NurbsBoxes<'a, T>,
    /// The chart speeds `(u₁, v₁, u₂, v₂)`.
    pub(crate) speeds: [f64; 4],
}

impl<'a, T: CertifiedBounds> PairBoxes<'a, T> {
    /// The pair, with its four chart speeds.
    pub(crate) fn new(a: &'a NurbsSurface<T>, b: &'a NurbsSurface<T>, speeds: [f64; 4]) -> Self {
        Self {
            a: NurbsBoxes::new(a),
            b: NurbsBoxes::new(b),
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
        self.jacobian_read(x, true)
    }

    /// The Jacobian, each net cut to `x` or read whole.
    fn jacobian_read(&self, x: Box4, cut: bool) -> [[Interval; 4]; 3] {
        let col = |b: Box3, s: f64| {
            [
                b.x * Interval::point(s),
                b.y * Interval::point(s),
                b.z * Interval::point(s),
            ]
        };
        let read = |n: &NurbsBoxes<'_, T>, w: Option<UvWindow>, u: bool| {
            w.map_or_else(refused_box, |w| {
                if cut {
                    n.deriv_box(w, u)
                } else {
                    n.cell_deriv_box(w, u)
                }
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

/// The determinant of a 3×3 interval matrix.
pub(crate) fn det3(m: [[Interval; 3]; 3]) -> Interval {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
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
    let mut inside = true;
    let mut misses = false;
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
        let s = x.0[free[r]];
        inside &= v.lo() > s.0 && v.hi() < s.1;
        misses |= v.hi() < s.0 || v.lo() > s.1;
        k_box.0[free[r]] = (v.lo(), v.hi());
    }
    if inside {
        Some(Krawczyk::One(k_box))
    } else if misses {
        Some(Krawczyk::None)
    } else {
        None
    }
}

/// The inverse of a 3×3 matrix, `None` where it is singular or not
/// finite.
fn inverse3(m: [[f64; 3]; 3]) -> Option<[[f64; 3]; 3]> {
    let c =
        |r0: usize, r1: usize, c0: usize, c1: usize| m[r0][c0] * m[r1][c1] - m[r0][c1] * m[r1][c0];
    let det = m[0][0] * c(1, 2, 1, 2) - m[0][1] * c(1, 2, 0, 2) + m[0][2] * c(1, 2, 0, 1);
    if !det.is_finite() || det == 0.0 {
        return None;
    }
    let inv = [
        [c(1, 2, 1, 2), -c(0, 2, 1, 2), c(0, 1, 1, 2)],
        [-c(1, 2, 0, 2), c(0, 2, 0, 2), -c(0, 1, 0, 2)],
        [c(1, 2, 0, 1), -c(0, 2, 0, 1), c(0, 1, 0, 1)],
    ];
    let out = inv.map(|row| row.map(|v| v / det));
    out.iter().flatten().all(|v| v.is_finite()).then_some(out)
}

/// The solutions on the face `x` held at coordinate `k`, pushed onto
/// `roots` (module docs). `None` when a piece resolves no way within
/// [`EXIT_DEPTH`] cuts, or `pieces` passes `budget`.
fn face_walk<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    (x, k): (Box4, usize),
    roots: &mut Vec<Box4>,
    (pieces, budget): (&mut u32, u32),
) -> Option<()> {
    let free: [usize; 3] = match k {
        0 => [1, 2, 3],
        1 => [0, 2, 3],
        2 => [0, 1, 3],
        _ => [0, 1, 2],
    };
    if !x.is_box() {
        return None;
    }
    let mut stack = vec![(x, 0u32)];
    while let Some((x, depth)) = stack.pop() {
        *pieces += 1;
        if *pieces > budget {
            return None;
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
            return None;
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
    Some(())
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
    Some(out)
}

/// The near reading of an end ([`reaches_end`]).
enum Near {
    Found,
    Far,
    Unknown,
}

/// **The box `r`'s piece reaches the carrier's end `at`**, `k` the
/// coordinate the carrier's state runs most along there, in metres.
/// - The slice of `r` through the end across `k` holds a solution.
/// - Or a box about the end, its half-width `eps/8` metres at each
///   axis's chart speed, cut to `r`, holds one on its boundary.
///
/// The box is r3's ball of diameter `eps` (`one_arc::r3_reaches_end`,
/// half-width `eps/(2√3)` less the end's width) read in chart units: a
/// chart's speed bounds its surface's motion along each of its two
/// axes, so a point of the box lands within `eps/8 + eps/8 = eps/4` of
/// the end's image on either chart, inside that ball with room. The end
/// is a point, so no width of its own is taken off.
///
/// [`Shortfall::Short`] only where both are certified not so; an end
/// whose slice lies outside `r` along `k` is [`Shortfall::Undecided`].
fn reaches_end<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    r: Box4,
    (at, k): ([f64; 4], usize),
    eps: f64,
) -> Result<(), Shortfall> {
    let c = at[k];
    if !(r.0[k].0 <= c && c <= r.0[k].1) {
        return Err(Shortfall::Undecided);
    }
    let mut roots = Vec::new();
    let sliced = face_walk(
        pair,
        (r.holding(k, c), k),
        &mut roots,
        (&mut 0, PAIR_EXIT_PIECES),
    );
    if !roots.is_empty() {
        return Ok(());
    }
    let h: [f64; 4] = core::array::from_fn(|i| eps / 8.0 / pair.speeds[i]);
    let ball = Box4::at(at).pad(h);
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
/// # Errors
///
/// The first [`Shortfall`] met, boxes in order, then links in order,
/// then the start and the end.
pub(crate) fn one_arc_pair<T: CertifiedBounds>(
    pair: &PairBoxes<'_, T>,
    (boxes, domain): (&[Box4], Box4),
    ends: [([f64; 4], usize); 2],
    eps: f64,
) -> Result<(), Shortfall> {
    let cut = boxes
        .iter()
        .map(|b| b.meet(domain))
        .collect::<Option<Vec<Box4>>>()
        .ok_or(Shortfall::Undecided)?;
    let mut roots = Vec::with_capacity(cut.len());
    for r in &cut {
        match face_roots(pair, *r, PAIR_EXIT_PIECES) {
            Some(v) if v.len() == 2 => roots.push(v),
            #[allow(clippy::cast_possible_truncation)]
            Some(v) => return Err(Shortfall::Count(v.len() as u32)),
            None => return Err(Shortfall::Undecided),
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
    let (Some(&first), Some(&last)) = (cut.first(), cut.last()) else {
        return Err(Shortfall::Undecided);
    };
    reaches_end(pair, first, ends[0], eps)?;
    reaches_end(pair, last, ends[1], eps)
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
    let mut j = pair.jacobian(x);
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

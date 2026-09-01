//! **Tensor-product coefficient nets** — the two-dimensional companion
//! of [`super::hull`]'s scalar coefficient lines: a rectangular grid of
//! ring enclosures, its per-direction derivative assembly, and the
//! window hull that reads a bound off it.
//!
//! # Why this is here and not in a consumer
//!
//! A tensor patch's partials are themselves tensor-product B-splines
//! whose coefficient nets come from knot differencing **per direction**
//! (The NURBS Book Eq. 3.24 — [`super::hull::derivative_coeffs`],
//! iterated across lines of the net). That iteration — transpose, apply
//! the one-dimensional step down each line, scatter the result back —
//! is the same operation for every consumer, and it had been written
//! out three times in the tree with three storage shapes.
//!
//! What is NOT here is any consumer's **reading** of the nets. A hull
//! over a cell's active window is here because every consumer wants
//! exactly that; a recentred `A − c·w` hull, a quotient-rule
//! recurrence, a de Boor collapse and a per-span constant ladder are
//! each one consumer's own arithmetic and stay with it.
//!
//! # The differencing step is a parameter, not a fixed formula
//!
//! [`TensorNet::diff_u`] and [`TensorNet::diff_v`] take the
//! one-dimensional step as a closure. A caller whose direction is a
//! clamped [`KnotVector`] passes [`TensorNet::diff_u_knots`] /
//! [`TensorNet::diff_v_knots`], which is
//! [`super::hull::derivative_coeffs`]. A caller carrying a direction
//! the clamped invariant cannot spell — a derivative whose interior
//! multiplicity equals the parent degree, so it is genuinely
//! discontinuous — passes its own step and keeps its own structure.
//! The tensor bookkeeping is what is shared; the one-dimensional
//! formula is what is not.
//!
//! # A step that does not answer the direction's new extent is an ERROR
//!
//! [`TensorNet::diff_u`] and [`TensorNet::diff_v`] shrink one direction
//! by exactly one, so a step handed a line of `n` coefficients owes
//! `n - 1`. Anything else — short or long — is a structure error in the
//! caller, and this module **poisons that whole line** rather than
//! padding it or truncating it.
//!
//! Both spellings were tried and both are wrong. **Padding** a short
//! answer with zeros turns one refusal into one poison plus `n - 2`
//! finite zeros, which is a finite bound over a window nothing covered
//! — the failure this module exists to make impossible, and the same
//! shape [`super::hull::sup_norm_bound_span`] refuses one dimension
//! down. **Truncating** a long answer hides a caller bug behind a
//! plausible net. Poisoning the line is the only answer that reaches a
//! consumer as a refusal.
//!
//! *Measured reachability, so this is a guard and not a story about
//! one:* neither shipped caller can trip it.
//! [`super::hull::derivative_coeffs`] answers `n - 1` for every line
//! its knot vector admits, and short-answers only on a
//! coefficient-count mismatch; `geom_brep::props::quad`'s `raw_deriv`
//! answers `n - 1` whenever its degree is at least 1, which its own
//! `Dir` construction guarantees, and it fills a DEGENERATE (empty)
//! span itself with an explicit zero rather than by returning fewer
//! coefficients. So this is a latent-bug guard on a path no caller
//! reaches today, and it is written that way rather than as a fill
//! policy the callers choose between.
//!
//! # Poison (fail-loud, D4 ¶2)
//!
//! Every out-of-range read is [`RingInterval::poison`]; a shape that
//! does not multiply out is a poisoned net rather than a panic or a
//! truncation. Nothing here compares anything.

use core::ops::RangeInclusive;

use super::hull::derivative_coeffs;
use super::knots::KnotVector;
use crate::ring_interval::RingInterval;

/// A rectangular tensor coefficient net of ring enclosures, stored
/// **row-major** (`u`-major): entry `(i, j)` — `u` index `i`, `v` index
/// `j` — lives at `i * nv + j`.
///
/// The layout is the one a control net already has
/// (`NurbsSurface::control()`), so a net built from one is a map, not
/// a transpose. Both access shapes are first-class:
/// [`TensorNet::row`] hands out a `v`-line as a borrowed slice,
/// [`TensorNet::column`] materialises a `u`-line, and
/// [`TensorNet::as_flat`] hands back the whole thing for a consumer
/// that indexes it itself.
#[derive(Clone, Debug)]
pub struct TensorNet {
    nu: usize,
    nv: usize,
    c: Vec<RingInterval>,
}

impl TensorNet {
    /// A net from a row-major coefficient vector.
    ///
    /// A length that is not `nu * nv` is a shape error, and it yields a
    /// net of the DECLARED extent filled with poison rather than a
    /// short one: a caller reading a bound off it gets poison, where a
    /// silently-truncated net would answer a finite bound over a window
    /// it never covered.
    #[must_use]
    pub fn from_flat(nu: usize, nv: usize, c: Vec<RingInterval>) -> Self {
        if c.len() == nu.saturating_mul(nv) {
            Self { nu, nv, c }
        } else {
            Self::poisoned(nu, nv)
        }
    }

    /// A net of the given extent, every entry poison.
    #[must_use]
    pub fn poisoned(nu: usize, nv: usize) -> Self {
        Self {
            nu,
            nv,
            c: vec![RingInterval::poison(); nu.saturating_mul(nv)],
        }
    }

    /// A net from `u`-major nested rows (`rows[i][j]`). A ragged input
    /// is a shape error and poisons, per [`TensorNet::from_flat`].
    #[must_use]
    pub fn from_rows(rows: &[Vec<RingInterval>]) -> Self {
        let nu = rows.len();
        let nv = rows.first().map_or(0, Vec::len);
        if rows.iter().any(|r| r.len() != nv) {
            return Self::poisoned(nu, nv);
        }
        Self {
            nu,
            nv,
            c: rows.iter().flat_map(|r| r.iter().copied()).collect(),
        }
    }

    /// A net built entrywise from its indices.
    #[must_use]
    pub fn from_fn(nu: usize, nv: usize, f: impl Fn(usize, usize) -> RingInterval) -> Self {
        let mut c = Vec::with_capacity(nu.saturating_mul(nv));
        for i in 0..nu {
            for j in 0..nv {
                c.push(f(i, j));
            }
        }
        Self { nu, nv, c }
    }

    /// The `u` extent (number of `u` indices).
    #[must_use]
    pub fn nu(&self) -> usize {
        self.nu
    }

    /// The `v` extent (number of `v` indices).
    #[must_use]
    pub fn nv(&self) -> usize {
        self.nv
    }

    /// Whether the net holds no coefficients at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.c.is_empty()
    }

    /// The whole net, row-major.
    #[must_use]
    pub fn as_flat(&self) -> &[RingInterval] {
        &self.c
    }

    /// Entry `(i, j)`; out of range is poison.
    #[must_use]
    pub fn get(&self, i: usize, j: usize) -> RingInterval {
        if i >= self.nu || j >= self.nv {
            return RingInterval::poison();
        }
        self.c
            .get(i * self.nv + j)
            .copied()
            .unwrap_or_else(RingInterval::poison)
    }

    /// The `v`-line at `u` index `i`, borrowed. Out of range is empty —
    /// a caller differencing it gets the step's own answer for an empty
    /// line, which the fill then covers.
    #[must_use]
    pub fn row(&self, i: usize) -> &[RingInterval] {
        if i >= self.nu {
            return &[];
        }
        let base = i * self.nv;
        self.c.get(base..base + self.nv).unwrap_or(&[])
    }

    /// The `u`-line at `v` index `j`, materialised (the layout stores
    /// it strided). Out-of-range entries are poison.
    #[must_use]
    pub fn column(&self, j: usize) -> Vec<RingInterval> {
        (0..self.nu).map(|i| self.get(i, j)).collect()
    }

    /// **The signed hull of the net over a window**, `wu × wv` — the
    /// bound a consumer reads off one cell's active coefficients.
    ///
    /// Fixed association (D9): accumulated `u`-major, `i` outer and `j`
    /// inner, hulling left to right from the first entry. An empty
    /// window and an out-of-range index are both poison.
    #[must_use]
    pub fn window_hull(
        &self,
        wu: &RangeInclusive<usize>,
        wv: &RangeInclusive<usize>,
    ) -> RingInterval {
        let mut acc: Option<RingInterval> = None;
        for i in wu.clone() {
            for j in wv.clone() {
                let e = self.get(i, j);
                acc = Some(match acc {
                    None => e,
                    Some(h) => RingInterval::hull(h, e),
                });
            }
        }
        acc.unwrap_or_else(RingInterval::poison)
    }

    /// The hull of the WHOLE net — [`TensorNet::window_hull`] over
    /// every index. The coarser reading a whole-patch consumer takes,
    /// spelled once so it is visibly the same assembly as the per-cell
    /// one rather than a second differencing.
    #[must_use]
    pub fn hull(&self) -> RingInterval {
        if self.nu == 0 || self.nv == 0 {
            return RingInterval::poison();
        }
        self.window_hull(&(0..=self.nu - 1), &(0..=self.nv - 1))
    }

    /// **Differences the net once along `u`**: the step is applied to
    /// each `u`-line (one per `v` index) and the results scattered back
    /// into a `(nu − 1) × nv` net.
    ///
    /// A step that answers anything but `nu − 1` coefficients poisons
    /// that whole line (module docs — it is a caller structure error,
    /// and neither padding nor truncating it can be sound).
    ///
    /// A net with fewer than two `u` indices has no `u` derivative and
    /// yields the empty net.
    #[must_use]
    pub fn diff_u(&self, step: impl Fn(&[RingInterval]) -> Vec<RingInterval>) -> Self {
        let nu1 = self.nu.saturating_sub(1);
        if nu1 == 0 || self.nv == 0 {
            return Self::from_flat(nu1, self.nv, Vec::new());
        }
        let mut c = vec![RingInterval::poison(); nu1 * self.nv];
        for j in 0..self.nv {
            let d = step(&self.column(j));
            if d.len() != nu1 {
                continue;
            }
            for (i, q) in d.iter().enumerate() {
                if let Some(slot) = c.get_mut(i * self.nv + j) {
                    *slot = *q;
                }
            }
        }
        Self {
            nu: nu1,
            nv: self.nv,
            c,
        }
    }

    /// **Differences the net once along `v`**: the step is applied to
    /// each `v`-line (one per `u` index), yielding `nu × (nv − 1)`. A
    /// step that answers anything but `nv − 1` coefficients poisons
    /// that line ([`TensorNet::diff_u`]).
    #[must_use]
    pub fn diff_v(&self, step: impl Fn(&[RingInterval]) -> Vec<RingInterval>) -> Self {
        let nv1 = self.nv.saturating_sub(1);
        if nv1 == 0 || self.nu == 0 {
            return Self::from_flat(self.nu, nv1, Vec::new());
        }
        let mut c = vec![RingInterval::poison(); self.nu * nv1];
        for i in 0..self.nu {
            let d = step(self.row(i));
            if d.len() != nv1 {
                continue;
            }
            for (j, q) in d.iter().enumerate() {
                if let Some(slot) = c.get_mut(i * nv1 + j) {
                    *slot = *q;
                }
            }
        }
        Self {
            nu: self.nu,
            nv: nv1,
            c,
        }
    }

    /// [`TensorNet::diff_u`] with the clamped-knot-vector step
    /// ([`super::hull::derivative_coeffs`]): a line this vector does
    /// not admit yields a poisoned derivative rather than a widened
    /// one, because that step short-answers and the line poisons.
    #[must_use]
    pub fn diff_u_knots(&self, kv: &KnotVector) -> Self {
        self.diff_u(|c| derivative_coeffs(kv, c))
    }

    /// [`TensorNet::diff_v`] with the clamped-knot-vector step
    /// ([`TensorNet::diff_u_knots`]).
    #[must_use]
    pub fn diff_v_knots(&self, kv: &KnotVector) -> Self {
        self.diff_v(|c| derivative_coeffs(kv, c))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn pt(x: f64) -> RingInterval {
        RingInterval::point(x)
    }

    /// The two constructors agree, and the layout is `u`-major.
    #[test]
    fn rows_and_flat_agree() {
        let rows = vec![vec![pt(1.0), pt(2.0)], vec![pt(3.0), pt(4.0)]];
        let a = TensorNet::from_rows(&rows);
        let b = TensorNet::from_flat(2, 2, vec![pt(1.0), pt(2.0), pt(3.0), pt(4.0)]);
        assert_eq!((a.nu(), a.nv()), (b.nu(), b.nv()));
        for (x, y) in a.as_flat().iter().zip(b.as_flat()) {
            assert!(x.lo() == y.lo() && x.hi() == y.hi());
        }
        assert_eq!(a.get(1, 0).lo(), 3.0);
        assert_eq!(a.row(1)[1].lo(), 4.0);
        assert_eq!(a.column(0)[1].lo(), 3.0);
    }

    /// A shape that does not multiply out poisons rather than
    /// truncating: the bound it yields fails every comparison.
    #[test]
    fn a_bad_shape_poisons() {
        let n = TensorNet::from_flat(2, 3, vec![pt(1.0)]);
        assert!(n.hull().is_poison());
        let ragged = TensorNet::from_rows(&[vec![pt(1.0)], vec![pt(1.0), pt(2.0)]]);
        assert!(ragged.hull().is_poison());
    }

    /// Differencing a bilinear net along each direction, against the
    /// hand-computed answer: on the unit segment the derivative
    /// coefficient is `p·(c1 − c0)/Δu` with `p = 1`, `Δu = 1`.
    #[test]
    fn diff_matches_the_knot_difference() {
        let kv = KnotVector::unit_segment(1);
        let n = TensorNet::from_rows(&[vec![pt(0.0), pt(1.0)], vec![pt(2.0), pt(5.0)]]);
        // The ring rounds outward, so each answer is ENCLOSED, not
        // equalled (D4 ¶2: a bound, never an estimate).
        let holds = |iv: RingInterval, x: f64| iv.lo() <= x && x <= iv.hi();
        let du = n.diff_u_knots(&kv);
        assert_eq!((du.nu(), du.nv()), (1, 2));
        assert!(holds(du.get(0, 0), 2.0) && holds(du.get(0, 1), 4.0));
        let dv = n.diff_v_knots(&kv);
        assert_eq!((dv.nu(), dv.nv()), (2, 1));
        assert!(holds(dv.get(0, 0), 1.0) && holds(dv.get(1, 0), 3.0));
        // Mixed: the tensor collapse commutes — both orders enclose
        // the same `d^2/dudv = 2`.
        assert!(holds(du.diff_v_knots(&kv).get(0, 0), 2.0));
        assert!(holds(dv.diff_u_knots(&kv).get(0, 0), 2.0));
    }

    /// A step that does not answer the direction's new extent poisons
    /// the line — short OR long, because neither padding nor
    /// truncating can be sound (module docs).
    #[test]
    fn a_step_of_the_wrong_length_poisons_its_line() {
        let n = TensorNet::from_rows(&[vec![pt(1.0)], vec![pt(2.0)], vec![pt(3.0)]]);
        // Owes 2 coefficients per u-line.
        let nothing = |_: &[RingInterval]| Vec::new();
        let short = |_: &[RingInterval]| vec![pt(9.0)];
        let long = |_: &[RingInterval]| vec![pt(9.0), pt(9.0), pt(9.0)];
        let right = |_: &[RingInterval]| vec![pt(9.0), pt(8.0)];
        for bad in [n.diff_u(nothing), n.diff_u(short), n.diff_u(long)] {
            assert_eq!((bad.nu(), bad.nv()), (2, 1));
            assert!(bad.get(0, 0).is_poison() && bad.get(1, 0).is_poison());
        }
        let ok = n.diff_u(right);
        assert!(!ok.get(0, 0).is_poison() && ok.get(0, 0).lo() == 9.0);
    }

    /// The window hull is the hull of exactly the window, and the
    /// whole-net hull is the window hull over everything.
    #[test]
    fn window_hull_reads_the_window() {
        let n = TensorNet::from_rows(&[vec![pt(1.0), pt(-4.0)], vec![pt(2.0), pt(3.0)]]);
        let h = n.window_hull(&(0..=0), &(0..=0));
        assert_eq!((h.lo(), h.hi()), (1.0, 1.0));
        let all = n.hull();
        assert_eq!((all.lo(), all.hi()), (-4.0, 3.0));
        assert!(n.window_hull(&(0..=2), &(0..=0)).is_poison());
    }
}

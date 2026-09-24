//! **Tensor-product coefficient nets** — the two-dimensional companion
//! of [`super::hull`]'s scalar coefficient lines: a rectangular grid of
//! certification enclosures, its per-direction derivative assembly, and the
//! window hull that reads a bound off it.
//!
//! # Why this is here and not in a consumer
//!
//! A tensor patch's partials are themselves tensor-product B-splines
//! whose coefficient nets come from knot differencing **per direction**
//! (The NURBS Book Eq. 3.24 — [`super::hull::SplineCoeffs::derivative_coeffs`],
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
//! [`super::hull::SplineCoeffs::derivative_coeffs`]. A caller carrying a direction
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
//! caller, and this module **refuses that whole line** rather than
//! padding it or truncating it.
//!
//! Both spellings were tried and both are wrong. **Padding** a short
//! answer with zeros turns one refusal into one refused entry plus `n - 2`
//! finite zeros, which is a finite bound over a window nothing covered
//! — the failure this module exists to make impossible, and the same
//! shape the [`super::hull`] mint refuses for one dimension down when
//! the coefficient count disagrees. **Truncating** a long answer hides a caller bug behind a
//! plausible net. Refusing the line is the only answer that reaches a
//! consumer as a refusal.
//!
//! *Measured reachability, so this is a guard and not a story about
//! one:* neither shipped caller can trip it.
//! [`super::hull::SplineCoeffs::derivative_coeffs`] answers `n - 1` for
//! every line its knot vector admits, and the step short-answers only
//! when the mint refuses a coefficient-count mismatch; `geom_brep::props::quad`'s `raw_deriv`
//! answers `n - 1` whenever its degree is at least 1, which its own
//! `Dir` construction guarantees, and it fills a DEGENERATE (empty)
//! span itself with an explicit zero rather than by returning fewer
//! coefficients. So this is a latent-bug guard on a path no caller
//! reaches today, and it is written that way rather than as a fill
//! policy the callers choose between.
//!
//! # Refusal (fail-loud, D4 ¶2)
//!
//! Every out-of-range read is [`Interval::refused`]; a shape that
//! does not multiply out is a refused net rather than a panic or a
//! truncation. Nothing here compares anything.

use core::ops::RangeInclusive;

use super::algebra::CurvePlan;
use super::knots::KnotVector;
use crate::interval::Interval;
use crate::interval::certification::Certification;

/// A rectangular tensor coefficient net of certification enclosures, stored
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
    c: Vec<Interval>,
}

impl TensorNet {
    /// A net from a row-major coefficient vector.
    ///
    /// A length that is not `nu * nv` is a shape error, and it yields a
    /// net of the DECLARED extent filled with refusals rather than a
    /// short one: a caller reading a bound off it gets a refusal, where a
    /// silently-truncated net would answer a finite bound over a window
    /// it never covered.
    #[must_use]
    pub fn from_flat(nu: usize, nv: usize, c: Vec<Interval>) -> Self {
        if c.len() == nu.saturating_mul(nv) {
            Self { nu, nv, c }
        } else {
            Self::refused(nu, nv)
        }
    }

    /// A net of the given extent, every entry refused.
    #[must_use]
    pub fn refused(nu: usize, nv: usize) -> Self {
        Self {
            nu,
            nv,
            c: vec![Interval::refused(); nu.saturating_mul(nv)],
        }
    }

    /// A net from `u`-major nested rows (`rows[i][j]`). A ragged input
    /// is a shape error and refuses, per [`TensorNet::from_flat`].
    #[must_use]
    pub fn from_rows(rows: &[Vec<Interval>]) -> Self {
        let nu = rows.len();
        let nv = rows.first().map_or(0, Vec::len);
        if rows.iter().any(|r| r.len() != nv) {
            return Self::refused(nu, nv);
        }
        Self {
            nu,
            nv,
            c: rows.iter().flat_map(|r| r.iter().copied()).collect(),
        }
    }

    /// A net built entrywise from its indices.
    #[must_use]
    pub fn from_fn(nu: usize, nv: usize, f: impl Fn(usize, usize) -> Interval) -> Self {
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
    pub fn as_flat(&self) -> &[Interval] {
        &self.c
    }

    /// Entry `(i, j)`; out of range is refused.
    #[must_use]
    pub fn get(&self, i: usize, j: usize) -> Interval {
        if i >= self.nu || j >= self.nv {
            return Interval::refused();
        }
        self.c
            .get(i * self.nv + j)
            .copied()
            .unwrap_or_else(Interval::refused)
    }

    /// The `v`-line at `u` index `i`, borrowed. Out of range is empty —
    /// a caller differencing it gets the step's own answer for an empty
    /// line, which the fill then covers.
    #[must_use]
    pub fn row(&self, i: usize) -> &[Interval] {
        if i >= self.nu {
            return &[];
        }
        let base = i * self.nv;
        self.c.get(base..base + self.nv).unwrap_or(&[])
    }

    /// The `u`-line at `v` index `j`, materialised (the layout stores
    /// it strided). Out-of-range entries are refused.
    #[must_use]
    pub fn column(&self, j: usize) -> Vec<Interval> {
        (0..self.nu).map(|i| self.get(i, j)).collect()
    }

    /// **The signed hull of the net over a window**, `wu × wv` — the
    /// bound a consumer reads off one cell's active coefficients.
    ///
    /// Fixed association (D9): accumulated `u`-major, `i` outer and `j`
    /// inner, hulling left to right from the first entry. An empty
    /// window and an out-of-range index are both refused.
    #[must_use]
    pub fn window_hull(&self, wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>) -> Interval {
        let mut acc: Option<Interval> = None;
        for i in wu.clone() {
            for j in wv.clone() {
                let e = self.get(i, j);
                acc = Some(match acc {
                    None => e,
                    Some(h) => Interval::hull(h, e),
                });
            }
        }
        acc.unwrap_or_else(Interval::refused)
    }

    /// The hull of the WHOLE net — [`TensorNet::window_hull`] over
    /// every index. The coarser reading a whole-patch consumer takes,
    /// spelled once so it is visibly the same assembly as the per-cell
    /// one rather than a second differencing.
    #[must_use]
    pub fn hull(&self) -> Interval {
        if self.nu == 0 || self.nv == 0 {
            return Interval::refused();
        }
        self.window_hull(&(0..=self.nu - 1), &(0..=self.nv - 1))
    }

    /// **Differences the net once along `u`**: the step is applied to
    /// each `u`-line (one per `v` index) and the results scattered back
    /// into a `(nu − 1) × nv` net.
    ///
    /// A step that answers anything but `nu − 1` coefficients refuses
    /// that whole line (module docs — it is a caller structure error,
    /// and neither padding nor truncating it can be sound).
    ///
    /// A net with fewer than two `u` indices has no `u` derivative and
    /// yields the empty net.
    #[must_use]
    pub fn diff_u(&self, step: impl Fn(&[Interval]) -> Vec<Interval>) -> Self {
        let nu1 = self.nu.saturating_sub(1);
        if nu1 == 0 || self.nv == 0 {
            return Self::from_flat(nu1, self.nv, Vec::new());
        }
        let mut c = vec![Interval::refused(); nu1 * self.nv];
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
    /// step that answers anything but `nv − 1` coefficients refuses
    /// that line ([`TensorNet::diff_u`]).
    #[must_use]
    pub fn diff_v(&self, step: impl Fn(&[Interval]) -> Vec<Interval>) -> Self {
        let nv1 = self.nv.saturating_sub(1);
        if nv1 == 0 || self.nu == 0 {
            return Self::from_flat(self.nu, nv1, Vec::new());
        }
        let mut c = vec![Interval::refused(); self.nu * nv1];
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
    /// ([`KnotVector::difference_coeffs`]): a line this vector does
    /// not admit — one the mint refuses — yields a refused derivative
    /// rather than a widened one, because the step then short-answers
    /// (one refused entry, never zero) and the line is refused.
    #[must_use]
    pub fn diff_u_knots(&self, kv: &KnotVector) -> Self {
        self.diff_u(|c| kv.difference_coeffs(c))
    }

    /// [`TensorNet::diff_v`] with the clamped-knot-vector step
    /// ([`TensorNet::diff_u_knots`]).
    #[must_use]
    pub fn diff_v_knots(&self, kv: &KnotVector) -> Self {
        self.diff_v(|c| kv.difference_coeffs(c))
    }

    /// **Refines the net along `u` IN INTERVAL ARITHMETIC**: the insertion chain is
    /// applied to each `u`-line by [`CurvePlan::apply_ring`], so the
    /// answer ENCLOSES the refined net of the described coefficients
    /// instead of being a rounded copy of it. That is the difference
    /// between a bound on the patch a caller described and a bound on
    /// the one `f64` refinement happened to produce.
    ///
    /// The net must be HOMOGENEOUS for this to mean what it says — the
    /// weight net `w`, or one channel of `w·P` — because that is the
    /// form in which insertion is the plain affine combination interval arithmetic
    /// applier takes.
    ///
    /// One schedule for every line: a Boehm step's targets, sources and
    /// ratio come from the knot structure alone, which the `u` direction
    /// shares across all `nv` lines. An empty chain is the identity.
    ///
    /// **A net whose extent the schedule was not built for REFUSES**, at
    /// the new extent, and the guard is on the extent rather than on the
    /// answer's length because the answer's length cannot report it.
    /// [`CurvePlan::apply_ring`] answers its own plan's control count
    /// whatever it is handed — that is what makes it total — so a line
    /// LONGER than the schedule expects comes back the right length with
    /// its tail silently dropped, which is a hull over fewer
    /// coefficients than the net has and therefore too NARROW. A shorter
    /// line refuses on its own, through the missing sources. Neither is
    /// reachable from a caller that built the schedule from the same
    /// direction it is refining, and both are refused rather than
    /// argued: an insertion chain adds exactly one coefficient per plan,
    /// so `nu_new == nu + plans.len()` is the whole test.
    #[must_use]
    pub fn refine_u(&self, plans: &[CurvePlan]) -> Self {
        let Some(last) = plans.last() else {
            return self.clone();
        };
        let nu_new = last.knots().control_count();
        if nu_new != self.nu + plans.len() {
            return Self::refused(nu_new, self.nv);
        }
        let mut c = vec![Interval::refused(); nu_new * self.nv];
        for j in 0..self.nv {
            let mut line = self.column(j);
            for plan in plans {
                line = plan.apply_ring(&line);
            }
            for (i, q) in line.iter().enumerate() {
                if let Some(slot) = c.get_mut(i * self.nv + j) {
                    *slot = *q;
                }
            }
        }
        Self {
            nu: nu_new,
            nv: self.nv,
            c,
        }
    }

    /// [`TensorNet::refine_u`] along `v`, per `v`-line, with the same
    /// extent guard.
    #[must_use]
    pub fn refine_v(&self, plans: &[CurvePlan]) -> Self {
        let Some(last) = plans.last() else {
            return self.clone();
        };
        let nv_new = last.knots().control_count();
        if nv_new != self.nv + plans.len() {
            return Self::refused(self.nu, nv_new);
        }
        let mut c = vec![Interval::refused(); self.nu * nv_new];
        for i in 0..self.nu {
            let mut line = self.row(i).to_vec();
            for plan in plans {
                line = plan.apply_ring(&line);
            }
            for (j, q) in line.iter().enumerate() {
                if let Some(slot) = c.get_mut(i * nv_new + j) {
                    *slot = *q;
                }
            }
        }
        Self {
            nu: self.nu,
            nv: nv_new,
            c,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::real::Bounds;

    fn pt(x: f64) -> Interval {
        Interval::point(x)
    }

    /// The refinement schedule for one direction: every nonempty span of
    /// `kv` cut into `splits` equal pieces.
    fn chain(kv: &KnotVector, splits: usize) -> Vec<crate::spline::CurvePlan> {
        let mut add = Vec::new();
        for span in kv.first_span()..=kv.last_span() {
            if !kv.span_is_nonempty(span) {
                continue;
            }
            let (lo, hi) = (kv.knots()[span], kv.knots()[span + 1]);
            for k in 1..splits {
                #[allow(clippy::cast_precision_loss)]
                let t = lo + (hi - lo) * (k as f64 / splits as f64);
                if t > lo && t < hi {
                    add.push(t);
                }
            }
        }
        crate::spline::algebra::refine_plan_homogeneous(kv, &add).unwrap()
    }

    /// **Ring refinement is the per-line chain, scattered back** — each
    /// direction's own claim, and the one a transposed index would break.
    /// Refining along `u` must answer, at every `v` index, exactly what
    /// the chain answers for that column read on its own; refining along
    /// `v` the same for each row. An empty chain is the identity in both.
    #[test]
    fn refinement_is_the_per_line_chain_in_each_direction() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        // Distinct per-slot values, so a transposed scatter cannot pass.
        #[allow(clippy::cast_precision_loss)]
        let net = TensorNet::from_fn(nu, nv, |i, j| pt((i * 10 + j) as f64));
        let plans_u = chain(&kv_u, 3);
        let refined_u = net.refine_u(&plans_u);
        assert_eq!(refined_u.nv(), nv);
        assert_eq!(
            refined_u.nu(),
            plans_u.last().unwrap().knots().control_count()
        );
        for j in 0..nv {
            let mut want = net.column(j);
            for plan in &plans_u {
                want = plan.apply_ring(&want);
            }
            let got = refined_u.column(j);
            assert_eq!(got.len(), want.len());
            for (i, (g, w)) in got.iter().zip(&want).enumerate() {
                assert!(
                    g.lo().to_bits() == w.lo().to_bits() && g.hi().to_bits() == w.hi().to_bits(),
                    "u-refined ({i}, {j}) = [{:.17e}, {:.17e}], the column's own chain \
                     gives [{:.17e}, {:.17e}]",
                    g.lo(),
                    g.hi(),
                    w.lo(),
                    w.hi()
                );
            }
        }
        let plans_v = chain(&kv_v, 4);
        let refined_v = net.refine_v(&plans_v);
        assert_eq!(refined_v.nu(), nu);
        assert_eq!(
            refined_v.nv(),
            plans_v.last().unwrap().knots().control_count()
        );
        for i in 0..nu {
            let mut want = net.row(i).to_vec();
            for plan in &plans_v {
                want = plan.apply_ring(&want);
            }
            let got = refined_v.row(i);
            assert_eq!(got.len(), want.len());
            for (j, (g, w)) in got.iter().zip(&want).enumerate() {
                assert!(
                    g.lo().to_bits() == w.lo().to_bits() && g.hi().to_bits() == w.hi().to_bits(),
                    "v-refined ({i}, {j}) disagrees with the row's own chain"
                );
            }
        }
        // An empty chain is the identity, bitwise, in both directions.
        for identity in [net.refine_u(&[]), net.refine_v(&[])] {
            assert_eq!((identity.nu(), identity.nv()), (nu, nv));
            for (a, b) in identity.as_flat().iter().zip(net.as_flat()) {
                assert_eq!(a.lo().to_bits(), b.lo().to_bits());
            }
        }
    }

    /// **A net whose extent the schedule was not built for comes back
    /// REFUSED at the new extent** — never short, and never silently
    /// finite over coefficients that are not this net's. The direction
    /// that needed the guard is the LONG one: the applier answers its
    /// plan's extent from whatever prefix it can source, so a long line
    /// would otherwise refine as if its tail did not exist, giving a hull
    /// too narrow rather than too wide.
    #[test]
    fn refining_a_net_of_the_wrong_extent_refuses_rather_than_answers() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let plans = chain(&kv, 2);
        let n_new = plans.last().unwrap().knots().control_count();
        // The schedule is for 3 coefficients per line. BOTH directions of
        // mismatch: 5 would be silently truncated by the applier, 2 would
        // refuse through its missing sources. Neither may answer finitely.
        for extent in [5usize, 2] {
            let net =
                TensorNet::from_fn(extent, 2, |i, _| pt(f64::from(u32::try_from(i).unwrap())));
            let refined = net.refine_u(&plans);
            assert_eq!((refined.nu(), refined.nv()), (n_new, 2));
            assert!(
                refined.as_flat().iter().all(|r| !r.is_certified()),
                "a {extent}-coefficient line refined by a 3-coefficient schedule answered \
                 a finite slot — a hull over coefficients that are not this net's"
            );
        }
        // And the same guard on the other direction.
        let net = TensorNet::from_fn(2, 5, |_, j| pt(f64::from(u32::try_from(j).unwrap())));
        let refined = net.refine_v(&plans);
        assert_eq!((refined.nu(), refined.nv()), (2, n_new));
        assert!(refined.as_flat().iter().all(|r| !r.is_certified()));
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

    /// A shape that does not multiply out refuses rather than
    /// truncating: the bound it yields fails every comparison.
    #[test]
    fn a_bad_shape_refuses() {
        let n = TensorNet::from_flat(2, 3, vec![pt(1.0)]);
        assert!(!n.hull().is_certified());
        let ragged = TensorNet::from_rows(&[vec![pt(1.0)], vec![pt(1.0), pt(2.0)]]);
        assert!(!ragged.hull().is_certified());
    }

    /// Differencing a bilinear net along each direction, against the
    /// hand-computed answer: on the unit segment the derivative
    /// coefficient is `p·(c1 − c0)/Δu` with `p = 1`, `Δu = 1`.
    #[test]
    fn diff_matches_the_knot_difference() {
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let n = TensorNet::from_rows(&[vec![pt(0.0), pt(1.0)], vec![pt(2.0), pt(5.0)]]);
        // Interval arithmetic rounds outward, so each answer is ENCLOSED, not
        // equalled (D4 ¶2: a bound, never an estimate).
        let holds = |iv: Interval, x: f64| iv.lo() <= x && x <= iv.hi();
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

    /// A step that does not answer the direction's new extent refuses
    /// the line — short OR long, because neither padding nor
    /// truncating can be sound (module docs).
    #[test]
    fn a_step_of_the_wrong_length_refuses_its_line() {
        let n = TensorNet::from_rows(&[vec![pt(1.0)], vec![pt(2.0)], vec![pt(3.0)]]);
        // Owes 2 coefficients per u-line.
        let nothing = |_: &[Interval]| Vec::new();
        let short = |_: &[Interval]| vec![pt(9.0)];
        let long = |_: &[Interval]| vec![pt(9.0), pt(9.0), pt(9.0)];
        let right = |_: &[Interval]| vec![pt(9.0), pt(8.0)];
        for bad in [n.diff_u(nothing), n.diff_u(short), n.diff_u(long)] {
            assert_eq!((bad.nu(), bad.nv()), (2, 1));
            assert!(!bad.get(0, 0).is_certified() && !bad.get(1, 0).is_certified());
        }
        let ok = n.diff_u(right);
        assert!(ok.get(0, 0).is_certified() && ok.get(0, 0).lo() == 9.0);
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
        assert!(!n.window_hull(&(0..=2), &(0..=0)).is_certified());
    }

    /// A member that divided by an exact zero is EMPTY — decoration
    /// `Trv`, NaN endpoints — and reaches a net through the public
    /// constructor like any other coefficient. The backend's hull treats
    /// the empty set as an identity and would answer the other members'
    /// hull, certified, so a refused coefficient would vanish from the
    /// bound; the net's hulls refuse instead, on either side of the fold.
    #[test]
    fn an_empty_member_refuses_every_hull_it_is_in() {
        let empty = Interval::from_bounds(1.0, 2.0) / pt(0.0);
        assert!(
            !empty.is_certified() && empty.lo().is_nan() && empty.hi().is_nan(),
            "fixture drifted: {empty:?}"
        );
        let n = TensorNet::from_flat(
            1,
            3,
            vec![
                Interval::from_bounds(1.0, 2.0),
                empty,
                Interval::from_bounds(3.0, 4.0),
            ],
        );
        assert!(!n.hull().is_certified(), "{:?}", n.hull());
        assert!(!n.window_hull(&(0..=0), &(0..=1)).is_certified());
        assert!(!n.window_hull(&(0..=0), &(1..=2)).is_certified());
        // The control: the window that does not hold it certifies, so the
        // rows above pin the refused member and not a net that fails on
        // everything.
        let clean = n.window_hull(&(0..=0), &(2..=2));
        assert!(clean.is_certified() && (clean.lo(), clean.hi()) == (3.0, 4.0));
    }
}

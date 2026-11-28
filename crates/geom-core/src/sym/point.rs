//! **The point reading**: the refuter a decision asks before the symbolic
//! tier (ERROR-DESIGN E12, "the two channels are checked where an answer
//! is returned").
//!
//! The tier can answer only `Zero`: a residual form that is not the zero
//! polynomial proves nothing, because its atoms are unknown functions that
//! may still cancel under rules the ladder has not tried. So an ask on a
//! margin that is not an identity climbs every rung and every retry and
//! ends where it began. This module answers the question the tier cannot:
//! **is the margin certainly no identity over the box?**
//!
//! # What is read
//!
//! The margin's expression graph, op by op, in the outward-rounded
//! [`Interval`](crate::interval::Interval), with each document parameter at ONE point of the bracket
//! the session recorded for it ([`Sym::param_over`](crate::sym::Sym::param_over)). Each node's
//! reading is kept for the leaf's session (`Session::points`), so
//! a sub-graph several margins share is read once; nothing outlives the
//! leaf.
//!
//! # Why an enclosure that excludes 0 refutes
//!
//! A `Zero` the tier proves is a theorem: the margin's real value is 0 at
//! every point of the box at which the build takes the decisions it took
//! (E12). The point read here is a point of the box, and the reading
//! encloses the margin's real value there, node by node:
//!
//! - a parameter is the point itself, a literal its own `f64`, `π` its
//!   rounded bracket, and every ring op and function is `Interval`'s
//!   certified enclosure of the real op;
//! - a `select`, `min` or `max` is the construction's own decision: at the
//!   point, `select` takes the arm its decision's reading certifies there
//!   and the hull of both arms where that reading straddles, and `min` /
//!   `max` take the envelope of their arms, which holds whichever the
//!   construction takes ([`Real::select_le_zero`](crate::Real::select_le_zero), [`Real::min`](crate::Real::min) at
//!   `Interval`). An arm the reading does not take is not read;
//! - a value whose expression the graph does not hold declines the whole
//!   reading: an opaque value, a hull (a function of enclosures, not of
//!   the reals they stand for), a `SymOp::Free` variable (the
//!   mark of [`crate::Decide::free_identity`], whose number is a measured
//!   bound and not its expression's value), a node this session's table
//!   does not hold, and a parameter with no finite bracket;
//! - and a reading that lost its domain at the point (decoration below
//!   `Def`: an empty or ill-formed enclosure among them) declines rather
//!   than refutes, because there the margin has no real value to enclose.
//!
//! So an enclosure that excludes 0 is a point of the box where the margin
//! is not 0, and no rung could have proved it zero. The decision is then
//! answered as a climb that proved nothing answers it: the numeric
//! reading, undecided. Where the reading holds 0 or declines, the tier is
//! asked as before.
//!
//! The structural question ([`crate::Expression::proves_zero`]) never
//! takes this reading: E12 makes its answer a function of the graph
//! alone, and a reading is a function of the box.

use super::{IdMap, SESSION, Session, SymId, SymOp, indet_param};
use crate::interval::Interval;
use crate::real::{Bounds, Real};

/// **The point a parameter is read at**: `lo + f·(hi − lo)` with `f` a
/// fraction of its own, a fixed function of the parameter's symbol, in
/// `[0.15, 0.35]` or `[0.65, 0.85]`. `None` for a bracket that is not
/// finite and ordered.
///
/// The fraction is never the centre and never a face, because that is
/// where a document puts the values a designed coincidence holds at: a
/// root box is centred on the nominal, and bisection puts the nominal on
/// a child's face, so a margin that vanishes there would read 0 at the
/// point and climb on every leaf. It differs per symbol so that two
/// parameters over equal brackets are not read at one value, where
/// `x − y` would read 0. A function of the symbol and the bracket alone,
/// so the reading is the same in every replay and on every schedule.
fn point_of(symbol: u64, lo: f64, hi: f64) -> Option<f64> {
    if !(lo.is_finite() && hi.is_finite() && lo <= hi) {
        return None;
    }
    // SplitMix64's finalizer: a fixed bijection of the symbol, so nearby
    // ids land on unrelated fractions.
    let mut z = symbol.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    let u = (z >> 11) as f64 / (1u64 << 53) as f64;
    let f = 0.15 + 0.2 * u;
    let f = if z & 1 == 1 { 1.0 - f } else { f };
    Some((lo + f * (hi - lo)).clamp(lo, hi))
}

/// **Whether the point reading refutes `id`**: its enclosure at the
/// leaf's point is certified and excludes 0. `false` outside a session,
/// where there is no bracket to read, at a zero-term budget, where the
/// tier proves nothing and the ask costs nothing, and where a memo
/// already holds the root's plain form as the zero form, which the
/// tier's first rung answers from that memo and no reading can refute.
///
/// The memo check is what the reading costs on an identity it cannot
/// refute: on `demo-tour certified` with the switch on, about half of
/// the 1.4 M asks that reach here are roots a memo already holds as
/// zero (measured by the lane that built this; the PR records the
/// tour's time with and without it).
pub(super) fn refutes(id: SymId) -> bool {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        slot.as_mut().is_some_and(|sess| {
            if sess.budget.max_terms == 0 || plain_is_zero(sess, id) {
                return false;
            }
            reading(sess, id).is_some_and(|x| Bounds::lo(x) > 0.0 || Bounds::hi(x) < 0.0)
        })
    })
}

/// Whether this leaf's plain memo, or the drive's, already holds `id`'s
/// plain form and it is the zero form.
fn plain_is_zero(sess: &Session, id: SymId) -> bool {
    match sess.forms.get(&id) {
        Some(f) => f.is_zero(),
        None => sess
            .memo
            .as_ref()
            .and_then(|m| m.form(id))
            .is_some_and(|f| f.is_zero()),
    }
}

/// `id`'s certified enclosure at the leaf's point, `None` where the
/// reading declines (the module docs list when).
fn reading(sess: &mut Session, id: SymId) -> Option<Interval> {
    let mut memo = core::mem::take(&mut sess.points);
    let out = evaluate(sess, &mut memo, id, &|symbol| {
        let &(lo, hi) = sess.params.get(&indet_param(symbol))?;
        point_of(symbol, lo, hi).map(Interval::from_f64)
    });
    sess.points = memo;
    out.filter(|x| x.is_certified())
}

/// **The graph evaluated at a scalar**, op by op as `Sym<T>` performs
/// it, into `memo` (one value per node, `None` for a node with no
/// value): the one walk the point reading and the re-valuation
/// instrument (`sym::revalue`, at `f64`) share. A parameter is `param`
/// of its symbol; an opaque value, a hull, a free variable and a node
/// no table holds have no value; a quotient (`Mul(a, Inv(b))`) is read
/// as `a / b`, so a graph that IS the computation re-values to its own
/// bits at `f64`. A `select` reads its decision first and then only the
/// arm that decision takes (both where it straddles), so an arm the
/// computation does not take at this value cannot leave it without
/// one. Without recursion, since a margin's graph can be deep.
pub(super) fn evaluate<T: Real + Bounds>(
    sess: &Session,
    memo: &mut IdMap<Option<T>>,
    root: SymId,
    param: &dyn Fn(u64) -> Option<T>,
) -> Option<T> {
    let mut stack = vec![(root, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        let Some(n) = sess.node_of(id) else {
            memo.insert(id, None);
            continue;
        };
        if n.op == SymOp::Select && !expanded {
            match memo.get(&n.kids[0]) {
                None => {
                    stack.push((id, false));
                    stack.push((n.kids[0], false));
                }
                Some(d) => {
                    stack.push((id, true));
                    for &k in select_arms(*d) {
                        stack.push((n.kids[k], false));
                    }
                }
            }
            continue;
        }
        if !expanded {
            stack.push((id, true));
            stack.extend(n.kids().iter().map(|k| (*k, false)));
            continue;
        }
        let kid = |i: usize| memo.get(&n.kids[i]).copied().flatten();
        // A closure, so that a child with no value leaves THIS node with
        // none (memoised) rather than returning from the walk.
        let v = (|| match n.op {
            SymOp::Param => param(n.payload),
            SymOp::Opaque | SymOp::Hull | SymOp::Free => None,
            SymOp::Lit => Some(T::from_f64(f64::from_bits(n.payload))),
            SymOp::Pi => Some(T::pi()),
            SymOp::Add => Some(kid(0)? + kid(1)?),
            SymOp::Sub => Some(kid(0)? - kid(1)?),
            SymOp::Mul => match sess.node_of(n.kids[1]) {
                Some(inv) if inv.op == SymOp::Inv => {
                    Some(kid(0)? / memo.get(&inv.kids[0]).copied().flatten()?)
                }
                _ => Some(kid(0)? * kid(1)?),
            },
            SymOp::Neg => Some(-kid(0)?),
            SymOp::Powi => Some(kid(0)?.powi(n.payload as u32 as i32)),
            SymOp::Inv => Some(T::one() / kid(0)?),
            SymOp::Sqrt => Some(kid(0)?.sqrt()),
            SymOp::Abs => Some(kid(0)?.abs()),
            SymOp::Sin => Some(kid(0)?.sin_cos().0),
            SymOp::Cos => Some(kid(0)?.sin_cos().1),
            SymOp::Tan => Some(kid(0)?.tan()),
            SymOp::Asin => Some(kid(0)?.asin()),
            SymOp::Acos => Some(kid(0)?.acos()),
            SymOp::Atan => Some(kid(0)?.atan()),
            SymOp::Floor => Some(kid(0)?.floor()),
            SymOp::LessSin => Some(kid(0)?.less_sin()),
            SymOp::Atan2 => Some(kid(0)?.atan2(kid(1)?)),
            SymOp::Min => Some(kid(0)?.min(kid(1)?)),
            SymOp::Max => Some(kid(0)?.max(kid(1)?)),
            SymOp::Copysign => Some(kid(0)?.copysign(kid(1)?)),
            SymOp::Select => kid(0).and_then(|d| match *select_arms(Some(d)) {
                [k] => Some(d.select_le_zero(kid(k)?, kid(k)?)),
                [a, b] => Some(d.select_le_zero(kid(a)?, kid(b)?)),
                _ => None,
            }),
        })();
        memo.insert(id, v);
    }
    memo.get(&root).copied().flatten()
}

/// The child slots of a `select` its decision's value `d` takes:
/// `when_le` where `d ≤ 0` holds over all of `d`, `when_gt` where
/// `d > 0` does, both otherwise (a straddling enclosure, or a `NaN`),
/// and none where the decision has no value. One rule for the walk and
/// the evaluation.
fn select_arms<T: Bounds>(d: Option<T>) -> &'static [usize] {
    match d {
        None => &[],
        Some(d) if Bounds::hi(d) <= 0.0 => &[1],
        Some(d) if Bounds::lo(d) > 0.0 => &[2],
        Some(_) => &[1, 2],
    }
}

/// **What [`check`] read**: the instrument's receipt.
#[cfg(feature = "sym-revalue-testing")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Refuted {
    /// Distinct margins of undecided decisions the point reading refutes,
    /// each climbed anyway.
    pub refuted: usize,
    /// Of them, every one the ladder proved zero: a margin nonzero at a
    /// point of the box that the tier calls an identity is a kernel
    /// defect (an algebra, enclosure or registrant one), and each entry
    /// is `(predicate, node)`.
    pub zeros: Vec<(&'static str, SymId)>,
}

/// **The test instrument for the refuter** (DESIGN `[simple.checks]`: no
/// shipped build contains it): every decision in `decisions` the numbers
/// left undecided and the point reading refutes, asked of the whole
/// ladder anyway, the door included, inside the session that took them.
/// `None` outside a session or at a zero-term budget. Asked last, as
/// [`contradiction::check`](crate::sym::contradiction::check) is and for its reason: its walks are in
/// the session's memos.
#[cfg(feature = "sym-revalue-testing")]
#[must_use]
pub fn check(decisions: &[super::revalue::Decision]) -> Option<Refuted> {
    use crate::predicate::Sign;
    let undecided: Vec<_> = decisions
        .iter()
        .filter(|d| !d.proven && !matches!(d.sign, Some(Sign::Positive | Sign::Negative)))
        .collect();
    let mut asked = std::collections::HashSet::new();
    let mut out = Refuted::default();
    for d in undecided {
        if !asked.insert(d.node) || !refutes(d.node) {
            continue;
        }
        out.refuted += 1;
        let zero = SESSION.with(|s| {
            let mut slot = s.borrow_mut();
            let sess = slot.as_mut()?;
            if sess.budget.max_terms == 0 {
                return None;
            }
            sess.instrumented = true;
            let counts = sess.node.counts;
            let at = super::ladder(sess, d.node, super::Path::Decision);
            sess.node.counts = counts;
            Some(at.is_some())
        });
        match zero {
            None => return None,
            Some(true) => out.zeros.push((d.predicate, d.node)),
            Some(false) => {}
        }
    }
    Some(out)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::super::{ParamSymbol, Sym, SymBudget, SymCounts, SymRules, with_session_rules};
    use super::*;
    use crate::predicate::{Band, Margin, Sign};
    use crate::spline::SpanLocate;
    use crate::tolerance::Tol;

    type S = Sym<Interval>;

    fn budget() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    fn symbol(name: &str) -> u64 {
        test_utils::symbol_id(name)
    }

    /// `name` over `[lo, hi]`, its bracket recorded.
    fn param(name: &str, lo: f64, hi: f64) -> S {
        Sym::param_over(
            ParamSymbol::new(symbol(name)),
            Interval::from_bounds(lo, hi),
            lo,
            hi,
        )
    }

    fn lit(x: f64) -> S {
        S::from_f64(x)
    }

    /// The first of `x0, x1, …` read below the centre of `[-1, 1]`, and
    /// where: the rows that want a negative point name it.
    fn below_centre() -> (String, f64) {
        (0..)
            .map(|k| format!("x{k}"))
            .find_map(|n| {
                let p = point_of(symbol(&n), -1.0, 1.0)?;
                (p < 0.0).then_some((n, p))
            })
            .expect("a symbol reads below the centre")
    }

    /// The forms this session built, every memo counted.
    fn forms_built() -> usize {
        SESSION.with(|s| {
            s.borrow().as_ref().map_or(0, |s| {
                s.forms.len()
                    + s.forms_early.len()
                    + s.node.forms_door.len()
                    + s.node.retries.iter().map(|m| m.len()).sum::<usize>()
            })
        })
    }

    /// `m` decided in the installed session: its sign where one was
    /// returned, and the forms the decision built.
    fn asked(m: S) -> (Option<Sign>, usize) {
        let before = forms_built();
        let band = Band::linear(Tol::witness()).expect("a linear band");
        let sign = crate::k_stats::decide("point_test", Margin::of(m), band).ok();
        (sign, forms_built() - before)
    }

    fn shipped<R>(f: impl FnOnce() -> R) -> (R, SymCounts) {
        with_session_rules(budget(), SymRules::shipped(), f)
    }

    /// What the leaf's point memo holds for `id`: `None` where it was
    /// never read, `Some(None)` where it declined.
    fn memo_of(id: SymId) -> Option<Option<Interval>> {
        SESSION.with(|s| s.borrow().as_ref()?.points.get(&id).copied())
    }

    /// How many nodes the leaf's point memo holds.
    fn memo_len() -> usize {
        SESSION.with(|s| s.borrow().as_ref().map_or(0, |s| s.points.len()))
    }

    /// `id`'s reading at the leaf's point, as the refuter takes it.
    fn read_now(id: SymId) -> Option<Interval> {
        SESSION.with(|s| reading(s.borrow_mut().as_mut().expect("a session"), id))
    }

    /// **A nonzero margin undecided over the box is refuted with no form
    /// built.** `x² − 0.01` straddles over `[-1, 1]` and reads at least
    /// `0.15² · 4 − 0.01 > 0` at any point the rule picks. Red on main,
    /// which climbs it.
    #[test]
    fn a_nonzero_margin_undecided_over_the_box_builds_no_form() {
        let ((sign, forms), counts) = shipped(|| {
            let x = param("x", -1.0, 1.0);
            asked(x * x - lit(0.01))
        });
        assert_eq!(forms, 0, "refuted margins build no form");
        assert_ne!(sign, Some(Sign::Zero), "and decide nothing: {counts:?}");
        assert_eq!((counts.symbolic, counts.numeric), (0, 1), "{counts:?}");
    }

    /// **An identity still proves `Zero`, and a reading is per box.**
    /// `max(x, −x) − x` is `|x| − x` spelled through the construction's
    /// decision (`Real::abs` is no decision, and the tier proves nothing
    /// through it: `a_root_and_a_magnitude_over_a_signed_bracket_are_not_folded`).
    /// Over `[-1, 1]` the point reads below zero, where the margin is
    /// `2|x|`: refuted, no form. Over a sub-box past the tie,
    /// `[0.25, 1]`, the decision is certified and the tier proves it zero
    /// (over `[0, 1]` it does not: the decision read reads `≤ 0` as `< 0`
    /// at a tie, `signed::decision`). Red under the mutant that refutes
    /// on any certified reading (zero included): the sub-box's zero is
    /// lost.
    #[test]
    fn an_identity_still_proves_zero_and_its_parent_box_is_refuted() {
        let (name, _) = below_centre();
        let m = |x: S| Real::max(x, -x) - x;
        let ((sign, forms), counts) = shipped(|| asked(m(param(&name, -1.0, 1.0))));
        assert_eq!(forms, 0, "over [-1, 1]: refuted ({counts:?})");
        assert_ne!(sign, Some(Sign::Zero));
        let ((sign, _), counts) = shipped(|| asked(m(param(&name, 0.25, 1.0))));
        assert_eq!(sign, Some(Sign::Zero), "over [0.25, 1]: an identity");
        assert_eq!(counts.symbolic, 1, "{counts:?}");
    }

    /// **A margin that vanishes at the point climbs as before.** Red
    /// under the mutant that refutes where the reading only touches 0.
    #[test]
    fn a_margin_that_vanishes_at_the_point_climbs() {
        let p = point_of(symbol("x"), -1.0, 1.0).unwrap();
        let ((sign, forms), _) = shipped(|| asked(param("x", -1.0, 1.0) - lit(p)));
        assert!(forms > 0, "the tier is asked");
        assert_ne!(sign, Some(Sign::Zero), "and proves nothing");
    }

    /// **A `select` takes, at the point, the arm its decision reads
    /// there.** With `x` at a negative point, `select(x, ·, ·)` takes
    /// `when_le`: `select(x, 1, x − p)` is refuted by its `1`, and
    /// `select(x, x − p, 1)` reads its `x − p`, which is 0 there, and
    /// climbs. The second is red under the mutant that takes `when_gt`
    /// (an unsound refutation); the first under the mutant that hulls
    /// every select; the third, whose untaken arm must stay out of the
    /// memo, under the mutant that walks both arms.
    #[test]
    fn a_select_takes_the_arm_its_decision_reads_at_the_point() {
        let (name, p) = below_centre();
        let ((taken, forms), _) = shipped(|| {
            let x = param(&name, -1.0, 1.0);
            asked(x.select_le_zero(lit(1.0), x - lit(p)))
        });
        assert_eq!(forms, 0, "the arm it takes reads 1: refuted");
        assert_ne!(taken, Some(Sign::Zero));
        let ((_, forms), _) = shipped(|| {
            let x = param(&name, -1.0, 1.0);
            asked(x.select_le_zero(x - lit(p), lit(1.0)))
        });
        assert!(forms > 0, "the arm it takes reads 0 at the point: asked");
        let ((forms, unread), _) = shipped(|| {
            let x = param(&name, -1.0, 1.0);
            let gt = x * lit(7.0) - lit(0.5);
            let (_, forms) = asked(x.select_le_zero(lit(1.0), gt));
            (forms, memo_of(gt.node()))
        });
        assert_eq!(forms, 0, "the arm it takes reads 1: refuted");
        assert!(unread.is_none(), "the arm it does not take is not read");
    }

    /// **A `select` whose decision straddles at the point reads the hull
    /// of both arms.** `π − fl(π)` straddles at every point; the hull of
    /// `x − p` (0 at the point) and `1` holds 0, and the tier is asked.
    /// Red under the mutant that takes either arm alone.
    #[test]
    fn a_select_that_straddles_at_the_point_reads_both_arms() {
        let (name, p) = below_centre();
        let ((_, forms), _) = shipped(|| {
            let x = param(&name, -1.0, 1.0);
            let d = S::pi() - lit(core::f64::consts::PI);
            asked(d.select_le_zero(x - lit(p), lit(1.0)))
        });
        assert!(forms > 0, "the hull holds 0: asked");
        let ((_, forms), _) = shipped(|| {
            let x = param(&name, -1.0, 1.0);
            let d = S::pi() - lit(core::f64::consts::PI);
            asked(d.select_le_zero(x - lit(p) + lit(0.05), lit(0.5)))
        });
        assert_eq!(forms, 0, "the hull excludes 0: refuted");
    }

    /// **Values with no expression decline**: a `free_identity` margin,
    /// read by the `SymOp::Free` it mints (its number is a measured bound,
    /// not its expression's value; the variable is minted here as the
    /// door mints it, the door's one caller being limb 2), an
    /// opaque value, a hull and a parameter with no bracket each read
    /// nothing, and the tier is asked as before. The row asserts the
    /// reading DECLINED (the memo holds no value for the margin), so it
    /// is red under the mutant that reads the value as 0 (refuted) and
    /// under one that reads it as any enclosure, 0 held or not.
    #[test]
    fn values_with_no_expression_decline() {
        type Row = (&'static str, fn() -> S);
        let rows: [Row; 4] = [
            ("free identity", || {
                S::nullary(Interval::from_bounds(-1.0, 1.0), SymOp::Free, 0) + lit(0.5)
            }),
            ("opaque", || {
                S::opaque(Interval::from_bounds(-1.0, 1.0)) + lit(0.5)
            }),
            ("hull", || {
                let x = param("x", -1.0, 1.0);
                x.enclosure_hull(x) + lit(0.5)
            }),
            ("unbracketed", || {
                S::param(
                    ParamSymbol::new(symbol("x")),
                    Interval::from_bounds(-1.0, 1.0),
                ) + lit(0.5)
            }),
        ];
        for (what, m) in rows {
            let ((forms, held), _) = shipped(|| {
                let m = m();
                let (_, forms) = asked(m);
                (forms, memo_of(m.node()))
            });
            assert!(
                matches!(held, Some(None)),
                "{what}: the reading declined: {held:?}"
            );
            assert!(forms > 0, "{what}: and the tier is asked");
        }
    }

    /// **A reading that loses its domain at the point declines.** An
    /// enclosure with no real in it reads `NaN` ends, which exclude
    /// nothing; the edge is a reading whose ends are finite and whose
    /// domain is not certified: `π − fl(π)⁺` (the next `f64` above π) is
    /// negative, and its enclosure ends at 0, so its root reads a point at
    /// `Trv`, and `+ 1` reads a finite bracket clear of 0. (Over a box the value
    /// channel reads the same violation, so no decision asks; the
    /// reading is asked here.) Red under the mutant that drops the
    /// certification check.
    #[test]
    fn a_reading_with_a_lost_domain_declines() {
        let (out, _) = shipped(|| {
            let x = param("x", -1.0, 1.0);
            let lost = (S::pi() - lit(core::f64::consts::PI.next_up())).sqrt() + lit(1.0);
            let none = (x - lit(2.0)).sqrt() + lit(1.0);
            (refutes(lost.node()), refutes(none.node()))
        });
        assert_eq!(out, (false, false));
    }

    /// **The answer is a function of the graph and the box.** Three
    /// margins sharing a sub-graph read the same in either order within
    /// a leaf, and on another thread's session; and a reading taken
    /// before a parameter's bracket is recorded does not outlive it (red
    /// under the mutant that keeps the memo across `param_over`).
    #[test]
    fn the_reading_is_the_same_in_any_order_and_on_any_thread() {
        fn margins() -> Vec<S> {
            let (x, y) = (param("x", -1.0, 1.0), param("y", 2.0, 3.0));
            let shared = x * x - lit(0.01);
            vec![shared * y, shared - x * y, (x * y).sin() - x]
        }
        let read = |order: &[usize]| {
            shipped(|| {
                let ms = margins();
                let mut out = vec![false; ms.len()];
                for &i in order {
                    out[i] = refutes(ms[i].node());
                }
                out
            })
            .0
        };
        let forward = read(&[0, 1, 2]);
        assert_eq!(forward, read(&[2, 1, 0]), "order within a leaf");
        let threaded = std::thread::spawn(move || read(&[1, 0, 2])).join().unwrap();
        assert_eq!(forward, threaded, "another thread's session");
        let (late, _) = shipped(|| {
            let sym = ParamSymbol::new(symbol("x"));
            let x = S::param(sym, Interval::from_bounds(-1.0, 1.0));
            let m = x * x - lit(0.01);
            let before = refutes(m.node());
            let _ = Sym::param_over(sym, Interval::from_bounds(-1.0, 1.0), -1.0, 1.0);
            (before, refutes(m.node()))
        });
        assert_eq!(
            late,
            (false, true),
            "declines unbracketed, refutes once bracketed"
        );
    }

    /// **The point is off the centre and the faces, and per symbol.**
    #[test]
    fn the_point_is_off_the_centre_and_the_faces_and_differs_per_symbol() {
        let names = ["x", "y", "w", "h", "r", "a", "b"];
        let points: Vec<f64> = names
            .iter()
            .map(|n| point_of(symbol(n), 0.0, 1.0).unwrap())
            .collect();
        for (n, &p) in names.iter().zip(&points) {
            assert!(
                (0.15..=0.35).contains(&p) || (0.65..=0.85).contains(&p),
                "{n}: {p}"
            );
        }
        let mut sorted = points.clone();
        sorted.sort_by(f64::total_cmp);
        sorted.dedup();
        assert_eq!(sorted.len(), points.len(), "{points:?}");
        assert_eq!(point_of(1, 1.0, 1.0), Some(1.0), "a point bracket");
        assert_eq!(point_of(1, f64::NEG_INFINITY, 1.0), None);
        assert_eq!(point_of(1, 2.0, 1.0), None, "reversed");
        assert_eq!(point_of(1, f64::NAN, 1.0), None);
        // At the edges the point can land on a face (`point_of`'s docs):
        // inside the bracket all the same, which is all soundness needs.
        for (lo, hi) in [
            (1.0, 1.0f64.next_up()),
            (-f64::MAX, f64::MAX),
            (0.0, f64::MIN_POSITIVE),
            (1e300, f64::MAX),
        ] {
            for n in names {
                let q = point_of(symbol(n), lo, hi).expect("finite and ordered");
                assert!(lo <= q && q <= hi, "[{lo:e}, {hi:e}] {n}: {q:e}");
            }
        }
    }

    /// **Every arm of the op table reads the op's own enclosure at the
    /// point.** Each row builds one op over `x` (and `y`) and compares the
    /// reading, bit for bit, with `Interval` performing that op on the
    /// point values. Red under any arm that reads another op: `Powi`
    /// through `|exponent|`, `Floor` as the identity, `Sin` for `Cos`, a
    /// `Div` read as a product. (`Inv` is minted only as a `Div`'s
    /// second factor, which the `Mul` arm reads as the quotient; its own
    /// arm is the `Div` row's.)
    #[test]
    fn every_arm_of_the_op_table_reads_its_op_at_the_point() {
        type Row = (
            &'static str,
            fn(S, S) -> S,
            fn(Interval, Interval) -> Interval,
        );
        let rows: [Row; 24] = [
            ("lit", |_, _| lit(0.3), |_, _| Interval::from_f64(0.3)),
            ("pi", |_, _| S::pi(), |_, _| Interval::pi()),
            ("add", |x, y| x + y, |x, y| x + y),
            ("sub", |x, y| x - y, |x, y| x - y),
            ("mul", |x, y| x * y, |x, y| x * y),
            ("div", |x, y| x / y, |x, y| x / y),
            ("neg", |x, _| -x, |x, _| -x),
            ("powi 3", |x, _| x.powi(3), |x, _| x.powi(3)),
            ("powi -2", |x, _| x.powi(-2), |x, _| x.powi(-2)),
            ("sqrt", |x, _| x.sqrt(), |x, _| x.sqrt()),
            ("abs", |x, _| (-x).abs(), |x, _| (-x).abs()),
            ("sin", |x, _| x.sin_cos().0, |x, _| x.sin_cos().0),
            ("cos", |x, _| x.sin_cos().1, |x, _| x.sin_cos().1),
            ("tan", |x, _| x.tan(), |x, _| x.tan()),
            ("asin", |x, _| x.asin(), |x, _| x.asin()),
            ("acos", |x, _| x.acos(), |x, _| x.acos()),
            ("atan", |x, _| x.atan(), |x, _| x.atan()),
            (
                "floor",
                |x, _| (x * lit(7.0)).floor(),
                |x, _| (x * Interval::from_f64(7.0)).floor(),
            ),
            ("less_sin", |x, _| x.less_sin(), |x, _| x.less_sin()),
            ("atan2", |x, y| x.atan2(-y), |x, y| x.atan2(-y)),
            ("min", |x, y| x.min(y), |x, y| x.min(y)),
            ("max", |x, y| x.max(y), |x, y| x.max(y)),
            ("copysign", |x, y| x.copysign(-y), |x, y| x.copysign(-y)),
            (
                "select",
                |x, y| (x - y).select_le_zero(x, y),
                |x, y| (x - y).select_le_zero(x, y),
            ),
        ];
        let at = |name: &str, lo: f64, hi: f64| {
            Interval::from_f64(point_of(symbol(name), lo, hi).unwrap())
        };
        let (px, py) = (at("x", 0.2, 0.9), at("y", 1.5, 2.5));
        for (what, built, op) in rows {
            let (read, _) = shipped(|| {
                let m = built(param("x", 0.2, 0.9), param("y", 1.5, 2.5));
                read_now(m.node())
            });
            let want = op(px, py);
            assert_eq!(
                read.map(Interval::repr_bits),
                Some(want.repr_bits()),
                "{what}: {read:?} against {want:?}"
            );
        }
    }

    /// **A bracket that MOVES clears the readings taken under the old
    /// one** (one reviewer's row). `max(x, −x) − x` is `−2x` at the point
    /// of `[-1, 1e-12]`, refuted; over `[0.25, 1]` it is an identity. The
    /// node is one node under both brackets, so a reading kept across the
    /// move refutes the identity. Red under the mutant that clears the
    /// memo only on a parameter's first bracket.
    #[test]
    fn a_moved_bracket_clears_the_readings() {
        let m = |x: S| x.max(-x) - x;
        let ((first, second), counts) = shipped(|| {
            let (first, _) = asked(m(param("x", -1.0, 1e-12)));
            let (second, _) = asked(m(param("x", 0.25, 1.0)));
            (first, second)
        });
        assert_ne!(first, Some(Sign::Zero), "refuted under the first bracket");
        assert_eq!(
            second,
            Some(Sign::Zero),
            "proven under the second: {counts:?}"
        );
        assert_eq!(counts.symbolic, 1, "{counts:?}");
    }

    /// **With the shape report installed, a refuted decision records its
    /// numeric outcome and renders nothing**: no form is built (the
    /// renderer builds the plain and early forms of what it renders) and
    /// its record has no form. Red under the mutant that renders before
    /// the refutation.
    #[test]
    fn a_refuted_decision_renders_no_form_for_the_shape_report() {
        let ((forms, shapes), _) = shipped(|| {
            super::super::report::start_shape_report();
            let x = param("x", -1.0, 1.0);
            let (_, forms) = asked(x * x - lit(0.01));
            (forms, super::super::report::take_shape_report())
        });
        assert_eq!(forms, 0, "no form built, the render's included");
        assert_eq!(shapes.len(), 1, "{shapes:?}");
        assert!(shapes[0].form.is_none(), "{:?}", shapes[0]);
    }

    /// **A sub-graph several margins share is read once per leaf**: after
    /// `s = x² − 0.01` is read, reading `s · y` adds two nodes to the
    /// memo (`y` and the product), not the sub-graph again. Red under the
    /// mutant that reads each margin into a fresh memo.
    #[test]
    fn a_shared_sub_graph_is_read_once() {
        let ((a, b, ab), _) = shipped(|| {
            let (x, y) = (param("x", -1.0, 1.0), param("y", 2.0, 3.0));
            let shared = x * x - lit(0.01);
            let _ = read_now(shared.node());
            let a = memo_len();
            let _ = read_now((shared * y).node());
            let b = memo_len();
            let _ = read_now((shared * y).node());
            (a, b, memo_len())
        });
        assert_eq!(b - a, 2, "only the new nodes are read");
        assert_eq!(ab, b, "a margin read is read once");
    }

    /// **A margin over nodes another leaf recorded is read, not
    /// declined**: the nodes are in the drive's table, which the reading
    /// resolves through. Leaf A mints `x² − 0.01`; leaf B, on another
    /// thread so that none of A's caches are its own, records `x`'s
    /// bracket and decides the margin A minted: refuted, no form. Red
    /// under the mutant that resolves nodes in the leaf's own cache
    /// alone.
    #[test]
    fn a_margin_another_leaf_recorded_is_read() {
        use super::super::{DriveMemo, with_session_memo};
        let table = std::sync::Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
        let table = &table;
        let ((cached, forms), _) = with_session_memo(budget(), SymRules::shipped(), table, || {
            let x = param("x", -1.0, 1.0);
            let m = x * x - lit(0.01);
            // Leaf B runs while A is open, so A's cache is not one it can
            // be handed.
            std::thread::scope(|s| {
                s.spawn(move || {
                    with_session_memo(budget(), SymRules::shipped(), table, || {
                        let _ = param("x", -1.0, 1.0);
                        let cached = SESSION.with(|s| {
                            s.borrow()
                                .as_ref()
                                .is_some_and(|s| s.nodes.contains_key(&m.node()))
                        });
                        (cached, asked(m).1)
                    })
                    .0
                })
                .join()
                .expect("leaf B ran")
            })
        });
        assert!(!cached, "leaf B does not hold the margin in its own cache");
        assert_eq!(forms, 0, "read through the drive's table and refuted");
    }

    /// **A dual lane reads through its inner scalar**: `Sym<Dual<Interval>>`
    /// decides at an exact witness, so the margin is refuted there as at
    /// `Sym<Interval>`. Red under the mutant in which a dual reads
    /// nothing.
    #[test]
    fn a_dual_lane_reads_at_its_exact_witness() {
        use crate::Dual;
        let ((sign, forms), _) = shipped(|| {
            let x = Sym::param_over(
                ParamSymbol::new(symbol("x")),
                Dual::variable(Interval::from_bounds(-1.0, 1.0)),
                -1.0,
                1.0,
            );
            let m = x * x - Sym::<Dual<Interval>>::from_f64(0.01);
            let before = forms_built();
            let band = Band::linear(Tol::witness()).expect("a linear band");
            let sign = crate::k_stats::decide("point_test", Margin::of(m), band).ok();
            (sign, forms_built() - before)
        });
        assert_eq!(forms, 0, "refuted at the dual's witness");
        assert_ne!(sign, Some(Sign::Zero));
    }

    /// **A refutation can move a LATER decision of the same scope, and
    /// only towards a proof** (one reviewer's row): the growth guard
    /// (`SymRetry::max_forms`, per node) withholds a retry attempt from a
    /// node whose attempt memos are at the cap, and a refuted ask builds
    /// no retry forms. At a cap of one form, `x² − 0.01` climbed first
    /// (main's behaviour) fills the cap, and the identity after it is
    /// left to the numeric channel's in-band `Zero`; refuted first, the
    /// identity is proven on the retry. Same sign either way; the
    /// symbolic count is what moves.
    #[test]
    fn a_refutation_moves_a_later_identity_through_the_growth_guard() {
        use super::super::{Path, SymRetry, discharge_in, with_session_retry};
        let c = || S::from_f64(1.0 / 3.0);
        let chain = |k: usize| (1..k).fold(c(), |acc, _| acc * c());
        let identity = move || chain(5) - chain(2) * chain(3);
        let band = || Band::linear(Tol::witness()).expect("band");
        let capped = SymRetry {
            bits: Some(512),
            max_forms: 1,
            ..SymRetry::none()
        };
        let run = |climb_first: bool| {
            with_session_retry(budget(), SymRules::shipped(), capped, || {
                let x = param("x", -1.0, 1.0);
                let m1 = x * x - lit(0.01);
                if climb_first {
                    let _ = discharge_in(m1.node(), Path::Decision);
                } else {
                    let _ = crate::k_stats::decide("probe_m1", Margin::of(m1), band());
                }
                crate::k_stats::decide("probe_id", Margin::of(identity()), band())
            })
        };
        let (head, hc) = run(false);
        let (main, mc) = run(true);
        assert_eq!((head, main), (Ok(Sign::Zero), Ok(Sign::Zero)));
        assert_eq!(
            (hc.symbolic, hc.retried),
            (1, 1),
            "refuted first: proven on the retry"
        );
        assert_eq!(
            (mc.symbolic, mc.retried),
            (0, 0),
            "climbed first: the retry is withheld"
        );
    }

    /// **A registered identity true over the box is not refuted**: the
    /// point reads it near 0, and the door proves it (one reviewer's
    /// row, at the decision).
    #[test]
    fn a_true_registered_identity_is_not_refuted() {
        let ((sign, _), counts) = shipped(|| {
            let x = param("x", 0.9, 1.1);
            let a = (x * x).sqrt() * lit(3.0);
            let b = x * lit(3.0);
            let _ = a.register_equal(b, Tol::witness());
            asked(a - b)
        });
        assert_eq!(sign, Some(Sign::Zero), "{counts:?}");
        assert_eq!(counts.symbolic, 1, "{counts:?}");
    }

    /// **The instrument**: refuted asks climbed anyway prove no zero, and
    /// it asked some.
    #[cfg(feature = "sym-revalue-testing")]
    #[test]
    fn refuted_asks_climbed_anyway_prove_no_zero() {
        let ((report, decisions), _) = shipped(|| {
            let ((), decisions) = super::super::revalue::record(|| {
                let x = param("x", -1.0, 1.0);
                let _ = asked(x * x - lit(0.01));
                let _ = asked((x * lit(3.0)).sin() * x - lit(0.2));
            });
            (check(&decisions).expect("in a session"), decisions.len())
        });
        assert_eq!(decisions, 2);
        assert_eq!(report.refuted, 2, "{report:?}");
        assert!(report.zeros.is_empty(), "{report:?}");
    }
}

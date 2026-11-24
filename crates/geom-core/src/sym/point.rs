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
//! [`Interval`], with each document parameter at ONE point of the bracket
//! the session recorded for it ([`super::Sym::param_over`]). Each node's
//! reading is kept for the leaf's session ([`super::Session::points`]), so
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
//!   construction takes ([`Real::select_le_zero`], [`Real::min`] at
//!   `Interval`). An arm the reading does not take is not read;
//! - a value whose expression the graph does not hold declines the whole
//!   reading: an opaque value, a hull (a function of enclosures, not of
//!   the reals they stand for), a [`super::SymOp::Free`] variable (the
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
/// where there is no bracket to read.
pub(super) fn refutes(id: SymId) -> bool {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        slot.as_mut().is_some_and(|sess| {
            reading(sess, id).is_some_and(|x| Bounds::lo(x) > 0.0 || Bounds::hi(x) < 0.0)
        })
    })
}

/// `id`'s certified enclosure at the leaf's point, `None` where the
/// reading declines (the module docs list when).
fn reading(sess: &mut Session, id: SymId) -> Option<Interval> {
    let mut memo = core::mem::take(&mut sess.points);
    let out = read(sess, &mut memo, id);
    sess.points = memo;
    out.filter(|x| x.is_certified())
}

/// The post-order walk, into `memo`: one reading per node, `None` for a
/// node that declines. Without recursion, since a margin's graph can be
/// deep.
fn read(sess: &Session, memo: &mut IdMap<Option<Interval>>, root: SymId) -> Option<Interval> {
    let mut stack = vec![(root, false)];
    while let Some((id, expanded)) = stack.pop() {
        if memo.contains_key(&id) {
            continue;
        }
        let Some(n) = sess.nodes.get(&id) else {
            memo.insert(id, None);
            continue;
        };
        if n.op == SymOp::Select && !expanded {
            // The decision first, then only the arm it takes (both where
            // it straddles), so an arm the construction does not take at
            // the point is not read and cannot decline the reading.
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
        let v = match n.op {
            SymOp::Param => sess
                .params
                .get(&indet_param(n.payload))
                .and_then(|&(lo, hi)| point_of(n.payload, lo, hi))
                .map(Interval::from_f64),
            SymOp::Opaque | SymOp::Hull | SymOp::Free => None,
            SymOp::Lit => Some(Interval::from_f64(f64::from_bits(n.payload))),
            SymOp::Pi => Some(Interval::pi()),
            SymOp::Add => Some(kid(0)? + kid(1)?),
            SymOp::Sub => Some(kid(0)? - kid(1)?),
            // `Div` mints `Mul(a, Inv(b))`; read it as the quotient.
            SymOp::Mul => match sess.nodes.get(&n.kids[1]) {
                Some(inv) if inv.op == SymOp::Inv => {
                    Some(kid(0)? / memo.get(&inv.kids[0]).copied().flatten()?)
                }
                _ => Some(kid(0)? * kid(1)?),
            },
            SymOp::Neg => Some(-kid(0)?),
            SymOp::Powi => Some(Real::powi(kid(0)?, n.payload as u32 as i32)),
            SymOp::Inv => Some(Interval::one() / kid(0)?),
            SymOp::Sqrt => Some(Real::sqrt(kid(0)?)),
            SymOp::Abs => Some(Real::abs(kid(0)?)),
            SymOp::Sin => Some(Real::sin_cos(kid(0)?).0),
            SymOp::Cos => Some(Real::sin_cos(kid(0)?).1),
            SymOp::Tan => Some(Real::tan(kid(0)?)),
            SymOp::Asin => Some(Real::asin(kid(0)?)),
            SymOp::Acos => Some(Real::acos(kid(0)?)),
            SymOp::Atan => Some(Real::atan(kid(0)?)),
            SymOp::Floor => Some(Real::floor(kid(0)?)),
            SymOp::LessSin => Some(Real::less_sin(kid(0)?)),
            SymOp::Atan2 => Some(Real::atan2(kid(0)?, kid(1)?)),
            SymOp::Min => Some(Real::min(kid(0)?, kid(1)?)),
            SymOp::Max => Some(Real::max(kid(0)?, kid(1)?)),
            SymOp::Copysign => Some(Real::copysign(kid(0)?, kid(1)?)),
            SymOp::Select => kid(0).and_then(|d| match *select_arms(Some(d)) {
                [k] => Some(Real::select_le_zero(d, kid(k)?, kid(k)?)),
                [a, b] => Some(Real::select_le_zero(d, kid(a)?, kid(b)?)),
                _ => None,
            }),
        };
        memo.insert(id, v);
    }
    memo.get(&root).copied().flatten()
}

/// The child slots of a `select` its decision's reading `d` takes:
/// `when_le` where `d ≤ 0` is certified, `when_gt` where `d > 0` is, both
/// where it straddles, and none where the decision declined. One rule for
/// the walk and the reading.
fn select_arms(d: Option<Interval>) -> &'static [usize] {
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
/// [`super::contradiction::check`] is and for its reason: its walks are in
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
            let counts = sess.counts;
            let at = super::ladder(sess, d.node, super::Path::Decision);
            sess.counts = counts;
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
                    + s.forms_door.len()
                    + s.retries.iter().map(|m| m.len()).sum::<usize>()
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
    /// every select.
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
        let ((_, forms), _) = shipped(|| {
            let x = param(&name, -1.0, 1.0);
            asked(x.select_le_zero(lit(1.0), S::opaque(Interval::from_bounds(-1.0, 1.0))))
        });
        assert_eq!(forms, 0, "the arm it does not take is not read");
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
    /// nothing, and the tier is asked as before. Each red under the
    /// mutant that reads the value as 0.
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
            let ((_, forms), _) = shipped(|| asked(m()));
            assert!(
                forms > 0,
                "{what}: the reading declines and the tier is asked"
            );
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

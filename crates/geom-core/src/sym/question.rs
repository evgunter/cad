//! **The structural question** (ERROR-DESIGN E12): whether an
//! expression is zero as a function of the document's variables, asked
//! of the expression ([`crate::Expression::proves_zero`]) and answered
//! by the normal form and the retry ladder.
//!
//! **What an ask can read.** Each ask runs in a state of its own, made
//! for that ask and dropped with it, that reads nodes where the
//! installed session does and holds nothing else of it: the leaf's node
//! cache for the ask's length, and the drive's table behind it for a
//! node another leaf recorded (`Session::node_of`). No registry, no
//! parameter bracket, no witness kind, no memo, no count, no deferral
//! list, and no form of the drive's table, whose forms are under the
//! drive's rules and budget and not the question's. So no rule that
//! reads a value has anything to read (the certified read at `min` /
//! `max` / `select` needs a bracket and a witness kind; the
//! registered-identity door needs a registry), no earlier ask can move
//! a later one, and neither the installed session nor the drive's
//! table is written. The rules, the retry ladder and the budget are
//! constants here, not the session's.
//!
//! **What the answer is a function of**: the nodes the expression
//! reaches. For a document built inside its drive that is the
//! document's expression graph, whichever leaf minted each node, and
//! one graph gets one answer at every valuation and in every session. A
//! node minted outside every session on the table is an unknown to the
//! ask, as to every walk (`memo`'s header, with the one order it still
//! reads), and proves nothing; outside any session there is no graph.

use super::{Path, SESSION, Session, SymBudget, SymId, SymRetry, SymRules, ladder};
use crate::real::ProvedZero;

/// The question's term and degree bounds. Any budget is sound (a form
/// past it freezes into one unknown, which still cancels with itself);
/// a smaller one fails faster on a value that is not zero and proves
/// less only where two different spellings of one form both pass it
/// (`the_budget_bounds_what_is_proved`). A chord's component that is
/// zero by identity is a difference of two short forms. No row holds
/// the budget against the document corpus: that measurement needs the
/// documents, which live above this crate.
pub(super) const BUDGET: SymBudget = SymBudget {
    max_terms: 64,
    max_degree: 128,
};

/// The shipped algebra. The two rules that read a value are named off,
/// a second guard behind the first: an ask's state has nothing for them
/// to read, so neither switch alone changes an answer (module docs;
/// `a_max_is_not_proved_once_the_session_holds_a_witness`, and `sym`'s
/// `the_structural_question_does_not_read_the_registry`).
const RULES: SymRules = SymRules {
    decision_read: false,
    registered: false,
    ..SymRules::shipped()
};

/// The drive's shipped ladder, as E12 states the question. At this
/// budget the shapes the ladder was measured on close on the first
/// attempt (a form the retry would keep as an atom is already frozen
/// into one), and no identity is known that needs it here
/// (`sym`'s `the_kept_atom_shapes_close_without_the_ladder_at_the_questions_budget`).
const RETRY: SymRetry = SymRetry::kept_atom();

impl crate::real::expression_seal::Sealed for SymId {}

impl crate::real::Expression for SymId {
    /// `None` outside a session: there is no graph to read.
    fn proves_zero(self) -> Option<ProvedZero> {
        asked(self, BUDGET, RETRY).then(ProvedZero::by_the_tier)
    }
}

/// Whether the ladder proves `id` zero in a fresh ask at `budget` and
/// `retry`.
pub(super) fn asked(id: SymId, budget: SymBudget, retry: SymRetry) -> bool {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let Some(installed) = slot.as_mut() else {
            return false;
        };
        let mut ask = Ask::open(installed, budget, retry);
        ladder(&mut ask.own, id, Path::Decision).is_some()
    })
}

/// One ask: its own state, holding the installed session's node table
/// until the ask ends, by any path.
struct Ask<'s> {
    installed: &'s mut Session,
    own: Session,
    #[cfg(feature = "sym-profile-testing")]
    origin: super::profile::Origin,
}

impl<'s> Ask<'s> {
    fn open(installed: &'s mut Session, budget: SymBudget, retry: SymRetry) -> Self {
        let mut own = Session::new(budget, RULES, retry, None);
        own.lend_nodes(installed);
        // The node table is the drive's: a node another leaf recorded
        // is in no cache of this leaf's, and the ask reads it where the
        // installed session would (`Session::node_of`).
        own.nodes_in.clone_from(&installed.memo);
        Self {
            installed,
            own,
            #[cfg(feature = "sym-profile-testing")]
            origin: super::profile::set_origin(super::profile::Origin::Question),
        }
    }
}

impl Drop for Ask<'_> {
    fn drop(&mut self) {
        self.own.lend_nodes(self.installed);
        #[cfg(feature = "sym-profile-testing")]
        super::profile::set_origin(self.origin);
    }
}

impl Session {
    /// Swaps the node cache with `other`'s: lent on the way into an ask
    /// and returned on the way out.
    fn lend_nodes(&mut self, other: &mut Session) {
        core::mem::swap(&mut self.nodes, &mut other.nodes);
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::super::deferred::with_deferred_session;
    use super::super::{ParamSymbol, Sym, SymCounts, session_counts, with_session_in};
    use super::*;
    use crate::predicate::{Band, Decide, Sign};
    use crate::real::{Expression, Real};
    use crate::{Interval, k_stats};

    fn shipped() -> SymBudget {
        SymBudget {
            max_terms: 4096,
            max_degree: 128,
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum Kind {
        Eager,
        Deferring,
    }

    const KINDS: [Kind; 2] = [Kind::Eager, Kind::Deferring];

    fn session<R>(kind: Kind, budget: SymBudget, rules: SymRules, f: impl FnOnce() -> R) -> R {
        match kind {
            Kind::Eager => with_session_in(budget, rules, SymRetry::none(), None, f).0,
            Kind::Deferring => with_deferred_session(budget, rules, SymRetry::none(), f).0,
        }
    }

    fn p(name: &str, v: f64) -> Sym<f64> {
        Sym::param(ParamSymbol::new(test_utils::symbol_id(name)), v)
    }

    /// A parameter over the box `c ± h`.
    fn boxed(name: &str, c: f64, h: f64) -> Sym<Interval> {
        let (lo, hi) = (c - h, c + h);
        Sym::param_over(
            ParamSymbol::new(test_utils::symbol_id(name)),
            Interval::from_bounds(lo, hi),
            lo,
            hi,
        )
    }

    fn proved<T: Real>(x: T) -> bool {
        x.expression().and_then(Expression::proves_zero).is_some()
    }

    /// **One graph, one answer, whatever the value channel reads**:
    /// `(x + w) − w − x` at `Sym<f64>` computes `0.0` at `(1, 1)` and
    /// `2.8e-17` at `(0.1, 0.3)`, and is proved zero at both, in an
    /// eager and in a deferring session. The profile's mint holds the
    /// other half (a sign reading asks only where the value reads no
    /// definite sign: `profile/tests/recorded_directions.rs`). An ask
    /// that does not borrow the session's node table is red here.
    #[test]
    fn an_identity_that_rounds_is_proved_at_every_valuation() {
        let rounds = |x: f64, w: f64| ((x + w) - w) - x;
        assert_eq!(rounds(1.0, 1.0), 0.0);
        assert_ne!(rounds(0.1, 0.3), 0.0);
        for kind in KINDS {
            for (x, w) in [(1.0, 1.0), (0.1, 0.3)] {
                let answer = session(kind, shipped(), SymRules::shipped(), || {
                    let (x, w) = (p("x", x), p("w", w));
                    proved(((x + w) - w) - x)
                });
                assert!(answer, "{kind:?} at ({x}, {w})");
            }
        }
    }

    /// **An identity over a box is proved where a sign reading
    /// escalates**: `(W + 1) − 1 − W` at `Sym<Interval>` over a box
    /// straddles zero, so a deferring session's reading of it escalates
    /// and names no Zero to discharge. The question proves it in both
    /// sessions. An ask that does not borrow the session's node table
    /// is red here.
    #[test]
    fn an_identity_over_a_box_is_proved_in_both_sessions() {
        for kind in KINDS {
            let (answer, reading) = session(kind, shipped(), SymRules::shipped(), || {
                let w = boxed("W", 1.0, 0.25);
                let hop = ((w + Sym::one()) - Sym::one()) - w;
                (proved(hop), hop.sign_within(Band::least()).map(|d| d.sign))
            });
            assert!(answer, "{kind:?}");
            match kind {
                Kind::Eager => assert_eq!(reading.ok(), Some(Sign::Zero), "eager reads the tier"),
                Kind::Deferring => assert!(reading.is_err(), "deferring escalates: {reading:?}"),
            }
        }
    }

    /// **A coincidence of values is not proved**: two parameters equal
    /// at the valuation. An ask that proves everything is red here.
    #[test]
    fn equal_values_are_not_proved() {
        for kind in KINDS {
            session(kind, shipped(), SymRules::shipped(), || {
                assert!(!proved(p("a", 1.0) - p("b", 1.0)), "{kind:?}");
            });
        }
    }

    /// **A `max` is not proved once the session holds a witness and
    /// brackets**: the session takes a decision first, so its witness
    /// kind is set and its certified read at `max` can take an arm
    /// (`a < b` over point boxes, so `max(a, b)` reads `b`). The
    /// question's state holds neither a bracket nor a witness kind, and
    /// `max(a, b) − b` is not proved. An answer taken from the installed
    /// session's ladder proves it and is red here; so is an ask's state
    /// lent the session's brackets and witness with the read's switch
    /// on. Either guard alone keeps the row green.
    #[test]
    fn a_max_is_not_proved_once_the_session_holds_a_witness() {
        let answer = session(Kind::Eager, shipped(), SymRules::shipped(), || {
            let (a, b) = (boxed("a", 0.5, 0.0), boxed("b", 1.0, 0.0));
            assert!((a - b).sign_within(Band::least()).is_ok(), "a decision");
            proved(a.max(b) - b)
        });
        assert!(!answer);
    }

    /// Expressions over one variable set; four are built on an 84-term
    /// power, which freezes at the question's 64 terms and not at the
    /// drive's 4096.
    fn mixed_bag() -> Vec<Sym<f64>> {
        let (x, y, z, w) = (p("x", 0.3), p("y", 0.7), p("z", 1.1), p("w", 1.9));
        let s = x + y + z + w;
        let big = s.powi(6);
        let r = (x * x + y * y).sqrt();
        let same = big;
        vec![
            big - same,
            big - (s * s * s) * (s * s * s),
            (big + x) - x - big,
            r * r - (x * x + y * y),
            (x + y) * (x - y) - (x * x - y * y),
            ((x + w) - w) - x,
            r - (y * y + x * x).sqrt(),
            big * r - r * big,
            x - y,
        ]
    }

    fn in_one_session<R>(f: impl FnOnce() -> R) -> R {
        session(Kind::Eager, shipped(), SymRules::shipped(), f)
    }

    /// **No ask leaves a trace for the next**: each of nine expressions
    /// answers alone as it answers after any other of them, and a node
    /// the session records only after it was first asked about answers
    /// as it would in a session that recorded it first. An ask's state
    /// kept between asks is red on the second half: the node stays the
    /// unknown it was at the first ask.
    #[test]
    fn an_ask_does_not_move_a_later_answer() {
        let n = mixed_bag().len();
        let alone: Vec<bool> = (0..n)
            .map(|k| in_one_session(|| proved(mixed_bag()[k])))
            .collect();
        assert!(alone.contains(&true) && alone.contains(&false), "{alone:?}");
        for a in 0..n {
            for b in 0..n {
                let pair = in_one_session(|| {
                    let e = mixed_bag();
                    [proved(e[a]), proved(e[b])]
                });
                assert_eq!(pair, [alone[a], alone[b]], "{a} then {b}");
            }
        }
        let outside = p("x", 0.3) * p("y", 0.7);
        let (unknown, recorded) = in_one_session(|| {
            let (x, y) = (p("x", 0.3), p("y", 0.7));
            let unknown = proved(outside - y * x);
            let inside = x * y;
            (unknown, proved(inside - y * x))
        });
        assert!(!unknown, "a node minted outside the session is an unknown");
        assert!(recorded, "recorded since, it is read");
    }

    /// **What the budget trades.** A form past the budget freezes into
    /// one unknown, which still cancels with itself: every identity
    /// around the 84-term power is proved at 64 terms as at 4096. Two
    /// different spellings of that power are one form only where it is
    /// expanded: proved at 4096, not at 64. A budget under which a
    /// frozen node stopped cancelling with itself is red on the first
    /// half; the second half is the price the constant states.
    #[test]
    fn the_budget_bounds_what_is_proved() {
        let at = |budget: SymBudget| {
            in_one_session(|| {
                mixed_bag()
                    .into_iter()
                    .map(|e| asked(e.node(), budget, RETRY))
                    .collect::<Vec<_>>()
            })
        };
        let (small, large) = (at(BUDGET), at(shipped()));
        let two_spellings = 1;
        for (k, (s, l)) in small.iter().zip(&large).enumerate() {
            if k == two_spellings {
                assert_eq!((*s, *l), (false, true), "two spellings past the budget");
            } else {
                assert_eq!(s, l, "expression {k}");
            }
        }
    }

    /// What of the installed session an ask could move: its counts, its
    /// memos' sizes and its deferral list's length.
    fn footprint() -> (Option<SymCounts>, [usize; 6], Option<usize>) {
        let sizes = SESSION.with(|s| {
            let s = s.borrow();
            let sess = s.as_ref().expect("inside a session");
            (
                [
                    sess.forms.len(),
                    sess.forms_early.len(),
                    sess.node.forms_door.len(),
                    sess.atoms.len(),
                    sess.node.retries.len(),
                    sess.nodes.len(),
                ],
                sess.deferred.as_ref().map(std::collections::HashMap::len),
            )
        });
        (session_counts(), sizes.0, sizes.1)
    }

    /// **The question is not a decision**: asking it, of an identity and
    /// of a value that is none, leaves the session's counts, its memos,
    /// its deferral list and the verdict log as they were, in an eager
    /// and in a deferring session. An answer taken from the installed
    /// session's ladder is red on its memos.
    #[test]
    fn asking_leaves_the_session_and_the_log_as_they_were() {
        for kind in KINDS {
            session(kind, shipped(), SymRules::shipped(), || {
                let (x, w) = (p("x", 1.0), p("w", 1.0));
                let (identity, other) = (((x + w) - w) - x, x - w);
                let before = footprint();
                let (answers, log) = k_stats::detached(|| [proved(identity), proved(other)]);
                assert_eq!(answers, [true, false], "{kind:?}");
                assert_eq!(footprint(), before, "{kind:?}: the session");
                let log = log.recorded();
                assert!(
                    log.verdicts.is_empty() && log.escalations.is_empty() && log.marks.is_empty(),
                    "{kind:?}: the log: {log:?}"
                );
            });
        }
    }

    /// **The answer is the same whatever the session's dials**: a tier
    /// switched off by its budget, one of two terms, no rule at all, the
    /// shipped set. `sqrt(s)² − s` needs rule A and four terms' room;
    /// an answer taken at the session's own rules or budget is red at
    /// the first three.
    #[test]
    fn the_answer_does_not_read_the_sessions_dials() {
        let two = SymBudget {
            max_terms: 2,
            max_degree: 128,
        };
        let dials = [
            (SymBudget::none(), SymRules::shipped()),
            (two, SymRules::shipped()),
            (shipped(), SymRules::none()),
            (shipped(), SymRules::shipped()),
        ];
        for kind in KINDS {
            for (budget, rules) in dials {
                let answers = session(kind, budget, rules, || {
                    let (x, y) = (p("x", 3.0), p("y", 4.0));
                    let s = x * x + y * y + x + y;
                    [
                        proved(s.sqrt() * s.sqrt() - s),
                        proved((x + y) * (x - y) - (x * x - y * y)),
                        proved(s.sqrt() - s),
                    ]
                });
                assert_eq!(
                    answers,
                    [true, true, false],
                    "{kind:?} {budget:?} {rules:?}"
                );
            }
        }
    }

    /// **A symbolic scalar over a dual carries its expression**, and the
    /// question is put to it as to any other; outside a session there is
    /// no graph, and no proof.
    #[test]
    fn a_symbolic_dual_is_asked_and_no_session_proves_nothing() {
        let w = |v: f64| {
            Sym::param(
                ParamSymbol::new(test_utils::symbol_id("w")),
                crate::Dual64::variable(v),
            )
        };
        let hop = |w: Sym<crate::Dual64>| ((w + Sym::one()) - Sym::one()) - w;
        assert!(session(Kind::Eager, shipped(), SymRules::shipped(), || {
            proved(hop(w(0.3)))
        }));
        assert!(!proved(hop(w(0.3))), "no session");
    }
}

//! **The structural question** (ERROR-DESIGN E12): whether an
//! expression is zero as a function of the document's variables, asked
//! of the expression ([`crate::Expression::proves_zero`]) and answered
//! by the normal form and the retry ladder with every rule that reads a
//! value shut, at rules and a budget that are constants here.
//!
//! The answer is a function of the nodes the installed session
//! recorded and of nothing else in it. The walks run in a second
//! session's state ([`Session::question`]) that borrows the installed
//! one's node table for the ask: its forms, atoms and counts are its
//! own, so the installed session's memos, counts, deferral list and
//! drive memo are neither read nor written. A node the session did not
//! record (minted outside it) is an unknown, as it is to every walk.

use super::{Path, SESSION, Session, SymBudget, SymId, SymRetry, SymRules, ladder};
use crate::real::ProvedZero;

/// The question's term and degree bounds. A hop between two points
/// that share a coordinate is a difference of two short forms, and a
/// form that is not zero has to fail fast: every identity the profile
/// corpus asks about is proved at 64 terms exactly as at 4096, where
/// one unproved guided hop costs 750 ms against 4 ms here.
const BUDGET: SymBudget = SymBudget {
    max_terms: 64,
    max_degree: 128,
};

/// The shipped algebra without the two rules that read a value: the
/// certified read at `min` / `max` / `select`, and the
/// registered-identity door, whose entries are witnessed against
/// values.
const RULES: SymRules = SymRules {
    decision_read: false,
    registered: false,
    ..SymRules::shipped()
};

/// The drive's shipped ladder.
const RETRY: SymRetry = SymRetry::kept_atom();

impl crate::real::expression_seal::Sealed for SymId {}

impl crate::real::Expression for SymId {
    /// `None` outside a session: there is no graph to read.
    fn proves_zero(self) -> Option<ProvedZero> {
        SESSION.with(|s| {
            let mut slot = s.borrow_mut();
            let mut ask = Ask::open(slot.as_mut()?);
            ladder(&mut ask.own, self, Path::Decision).map(|_| ProvedZero::by_the_tier())
        })
    }
}

/// One ask: the question's state holding the installed session's node
/// table, handed back when the ask ends, by any path.
struct Ask<'s> {
    installed: &'s mut Session,
    own: Box<Session>,
    #[cfg(feature = "sym-profile-testing")]
    origin: super::profile::Origin,
}

impl<'s> Ask<'s> {
    fn open(installed: &'s mut Session) -> Self {
        let mut own = installed
            .question
            .take()
            .unwrap_or_else(|| Box::new(Session::new(BUDGET, RULES, RETRY, None)));
        own.lend_nodes(installed);
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
        // The state goes back to its slot; the empty box left here is
        // dropped with the ask.
        let own = core::mem::replace(
            &mut self.own,
            Box::new(Session::new(BUDGET, RULES, RETRY, None)),
        );
        self.installed.question = Some(own);
        #[cfg(feature = "sym-profile-testing")]
        super::profile::set_origin(self.origin);
    }
}

impl Session {
    /// Swaps the node table, and the free indeterminates it names, with
    /// `other`'s: lent on the way into an ask and returned on the way
    /// out.
    fn lend_nodes(&mut self, other: &mut Session) {
        core::mem::swap(&mut self.nodes, &mut other.nodes);
        core::mem::swap(&mut self.free, &mut other.free);
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::super::deferred::with_deferred_session;
    use super::super::{
        ParamSymbol, Sym, SymCounts, SymRegistration, session_counts, with_session_in,
    };
    use super::*;
    use crate::predicate::{Band, Decide, Sign};
    use crate::real::{Expression, Real};
    use crate::tolerance::Tol;
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

    /// **A coincidence of values is not proved, and neither is anything
    /// that rests on a value**: two parameters equal at the valuation;
    /// a `max` on either side of its comparison, which the session's
    /// certified read takes an arm of; two values a constructor
    /// registered equal, which the session's door then decides Zero.
    /// An answer taken from the installed session's ladder is red on
    /// the registered pair.
    #[test]
    fn nothing_that_reads_a_value_is_proved() {
        for kind in KINDS {
            session(kind, shipped(), SymRules::shipped(), || {
                assert!(!proved(p("a", 1.0) - p("b", 1.0)), "{kind:?}: equal values");
                for a in [0.5, 1.5] {
                    let (a, b) = (boxed("a", a, 0.0), boxed("b", 1.0, 0.0));
                    assert!(!proved(a.max(b) - b), "{kind:?}: max, a = {a:?}");
                    assert!(!proved(a.min(b) - b), "{kind:?}: min, a = {a:?}");
                }
            });
        }
        let band = Band::linear(Tol::witness()).expect("the witness tolerance has a band");
        with_session_in(
            shipped(),
            SymRules::shipped(),
            SymRetry::none(),
            None,
            || {
                let (n, r) = (boxed("n", 1.0, 0.0), boxed("r", 1.0, 0.0));
                assert_eq!(
                    n.register_equal(r, Tol::witness()),
                    SymRegistration::Recorded
                );
                let residual = n - r;
                let decided = residual.sign_within(band).expect("a point box decides");
                assert!(
                    decided.margin.proven(),
                    "the session's door proves the registered pair"
                );
                assert!(!proved(residual), "the question does not read the registry");
            },
        );
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
                    sess.forms_door.len(),
                    sess.atoms.len(),
                    sess.retries.len(),
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

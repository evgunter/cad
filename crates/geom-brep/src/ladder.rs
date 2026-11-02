//! **The ladder**: one question read off several readings in order,
//! where the first reading that answers decides it.
//!
//! The rule, stated once: **any definite reading answers; one the band
//! does not decide is kept, and refuses only where no later reading
//! answers.** A ladder that returned on its first undecided reading would
//! escalate where a reading it had not yet read already answers, and the
//! refusal would name a tolerance that cannot change the answer.
//!
//! - [`climb`] is that rule over any reader: each rung
//!   [answers](Rung::Answers), [passes](Rung::Passes) the question on, or
//!   is kept ([`Rung::InBand`], [`Rung::Blocked`]). Which decision answers
//!   is the reader's: "decided apart" for an identity, "decided one" for a
//!   search over candidates.
//! - [`read_identity`] is the **identity ladder**, the ladder over banded
//!   data each required `Zero` for two descriptions to be one, so any
//!   datum decided apart answers.
//! - [`Evidence`] is **the one ranking of what a ladder kept** when nothing
//!   answered: a limit no tolerance moves ahead of an in-band reading.
//!
//! [`climb`] owns the recording: a kept reading's run is held, and spliced
//! as superseded ([`geom_core::k_stats::splice_superseded`]) when a later
//! reading answers, so its escalation does not read as one the answer
//! rests on; every run is spliced in rung order.

use geom_core::k_stats::{Detached, decide_reported, detached, splice, splice_superseded};
use geom_core::predicate::{Decided, Indeterminate};
use geom_core::{Band, Decide, Margin, MarginDiag, Sign};

/// What one rung of a ladder read ([`climb`]).
#[derive(Clone, Debug, PartialEq)]
pub enum Rung<A, E> {
    /// The reading answers the question: the ladder stops here.
    Answers(A),
    /// The reading decided, and its decision leaves the question to the
    /// later rungs.
    Passes,
    /// The reading did not decide, on a margin a tighter tolerance would
    /// decide: kept.
    InBand(E),
    /// The reading met a limit no tolerance moves: kept, and ranked ahead
    /// of an in-band one ([`Evidence`]).
    Blocked(E),
}

impl<A> Rung<A, Indeterminate> {
    /// A banded reading the band did not decide, kept as what it is: a
    /// limit where its margin cannot be read (poisoned), in band
    /// otherwise.
    #[must_use]
    pub fn kept(cause: Indeterminate) -> Self {
        if cause.margin.is_invalid() {
            Self::Blocked(cause)
        } else {
            Self::InBand(cause)
        }
    }
}

/// What a ladder read ([`climb`]).
#[derive(Clone, Debug, PartialEq)]
pub enum Climb<A, E> {
    /// A rung answered: its index among the rungs, and its answer.
    Answered {
        /// The answering rung's index.
        at: usize,
        /// Its answer.
        answer: A,
    },
    /// Every rung passed.
    Passed,
    /// No rung answered and at least one was kept: the one it refuses on
    /// ([`Evidence`]'s ranking).
    Undecided(E),
}

/// Reads `rungs` in order through `read` under the ladder's rule (module
/// docs), recording as a serial read of the same rungs would, in rung
/// order.
///
/// Each kept reading's run is held until the ladder's outcome is known:
/// spliced whole where no later rung answers (the escalation is what the
/// outcome rests on), spliced superseded where one does.
pub fn climb<I, A, E>(
    rungs: impl IntoIterator<Item = I>,
    mut read: impl FnMut(I) -> Rung<A, E>,
) -> Climb<A, E> {
    let mut kept = Evidence::default();
    // The runs read since the first kept one, in rung order, each with
    // whether it was kept: held so the order survives the splice.
    let mut held: Vec<(Detached, bool)> = Vec::new();
    for (at, rung) in rungs.into_iter().enumerate() {
        let (got, log) = detached(|| read(rung));
        match got {
            Rung::Answers(answer) => {
                for (run, was_kept) in held {
                    if was_kept {
                        splice_superseded(run);
                    } else {
                        splice(run);
                    }
                }
                splice(log);
                return Climb::Answered { at, answer };
            }
            Rung::Passes if held.is_empty() => splice(log),
            Rung::Passes => held.push((log, false)),
            Rung::InBand(e) => {
                kept.in_band(e);
                held.push((log, true));
            }
            Rung::Blocked(e) => {
                kept.blocked(e);
                held.push((log, true));
            }
        }
    }
    held.into_iter().for_each(|(run, _)| splice(run));
    match kept.ranked() {
        Ranked::Blocked(e) | Ranked::InBand(e) => Climb::Undecided(e),
        Ranked::Neither => Climb::Passed,
    }
}

/// What an identity ladder's data answer ([`read_identity`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IdentityLadder {
    /// A datum decided past the band: the identity does not hold. `at` is
    /// its index in the data, `sign` its sign.
    Apart {
        /// The answering datum's index.
        at: usize,
        /// Its sign (never [`Sign::Zero`]).
        sign: Sign,
    },
    /// Every datum decided `Zero`: the identity holds, with the data's
    /// decided margins taken jointly (`None` for no data).
    Zero(Option<MarginDiag>),
    /// No datum decided past the band and one did not decide: the first
    /// that cannot be read (its margin poisoned), else the first in band.
    Undecided(Indeterminate),
}

/// **Whether two descriptions are one**, read off `data` that are each
/// zero exactly where the two agree, every datum decided on `band` under
/// its own name: the ladder ([`climb`]) whose answer is a datum decided
/// apart. A datum that is not finite reads no sign: the funnel reads its
/// residual `x − x`, which is poison, so it is kept as a limit.
pub fn read_identity<T: Decide>(
    data: impl IntoIterator<Item = (&'static str, Margin<T>)>,
    band: Band,
) -> IdentityLadder {
    let mut joint: Option<MarginDiag> = None;
    let climbed = climb(data, |(name, margin)| {
        let read_on = if geom_core::is_finite_length(margin.value()) {
            margin
        } else {
            Margin::of(margin.value() - margin.value())
        };
        match decide_reported(name, read_on, band) {
            Ok(Decided {
                sign: sign @ (Sign::Positive | Sign::Negative),
                ..
            }) => Rung::Answers(sign),
            Ok(Decided {
                sign: Sign::Zero,
                margin,
            }) => {
                joint = Some(joint.map_or(margin, |first| first.jointly(margin)));
                Rung::Passes
            }
            Err(cause) => Rung::kept(cause),
        }
    });
    match climbed {
        Climb::Answered { at, answer: sign } => IdentityLadder::Apart { at, sign },
        Climb::Passed => IdentityLadder::Zero(joint),
        Climb::Undecided(cause) => IdentityLadder::Undecided(cause),
    }
}

/// **The one ranking of what a run of readings kept** when none of them
/// answered: over a ladder's rungs ([`climb`]), over one point's rays
/// (`topo`'s ray walk), and over a ladder of witnesses each of which kept
/// its own walk's refusal (`topo`'s shell witness and loop roles):
///
/// 1. the first blocked reading — a limit no tolerance moves, true of
///    every reading it stopped whatever the band, and its own recourse is
///    the one that frees those readings;
/// 2. else the first in-band reading — a margin a tighter tolerance would
///    decide, carried with its value. It is not ranked first because the
///    reading it stopped went no further: a tighter tolerance may decide
///    it and still leave it to meet the limit;
/// 3. else neither, and the caller says what that means.
#[derive(Debug)]
pub struct Evidence<E> {
    blocked: Option<E>,
    in_band: Option<E>,
}

/// [`Evidence`]'s verdict.
#[derive(Debug)]
pub enum Ranked<E> {
    /// The first limit met.
    Blocked(E),
    /// No limit met; the first in-band reading.
    InBand(E),
    /// Neither was kept.
    Neither,
}

impl<E> Default for Evidence<E> {
    fn default() -> Self {
        Self {
            blocked: None,
            in_band: None,
        }
    }
}

impl<E> Evidence<E> {
    /// Keeps `e` as a limit met, if it is the first.
    pub fn blocked(&mut self, e: E) {
        self.blocked.get_or_insert(e);
    }

    /// Keeps `e` as an in-band reading, if it is the first.
    pub fn in_band(&mut self, e: E) {
        self.in_band.get_or_insert(e);
    }

    /// `self`'s readings first, then `later`'s: the evidence of two runs
    /// read in that order.
    #[must_use]
    pub fn then(self, later: Self) -> Self {
        Self {
            blocked: self.blocked.or(later.blocked),
            in_band: self.in_band.or(later.in_band),
        }
    }

    /// Whether a limit was kept: [`Self::ranked`] reads
    /// [`Ranked::Blocked`].
    #[must_use]
    pub const fn met_a_limit(&self) -> bool {
        self.blocked.is_some()
    }

    /// The one ranking (the type's docs).
    #[must_use]
    pub fn ranked(self) -> Ranked<E> {
        match (self.blocked, self.in_band) {
            (Some(e), _) => Ranked::Blocked(e),
            (None, Some(e)) => Ranked::InBand(e),
            (None, None) => Ranked::Neither,
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::k_stats::{Bracket, decide_reported};
    use geom_core::predicate::{Band, Indeterminate};
    use geom_core::{Margin, Sign};

    use super::{Climb, Evidence, IdentityLadder, Ranked, Rung, climb, read_identity};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).expect("a valid band")
    }

    /// Runs `f` under a bracket: its outcome, and the names its run
    /// recorded as verdicts and as escalations.
    fn recorded<R>(f: impl FnOnce() -> R) -> (R, Vec<&'static str>, Vec<&'static str>) {
        let bracket = Bracket::open();
        let got = f();
        let recorded = bracket.finish();
        (
            got,
            recorded.verdicts.iter().map(|v| v.predicate).collect(),
            recorded
                .escalations
                .iter()
                .map(geom_core::k_stats::Escalation::predicate)
                .collect(),
        )
    }

    /// The identity ladder over `data`, recorded.
    fn read(
        data: &[(&'static str, f64)],
    ) -> (IdentityLadder, Vec<&'static str>, Vec<&'static str>) {
        recorded(|| read_identity(data.iter().map(|&(n, m)| (n, Margin::of(m))), band()))
    }

    /// **A later definite datum answers past an earlier in-band one**,
    /// and the kept reading's escalation leaves the log, since the
    /// answer does not rest on it.
    #[test]
    fn a_later_definite_datum_answers_past_an_in_band_one() {
        let (got, verdicts, escalations) = read(&[("a", 0.0), ("b", 5e-9), ("c", 0.0), ("d", 1.0)]);
        assert_eq!(
            got,
            IdentityLadder::Apart {
                at: 3,
                sign: Sign::Positive
            }
        );
        assert_eq!(
            verdicts,
            ["a", "c", "d"],
            "every decided reading, in data order"
        );
        assert!(
            escalations.is_empty(),
            "the superseded escalation: {escalations:?}"
        );
    }

    /// **An in-band datum no later datum decides refuses on it**, its
    /// escalation recorded: no definite datum answers, so the in-band
    /// one is what the outcome rests on.
    #[test]
    fn an_in_band_datum_refuses_where_no_later_one_decides() {
        let (got, verdicts, escalations) = read(&[("a", 0.0), ("b", 5e-9), ("c", 0.0)]);
        let IdentityLadder::Undecided(cause) = got else {
            panic!("undecided: {got:?}")
        };
        assert_eq!(cause.predicate, Some("b"));
        assert_eq!(verdicts, ["a", "c"]);
        assert_eq!(escalations, ["b"]);
    }

    /// **The first definite datum answers, and nothing after it is
    /// read**; all Zero is the identity, its margins joined.
    #[test]
    fn the_first_definite_datum_answers_and_all_zero_holds() {
        let (got, verdicts, _) = read(&[("a", -1.0), ("b", 5e-9), ("c", 1.0)]);
        assert_eq!(
            got,
            IdentityLadder::Apart {
                at: 0,
                sign: Sign::Negative
            }
        );
        assert_eq!(verdicts, ["a"], "nothing past the answer is read");
        let (got, _, _) = read(&[("a", 0.0), ("b", 5e-10)]);
        assert!(matches!(got, IdentityLadder::Zero(Some(_))), "{got:?}");
        let (got, _, _) = read(&[]);
        assert_eq!(got, IdentityLadder::Zero(None));
    }

    /// **An unreadable datum is the refusal's cause ahead of an in-band
    /// one**, whichever was read first: a poisoned margin is a limit no
    /// tolerance moves. An infinite datum is unreadable too, not apart.
    #[test]
    fn an_unreadable_datum_refuses_ahead_of_an_in_band_one() {
        for poison in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let (got, _, _) = read(&[("a", 5e-9), ("b", poison), ("c", 0.0)]);
            let IdentityLadder::Undecided(cause) = got else {
                panic!("{poison}: undecided: {got:?}")
            };
            assert_eq!(cause.predicate, Some("b"), "{poison}");
            assert!(cause.margin.is_invalid(), "{poison}: {cause:?}");
        }
    }

    /// **The reversed polarity reads the same rule**: a search whose
    /// answer is a candidate decided `Zero` passes the ones decided
    /// apart, answers past an earlier in-band one with its escalation
    /// superseded, and refuses on an unreadable candidate ahead of an
    /// earlier in-band one where none answers.
    #[test]
    fn a_search_for_the_first_candidate_within_reads_the_same_rule() {
        let search = |margins: &'static [f64]| {
            recorded(|| {
                climb(margins.iter().copied(), |m| {
                    match decide_reported("within", Margin::of(m), band()) {
                        Ok(d) if d.sign == Sign::Zero => Rung::Answers(m),
                        Ok(_) => Rung::Passes,
                        Err(cause) => Rung::kept(cause),
                    }
                })
            })
        };
        let (got, verdicts, escalations) = search(&[1.0, 5e-9, 0.0]);
        assert_eq!(got, Climb::Answered { at: 2, answer: 0.0 });
        assert_eq!(verdicts, ["within", "within"]);
        assert!(escalations.is_empty(), "superseded: {escalations:?}");
        let (got, _, escalations) = search(&[5e-9, 1.0, f64::NAN]);
        let Climb::Undecided::<f64, Indeterminate>(cause) = got else {
            panic!("undecided: {got:?}")
        };
        assert!(cause.margin.is_invalid(), "the unreadable one: {cause:?}");
        assert_eq!(escalations.len(), 2, "both kept runs recorded");
        let (got, _, _) = search(&[1.0, -1.0]);
        assert_eq!(got, Climb::Passed);
    }

    /// [`Evidence`] over two runs read in order: the first run's of each
    /// kind wins, and a limit ranks ahead of an in-band reading whichever
    /// run kept it.
    #[test]
    fn evidence_ranks_a_limit_first_and_keeps_run_order() {
        let rank = |e: Evidence<u8>| match e.ranked() {
            Ranked::Blocked(e) => (1, e),
            Ranked::InBand(e) => (2, e),
            Ranked::Neither => (3, 0),
        };
        let mut a = Evidence::default();
        a.in_band(1);
        let mut b = Evidence::default();
        b.blocked(2);
        b.in_band(3);
        assert_eq!(rank(a.then(b)), (1, 2));
        let (mut a, mut b) = (Evidence::default(), Evidence::default());
        a.in_band(1);
        b.in_band(3);
        assert_eq!(rank(a.then(b)), (2, 1));
        assert_eq!(rank(Evidence::<u8>::default()), (3, 0));
    }
}

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
//!   is the reader's: "decided apart" for an identity, "decided within"
//!   for a search over candidates. A banded reading becomes a rung
//!   through [`Rung::decide`], the one place a margin is read, so no
//!   ladder reads a non-finite margin as a definite answer.
//! - [`read_identity`] is the **identity ladder**, the ladder over banded
//!   data each required `Zero` for two descriptions to be one, so any
//!   datum decided apart answers.
//! - [`Evidence`] is **the one ranking of what a ladder kept** when nothing
//!   answered: a limit no tolerance moves ahead of an in-band reading.
//!
//! [`climb`] owns the recording, and the caller declares what an answer
//! may do with the kept readings ahead of it ([`Answering`]): where every
//! answering rung gives one answer, a kept reading's run is spliced
//! superseded ([`geom_core::k_stats::splice_superseded`]), since the
//! answer does not rest on it; where which rung answers chooses the
//! answer, it stays live, since a kept rung ahead may truly have answered
//! first. Every run is spliced in rung order.

use geom_core::k_stats::{
    Detached, Magnitude, decide_magnitude_reported, decide_reported, detached, splice,
    splice_superseded,
};
use geom_core::predicate::{Decided, Indeterminate};
use geom_core::{Band, Decide, Margin, MarginDiag, Sign};

/// What one rung of a ladder read ([`climb`]).
#[derive(Clone, Debug, PartialEq)]
pub enum Rung<A, E> {
    /// The reading answers the question: the ladder stops here.
    Answers(A),
    /// The reading refuses the whole question (a corrupt input, a
    /// reading of the question rather than of this rung): the ladder
    /// stops here with `A`, and the refusal rests on every reading kept
    /// ahead of it whatever the caller declared ([`Answering`]): a kept
    /// rung a tighter tolerance decides may have answered before this one
    /// was read.
    Aborts(A),
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

/// A reading a band did not decide, as what it is ([`Rung::InBand`],
/// [`Rung::Blocked`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kept<E> {
    /// A margin a tighter tolerance would decide.
    InBand(E),
    /// A limit no tolerance moves.
    Blocked(E),
}

impl Kept<Indeterminate> {
    /// `cause` kept as what it is: a limit where its margin cannot be
    /// read (poisoned), in band otherwise.
    #[must_use]
    pub fn of(cause: Indeterminate) -> Self {
        if cause.margin.is_invalid() {
            Self::Blocked(cause)
        } else {
            Self::InBand(cause)
        }
    }
}

impl<A, E> From<Kept<E>> for Rung<A, E> {
    fn from(kept: Kept<E>) -> Self {
        match kept {
            Kept::InBand(e) => Self::InBand(e),
            Kept::Blocked(e) => Self::Blocked(e),
        }
    }
}

/// **One banded reading**, the one place a ladder's margin is read:
/// `margin` decided on `band` under `name`, or what the band left it
/// ([`Kept::of`]). A margin that is not finite reads no sign: it is
/// decided as its residual `x − x`, which is poison, so ±∞ is kept as a
/// limit rather than read as a maximally definite answer.
///
/// # Errors
///
/// The reading, kept.
pub fn decided<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Decided, Kept<Indeterminate>> {
    banded(margin, |m| decide_reported(name, m, band))
}

/// `margin` read by `read`, the non-finite rule ([`decided`]) applied
/// first: `margin` where it is finite, else its residual; and what the
/// band left it, kept ([`Kept::of`]).
fn banded<T: Decide, R>(
    margin: Margin<T>,
    read: impl FnOnce(Margin<T>) -> Result<R, Indeterminate>,
) -> Result<R, Kept<Indeterminate>> {
    let margin = if geom_core::is_finite_length(margin.value()) {
        margin
    } else {
        Margin::of(margin.value() - margin.value())
    };
    read(margin).map_err(Kept::of)
}

impl<A> Rung<A, Indeterminate> {
    /// **One banded reading as a rung** ([`decided`]): answering where
    /// `answer` returns one and passing where it returns `None`; kept
    /// where the band decides nothing.
    pub fn decide<T: Decide>(
        name: &'static str,
        margin: Margin<T>,
        band: Band,
        answer: impl FnOnce(Decided) -> Option<A>,
    ) -> Self {
        match decided(name, margin, band) {
            Ok(decided) => answer(decided).map_or(Self::Passes, Self::Answers),
            Err(kept) => kept.into(),
        }
    }

    /// [`Self::decide`] for a quantity nonnegative by construction (a
    /// norm, a fraction of one): its reading is a [`Magnitude`], and a
    /// definite `Negative` panics in the one place
    /// ([`decide_magnitude_reported`]) that states the precondition.
    pub fn decide_magnitude<T: Decide>(
        name: &'static str,
        margin: Margin<T>,
        band: Band,
        answer: impl FnOnce(Magnitude, MarginDiag) -> Option<A>,
    ) -> Self {
        match banded(margin, |m| decide_magnitude_reported(name, m, band)) {
            Ok((magnitude, at)) => answer(magnitude, at).map_or(Self::Passes, Self::Answers),
            Err(kept) => kept.into(),
        }
    }

    /// A reading that answers `Some`, says nothing (`None`: the zero
    /// band, passed to the later rungs), or did not decide (kept).
    #[must_use]
    pub fn of_reading(reading: Result<Option<A>, Indeterminate>) -> Self {
        match reading {
            Ok(Some(a)) => Self::Answers(a),
            Ok(None) => Self::Passes,
            Err(cause) => Self::kept(cause),
        }
    }

    /// A reading the band did not decide, kept as what it is
    /// ([`Kept::of`]).
    #[must_use]
    pub fn kept(cause: Indeterminate) -> Self {
        Kept::of(cause).into()
    }
}

impl<A, E> Rung<A, E> {
    /// The rung with its kept reading mapped by `f`, for a reader whose
    /// refusal carries more than the reading's own cause (the entity it
    /// was read at). An answer and a pass are unchanged.
    pub fn map_kept<F>(self, f: impl FnOnce(E) -> F) -> Rung<A, F> {
        match self {
            Self::Answers(a) => Rung::Answers(a),
            Self::Aborts(a) => Rung::Aborts(a),
            Self::Passes => Rung::Passes,
            Self::InBand(e) => Rung::InBand(f(e)),
            Self::Blocked(e) => Rung::Blocked(f(e)),
        }
    }
}

/// **What an answer may do with the kept readings ahead of it**, which
/// the caller of [`climb`] declares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answering {
    /// Any answering rung gives the one answer, so which rung answers
    /// does not matter: an identity's "apart", a side every off-plane
    /// point reads alike. A kept reading ahead of the answer could only
    /// have given the same answer, so its run is spliced superseded.
    Any,
    /// Which rung answers chooses the answer: the first candidate within,
    /// the first member that projects. A kept reading ahead of the answer
    /// may truly have answered first, so the answer rests on it and its
    /// run is spliced live.
    First,
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

impl<A, E> Climb<A, E> {
    /// The ladder's outcome as one reading ([`Rung::of_reading`]'s
    /// inverse): the answer, `None` where every rung passed, and the
    /// ranked kept reading where none answered.
    ///
    /// # Errors
    ///
    /// That kept reading.
    pub fn into_reading(self) -> Result<Option<A>, E> {
        match self {
            Self::Answered { answer, .. } => Ok(Some(answer)),
            Self::Passed => Ok(None),
            Self::Undecided(e) => Err(e),
        }
    }
}

/// Reads `rungs` in order through `read` under the ladder's rule (module
/// docs), recording as a serial read of the same rungs would, in rung
/// order.
///
/// Each kept reading's run is held until the ladder's outcome is known:
/// spliced whole where no later rung answers (the escalation is what the
/// outcome rests on); where one does, spliced superseded under
/// [`Answering::Any`] and whole under [`Answering::First`], or after a
/// [`Rung::Aborts`].
pub fn climb<I, A, E>(
    answering: Answering,
    rungs: impl IntoIterator<Item = I>,
    read: impl FnMut(I) -> Rung<A, E>,
) -> Climb<A, E> {
    match climb_evidence(answering, rungs, read) {
        Ok((at, answer)) => Climb::Answered { at, answer },
        Err(kept) => match kept.ranked() {
            Ranked::Blocked(e) | Ranked::InBand(e) => Climb::Undecided(e),
            Ranked::Neither => Climb::Passed,
        },
    }
}

/// [`climb`], keeping everything the rungs kept where none answered: the
/// [`Evidence`] whole, for a caller that ranks two ladders' evidence
/// together ([`Evidence::then`]). Empty evidence is every rung passed.
///
/// # Errors
///
/// That evidence, where no rung answered or aborted.
pub fn climb_evidence<I, A, E>(
    answering: Answering,
    rungs: impl IntoIterator<Item = I>,
    mut read: impl FnMut(I) -> Rung<A, E>,
) -> Result<(usize, A), Evidence<E>> {
    let mut kept = Evidence::default();
    // The runs read since the first kept one, in rung order, each with
    // whether it was kept: held so the order survives the splice.
    let mut held: Vec<(Detached, bool)> = Vec::new();
    for (at, rung) in rungs.into_iter().enumerate() {
        let (got, log) = detached(|| read(rung));
        let (answer, supersede) = match got {
            Rung::Answers(answer) => (answer, answering == Answering::Any),
            Rung::Aborts(answer) => (answer, false),
            Rung::Passes => {
                if held.is_empty() {
                    splice(log);
                } else {
                    held.push((log, false));
                }
                continue;
            }
            Rung::InBand(e) => {
                kept.in_band(e);
                held.push((log, true));
                continue;
            }
            Rung::Blocked(e) => {
                kept.blocked(e);
                held.push((log, true));
                continue;
            }
        };
        for (run, was_kept) in held {
            if was_kept && supersede {
                splice_superseded(run);
            } else {
                splice(run);
            }
        }
        splice(log);
        return Ok((at, answer));
    }
    held.into_iter().for_each(|(run, _)| splice(run));
    Err(kept)
}

/// What an identity ladder's data answer ([`read_identity`]): a
/// [`Climb`] in the identity's own terms, which carry what `Climb` does
/// not — the answering datum's sign, and the data's joint margin where
/// the identity holds.
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
/// its own name: the ladder ([`climb`], [`Answering::Any`]: every datum
/// decided apart says the same "not one") whose answer is a datum decided
/// apart.
pub fn read_identity<T: Decide>(
    data: impl IntoIterator<Item = (&'static str, Margin<T>)>,
    band: Band,
) -> IdentityLadder {
    let mut joint: Option<MarginDiag> = None;
    let climbed = climb(Answering::Any, data, |(name, margin)| {
        Rung::decide(name, margin, band, |decided| match decided.sign {
            Sign::Positive | Sign::Negative => Some(decided.sign),
            Sign::Zero => {
                joint = Some(joint.map_or(decided.margin, |first| first.jointly(decided.margin)));
                None
            }
        })
    });
    match climbed {
        Climb::Answered { at, answer: sign } => IdentityLadder::Apart { at, sign },
        Climb::Passed => IdentityLadder::Zero(joint),
        Climb::Undecided(cause) => IdentityLadder::Undecided(cause),
    }
}

/// **The one ranking of what a run of readings kept** when none of them
/// answered — a ladder's rungs ([`climb`]), and any other run of readings
/// that keeps refusals while it looks for one that decides:
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
    use geom_core::k_stats::Bracket;
    use geom_core::predicate::{Band, Indeterminate};
    use geom_core::{Margin, Sign};

    use super::{Answering, Climb, Evidence, IdentityLadder, Ranked, Rung, climb, read_identity};

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

    /// A search for the first candidate within, under `answering`,
    /// recorded.
    fn search(
        answering: Answering,
        margins: &[f64],
    ) -> (
        Climb<f64, Indeterminate>,
        Vec<&'static str>,
        Vec<&'static str>,
    ) {
        recorded(|| {
            climb(answering, margins.iter().copied(), |m| {
                Rung::decide("within", Margin::of(m), band(), |d| {
                    (d.sign == Sign::Zero).then_some(m)
                })
            })
        })
    }

    /// **A search's answer rests on the kept candidates ahead of it**: a
    /// later candidate within answers past an earlier in-band one, whose
    /// escalation stays live, since that candidate may truly be within
    /// and would have been the answer. Where every answer is alike the
    /// same escalation is superseded. Where none answers, an unreadable
    /// candidate refuses ahead of an earlier in-band one, and every
    /// candidate decided apart passes.
    #[test]
    fn a_search_keeps_the_escalations_ahead_of_its_answer_live() {
        let (got, verdicts, escalations) = search(Answering::First, &[1.0, 5e-9, 0.0]);
        assert_eq!(got, Climb::Answered { at: 2, answer: 0.0 });
        assert_eq!(verdicts, ["within", "within"]);
        assert_eq!(escalations, ["within"], "the answer rests on it");
        let (_, _, escalations) = search(Answering::Any, &[1.0, 5e-9, 0.0]);
        assert!(escalations.is_empty(), "alike answers supersede it");
        let (got, _, escalations) = search(Answering::First, &[5e-9, 1.0, f64::NAN]);
        let Climb::Undecided(cause) = got else {
            panic!("undecided: {got:?}")
        };
        assert!(cause.margin.is_invalid(), "the unreadable one: {cause:?}");
        assert_eq!(escalations.len(), 2, "both kept runs recorded");
        assert_eq!(search(Answering::First, &[1.0, -1.0]).0, Climb::Passed);
    }

    /// **An abort keeps every kept reading ahead of it live, under either
    /// declaration**, and returns its own answer: the refusal rests on
    /// the kept rungs, any of which a tighter tolerance may decide first.
    #[test]
    fn an_abort_keeps_the_readings_ahead_of_it_live() {
        for answering in [Answering::Any, Answering::First] {
            let (got, verdicts, escalations) = recorded(|| {
                climb(answering, [5e-9, 1.0, f64::NAN], |m| {
                    if m.is_nan() {
                        return Rung::Aborts(Err("corrupt"));
                    }
                    Rung::decide("within", Margin::of(m), band(), |d| {
                        (d.sign == Sign::Zero).then_some(Ok(m))
                    })
                })
            });
            assert_eq!(
                got,
                Climb::Answered {
                    at: 2,
                    answer: Err("corrupt")
                },
                "{answering:?}: the abort's error"
            );
            assert_eq!(verdicts, ["within"], "{answering:?}: the decided rung");
            assert_eq!(
                escalations,
                ["within"],
                "{answering:?}: the kept rung ahead stays live"
            );
        }
    }

    /// **A non-finite margin is a limit at every rung, never an answer**:
    /// a search that answers on a definite sign does not answer on ±∞.
    #[test]
    fn a_non_finite_margin_never_answers() {
        for poison in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            let got = climb(Answering::First, [poison], |m| {
                Rung::decide("apart", Margin::of(m), band(), |d| {
                    (d.sign != Sign::Zero).then_some(d.sign)
                })
            });
            let Climb::Undecided(cause) = got else {
                panic!("{poison}: {got:?}")
            };
            assert!(cause.margin.is_invalid(), "{poison}: {cause:?}");
        }
    }

    /// **The probe sample stream reads the rungs in rung order**, across
    /// the verdict and escalation channels: a kept datum's sample comes
    /// before the `Zero` samples read after it, as a serial read records
    /// them, not after them where its run was held.
    #[cfg(feature = "probe")]
    #[test]
    fn the_sample_stream_interleaves_in_rung_order() {
        use geom_core::k_stats::{self, Probe};
        k_stats::start_recording();
        let data = [("a", 0.0), ("b", 5e-9), ("c", 0.0), ("d", 0.0)];
        let got = read_identity(data.iter().map(|&(n, m)| (n, Margin::of(Probe(m)))), band());
        let names: Vec<&str> = k_stats::take_samples()
            .iter()
            .map(|s| s.predicate)
            .collect();
        assert!(matches!(got, IdentityLadder::Undecided(_)), "{got:?}");
        assert_eq!(names, ["a", "b", "c", "d"], "rung order across channels");
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
        // Within one run the first of each kind is kept, never the last.
        let mut e = Evidence::default();
        e.in_band(1);
        e.in_band(2);
        assert_eq!(rank(e), (2, 1), "the first in-band reading");
        let mut e = Evidence::default();
        e.blocked(3);
        e.in_band(1);
        e.blocked(4);
        assert_eq!(rank(e), (1, 3), "the first limit");
    }
}

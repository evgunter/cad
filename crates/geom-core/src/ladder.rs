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
//!   datum decided apart answers. [`data_beneath_sum`] reads its data,
//!   every one decided `Zero`, beside a sum of them that did not decide
//!   `Zero`: the two straddle the band where the sum decides past it.
//! - [`climb_agreeing`] is the **agreeing ladder**: it reads every rung
//!   rather than stopping at the first answer, and hands back
//!   [`Disagree`] where two decided answers differ. Its caller says what
//!   a disagreement means.
//! - [`Evidence`] is **the one ranking of what a ladder kept** when nothing
//!   answered: a limit no tolerance moves ahead of an in-band reading.
//! - [`Refusal`] is **a refusal's standing**, stated by its own type: a
//!   type that never refuses the whole query returns `Refusal`, a rung by
//!   itself ([`Refusal::rung`]) and kept by itself ([`Evidence::keep`]);
//!   one that can returns `Result<Refusal, Aborts>`, and its caller names
//!   the answer an abort stops the ladder with ([`rung_or_abort`]). The
//!   return type is the type's claim about whether it can abort.
//!
//! The ladders own the recording, through one private discipline (`Held`)
//! that [`climb_evidence`] (and so [`climb`]) and [`climb_agreeing`]
//! share: each rung's run is held until the outcome is known, and a pass
//! ahead of every kept rung is spliced at once. The caller declares what
//! an answer may do with the kept readings ahead of it ([`Answering`]):
//! where every answering rung gives one answer, a kept reading's run is
//! spliced superseded ([`crate::k_stats::splice_superseded`]), since
//! the answer does not rest on it; where which rung answers chooses the
//! answer, it stays live, since a kept rung ahead may truly have answered
//! first. An agreeing ladder's answer supersedes its kept rungs as `Any`
//! does; its disagreement and an abort keep every run live. Every run is
//! spliced in rung order.

use crate::k_stats::{
    Detached, Magnitude, decide_magnitude_reported, decide_reported, detached, splice,
    splice_superseded,
};
use crate::predicate::{Decided, Indeterminate};
use crate::{Band, Decide, Margin, MarginDiag, Sign};

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
    /// The reading did not settle, on no margin and in a way the band
    /// moves (a coarser one may settle it): kept, and ranked after every
    /// other kept reading, so a margin in band, which says how far the
    /// question is from deciding, is the refusal ahead of it.
    Unsettled(E),
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

/// **What a refusal a ladder's reader met is to the ladder**: the one
/// classification of a refusal the ladder keeps, stated by the
/// refusal's own type beside its variants (a `refusal()` method on it),
/// so no reader spells a second table of which variant is a limit.
///
/// A refusal's standing has one of two types, and the type says which
/// a refusal can do:
///
/// - one that never refuses the whole query returns `Refusal`, and is a
///   rung by [`Refusal::rung`] alone: no answer is needed, so none can
///   be supplied;
/// - one with a variant that does (an invalid input, a kernel fault,
///   two readings that contradict) returns
///   `Result<Refusal, Aborts>`, and its caller names the answer the
///   ladder stops with ([`rung_or_abort`]).
///
/// **A standing reads DESIGN `[failure.taxonomy]`'s rows.** Row 1 is
/// read here in two parts, 1(a) and 1(b). That split is this code's
/// reading of row 1's own bullets, not a sub-row of DESIGN's:
///
/// > *Row 1 absorbs the terminal indeterminates*, but the axis is
/// > curable-vs-terminal, not bug-vs-invalid
///
/// > A site whose firing a correct kernel can reach is not a defect but a
/// > precision limit the model controls (row 1, a lever and
/// > `KERNEL_LIMIT_RECOURSE`)
///
/// - **1(a)**, the terminal half: an invalid input no lever cures,
///   including a request whose result the kernel's model never
///   represents (manifold only, bounded). It aborts.
/// - **1(b)**, the curable half: a precision limit the model controls,
///   which a lever or a tolerance cures. It is ranked by its margin where
///   it carries one, kept in band where a tighter tolerance may decide
///   it, and a limit otherwise.
///
/// Row 2 (valid input, unbuilt) is a limit, and row 4 (a kernel fault)
/// aborts. Row 3's poison flows through values: where it reaches a
/// decision, that refusal is 1(b) where input causes it and row 4 where
/// none can. A payload that mixes classes aborts until it is split.
///
/// [`Kept`] is not a standing: it is what a margin's reading leaves
/// ([`decided`]), which is never unsettled and never aborts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Refusal {
    /// Ranked by its margin ([`Kept::of`]).
    Margin(Indeterminate),
    /// In band, with no margin carried: a tighter tolerance may decide it.
    InBand,
    /// A limit no tolerance moves.
    Limit,
    /// Band-dependent with no margin ([`Rung::Unsettled`]).
    Unsettled,
}

/// **The ending a rendering of a standing composes**
/// ([`Refusal::recourse`]): which recourse, kept as a value until it is
/// rendered, since a margin's sizes its own words.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Ending {
    /// The tolerance this margin sizes, then moving the geometry
    /// ([`Indeterminate::ending`]).
    Sized(Indeterminate),
    /// Moving the geometry ([`crate::MOVE_RECOURSE`]).
    Move,
    /// A kernel approximation limit's lever
    /// ([`crate::KERNEL_LIMIT_RECOURSE`]).
    KernelLimit,
    /// No way through yet ([`crate::NOT_YET_ENDING`]).
    NotYet,
}

impl Ending {
    /// The ending's words.
    #[must_use]
    pub fn render(self) -> std::borrow::Cow<'static, str> {
        match self {
            Self::Sized(diag) => diag.ending(crate::COINCIDENCE_RECOURSE).into(),
            Self::Move => crate::MOVE_RECOURSE.into(),
            Self::KernelLimit => crate::KERNEL_LIMIT_RECOURSE.into(),
            Self::NotYet => crate::NOT_YET_ENDING.into(),
        }
    }
}

/// A refusal of the whole query ([`Refusal`]'s docs): the ladder stops
/// on it ([`Rung::Aborts`]) with the answer its caller names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aborts;

impl Refusal {
    /// **The recourse a rendering of this standing ends with** (D4 ¶1
    /// (i), DESIGN `[failure.taxonomy]` as this type's docs read it):
    /// - a reading ranked by its margin offers the tolerance that margin
    ///   sizes, then moving the geometry ([`Ending::Sized`]);
    /// - one in band with no margin, moving the geometry
    ///   ([`Ending::Move`]);
    /// - one a coarser band or a budget may settle, the kernel limit's
    ///   lever ([`Ending::KernelLimit`]);
    /// - a valid input the kernel does not read yet (row 2), no way
    ///   through yet ([`Ending::NotYet`]).
    ///
    /// An abort has no ending here: its caller names its answer.
    #[must_use]
    pub const fn recourse(self) -> Ending {
        match self {
            Self::Margin(diag) => Ending::Sized(diag),
            Self::InBand => Ending::Move,
            Self::Unsettled => Ending::KernelLimit,
            Self::Limit => Ending::NotYet,
        }
    }

    /// **A crease reading's standing**, stated once for every reader of
    /// one (a spline chart's normal at a point within the band of a line
    /// its normal is not continuous across, or of a pole): a distance in
    /// band is ranked by its margin, and one decided zero is a limit no
    /// tolerance moves, the pose not read (`[failure.taxonomy]` row 2).
    /// The reading is `geom_brep::dihedral_side::CreaseReading`, spelled
    /// out here, whose crate this one does not see.
    #[must_use]
    pub const fn of_crease(reading: Result<crate::MarginDiag, crate::Indeterminate>) -> Self {
        match reading {
            Err(diag) => Self::Margin(diag),
            Ok(_) => Self::Limit,
        }
    }

    /// **A reading within the band of a cone's apex**, stated once for
    /// every reader of one: in band. It carries no reading to tell a point
    /// at the apex from one in band of the axis, and a tighter tolerance
    /// may part the latter from it (row 1(b) above).
    #[must_use]
    pub const fn of_apex() -> Self {
        Self::InBand
    }

    /// The rung a reading refused with `e` stands as: kept by its kind.
    pub fn rung<A, E>(self, e: E) -> Rung<A, E> {
        self.kept(e).map_or_else(Rung::Unsettled, Rung::from)
    }

    /// The one table of a refusal's kind: `e` kept in band or as a
    /// limit, or `Err(e)` where it is unsettled.
    fn kept<E>(self, e: E) -> Result<Kept<E>, E> {
        match self {
            Self::Margin(cause) => Ok(Kept::of(cause).map(|_| e)),
            Self::InBand => Ok(Kept::InBand(e)),
            Self::Limit => Ok(Kept::Blocked(e)),
            Self::Unsettled => Err(e),
        }
    }
}

/// **The rung a reading refused with `e` stands as**, for a refusal that
/// may abort: kept by its kind ([`Refusal::rung`]), or, where it
/// refuses the whole query, stopping the ladder with `abort(e)`. The one
/// home of the rule. Its standing is read already, so the reading kept
/// may be another value than the one the standing was read off (a
/// refusal wrapped in its caller's answer, say). Where the two are one
/// value, [`Rung::refused`] says it, reading the standing off `e`.
pub fn rung_or_abort<A, E>(
    standing: Result<Refusal, Aborts>,
    e: E,
    abort: impl FnOnce(E) -> A,
) -> Rung<A, E> {
    match standing {
        Ok(refusal) => refusal.rung(e),
        Err(Aborts) => Rung::Aborts(abort(e)),
    }
}

impl<E> Kept<E> {
    /// The same kept kind over `f`'s reading: a reader whose refusal
    /// carries more than the cause it was ranked by.
    pub fn map<F>(self, f: impl FnOnce(E) -> F) -> Kept<F> {
        match self {
            Self::InBand(e) => Kept::InBand(f(e)),
            Self::Blocked(e) => Kept::Blocked(f(e)),
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
    let margin = if crate::is_finite_length(margin.value()) {
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
    /// band, passed to the later rungs), or did not decide (kept): an
    /// [`Indeterminate`] stands by its margin and never aborts, so the
    /// answer is `a` itself, with no refusal beside it.
    #[must_use]
    pub fn of_reading(reading: Result<Option<A>, Indeterminate>) -> Self {
        Self::of_option(reading, Self::kept)
    }

    /// A reading the band did not decide, kept as what it is
    /// ([`Kept::of`]).
    #[must_use]
    pub fn kept(cause: Indeterminate) -> Self {
        Kept::of(cause).into()
    }
}

impl<A, E> Rung<A, E> {
    /// **A reading of an `Option` as its rung**, the one door for it:
    /// `Some(a)` answers `a`, `None` passes, and a refusal stands as
    /// `refused` says. [`Self::of_reading`] and [`Rung::answers`] read
    /// through it; they differ only in what a refusal can do, so each
    /// names that and nothing else.
    pub fn of_option<R>(reading: Result<Option<A>, R>, refused: impl FnOnce(R) -> Self) -> Self {
        match reading {
            Ok(Some(a)) => Self::Answers(a),
            Ok(None) => Self::Passes,
            Err(e) => refused(e),
        }
    }

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
            Self::Unsettled(e) => Rung::Unsettled(f(e)),
        }
    }
}

impl<A, E> Rung<Result<A, E>, E> {
    /// **A reading of whether one rung holds** as its rung: holding
    /// answers `Ok(yes)`, not holding passes, and a refusal stands by
    /// `standing` ([`Self::answers`]). The one home of the reader that
    /// answers a `bool` over a refusal with a standing.
    pub fn holds(
        reading: Result<bool, E>,
        yes: A,
        standing: impl FnOnce(&E) -> Result<Refusal, Aborts>,
    ) -> Self {
        Self::answers(reading.map(|held| held.then_some(yes)), standing)
    }

    /// **A reading that answers `Some(a)`** as its rung: `Some` answers
    /// `Ok(a)`, `None` passes, and a refusal stands by `standing`
    /// ([`Self::refused`]): [`Self::holds`] where the answer is read
    /// rather than fixed.
    pub fn answers(
        reading: Result<Option<A>, E>,
        standing: impl FnOnce(&E) -> Result<Refusal, Aborts>,
    ) -> Self {
        Self::of_option(reading.map(|a| a.map(Ok)), |e| {
            Self::refused(e, standing, Err)
        })
    }
}

impl<A, E> Rung<A, E> {
    /// **The rung a reading refused with `e` stands as**, its standing
    /// read off `e` itself: kept by its kind, or, where it refuses the
    /// whole query, stopping the ladder with `abort(e)`
    /// ([`rung_or_abort`], the rule's one home, which this reads). The
    /// shape for a rung whose kept reading and whose standing are one
    /// value; a caller that keeps another value than the one its
    /// standing is read off calls [`rung_or_abort`] itself. `abort` is
    /// `Err` for a ladder answering `Result`, the identity for one whose
    /// abort is the refusal itself.
    pub fn refused(
        e: E,
        standing: impl FnOnce(&E) -> Result<Refusal, Aborts>,
        abort: impl FnOnce(E) -> A,
    ) -> Self {
        rung_or_abort(standing(&e), e, abort)
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
    ///
    /// The answer may carry which rung gave it (its witness), but only as
    /// a diagnostic: a name a refusal or a record shows, never a value a
    /// later decision reads. A payload that decides anything downstream
    /// makes the ladder [`Self::First`].
    Any,
    /// Which rung answers chooses the answer: the first candidate within,
    /// the first member that projects. A kept reading ahead of the answer
    /// may truly have answered first, so the answer rests on it and its
    /// run is spliced live.
    ///
    /// **When a "which" answer is given past a kept rung, and when it is
    /// not** (the one statement of the rule; PR 6553's `place_off`
    /// ruling). A consumer that reads WHICH rung answered asks one of two
    /// questions, and a site tells them apart by asking: *can two rungs
    /// both answer, by the door's premise, within the band?* (Not "in
    /// exact arithmetic": two answers a band apart are two answers.)
    ///
    /// - **At most one can** (the answer is unique by the door's premise:
    ///   `rim_wedge::shared_rim`'s one shared rim, where two would be no
    ///   rim contact the door claims to see). A later rung decided to
    ///   answer shows the kept rung ahead was not one, so the ladder gives
    ///   it here, under `First`, its kept runs live.
    /// - **Several can, and the consumer reads the first in order** (the
    ///   cell holding a point, `CarrierLoop::place_off`'s edge; the cut
    ///   that rotates a pairing, `splitting::join::conic_pairs`; the end
    ///   whose spread is read next, `boolean::reduce`'s line-cone apex end;
    ///   the surface carrying an iso, `boolean::edge_join::joinable`). A
    ///   kept rung ahead may be the answer itself, so no later rung
    ///   answers past it: the site refuses at its first kept rung, which
    ///   is the plain `?`, not a climb.
    First,
}

/// What a ladder read ([`climb`]).
#[derive(Clone, Debug, PartialEq)]
pub enum Climb<A, E> {
    /// A rung answered: its index among the rungs, and its answer.
    Answered {
        /// The answering rung's index. Under [`Answering::Any`] it is a
        /// diagnostic only: which rung answered rests on rung order, so no
        /// caller branches on it.
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
        Err(kept) => kept.refusal().map_or(Climb::Passed, Climb::Undecided),
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
    let mut held = Held::default();
    for (at, rung) in rungs.into_iter().enumerate() {
        let (got, log) = detached(|| read(rung));
        let Some((answer, aborts)) = held.take(got, log) else {
            // The first-refusal mutant (`scripts/any-first-mutant.sh`): an
            // `Any` ladder refuses at its first kept rung, so a pin that
            // stays green under it does not reach the answer past one.
            #[cfg(any_first_mutant)]
            if answering == Answering::Any && !held.kept.is_empty() {
                return Err(held.splice_all(false));
            }
            continue;
        };
        held.splice_all(!aborts && answering == Answering::Any);
        return Ok((at, answer));
    }
    Err(held.splice_all(false))
}

/// **The runs a ladder has read, held in rung order** until its outcome
/// is known, and what its kept rungs kept ([`Evidence`]): the one
/// recording discipline of [`climb_evidence`] and [`climb_agreeing`].
struct Held<E> {
    kept: Evidence<E>,
    /// Each run read since the first kept one, with whether it was kept.
    runs: Vec<(Detached, bool)>,
}

impl<E> Default for Held<E> {
    fn default() -> Self {
        Self {
            kept: Evidence::default(),
            runs: Vec::new(),
        }
    }
}

impl<E> Held<E> {
    /// Takes one rung's reading and its run: a passing or kept rung is
    /// held (a pass ahead of every kept one spliced at once), an answer
    /// or an abort is held live and handed back, with whether it aborts.
    fn take<A>(&mut self, got: Rung<A, E>, log: Detached) -> Option<(A, bool)> {
        let (kept, answer) = match got {
            Rung::Answers(answer) => (false, Some((answer, false))),
            Rung::Aborts(answer) => (false, Some((answer, true))),
            Rung::Passes if self.runs.is_empty() => {
                splice(log);
                return None;
            }
            Rung::Passes => (false, None),
            Rung::InBand(e) => {
                self.kept.in_band(e);
                (true, None)
            }
            Rung::Blocked(e) => {
                self.kept.blocked(e);
                (true, None)
            }
            Rung::Unsettled(e) => {
                self.kept.unsettled(e);
                (true, None)
            }
        };
        self.runs.push((log, kept));
        answer
    }

    /// Splices every held run in rung order, a kept one superseded where
    /// `supersede` (the answer does not rest on it), and hands back what
    /// was kept.
    fn splice_all(self, supersede: bool) -> Evidence<E> {
        for (run, kept) in self.runs {
            if kept && supersede {
                splice_superseded(run);
            } else {
                splice(run);
            }
        }
        self.kept
    }
}

/// Two decided answers of one agreeing ladder that differ
/// ([`climb_agreeing`]): the first, and the one that disagreed with it.
#[derive(Clone, Debug, PartialEq)]
pub struct Disagree<A> {
    /// The first answer read.
    pub first: A,
    /// The later answer that differs from it.
    pub other: A,
}

/// **The ladder that reads every rung and checks its answers agree**:
/// [`Answering::Any`]'s premise, every answering rung gives one answer,
/// read rather than assumed: `agree` says whether two answers are one.
/// Every rung is read (an abort still stops it). Where every decided
/// answer agrees, that answer. A rung kept in band beside it is spliced
/// superseded: the agreement was checked among the decided rungs only,
/// and the kept rung's own answer is unread, so the record says the
/// answer does not rest on it, not that it agreed. Where none answers,
/// the kept readings ranked ([`Evidence`]) or [`Climb::Passed`].
///
/// # Errors
///
/// [`Disagree`] where two decided answers differ, with every run read
/// spliced live. What a disagreement means is the caller's: a fault where
/// the answers are readings of one fact (`seam_germ_side`, `tangent_lump`,
/// which map it to `ClassificationInvariant`), or a valid answer of its
/// own where two answers differing is the answer (a mixed departure).
pub fn climb_agreeing<I, A, E>(
    rungs: impl IntoIterator<Item = I>,
    mut read: impl FnMut(I) -> Rung<A, E>,
    agree: impl Fn(&A, &A) -> bool,
) -> Result<Climb<A, E>, Disagree<A>> {
    let mut held = Held::default();
    let mut first: Option<(usize, A)> = None;
    for (at, rung) in rungs.into_iter().enumerate() {
        let (got, log) = detached(|| read(rung));
        let Some((answer, aborts)) = held.take(got, log) else {
            continue;
        };
        if aborts {
            held.splice_all(false);
            return Ok(Climb::Answered { at, answer });
        }
        match first {
            None => first = Some((at, answer)),
            Some((_, ref seen)) if agree(seen, &answer) => {}
            Some((_, seen)) => {
                held.splice_all(false);
                return Err(Disagree {
                    first: seen,
                    other: answer,
                });
            }
        }
    }
    Ok(match first {
        Some((at, answer)) => {
            held.splice_all(true);
            Climb::Answered { at, answer }
        }
        None => held
            .splice_all(false)
            .refusal()
            .map_or(Climb::Passed, Climb::Undecided),
    })
}

/// What an identity ladder's data answer ([`read_identity`]): a
/// [`Climb`] in the identity's own terms, which carry what `Climb` does
/// not — the answering datum's sign, and the data's joint margin where
/// the identity holds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IdentityLadder {
    /// A datum decided past the band: the identity does not hold. `at` is
    /// its index in the data, `decided` its reading.
    Apart {
        /// The answering datum's index.
        at: usize,
        /// Its reading: the sign (never [`Sign::Zero`]) and the margin
        /// it was decided on.
        decided: Decided,
    },
    /// Every datum decided `Zero`: the identity holds. Its margin is the
    /// first datum's reading, proven only where every datum's is
    /// ([`MarginDiag::jointly`]); not the largest of them (`None` for no
    /// data).
    Zero(Option<MarginDiag>),
    /// No datum decided past the band and one did not decide: the first
    /// that cannot be read (its margin poisoned), else the first in band.
    Undecided(Indeterminate),
}

impl IdentityLadder {
    /// The identity as a yes-or-no reading: `true` where every datum
    /// decided `Zero`, `false` where one decided apart.
    ///
    /// # Errors
    ///
    /// The kept reading, where none decided apart and one did not decide.
    pub fn holds(self) -> Result<bool, Indeterminate> {
        match self {
            Self::Apart { .. } => Ok(false),
            Self::Zero(_) => Ok(true),
            Self::Undecided(cause) => Err(cause),
        }
    }
}

/// **Whether two descriptions are one**, read off `data` that are each
/// zero exactly where the two agree, every datum decided on `band` under
/// its own name: the ladder ([`climb`], [`Answering::Any`]: every datum
/// decided apart says the same "not one") whose answer is a datum decided
/// apart.
///
/// **A datum's name is spelled once.** `test_utils::source::predicate_census`
/// cannot follow the data to their names, so it reports every call of
/// this function as an indirect site; a crate whose roster runs it
/// (sweep's and topo's `recourse_roster`) lists each call and spells
/// each datum's name in a `const` the roster declares
/// (`NameCarrier::Const`: sweep's `LOFT_RIM_PINNED`, topo's
/// `BOOL_WRAP_RIM`), under its door's prefix where the roster keys doors
/// by prefix (topo's).
pub fn read_identity<T: Decide>(
    data: impl IntoIterator<Item = (&'static str, Margin<T>)>,
    band: Band,
) -> IdentityLadder {
    let mut joint: Option<MarginDiag> = None;
    let climbed = climb(Answering::Any, data, |(name, margin)| {
        Rung::decide(name, margin, band, |decided| match decided.sign {
            Sign::Positive | Sign::Negative => Some(decided),
            Sign::Zero => {
                joint = Some(joint.map_or(decided.margin, |first| first.jointly(decided.margin)));
                None
            }
        })
    });
    match climbed {
        Climb::Answered {
            at,
            answer: decided,
        } => IdentityLadder::Apart { at, decided },
        Climb::Passed => IdentityLadder::Zero(joint),
        Climb::Undecided(cause) => IdentityLadder::Undecided(cause),
    }
}

/// **An identity's data beneath a sum that did not decide `Zero`.**
/// Every datum decided `Zero`, and each bounds from below how far the two
/// descriptions stand apart somewhere on the whole carriers, their
/// separation; `upper`, the reading of a sum of the data's magnitudes,
/// bounds the displacement from above. Past the band it straddles the
/// band with the data, under the site's `straddle` row
/// ([`crate::k_stats::straddle`]); short of deciding, it is its own
/// reading. `data` is the data's `Zero` as [`IdentityLadder::Zero`] keeps
/// it, the first datum's reading, so the straddle's low end is that
/// datum's reading and not the largest of them.
///
/// A datum bounds the carriers' separation, not the displacement over a
/// part of them: two spheres tangent inside each other stand within the
/// band near the tangency while their centre datum reads the whole
/// offset. So over a consumed extent the low end is the carriers', and
/// the straddle says only that neither bound answers for the extent.
///
/// # Panics
///
/// Where `upper` decided `Zero`, the caller's one carrier, or
/// `Negative`, which a sum of magnitudes never reads
/// ([`crate::k_stats::straddle`]'s panic).
pub fn data_beneath_sum(
    straddle: &'static str,
    data: MarginDiag,
    upper: Result<Decided, Indeterminate>,
    band: Band,
) -> Indeterminate {
    match upper {
        Ok(upper) => crate::k_stats::straddle(
            straddle,
            Decided {
                sign: Sign::Zero,
                margin: data,
            },
            upper,
            band,
        ),
        Err(diag) => diag,
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
/// 3. else the first unsettled reading ([`Rung::Unsettled`]): it carries
///    no margin, and the band moves it;
/// 4. else neither, and the caller says what that means.
#[derive(Debug)]
pub struct Evidence<E> {
    blocked: Option<E>,
    in_band: Option<E>,
    unsettled: Option<E>,
}

/// [`Evidence`]'s verdict.
#[derive(Debug)]
pub enum Ranked<E> {
    /// The first limit met.
    Blocked(E),
    /// No limit met; the first in-band reading.
    InBand(E),
    /// Neither a limit nor a margin in band; the first unsettled reading.
    Unsettled(E),
    /// Neither was kept.
    Neither,
}

impl<E> Default for Evidence<E> {
    fn default() -> Self {
        Self {
            blocked: None,
            in_band: None,
            unsettled: None,
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

    /// Keeps `e` as the kind `refusal` keeps it as (its rung,
    /// [`Refusal::rung`]), if it is the first of its kind: for a run of
    /// readings outside a climb, which no reading of answers (the cuts a
    /// later search reads its candidates off).
    pub fn keep(&mut self, refusal: Refusal, e: E) {
        match refusal.kept(e) {
            Ok(Kept::InBand(e)) => self.in_band(e),
            Ok(Kept::Blocked(e)) => self.blocked(e),
            Err(e) => self.unsettled(e),
        }
    }

    /// Keeps `e` as what `standing` says it is ([`Self::keep`]), for a
    /// refusal that may abort ([`rung_or_abort`]'s twin outside a climb).
    ///
    /// # Errors
    ///
    /// `e`, where it refuses the whole query: the run stops on it.
    pub fn keep_or_abort(&mut self, standing: Result<Refusal, Aborts>, e: E) -> Result<(), E> {
        match standing {
            Ok(refusal) => {
                self.keep(refusal, e);
                Ok(())
            }
            Err(Aborts) => Err(e),
        }
    }

    /// Keeps `e` as an unsettled reading ([`Rung::Unsettled`]), if it is
    /// the first.
    pub fn unsettled(&mut self, e: E) {
        self.unsettled.get_or_insert(e);
    }

    /// `self`'s readings first, then `later`'s: the evidence of two runs
    /// read in that order.
    #[must_use]
    pub fn then(self, later: Self) -> Self {
        Self {
            blocked: self.blocked.or(later.blocked),
            in_band: self.in_band.or(later.in_band),
            unsettled: self.unsettled.or(later.unsettled),
        }
    }

    /// Whether nothing was kept: [`Self::ranked`] reads
    /// [`Ranked::Neither`].
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.blocked.is_none() && self.in_band.is_none() && self.unsettled.is_none()
    }

    /// Whether a limit was kept: [`Self::ranked`] reads
    /// [`Ranked::Blocked`].
    #[must_use]
    pub const fn met_a_limit(&self) -> bool {
        self.blocked.is_some()
    }

    /// The refusal the ranking names, of whatever kind; `None` where
    /// nothing was kept. For a caller that refuses alike on every kind.
    #[must_use]
    pub fn refusal(self) -> Option<E> {
        match self.ranked() {
            Ranked::Blocked(e) | Ranked::InBand(e) | Ranked::Unsettled(e) => Some(e),
            Ranked::Neither => None,
        }
    }

    /// **The evidence as one rung** of an outer ladder, for a ladder
    /// read inside one reading: the ranked kept reading as what it was
    /// kept as, and a pass where nothing was kept.
    #[must_use]
    pub fn into_rung<A>(self) -> Rung<A, E> {
        match self.ranked() {
            Ranked::Blocked(e) => Rung::Blocked(e),
            Ranked::InBand(e) => Rung::InBand(e),
            Ranked::Unsettled(e) => Rung::Unsettled(e),
            Ranked::Neither => Rung::Passes,
        }
    }

    /// The one ranking (the type's docs).
    #[must_use]
    pub fn ranked(self) -> Ranked<E> {
        match (self.blocked, self.in_band, self.unsettled) {
            (Some(e), _, _) => Ranked::Blocked(e),
            (None, Some(e), _) => Ranked::InBand(e),
            (None, None, Some(e)) => Ranked::Unsettled(e),
            (None, None, None) => Ranked::Neither,
        }
    }
}

/// **What an argmin's caller has proved its minimum free of**
/// ([`argmin_decided`]): which undecided pairs among the candidates left
/// in contention cannot change what the caller does with the minimum, so
/// that the first of them in candidate order may stand for it. The
/// levels nest: `Band` covers what `Ties` does.
///
/// Every declaration above [`FreeOf::Nothing`] carries its proof at the
/// call.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FreeOf {
    /// Nothing: any two candidates left in contention refuse.
    Nothing,
    /// Exact ties: candidates decided equally near may stand for each
    /// other.
    Ties,
    /// Ties and in-band readings: candidates no reading parts may stand
    /// for each other.
    Band,
}

/// **A comparison [`argmin_decided`] could not decide**, as the caller's
/// comparison states it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Undecided<E> {
    /// In band: a tighter tolerance may decide it. The argmin keeps it,
    /// and [`FreeOf::Band`] covers it.
    InBand(E),
    /// Any other undecided reading (a poisoned margin, an enclosure
    /// straddling the band's edge): the argmin stops on it at once.
    Aborts(E),
}

impl Undecided<Indeterminate> {
    /// **The one classifier of an undecided reading for
    /// [`argmin_decided`]**: in band ([`in_band`]) is kept, and a later
    /// candidate may close it; any other (a poisoned margin, an enclosure
    /// straddling the band's edge) stops the argmin. Every caller whose
    /// comparison reads a margin classifies through here, so a poison is
    /// never superseded away.
    #[must_use]
    pub fn of(diag: Indeterminate) -> Self {
        if in_band(&diag) {
            Self::InBand(diag)
        } else {
            Self::Aborts(diag)
        }
    }
}

impl<E> Undecided<E> {
    /// The same classification over another payload.
    #[must_use]
    pub fn map<F>(self, f: impl FnOnce(E) -> F) -> Undecided<F> {
        match self {
            Self::InBand(e) => Undecided::InBand(f(e)),
            Self::Aborts(e) => Undecided::Aborts(f(e)),
        }
    }
}

/// **Whether an undecided reading lies inside the band**: a point
/// margin, or an enclosure wholly inside one sliver band
/// ([`Indeterminate::terminal_sliver`]). An enclosure straddling the
/// band's edge, or a poisoned margin, is not.
#[must_use]
pub fn in_band(diag: &Indeterminate) -> bool {
    match diag.margin.kind() {
        crate::MarginKind::Value => true,
        crate::MarginKind::Enclosure => diag.terminal_sliver,
        crate::MarginKind::Invalid => false,
    }
}

/// **Why [`argmin_decided`] picked no minimum.**
#[derive(Clone, Debug, PartialEq)]
pub enum Unpicked<C, E> {
    /// Several candidates are left in contention, and what the caller is
    /// free of does not cover them: every one of them, in candidate
    /// order, and the first in-band reading among them, if any. With
    /// none, they are exact ties the caller refuses as its own.
    Contested {
        /// The candidates no candidate is decided nearer than.
        members: Vec<C>,
        /// The first in-band reading between two of them.
        in_band: Option<E>,
    },
    /// A comparison stopped the argmin ([`Undecided::Aborts`]).
    Aborted(E),
    /// Every candidate is decided farther than some other: the
    /// comparison ranks in a cycle, which no distance does. A kernel
    /// fault in the caller's `cmp` (row 4), never a minimum of none.
    Cyclic,
}

/// **The nearest of `candidates` by decided comparisons.** `cmp(a, b)`
/// is the sign of `a`'s distance less `b`'s: `Negative` where `a` is
/// decided nearer, `Zero` for a decided tie, or an [`Undecided`] reading.
///
/// **The set rule.** A candidate drops out only where some candidate is
/// decided nearer than it, read directly between the two. Every pair is
/// compared, so what is left in contention is the same in every order of
/// the candidates. Each pair left in contention is a tie or undecided.
///
/// **The ending.** One candidate left is the minimum. Several are, where
/// `free_of` covers every pair between them ([`FreeOf`]): then the first
/// in candidate order stands for them. Otherwise they are
/// [`Unpicked::Contested`].
///
/// **How it differs from [`climb`] under [`Answering::First`]**: a
/// ladder wants the first rung that answers, so a kept rung ahead of it
/// may be that answer, and it refuses there. An argmin wants the minimum,
/// so a later candidate can close the question: one decided nearer than
/// both of an undecided pair puts neither of them in contention.
///
/// **What it presupposes**: `cmp` is antisymmetric (`cmp(a, b)` is
/// `cmp(b, a)` negated, and undecided where it is) and ranks in no
/// cycle, as a sign of one distance less another does. Order-freedom
/// rests on the first; a cycle, where every candidate drops out, aborts
/// ([`Unpicked::Cyclic`]).
///
/// **Recording**, in pair order, as a serial read of the same pairs
/// would: a decided reading live; an undecided one superseded where the
/// minimum does not rest on it (a candidate of the pair dropped out, or
/// `free_of` covers it at the ending), and live otherwise. An abort
/// splices every run read so far live.
///
/// # Errors
///
/// [`Unpicked`].
pub fn argmin_decided<C, E>(
    candidates: impl IntoIterator<Item = C>,
    mut cmp: impl FnMut(&C, &C) -> Result<Sign, Undecided<E>>,
    free_of: FreeOf,
) -> Result<Option<C>, Unpicked<C, E>> {
    /// A pair's reading, held with its run until the ending.
    enum Read<E> {
        Decided,
        InBand(E),
    }
    let candidates: Vec<C> = candidates.into_iter().collect();
    let n = candidates.len();
    let mut out = vec![false; n];
    let mut reads: Vec<((usize, usize), Read<E>, Detached)> = Vec::new();
    // Every pair, a retired candidate's included: comparing only against
    // the ones still in contention is not free of order. With `a` and `b`
    // in band of each other and `c` decided farther than `a` but nearer
    // than `b`, the order `a, b, c` retires `b` against `c` and leaves `a`;
    // the order `c, a, b` retires `c` against `a` and then meets `b` only
    // against `a`, in band, leaving both.
    for i in 0..n {
        for j in i + 1..n {
            let (got, log) = detached(|| cmp(&candidates[i], &candidates[j]));
            match got {
                Ok(sign) => {
                    match sign {
                        Sign::Negative => out[j] = true,
                        Sign::Positive => out[i] = true,
                        Sign::Zero => {}
                    }
                    reads.push(((i, j), Read::Decided, log));
                }
                Err(Undecided::InBand(e)) => reads.push(((i, j), Read::InBand(e), log)),
                Err(Undecided::Aborts(e)) => {
                    for (_, _, run) in reads {
                        splice(run);
                    }
                    splice(log);
                    return Err(Unpicked::Aborted(e));
                }
            }
        }
    }
    let live = |(i, j): (usize, usize)| !out[i] && !out[j];
    let tied = reads
        .iter()
        .filter(|&&(pair, ..)| live(pair))
        .all(|(_, read, _)| matches!(read, Read::Decided));
    let covered = match free_of {
        FreeOf::Nothing => false,
        FreeOf::Ties => tied,
        FreeOf::Band => true,
    };
    let mut in_band = None;
    for (pair, read, run) in reads {
        match read {
            Read::Decided => splice(run),
            Read::InBand(e) if live(pair) && !covered => {
                splice(run);
                in_band.get_or_insert(e);
            }
            Read::InBand(_) => splice_superseded(run),
        }
    }
    let mut members: Vec<C> = candidates
        .into_iter()
        .zip(out)
        .filter_map(|(c, out)| (!out).then_some(c))
        .collect();
    if n > 0 && members.is_empty() {
        return Err(Unpicked::Cyclic);
    }
    if members.len() <= 1 || covered {
        return Ok((!members.is_empty()).then(|| members.swap_remove(0)));
    }
    Err(Unpicked::Contested { members, in_band })
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use crate::k_stats::Bracket;
    use crate::predicate::{Band, Indeterminate};
    use crate::{Margin, MarginDiag, Sign};

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
                .map(crate::k_stats::Escalation::predicate)
                .collect(),
        )
    }

    /// The identity ladder over `data`, recorded.
    fn read_as(
        data: &[(&'static str, f64)],
    ) -> (IdentityLadder, Vec<&'static str>, Vec<&'static str>) {
        recorded(|| read_identity(data.iter().map(|&(n, m)| (n, Margin::of(m))), band()))
    }

    /// **`Evidence::refusal` names the ranked refusal**, whatever its
    /// kind: a limit ahead of an in-band reading read before it, in band
    /// ahead of unsettled, and none where nothing was kept.
    #[test]
    fn evidence_refusal_is_the_ranked_one_of_any_kind() {
        use super::Refusal;
        let mut e = Evidence::default();
        e.unsettled("unsettled");
        e.keep(Refusal::InBand, "in band");
        e.keep(Refusal::Limit, "limit");
        assert_eq!(
            e.refusal(),
            Some("limit"),
            "a limit outranks what came before it"
        );
        let mut e = Evidence::default();
        e.unsettled("unsettled");
        e.keep(Refusal::InBand, "in band");
        assert_eq!(e.refusal(), Some("in band"), "in band outranks unsettled");
        let mut e = Evidence::default();
        e.unsettled("unsettled");
        assert_eq!(e.refusal(), Some("unsettled"), "unsettled is a refusal");
        assert_eq!(Evidence::<&str>::default().refusal(), None, "nothing kept");
    }

    /// **Each standing is kept as its kind** ([`super::Refusal::rung`]):
    /// a poisoned margin as a limit and a finite one in band; and a
    /// standing that aborts stops the ladder with the answer its caller
    /// names.
    #[test]
    fn each_standing_is_kept_as_its_kind() {
        use super::{Aborts, Refusal, rung_or_abort};
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: Some("t"),
            terminal_sliver: false,
        };
        let standings = [
            (Refusal::Margin(cause(MarginDiag::value(5e-9))), "in band"),
            (Refusal::Margin(cause(MarginDiag::INVALID)), "blocked"),
            (Refusal::InBand, "in band"),
            (Refusal::Limit, "blocked"),
            (Refusal::Unsettled, "unsettled"),
        ];
        for (refusal, kind) in standings {
            let as_rung = match refusal.rung::<(), _>(()) {
                Rung::InBand(_) => "in band",
                Rung::Blocked(_) => "blocked",
                Rung::Unsettled(_) => "unsettled",
                other => panic!("{refusal:?} is kept, not {other:?}"),
            };
            assert_eq!(as_rung, kind, "{refusal:?} as a rung");
            assert_eq!(
                rung_or_abort::<Result<(), &str>, _>(Ok(refusal), kind, Err),
                refusal.rung(kind),
                "{refusal:?} that does not abort is its rung"
            );
        }
        assert_eq!(
            rung_or_abort::<Result<(), &str>, _>(Err(Aborts), "corrupt", Err),
            Rung::Aborts(Err("corrupt")),
            "an abort stops the ladder with its caller's answer"
        );
    }

    /// **A refused reading stands as its own standing says**, whatever
    /// the ladder's abort map: kept by its kind, or stopping the ladder
    /// with the map of the refusal itself, `Err` for a ladder answering
    /// `Result` and the identity for one whose abort is the refusal.
    #[test]
    fn a_refused_reading_stands_by_its_own_standing() {
        use super::{Aborts, Refusal};
        let standing = |e: &&str| match *e {
            "corrupt" => Err(Aborts),
            "limit" => Ok(Refusal::Limit),
            _ => Ok(Refusal::InBand),
        };
        assert_eq!(
            Rung::<Result<(), &str>, &str>::refused("corrupt", standing, Err),
            Rung::Aborts(Err("corrupt")),
            "an abort under Err stops with Err(refusal)"
        );
        assert_eq!(
            Rung::<&str, &str>::refused("corrupt", standing, core::convert::identity),
            Rung::Aborts("corrupt"),
            "an abort under the identity stops with the refusal"
        );
        assert_eq!(
            Rung::<&str, &str>::refused("limit", standing, core::convert::identity),
            Rung::Blocked("limit"),
            "a limit is kept under the identity"
        );
        assert_eq!(
            Rung::<Result<(), &str>, &str>::refused("limit", standing, Err),
            Rung::Blocked("limit"),
            "a limit is kept under Err"
        );
        assert_eq!(
            Rung::<Result<(), &str>, &str>::refused("near", standing, Err),
            Rung::InBand("near"),
            "an in-band refusal is kept in band"
        );
    }

    /// **A kept limit does not stop a ladder no reading answers**: a
    /// later reading that refuses the whole query is its outcome; with
    /// none, the limit is, ranked ahead of an in-band reading kept before
    /// it.
    #[test]
    fn a_later_abort_outranks_a_kept_limit() {
        use super::climb_evidence;
        let rungs = |later: Rung<&'static str, &'static str>| {
            [Rung::InBand("near"), Rung::Blocked("limit"), later]
        };
        assert_eq!(
            climb_evidence(Answering::First, rungs(Rung::Aborts("corrupt")), |r| r).ok(),
            Some((2, "corrupt")),
            "the later abort is the outcome"
        );
        let kept = climb_evidence(Answering::First, rungs(Rung::Passes), |r| r)
            .expect_err("no reading answers or aborts");
        assert!(!kept.is_empty(), "the kept readings are evidence");
        assert_eq!(
            kept.refusal(),
            Some("limit"),
            "the limit ranks ahead of in band"
        );
    }

    /// **Evidence as one rung** keeps its ranked reading as what it was
    /// kept as, and passes where nothing was kept; only empty evidence
    /// is empty.
    #[test]
    fn evidence_is_one_rung_of_its_ranked_kind() {
        let mut e = Evidence::default();
        assert!(e.is_empty(), "nothing kept");
        assert_eq!(Evidence::<&str>::default().into_rung::<()>(), Rung::Passes);
        e.unsettled("unsettled");
        assert!(!e.is_empty(), "an unsettled reading kept");
        e.in_band("in band");
        let mut in_band = Evidence::default();
        in_band.unsettled("unsettled");
        in_band.in_band("in band");
        assert_eq!(in_band.into_rung::<()>(), Rung::InBand("in band"));
        e.blocked("limit");
        assert_eq!(e.into_rung::<()>(), Rung::Blocked("limit"));
        let mut unsettled = Evidence::default();
        unsettled.unsettled("unsettled");
        assert_eq!(unsettled.into_rung::<()>(), Rung::Unsettled("unsettled"));
    }

    /// **An agreeing ladder reads every rung**: answers that agree give
    /// that answer, with the kept readings beside them superseded on the
    /// record; two that differ disagree; an abort stops it; and with no
    /// answer, the kept readings rank as any ladder's do.
    #[test]
    fn an_agreeing_ladder_reads_every_rung_and_checks_its_answers() {
        use super::{Disagree, climb_agreeing};
        let rung = |r: Rung<&'static str, &'static str>| r;
        assert_eq!(
            climb_agreeing(
                [
                    Rung::InBand("kept"),
                    Rung::Answers("in"),
                    Rung::Passes,
                    Rung::Answers("in")
                ],
                rung,
                PartialEq::eq
            ),
            Ok(Climb::Answered {
                at: 1,
                answer: "in"
            }),
            "answers that agree give that answer"
        );
        assert_eq!(
            climb_agreeing(
                [
                    Rung::Answers("in"),
                    Rung::InBand("kept"),
                    Rung::Answers("out")
                ],
                rung,
                PartialEq::eq
            ),
            Err(Disagree {
                first: "in",
                other: "out"
            }),
            "two answers that differ disagree"
        );
        assert_eq!(
            climb_agreeing(
                [
                    Rung::Answers("in"),
                    Rung::Aborts("corrupt"),
                    Rung::Answers("out")
                ],
                rung,
                PartialEq::eq
            ),
            Ok(Climb::Answered {
                at: 1,
                answer: "corrupt"
            }),
            "an abort stops the reading"
        );
        assert_eq!(
            climb_agreeing(
                [
                    Rung::InBand("in band"),
                    Rung::Blocked("limit"),
                    Rung::Passes
                ],
                rung,
                PartialEq::eq
            ),
            Ok(Climb::Undecided("limit")),
            "with no answer, the kept readings rank"
        );
        let (got, verdicts, escalations) = recorded(|| {
            climb_agreeing(
                [("a", 5e-9), ("b", 0.0), ("c", 0.0)],
                |(name, m): (&'static str, f64)| {
                    Rung::<bool, crate::predicate::Indeterminate>::decide(
                        name,
                        Margin::of(m),
                        band(),
                        |d| (d.sign == Sign::Zero).then_some(true),
                    )
                },
                PartialEq::eq,
            )
        });
        assert!(
            matches!(got, Ok(Climb::Answered { answer: true, .. })),
            "{got:?}"
        );
        assert!(
            escalations.is_empty() && verdicts.contains(&"b") && verdicts.contains(&"c"),
            "the kept reading is superseded, every answer's verdict stands: \
             {verdicts:?} {escalations:?}"
        );
    }

    /// **A later definite datum answers past an earlier in-band one**,
    /// and the kept reading's escalation leaves the log, since the
    /// answer does not rest on it.
    #[test]
    fn a_later_definite_datum_answers_past_an_in_band_one() {
        let (got, verdicts, escalations) =
            read_as(&[("a", 0.0), ("b", 5e-9), ("c", 0.0), ("d", 1.0)]);
        let IdentityLadder::Apart { at: 3, decided } = got else {
            panic!("the last datum answers: {got:?}")
        };
        assert_eq!(decided.sign, Sign::Positive);
        assert_eq!(
            decided.margin,
            crate::MarginDiag::value(1.0),
            "the answer keeps the margin it was decided on"
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
        let (got, verdicts, escalations) = read_as(&[("a", 0.0), ("b", 5e-9), ("c", 0.0)]);
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
        let (got, verdicts, _) = read_as(&[("a", -1.0), ("b", 5e-9), ("c", 1.0)]);
        let IdentityLadder::Apart { at: 0, decided } = got else {
            panic!("the first datum answers: {got:?}")
        };
        assert_eq!(decided.sign, Sign::Negative);
        assert_eq!(decided.margin, crate::MarginDiag::value(-1.0));
        assert_eq!(verdicts, ["a"], "nothing past the answer is read");
        let (got, _, _) = read_as(&[("a", 0.0), ("b", 5e-10)]);
        assert!(matches!(got, IdentityLadder::Zero(Some(_))), "{got:?}");
        let (got, _, _) = read_as(&[]);
        assert_eq!(got, IdentityLadder::Zero(None));
    }

    /// **An unreadable datum is the refusal's cause ahead of an in-band
    /// one**, whichever was read first: a poisoned margin is a limit no
    /// tolerance moves. An infinite datum is unreadable too, not apart.
    #[test]
    fn an_unreadable_datum_refuses_ahead_of_an_in_band_one() {
        for poison in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let (got, _, _) = read_as(&[("a", 5e-9), ("b", poison), ("c", 0.0)]);
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
        use crate::k_stats::{self, Probe};
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
            Ranked::Unsettled(e) => (4, e),
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

    /// A decided comparison over named candidates, read off `table`'s
    /// unordered pairs (`(a, b, sign of a less b)`), in band where a pair
    /// is missing; `poison` aborts.
    fn table_cmp<'a>(
        table: &'a [(&'a str, &'a str, Sign)],
        poison: Option<(&'a str, &'a str)>,
    ) -> impl FnMut(&&'a str, &&'a str) -> Result<Sign, super::Undecided<String>> + 'a {
        use super::Undecided;
        move |a: &&str, b: &&str| {
            let (a, b) = (*a, *b);
            if poison.is_some_and(|(x, y)| (x, y) == (a, b) || (x, y) == (b, a)) {
                return Err(Undecided::Aborts(format!("poison {a}{b}")));
            }
            for &(x, y, sign) in table {
                if (x, y) == (a, b) {
                    return Ok(sign);
                }
                if (x, y) == (b, a) {
                    return Ok(match sign {
                        Sign::Negative => Sign::Positive,
                        Sign::Positive => Sign::Negative,
                        Sign::Zero => Sign::Zero,
                    });
                }
            }
            Err(Undecided::InBand(format!("{a}{b}")))
        }
    }

    /// **A decided argmin's result is the same in every order of its
    /// candidates**: `a` and `b` read in band of each other, and `c` is
    /// decided farther than `a` and nearer than `b`. Every order leaves
    /// `a` alone, which is the minimum. Red where a candidate is compared
    /// only against the ones still in contention, which in the order
    /// `c, a, b` leaves `a` and `b` contested.
    #[test]
    fn a_decided_argmin_is_free_of_its_candidates_order() {
        use super::{FreeOf, argmin_decided};
        let table = [("c", "a", Sign::Positive), ("c", "b", Sign::Negative)];
        for order in [
            ["a", "b", "c"],
            ["a", "c", "b"],
            ["b", "a", "c"],
            ["b", "c", "a"],
            ["c", "a", "b"],
            ["c", "b", "a"],
        ] {
            assert_eq!(
                argmin_decided(order, table_cmp(&table, None), FreeOf::Nothing),
                Ok(Some("a")),
                "{order:?}"
            );
        }
    }

    /// **A decided argmin records an in-band reading live only where it
    /// refuses on it.** Candidates at `0` and `5e-9` read in band of
    /// each other (the band's zero is `1e-9`, its escalation `1e-8`).
    /// Joined by one at `-1`, decided nearer than both, the minimum is
    /// that one, and the in-band reading is superseded: no live
    /// escalation is recorded. Alone, the two are contested and the
    /// reading is live. Red where the in-band run is spliced live
    /// whatever the outcome, which the subdivision driver would read as
    /// the reason a node did not decide.
    #[test]
    fn a_decided_argmin_records_an_in_band_reading_live_only_where_it_refuses() {
        use super::{FreeOf, Undecided, Unpicked, argmin_decided};
        let cmp = |a: &f64, b: &f64| {
            crate::k_stats::decide_reported("argmin_row", Margin::of(*a - *b), band())
                .map(|d| d.sign)
                .map_err(Undecided::InBand)
        };
        let (got, verdicts, escalations) =
            recorded(|| argmin_decided([0.0, 5e-9, -1.0], cmp, FreeOf::Nothing));
        assert_eq!(got, Ok(Some(-1.0)), "the nearer candidate closes the pair");
        assert!(
            escalations.is_empty(),
            "no live escalation: {escalations:?}"
        );
        assert_eq!(
            verdicts.len(),
            2,
            "the two decided readings are live: {verdicts:?}"
        );
        let (got, _, escalations) = recorded(|| argmin_decided([0.0, 5e-9], cmp, FreeOf::Nothing));
        assert!(
            matches!(got, Err(Unpicked::Contested { ref members, in_band: Some(_) }) if members.len() == 2),
            "the pair alone is contested: {got:?}"
        );
        assert_eq!(
            escalations,
            vec!["argmin_row"],
            "the refusal's reading is live"
        );
    }

    /// **What a decided argmin is free of decides what is left in
    /// contention**: `a` and `b` tied, `c` in band of `a` and decided
    /// farther than `b`. Under `Nothing` the tie is contested with no
    /// in-band reading; under `Ties` it stands, the first in order. Adding
    /// `d`, in band of the tied pair and decided nearer than `c`, leaves a
    /// tie and an in-band pair: `Ties` leaves it contested with the
    /// in-band reading, and `Band` covers it. A poisoned pair stops the
    /// argmin whatever it is free of, and no candidates are no minimum.
    #[test]
    fn a_decided_argmin_ends_by_what_it_is_free_of() {
        use super::{FreeOf, Unpicked, argmin_decided};
        let ties = [("a", "b", Sign::Zero), ("b", "c", Sign::Negative)];
        assert_eq!(
            argmin_decided(["a", "b", "c"], table_cmp(&ties, None), FreeOf::Nothing),
            Err(Unpicked::Contested {
                members: vec!["a", "b"],
                in_band: None,
            }),
            "an exact tie, contested"
        );
        assert_eq!(
            argmin_decided(["a", "b", "c"], table_cmp(&ties, None), FreeOf::Ties),
            Ok(Some("a")),
            "an exact tie, covered"
        );
        let banded = [
            ("a", "b", Sign::Zero),
            ("b", "c", Sign::Negative),
            ("d", "c", Sign::Negative),
        ];
        let four = ["a", "b", "c", "d"];
        assert_eq!(
            argmin_decided(four, table_cmp(&banded, None), FreeOf::Ties),
            Err(Unpicked::Contested {
                members: vec!["a", "b", "d"],
                in_band: Some("ad".to_owned()),
            }),
            "a tie and an in-band pair are past Ties"
        );
        assert_eq!(
            argmin_decided(four, table_cmp(&banded, None), FreeOf::Band),
            Ok(Some("a")),
            "Band covers them"
        );
        assert_eq!(
            argmin_decided(four, table_cmp(&banded, Some(("c", "d"))), FreeOf::Band),
            Err(Unpicked::Aborted("poison cd".to_owned())),
            "a poisoned pair stops the argmin"
        );
        assert_eq!(
            argmin_decided(
                Vec::<&str>::new(),
                table_cmp(&banded, None),
                FreeOf::Nothing
            ),
            Ok(None),
            "no candidate"
        );
        let cycle = [
            ("a", "b", Sign::Negative),
            ("b", "c", Sign::Negative),
            ("c", "a", Sign::Negative),
        ];
        assert_eq!(
            argmin_decided(["a", "b", "c"], table_cmp(&cycle, None), FreeOf::Band),
            Err(Unpicked::Cyclic),
            "a cyclic comparison aborts, not a minimum of none"
        );
    }

    /// **An in-band reading `free_of` covers is superseded**: two
    /// candidates in band of each other under [`super::FreeOf::Band`] pick
    /// the first, and the reading is recorded superseded, since the
    /// minimum does not rest on it. Red where the coverage branch splices
    /// a live pair's run live whatever `free_of` covers (`&& !covered`
    /// dropped), which the subdivision driver would read as the reason a
    /// node did not decide.
    #[test]
    fn an_in_band_reading_band_covers_is_recorded_superseded() {
        use super::{FreeOf, Undecided, argmin_decided};
        let cmp = |a: &f64, b: &f64| {
            crate::k_stats::decide_reported("argmin_row", Margin::of(*a - *b), band())
                .map(|d| d.sign)
                .map_err(Undecided::InBand)
        };
        let (got, _, escalations) = recorded(|| argmin_decided([0.0, 5e-9], cmp, FreeOf::Band));
        assert_eq!(got, Ok(Some(0.0)), "Band covers the pair: the first stands");
        assert!(
            escalations.is_empty(),
            "the covered reading is superseded: {escalations:?}"
        );
    }
}

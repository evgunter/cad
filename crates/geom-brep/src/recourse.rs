//! **The ending of a sized decision's refusal** (D4 ¶1 (i)/(iv)): the
//! one table a decision on a size the user may intend reads, whichever
//! module owns the decision's closed type.
//!
//! A decision names its geometry lever, the size its margin measures,
//! what it passes on and what its refused side is ([`SizedDecision`]);
//! the refused arm ([`RefusedArm`]) and the door that reads it
//! ([`ReadAt`]) decide the rest. Edge certification
//! (`crate::certify::recourse`) and the offset meters
//! (`crate::offset_meters::Meter`) both end their sized decisions here.
//!
//! A decision with no size the user chose ends here too: in a defect or
//! the last resort ([`Unsized`]), or in its geometry lever alone
//! ([`LeverOnly`]).

use geom_core::{
    Band, Culprit, Decided, FileCoincidence, Indeterminate, KERNEL_LIMIT_RECOURSE, MarginDiag,
    MissReading, MissSource, NOT_YET_ENDING, RefusedReading, Sign, SizedWords, StoredCulprit,
    lever_recourse, noted,
};
pub use geom_core::{
    SizedPass, UNREADABLE_FILE_MARGIN_NOTE, UNREADABLE_MARGIN_NOTE, UNREADABLE_STORED_MARGIN_NOTE,
};

/// Where a refusal is read: the door that reports it, which decides the
/// ending (D4 ¶1 (i)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reading {
    /// At the operation that built the geometry: it came from the
    /// kernel's own construction.
    Build,
    /// Over a body at rest, which a damaged file reaches as surely as a
    /// defective operation does. The STEP import door is its own door
    /// ([`ReadAt::File`]), and an ending a bad file reaches there takes a
    /// [`ReadAt`].
    AtRest,
}

/// **The door a refusal's ending is read at** (D4 ¶1): a [`Reading`] at
/// this run's ε, or the STEP import door, where the file's declared
/// coincidence distance picks the words and, past it, a defect ending
/// names the file ([`ReadAt::culprit`]). Every ending in the refusal
/// stack takes one, so one body serves every door.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadAt {
    /// At this run's ε.
    Run(Reading),
    /// At the import door, with the file's ε_in.
    File(FileCoincidence),
}

impl ReadAt {
    /// **The culprit a defect ending names at this door** (D4 ¶1 (iv)):
    /// the one chooser every defect ending read through this module
    /// takes, sized or not. A run door names its own
    /// ([`Reading::culprit`]); at the import door, past ε_in, the file,
    /// since a bad file reaches every refusal there, whether the reading
    /// compares a value the file stored or one adoption derived from it.
    #[must_use]
    pub const fn culprit(self) -> Culprit {
        match self {
            Self::Run(reading) => reading.culprit(),
            Self::File(_) => Culprit::Stored(StoredCulprit::File),
        }
    }
}

impl From<Reading> for ReadAt {
    fn from(reading: Reading) -> Self {
        Self::Run(reading)
    }
}

impl From<FileCoincidence> for ReadAt {
    fn from(file: FileCoincidence) -> Self {
        Self::File(file)
    }
}

impl Reading {
    /// **The culprit a defect ending names at this door**: a build's is
    /// the kernel alone; over a body at rest a damaged file reaches it as
    /// surely.
    #[must_use]
    pub const fn culprit(self) -> Culprit {
        match self {
            Self::Build => Culprit::Kernel,
            Self::AtRest => Culprit::Stored(StoredCulprit::KernelOrFile),
        }
    }
}

/// The defect ending at `at` ([`ReadAt::culprit`]).
#[must_use]
pub fn defect_ending(at: impl Into<ReadAt>) -> &'static str {
    at.into().culprit().ending()
}

/// The unreadable-margin note at `at`, by its culprit
/// ([`ReadAt::culprit`]): the kernel's alone at a build, the kernel's or
/// the file's at rest, and the file's at the import door. A poisoned
/// margin's ending takes it ([`MarginDiag::unreadable_note`]).
#[must_use]
pub fn unreadable_margin_note(at: impl Into<ReadAt>) -> &'static str {
    match at.into().culprit() {
        Culprit::Kernel => UNREADABLE_MARGIN_NOTE,
        Culprit::Stored(StoredCulprit::KernelOrFile) => UNREADABLE_STORED_MARGIN_NOTE,
        Culprit::Stored(StoredCulprit::File) => UNREADABLE_FILE_MARGIN_NOTE,
    }
}

/// **The ending of a refusal at a shape the kernel has no arm for yet**
/// ([`geom_core::NOT_YET_ENDING`]): nothing the user changes in the
/// model gets through today, and nothing is wrong with what they asked
/// for, so the sentence says so plainly rather than labelling a
/// capability gap `Recourse:`. A margin that could not be read adds its
/// note at `reading` after the ending's joint ([`noted`]), as
/// [`LeverOnly`] and [`SizedDecision`] do.
#[must_use]
pub fn not_yet(arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
    noted(NOT_YET_ENDING, arm.unreadable_note(at))
}

/// A decision with no size the user chose: a residual (it passes only at
/// zero, so a refused margin is a miss) or a form selection (it passes on
/// any definite sign). No smaller tolerance is its recourse (D4 ¶1 (i)).
///
/// Its endings take no unreadable-margin note on a margin that could not
/// be read: each already asks for the report the note would.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unsized {
    /// The kernel built what it claims exactly, so a miss is a defect.
    Defect,
    /// The kernel approximated (a fitted carrier, a settled root, a
    /// spent budget), so a miss may be the approximation's limit (D4 ¶1
    /// (i)'s last resort): at a build on every arm, and at rest on an
    /// undecided one. A definite miss read at rest, or at the import
    /// door past ε_in, is a stored contradiction, which ends in the
    /// defect ending ([`Unsized::recourse`]).
    Fit,
    /// The refused margin is a certified upper bound on the miss, not
    /// the miss: a loose bound is the certificate's own limit, and
    /// contradicts nothing stored.
    Bound,
}

impl Unsized {
    /// The one ending a refusal of this decision carries on `arm`, read
    /// at `at`.
    ///
    /// Every arm of an exact decision ends in the defect ending, naming
    /// the culprit the door gives ([`ReadAt::culprit`]). Where the
    /// kernel approximated, an arm read at a
    /// build, and an undecided arm read anywhere else, end in the last
    /// resort; a definite arm read past a build ends in the defect
    /// ending, since no loosening repairs a stored contradiction. A bound
    /// ends in the last resort on every arm at every door. An arm whose
    /// margin could not be read ends in the defect ending, whatever the
    /// decision: no tolerance makes it readable.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
        let at = at.into();
        let defect = defect_ending(at);
        match self {
            Self::Defect => defect.to_owned(),
            Self::Fit | Self::Bound if arm.unreadable() => defect.to_owned(),
            Self::Bound => KERNEL_LIMIT_RECOURSE.to_owned(),
            Self::Fit => match (at, arm) {
                (ReadAt::Run(Reading::Build), _)
                | (_, RefusedArm::Undecided(_) | RefusedArm::Straddle) => {
                    KERNEL_LIMIT_RECOURSE.to_owned()
                }
                (_, RefusedArm::Zero(_) | RefusedArm::SignCertain(_)) => defect.to_owned(),
            },
        }
    }

    /// The ending a residual's refusal carries on `arm` at the import
    /// door (D4 ¶1): a miss within the file's declared coincidence
    /// distance but beyond ε names setting ε to ε_in as a stopgap, beside
    /// re-exporting the file more precisely
    /// ([`FileCoincidence::miss_recourse_in_file`]); any other reading
    /// ends as the door ends it ([`Unsized::recourse`]). A residual passes
    /// only at zero, so its refused margin is a miss, on the sign-certain
    /// arm too, which every residual refusal carries its reading on; a
    /// bound's is a bound on the miss, on its undecided and sign-certain
    /// arms alike.
    #[must_use]
    pub fn residual_in_file(self, arm: RefusedArm<'_>, file: FileCoincidence) -> String {
        let miss = match (self, arm) {
            // A bound at zero refuses nothing a residual would.
            (Self::Bound, RefusedArm::Zero(_)) => return self.recourse(arm, file),
            (Self::Bound, RefusedArm::Undecided(cause)) => MissReading::Bound(cause.margin),
            (Self::Bound, RefusedArm::SignCertain(Some(margin))) => MissReading::Bound(margin),
            (_, RefusedArm::Undecided(cause)) => MissReading::Banded(cause.margin, cause.band),
            (_, RefusedArm::Zero(Classified { margin, band })) => MissReading::Banded(margin, band),
            (_, RefusedArm::SignCertain(Some(margin))) => MissReading::Definite(margin),
            // A straddle, or a sign-certain arm without its reading,
            // carries no single reading of the miss to compare with the
            // file's coincidence distance: it ends as the door ends it.
            (_, RefusedArm::SignCertain(None) | RefusedArm::Straddle) => {
                return self.recourse(arm, file);
            }
        };
        let source = match self {
            Self::Defect => MissSource::File,
            Self::Fit | Self::Bound => MissSource::Fit,
        };
        file.miss_recourse_in_file(miss, source, &self.recourse(arm, file))
    }
}

/// The reporting margin a band classified, where the variant reporting
/// the verdict keeps it: for its words only ([`MarginDiag`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Classified {
    /// What the classifier saw.
    pub margin: MarginDiag,
    /// The band that classified it.
    pub band: Band,
}

/// A decision's decided refusal, carrying the reporting margin it
/// classified: the verdict of a decision that passes on a positive sign.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Refused {
    /// Within the zero band — band-decided: a smaller tolerance may
    /// decide a positive margin passing.
    Zero(Classified),
    /// Definitely negative — sign-certain.
    Negative {
        /// What the classifier saw.
        margin: MarginDiag,
    },
}

impl Refused {
    /// The verdict `decided` gives at `band`, or `None` for the positive
    /// sign that passes.
    #[must_use]
    pub fn of(decided: Decided, band: Band) -> Option<Self> {
        let Decided { sign, margin } = decided;
        match sign {
            Sign::Positive => None,
            Sign::Zero => Some(Self::Zero(Classified { margin, band })),
            Sign::Negative => Some(Self::Negative { margin }),
        }
    }

    /// The verdict a gate that passes on a positive sign carries in its
    /// rejection ([`MarginDiag::rejected_sign`]): the quantity it meters
    /// is not there, decided zero or negative. `None` where the gate
    /// could not decide (in band, straddling, poisoned).
    #[must_use]
    pub fn rejected(cause: &Indeterminate) -> Option<Self> {
        let sign = cause.margin.rejected_sign()?;
        Self::of(
            Decided {
                sign,
                margin: cause.margin,
            },
            cause.band,
        )
    }

    /// The reporting margin the verdict was classified on.
    #[must_use]
    pub fn margin(self) -> MarginDiag {
        match self {
            Self::Zero(Classified { margin, .. }) | Self::Negative { margin } => margin,
        }
    }

    /// `zero` on the zero arm, whose reading did not show the quantity
    /// positive, and `negative` on the definite arm, which decided it
    /// below zero: a payload hedges only what its verdict left open.
    #[must_use]
    pub(crate) fn worded(self, zero: &'static str, negative: &'static str) -> &'static str {
        match self {
            Self::Zero(_) => zero,
            Self::Negative { .. } => negative,
        }
    }

    /// The refused arm this verdict is.
    #[must_use]
    pub fn arm(self) -> RefusedArm<'static> {
        match self {
            Self::Zero(classified) => RefusedArm::Zero(classified),
            // A sized verdict's margin is a signed size, not a residual miss.
            Self::Negative { .. } => RefusedArm::SignCertain(None),
        }
    }
}

/// The refused arm of a decision a refusal reports.
#[derive(Clone, Copy, Debug)]
pub enum RefusedArm<'a> {
    /// The margin landed in the band, or was poisoned; the classifier's
    /// diagnostic carries it.
    Undecided(&'a Indeterminate),
    /// The margin classified as zero where zero does not pass, with the
    /// reporting margin and its band.
    Zero(Classified),
    /// The margin classified with a definite sign that refuses, with the
    /// reporting margin where the verdict kept one.
    SignCertain(Option<MarginDiag>),
    /// Two sound bounds on the one margin straddle the band — the lower
    /// within the zero band, the upper beyond the escalation band — so
    /// no single margin states the reading and none is carried to size a
    /// tolerance by. Not a poisoned margin: neither bound is unreadable.
    Straddle,
}

impl RefusedArm<'_> {
    /// The margin a band-decided arm carries: in band, or at zero. A
    /// straddle is two readable bounds, and a sign-certain arm was read.
    fn banded(self) -> Option<MarginDiag> {
        match self {
            Self::Undecided(cause) => Some(cause.margin),
            Self::Zero(Classified { margin, .. }) => Some(margin),
            Self::Straddle | Self::SignCertain(_) => None,
        }
    }

    /// Whether the arm's margin could not be read: the test
    /// [`Unsized::Fit`] and [`Unsized::Bound`] end in the defect ending on.
    fn unreadable(self) -> bool {
        self.banded().is_some_and(MarginDiag::is_invalid)
    }

    /// The note the arm's ending takes at `reading`
    /// ([`MarginDiag::unreadable_note`]), for [`not_yet`] and
    /// [`LeverOnly`].
    fn unreadable_note(self, at: impl Into<ReadAt>) -> Option<&'static str> {
        self.banded()?.unreadable_note(unreadable_margin_note(at))
    }
}

/// A decision on no size the user chose whose refusal a geometry lever
/// reaches: every arm ends in the lever alone, since no smaller tolerance
/// is its recourse (D4 ¶1 (i)), and a margin that could not be read adds
/// its note at the reading, as a sized decision's does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeverOnly {
    /// The lever, after "Recourse: ".
    pub lever: &'static str,
}

/// **An edge's curve read, within the band, at a pose where its face
/// holds no curve of its kind** ([`crate::OffMargin::InBand`]): a reading
/// of the geometry's pose, not shown off the face, so its lever moves the
/// pose at every door ([`crate::IncidenceRead::ending`]).
pub const INCIDENCE_AT_POSE: LeverOnly = LeverOnly {
    lever: "move the geometry so the edge sits clear of that pose by more than the tolerance, \
            or so no piece of it sits there",
};

/// **An edge's curve whose incidence on its face is undecided**: not
/// shown on the face and not shown off it, so its lever moves the
/// geometry until the reading decides, at every door.
pub const INCIDENCE_UNSHOWN: LeverOnly = LeverOnly {
    lever: "the edge's curve is not shown on the face it bounds at this tolerance: move the \
            geometry so it lies clearly on that face",
};

impl LeverOnly {
    /// The one ending a refusal of this decision carries on `arm`, read
    /// at `reading`.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
        lever_recourse(self.lever, arm.unreadable_note(at))
    }
}

/// **What a decision's refused side is** (D4 ¶1 (iv)): how a reading
/// certainly on it ends — an exact zero, a zero the symbolic tier
/// proved, a reading in band wholly on the refused side, or one past the
/// band there, whichever arm carries it. A reading that may still show a
/// size (an offer, a floor, an unresolved enclosure, a straddle) ends in
/// the lever whatever the side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusedSide {
    /// The decision's own lever reaches it: the lever and the decision's
    /// note at every door.
    Lever,
    /// A contradiction between what the edge states and the geometry
    /// it is read against: a stored description and its stored geometry,
    /// or, at the import door, what adoption derived from the file (a
    /// chart image, the ladder's candidate description, the parameters
    /// it samples at). At a build it is the user's geometry, which the lever
    /// moves: the lever and the note. Past a build it ends in the defect
    /// ending naming the door's culprit ([`ReadAt::culprit`]): over a
    /// body at rest [`geom_core::KERNEL_OR_FILE_DEFECT_ENDING`], and at
    /// the import door past ε_in [`geom_core::DAMAGED_FILE_ENDING`].
    StoredContradiction,
    /// A side no kernel construction produces. Every public door that
    /// runs the decision reports a reading certainly on this side as its
    /// input's, in a refusal of its own
    /// ([`crate::CertifyError::StatedInterval`]), and where that input is
    /// data the kernel holds, the kernel's call site panics on it. A
    /// refusal built by hand and rendered ends as a stored contradiction
    /// does at every door: at a build in the lever and the note. What
    /// sets it apart from [`Self::StoredContradiction`] is that mapping
    /// at the doors, not its ending.
    Unminted,
}

/// **An edge interval's two span decisions**: whether the interval runs
/// forward over a clear length, and whether a periodic carrier's stays
/// short of one full turn. The one home of each decision's lever, size
/// and pass rule, which every reader of it shares — the edge certifier's
/// checks ([`crate::CertCheck::ParamSpan`],
/// [`crate::CertCheck::ParamWinding`], [`crate::CertCheck::ImageSpan`]),
/// the chart image's that restate them, and a public door's refusal of
/// its input interval ([`crate::CertifyError::StatedInterval`]).
/// Each reader names its own refused side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanDecision {
    /// The interval runs forward over a clear length.
    Forward,
    /// A periodic carrier's interval stays short of one full turn.
    Winding,
}

impl SpanDecision {
    /// Its lever, after "Recourse: ", whatever side a reader refuses it on.
    #[must_use]
    pub const fn lever(self) -> &'static str {
        match self {
            Self::Forward => "move the geometry so this edge runs forward over a clear length",
            Self::Winding => "move the geometry so this arc stays clearly short of a full turn",
        }
    }

    /// The size its ending names ("if this {size} is intended"), which its
    /// lever moves and its check's word names.
    #[must_use]
    pub const fn size(self) -> &'static str {
        match self {
            Self::Forward => crate::certify::length_noun!(),
            Self::Winding => crate::certify::arc_noun!(),
        }
    }

    /// The decision, read with `side` as its refused side.
    pub(crate) const fn on(self, side: RefusedSide) -> SizedDecision {
        let passes = match self {
            Self::Forward => SizedPass::Positive,
            Self::Winding => SizedPass::NonNegative,
        };
        SizedDecision {
            lever: self.lever(),
            size: self.size(),
            passes,
            side,
            no_size: None,
        }
    }
}

/// A decision on a size the user may intend (D4 ¶1 (i)): its own
/// geometry lever, and on a band-decided arm the tolerance that decides
/// it.
#[derive(Clone, Copy, Debug)]
pub struct SizedDecision {
    /// The lever, after "Recourse: ".
    pub lever: &'static str,
    /// The noun the conditional names ("if this {size} is intended").
    pub size: &'static str,
    /// What it passes on.
    pub passes: SizedPass,
    /// What its refused side is.
    pub side: RefusedSide,
    /// What a reading that shows no size may mean beyond the lever, where
    /// the decision's own metering can reach zero on sound geometry:
    /// appended to the lever as "; {note}" on a reading certainly on the
    /// refused side that ends in the lever ([`RefusedSide`]), and on no
    /// other reading.
    pub no_size: Option<&'static str>,
}

impl SizedDecision {
    /// The one ending a refusal of this decision carries on `arm`, read
    /// at `at` (D4 ¶1 (i)/(iv)).
    ///
    /// - Every band-decided arm — in band, zero where zero does not
    ///   pass, or an undecided enclosure straddling zero — ends as its
    ///   reporting margin's class gives ([`MarginDiag::sized_recourse`]),
    ///   read from the number whichever arm its reader routed it on.
    /// - The sign-certain arm ends as a reading past the band wholly on
    ///   the refused side does, on either route
    ///   ([`geom_core::SizedWords::sign_certain_recourse`]).
    /// - A reading certainly on the refused side ends as
    ///   [`SizedDecision::side`] says at `at`.
    /// - At the import door ([`ReadAt::File`]), a band-decided arm whose
    ///   margin's nearer end lies at or below the file's declared
    ///   coincidence distance ends in the door's one sentence for a size
    ///   the file cannot tell from zero
    ///   ([`MarginDiag::sized_recourse_in_file`]), whichever arm the run's
    ///   band placed it on, with no decision's note. Past ε_in it ends as
    ///   at rest, except that a reading certainly on a stored
    ///   contradiction or an unminted side names the file
    ///   ([`ReadAt::culprit`]). A straddle
    ///   carries no single margin to place against ε_in.
    ///
    /// It returns text for every decision, arm and door a caller can
    /// build.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
        let Self { lever, passes, .. } = self;
        let at = at.into();
        let (margin, band) = match arm {
            RefusedArm::Zero(_) if passes.passes_zero() => return lever_recourse(lever, None),
            RefusedArm::Zero(Classified { margin, band }) => (margin, band),
            RefusedArm::Undecided(cause) => (cause.margin, cause.band),
            // No single margin is carried, so none sizes a tolerance.
            RefusedArm::Straddle => return lever_recourse(lever, None),
            RefusedArm::SignCertain(_) => return self.words(at).sign_certain_recourse(),
        };
        let words = self.words(at);
        match at {
            ReadAt::Run(_) => margin.sized_recourse(band, words),
            ReadAt::File(file) => margin.sized_recourse_in_file(band, words, file),
        }
    }

    /// **Whether a refusal on `arm` is certainly on this decision's
    /// refused side** — the readings [`SizedDecision::side`] ends
    /// ([`MarginDiag::shows_no_size`]): every sign-certain refusal, and a
    /// band-decided one whose margin shows no size. A Zero this decision
    /// passes, and a straddle, are not.
    #[must_use]
    pub(crate) fn certainly_refused(self, arm: RefusedArm<'_>) -> bool {
        let (margin, band) = match arm {
            RefusedArm::Zero(_) if self.passes.passes_zero() => return false,
            RefusedArm::Zero(Classified { margin, band }) => (margin, band),
            RefusedArm::Undecided(cause) => (cause.margin, cause.band),
            RefusedArm::Straddle => return false,
            RefusedArm::SignCertain(_) => return true,
        };
        margin.shows_no_size(band, self.passes)
    }

    /// Everything the reporting margin's sentence needs but the number,
    /// at `at`: the one place a reading certainly on the refused side has
    /// its ending chosen, so every arm that carries one ends alike.
    fn words(self, at: ReadAt) -> SizedWords<'static> {
        let refused = match self.side {
            RefusedSide::Lever => RefusedReading::AsShown,
            RefusedSide::StoredContradiction | RefusedSide::Unminted => match at.culprit() {
                // At a build the refused side is the user's geometry,
                // which the lever moves.
                Culprit::Kernel => RefusedReading::AsShown,
                Culprit::Stored(stored) => RefusedReading::Defect(stored),
            },
        };
        SizedWords {
            lever: self.lever,
            size: self.size,
            passes: self.passes,
            no_size: self.no_size,
            unreadable: unreadable_margin_note(at),
            refused,
        }
    }
}

/// [`NARROW_SPAN`] as a literal, for `concat!`.
macro_rules! narrow_span {
    () => {
        "a knot span too narrow, for the size of its parameter values, to be subdivided"
    };
}
pub(crate) use narrow_span;

/// [`NARROW_SPAN_RECOURSE`] as a literal, for `concat!`.
macro_rules! narrow_span_recourse {
    () => {
        "Recourse: widen the face's narrow knot spans, or move its parameter range nearer zero"
    };
}
pub(crate) use narrow_span_recourse;

/// **The narrow-span decision's subject**: a knot span so narrow against
/// the size of its parameter values that `f64` holds only a few values
/// inside it. Whatever cuts it into pieces then rounds its cut points
/// onto one another or onto the span's ends: the patch bound's
/// equal-split refinement inserts one value past the direction's degree
/// ([`crate::patch_bound::PatchBoundError::RefinementFailed`]) or up to
/// it, leaving a refined knot of full multiplicity
/// ([`crate::patch_bound::PatchBoundError::DerivedKnots`]), and the
/// offset fit seeds repeated parameters or too few distinct ones
/// ([`crate::offset_fit::OffsetFitError::Fit`]). A valid face reaches
/// each; `tests/offset_fit.rs` `a_knot_span_a_few_ulps_wide_refuses_on_its_span`
/// drives them through the fit door.
pub const NARROW_SPAN: &str = narrow_span!();

/// **The narrow-span decision's one recourse** (D4 ¶1 (i)): every
/// refusal of [`NARROW_SPAN`] ends in it. The span's width against its
/// parameter values is the lever, so it names both ways to change that
/// ratio. Which span is not carried yet
/// (`encl/narrow-span-refusals-do-not-say-which-span`).
pub const NARROW_SPAN_RECOURSE: &str = narrow_span_recourse!();

/// The content words `text` says more than once, for the payload pins:
/// a payload says each thing once. Possessives fold into their noun
/// (`edge's` is `edge`), and function words and pronouns are not
/// counted.
#[cfg(test)]
pub(crate) fn repeated_words(text: &str) -> Vec<String> {
    const FUNCTION: &[&str] = &[
        "than", "that", "their", "they", "which", "where", "with", "from",
    ];
    let mut seen = std::collections::BTreeSet::new();
    let mut repeated = Vec::new();
    for word in text.split(|c: char| !(c.is_alphabetic() || c == '\'')) {
        let word = word.trim_end_matches("'s").to_lowercase();
        if word.chars().count() < 4 || FUNCTION.contains(&word.as_str()) {
            continue;
        }
        if !seen.insert(word.clone()) && !repeated.contains(&word) {
            repeated.push(word);
        }
    }
    repeated
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use geom_core::KERNEL_OR_FILE_DEFECT_ENDING;

    use super::*;

    /// Every refused side.
    const SIDES: [RefusedSide; 3] = [
        RefusedSide::Lever,
        RefusedSide::StoredContradiction,
        RefusedSide::Unminted,
    ];

    /// `K = 10`: a margin `m` is decided at every tolerance below `|m|/10`.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// The payload pins' repeat check sees a doubled content word
    /// through a possessive, and passes a pronoun said twice.
    #[test]
    fn a_doubled_word_is_found() {
        assert_eq!(
            repeated_words("the edge's faces cross where they say the edge does not cross"),
            ["edge", "cross"]
        );
        assert!(repeated_words("they curve apart, though they do").is_empty());
    }

    /// Every reporting margin a refused arm can carry, both lanes.
    fn margins() -> Vec<MarginDiag> {
        let mut all: Vec<MarginDiag> = [-5e-9, -5e-10, -0.0, 0.0, 5e-10, 1e-9, 5e-9, -1.0]
            .into_iter()
            .map(MarginDiag::value)
            .collect();
        all.extend([
            MarginDiag::enclosure(2e-10, 5e-10),
            MarginDiag::enclosure(-5e-10, -2e-10),
            MarginDiag::enclosure(-2e-10, 3e-10),
            MarginDiag::enclosure(0.0, 3e-10),
            MarginDiag::INVALID,
        ]);
        all
    }

    /// **No offer to tighten is ever unvalued**, over every sized
    /// decision's shape, every reading and every arm with every margin
    /// either lane reports: wherever an ending says "tighten", it names
    /// the tolerance to tighten below, in metres, and ends there; the
    /// margin is on a side the decision passes on (both ends of an
    /// enclosure), so a smaller tolerance does decide it passing; and
    /// the value is its nearer end's `|m|/K`. A Negative verdict never
    /// offers one (no smaller tolerance passes it).
    #[test]
    fn no_tighten_offer_is_unvalued_and_a_negative_verdict_offers_none() {
        let band = band();
        let k = band.escalate() / band.zero();
        // The tolerance a smaller one than which decides `margin`
        // passing, read through the door this test may use.
        let passing_below = |passes: SizedPass, margin: MarginDiag| {
            let on = |v: f64| match passes {
                SizedPass::Positive | SizedPass::NonNegative => v > 0.0,
                SizedPass::NonZero | SizedPass::AnySign => v != 0.0,
                SizedPass::Negative => v < 0.0,
                SizedPass::Zero => false,
            };
            match margin.diagnostic_f64_for_error_text() {
                geom_core::ErrorTextReading::Value(m) => on(m).then(|| m.abs() / k),
                geom_core::ErrorTextReading::Enclosure { lo, hi } => {
                    (on(lo) && on(hi) && (lo > 0.0) == (hi > 0.0))
                        .then(|| lo.abs().min(hi.abs()) / k)
                }
                geom_core::ErrorTextReading::Invalid => None,
            }
        };
        let valued = |got: &str, passes, margin| match got.split_once("tighten the tolerance") {
            None => true,
            Some((_, tail)) => tail
                .strip_prefix(" below ")
                .and_then(|v| v.strip_suffix(" m"))
                .and_then(|v| v.parse::<f64>().ok())
                .is_some_and(|v| {
                    passing_below(passes, margin).is_some_and(|want| v == want && v > 0.0)
                }),
        };
        for passes in [
            SizedPass::Positive,
            SizedPass::NonNegative,
            SizedPass::NonZero,
            SizedPass::Negative,
            SizedPass::AnySign,
            SizedPass::Zero,
        ] {
            for side in SIDES {
                for no_size in [None, Some("a note")] {
                    let decision = SizedDecision {
                        lever: "move it",
                        size: "size",
                        passes,
                        side,
                        no_size,
                    };
                    let file = FileCoincidence::new(2e-9);
                    for reading in [Some(Reading::Build), Some(Reading::AtRest), None] {
                        let end = |arm: RefusedArm<'_>| match reading {
                            Some(reading) => decision.recourse(arm, reading),
                            None => decision.recourse(arm, file),
                        };
                        for margin in margins() {
                            let negative = end(Refused::Negative { margin }.arm());
                            assert!(
                                !negative.contains("tighten"),
                                "{decision:?} {reading:?} {margin}: {negative}"
                            );
                            let zero = end(Refused::Zero(Classified { margin, band }).arm());
                            let cause = Indeterminate {
                                margin,
                                band,
                                predicate: None,
                                terminal_sliver: false,
                            };
                            let undecided = end(RefusedArm::Undecided(&cause));
                            for got in [zero, undecided] {
                                assert!(
                                    valued(&got, passes, margin),
                                    "{decision:?} {reading:?} {margin}: {got}"
                                );
                                // At the import door no offer keeps a
                                // size at or below the file's ε_in by
                                // tightening alone.
                                let within = match margin.diagnostic_f64_for_error_text() {
                                    geom_core::ErrorTextReading::Value(m) => m.abs() <= 2e-9,
                                    geom_core::ErrorTextReading::Enclosure { lo, hi } => {
                                        lo <= 0.0 && hi >= 0.0 || lo.abs().min(hi.abs()) <= 2e-9
                                    }
                                    geom_core::ErrorTextReading::Invalid => true,
                                };
                                assert!(
                                    reading.is_some()
                                        || !within
                                        || !got.contains("tighten")
                                        || got.contains(
                                            "re-export the file with its uncertainty declared \
                                             below"
                                        ) && got.contains(" m and tighten the tolerance below "),
                                    "{decision:?} at the import door {margin}: {got}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// The two-sided decision through the table: a void-side margin is
    /// offered the tolerance at its magnitude, on either band-decided
    /// arm; an enclosure across zero names the lever alone; a zero of no
    /// size names the lever and the decision's note, save at the import
    /// door, where the lever stands alone; an unreadable margin names the
    /// lever and says what it may mean.
    #[test]
    fn a_two_sided_decision_ends_by_its_verdict() {
        let decision = SizedDecision {
            lever: "L",
            size: "thickness",
            passes: SizedPass::NonZero,
            side: RefusedSide::Lever,
            no_size: Some("N"),
        };
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: None,
            terminal_sliver: false,
        };
        let ends =
            |margin| decision.recourse(RefusedArm::Undecided(&cause(margin)), Reading::Build);
        let zero = |margin, reading| {
            decision.recourse(
                RefusedArm::Zero(Classified {
                    margin,
                    band: band(),
                }),
                reading,
            )
        };
        let offer = "Recourse: L, or, if this thickness is intended, tighten the tolerance below \
                     5e-10 m";
        assert_eq!(ends(MarginDiag::value(-5e-9)), offer);
        assert_eq!(zero(MarginDiag::value(-5e-9), Reading::AtRest), offer);
        assert_eq!(ends(MarginDiag::enclosure(-2e-9, 3e-9)), "Recourse: L");
        assert_eq!(
            ends(MarginDiag::INVALID),
            format!("Recourse: L; {UNREADABLE_MARGIN_NOTE}")
        );
        assert_eq!(
            zero(MarginDiag::value(0.0), Reading::Build),
            "Recourse: L; N"
        );
        assert_eq!(
            zero(MarginDiag::value(-0.0), Reading::AtRest),
            "Recourse: L; N"
        );
        let in_file =
            |arm: RefusedArm<'_>, eps_in| decision.recourse(arm, FileCoincidence::new(eps_in));
        let zero_in_file = |eps_in| {
            in_file(
                RefusedArm::Zero(Classified {
                    margin: MarginDiag::value(-5e-9),
                    band: band(),
                }),
                eps_in,
            )
        };
        let untold = "This thickness is within the file's declared coincidence distance \
                        ε_in = 1e-8 m, so the file cannot tell it from zero. Recourse: L, or, if \
                        this thickness is intended, re-export the file with its uncertainty \
                        declared below 5e-9 m and tighten the tolerance below 5e-10 m";
        assert_eq!(
            zero_in_file(1e-8),
            untold,
            "a zero within ε_in reads as the in-band arm does at the import door"
        );
        assert_eq!(
            in_file(
                RefusedArm::Undecided(&cause(MarginDiag::value(-5e-9))),
                1e-8
            ),
            untold,
            "an undecided size within ε_in names both steps that keep it"
        );
        assert_eq!(
            in_file(
                RefusedArm::Zero(Classified {
                    margin: MarginDiag::value(0.0),
                    band: band(),
                }),
                1e-8
            ),
            "This thickness is within the file's declared coincidence distance ε_in = 1e-8 m, so \
             the file cannot tell it from zero. Recourse: L",
            "a zero of no size at the import door names the lever and no decision's note"
        );
        assert_eq!(
            zero_in_file(1e-9),
            offer,
            "a size past ε_in reads as at rest"
        );
    }

    /// **An approximation's gate rejection ends in the last resort at
    /// every door** (`Unsized::Fit`): a fit's meter that a gate rejects,
    /// zero or past the band, is the approximation's limit, not a stored
    /// contradiction, so carried as `RefusedArm::Undecided` it ends as the
    /// undecided arm does, at a build, at rest and at the import door.
    /// Red if the table converts a gate's tagged reading into its
    /// definite arm.
    #[test]
    fn a_fits_gate_rejection_ends_in_the_last_resort_at_every_door() {
        use geom_core::{Margin, k_stats::decide_positive};
        let rejected = |m: f64| decide_positive("p", Margin::of(m), band()).unwrap_err();
        for cause in [rejected(5e-10), rejected(-1.0)] {
            let arm = RefusedArm::Undecided(&cause);
            let label = cause.margin;
            assert!(cause.margin.rejected_sign().is_some(), "tagged: {label}");
            for reading in [Reading::Build, Reading::AtRest] {
                assert_eq!(
                    Unsized::Fit.recourse(arm, reading),
                    KERNEL_LIMIT_RECOURSE,
                    "{label}, {reading:?}"
                );
            }
            assert_eq!(
                Unsized::Fit.residual_in_file(arm, FileCoincidence::new(1e-12)),
                KERNEL_LIMIT_RECOURSE,
                "{label}, at the import door"
            );
        }
    }

    /// **Either route ends a reading alike** (`SizedDecision::recourse`):
    /// what the ending adds is read from the number, so a gate's rejected
    /// Zero routed as `RefusedArm::Undecided`, the same reading untagged
    /// on that route, and the Zero arm end identically at a build, at
    /// rest and at the import door, on both lanes — the decision's note on
    /// a reading that shows no size, the floor's sentence on a sub-floor
    /// size, the lever alone on an enclosure across zero, the offer on a
    /// size — and the import door adds no decision's note to any of these
    /// readings, all within ε_in, but keeps the floor's sentence. Red if a
    /// route keys the note on the tag rather than the reading.
    #[test]
    fn either_route_ends_a_reading_alike() {
        use geom_core::{Interval, Margin, floor_note, k_stats::decide_positive};

        let band = band();
        let decision = SizedDecision {
            lever: "L",
            size: "length",
            passes: SizedPass::Positive,
            side: RefusedSide::StoredContradiction,
            no_size: Some("N"),
        };
        let noted = "Recourse: L; N".to_owned();
        let point = |m: f64| {
            (
                decide_positive("p", Margin::of(m), band).unwrap_err(),
                MarginDiag::value(m),
            )
        };
        let enclosure = |lo: f64, hi: f64| {
            (
                decide_positive("p", Margin::of(Interval::from_bounds(lo, hi)), band).unwrap_err(),
                MarginDiag::enclosure(lo, hi),
            )
        };
        let rows = [
            (point(0.0), noted.clone()),
            (point(-5e-10), noted.clone()),
            (
                point(2e-12),
                format!("Recourse: L; {}", floor_note("length")),
            ),
            (
                point(5e-10),
                "Recourse: L, or, if this length is intended, tighten the tolerance below 5e-11 m"
                    .to_owned(),
            ),
            (enclosure(-2e-12, 6e-12), "Recourse: L".to_owned()),
            (enclosure(-5e-10, 0.0), noted.clone()),
            (enclosure(0.0, 0.0), noted),
        ];
        let file = FileCoincidence::new(1e-8);
        for ((tagged, margin), want) in rows {
            assert_eq!(tagged.margin.rejected_sign(), Some(Sign::Zero), "{margin}");
            let untagged = Indeterminate {
                margin,
                band,
                predicate: None,
                terminal_sliver: false,
            };
            let zero = Refused::rejected(&tagged).unwrap().arm();
            for at in [
                ReadAt::Run(Reading::Build),
                ReadAt::Run(Reading::AtRest),
                ReadAt::File(file),
            ] {
                let got = decision.recourse(RefusedArm::Undecided(&tagged), at);
                assert_eq!(
                    got,
                    decision.recourse(zero, at),
                    "{margin} at {at:?}: Zero arm"
                );
                assert_eq!(
                    got,
                    decision.recourse(RefusedArm::Undecided(&untagged), at),
                    "{margin} at {at:?}: untagged"
                );
                match at {
                    ReadAt::Run(Reading::Build) => assert_eq!(got, want, "{margin}"),
                    ReadAt::File(_) => {
                        // No decision's note within ε_in; the floor's
                        // sentence, true of the number, rides as at a build.
                        assert!(!got.ends_with("; N"), "{margin} at the import door: {got}");
                        assert_eq!(
                            got.ends_with(&floor_note("length")),
                            want.ends_with(&floor_note("length")),
                            "{margin} at the import door: {got}"
                        );
                    }
                    ReadAt::Run(Reading::AtRest) => {}
                }
            }
        }
    }

    /// **A residual's sign-certain arm is read by the margin it carries**
    /// at the import door: a definite miss within ε_in says it lies
    /// within, and an arm with no reading compares nothing with ε_in and
    /// ends in the damaged file's ending, as a definite miss past ε_in
    /// does.
    #[test]
    fn a_sign_certain_residual_reads_its_carried_margin_at_the_import_door() {
        let tol = geom_core::Tol::witness();
        let file = FileCoincidence::new(1e3 * tol.eps());
        let miss = MarginDiag::value(1e2 * tol.eps());
        for residual in [Unsized::Defect, Unsized::Fit] {
            let valued = residual.residual_in_file(RefusedArm::SignCertain(Some(miss)), file);
            let unvalued = residual.residual_in_file(RefusedArm::SignCertain(None), file);
            assert!(
                valued.starts_with("This miss lies beyond the tolerance and within the file's"),
                "{residual:?}: {valued}"
            );
            assert_eq!(unvalued, geom_core::DAMAGED_FILE_ENDING, "{residual:?}");
        }
    }

    /// **One definite reading ends one way at a door, whichever arm
    /// routes it, and by its decision's refused side** (D4 ¶1 (iv)): over
    /// a decision of each [`RefusedSide`], a refused reading past the
    /// band routed as undecided (tagged by its gate, or not), the same
    /// verdict on the sign-certain arm (with its reading or without), and
    /// a reading in band wholly on the refused side all end alike. At a
    /// build each ends in the lever and the note, on every side; at rest
    /// and at the import door past ε_in a decision the lever reaches
    /// keeps them, while a stored contradiction and an unminted side end
    /// in the kernel-or-file defect ending at rest and in the damaged
    /// file's at the door. Red if a route keys its ending on the arm, or
    /// the in-band reading on its distance from zero, rather than the
    /// reading's class and the decision, or if the door names the kernel.
    #[test]
    fn one_definite_reading_ends_one_way_whichever_arm_routes_it() {
        use geom_core::{DAMAGED_FILE_ENDING, Margin, k_stats::decide_positive};

        use crate::CertCheck;
        use crate::offset_meters::Meter;

        let band = band();
        let gate = decide_positive("p", Margin::of(-1e-3), band).unwrap_err();
        assert_eq!(gate.margin.rejected_sign(), Some(Sign::Negative));
        let cause = |m: f64| Indeterminate {
            margin: MarginDiag::value(m),
            band,
            predicate: None,
            terminal_sliver: false,
        };
        let (untagged, in_band) = (cause(-1e-3), cause(-5e-9));
        let verdict = Refused::rejected(&gate).unwrap().arm();
        let routes = [
            ("tagged past", RefusedArm::Undecided(&gate)),
            ("untagged past", RefusedArm::Undecided(&untagged)),
            ("the gate's verdict", verdict),
            (
                "sign-certain, read",
                RefusedArm::SignCertain(Some(gate.margin)),
            ),
            ("in band", RefusedArm::Undecided(&in_band)),
        ];
        let file = ReadAt::File(FileCoincidence::new(1e-9));
        let meter = "Recourse: move the geometry so this spline edge runs steadily forward, never \
                     stalling or turning back; if it does neither, the speed floor has reached \
                     its limit, which is worth reporting";
        let floor = "Recourse: split the face clear of any pole, cusp or pinch; if it has none, \
                     this may indicate a kernel bug worth reporting";
        let arm = "Recourse: move the geometry so that edge's faces clearly run farther and none \
                   curves tightly there; faces of no extent or a cone's point leave no angle to \
                   measure";
        let angle = "Recourse: move the geometry so the surfaces cross at a clearer angle";
        let span = "Recourse: move the geometry so this edge runs forward over a clear length";
        let (defect, damaged) = (
            Some(KERNEL_OR_FILE_DEFECT_ENDING.to_owned()),
            Some(DAMAGED_FILE_ENDING.to_owned()),
        );
        // `None` where the decision has no import door: the offset
        // meters are asked at a build or at rest only (`Meter::recourse`).
        type End = Box<dyn Fn(RefusedArm<'_>, ReadAt) -> Option<String>>;
        let certify =
            |check| -> End { Box::new(move |a, at| Some(crate::certify::recourse(check, a, at))) };
        let lever = |ending: &str| Some(ending.to_owned());
        // (decision, its ending, at a build, at rest, at the door past
        // ε_in)
        type Row = (
            &'static str,
            End,
            Option<String>,
            Option<String>,
            Option<String>,
        );
        let stored = SizedDecision {
            lever: "L",
            size: "length",
            passes: SizedPass::Positive,
            side: RefusedSide::StoredContradiction,
            no_size: Some("N"),
        };
        let rows: [Row; 6] = [
            (
                "ParamSpan (unminted)",
                certify(CertCheck::ParamSpan),
                lever(span),
                defect.clone(),
                damaged.clone(),
            ),
            (
                "ParamSpanMeter (lever)",
                certify(CertCheck::ParamSpanMeter),
                lever(meter),
                lever(meter),
                lever(meter),
            ),
            (
                "NormalFloor (lever, no door)",
                Box::new(|a, at| match at {
                    ReadAt::Run(reading) => Some(Meter::NormalFloor.recourse(a, reading)),
                    ReadAt::File(_) => None,
                }),
                lever(floor),
                lever(floor),
                None,
            ),
            (
                "DIHEDRAL_ARM (lever)",
                Box::new(|a, at| Some(crate::DIHEDRAL_ARM.recourse(a, at))),
                lever(arm),
                lever(arm),
                lever(arm),
            ),
            (
                "Transversality (stored contradiction)",
                certify(CertCheck::Transversality),
                lever(angle),
                defect.clone(),
                damaged.clone(),
            ),
            (
                "a hand-built stored contradiction",
                Box::new(move |a, at| Some(stored.recourse(a, at))),
                lever("Recourse: L; N"),
                defect,
                damaged,
            ),
        ];
        for (decision, end, build, rest, door) in rows {
            for (route, refused) in routes {
                let at = |at: ReadAt| end(refused, at);
                assert_eq!(
                    at(Reading::Build.into()),
                    build,
                    "{decision}, {route}, at a build"
                );
                assert_eq!(
                    at(Reading::AtRest.into()),
                    rest,
                    "{decision}, {route}, at rest"
                );
                assert_eq!(at(file), door, "{decision}, {route}, at the door past ε_in");
            }
        }
    }

    /// **A hand-built unminted side renders, and ends as a stored
    /// contradiction does** (`[failure]`): a public render of a value a
    /// caller built is input, so it never panics. On every arm and at
    /// every door the two sides end alike, and a reading certainly on the
    /// refused side ends at a build in the lever, as a door's
    /// `StatedInterval` does; a reading that still shows a size keeps its
    /// offer.
    #[test]
    fn a_hand_built_unminted_side_ends_as_a_stored_contradiction_does() {
        let on = |side| SizedDecision {
            lever: "L",
            size: "length",
            passes: SizedPass::Positive,
            side,
            no_size: Some("N"),
        };
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: None,
            terminal_sliver: false,
        };
        let (offer, in_band, poisoned) = (
            cause(MarginDiag::value(5e-9)),
            cause(MarginDiag::value(-5e-9)),
            cause(MarginDiag::INVALID),
        );
        let arms = [
            RefusedArm::SignCertain(None),
            RefusedArm::SignCertain(Some(MarginDiag::value(-1e-3))),
            RefusedArm::Zero(Classified {
                margin: MarginDiag::value(0.0),
                band: band(),
            }),
            RefusedArm::Undecided(&offer),
            RefusedArm::Undecided(&in_band),
            RefusedArm::Undecided(&poisoned),
            RefusedArm::Straddle,
        ];
        let doors = [
            ReadAt::Run(Reading::Build),
            ReadAt::Run(Reading::AtRest),
            ReadAt::File(FileCoincidence::new(1e-12)),
        ];
        let (unminted, stored) = (
            on(RefusedSide::Unminted),
            on(RefusedSide::StoredContradiction),
        );
        for arm in arms {
            for at in doors {
                assert_eq!(
                    unminted.recourse(arm, at),
                    stored.recourse(arm, at),
                    "{arm:?} at {at:?}"
                );
            }
        }
        for arm in [
            RefusedArm::SignCertain(None),
            RefusedArm::Undecided(&in_band),
        ] {
            assert_eq!(
                unminted.recourse(arm, Reading::Build),
                "Recourse: L; N",
                "{arm:?} at a build"
            );
        }
        assert!(
            unminted
                .recourse(RefusedArm::Undecided(&offer), Reading::Build)
                .contains("tighten the tolerance below"),
            "a size ends in its offer"
        );
    }

    /// A straddle of two bounds names the lever alone on a sized decision
    /// at every reading, and the last resort at rest.
    #[test]
    fn a_straddle_names_the_lever_alone() {
        let decision = SizedDecision {
            lever: "L",
            size: "distance",
            passes: SizedPass::AnySign,
            side: RefusedSide::Lever,
            no_size: None,
        };
        for reading in [Reading::Build, Reading::AtRest] {
            assert_eq!(
                decision.recourse(RefusedArm::Straddle, reading),
                "Recourse: L"
            );
        }
        assert_eq!(
            Unsized::Fit.recourse(RefusedArm::Straddle, Reading::AtRest),
            KERNEL_LIMIT_RECOURSE
        );
    }

    /// **A lever-only decision ends in its lever on every arm**: plain
    /// on every arm whose margin was read — in band, at zero, sign-certain
    /// and straddling — and with the unreadable-margin note its reading
    /// gives after "; " on every arm whose margin was poisoned, in band or
    /// at zero.
    #[test]
    fn a_lever_only_decision_ends_in_its_lever_and_notes_a_poisoned_margin() {
        let lever = LeverOnly { lever: "L" };
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: None,
            terminal_sliver: false,
        };
        let zero = |margin| {
            RefusedArm::Zero(Classified {
                margin,
                band: band(),
            })
        };
        let plain = "Recourse: L".to_owned();
        for reading in [Reading::Build, Reading::AtRest] {
            let note = match reading {
                Reading::Build => UNREADABLE_MARGIN_NOTE,
                Reading::AtRest => UNREADABLE_STORED_MARGIN_NOTE,
            };
            let noted = format!("Recourse: L; {note}");
            for margin in margins() {
                let want = if margin.is_invalid() { &noted } else { &plain };
                let undecided = cause(margin);
                // geom-core's build-reading spelling of the undecided arm
                // says the same bytes.
                if reading == Reading::Build {
                    assert_eq!(&undecided.lever_ending("L"), want, "{margin:?}");
                }
                for arm in [RefusedArm::Undecided(&undecided), zero(margin)] {
                    assert_eq!(
                        &lever.recourse(arm, reading),
                        want,
                        "{arm:?} at {reading:?}"
                    );
                }
            }
            for arm in [
                RefusedArm::SignCertain(None),
                RefusedArm::SignCertain(Some(MarginDiag::value(-5e-9))),
                RefusedArm::Straddle,
            ] {
                assert_eq!(
                    lever.recourse(arm, reading),
                    plain,
                    "{arm:?} at {reading:?}"
                );
            }
        }
    }

    /// **A poisoned sized arm keeps its lever, and its note names the
    /// door's culprit** (D4 ¶1 (i), D7): at a build the kernel made the
    /// geometry, so the note names the kernel; at rest the kernel or the
    /// file; at the import door the file, as the defect ending does
    /// there. Red if the import door reads the note at rest.
    #[test]
    fn a_poisoned_sized_arm_keeps_its_lever_and_notes_its_doors_culprit() {
        let decision = SizedDecision {
            lever: "L",
            size: "thickness",
            passes: SizedPass::Positive,
            side: RefusedSide::Lever,
            no_size: Some("N"),
        };
        let cause = Indeterminate {
            margin: MarginDiag::INVALID,
            band: band(),
            predicate: None,
            terminal_sliver: false,
        };
        let zero = RefusedArm::Zero(Classified {
            margin: MarginDiag::INVALID,
            band: band(),
        });
        for arm in [RefusedArm::Undecided(&cause), zero] {
            for (at, note) in [
                (ReadAt::Run(Reading::Build), UNREADABLE_MARGIN_NOTE),
                (ReadAt::Run(Reading::AtRest), UNREADABLE_STORED_MARGIN_NOTE),
                (
                    ReadAt::File(FileCoincidence::new(1e-6)),
                    UNREADABLE_FILE_MARGIN_NOTE,
                ),
            ] {
                assert_eq!(
                    decision.recourse(arm, at),
                    format!("Recourse: L; {note}"),
                    "at {at:?} on {arm:?}"
                );
            }
        }
    }

    /// **No tolerance answers a poisoned margin** (D4 ¶1 (i)): on an arm
    /// whose margin could not be read, every table's ending at every door
    /// names no tolerance to loosen or tighten, and a decision with no
    /// size the user chose ends in the defect ending its reading names,
    /// the last resort's too. An arm that was read keeps the last resort
    /// where the kernel approximated.
    #[test]
    fn no_ending_offers_a_tolerance_on_a_poisoned_margin() {
        let cause = Indeterminate {
            margin: MarginDiag::INVALID,
            band: band(),
            predicate: None,
            terminal_sliver: false,
        };
        let zero = RefusedArm::Zero(Classified {
            margin: MarginDiag::INVALID,
            band: band(),
        });
        let poisoned = [RefusedArm::Undecided(&cause), zero];
        let file = FileCoincidence::new(1e-6);
        let sized: Vec<SizedDecision> = [
            SizedPass::Positive,
            SizedPass::NonNegative,
            SizedPass::NonZero,
            SizedPass::AnySign,
            SizedPass::Negative,
        ]
        .into_iter()
        .flat_map(|passes| {
            SIDES.map(|side| SizedDecision {
                lever: "L",
                size: "distance",
                passes,
                side,
                no_size: Some("N"),
            })
        })
        .collect();
        for arm in poisoned {
            let mut endings = Vec::new();
            for reading in [Reading::Build, Reading::AtRest] {
                endings.push(not_yet(arm, reading));
                endings.push(LeverOnly { lever: "L" }.recourse(arm, reading));
            }
            for residual in [Unsized::Defect, Unsized::Fit, Unsized::Bound] {
                for reading in [Reading::Build, Reading::AtRest] {
                    assert_eq!(
                        residual.recourse(arm, reading),
                        defect_ending(reading),
                        "{residual:?} at {reading:?} on {arm:?}"
                    );
                }
                assert_eq!(
                    residual.residual_in_file(arm, file),
                    geom_core::DAMAGED_FILE_ENDING,
                    "{residual:?} at the import door on {arm:?}"
                );
                assert_eq!(
                    residual.recourse(arm, file),
                    geom_core::DAMAGED_FILE_ENDING,
                    "{residual:?} unsized at the import door on {arm:?}"
                );
            }
            for decision in &sized {
                endings.push(decision.recourse(arm, Reading::Build));
                endings.push(decision.recourse(arm, Reading::AtRest));
                endings.push(decision.recourse(arm, file));
            }
            for ending in endings {
                assert!(!ending.contains("tolerance"), "{arm:?}: {ending}");
            }
        }
        let read = Indeterminate {
            margin: MarginDiag::value(5e-9),
            ..cause
        };
        for reading in [Reading::Build, Reading::AtRest] {
            assert_eq!(
                Unsized::Fit.recourse(RefusedArm::Undecided(&read), reading),
                KERNEL_LIMIT_RECOURSE,
                "a read margin at {reading:?}"
            );
        }
    }
}

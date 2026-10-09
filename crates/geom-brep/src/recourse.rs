//! **The ending of a sized decision's refusal** (D4 ¶1 (i)/(iv)): the
//! one table a decision on a size the user may intend reads, whichever
//! module owns the decision's closed type.
//!
//! A decision names its geometry lever, the size its margin measures,
//! what it passes on and how a definite refusal read over stored
//! geometry ends ([`SizedDecision`]); the refused arm ([`RefusedArm`]) and the
//! door that reads it ([`Reading`]) decide the rest. Edge certification
//! (`crate::certify::recourse`) and the offset meters
//! (`crate::offset_meters::Meter`) both end their sized decisions here.
//!
//! A decision with no size the user chose ends here too: in a defect or
//! the last resort ([`Unsized`]), or in its geometry lever alone
//! ([`LeverOnly`]).

use geom_core::{
    Band, Decided, FileCoincidence, Indeterminate, KERNEL_DEFECT_ENDING, KERNEL_LIMIT_RECOURSE,
    KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag, MissReading, MissSource, NOT_YET_ENDING, Sign,
    SizedWords, lever_recourse, noted,
};
pub use geom_core::{SizedPass, UNREADABLE_MARGIN_NOTE};

/// Where a refusal is read: the door that reports it, which decides the
/// ending (D4 ¶1 (i)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reading {
    /// At the operation that built the geometry: it came from the
    /// kernel's own construction.
    Build,
    /// Over a body at rest, which a damaged file reaches as surely as a
    /// defective operation does. Certification at STEP adoption reads
    /// here too (D4 ¶1); the import door's own ε_in words are
    /// [`SizedDecision::recourse_in_file`]'s and
    /// [`Unsized::residual_in_file`]'s.
    AtRest,
}

/// The defect ending at `reading`: a build's is the kernel's alone; over
/// stored geometry a damaged file reaches it as surely.
#[must_use]
pub fn defect_ending(reading: Reading) -> &'static str {
    match reading {
        Reading::Build => KERNEL_DEFECT_ENDING,
        Reading::AtRest => KERNEL_OR_FILE_DEFECT_ENDING,
    }
}

/// **The ending of a refusal at a shape the kernel has no arm for yet**
/// ([`geom_core::NOT_YET_ENDING`]): nothing the user changes in the
/// model gets through today, and nothing is wrong with what they asked
/// for, so the sentence says so plainly rather than labelling a
/// capability gap `Recourse:`. A margin that could not be read adds
/// [`UNREADABLE_MARGIN_NOTE`] after the ending's joint ([`noted`]), as
/// [`LeverOnly`] and [`SizedDecision`] do.
#[must_use]
pub fn not_yet(arm: RefusedArm<'_>) -> String {
    noted(
        NOT_YET_ENDING,
        arm.unreadable().then_some(UNREADABLE_MARGIN_NOTE),
    )
}

/// A decision with no size the user chose: a residual (it passes only at
/// zero, so a refused margin is a miss) or a form selection (it passes on
/// any definite sign). No smaller tolerance is its recourse (D4 ¶1 (i)).
///
/// Its endings take no [`UNREADABLE_MARGIN_NOTE`] on a margin that could
/// not be read: each already asks for the report the note would.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unsized {
    /// The kernel built what it claims exactly, so a miss is a defect.
    Defect,
    /// The kernel approximated (a fitted carrier, a certified bound), so
    /// a miss may be the approximation's limit (D4 ¶1 (i)'s last resort).
    LastResort,
}

impl Unsized {
    /// The one ending a refusal of this decision carries on `arm`, read
    /// at `reading`.
    ///
    /// Every arm of an exact decision ends in the defect ending. Where
    /// the kernel approximated, an arm read at a build, and an undecided
    /// arm read at rest, end in the last resort; a definite arm at rest
    /// ends in the file's defect ending, since no loosening repairs a
    /// stored contradiction. An arm whose margin could not be read ends
    /// in the defect ending too: no tolerance makes it readable.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        let defect = defect_ending(reading);
        match self {
            Self::Defect => defect.to_owned(),
            Self::LastResort if arm.unreadable() => defect.to_owned(),
            Self::LastResort => match (reading, arm) {
                (Reading::Build, _)
                | (Reading::AtRest, RefusedArm::Undecided(_) | RefusedArm::Straddle) => {
                    KERNEL_LIMIT_RECOURSE.to_owned()
                }
                (Reading::AtRest, RefusedArm::Zero(_) | RefusedArm::SignCertain(_)) => {
                    defect.to_owned()
                }
            },
        }
    }

    /// The ending a residual's refusal carries on `arm` at the import
    /// door (D4 ¶1): read at rest, except that a miss within the file's
    /// declared coincidence distance but beyond ε names setting ε to ε_in
    /// as a stopgap, beside re-exporting the file more precisely
    /// ([`FileCoincidence::miss_recourse_in_file`]). A residual passes
    /// only at zero, so its refused margin is a miss, on the sign-certain
    /// arm too, which every residual refusal carries its reading on.
    #[must_use]
    pub fn residual_in_file(self, arm: RefusedArm<'_>, file: FileCoincidence) -> String {
        let miss = match arm {
            RefusedArm::Undecided(cause) => MissReading::Banded(cause.margin, cause.band),
            RefusedArm::Zero(Classified { margin, band }) => MissReading::Banded(margin, band),
            RefusedArm::SignCertain(Some(margin)) => MissReading::Definite(margin),
            // A straddle, or a sign-certain arm without its reading,
            // carries no single reading of the miss to compare with the
            // file's coincidence distance: it ends at rest.
            RefusedArm::SignCertain(None) | RefusedArm::Straddle => {
                return self.recourse(arm, Reading::AtRest);
            }
        };
        let source = match self {
            Self::Defect => MissSource::File,
            Self::LastResort => MissSource::Fit,
        };
        file.miss_recourse_in_file(miss, source, &self.recourse(arm, Reading::AtRest))
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
    /// Whether the arm's margin could not be read: the test [`not_yet`]
    /// and [`LeverOnly`] add the unreadable-margin note on, and
    /// [`Unsized::LastResort`] ends in the defect ending on. A sized
    /// decision does not use it: [`MarginDiag::sized_recourse`] tests
    /// the margin itself. A straddle is two readable bounds, and a
    /// sign-certain arm was read.
    fn unreadable(self) -> bool {
        match self {
            Self::Undecided(cause) => cause.margin.is_invalid(),
            Self::Zero(Classified { margin, .. }) => margin.is_invalid(),
            Self::Straddle | Self::SignCertain(_) => false,
        }
    }
}

/// A decision on no size the user chose whose refusal a geometry lever
/// reaches: every arm ends in the lever alone, since no smaller tolerance
/// is its recourse (D4 ¶1 (i)), and a margin that could not be read adds
/// [`UNREADABLE_MARGIN_NOTE`], as a sized decision's does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LeverOnly {
    /// The lever, after "Recourse: ".
    pub lever: &'static str,
}

impl LeverOnly {
    /// The one ending a refusal of this decision carries on `arm`, at
    /// every reading.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>) -> String {
        lever_recourse(
            self.lever,
            arm.unreadable().then_some(UNREADABLE_MARGIN_NOTE),
        )
    }
}

/// What a zero verdict that leaves no size to tighten below may mean
/// beyond the lever, by where it is read: appended to the lever as
/// "; {note}".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtZero {
    /// The note at a build, where the kernel made the geometry.
    pub build: &'static str,
    /// The note over stored geometry, which a damaged file reaches as
    /// surely.
    pub stored: &'static str,
}

impl AtZero {
    /// The same note wherever the refusal is read.
    #[must_use]
    pub const fn same(note: &'static str) -> Self {
        Self {
            build: note,
            stored: note,
        }
    }

    /// The note at `reading`.
    #[must_use]
    pub fn at(self, reading: Reading) -> &'static str {
        match reading {
            Reading::Build => self.build,
            Reading::AtRest => self.stored,
        }
    }
}

/// How a sized decision's sign-certain refusal ends when it is read over
/// stored geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoredDefinite {
    /// A contradiction between a stored description and its stored
    /// geometry, which no move of the geometry reaches: it ends as a
    /// defect of the kernel or the file (D4 ¶1 (iv)).
    Contradiction,
    /// The decision's own lever reaches it (the stored description is
    /// what the lever edits), so it ends in the lever alone, as at a
    /// build.
    Lever,
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
    /// How its sign-certain arm ends over stored geometry.
    pub stored: StoredDefinite,
    /// What a zero verdict leaving no size to tighten below may mean
    /// beyond the lever, where the decision's own metering can reach
    /// zero on sound geometry or no construction mints it.
    pub at_zero: Option<AtZero>,
}

impl SizedDecision {
    /// The one ending a refusal of this decision carries on `arm`, read
    /// at `reading` (D4 ¶1 (i)/(iv)).
    ///
    /// - Every band-decided arm — in band, zero where zero does not
    ///   pass, or an enclosure straddling zero — ends in the lever at
    ///   every reading, with the words the arm's reporting margin gives
    ///   ([`MarginDiag::sized_recourse`]): at a build or at rest, the
    ///   tolerance below which a smaller one decides the margin passing,
    ///   where one does. A zero verdict that leaves no size to tighten
    ///   below adds [`SizedDecision::at_zero`] where the decision has
    ///   one.
    /// - The sign-certain arm names the lever alone at a build. Read
    ///   over stored geometry it ends as [`SizedDecision::stored`] says.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        let Self {
            lever,
            passes,
            stored,
            at_zero,
            ..
        } = self;
        match arm {
            RefusedArm::Zero(_) if passes.passes_zero() => lever_recourse(lever, None),
            RefusedArm::Zero(Classified { margin, band }) => {
                margin.sized_recourse(band, self.words(at_zero.map(|note| note.at(reading))))
            }
            // No single margin is carried, so none sizes a tolerance.
            RefusedArm::Straddle => lever_recourse(lever, None),
            RefusedArm::SignCertain(_) => match (reading, stored) {
                (Reading::Build, _) | (Reading::AtRest, StoredDefinite::Lever) => {
                    lever_recourse(lever, None)
                }
                (Reading::AtRest, StoredDefinite::Contradiction) => {
                    KERNEL_OR_FILE_DEFECT_ENDING.to_owned()
                }
            },
            RefusedArm::Undecided(cause) => {
                cause.margin.sized_recourse(cause.band, self.words(None))
            }
        }
    }

    /// The ending a refusal of this decision carries on `arm` at the
    /// import door (D4 ¶1): read at rest, except that a band-decided arm
    /// whose margin's nearer end lies at or below the file's declared
    /// coincidence distance ends in the door's one sentence for a size the
    /// file does not state ([`MarginDiag::sized_recourse_in_file`]),
    /// whichever arm the run's band placed it on.
    #[must_use]
    pub fn recourse_in_file(self, arm: RefusedArm<'_>, file: FileCoincidence) -> String {
        match arm {
            RefusedArm::Zero(_) if self.passes.passes_zero() => self.recourse(arm, Reading::AtRest),
            RefusedArm::Zero(Classified { margin, band }) => margin.sized_recourse_in_file(
                band,
                self.words(self.at_zero.map(|note| note.stored)),
                file,
            ),
            RefusedArm::Undecided(cause) => {
                cause
                    .margin
                    .sized_recourse_in_file(cause.band, self.words(None), file)
            }
            // A straddle carries no single margin to place against the
            // file's coincidence distance.
            RefusedArm::SignCertain(_) | RefusedArm::Straddle => {
                self.recourse(arm, Reading::AtRest)
            }
        }
    }

    /// Everything the reporting margin's sentence needs but the number.
    fn words(self, otherwise: Option<&'static str>) -> SizedWords<'static> {
        SizedWords {
            lever: self.lever,
            size: self.size,
            passes: self.passes,
            otherwise,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// `K = 10`: a margin `m` is decided at every tolerance below `|m|/10`.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
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
        ] {
            for stored in [StoredDefinite::Contradiction, StoredDefinite::Lever] {
                for at_zero in [None, Some(AtZero::same("a note"))] {
                    let decision = SizedDecision {
                        lever: "move it",
                        size: "size",
                        passes,
                        stored,
                        at_zero,
                    };
                    let file = FileCoincidence::new(2e-9);
                    for reading in [Some(Reading::Build), Some(Reading::AtRest), None] {
                        let end = |arm: RefusedArm<'_>| match reading {
                            Some(reading) => decision.recourse(arm, reading),
                            None => decision.recourse_in_file(arm, file),
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
    /// size names the lever and the decision's note; an unreadable
    /// margin names the lever and says what it may mean.
    #[test]
    fn a_two_sided_decision_ends_by_its_verdict() {
        let decision = SizedDecision {
            lever: "L",
            size: "thickness",
            passes: SizedPass::NonZero,
            stored: StoredDefinite::Lever,
            at_zero: Some(AtZero {
                build: "at a build",
                stored: "at rest",
            }),
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
            "Recourse: L; at a build"
        );
        assert_eq!(
            zero(MarginDiag::value(-0.0), Reading::AtRest),
            "Recourse: L; at rest"
        );
        let in_file = |arm: RefusedArm<'_>, eps_in| {
            decision.recourse_in_file(arm, FileCoincidence::new(eps_in))
        };
        let zero_in_file = |eps_in| {
            in_file(
                RefusedArm::Zero(Classified {
                    margin: MarginDiag::value(-5e-9),
                    band: band(),
                }),
                eps_in,
            )
        };
        let unstated = "This thickness is below the file's declared coincidence distance \
                        ε_in = 1e-8 m, so the file does not state it. Recourse: L, or, if this \
                        thickness is intended, re-export the file with its uncertainty declared \
                        below 5e-9 m and tighten the tolerance below 5e-10 m";
        assert_eq!(
            zero_in_file(1e-8),
            unstated,
            "a zero within ε_in reads as the in-band arm does at the import door"
        );
        assert_eq!(
            in_file(
                RefusedArm::Undecided(&cause(MarginDiag::value(-5e-9))),
                1e-8
            ),
            unstated,
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
            "This thickness is below the file's declared coincidence distance ε_in = 1e-8 m, so \
             the file does not state it. Recourse: L; at rest",
            "a zero of no size names the lever and the decision's note"
        );
        assert_eq!(
            zero_in_file(1e-9),
            offer,
            "a size past ε_in reads as at rest"
        );
    }

    /// **A residual's sign-certain arm is read by the margin it carries**
    /// at the import door: a definite miss within ε_in says it lies
    /// within, and an arm with no reading compares nothing with ε_in and
    /// ends at rest.
    #[test]
    fn a_sign_certain_residual_reads_its_carried_margin_at_the_import_door() {
        let tol = geom_core::Tol::witness();
        let file = FileCoincidence::new(1e3 * tol.eps());
        let miss = MarginDiag::value(1e2 * tol.eps());
        for residual in [Unsized::Defect, Unsized::LastResort] {
            let valued = residual.residual_in_file(RefusedArm::SignCertain(Some(miss)), file);
            let unvalued = residual.residual_in_file(RefusedArm::SignCertain(None), file);
            assert!(
                valued.starts_with("This miss lies beyond the tolerance and within the file's"),
                "{residual:?}: {valued}"
            );
            assert_eq!(
                unvalued,
                residual.recourse(RefusedArm::SignCertain(None), Reading::AtRest),
                "{residual:?}"
            );
        }
    }

    /// A straddle of two bounds names the lever alone on a sized decision
    /// at every reading, and the last resort at rest.
    #[test]
    fn a_straddle_names_the_lever_alone() {
        let decision = SizedDecision {
            lever: "L",
            size: "distance",
            passes: SizedPass::AnySign,
            stored: StoredDefinite::Lever,
            at_zero: None,
        };
        for reading in [Reading::Build, Reading::AtRest] {
            assert_eq!(
                decision.recourse(RefusedArm::Straddle, reading),
                "Recourse: L"
            );
        }
        assert_eq!(
            Unsized::LastResort.recourse(RefusedArm::Straddle, Reading::AtRest),
            KERNEL_LIMIT_RECOURSE
        );
    }

    /// **A lever-only decision ends in its lever on every arm**: plain
    /// on every arm whose margin was read — in band, at zero, sign-certain
    /// and straddling — and with the unreadable-margin note after "; "
    /// on every arm whose margin was poisoned, in band or at zero.
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
        let noted = format!("Recourse: L; {UNREADABLE_MARGIN_NOTE}");
        for margin in margins() {
            let want = if margin.is_invalid() { &noted } else { &plain };
            let undecided = cause(margin);
            for arm in [RefusedArm::Undecided(&undecided), zero(margin)] {
                assert_eq!(&lever.recourse(arm), want, "{arm:?}");
            }
        }
        for arm in [
            RefusedArm::SignCertain(None),
            RefusedArm::SignCertain(Some(MarginDiag::value(-5e-9))),
            RefusedArm::Straddle,
        ] {
            assert_eq!(lever.recourse(arm), plain, "{arm:?}");
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
            [StoredDefinite::Lever, StoredDefinite::Contradiction].map(|stored| SizedDecision {
                lever: "L",
                size: "distance",
                passes,
                stored,
                at_zero: Some(AtZero::same("N")),
            })
        })
        .collect();
        for arm in poisoned {
            let mut endings = vec![not_yet(arm), LeverOnly { lever: "L" }.recourse(arm)];
            for residual in [Unsized::Defect, Unsized::LastResort] {
                for reading in [Reading::Build, Reading::AtRest] {
                    assert_eq!(
                        residual.recourse(arm, reading),
                        defect_ending(reading),
                        "{residual:?} at {reading:?} on {arm:?}"
                    );
                }
                assert_eq!(
                    residual.residual_in_file(arm, file),
                    defect_ending(Reading::AtRest),
                    "{residual:?} at the import door on {arm:?}"
                );
            }
            for decision in &sized {
                endings.push(decision.recourse(arm, Reading::Build));
                endings.push(decision.recourse(arm, Reading::AtRest));
                endings.push(decision.recourse_in_file(arm, file));
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
                Unsized::LastResort.recourse(RefusedArm::Undecided(&read), reading),
                KERNEL_LIMIT_RECOURSE,
                "a read margin at {reading:?}"
            );
        }
    }
}

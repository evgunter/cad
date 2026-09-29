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
//! A decision with no size the user chose ([`Unsized`]) ends here too.

use geom_core::k_stats::NonPositiveSign;
use geom_core::{
    Band, Decided, Indeterminate, KERNEL_DEFECT_ENDING, KERNEL_LIMIT_RECOURSE,
    KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag, Sign, SizedWords,
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
    /// defective operation does.
    AtRest,
    /// At STEP adoption: the geometry is the file's, certified at the
    /// kernel's tolerance. D4 ¶1: "Import does not motivate loosening ε
    /// (D7)", and "at adoption the lever is the import's own ε_in (D7),
    /// phrased by the import door". No such lever exists yet — the
    /// adoption ladder certifies at the kernel's ε, which no ε_in
    /// reaches — so no arm read here names a tolerance: the missing ε_in
    /// lever is why, not a D4 prohibition on tightening. A band-decided
    /// arm names its decision's geometry lever alone.
    Adopt,
}

/// The defect ending at `reading`: a build's is the kernel's alone; over
/// stored geometry a damaged file reaches it as surely.
#[must_use]
pub fn defect_ending(reading: Reading) -> &'static str {
    match reading {
        Reading::Build => KERNEL_DEFECT_ENDING,
        Reading::AtRest | Reading::Adopt => KERNEL_OR_FILE_DEFECT_ENDING,
    }
}

/// A decision with no size the user chose: a residual (it passes only at
/// zero, so a refused margin is a miss) or a form selection (it passes on
/// any definite sign). No smaller tolerance is its recourse (D4 ¶1 (i)).
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
    /// arm read at rest, end in the last resort; a definite arm at rest,
    /// and every arm at adoption, end in the file's defect ending, since
    /// no loosening repairs a stored contradiction.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        let defect = defect_ending(reading);
        match self {
            Self::Defect => defect.to_owned(),
            Self::LastResort => match (reading, arm) {
                (Reading::Build, _) | (Reading::AtRest, RefusedArm::Undecided(_)) => {
                    KERNEL_LIMIT_RECOURSE.to_owned()
                }
                (Reading::AtRest, RefusedArm::Zero(_) | RefusedArm::SignCertain)
                | (Reading::Adopt, _) => defect.to_owned(),
            },
        }
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

    /// The verdict a collapsed-arm gate refused
    /// ([`geom_core::k_stats::GateRefusal::Collapsed`]), classified at
    /// `band`.
    #[must_use]
    pub fn collapsed(sign: NonPositiveSign, margin: MarginDiag, band: Band) -> Self {
        match sign {
            NonPositiveSign::Zero => Self::Zero(Classified { margin, band }),
            NonPositiveSign::Negative => Self::Negative { margin },
        }
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
            Self::Negative { .. } => RefusedArm::SignCertain,
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
    /// The margin classified with a definite sign that refuses.
    SignCertain,
}

/// What a zero verdict that leaves no size to tighten below may mean
/// beyond the lever, by where it is read: appended to the lever as
/// "; {note}".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtZero {
    /// The note at a build, where the kernel made the geometry.
    pub build: &'static str,
    /// The note over stored geometry (at rest, or at adoption), which a
    /// damaged file reaches as surely.
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
            Reading::AtRest | Reading::Adopt => self.stored,
        }
    }
}

/// How a sized decision's sign-certain refusal ends when it is read over
/// stored geometry (at rest, or at adoption).
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
    ///   one. At adoption no arm names a tolerance ([`Reading::Adopt`]).
    /// - The sign-certain arm names the lever alone at a build. Read
    ///   over stored geometry it ends as [`SizedDecision::stored`] says.
    #[must_use]
    pub fn recourse(self, arm: RefusedArm<'_>, reading: Reading) -> String {
        let Self {
            lever,
            size,
            passes,
            stored,
            at_zero,
        } = self;
        let words = |otherwise| SizedWords {
            lever,
            size,
            passes,
            may_tighten: reading != Reading::Adopt,
            otherwise,
        };
        match arm {
            RefusedArm::Zero(_) if passes.passes_zero() => format!("Recourse: {lever}"),
            RefusedArm::Zero(Classified { margin, band }) => {
                margin.sized_recourse(band, words(at_zero.map(|note| note.at(reading))))
            }
            RefusedArm::SignCertain => match (reading, stored) {
                (Reading::Build, _) | (Reading::AtRest | Reading::Adopt, StoredDefinite::Lever) => {
                    format!("Recourse: {lever}")
                }
                (Reading::AtRest | Reading::Adopt, StoredDefinite::Contradiction) => {
                    KERNEL_OR_FILE_DEFECT_ENDING.to_owned()
                }
            },
            RefusedArm::Undecided(cause) => cause.margin.sized_recourse(cause.band, words(None)),
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
                SizedPass::NonZero => v != 0.0,
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
                    for reading in [Reading::Build, Reading::AtRest, Reading::Adopt] {
                        let end = |arm: RefusedArm<'_>| decision.recourse(arm, reading);
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
        assert_eq!(
            zero(MarginDiag::value(-5e-9), Reading::Adopt),
            "Recourse: L",
            "adoption names no tolerance"
        );
    }
}

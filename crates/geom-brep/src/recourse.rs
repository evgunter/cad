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
    Band, Indeterminate, KERNEL_DEFECT_ENDING, KERNEL_LIMIT_RECOURSE, KERNEL_OR_FILE_DEFECT_ENDING,
    MarginDiag, Sign,
};

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
                (Reading::Build, _)
                | (Reading::AtRest, RefusedArm::Undecided(_) | RefusedArm::Straddle) => {
                    KERNEL_LIMIT_RECOURSE.to_owned()
                }
                (Reading::AtRest, RefusedArm::Zero(_) | RefusedArm::SignCertain)
                | (Reading::Adopt, _) => defect.to_owned(),
            },
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
        let Self { lever } = self;
        match arm {
            RefusedArm::Undecided(Indeterminate {
                margin: MarginDiag::Invalid,
                ..
            }) => format!("Recourse: {lever}; {UNREADABLE_MARGIN_NOTE}"),
            RefusedArm::Undecided(_)
            | RefusedArm::Straddle
            | RefusedArm::Zero(_)
            | RefusedArm::SignCertain => format!("Recourse: {lever}"),
        }
    }
}

/// The margin a band classified, where the variant reporting the
/// verdict keeps it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Classified {
    /// The classified margin, in metres.
    pub margin: f64,
    /// The band that classified it.
    pub band: Band,
}

/// A decision's decided refusal, carrying the margin it classified: the
/// verdict of a decision that passes on a positive sign, reported where
/// the margin is an `f64` the reporting site may echo.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Refused {
    /// Within the zero band — band-decided: a smaller tolerance may
    /// decide a positive margin passing.
    Zero(Classified),
    /// Definitely negative — sign-certain.
    Negative {
        /// The classified margin, in metres.
        margin: f64,
    },
}

impl Refused {
    /// The verdict `sign` gives `margin` at `band`, or `None` for the
    /// positive sign that passes.
    pub(crate) fn of(sign: Sign, margin: f64, band: Band) -> Option<Self> {
        match sign {
            Sign::Positive => None,
            Sign::Zero => Some(Self::Zero(Classified { margin, band })),
            Sign::Negative => Some(Self::Negative { margin }),
        }
    }

    /// The classified margin, in metres.
    #[must_use]
    pub fn margin(self) -> f64 {
        match self {
            Self::Zero(Classified { margin, .. }) | Self::Negative { margin } => margin,
        }
    }

    /// The refused arm this verdict is.
    #[must_use]
    pub fn arm(self) -> RefusedArm<'static> {
        match self {
            Self::Zero(classified) => RefusedArm::Zero(Some(classified)),
            Self::Negative { .. } => RefusedArm::SignCertain,
        }
    }
}

/// [`Refused`] without its margin, for a decision taken at a generic
/// `T: Decide` scalar: a margin derived there is no refusal payload
/// outside a ratified seam (`geom_core::real`'s `Bounds` scope rule), so
/// the variant that reports the verdict keeps the verdict alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Definite {
    /// Within the zero band — band-decided.
    Zero,
    /// Definitely negative — sign-certain.
    Negative,
}

impl Definite {
    /// The verdict `sign` gives, or `None` for the positive sign that
    /// passes.
    pub(crate) fn of(sign: Sign) -> Option<Self> {
        match sign {
            Sign::Positive => None,
            Sign::Zero => Some(Self::Zero),
            Sign::Negative => Some(Self::Negative),
        }
    }

    /// The refused arm this verdict is.
    #[must_use]
    pub fn arm(self) -> RefusedArm<'static> {
        match self {
            Self::Zero => RefusedArm::Zero(None),
            Self::Negative => RefusedArm::SignCertain,
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
    /// margin and its band where the reporting variant may carry them
    /// ([`Definite`] says where it may not).
    Zero(Option<Classified>),
    /// The margin classified with a definite sign that refuses.
    SignCertain,
    /// Two sound bounds on the one margin straddle the band — the lower
    /// within the zero band, the upper beyond the escalation band — so
    /// no single margin states the reading and none is carried. Not a
    /// poisoned margin: neither bound is unreadable.
    Straddle,
}

/// The pass set of a decision on a size the user may intend: one that
/// passes on a nonzero sign, the only kind a smaller tolerance can
/// decide passing (D4 ¶1 (i)).
///
/// A decision whose every definite reading answers ([`Self::Definite`])
/// refuses only an undecided margin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizedPass {
    /// A definitely positive margin.
    Positive,
    /// A positive or zero margin.
    NonNegative,
    /// A definitely positive or definitely negative margin: the decision
    /// passes on either side and refuses only at zero.
    NonZero,
    /// Any definite margin — positive, zero or negative: every decided
    /// reading answers (a point on a boundary, or clearly to either side
    /// of it), and only an undecided one refuses.
    Definite,
}

impl SizedPass {
    /// Whether a zero margin passes this decision.
    fn passes_zero(self) -> bool {
        match self {
            Self::Positive | Self::NonZero => false,
            Self::NonNegative | Self::Definite => true,
        }
    }

    /// Whether a tolerance below `v`'s own size decides `v` passing: `v`
    /// is a nonzero margin on a side the decision accepts. A zero margin
    /// (either sign of it) leaves no size to tighten below.
    fn tightens(self, v: f64) -> bool {
        match self {
            Self::Positive | Self::NonNegative => v > 0.0,
            Self::NonZero | Self::Definite => v != 0.0,
        }
    }

    /// The tolerance every margin in `[lo, hi]` is decided passing below,
    /// where both ends tighten on one side.
    fn below(self, lo: f64, hi: f64, k: f64) -> Option<f64> {
        (self.tightens(lo) && self.tightens(hi) && (lo > 0.0) == (hi > 0.0))
            .then(|| lo.abs().min(hi.abs()) / k)
    }
}

/// What an unreadable margin may mean, appended to the decision's lever:
/// the one note an [`RefusedArm::Undecided`] arm whose margin is
/// [`MarginDiag::Invalid`] carries.
pub const UNREADABLE_MARGIN_NOTE: &str =
    "an unreadable or collapsed margin may indicate a kernel bug worth reporting";

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
    /// zero on sound geometry: appended to the lever as "; {at_zero}".
    pub at_zero: Option<&'static str>,
}

/// The ambiguity multiplier `K` of `band`: a margin `m` is decided to
/// its sign at every tolerance below `|m|/K`.
fn k(band: Band) -> f64 {
    band.escalate() / band.zero()
}

impl SizedDecision {
    /// The one ending a refusal of this decision carries on `arm`, read
    /// at `reading` (D4 ¶1 (i)/(iv)).
    ///
    /// - Every band-decided arm — in band, zero where zero does not
    ///   pass, or an enclosure straddling zero — ends in the lever at
    ///   every reading. At a build or at rest it adds the tolerance that
    ///   decides the margin, conditionally, where one does: below
    ///   `|m|/K` for a margin `m` (or the nearer end of a one-sided
    ///   enclosure) that is nonzero and on a side the decision accepts,
    ///   and with no value on a zero arm whose variant carries none. A
    ///   margin on the refused side, at zero, straddling zero, or read
    ///   only as two bounds straddling the band ([`RefusedArm::Straddle`])
    ///   is passed by no smaller tolerance and names the lever alone (and,
    ///   on a zero verdict, [`SizedDecision::at_zero`] where the
    ///   decision has one); a margin that could not be read keeps the
    ///   lever and adds [`UNREADABLE_MARGIN_NOTE`]. At adoption no arm
    ///   names a tolerance ([`Reading::Adopt`]).
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
        let alone = || format!("Recourse: {lever}");
        let tighten = |below: Option<f64>| match (reading, below) {
            (Reading::Adopt, _) => alone(),
            (Reading::Build | Reading::AtRest, Some(v)) => format!(
                "Recourse: {lever}, or, if this {size} is intended, tighten the tolerance below \
                 {v:e} m"
            ),
            (Reading::Build | Reading::AtRest, None) => {
                format!("Recourse: {lever}, or, if this {size} is intended, tighten the tolerance")
            }
        };
        match arm {
            RefusedArm::Zero(_) if passes.passes_zero() => alone(),
            RefusedArm::Zero(None) => tighten(None),
            RefusedArm::Zero(Some(Classified { margin, band })) if passes.tightens(margin) => {
                tighten(Some(margin.abs() / k(band)))
            }
            RefusedArm::Zero(Some(_)) => match at_zero {
                Some(note) => format!("Recourse: {lever}; {note}"),
                None => alone(),
            },
            RefusedArm::Straddle => alone(),
            RefusedArm::SignCertain => match (reading, stored) {
                (Reading::Build, _) | (Reading::AtRest | Reading::Adopt, StoredDefinite::Lever) => {
                    alone()
                }
                (Reading::AtRest | Reading::Adopt, StoredDefinite::Contradiction) => {
                    KERNEL_OR_FILE_DEFECT_ENDING.to_owned()
                }
            },
            RefusedArm::Undecided(cause) => {
                let k = k(cause.band);
                match cause.margin {
                    MarginDiag::Value(m) if passes.tightens(m) => tighten(Some(m.abs() / k)),
                    MarginDiag::Enclosure { lo, hi } => match passes.below(lo, hi, k) {
                        Some(v) => tighten(Some(v)),
                        None => alone(),
                    },
                    MarginDiag::Value(_) => alone(),
                    MarginDiag::Invalid => format!("Recourse: {lever}; {UNREADABLE_MARGIN_NOTE}"),
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Over every sized decision's shape and every reading: a Negative
    /// verdict, valued or not, offers no tolerance (no smaller one
    /// passes it), and a valued Zero verdict offers one only with the
    /// value its margin gives (D4 ¶1 (i)).
    #[test]
    fn a_negative_verdict_never_tightens_and_a_valued_zero_quotes_its_value() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        for passes in [
            SizedPass::Positive,
            SizedPass::NonNegative,
            SizedPass::NonZero,
            SizedPass::Definite,
        ] {
            for stored in [StoredDefinite::Contradiction, StoredDefinite::Lever] {
                for at_zero in [None, Some("a note")] {
                    let decision = SizedDecision {
                        lever: "move it",
                        size: "size",
                        passes,
                        stored,
                        at_zero,
                    };
                    for reading in [Reading::Build, Reading::AtRest, Reading::Adopt] {
                        let end = |arm| decision.recourse(arm, reading);
                        for margin in [-1.0, -5e-9] {
                            let got = end(Refused::Negative { margin }.arm());
                            assert!(!got.contains("tighten"), "{decision:?} {reading:?}: {got}");
                        }
                        let got = end(Definite::Negative.arm());
                        assert!(!got.contains("tighten"), "{decision:?} {reading:?}: {got}");
                        for margin in [-5e-10, -0.0, 0.0, 5e-10, 1e-9] {
                            let got = end(Refused::Zero(Classified { margin, band }).arm());
                            assert!(
                                !got.contains("tighten") || got.contains(" below "),
                                "{decision:?} {reading:?} {margin:e}: {got}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// `K = 10`: a margin `m` is decided at every tolerance below `|m|/10`.
    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// Which margins a smaller tolerance decides passing, per pass set:
    /// a one-sided set tightens positive margins only, the two-sided set
    /// any nonzero one, and none tightens zero.
    #[test]
    fn each_pass_set_tightens_the_margins_it_accepts() {
        use SizedPass::{Definite, NonNegative, NonZero, Positive};
        let rows = [
            (Positive, 5e-9, true),
            (Positive, -5e-9, false),
            (NonNegative, 5e-9, true),
            (NonNegative, -5e-9, false),
            (NonZero, 5e-9, true),
            (NonZero, -5e-9, true),
            (Positive, 0.0, false),
            (NonNegative, -0.0, false),
            (NonZero, 0.0, false),
            (NonZero, -0.0, false),
            (Definite, 5e-9, true),
            (Definite, -5e-9, true),
            (Definite, 0.0, false),
        ];
        for (pass, v, want) in rows {
            assert_eq!(pass.tightens(v), want, "{pass:?} at {v:e}");
        }
        assert!(!Positive.passes_zero() && !NonZero.passes_zero() && NonNegative.passes_zero());
        assert!(Definite.passes_zero());
    }

    /// An enclosure is decided below its nearer end's `|m|/K` only when
    /// both ends sit on one accepted side; one across zero is decided
    /// passing by no tolerance.
    #[test]
    fn an_enclosure_tightens_only_with_both_ends_on_one_side() {
        use SizedPass::{NonZero, Positive};
        let k = 10.0;
        let rows = [
            (NonZero, 2e-9, 5e-9, Some(2e-10)),
            (NonZero, -5e-9, -2e-9, Some(2e-10)),
            (NonZero, -2e-9, 3e-9, None),
            (NonZero, 0.0, 3e-9, None),
            (Positive, 2e-9, 5e-9, Some(2e-10)),
            (Positive, -5e-9, -2e-9, None),
            (Positive, -2e-9, 3e-9, None),
        ];
        for (pass, lo, hi, want) in rows {
            assert_eq!(
                pass.below(lo, hi, k),
                want,
                "{pass:?} over [{lo:e}, {hi:e}]"
            );
        }
    }

    /// The two-sided decision through the table: a void-side margin is
    /// offered the tolerance at its magnitude, an enclosure across zero
    /// names the lever alone, and a zero with no margin the valueless
    /// offer.
    #[test]
    fn a_two_sided_decision_ends_by_its_verdict() {
        let decision = SizedDecision {
            lever: "L",
            size: "thickness",
            passes: SizedPass::NonZero,
            stored: StoredDefinite::Lever,
            at_zero: None,
        };
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: None,
        };
        let ends =
            |margin| decision.recourse(RefusedArm::Undecided(&cause(margin)), Reading::Build);
        assert_eq!(
            ends(MarginDiag::Value(-5e-9)),
            "Recourse: L, or, if this thickness is intended, tighten the tolerance below 5e-10 m"
        );
        assert_eq!(
            ends(MarginDiag::Enclosure {
                lo: -2e-9,
                hi: 3e-9
            }),
            "Recourse: L"
        );
        assert_eq!(
            ends(MarginDiag::Invalid),
            format!("Recourse: L; {UNREADABLE_MARGIN_NOTE}")
        );
        assert_eq!(
            decision.recourse(RefusedArm::Zero(None), Reading::Build),
            "Recourse: L, or, if this thickness is intended, tighten the tolerance"
        );
    }

    /// A decision whose every definite reading answers tightens an
    /// in-band margin on either side, and a straddle of two bounds names
    /// the lever alone at every reading, as it does for any sized
    /// decision: no single margin is carried to size a tolerance by.
    #[test]
    fn a_definite_decision_tightens_either_side_and_a_straddle_names_the_lever() {
        let decision = SizedDecision {
            lever: "L",
            size: "distance",
            passes: SizedPass::Definite,
            stored: StoredDefinite::Lever,
            at_zero: None,
        };
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: None,
        };
        for m in [5e-9, -5e-9] {
            assert_eq!(
                decision.recourse(
                    RefusedArm::Undecided(&cause(MarginDiag::Value(m))),
                    Reading::AtRest
                ),
                "Recourse: L, or, if this distance is intended, tighten the tolerance below \
                 5e-10 m"
            );
        }
        for reading in [Reading::Build, Reading::AtRest, Reading::Adopt] {
            assert_eq!(
                decision.recourse(RefusedArm::Straddle, reading),
                "Recourse: L"
            );
        }
    }

    /// A lever-only decision ends in its lever on every arm, and adds the
    /// unreadable-margin note on a poisoned margin alone — never on a
    /// straddle, which is two readable bounds.
    #[test]
    fn a_lever_only_decision_ends_in_its_lever() {
        let decision = LeverOnly { lever: "L" };
        let cause = |margin| Indeterminate {
            margin,
            band: band(),
            predicate: None,
        };
        let value = cause(MarginDiag::Value(5e-9));
        let poisoned = cause(MarginDiag::Invalid);
        for arm in [
            RefusedArm::Undecided(&value),
            RefusedArm::Straddle,
            RefusedArm::Zero(None),
            RefusedArm::SignCertain,
        ] {
            assert_eq!(decision.recourse(arm), "Recourse: L", "{arm:?}");
        }
        assert_eq!(
            decision.recourse(RefusedArm::Undecided(&poisoned)),
            format!("Recourse: L; {UNREADABLE_MARGIN_NOTE}")
        );
    }
}

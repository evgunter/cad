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

use geom_core::{Band, Indeterminate, KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag, Sign};

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
}

/// The pass set of a decision on a size the user may intend: one that
/// passes on a nonzero sign, the only kind a smaller tolerance can
/// decide passing (D4 ¶1 (i)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SizedPass {
    /// A definitely positive margin.
    Positive,
    /// A positive or zero margin.
    NonNegative,
}

impl SizedPass {
    /// Whether a zero margin passes this decision.
    fn passes_zero(self) -> bool {
        match self {
            Self::Positive => false,
            Self::NonNegative => true,
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
    /// zero on sound geometry: appended to the lever as "; {at_zero}".
    pub at_zero: Option<&'static str>,
}

/// Whether a tolerance below `v`'s own size decides `v` passing a sized
/// decision: `v` is a nonzero margin on the side the decision accepts.
/// Both sized pass sets accept every positive margin, and a zero one
/// (either sign of it) leaves no size to tighten below.
fn tightens(v: f64) -> bool {
    v > 0.0
}

/// The ambiguity multiplier `K` of `band`: a margin `m` is decided
/// positive at every tolerance below `m/K`.
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
    ///   decides the margin, conditionally, where one does: below `m/K`
    ///   for a margin `m` (or the nearer end of a one-sided enclosure)
    ///   that is nonzero and on the side the decision accepts, and with
    ///   no value on a zero arm whose variant carries none. A margin on
    ///   the refused side, at zero, or straddling zero is passed by no
    ///   smaller tolerance and names the lever alone (and, on a zero
    ///   verdict, [`SizedDecision::at_zero`] where the decision has
    ///   one); a margin that could not be read keeps the lever and says
    ///   what it may mean. At
    ///   adoption no arm names a tolerance ([`Reading::Adopt`]).
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
            RefusedArm::Zero(Some(Classified { margin, band })) if tightens(margin) => {
                tighten(Some(margin / k(band)))
            }
            RefusedArm::Zero(Some(_)) => match at_zero {
                Some(note) => format!("Recourse: {lever}; {note}"),
                None => alone(),
            },
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
                    MarginDiag::Value(m) if tightens(m) => tighten(Some(m / k)),
                    MarginDiag::Enclosure { lo, hi } if tightens(lo) && tightens(hi) => {
                        tighten(Some(lo.min(hi) / k))
                    }
                    MarginDiag::Value(_) | MarginDiag::Enclosure { .. } => alone(),
                    MarginDiag::Invalid => format!(
                        "Recourse: {lever}; an unreadable or collapsed margin may indicate a \
                         kernel bug worth reporting"
                    ),
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
        for passes in [SizedPass::Positive, SizedPass::NonNegative] {
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
}

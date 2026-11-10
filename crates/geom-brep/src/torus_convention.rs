//! **The ring-torus convention `R > r > 0` as a decision** (D3, D4 ¶1
//! (i)/(iv)): its two halves' subjects, levers and decided facts, in
//! the lowest crate that can name a [`SizedDecision`], so every door
//! that refuses a torus outside the convention reads one table:
//! `topo`'s pierce normal and tier 3, and [`crate::intersect`]'s
//! plane×torus section, which carries the half and its verdict as a
//! [`TorusRefusal`]. The margins themselves are decided in `geom`
//! ([`geom::torus_tube`], [`geom::ring_torus`]), beside the type whose
//! convention they state.

use geom_core::{Band, Decided, Indeterminate};

use crate::recourse::{Refused, RefusedArm, RefusedSide, SizedDecision, SizedPass};

/// **A half of the ring-torus convention `R > r > 0`**, as a decision.
/// A torus outside it (no tube, a horn, a spindle) has no
/// representation, so every arm that refuses one refuses the torus's
/// own shape and ends in the half's one lever, whether its margin fell
/// in band or was decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum TorusConvention {
    /// Whether the tube radius is positive at the band
    /// (`torus_tube_positive`).
    ///
    /// Tier 3 reads the same datum under another question: whether it
    /// lies inside `Surface::Torus`'s range at all, an unbanded
    /// representability read ending in "give the tube radius a value
    /// inside its range" (`topo`'s `UnrepresentableSurfaceDatum`). A tube
    /// thinner than the band passes that read and refuses this one, so
    /// the two are different decisions with different levers, not two
    /// stories of one.
    Tube,
    /// Whether the tube radius is smaller than the ring radius
    /// (`ring_torus_convention`), the half tier 3 decides on the same
    /// band.
    Ring,
}

impl TorusConvention {
    /// What the half decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Tube => "whether a torus's tube radius is positive",
            Self::Ring => "whether a torus's tube radius is smaller than its ring radius",
        }
    }

    /// The half's lever and the size its margin measures, at every door
    /// that reads it. The lever edits the stored radii, so a definite
    /// refusal read at rest ends in it too.
    #[must_use]
    pub const fn sized(self) -> SizedDecision {
        let (lever, size) = match self {
            Self::Tube => (
                "reshape the torus so its tube is clearly thicker than the tolerance",
                "tube radius",
            ),
            Self::Ring => (
                "make the tube radius clearly smaller than the ring radius",
                "difference between the radii",
            ),
        };
        SizedDecision {
            lever,
            size,
            passes: SizedPass::Positive,
            side: RefusedSide::Lever,
            no_size: None,
        }
    }

    /// The decided refusal as a clause about `whose` torus, with no
    /// colon or dash of its own.
    #[must_use]
    pub fn refused(self, whose: &str, verdict: Refused) -> String {
        let margin = verdict.margin().readable();
        match (self, verdict) {
            (Self::Tube, Refused::Zero(_)) => {
                format!("{whose} tube radius is not shown above zero (margin {margin} m)")
            }
            (Self::Tube, Refused::Negative { .. }) => {
                format!("{whose} tube radius is below zero (margin {margin} m)")
            }
            (Self::Ring, Refused::Zero(_)) => format!(
                "{whose} tube radius is not shown smaller than its ring's (margin {margin} m)"
            ),
            (Self::Ring, Refused::Negative { .. }) => {
                format!("{whose} tube radius is larger than its ring's (margin {margin} m)")
            }
        }
    }
}

/// **A torus refused at one half of its ring convention**, in band or
/// decided: the half and its verdict, carried from the door that decides
/// it to the one that renders it. Its `Display` is the payload alone, the
/// half's clause or undecided read ([`Self::said`]); read at a door it
/// ends in the half's one lever ([`Self::recourse`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TorusRefusal {
    /// The half that refused.
    pub convention: TorusConvention,
    /// How it refused.
    pub verdict: TorusVerdict,
}

/// How a half of the ring convention refused.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TorusVerdict {
    /// The margin landed in the band, or was poisoned.
    Undecided(Indeterminate),
    /// The margin was decided zero or negative.
    Refused(Refused),
}

/// **A torus refusal's standing, with what each class carries**
/// ([`TorusRefusal::standing`], the one place it is decided).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TorusStanding {
    /// A half the band left undecided, ranked by its margin (row 1(b)):
    /// its escalation.
    Ranked(Indeterminate),
    /// A torus decided outside the convention, a shape the kernel's model
    /// never represents: invalid input (row 1(a)), which aborts, with the
    /// half's decided verdict.
    InvalidInput(Refused),
}

impl TorusRefusal {
    /// **This refusal's standing**, the one home of a torus convention's
    /// class, which every reader of a torus refusal reads, ending it in its
    /// own type: the normal door, the Boolean, the face replacement, the
    /// frame and contact readers, the section's words. No reader matches
    /// [`Self::verdict`] itself (`test-utils`'s `torus_verdict_matches` row).
    #[must_use]
    pub const fn standing(&self) -> TorusStanding {
        match self.verdict {
            TorusVerdict::Undecided(diag) => TorusStanding::Ranked(diag),
            TorusVerdict::Refused(verdict) => TorusStanding::InvalidInput(verdict),
        }
    }

    /// **This refusal's standing on the ladder** ([`crate::ladder::Refusal`]),
    /// read off [`Self::standing`]: an undecided half is ranked by its
    /// margin, and a torus decided outside the convention aborts.
    ///
    /// # Errors
    ///
    /// [`crate::ladder::Aborts`] for a torus decided outside the convention.
    pub const fn refusal(&self) -> Result<crate::ladder::Refusal, crate::ladder::Aborts> {
        match self.standing() {
            TorusStanding::Ranked(diag) => Ok(crate::ladder::Refusal::Margin(diag)),
            TorusStanding::InvalidInput(_) => Err(crate::ladder::Aborts),
        }
    }

    /// The refusal `convention`'s half gives on `decided` at `band`, or
    /// `None` where the half holds.
    #[must_use]
    pub fn of(
        convention: TorusConvention,
        decided: Result<Decided, Indeterminate>,
        band: Band,
    ) -> Option<Self> {
        let verdict = match decided {
            Err(diag) => TorusVerdict::Undecided(diag),
            Ok(decided) => TorusVerdict::Refused(Refused::of(decided, band)?),
        };
        Some(Self {
            convention,
            verdict,
        })
    }

    /// Both halves of the convention on a torus of radii `major` and
    /// `minor`, the tube first ([`geom::ring_torus`] says why): the ring
    /// they read, or the first that refuses.
    ///
    /// # Errors
    ///
    /// The refusing half's [`TorusRefusal`].
    pub fn require<T: geom_core::Decide>(
        major: T,
        minor: T,
        band: Band,
    ) -> Result<geom::RingTorus<T>, Self> {
        Self::require_at(major, minor, geom::RingReader::Convention.sites(), band)
    }

    /// [`Self::require`] under a reader's own funnel rows `sites`
    /// ([`geom::RingTorus::of_at`]): a reader that decides the
    /// convention on its own rows reads its refusal here, the half
    /// undecided kept apart from the half decided outside.
    ///
    /// # Errors
    ///
    /// The refusing half's [`TorusRefusal`].
    pub fn require_at<T: geom_core::Decide>(
        major: T,
        minor: T,
        sites: geom::RingSites,
        band: Band,
    ) -> Result<geom::RingTorus<T>, Self> {
        geom::RingTorus::of_at(major, minor, sites, band).map_err(|not| {
            let (convention, verdict) = match not {
                geom::NotARing::Tube { verdict, .. } => (TorusConvention::Tube, verdict),
                geom::NotARing::Ring { verdict, .. } => (TorusConvention::Ring, verdict),
            };
            Self::of(convention, verdict, band).unwrap_or_else(|| {
                unreachable!("`RingTorus::of` refuses only a half that did not read Positive")
            })
        })
    }
}

impl TorusRefusal {
    /// The refused arm the verdict carries: the undecided margin, or the
    /// decided one's arm.
    #[must_use]
    pub fn arm(&self) -> RefusedArm<'_> {
        match &self.verdict {
            TorusVerdict::Undecided(diag) => RefusedArm::Undecided(diag),
            TorusVerdict::Refused(verdict) => verdict.arm(),
        }
    }

    /// **The refusal's one ending at the door `at` reads it**: its half's
    /// sized decision on its arm ([`TorusConvention::sized`]), in band or
    /// decided alike. A holder that ends a torus refusal in its own words
    /// reads its ending here.
    #[must_use]
    pub fn recourse(&self, at: impl Into<crate::recourse::ReadAt>) -> String {
        self.convention.sized().recourse(self.arm(), at)
    }
}

impl TorusRefusal {
    /// A torus decided outside `convention`'s half on `verdict`.
    #[must_use]
    pub const fn decided(convention: TorusConvention, verdict: Refused) -> Self {
        Self {
            convention,
            verdict: TorusVerdict::Refused(verdict),
        }
    }

    /// **The refusal said about `whose` torus, ended at the door `at`
    /// reads it**: a half decided outside the convention in the half's own
    /// words ([`TorusConvention::refused`]), a half in band as its
    /// undecided read, each ended by [`Self::recourse`]; the clause alone
    /// where `at` is `None`, as a `Display` writes it. Every door that
    /// renders a torus refusal says it so, naming the torus its own way
    /// (`whose`, which an undecided read does not use).
    #[must_use]
    pub fn said(&self, whose: &str, at: Option<crate::recourse::ReadAt>) -> String {
        match &self.verdict {
            TorusVerdict::Undecided(diag) => crate::recourse::undecided_at(
                diag,
                self.convention.subject(),
                at.map(|at| self.recourse(at)),
            )
            .to_string(),
            TorusVerdict::Refused(verdict) => {
                let clause = self.convention.refused(whose, *verdict);
                match at {
                    Some(at) => format!("{clause}. {}", self.recourse(at)),
                    None => clause,
                }
            }
        }
    }

    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<crate::recourse::ReadAt>,
    ) -> core::fmt::Result {
        f.write_str(&self.said("a torus's", at))
    }
}

crate::ended_by_write!(TorusRefusal);

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use geom_core::{Band, MarginDiag};

    use super::TorusConvention;
    use crate::recourse::{Classified, Refused};

    /// **A zero verdict prints its reading and names no shape**: a
    /// positive difference inside the zero band does not make a horn,
    /// nor does a positive tube radius inside it make no tube, so the
    /// clause says only what the band did, and an exact zero reads `0`.
    /// A negative verdict was decided, and says what it decided.
    #[test]
    fn a_zero_verdict_prints_its_reading() {
        let band = Band::new(1e-9, 1e-8).expect("a valid band");
        let zero = |m| {
            Refused::Zero(Classified {
                margin: MarginDiag::value(m),
                band,
            })
        };
        let negative = Refused::Negative {
            margin: MarginDiag::value(-0.5),
        };
        let whose = "a torus face's";
        let rows = [
            (
                TorusConvention::Tube.refused(whose, zero(5e-10)),
                "a torus face's tube radius is not shown above zero (margin 5e-10 m)",
            ),
            (
                TorusConvention::Tube.refused(whose, zero(0.0)),
                "a torus face's tube radius is not shown above zero (margin 0 m)",
            ),
            (
                TorusConvention::Tube.refused(whose, zero(-0.0)),
                "a torus face's tube radius is not shown above zero (margin 0 m)",
            ),
            (
                TorusConvention::Ring.refused(whose, zero(5e-10)),
                "a torus face's tube radius is not shown smaller than its ring's (margin 5e-10 m)",
            ),
            (
                TorusConvention::Tube.refused(whose, negative),
                "a torus face's tube radius is below zero (margin -0.5 m)",
            ),
            (
                TorusConvention::Ring.refused(whose, negative),
                "a torus face's tube radius is larger than its ring's (margin -0.5 m)",
            ),
        ];
        for (got, want) in rows {
            assert_eq!(got, want);
            assert_eq!(
                crate::recourse::repeated_words(&got),
                Vec::<String>::new(),
                "a word said twice: {got}"
            );
        }
    }
}

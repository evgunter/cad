//! **The interpretation layer's readings** (`[import]`, D7): what the
//! file's statements are evidence of, read at the file's band.
//!
//! The file declares a coincidence distance ε_in. A question import asks
//! of the file's evidence (does this point lie on that sphere, are these
//! two planes one locus, is this patch a cylinder) is a distance in
//! metres read through the kernel's one classification funnel
//! (`geom_core::k_stats::decide`) against the file's band
//! ([`file_band`]), with the three arms `[verdicts]` gives every reading:
//! [`Read::Zero`] (one, within ε_in), [`Read::InBand`] (undecided at the
//! file's precision) and [`Read::Apart`] (definitely not). A reading
//! that is a lane choice takes its closed form at Zero and its general
//! lane otherwise; a structural one acts at Zero, keeps apart where
//! apart, and refuses in band ([`Refusal::Ambiguous`]), naming what would
//! decide it. One pass still reads ε_in as a raw distance: the C⁰-knot
//! split (`knot_split`,
//! `work/importgate/the-c0-knot-split-reads-eps-in-raw-not-at-the-file-band.md`).

use geom_core::predicate::Band;
use geom_core::{Margin, Tol, k_stats};

/// **The band the file is read at** (D3: import reads the kernel's
/// readings with the file's own coincidence distance as the band):
/// `(e, K·e)` for `e` the larger of ε_in and the run's ε, the run's K. A
/// file declaring less than the run's tolerance is still read no finer
/// than the run can certify, so a declared ε_in of `0` reads at the
/// run's ε like any other value below it. (A declaration that is
/// negative or not finite never gets here: it refuses where it is read,
/// as an invalid root.) `None` where the pair makes no band (`K·e`
/// overflowing, or rounding back onto `e`).
pub(crate) fn file_band(eps_in: f64, tol: Tol) -> Option<Band> {
    band_at(eps_in, tol.eps(), tol.k())
}

/// [`file_band`] for a run of ε `run_eps` and `k`.
pub(crate) fn band_at(eps_in: f64, run_eps: f64, k: f64) -> Option<Band> {
    let e = eps_in.max(run_eps);
    Band::new(e, k * e).ok()
}

/// A recognized kind's certified residual against the file's statement
/// (a NURBS patch or carrier promoted to an analytic one): Zero promotes.
pub(crate) const STEP_RECOGNIZED_RESIDUAL: &str = "step_recognized_residual";

/// A recognition estimator's conditioning (a span, a sagitta, a radius,
/// a chord): Zero determines no candidate; in band and apart go on to
/// the certificate, which does not rest on the answer.
pub(crate) const STEP_RECOGNIZED_CONDITIONING: &str = "step_recognized_conditioning";

/// A promoted plane's frame: its estimated normal and its roll (the
/// patch's `∂u`), each length read at the least band — the certificate,
/// not this, decides the plane.
pub(crate) const STEP_RECOGNIZED_PLANE_FRAME: &str = "step_recognized_plane_frame";

/// A Nurbs-adjacent line rim's `PlacedSegment` frame: the line's
/// direction and its residual completion, each length read at the least
/// band.
pub(crate) const STEP_RIM_LINE_FRAME: &str = "step_rim_line_frame";

/// Whether a carrier closes on itself (its end on its start): Zero is the
/// closed class.
pub(crate) const STEP_RECOGNIZED_CLOSURE: &str = "step_recognized_closure";

/// Whether one face bound lies clear of another's chart polygon, in
/// metres: only apart decides which bound contains which.
pub(crate) const STEP_RING_CONTAINMENT: &str = "step_ring_containment";

/// Whether a `VERTEX_LOOP`'s point lies on its face's sphere: the
/// point's distance from the sphere must read Zero.
pub(crate) const STEP_VERTEX_LOOP_ON_SPHERE: &str = "step_vertex_loop_on_sphere";

/// Whether a seamless band's rim circle is coaxial with its surface (its
/// tilt at its radius, and its centre's distance off the axis): Zero
/// takes the band re-mint.
pub(crate) const STEP_BAND_RIM_COAXIAL: &str = "step_band_rim_coaxial";

/// Whether a rim vertex already sits at a band's seam azimuth: Zero uses
/// it, apart splits the rim there.
pub(crate) const STEP_BAND_SEAM_VERTEX: &str = "step_band_seam_vertex";

/// A length a construction takes only positive: it reads as a length only
/// apart from zero.
pub(crate) const STEP_POSITIVE_LENGTH: &str = "step_positive_length";

/// The file's ε_in and the band it is read at. The band is always a
/// band: a file whose ε_in makes none refuses where it is read
/// ([`FileBand::of_import`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct FileBand {
    eps_in: f64,
    /// The run's own ε, which floors the band.
    run_eps: f64,
    band: Band,
}

/// A reading of one distance at the file's band (`[verdicts]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Read {
    /// Within the band's zero: one, as far as the file can say.
    Zero,
    /// Between the band's zero and its escalation, or unreadable: the
    /// file's precision does not decide it.
    InBand,
    /// Past the band: definitely apart.
    Apart,
}

impl FileBand {
    /// [`Self::of_import`] for a declared `eps_in`: a unit test's band
    /// as production reads it.
    ///
    /// # Errors
    ///
    /// [`Self::of_import`]'s.
    #[cfg(test)]
    pub(crate) fn new(eps_in: f64, tol: Tol) -> Result<Self, crate::StepImportError> {
        Self::of_import(eps_in, false, tol)
    }

    /// The band for `eps_in` under the run's `tol` ([`file_band`]), where
    /// `overridden` says whether `eps_in` is the caller's override or
    /// the file's declaration, as a refusal names it.
    ///
    /// # Errors
    ///
    /// [`crate::StepImportError::UnbandedUncertainty`] where `eps_in`
    /// makes no band (`K·ε_in` past the format).
    pub(crate) fn of_import(
        eps_in: f64,
        overridden: bool,
        tol: Tol,
    ) -> Result<Self, crate::StepImportError> {
        Self::built(eps_in, overridden, tol.eps(), file_band(eps_in, tol))
    }

    /// [`Self::new`] for a run of ε `run_eps` and `k`: a unit test's
    /// band as production reads it, at a run it names.
    ///
    /// # Errors
    ///
    /// [`Self::new`]'s.
    #[cfg(test)]
    pub(crate) fn with_run(
        eps_in: f64,
        run_eps: f64,
        k: f64,
    ) -> Result<Self, crate::StepImportError> {
        Self::built(eps_in, false, run_eps, band_at(eps_in, run_eps, k))
    }

    /// The file band from its parts, refused where `band` is none.
    fn built(
        eps_in: f64,
        overridden: bool,
        run_eps: f64,
        band: Option<Band>,
    ) -> Result<Self, crate::StepImportError> {
        let band =
            band.ok_or(crate::StepImportError::UnbandedUncertainty { eps_in, overridden })?;
        Ok(Self {
            eps_in,
            run_eps,
            band,
        })
    }

    /// The band `(eps_in, K·eps_in)` at the default K, with no run floor:
    /// a unit test's budget read as the file states it.
    #[cfg(test)]
    #[allow(clippy::expect_used)]
    pub(crate) fn test(eps_in: f64) -> Self {
        Self {
            eps_in,
            run_eps: 0.0,
            band: Band::new(eps_in, geom_core::Tol::witness().k() * eps_in)
                .expect("a test's budget makes a band"),
        }
    }

    /// No budget, read as an exact comparison would: the unit pass's
    /// resolver, which reads no coordinate.
    pub(crate) fn unread() -> Self {
        Self {
            eps_in: 0.0,
            run_eps: 0.0,
            band: Band::least(),
        }
    }

    /// The file's declared coincidence distance, in metres.
    pub(crate) fn eps_in(self) -> f64 {
        self.eps_in
    }

    /// The run's ε, which floors the band.
    pub(crate) fn run_eps(self) -> f64 {
        self.run_eps
    }

    /// The band itself, for a kernel reading that takes one.
    pub(crate) fn band(self) -> Band {
        self.band
    }

    /// **`gap` (metres, a distance) read at the file's band** under the
    /// decision `name`. A gap the band cannot read (a non-finite gap) is
    /// [`Read::InBand`]: nothing is decided from it.
    pub(crate) fn read(self, name: &'static str, gap: f64) -> Read {
        match k_stats::decide(name, Margin::of(gap.abs()), self.band) {
            Ok(sign) if sign.is_zero() => Read::Zero,
            Ok(_) => Read::Apart,
            Err(_) => Read::InBand,
        }
    }

    /// The refusal of `gap` read Zero under `reading`, where the act needs
    /// the two apart.
    pub(crate) fn touching(self, reading: &'static str, gap: f64) -> Refusal {
        Refusal::Touching {
            reading,
            gap: gap.abs(),
            k: self.band.escalate() / self.band.zero(),
            run_eps: self.run_eps,
        }
    }

    /// The refusal of `gap` read in band under `reading`, where the act
    /// needs the two apart: reading them one would refuse too
    /// ([`Self::touching`]), so only the levers that read them apart are
    /// offered.
    pub(crate) fn apart_in_band(self, reading: &'static str, gap: f64) -> Refusal {
        Refusal::ApartInBand {
            reading,
            gap: gap.abs(),
            k: self.band.escalate() / self.band.zero(),
            run_eps: self.run_eps,
        }
    }

    /// The ambiguity of `gap` read in band under `reading`, a decision
    /// whose two answers are both the file's to mean.
    pub(crate) fn ambiguous(self, reading: &'static str, gap: f64) -> Refusal {
        Refusal::Ambiguous {
            reading,
            gap: gap.abs(),
            k: self.band.escalate() / self.band.zero(),
            run_eps: self.run_eps,
        }
    }

    /// **A length a construction takes only positive** (a radius, a
    /// semi-axis), as the file states it at `id`: refused as an invalid
    /// root where it is not a positive number, and as a length the band
    /// cannot tell from zero where it does not read apart.
    ///
    /// # Errors
    ///
    /// [`Refusal::InvalidRoot`] or [`Refusal::LengthUnread`].
    pub(crate) fn positive_length(
        self,
        id: u64,
        what: &'static str,
        length: f64,
    ) -> Result<f64, crate::StepImportError> {
        if !(length.is_finite() && length > 0.0) {
            return Err(self.refused(
                id,
                Refusal::InvalidRoot {
                    what,
                    value: length,
                    unit: "m",
                },
            ));
        }
        match self.read(STEP_POSITIVE_LENGTH, length) {
            Read::Apart => Ok(length),
            Read::Zero | Read::InBand => Err(self.refused(id, self.length_unread(what, length))),
        }
    }

    /// The refusal of a positive `length` the band does not tell from
    /// zero.
    pub(crate) fn length_unread(self, what: &'static str, length: f64) -> Refusal {
        Refusal::LengthUnread {
            what,
            length,
            k: self.band.escalate() / self.band.zero(),
            run_eps: self.run_eps,
        }
    }

    /// `gap` read under `name` at `id`, refused as the import's typed
    /// interpretation refusal unless it reads Zero ([`Self::agree`]).
    pub(crate) fn agree_at(
        self,
        id: u64,
        name: &'static str,
        gap: f64,
    ) -> Result<(), crate::StepImportError> {
        self.agree(name, gap)
            .map_err(|refusal| self.refused(id, refusal))
    }

    /// `refusal` at the file entity `id`, as the import door's error.
    pub(crate) fn refused(self, id: u64, refusal: Refusal) -> crate::StepImportError {
        crate::StepImportError::Interpretation {
            id,
            refusal,
            file: geom_core::FileCoincidence::new(self.eps_in),
        }
    }

    /// `gap` read under `name`, refused unless it reads Zero: a decision
    /// whose one passing arm is "the file's statements agree within its
    /// band", so its refused arms are misses, past the band
    /// ([`Refusal::EvidenceMiss`]) or in it ([`Refusal::MissInBand`]),
    /// and none offers a finer ε_in.
    pub(crate) fn agree(self, name: &'static str, gap: f64) -> Result<(), Refusal> {
        let miss = gap.abs();
        match self.read(name, gap) {
            Read::Zero => Ok(()),
            Read::InBand => Err(Refusal::MissInBand {
                reading: name,
                miss,
            }),
            Read::Apart => Err(Refusal::EvidenceMiss {
                reading: name,
                miss,
            }),
        }
    }
}

/// **Which lever decides**: moving `value` (ε_in, or the run's ε) below
/// `below` reads a distance of `below · K` apart from zero exactly where
/// `value` is not already below it. The one spelling of the rule: every
/// ending that names a finer ε_in or tolerance asks it here.
pub(crate) fn lever_decides(value: f64, below: f64) -> bool {
    value >= below
}

/// What reads a distance `below` apart from zero: every one of ε_in and
/// the run's ε that is not already below it, moved below it
/// ([`lever_decides`]).
fn finer(below: f64, eps_in: f64, run_eps: f64) -> String {
    use geom_core::Readable;
    let mut levers = Vec::new();
    if lever_decides(eps_in, below) {
        levers.push(format!(
            "import with ε_in below {} m (re-export with a smaller declared uncertainty, or pass \
             the override)",
            Readable(below)
        ));
    }
    if lever_decides(run_eps, below) {
        levers.push(format!("tighten the tolerance below {} m", Readable(below)));
    }
    levers.join(" and ")
}

/// **An interpretation refusal** (D7): the file's evidence read at its
/// band says no description, or more than one. Each names the reading,
/// the distance in metres, and (with the file's ε_in the error carries)
/// the recourse in the terms that decide it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Refusal {
    /// **An evidence miss**: two of the file's statements that must agree
    /// for its reading to hold disagree past its band. Either the file is
    /// damaged, or it is less exact than it declares; importing with an
    /// ε_in at least the miss reads them as one.
    EvidenceMiss {
        /// The reading (its decision name).
        reading: &'static str,
        /// The miss, in metres.
        miss: f64,
    },
    /// **A miss in band**: two of the file's statements that must agree
    /// for its reading to hold lie in the band the file is read at, so
    /// whether they agree is undecided. Only agreement passes, so the one
    /// lever is an ε_in at least the miss.
    MissInBand {
        /// The reading (its decision name).
        reading: &'static str,
        /// The miss, in metres.
        miss: f64,
    },
    /// **An in-band ambiguity**: the distance lies in the band the file
    /// is read at, `(e, K·e)` for `e` the larger of ε_in and the run's ε,
    /// where it is undecided whether the two are one. An ε_in at or above
    /// `gap` reads them one; reading them apart takes a band whose zero
    /// is below `gap / K`, so both ε_in and the run's ε below it.
    Ambiguous {
        /// The reading (its decision name).
        reading: &'static str,
        /// The distance, in metres.
        gap: f64,
        /// The band's K.
        k: f64,
        /// The run's ε (metres), which floors the band.
        run_eps: f64,
    },
    /// **An invalid root**: a value no kernel construction takes,
    /// whatever ε_in is (a radius that is not positive, a half-angle
    /// outside `(0, π/2)`).
    InvalidRoot {
        /// What the value is, and what a construction needs.
        what: &'static str,
        /// The value as the file states it, in `unit`.
        value: f64,
        /// The value's unit, as the refusal prints it (`m`, `rad`).
        unit: &'static str,
    },
    /// **Two statements that read as one where the act needs them
    /// apart**: the distance reads Zero at the band the file is read at
    /// (a face's bound touching another). Reading them apart takes a band
    /// whose escalation is below the distance, so both ε_in and the run's
    /// ε below `gap / K`; at a distance of zero, no band does.
    Touching {
        /// The reading (its decision name).
        reading: &'static str,
        /// The distance, in metres.
        gap: f64,
        /// The band's K.
        k: f64,
        /// The run's ε (metres), which floors the band.
        run_eps: f64,
    },
    /// **A distance in band where the act needs the two apart**: the
    /// distance lies in the band the file is read at, and the act (a
    /// bound's containment of another) passes only where it reads apart;
    /// an ε_in that read it Zero would end at [`Self::Touching`] instead.
    /// Reading it apart takes both ε_in and the run's ε below `gap / K`.
    ApartInBand {
        /// The reading (its decision name).
        reading: &'static str,
        /// The distance, in metres.
        gap: f64,
        /// The band's K.
        k: f64,
        /// The run's ε (metres), which floors the band.
        run_eps: f64,
    },
    /// **A length the band cannot tell from zero**: a positive length a
    /// construction takes only positive (a radius, a semi-axis) that does
    /// not read apart from zero at the band the file is read at. Reading
    /// it as a length takes a band whose escalation is below it, so both
    /// ε_in and the run's ε below `length / K`.
    LengthUnread {
        /// What the length is.
        what: &'static str,
        /// The length, in metres.
        length: f64,
        /// The band's K.
        k: f64,
        /// The run's ε (metres), which floors the band.
        run_eps: f64,
    },
}

impl Refusal {
    /// The ambiguity a kernel reading at the file's band escalated with
    /// (`cause`, its margin and band) under a run of ε `run_eps`, where
    /// the margin is a point value.
    pub(crate) fn in_band(
        reading: &'static str,
        cause: &geom_core::Indeterminate,
        run_eps: f64,
    ) -> Option<Self> {
        let gap = cause.margin.diagnostic_f64_for_error_text().value()?;
        Some(Self::Ambiguous {
            reading,
            gap: gap.abs(),
            k: cause.band.escalate() / cause.band.zero(),
            run_eps,
        })
    }

    /// The refusal's payload: the reading and its distance, against the
    /// file's `eps_in`.
    pub(crate) fn payload(&self, eps_in: f64) -> String {
        use geom_core::Readable;
        match *self {
            Self::EvidenceMiss { reading, miss } => format!(
                "the file's statements miss each other by {} m ({reading}), past the band it is \
                 read at (its declared coincidence distance ε_in = {} m)",
                Readable(miss),
                Readable(eps_in)
            ),
            Self::MissInBand { reading, miss } => format!(
                "the file's statements miss each other by {} m ({reading}), in the band it is \
                 read at (its declared coincidence distance ε_in = {} m), where whether they \
                 agree is undecided",
                Readable(miss),
                Readable(eps_in)
            ),
            Self::Ambiguous {
                reading,
                gap,
                run_eps,
                ..
            } => format!(
                "the distance {} m ({reading}) lies in the band the file is read at (from the \
                 larger of its declared ε_in = {} m and this run's tolerance {} m), where it is \
                 undecided whether the two are one",
                Readable(gap),
                Readable(eps_in),
                Readable(run_eps)
            ),
            Self::Touching {
                reading,
                gap,
                run_eps,
                ..
            } => format!(
                "the distance {} m ({reading}) reads as none at the band the file is read at (from \
                 the larger of its declared ε_in = {} m and this run's tolerance {} m), where the \
                 two must stand apart",
                Readable(gap),
                Readable(eps_in),
                Readable(run_eps)
            ),
            Self::ApartInBand {
                reading,
                gap,
                run_eps,
                ..
            } => format!(
                "the distance {} m ({reading}) lies in the band the file is read at (from the \
                 larger of its declared ε_in = {} m and this run's tolerance {} m), where the \
                 two must stand apart and do not read so",
                Readable(gap),
                Readable(eps_in),
                Readable(run_eps)
            ),
            Self::InvalidRoot { what, value, unit } => format!(
                "{what}: the file states {value} {unit}, which no kernel construction takes at any \
                 ε_in"
            ),
            Self::LengthUnread {
                what,
                length,
                run_eps,
                ..
            } => format!(
                "{what}: the file states {} m, which the band it is read at (from the larger of \
                 its declared ε_in = {} m and this run's tolerance {} m) does not tell from zero",
                Readable(length),
                Readable(eps_in),
                Readable(run_eps)
            ),
        }
    }

    /// The refusal's one ending at the import door: its recourse, in the
    /// terms that decide it.
    pub(crate) fn ending(&self, eps_in: f64) -> String {
        self.ending_with("re-export the file", eps_in)
    }

    /// [`Self::ending`] with the decision's own `lever` (the move that
    /// makes the reading true) in place of re-exporting the file.
    pub(crate) fn ending_with(&self, lever: &str, eps_in: f64) -> String {
        use geom_core::Readable;
        match *self {
            Self::EvidenceMiss { miss, .. } | Self::MissInBand { miss, .. } => format!(
                "Recourse: {lever}, or, if the file is less exact than it declares, import with \
                 ε_in ≥ {} m",
                Readable(miss)
            ),
            Self::Ambiguous {
                gap, k, run_eps, ..
            } => format!(
                "Recourse: {lever}, or, if this distance is intended, {}, or, if it is not, \
                 import with ε_in ≥ {} m",
                finer(gap / k, eps_in, run_eps),
                Readable(gap)
            ),
            Self::Touching {
                gap, k, run_eps, ..
            } => {
                if gap > 0.0 {
                    format!(
                        "Recourse: {lever}, or, if this distance is intended, {}",
                        finer(gap / k, eps_in, run_eps)
                    )
                } else {
                    format!("Recourse: {lever}, moving the two apart")
                }
            }
            Self::ApartInBand {
                gap, k, run_eps, ..
            } => format!(
                "Recourse: {lever}, or, if this distance is intended, {}",
                finer(gap / k, eps_in, run_eps)
            ),
            Self::LengthUnread {
                length, k, run_eps, ..
            } => format!(
                "Recourse: {lever}, or, if this length is intended, {}",
                finer(length / k, eps_in, run_eps)
            ),
            Self::InvalidRoot { .. } => format!("Recourse: {lever} from a valid model"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Refusal;

    /// **Touching bounds end in what parts them**: at a distance of zero
    /// no band reads them apart, so the ending is the lever alone, never a
    /// value of zero; at a positive distance it names the levers that do.
    #[test]
    fn touching_names_what_parts_them_and_no_lever_at_zero() {
        let touching = |gap: f64| Refusal::Touching {
            reading: "r",
            gap,
            k: 10.0,
            run_eps: 1e-9,
        };
        let at_zero = touching(0.0).ending(1e-6);
        assert!(
            !at_zero.contains("below") && !at_zero.contains("≥") && !at_zero.contains(" 0 m"),
            "{at_zero}"
        );
        let apart = touching(5e-7).ending(1e-6);
        assert!(apart.contains("import with ε_in below 5e-8 m"), "{apart}");
        assert!(
            !apart.contains("tighten"),
            "the run's ε is already below: {apart}"
        );
        assert!(
            !apart.contains("≥"),
            "no lever that reads them one: {apart}"
        );
    }

    /// **An ambiguity's ending names exactly the levers that decide it.**
    /// Apart takes a band whose zero is below `gap / K`, and the band's
    /// zero is the larger of ε_in and the run's ε, so each of the two
    /// that is not already below it is named, and neither that is.
    #[test]
    fn an_ambiguity_names_each_lever_not_already_below_the_distance_over_k() {
        let ending = |eps_in: f64, run_eps: f64| {
            Refusal::Ambiguous {
                reading: "r",
                gap: 3e-6,
                k: 10.0,
                run_eps,
            }
            .ending(eps_in)
        };
        for (eps_in, run_eps, file, run) in [
            (1e-6, 1e-9, true, false),
            (5e-7, 1e-6, true, true),
            (1e-7, 1e-6, false, true),
            (3e-7, 3e-7, true, true),
        ] {
            let text = ending(eps_in, run_eps);
            assert_eq!(
                text.contains("import with ε_in below 3e-7 m"),
                file,
                "ε_in {eps_in}, ε {run_eps}: {text}"
            );
            assert_eq!(
                text.contains("tighten the tolerance below 3e-7 m"),
                run,
                "ε_in {eps_in}, ε {run_eps}: {text}"
            );
            assert!(text.contains("with ε_in ≥ 3e-6 m"), "{text}");
        }
    }
}

//! **The interpretation layer's readings** (`[import]`, D7): what the
//! file's statements are evidence of, read at the file's band.
//!
//! The file declares a coincidence distance ε_in. Every question import
//! asks of the file's evidence (does this point lie on that sphere, are
//! these two planes one locus, is this patch a cylinder) is a distance in
//! metres read through the kernel's one classification funnel
//! (`geom_core::k_stats::decide`) against the file's band
//! ([`file_band`]), with the three arms `[verdicts]` gives every reading:
//! [`Read::Zero`] (one, within ε_in), [`Read::InBand`] (undecided at the
//! file's precision) and [`Read::Apart`] (definitely not). A reading
//! that is a lane choice takes its closed form at Zero and its general
//! lane otherwise; a structural one acts at Zero, keeps apart where
//! apart, and refuses in band ([`Refusal::Ambiguous`]), naming the ε_in
//! that would decide it.

use geom_core::predicate::Band;
use geom_core::{Margin, Tol, k_stats};

/// **The band the file is read at** (D3: import reads the kernel's
/// readings with the file's own coincidence distance as the band):
/// `(e, K·e)` for `e` the larger of ε_in and the run's ε, the run's K. A
/// file declaring less than the run's tolerance is still read no finer
/// than the run can certify. A file with no budget (`ε_in` of `0`,
/// refused later as `MissingUncertainty`) reads at [`Band::least`], as an
/// exact comparison would. `None` where the pair makes no band (`K·e`
/// overflowing, or rounding back onto `e`).
pub(crate) fn file_band(eps_in: f64, tol: Tol) -> Option<Band> {
    if eps_in == 0.0 {
        return Some(Band::least());
    }
    let e = eps_in.max(tol.eps());
    Band::new(e, tol.k() * e).ok()
}

/// A recognized kind's certified residual against the file's statement
/// (a NURBS patch or carrier promoted to an analytic one): Zero promotes.
pub(crate) const STEP_RECOGNIZED_RESIDUAL: &str = "step_recognized_residual";

/// A recognition estimator's conditioning (a span, a sagitta, a radius,
/// a chord): Zero determines no candidate; in band and apart go on to
/// the certificate, which does not rest on the answer.
pub(crate) const STEP_RECOGNIZED_CONDITIONING: &str = "step_recognized_conditioning";

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

/// Whether a seamless band's two rims meet at the seam azimuth (the
/// minted generator's length): only apart mints it.
pub(crate) const STEP_BAND_SEAM_LENGTH: &str = "step_band_seam_length";

/// Whether a rim vertex already sits at a band's seam azimuth: Zero uses
/// it, apart splits the rim there.
pub(crate) const STEP_BAND_SEAM_VERTEX: &str = "step_band_seam_vertex";

/// The file's ε_in and the band it is read at.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FileBand {
    eps_in: f64,
    band: Option<Band>,
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
    /// The band for `eps_in` under the run's `tol` ([`file_band`]).
    pub(crate) fn new(eps_in: f64, tol: Tol) -> Self {
        Self {
            eps_in,
            band: file_band(eps_in, tol),
        }
    }

    /// The band `(eps_in, K·eps_in)` at the default K, with no run floor:
    /// a unit test's budget read as the file states it.
    #[cfg(test)]
    pub(crate) fn test(eps_in: f64) -> Self {
        Self {
            eps_in,
            band: Band::new(eps_in, geom_core::Tol::witness().k() * eps_in).ok(),
        }
    }

    /// No budget and no band: the unit pass's resolver, which reads no
    /// coordinate.
    pub(crate) fn unread() -> Self {
        Self {
            eps_in: 0.0,
            band: None,
        }
    }

    /// The file's declared coincidence distance, in metres.
    pub(crate) fn eps_in(self) -> f64 {
        self.eps_in
    }

    /// The band's K (its escalation over its zero), `NaN` with no band.
    pub(crate) fn k(self) -> f64 {
        self.band.map_or(f64::NAN, |b| b.escalate() / b.zero())
    }

    /// The band itself, for a kernel reading that takes one.
    pub(crate) fn band(self) -> Option<Band> {
        self.band
    }

    /// **`gap` (metres, a distance) read at the file's band** under the
    /// decision `name`. A gap the band cannot read (no band, a non-finite
    /// gap) is [`Read::InBand`]: nothing is decided from it.
    pub(crate) fn read(self, name: &'static str, gap: f64) -> Read {
        let Some(band) = self.band else {
            return Read::InBand;
        };
        match k_stats::decide(name, Margin::of(gap.abs()), band) {
            Ok(sign) if sign.is_zero() => Read::Zero,
            Ok(_) => Read::Apart,
            Err(_) => Read::InBand,
        }
    }

    /// The refusal of a structural reading of `gap` that came out
    /// [`Read::InBand`] or [`Read::Apart`]: an ambiguity or an evidence
    /// miss, `None` at [`Read::Zero`].
    pub(crate) fn refusal(self, reading: &'static str, gap: f64, read: Read) -> Option<Refusal> {
        match read {
            Read::Zero => None,
            Read::InBand => Some(self.ambiguous(reading, gap)),
            Read::Apart => Some(Refusal::EvidenceMiss {
                reading,
                miss: gap.abs(),
            }),
        }
    }

    /// The ambiguity of `gap` read in band under `reading`.
    pub(crate) fn ambiguous(self, reading: &'static str, gap: f64) -> Refusal {
        Refusal::Ambiguous {
            reading,
            gap: gap.abs(),
            k: self.k(),
            floor: self.band.map_or(self.eps_in, Band::zero),
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

    /// `gap` read under `name`, and refused unless it reads Zero: the
    /// shape of a reading whose one passing arm is "the file's statements
    /// agree within ε_in".
    pub(crate) fn agree(self, name: &'static str, gap: f64) -> Result<(), Refusal> {
        match self.refusal(name, gap, self.read(name, gap)) {
            None => Ok(()),
            Some(refusal) => Err(refusal),
        }
    }
}

/// **An interpretation refusal** (D7): the file's evidence read at its
/// band says no description, or more than one. Each names the reading,
/// the distance in metres, and (with the file's ε_in the error carries)
/// the recourse in ε_in's own terms.
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
    /// **An in-band ambiguity**: the distance lies in the band the file
    /// is read at, past its zero but within `K` times it, where the
    /// file's precision does not decide whether the two are one. A band
    /// whose zero is below `gap / K` reads them apart, one at or above
    /// `gap` reads them one.
    Ambiguous {
        /// The reading (its decision name).
        reading: &'static str,
        /// The distance, in metres.
        gap: f64,
        /// The band's K.
        k: f64,
        /// The band's zero: ε_in, or the run's ε where that is coarser.
        floor: f64,
    },
    /// **An invalid root**: a value a kernel constructor refuses whatever
    /// ε_in is (a nonpositive radius, a half-angle outside `(0, π/2)`).
    InvalidRoot {
        /// What the value is and what the constructor needs.
        what: &'static str,
        /// The value as the file states it, in kernel units.
        value: f64,
    },
}

impl Refusal {
    /// The ambiguity a kernel reading at the file's band escalated with
    /// (`cause`, its margin and band), where the margin is a point value.
    pub(crate) fn in_band(reading: &'static str, cause: &geom_core::Indeterminate) -> Option<Self> {
        let gap = cause.margin.diagnostic_f64_for_error_text().value()?;
        Some(Self::Ambiguous {
            reading,
            gap: gap.abs(),
            k: cause.band.escalate() / cause.band.zero(),
            floor: cause.band.zero(),
        })
    }

    /// The refusal's payload: the reading and its distance, against the
    /// file's `eps_in`.
    pub(crate) fn payload(&self, eps_in: f64) -> String {
        use geom_core::Readable;
        match *self {
            Self::EvidenceMiss { reading, miss } => format!(
                "the file's statements miss each other by {} m ({reading}), past its declared \
                 coincidence distance ε_in = {} m",
                Readable(miss),
                Readable(eps_in)
            ),
            Self::Ambiguous {
                reading,
                gap,
                floor,
                ..
            } if floor > eps_in => format!(
                "the distance {} m ({reading}) lies in the band the file is read at (from {} m, \
                 this run's tolerance, coarser than the file's declared ε_in = {} m), where it is \
                 undecided whether the two are one",
                Readable(gap),
                Readable(floor),
                Readable(eps_in)
            ),
            Self::Ambiguous { reading, gap, .. } => format!(
                "the distance {} m ({reading}) lies in the file's band (ε_in = {} m, K·ε_in), \
                 where its declared precision does not decide whether the two are one",
                Readable(gap),
                Readable(eps_in)
            ),
            Self::InvalidRoot { what, value } => format!(
                "{what}: the file states {value}, which no kernel construction takes at any ε_in"
            ),
        }
    }

    /// The refusal's one ending at the import door: its recourse, in
    /// ε_in's own terms.
    pub(crate) fn ending(&self, eps_in: f64) -> String {
        self.ending_with("re-export the file", eps_in)
    }

    /// [`Self::ending`] with the decision's own `lever` (the move that
    /// makes the reading true) in place of re-exporting the file.
    pub(crate) fn ending_with(&self, lever: &str, eps_in: f64) -> String {
        use geom_core::Readable;
        match *self {
            Self::EvidenceMiss { miss, .. } => format!(
                "Recourse: {lever}, or, if the file is less exact than it declares, import with \
                 ε_in ≥ {} m",
                Readable(miss)
            ),
            Self::Ambiguous { gap, k, floor, .. } if floor > eps_in => format!(
                "Recourse: {lever}, or, if this distance is intended, tighten the tolerance below \
                 {} m (the file is read no finer than the run's tolerance), or, if it is not, \
                 import with ε_in ≥ {} m",
                Readable(gap / k),
                Readable(gap)
            ),
            Self::Ambiguous { gap, k, .. } => format!(
                "Recourse: {lever}, or, if this distance is intended, import with ε_in below {} m \
                 (re-export with a smaller declared uncertainty, or pass the override), or, if it \
                 is not, with ε_in ≥ {} m",
                Readable(gap / k),
                Readable(gap)
            ),
            Self::InvalidRoot { .. } => format!("Recourse: {lever} from a valid model"),
        }
    }
}

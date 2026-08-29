//! `Distribution` — the optional uncertainty annotation on a
//! continuous document parameter (ERROR-DESIGN E1/E2).
//!
//! **Document-layer metadata, and nothing else.** A distribution is
//! inert data hanging off a parameter: it feeds no evaluation, no
//! content key, no predicate. The kernel and geometry lanes never see
//! a probability; the ONE reader is [`crate::analysis`], which
//! projects a distribution to the analyzed box and to mass (E1's
//! boundary).
//!
//! **Offsets, in the parameter's own dimension.** Every field is an
//! offset RELATIVE to the parameter's nominal value, in canonical
//! kernel units of the parameter's own `dim` — the nominal stays the
//! single source of truth, and there is no separate dimension field
//! to disagree with it (E2).
//!
//! **Independence** (PL6): one distribution per parameter, product
//! measure only. Two slots driven by the SAME parameter name carry one
//! comoving marginal; distinct names are independent; derived
//! expressions comove through evaluation and carry nothing of their
//! own. Joint forms are foreclosed in v1 (E11.2).
//!
//! **Band carries no measure** (E2). It states limits without a shape,
//! which is real information; defaulting it to uniform would be a
//! different and stronger claim. Every mass door refuses on a Band.

/// The v1 distribution vocabulary (E2, verbatim).
///
/// Offsets relative to the parameter's nominal, in its own dimension.
/// Every inhabitant satisfies [`Distribution::check`]: the
/// construction doors (`SetDocParam`, the persistence validator)
/// refuse the rest.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
// Externally tagged with the variant names as written, matching the
// neighbouring `DocParam` spelling this type is nested inside.
#[serde(deny_unknown_fields)]
pub enum Distribution {
    /// Worst-case limits with NO shape claim, and therefore no
    /// measure: `[lo, hi]` bounds the parameter and prices nothing.
    Band {
        /// Lower offset (`<= 0`).
        lo: f64,
        /// Upper offset (`>= 0`).
        hi: f64,
    },
    /// Uniform over `[lo, hi]`.
    Uniform {
        /// Lower offset (`<= 0`).
        lo: f64,
        /// Upper offset (`>= 0`).
        hi: f64,
    },
    /// Zero-mean normal with standard deviation `sigma > 0`.
    /// Unbounded support — the analysis box is the analysis's knob,
    /// never a cutoff baked into the model (E2).
    Normal {
        /// Standard deviation, in the parameter's dimension.
        sigma: f64,
    },
    /// A [`Distribution::Normal`] restricted and renormalized to
    /// `[lo, hi]`: sugar whose tail mass is identically zero.
    TruncatedNormal {
        /// Standard deviation of the underlying normal.
        sigma: f64,
        /// Lower offset (`<= 0`).
        lo: f64,
        /// Upper offset (`>= 0`).
        hi: f64,
    },
}

/// Which field of a distribution a refusal is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistributionField {
    /// The standard deviation.
    Sigma,
    /// The lower offset.
    Lo,
    /// The upper offset.
    Hi,
}

impl core::fmt::Display for DistributionField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Sigma => "sigma",
            Self::Lo => "lo",
            Self::Hi => "hi",
        })
    }
}

/// A broken distribution invariant, named exactly (the typed refusal
/// both construction doors carry).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DistributionFault {
    /// A field is NaN or infinite.
    NonFinite {
        /// The offending field.
        field: DistributionField,
    },
    /// `sigma` is zero or negative — a degenerate normal is not a
    /// distribution, and a fixed parameter is spelled by having NO
    /// distribution.
    SigmaNotPositive {
        /// The offending value.
        sigma: f64,
    },
    /// The bounded form does not contain its own nominal:
    /// `lo <= 0 <= hi` fails (asymmetric bounds are legal; a nominal
    /// outside its own support is a document error, E2).
    NominalOutsideSupport {
        /// The lower offset.
        lo: f64,
        /// The upper offset.
        hi: f64,
    },
}

impl core::fmt::Display for DistributionFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite { field } => write!(f, "distribution field {field} is not finite"),
            Self::SigmaNotPositive { sigma } => {
                write!(f, "distribution sigma must be positive, got {sigma}")
            }
            Self::NominalOutsideSupport { lo, hi } => write!(
                f,
                "distribution bounds must contain the nominal (lo <= 0 <= hi), got lo = {lo}, hi = {hi}"
            ),
        }
    }
}

impl Distribution {
    /// The first non-finite field, if any.
    ///
    /// Split out from [`Self::check`] so the persistence validator can
    /// report a non-finite distribution field through the SAME
    /// non-finite walk (and the same site vocabulary) every other
    /// float in the document goes through, rather than minting a
    /// second spelling for the same defect.
    pub fn first_non_finite(&self) -> Option<DistributionField> {
        let bad = |field: DistributionField, v: f64| (!v.is_finite()).then_some(field);
        match *self {
            Self::Band { lo, hi } | Self::Uniform { lo, hi } => bad(DistributionField::Lo, lo)
                .or_else(|| bad(DistributionField::Hi, hi)),
            Self::Normal { sigma } => bad(DistributionField::Sigma, sigma),
            Self::TruncatedNormal { sigma, lo, hi } => bad(DistributionField::Sigma, sigma)
                .or_else(|| bad(DistributionField::Lo, lo))
                .or_else(|| bad(DistributionField::Hi, hi)),
        }
    }

    /// Every invariant of §E2, checked: fields finite, `sigma > 0`
    /// where present, `lo <= 0 <= hi` for the bounded forms.
    ///
    /// The ONE statement of the invariants; both construction doors
    /// call it, so a file that would refuse to load cannot be built
    /// through an edit.
    pub fn check(&self) -> Result<(), DistributionFault> {
        if let Some(field) = self.first_non_finite() {
            return Err(DistributionFault::NonFinite { field });
        }
        if let Self::Normal { sigma } | Self::TruncatedNormal { sigma, .. } = *self
            && sigma <= 0.0
        {
            return Err(DistributionFault::SigmaNotPositive { sigma });
        }
        // Every field is finite here, so the two comparisons are the
        // exact negation of `lo <= 0 <= hi`.
        if let Self::Band { lo, hi } | Self::Uniform { lo, hi } | Self::TruncatedNormal { lo, hi, .. } =
            *self
            && (lo > 0.0 || hi < 0.0)
        {
            return Err(DistributionFault::NominalOutsideSupport { lo, hi });
        }
        Ok(())
    }

    /// The bounded support as an offset interval, or `None` for the
    /// unbounded [`Self::Normal`].
    pub fn support(&self) -> Option<(f64, f64)> {
        match *self {
            Self::Band { lo, hi } | Self::Uniform { lo, hi } | Self::TruncatedNormal { lo, hi, .. } => {
                Some((lo, hi))
            }
            Self::Normal { .. } => None,
        }
    }

    /// Bit-exact equality on the floats — the document's replay
    /// identity (D7), where `0.0` and `-0.0` are different offsets.
    ///
    /// EXHAUSTIVE on purpose, on BOTH sides of the pair, for the
    /// reason [`crate::DocParam::bit_eq`] gives: a wildcard would
    /// answer `false` for a new variant against ITSELF.
    pub fn bit_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Band { lo: a, hi: b }, Self::Band { lo: c, hi: d })
            | (Self::Uniform { lo: a, hi: b }, Self::Uniform { lo: c, hi: d }) => {
                a.to_bits() == c.to_bits() && b.to_bits() == d.to_bits()
            }
            (Self::Normal { sigma: a }, Self::Normal { sigma: b }) => a.to_bits() == b.to_bits(),
            (
                Self::TruncatedNormal {
                    sigma: sa,
                    lo: la,
                    hi: ha,
                },
                Self::TruncatedNormal {
                    sigma: sb,
                    lo: lb,
                    hi: hb,
                },
            ) => {
                sa.to_bits() == sb.to_bits()
                    && la.to_bits() == lb.to_bits()
                    && ha.to_bits() == hb.to_bits()
            }
            (Self::Band { .. }, Self::Uniform { .. } | Self::Normal { .. } | Self::TruncatedNormal { .. })
            | (Self::Uniform { .. }, Self::Band { .. } | Self::Normal { .. } | Self::TruncatedNormal { .. })
            | (Self::Normal { .. }, Self::Band { .. } | Self::Uniform { .. } | Self::TruncatedNormal { .. })
            | (
                Self::TruncatedNormal { .. },
                Self::Band { .. } | Self::Uniform { .. } | Self::Normal { .. },
            ) => false,
        }
    }
}

//! Test vocabulary over this crate's types, for the suites downstream.

use crate::{Affine3, Band, Decide, ErrorTextReading, MarginDiag, RigidMotion, Tol};

/// The upper end of a reporting margin: the point margin, or the
/// enclosure's upper end; `NaN` for a poisoned reading, which no bound
/// comparison passes. The rows that pin a refusal's bound against a
/// planted miss read it here.
#[must_use]
pub fn upper(margin: MarginDiag) -> f64 {
    match margin.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(m) | ErrorTextReading::Enclosure { hi: m, .. } => m,
        ErrorTextReading::Invalid => f64::NAN,
    }
}

/// The lower end of a reporting margin: the point margin, or the
/// enclosure's lower end; `NaN` for a poisoned reading. A measured
/// miss's floor reads here: at an enclosure scalar limb 2 refuses on
/// the enclosure `[floor, upper]` of the carrier's distance, whose
/// lower end is the floor.
#[must_use]
pub fn lower(margin: MarginDiag) -> f64 {
    match margin.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(m) | ErrorTextReading::Enclosure { lo: m, .. } => m,
        ErrorTextReading::Invalid => f64::NAN,
    }
}

/// **A matrix a suite holds, admitted at the decided door** at `tol`'s
/// linear band ([`RigidMotion::check_rigid`]): the question a suite
/// that builds its map as twelve numbers asks before it moves a body
/// by it. Panics, naming the refusal, when the map is not admitted.
#[must_use]
#[allow(clippy::panic)]
pub fn rigid<T: Decide>(map: &Affine3<T>, tol: Tol) -> RigidMotion<T> {
    let band =
        Band::linear(tol).unwrap_or_else(|e| panic!("the run's tolerance forms no band: {e}"));
    RigidMotion::check_rigid(map, band)
        .unwrap_or_else(|e| panic!("the map is not admitted as a rigid motion: {e}"))
}

/// The Bernstein row on `[a, b] ⊆ [0, 1]` of a `[0, 1]` row, by the
/// arithmetic the retired `restrict_bernstein` ran: the oracle for the
/// trim bisection's bit-identity row against the door it moved to,
/// [`crate::spline::restrict_segment`] on `[0, 1]`.
///
/// # Errors
///
/// As the retired door: an empty row, or an end outside `[0, 1]`.
pub fn bernstein_restriction_as_retired(
    row: &[crate::Interval],
    ends: (crate::Interval, crate::Interval),
) -> Result<Vec<crate::Interval>, crate::spline::BlossomError> {
    crate::spline::blossom::bernstein_restriction_as_retired(row, ends)
}

/// **A refused point bracket at `x`**: both ends exactly `x`, its
/// decoration below `Def` (`√[-1, 1]·0 + x`). The plant a census row
/// reads to show a site asks [`crate::Interval::is_certified`] rather
/// than trusting ordinary ends.
#[must_use]
pub fn refused_point(x: f64) -> crate::Interval {
    use crate::{Bounds as _, Real as _};
    let p = crate::Interval::from_bounds(-1.0, 1.0).sqrt() * crate::Interval::from_bounds(0.0, 0.0)
        + crate::Interval::from_bounds(x, x);
    assert!(
        !p.is_certified() && p.lo() == x && p.hi() == x,
        "the plant must be a refused point bracket at {x:e}: {p:?}"
    );
    p
}

/// **A fixture's length as a lever arm**: a test's arm with no geometry
/// behind it (a hand-stated row, a probe at a stated scale). The one
/// door for it, counted by `scripts/gates/lever-sources.sh`; production
/// code builds its levers from their sources (`crate::Lever`).
#[must_use]
pub fn fixture_lever<T: crate::Real>(length: T) -> crate::Lever<T> {
    crate::Lever(length)
}

/// **The reading of a scale twin's margin streams**: one body at two
/// model scales, each run's [`MarginSample`](crate::k_stats::MarginSample)s
/// grouped by predicate in recording order, and the two read side by side.
/// A twin compares verdict streams sample for sample, and pairs only the
/// decisive margins that have a scale ([`Twins::decisive_pairs`]).
#[cfg(feature = "probe")]
pub mod twins {
    use crate::Sign;
    use crate::k_stats::{MarginSample, SampleOutcome};
    use std::collections::BTreeMap;

    /// One run's samples by predicate, each list in recording order.
    pub type Streams = BTreeMap<&'static str, Vec<MarginSample>>;

    /// `samples` grouped by predicate, in recording order.
    #[must_use]
    pub fn streams(samples: impl IntoIterator<Item = MarginSample>) -> Streams {
        let mut out = Streams::new();
        for s in samples {
            out.entry(s.predicate).or_default().push(s);
        }
        out
    }

    /// One predicate's streams in the smaller twin and the larger.
    #[derive(Clone, Copy, Debug)]
    pub struct Twins<'a> {
        /// The predicate both streams are of, for the messages.
        pub predicate: &'a str,
        /// The smaller twin's samples, in recording order.
        pub small: &'a [MarginSample],
        /// The larger twin's samples, in recording order.
        pub large: &'a [MarginSample],
    }

    impl<'a> Twins<'a> {
        /// `predicate`'s streams in both runs; an empty stream where a run
        /// did not fire it.
        #[must_use]
        pub fn of(predicate: &'a str, small: &'a Streams, large: &'a Streams) -> Self {
            let read = |s: &'a Streams| s.get(predicate).map_or(&[][..], Vec::as_slice);
            Twins {
                predicate,
                small: read(small),
                large: read(large),
            }
        }

        /// Each twin's verdict stream, in recording order.
        #[must_use]
        pub fn outcomes(&self) -> (Vec<SampleOutcome>, Vec<SampleOutcome>) {
            let read = |l: &[MarginSample]| l.iter().map(|s| s.outcome).collect();
            (read(self.small), read(self.large))
        }

        /// **The decisive margins with a scale, paired by rank**: each
        /// twin's definite nonzero samples not decided on their sign
        /// alone ([`MarginSample::reads_sign_only`]), as `|margin|`,
        /// those `keep` admits, sorted (recording order pairs unrelated
        /// sites when a predicate fires on several entities), then
        /// paired `(small, large)`.
        ///
        /// # Panics
        ///
        /// When the twins' sign-only masks differ (a sample read on its
        /// sign in one twin and against the band in the other), or the
        /// two decisive lists differ in length: pairing them would drop
        /// the excess unread.
        #[must_use]
        pub fn decisive_pairs(&self, keep: impl Fn(f64) -> bool) -> Vec<(f64, f64)> {
            let mask = |l: &[MarginSample]| {
                l.iter()
                    .map(MarginSample::reads_sign_only)
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                mask(self.small),
                mask(self.large),
                "{}: the twins read different samples on their sign alone",
                self.predicate
            );
            let decisive = |l: &[MarginSample]| {
                let mut v: Vec<f64> = l
                    .iter()
                    .filter(|s| {
                        matches!(
                            s.outcome,
                            SampleOutcome::Definite(Sign::Positive | Sign::Negative)
                        ) && !s.reads_sign_only()
                    })
                    .map(|s| s.margin.abs())
                    .filter(|m| keep(*m))
                    .collect();
                v.sort_by(f64::total_cmp);
                v
            };
            let (small, large) = (decisive(self.small), decisive(self.large));
            assert_eq!(
                small.len(),
                large.len(),
                "{}: the twins hold different numbers of decisive margins ({small:?} against \
                 {large:?})",
                self.predicate
            );
            small.into_iter().zip(large).collect()
        }
    }

    #[cfg(test)]
    #[allow(clippy::panic)]
    mod tests {
        use super::*;
        use crate::Band;

        fn sample(margin: f64, band: (f64, f64), outcome: SampleOutcome) -> MarginSample {
            MarginSample {
                predicate: "p",
                margin,
                band_zero: band.0,
                band_escalate: band.1,
                outcome,
            }
        }

        const POS: SampleOutcome = SampleOutcome::Definite(Sign::Positive);

        fn least() -> (f64, f64) {
            (Band::least().zero(), Band::least().escalate())
        }

        #[test]
        fn the_pairs_are_the_decisive_margins_with_a_scale_by_rank() {
            let linear = (1e-9, 1e-6);
            let small = [
                sample(-2e-3, linear, SampleOutcome::Definite(Sign::Negative)),
                sample(5e-7, least(), POS),
                sample(1e-3, linear, POS),
                sample(0.0, linear, SampleOutcome::Definite(Sign::Zero)),
            ];
            let large = [
                sample(1.0, linear, POS),
                sample(9.0, least(), POS),
                sample(-2.0, linear, SampleOutcome::Definite(Sign::Negative)),
                sample(0.0, linear, SampleOutcome::Definite(Sign::Zero)),
            ];
            let twins = Twins {
                predicate: "p",
                small: &small,
                large: &large,
            };
            assert_eq!(
                twins.decisive_pairs(|_| true),
                [(1e-3, 1.0), (2e-3, 2.0)],
                "sign-only and zero samples drop out, and the rest pair by rank"
            );
        }

        #[test]
        #[should_panic(expected = "read different samples on their sign alone")]
        fn twins_whose_sign_only_masks_differ_are_refused() {
            let linear = (1e-9, 1e-6);
            let small = [sample(1e-3, linear, POS), sample(5e-7, least(), POS)];
            let large = [sample(1.0, linear, POS), sample(9.0, linear, POS)];
            let _ = Twins {
                predicate: "p",
                small: &small,
                large: &large,
            }
            .decisive_pairs(|_| true);
        }

        #[test]
        #[should_panic(expected = "different numbers of decisive margins")]
        fn twins_with_unequal_decisive_lists_are_refused() {
            let linear = (1e-9, 1e-6);
            let small = [sample(1e-3, linear, POS), sample(2e-3, linear, POS)];
            let large = [sample(1.0, linear, POS), sample(2.0, linear, POS)];
            let _ = Twins {
                predicate: "p",
                small: &small,
                large: &large,
            }
            .decisive_pairs(|m| m < 1.5);
        }
    }
}

//! Test vocabulary over this crate's types, for the suites downstream.

use crate::{Affine3, Band, Decide, ErrorTextReading, MarginDiag, RigidMotion, Tol, Told};

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

/// **Whether the witness band is the one the carrier-frame cost rows
/// pin**, ε ≤ 1e-12 (PR 7131, Ev approved 2026-11-20): there each cost
/// row pins the refusal it gives, and at ε 1e-9 and 1e-6, where the
/// suites run it too, it builds. The one home of the rows' causes, each
/// named by the row that pins it:
///
/// - **polar**: a stored frame read as its polar enclosure (decision 1).
///   A frame stored at `f64` a rounding off rigid reads at `Interval`
///   about an ulp wide per entry, and a read levered by a far extent
///   carries that past the 1e-12 band. Mutant **A** (the read answers
///   the stored entries as points) removes it.
/// - **bracket**: a frame built at `Interval` from bracket inputs, a
///   rigid-motion claim (fork 274). Squaring the brackets carries the
///   squaring's width, which main's raw, unsquared frame did not.
///   Mutant **F1X** (that squaring, at an `Exact` kind off the
///   stated-point path, stores the stated vectors raw) removes it.
/// - **f64 squaring**: a frame squared at `f64` from stated directions,
///   the same claim, where main stored them raw: a rounding move of the
///   frame, read across a band's edge. Mutant **F1F** (squaring at an
///   `Inexact` kind stores the stated vectors raw) removes it.
/// - **both, jointly**: polar and bracket, each alone under the band, so
///   A alone and F1X alone each remove it.
///
/// The mutants are a patch in PR 7131's body; each row names its cause
/// and the mutants that turn its pin red.
#[must_use]
pub fn at_the_frame_cost_band(tol: Tol) -> bool {
    tol.eps() <= 1e-12
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

/// The number a told quantity was built from, where it is one value;
/// `NaN` for an enclosure with two ends and for an unreadable reading.
/// The rows that pin a refusal's payload against the scalar its door
/// computed read it here.
#[must_use]
pub fn told(told: &Told) -> f64 {
    match told.pinned_ends() {
        (lo, hi) if lo == hi => lo,
        _ => f64::NAN,
    }
}

/// Whether `offered`, a tolerance read off a refusal's sentence, is the
/// deciding tolerance `exact` (`|m|/K`) told as an offered bound: at
/// most it, and within the four-figure grid of it. A suite bounds an
/// offer by this rather than by the offer rule's own answer.
#[must_use]
pub fn is_told_offer_of(offered: f64, exact: f64) -> bool {
    offered <= exact && exact - offered <= OFFER_GRID_STEP * exact
}

/// The band at tolerance `eps` and the default `K`: the band an
/// executed-offer case's first raise runs at (`test_utils::offer::DESIGN_EPS`),
/// whatever the running process's own tolerance is. Panics where `eps`
/// forms no band.
#[must_use]
#[allow(clippy::panic)]
pub fn default_k_band(eps: f64) -> Band {
    match Band::new(eps, crate::tolerance::DEFAULT_K * eps) {
        Ok(band) => band,
        Err(e) => panic!("the default-K band at {eps:e}: {e:?}"),
    }
}

/// The widest relative step of the grid an offered bound is told on:
/// four significant figures.
const OFFER_GRID_STEP: f64 = 1e-3;

/// Whether `offered`, a tolerance read off a refusal's sentence, is the
/// offer `band` makes for a margin known only to within `slack` of
/// `margin`, by two checks that answer apart:
///
/// - **against the margin itself**, as [`is_told_offer_of`] is: at most
///   `(|m| + slack)/K`, and no more than the grid's step below
///   `(|m| − slack)/K`. A rule that rounds up, drops a figure or
///   misplaces a decade fails here whatever the sentence says.
/// - **against the rule's own answer** ([`Band::offer_deciding`]) at
///   the window's two ends: no lower than its offer at the low end and
///   no higher than its offer at the high end, so a sentence that does
///   not quote the rule fails here. A quote between two grid points
///   inside the window passes this half.
#[must_use]
pub fn is_offer_of_a_margin_within(band: Band, offered: f64, margin: f64, slack: f64) -> bool {
    let k = band.escalate() / band.zero();
    let (lo, hi) = (margin.abs() - slack, margin.abs() + slack);
    let independent =
        offered > 0.0 && offered <= hi / k && offered >= (1.0 - OFFER_GRID_STEP) * lo / k;
    let rule = band.offer_deciding(lo).is_none_or(|y| y <= offered)
        && band.offer_deciding(hi).is_some_and(|y| offered <= y);
    independent && rule
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod offer_tests {
    use super::{Band, is_offer_of_a_margin_within};

    /// **The offer check answers false on a quote the margin does not
    /// make**, and true on the one the rule makes. A margin of 5.5e-9 at
    /// the design band (ε 1e-9, K 10) offers 5.5e-10; one of 5.756955e-10
    /// offers 5.756e-11.
    #[test]
    fn the_offer_check_refuses_a_quote_the_margin_does_not_make() {
        let band = Band::new(1e-9, 1e-8).expect("the design band");
        let at = |v: f64, m: f64| is_offer_of_a_margin_within(band, v, m, 0.0);
        // The rule's own offer passes at every scale the suites raise at:
        // a quote equal to the rule's answer, so only the bound by |m|/K
        // can refuse it.
        for m in [
            3.0000000000031634e-10,
            9.999999999999951e-11,
            5.5e-9,
            1.234567e-7,
            4.2e-6,
        ] {
            let offered = band.offer_deciding(m).expect("an offer above the floor");
            assert!(at(offered, m), "the rule's offer {offered:e} for {m:e}");
        }
        assert!(at(5.5e-10, 5.5e-9), "the told offer of 5.5e-9");
        assert!(
            at(5.756e-11, 5.756955e-10),
            "the told offer of 5.756955e-10"
        );
        assert!(!at(5.757e-11, 5.756955e-10), "rounded up, above |m|/K");
        assert!(!at(5.75e-11, 5.756955e-10), "a figure dropped");
        assert!(!at(5.756e-12, 5.756955e-10), "a tenth");
        assert!(!at(5.756e-10, 5.756955e-10), "ten times");
        assert!(
            !at(5.7565e-11, 5.756955e-10),
            "off the grid, between two points"
        );
        // A margin known to a slack accepts either grid point it straddles.
        let straddle = 1.1e-12 * 3e-10;
        assert!(is_offer_of_a_margin_within(band, 3e-11, 3e-10, straddle));
        assert!(is_offer_of_a_margin_within(
            band, 2.999e-11, 3e-10, straddle
        ));
        assert!(
            !is_offer_of_a_margin_within(band, 2.998e-11, 3e-10, straddle),
            "a step below both"
        );
    }
}

/// The tolerance a refusal's sentence offers to tighten below, where it
/// ends in one: the number after "tighten the tolerance below ", in
/// metres.
#[must_use]
pub fn offered_tolerance(text: &str) -> Option<f64> {
    text.rsplit_once("tighten the tolerance below ")?
        .1
        .strip_suffix(" m")?
        .parse()
        .ok()
}

/// **A matrix a suite holds, admitted as the rigid motion it is** at
/// `tol`'s linear band: the question a suite that builds its map as
/// twelve numbers, at any scalar, asks before it moves a body by it. Its
/// columns are kept as given when each rigidity margin is within its own
/// rounding ([`RigidMotion::check_rigid`]'s rule) or, beyond it, levered
/// by the session box and decided `Zero` at the band. A test door: no
/// production door re-derives rigidity from entries at a box scalar.
/// Panics, naming the refusal, when the map is not admitted.
#[must_use]
#[allow(clippy::panic)]
pub fn rigid<T: Decide>(map: &Affine3<T>, tol: Tol) -> RigidMotion<T> {
    admitted(map, tol).unwrap_or_else(|e| panic!("the map is not admitted as a rigid motion: {e}"))
}

/// [`rigid`]'s answer, the refusal returned rather than panicked on: for
/// a suite asking which margin a map that is no rigid motion fails.
///
/// # Errors
///
/// The refusal, naming the margin.
#[allow(clippy::panic)]
pub fn admitted<T: Decide>(
    map: &Affine3<T>,
    tol: Tol,
) -> Result<RigidMotion<T>, crate::RigidMotionError> {
    let band =
        Band::linear(tol).unwrap_or_else(|e| panic!("the run's tolerance forms no band: {e}"));
    RigidMotion::admitted_for_a_suite(map, band)
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

pub use crate::predicate::fixture_lever;

/// **The least turn's conditioning edge at `arm`**: the chord from the
/// pole, `2·arm/R`, below which [`RigidMotion::least_turn`] refuses
/// ill-conditioned, `R` the run's rounding reach read through its one
/// door ([`Tol::rounding_reach`]). Near the pole the chord is the tilt
/// from backwards, so a row tilts by a fraction or a multiple of it. The
/// suites' one spelling of the edge.
#[must_use]
pub fn least_turn_band_edge(tol: Tol, arm: f64) -> f64 {
    2.0 * arm / tol.rounding_reach()
}

/// **The reading of a scale twin's margin streams**: one body at two
/// model scales, each run's [`MarginSample`](crate::k_stats::MarginSample)s
/// grouped by predicate in recording order, and the two read side by side.
/// A twin compares verdict streams sample for sample, and pairs only the
/// decisive margins that have a scale
/// ([`Twins::decisive_pairs`](twins::Twins::decisive_pairs)).
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

    /// How [`Twins::decisive_pairs`] pairs the twins' decisive margins.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Pairing {
        /// By rank, each list sorted: for a predicate that fires on
        /// several entities, where recording order pairs unrelated sites.
        Rank,
        /// By site, in recording order: for a fixture whose decisions are
        /// made in the same order at both scales, where a swap of two
        /// sites' margins is a divergence rank pairing would pass.
        Site,
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

        /// **The decisive margins with a scale, paired**: each twin's
        /// definite nonzero samples not decided on their sign alone
        /// ([`MarginSample::reads_sign_only`]), as `|margin|`, those
        /// `keep` admits, paired `(small, large)` as `pairing` says.
        ///
        /// # Panics
        ///
        /// When the twins' sign-only masks differ (a sample read on its
        /// sign in one twin and against the band in the other), or the
        /// two decisive lists differ in length: pairing them would drop
        /// the excess unread.
        #[must_use]
        pub fn decisive_pairs(
            &self,
            pairing: Pairing,
            keep: impl Fn(f64) -> bool,
        ) -> Vec<(f64, f64)> {
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
                if pairing == Pairing::Rank {
                    v.sort_by(f64::total_cmp);
                }
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
                twins.decisive_pairs(Pairing::Rank, |_| true),
                [(1e-3, 1.0), (2e-3, 2.0)],
                "sign-only and zero samples drop out, and the rest pair by rank"
            );
        }

        /// Site pairing catches two sites whose margins swapped between
        /// the twins, which rank pairing passes.
        #[test]
        fn site_pairing_keeps_recording_order() {
            let linear = (1e-9, 1e-6);
            let small = [sample(1e-3, linear, POS), sample(2e-3, linear, POS)];
            let large = [sample(2.0, linear, POS), sample(1.0, linear, POS)];
            let twins = Twins {
                predicate: "p",
                small: &small,
                large: &large,
            };
            assert_eq!(
                twins.decisive_pairs(Pairing::Site, |_| true),
                [(1e-3, 2.0), (2e-3, 1.0)],
                "by site"
            );
            assert_eq!(
                twins.decisive_pairs(Pairing::Rank, |_| true),
                [(1e-3, 1.0), (2e-3, 2.0)],
                "by rank"
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
            .decisive_pairs(Pairing::Rank, |_| true);
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
            .decisive_pairs(Pairing::Rank, |m| m < 1.5);
        }
    }
}

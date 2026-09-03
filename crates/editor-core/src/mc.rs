//! **The Monte-Carlo ADVISORY estimator lane** (ERROR-DESIGN E11.1;
//! the M10 plan's ruling Q3).
//!
//! Pure `f64` replay of the document over `N` parameter samples drawn
//! from its own distributions. Every number this module produces is
//! ADVISORY: it never gates, it is never persisted as an assertion, and
//! it never enters the mass accounting.
//!
//! # Why it exists at all, since intervals gate
//!
//! Because the certified answer covers the analyzed box and the box is
//! not the law. E2 keeps the excluded tail as an explicit additive
//! term, and the certified worst case says nothing about what is in it.
//! MC draws from the WHOLE distribution, tail included
//! ([`crate::analysis::sample_offset`]), so it estimates the quantity
//! the certified lane deliberately does not: what the measure does
//! where nothing is certified. E11's own words for the trade —
//! "certified intervals remain the ONLY gate; MC joins as a labeled
//! advisory estimator lane".
//!
//! # The label discipline, and where it is enforced
//!
//! E11.1 requires sample count and seed on every MC result. Here that
//! is structural rather than a convention: [`McReport`] carries both,
//! [`McReport::render`] puts them on the FIRST line and on every
//! estimate line, and no estimate is reachable without the report that
//! carries them — there is no door that hands out a mean.
//!
//! # Determinism (D9)
//!
//! The stream is `xorshift64*` — the same generator
//! `test-utils::fuzz::Rng` runs, re-stated here because that crate is
//! dev-only and this is production code, with the same zero-seed remap
//! for the same reason. Each SAMPLE gets its own stream, seeded by
//! `splitmix64(seed ⊕ index)`, so a sample's draw is a function of its
//! index and never of the order the samples ran in: the sequential and
//! the rayon schedules produce bit-identical reports, which is D9 idiom
//! 1's requirement and is what the suite's own row pins.
//!
//! # What it costs
//!
//! One full `f64` document evaluation per sample. That is the honest
//! price of "pure replay makes sampling trivial" (E11.1) and it is why
//! [`DEFAULT_SAMPLES`] is a few hundred rather than a few million: the
//! lane exists to put a labeled second opinion beside the certified
//! number, not to resolve a tail to six digits.

use std::sync::Arc;

use geom_core::Tol;

use crate::analysis::{AnalyzedBox, BoxAxis, MeasureUnavailable, ParamBox, sample_offset};
use crate::doc::{Doc, ParamName};
use crate::eval::{
    CancelToken, ContentKey, EvalOptions, Evaluation, NodeResult, ProfileLift, ValuePayload,
    evaluate,
};
use crate::measure::AssertionVerdict;
use crate::node::{Node, RecipeNodeId};
use crate::program::ProfileProgram;

/// The shipped sample count — a recorded run dial, not a constant of
/// nature.
///
/// 512 replays is about the point where the mean of a smooth measure
/// stops moving in its third digit while a run still costs a second or
/// two on a document of the corpus's size. A consumer who wants a tail
/// fraction resolved asks for more samples and pays linearly.
pub const DEFAULT_SAMPLES: usize = 512;

/// The shipped seed. RECORDED, never drawn from the clock: an advisory
/// number whose seed is not in the report is a number nobody can
/// reproduce, and E11.1 requires the seed to ride the result.
pub const DEFAULT_SEED: u64 = 0x4d43_5f45_3131_5f31;

/// How one MC run is configured.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McConfig {
    /// How many samples ([`DEFAULT_SAMPLES`]).
    pub samples: usize,
    /// The stream's seed ([`DEFAULT_SEED`]), recorded in the report.
    pub seed: u64,
    /// Whether samples run under rayon (D9 idiom 1). A RUNTIME switch
    /// in the driver's own mould, so one test run can compare the two
    /// schedules and pin that the report is the same bits.
    pub parallel: bool,
}

impl Default for McConfig {
    fn default() -> Self {
        Self {
            samples: DEFAULT_SAMPLES,
            seed: DEFAULT_SEED,
            parallel: true,
        }
    }
}

/// Why an MC run produced nothing.
#[derive(Debug, Clone, PartialEq)]
pub enum McRefusal {
    /// A varying parameter carries a band. Limits without a shape
    /// cannot be sampled, and the lane refuses the WHOLE run naming the
    /// parameter rather than sampling the rest: a mean over a subset of
    /// the parameters is an estimate of a different document.
    BandHasNoMeasure(MeasureUnavailable),
    /// The run asked for no samples. An estimator over zero draws has
    /// no estimate, and reporting `NaN` as a mean would be exactly the
    /// unlabeled number this lane exists to avoid.
    NoSamples,
    /// The document does not build at its nominal, so there is nothing
    /// to replay. The node and its rendered error, the driver's own
    /// shape.
    NominalDoesNotBuild {
        /// The refusing node.
        node: RecipeNodeId,
        /// Its error, rendered.
        cause: String,
    },
}

impl core::fmt::Display for McRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BandHasNoMeasure(e) => write!(
                f,
                "the Monte-Carlo lane cannot draw from a band: {e} — the certified lane \
                 answers over a band's limits, which is what a band is for"
            ),
            Self::NoSamples => {
                f.write_str("a Monte-Carlo run of zero samples has no estimate to report")
            }
            Self::NominalDoesNotBuild { node, cause } => write!(
                f,
                "the document does not build at its nominal (node {}), so there is nothing \
                 to replay: {cause}",
                node.0
            ),
        }
    }
}

impl core::error::Error for McRefusal {}

/// One measure node's empirical summary. ADVISORY — the label is on the
/// report, and this type is not reachable without it.
#[derive(Debug, Clone, PartialEq)]
pub struct McMeasure {
    /// The measure node.
    pub node: RecipeNodeId,
    /// The sample mean, over the samples where the measure had a value.
    pub mean: f64,
    /// The sample standard deviation (the `N − 1` form; `0.0` for one
    /// sample, which is the only value a single draw supports).
    pub sigma: f64,
    /// The least measured value.
    pub min: f64,
    /// The greatest.
    pub max: f64,
    /// How many samples produced a value.
    pub measured: usize,
    /// How many produced none — a refusing node, or a measure with no
    /// value at `f64` (`min_clearance`). Counted, never averaged over.
    pub unmeasured: usize,
}

/// One assertion node's empirical summary.
#[derive(Debug, Clone, PartialEq)]
pub struct McAssertion {
    /// The assertion node.
    pub node: RecipeNodeId,
    /// Samples whose verdict was `Holds`.
    pub holds: usize,
    /// Samples whose verdict was `Violated`.
    pub violated: usize,
    /// Samples with no verdict at all — `Unevaluated`, a poisoned
    /// assertion, or a refusing node. Kept OUT of the fraction below:
    /// an undecided sample is not a passing one.
    pub unevaluated: usize,
}

impl McAssertion {
    /// The empirical violation fraction over the DECIDED samples, or
    /// `None` when none were decided.
    ///
    /// The denominator is `holds + violated` and never the sample
    /// count, because an `Unevaluated` sample says the run could not
    /// decide — folding it into either side would be inventing the
    /// verdict E10's third state exists to withhold.
    pub fn violation_fraction(&self) -> Option<f64> {
        let decided = self.holds + self.violated;
        (decided > 0).then(|| self.violated as f64 / decided as f64)
    }
}

/// **The E11.1 advisory report.** Every number in it is an estimate,
/// and the count and seed that produced it ride at the top.
#[derive(Debug, Clone, PartialEq)]
pub struct McReport {
    /// How many samples were drawn.
    pub samples: usize,
    /// The seed they were drawn from.
    pub seed: u64,
    /// Per measure node, in node order.
    pub measures: Vec<McMeasure>,
    /// Per assertion node, in node order.
    pub assertions: Vec<McAssertion>,
    /// The fraction of samples that landed OUTSIDE the analyzed box —
    /// the empirical twin of E2's tail term.
    ///
    /// It is the one number here a reader can check against the
    /// certified side: the accounting's `unanalyzed` mass is the same
    /// quantity computed exactly, so the two converge as the sample
    /// count grows. That agreement is worth having precisely because
    /// the two are computed by completely different means.
    pub outside_box: f64,
}

impl McReport {
    /// The goldening form: exact bits, the reporting layer's idiom.
    ///
    /// The seed and the count are the FIRST line here too, so a golden
    /// that was re-blessed under a different dial is visibly different
    /// rather than subtly so.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(s, "mc samples={} seed={:016x}", self.samples, self.seed);
        for m in &self.measures {
            let _ = writeln!(
                s,
                "measure {} mean={:016x} sigma={:016x} min={:016x} max={:016x} measured={} \
                 unmeasured={}",
                m.node.0,
                m.mean.to_bits(),
                m.sigma.to_bits(),
                m.min.to_bits(),
                m.max.to_bits(),
                m.measured,
                m.unmeasured
            );
        }
        for a in &self.assertions {
            let _ = writeln!(
                s,
                "assertion {} holds={} violated={} unevaluated={}",
                a.node.0, a.holds, a.violated, a.unevaluated
            );
        }
        let _ = writeln!(s, "outside_box {:016x}", self.outside_box.to_bits());
        s
    }

    /// The content key of everything [`Self::serialize`] renders.
    pub fn content_key(&self) -> ContentKey {
        crate::report::key_of(0xE9, &self.serialize())
    }

    /// **The human form**, with the advisory label and the dials on
    /// every line that carries an estimate.
    ///
    /// Repeating "advisory (N samples, seed …)" on each line is
    /// deliberate: a reader who copies one line out of a report takes
    /// the label with it, which a single header line does not survive.
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let tag = format!(
            "ADVISORY — Monte-Carlo estimate over {} samples, seed {:#018x}",
            self.samples, self.seed
        );
        let mut s = String::new();
        let _ = writeln!(
            s,
            "{tag}. Never gates, never persisted as an assertion, never in the accounting."
        );
        for m in &self.measures {
            let _ = writeln!(
                s,
                "  node {}: mean {} σ {} min {} max {}   [{tag}]",
                m.node.0, m.mean, m.sigma, m.min, m.max
            );
            if m.unmeasured > 0 {
                let _ = writeln!(
                    s,
                    "    {} of {} samples had no measured value at f64 and are not averaged over",
                    m.unmeasured, self.samples
                );
            }
        }
        for a in &self.assertions {
            let _ = writeln!(
                s,
                "  node {}: empirical violation fraction {}   [{tag}]",
                a.node.0,
                match a.violation_fraction() {
                    Some(f) => format!(
                        "{:.4}% ({} of {} decided)",
                        f * 100.0,
                        a.violated,
                        a.holds + a.violated
                    ),
                    None => "[no sample decided this assertion]".to_owned(),
                }
            );
            if a.unevaluated > 0 {
                let _ = writeln!(
                    s,
                    "    {} of {} samples had no verdict and are outside the fraction",
                    a.unevaluated, self.samples
                );
            }
        }
        let _ = writeln!(
            s,
            "  {:.4}% of samples fell outside the analyzed box — the empirical twin of the \
             accounting's tail   [{tag}]",
            self.outside_box * 100.0
        );
        s
    }
}

/// **The MC lane's one door**: replay the document at `f64` over
/// `config.samples` draws from its own distributions, and summarize.
///
/// # Errors
///
/// [`McRefusal`]: a band (unsampleable by construction), a zero-sample
/// request, or a document that does not build at its nominal.
pub fn monte_carlo(
    doc: &Doc<ProfileProgram>,
    analyzed: &AnalyzedBox,
    config: &McConfig,
    tol: Tol,
) -> Result<McReport, McRefusal> {
    if config.samples == 0 {
        return Err(McRefusal::NoSamples);
    }
    // The nominal build, first: a document that does not build has no
    // replay, and finding that out once beats finding it out `samples`
    // times.
    let nominal: Evaluation<f64> = evaluate(doc, None, &CancelToken::new(), &lane_opts(), tol);
    if let Some(&node) = nominal
        .order
        .iter()
        .find(|id| !matches!(nominal.nodes.get(id), Some(NodeResult::Ok(_))))
    {
        let cause = nominal
            .node_error(node)
            .map_or_else(|| "not evaluated".to_owned(), |e| e.kind.to_string());
        return Err(McRefusal::NominalDoesNotBuild { node, cause });
    }

    // Every varying parameter must be sampleable BEFORE any sampling
    // happens: a band refuses the whole run, and refusing it here
    // rather than at the first draw keeps the refusal a property of the
    // document instead of a property of which parameter came first.
    let laws: Vec<(ParamName, crate::distribution::Distribution)> = analyzed
        .varying()
        .map(|(name, p)| {
            p.distribution.map(|d| (name.clone(), d)).ok_or_else(|| {
                MeasureUnavailable::BandHasNoMeasure {
                    param: name.clone(),
                }
            })
        })
        .collect::<Result<_, _>>()
        .map_err(McRefusal::BandHasNoMeasure)?;
    for (name, dist) in &laws {
        sample_offset(name, dist, 0.5).map_err(McRefusal::BandHasNoMeasure)?;
    }

    // The sinks, in the document's own node order — which is the order
    // every derived list in this kernel takes, so two runs report their
    // rows in one order and a golden over the report is stable.
    let sinks: Vec<(RecipeNodeId, bool)> = doc
        .order()
        .iter()
        .filter_map(|&id| match doc.node(id) {
            Some(Node::Measure { .. }) => Some((id, true)),
            Some(Node::Assertion { .. }) => Some((id, false)),
            _ => None,
        })
        .collect();

    let one = |index: usize| -> Sample {
        let mut rng = Rng::for_sample(config.seed, index);
        let mut axes = std::collections::BTreeMap::new();
        let mut outside = false;
        for (name, dist) in &laws {
            // `sample_offset` was proved total for these laws above, so
            // a refusal here is a kernel bug rather than a document
            // fault — and it is announced as one rather than silently
            // sampling the nominal.
            let Ok(offset) = sample_offset(name, dist, rng.unit()) else {
                unreachable!(
                    "every law was proved sampleable before the run, yet {} refused",
                    name.0
                )
            };
            if let Some(p) = analyzed.get(name)
                && (offset < p.offsets.lo || offset > p.offsets.hi)
            {
                outside = true;
            }
            axes.insert(
                name.clone(),
                BoxAxis::Varying {
                    lo: offset,
                    hi: offset,
                },
            );
        }
        // A DEGENERATE box is how a point sample reaches the evaluation
        // service: `AxisScalar for f64` admits an axis whose two ends
        // are bit-equal and refuses every other, so this is the one
        // door a point-scalar replay over a parameter value has, and
        // the MC lane uses it rather than a second binding path.
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::from_axes(axes))),
            ..lane_opts()
        };
        let ev: Evaluation<f64> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
        let readings = sinks
            .iter()
            .map(|&(id, is_measure)| {
                if is_measure {
                    match ev.result(id) {
                        Some(NodeResult::Ok(v)) => match &v.payload {
                            ValuePayload::Measure { value, .. } => Reading::Value(*value),
                            _ => Reading::NoValue,
                        },
                        _ => Reading::NoValue,
                    }
                } else {
                    match ev.result(id) {
                        Some(NodeResult::Ok(v)) => match &v.payload {
                            ValuePayload::Assertion(AssertionVerdict::Holds { .. }) => {
                                Reading::Holds
                            }
                            ValuePayload::Assertion(AssertionVerdict::Violated { .. }) => {
                                Reading::Violated
                            }
                            _ => Reading::NoValue,
                        },
                        _ => Reading::NoValue,
                    }
                }
            })
            .collect();
        Sample { readings, outside }
    };

    // D9 idiom 1: an INDEXED map, then one sequential fold — so the two
    // schedules see the same samples in the same order and the report
    // is the same bits either way.
    let samples: Vec<Sample> = if config.parallel {
        use rayon::prelude::*;
        (0..config.samples).into_par_iter().map(one).collect()
    } else {
        (0..config.samples).map(one).collect()
    };

    let mut measures = Vec::new();
    let mut assertions = Vec::new();
    for (slot, &(node, is_measure)) in sinks.iter().enumerate() {
        if is_measure {
            let values: Vec<f64> = samples
                .iter()
                .filter_map(|s| match s.readings.get(slot) {
                    Some(Reading::Value(v)) => Some(*v),
                    _ => None,
                })
                .collect();
            let measured = values.len();
            let unmeasured = samples.len() - measured;
            let (mean, sigma, min, max) = summarize(&values);
            measures.push(McMeasure {
                node,
                mean,
                sigma,
                min,
                max,
                measured,
                unmeasured,
            });
        } else {
            let mut row = McAssertion {
                node,
                holds: 0,
                violated: 0,
                unevaluated: 0,
            };
            for s in &samples {
                match s.readings.get(slot) {
                    Some(Reading::Holds) => row.holds += 1,
                    Some(Reading::Violated) => row.violated += 1,
                    _ => row.unevaluated += 1,
                }
            }
            assertions.push(row);
        }
    }
    let outside = samples.iter().filter(|s| s.outside).count();
    Ok(McReport {
        samples: samples.len(),
        seed: config.seed,
        measures,
        assertions,
        outside_box: outside as f64 / samples.len() as f64,
    })
}

/// The replay's evaluation options: the guided profile lift, so a
/// sampled profile dimension reaches the geometry the same way the
/// driver's leaves do (M10-P), and nothing else.
fn lane_opts() -> EvalOptions {
    EvalOptions {
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    }
}

/// What one sample read at each sink, in `sinks` order.
struct Sample {
    readings: Vec<Reading>,
    outside: bool,
}

/// One sink's reading in one sample.
enum Reading {
    /// A measure's value.
    Value(f64),
    /// An assertion that held.
    Holds,
    /// An assertion that was violated.
    Violated,
    /// No value and no verdict: a refusing node, a poisoned one, or
    /// E10's third state. Never folded into either side.
    NoValue,
}

/// Mean, standard deviation (`N − 1`), min and max of a sample.
///
/// The two-pass form rather than the sum-of-squares shortcut: the
/// shortcut cancels catastrophically when the mean dominates the
/// spread, which is exactly the regime a tolerance study lives in (a
/// 0.6 m web with a 0.02 mm spread).
fn summarize(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    }
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let sigma = if values.len() > 1 {
        (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt()
    } else {
        0.0
    };
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, sigma, min, max)
}

/// `xorshift64*`, one stream per sample.
///
/// Re-stated here rather than shared with `test-utils::fuzz::Rng`
/// because that crate is a DEV dependency and this is production code;
/// the constants and the zero remap are that generator's, so a reader
/// comparing the two finds the same stream rather than a second one.
struct Rng(u64);

impl Rng {
    /// The stream for one sample index: `splitmix64(seed ⊕ index)`.
    ///
    /// Per-sample rather than one shared stream, and that is what makes
    /// the rayon schedule irrelevant: sample `i` draws the same numbers
    /// whether it ran first, last, or on another thread.
    fn for_sample(seed: u64, index: usize) -> Self {
        let mut z = seed ^ (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        // xorshift64* is absorbing at zero, so a zero state would make
        // one sample a constant stream.
        Self(if z == 0 { 0x9e37_79b9_7f4a_7c15 } else { z })
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// Uniform in `[0, 1)` from the top 53 bits.
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / 9_007_199_254_740_992.0)
    }
}

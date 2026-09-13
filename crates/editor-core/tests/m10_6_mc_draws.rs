//! **The MC lane's draws, handed out** — `mc::sample_offsets`, and the
//! two things a consumer needs it to be true of.
//!
//! Filed as `work/m10`'s
//! `mc-lanes-draws-are-not-reproducible-from-outside-the-crate`: the
//! report says what the mean and the spread are, and before this door
//! there was no way to ask what was DRAWN. A consumer that wants to
//! look at a sample — evaluate the document there, tessellate it, draw
//! it — had to re-transcribe `xorshift64*` and its `[0, 1)` reduction
//! outside this crate, which is the third copy of that arithmetic that
//! `mc::sample_stream`'s own doc argues against.
//!
//! Two rows, and they are different claims:
//!
//! * **the door draws what the lane draws** — the population
//!   `sample_offsets` enumerates summarizes to the SAME four numbers
//!   `monte_carlo` reports, bit for bit;
//! * **`nominal + offset` is where the lane put the sample** — which
//!   is the claim a consumer needs in order to place a draw through an
//!   ordinary `SetDocParamValue` edit rather than through a
//!   `ParamBox`, since that box is the interval driver's type and the
//!   advisory lane is meant to be reachable without it.
//!
//! UNGATED on purpose. Every other MC row in this tree lives in a
//! `*_interval.rs` file, which is itself part of what the issue
//! records: a lane advertised as reachable in a default build was only
//! ever exercised in a build that had the feature.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::mc::{McConfig, McRefusal, monte_carlo, sample_offsets};
use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, Expr, MeasureExpr, Node, ParamName, ProfileDoc,
    RecipeNodeId, UnitSym, apply,
};

/// The nominal, and a number with no dyadic shortcuts in it: a mean
/// that agreed only because every draw rounded the same way would be
/// no evidence.
const NOMINAL: f64 = 1.234_567_89;
/// Sample count for these rows. Smaller than the shipped default
/// because what is being compared is an identity, not a convergence.
const SAMPLES: usize = 96;

/// The document: one varying parameter, and a measure that reads it.
///
/// Deliberately the SIMPLEST document that has both — the measured
/// value is the parameter's own value, so a disagreement between the
/// two paths cannot be blamed on geometry standing between the draw
/// and the reading.
fn doc_with_one_law(law: Distribution) -> (ProfileDoc, RecipeNodeId) {
    let tol = Tol::witness();
    let mut doc = ProfileDoc::empty_derived("m10-mc-draws", tol);
    let applied = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("x"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: NOMINAL,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(law),
            },
        },
        tol,
    )
    .expect("the parameter declares");
    doc = applied.doc;

    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::value(Expr::param(ParamName::new("x"), Dimension::Length)),
                Vec::new(),
            )
            .expect("a measure over a value leaf takes no references"),
        },
        tol,
    )
    .expect("the measure inserts");
    doc = applied.doc;
    let measure = applied.record.minted.expect("an insert mints an id");
    (doc, measure)
}

/// `summarize`'s arithmetic, in the order the lane runs it. Re-stated
/// rather than shared because the point of the row is that an OUTSIDE
/// consumer reaches the same four numbers; borrowing the private
/// helper would assume away half of what is being asked.
fn summarize(values: &[f64]) -> (f64, f64, f64, f64) {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let sigma = (values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt();
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, sigma, min, max)
}

/// **The door draws what the lane draws, and the lane puts the sample
/// at `nominal + offset`.**
///
/// Both halves at once, because one comparison decides both: the
/// values are built OUTSIDE the lane, as `NOMINAL + offset` over
/// `sample_offsets`, and they are held to the lane's own four
/// summaries bit for bit. If the stream had forked, the numbers would
/// differ; if the lane placed a sample anywhere but at
/// `nominal + offset`, they would differ too.
#[test]
fn sample_offsets_enumerates_the_population_monte_carlo_summarizes() {
    let tol = Tol::witness();
    let (doc, measure) = doc_with_one_law(Distribution::Normal { sigma: 0.01 });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let config = McConfig {
        samples: SAMPLES,
        parallel: false,
        ..McConfig::default()
    };
    let report = monte_carlo(&doc, &analyzed, &config, tol).expect("the nominal builds");
    let row = report
        .measures
        .iter()
        .find(|m| m.node == measure)
        .expect("the document's one measure has a row");
    assert_eq!(row.measured, SAMPLES, "every sample measured");

    let values: Vec<f64> = (0..SAMPLES)
        .map(|i| {
            let offsets =
                sample_offsets(&analyzed, &config, i).expect("a normal law is sampleable");
            assert_eq!(offsets.len(), 1, "one varying parameter, one offset");
            NOMINAL + offsets[&ParamName::new("x")]
        })
        .collect();
    let (mean, sigma, min, max) = summarize(&values);

    assert_eq!(mean.to_bits(), row.mean.to_bits(), "the mean, bit for bit");
    assert_eq!(sigma.to_bits(), row.sigma.to_bits(), "the spread");
    assert_eq!(min.to_bits(), row.min.to_bits(), "the smallest draw");
    assert_eq!(max.to_bits(), row.max.to_bits(), "the largest draw");
}

/// **A sample's draw is a function of its index, not of what ran
/// before it** — the D9 property the module header states, now
/// asserted from outside, because it is what licenses asking for
/// sample 400 without asking for the 400 before it.
#[test]
fn a_samples_draw_depends_on_its_index_alone() {
    let (doc, _) = doc_with_one_law(Distribution::Uniform { lo: -0.5, hi: 0.5 });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let config = McConfig::default();
    let x = ParamName::new("x");

    let ascending: Vec<f64> = (0..8)
        .map(|i| sample_offsets(&analyzed, &config, i).expect("sampleable")[&x])
        .collect();
    let descending: Vec<f64> = (0..8)
        .rev()
        .map(|i| sample_offsets(&analyzed, &config, i).expect("sampleable")[&x])
        .collect();
    let mut descending = descending;
    descending.reverse();
    for (i, (a, b)) in ascending.iter().zip(&descending).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "sample {i} drew the same value");
    }
    // …and they are not all one number, so the row above is comparing
    // something. A constant stream would satisfy every equality here.
    assert!(
        ascending.windows(2).any(|w| w[0] != w[1]),
        "the draws vary: {ascending:?}"
    );
}

/// **A band still refuses, and it refuses HERE too.** The door is a
/// second entrance to the same population, so a document the whole run
/// would refuse cannot be sampled one draw at a time through it.
#[test]
fn a_band_refuses_at_the_draw_door_as_it_does_at_the_run() {
    let (doc, _) = doc_with_one_law(Distribution::Band {
        lo: -0.05,
        hi: 0.05,
    });
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let config = McConfig::default();
    assert!(
        matches!(
            sample_offsets(&analyzed, &config, 0),
            Err(McRefusal::BandHasNoMeasure(_))
        ),
        "a band has no shape to draw from, at either door"
    );
    assert!(
        matches!(
            monte_carlo(&doc, &analyzed, &config, Tol::witness()),
            Err(McRefusal::BandHasNoMeasure(_))
        ),
        "and the whole run says the same thing"
    );
}

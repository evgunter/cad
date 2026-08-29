//! **Driver-path predicate samples in the K funnel** (E6's T6
//! obligation).
//!
//! The mechanism is the one that already exists and nothing else: the
//! `k_stats` classification funnel, the `Probe` recording scalar, and
//! the thread-local sink `start_recording`/`take_samples` install. What
//! this unit adds is a config dial —
//! [`DriveConfig::k_probe`](editor_core::drive::DriveConfig::k_probe) —
//! that decides WHICH parameter points get sampled: with
//! `KProbe::CertifiedMidpoints`, every leaf the driver certified is
//! replayed at `Probe` over the degenerate box at its own midpoint.
//!
//! Why those points. A certified leaf is one the driver refined until
//! every predicate in it came out definite, so its midpoint is a
//! concrete parameter value at which the kernel's margins are as small
//! as refinement drove them — the "margins driven toward zero by
//! refinement" population E6 names, and the one K has never seen,
//! because every K corpus row to date is an author-chosen nominal.
//!
//! # What a K-REPORT re-examination run would execute
//!
//! ```sh
//! CAD_TOLERANCE_EPS=1e-9 CAD_K_REPORT_OUT=/tmp/driver-eps-1e-9.csv \
//!   cargo test -p editor-core --features probe,interval --test all -- \
//!   m10_3_driver_k_probe_interval:: --ignored --nocapture
//! ```
//!
//! It writes the M2 file convention — `shape,predicate,margin,
//! band_zero,band_escalate,outcome` — with shapes namespaced
//! `driver/<fixture>`, so a merged CSV stays attributable beside
//! `corpus/<doc>` and `demo/<scene>`. `scripts/k_probe_sweep.sh` runs
//! it into `<outdir>/driver/`, BESIDE the linted CSV rather than
//! inside it: what `k-lint` gates is a distribution whose thresholds
//! were argued over the Band 4 corpus and the tour scenes, and folding
//! a new population into it would move the gate's subject matter, which
//! is a K conversation and not a coverage one. The funnel row is the
//! deliverable here; the K verdict is not.
//!
//! Both features are needed, and that is inherent: `Probe` is the
//! `probe` feature's scalar and the driver is the `interval` feature's
//! service. Neither hosted lane builds that pair today, so this file's
//! standing row runs locally and under `local-scripts/ci-local.sh`.
#![cfg(all(feature = "probe", feature = "interval"))]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::io::Write;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, KProbe, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram,
};
use geom_core::k_stats::{self, MarginSample, SampleOutcome};
use geom_core::{Sign, Tol};
use profile::SketchPlane;

use fixture::Recorder;

/// The fixture: a square extruded by a document-parameter depth, over a
/// box narrow enough that the driver certifies most of it. Deliberately
/// the SAME shape the driver suite drives, so the funnel population and
/// the certification population are the same population.
fn slab(nominal: f64, half: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("depth"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: nominal,
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                .expect("finite square corners"),
        ],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: editor_core::Expr::param(ParamName::new("depth"), Dimension::Length),
    });
    r.doc
}

fn documents() -> Vec<(&'static str, ProfileDoc)> {
    let eps = Tol::witness().eps();
    vec![
        ("slab_narrow", slab(1.0, eps / 16.0)),
        ("slab_across_zero", slab(20.0 * eps, 40.0 * eps)),
    ]
}

fn probing() -> DriveConfig {
    DriveConfig {
        max_leaves: 256,
        k_probe: KProbe::CertifiedMidpoints,
        ..DriveConfig::default()
    }
}

/// One document's driver sweep: the samples the funnel received while
/// the driver replayed its certified leaves' midpoints.
fn run_doc(doc: &ProfileDoc) -> Vec<MarginSample> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    k_stats::start_recording();
    let v =
        drive(doc, &analyzed, &probing(), Tol::witness()).expect("the fixture's nominal builds");
    let samples = k_stats::take_samples();
    assert!(
        !v.certified().is_empty(),
        "nothing certified, nothing to sample"
    );
    samples
}

/// **The standing row**: the funnel actually receives driver-path
/// samples, and only when the dial asks for them.
///
/// A type-check cannot see this — the dial could be read and ignored —
/// so the claim is made by counting what arrived in the sink.
#[test]
fn the_dial_puts_driver_path_margins_in_the_funnel_and_nothing_else_does() {
    let (_, doc) = documents().remove(0);
    let with = run_doc(&doc);
    assert!(
        !with.is_empty(),
        "the certified-midpoint replay recorded no margins"
    );
    // Named predicates, real bands: these are the kernel's own
    // decisions, not a parallel stream this unit invented.
    assert!(with.iter().all(|s| !s.predicate.is_empty()));
    assert!(with.iter().any(|s| s.band_zero > 0.0));

    // ONE DIAL VARIES. The `with` run used `probing()`, so the `off`
    // run must be `probing()` with the probe turned off and nothing
    // else changed — comparing against `DriveConfig::default()` would
    // have moved `max_leaves` too, and "fewer samples" would then be
    // explicable by a smaller drive.
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    k_stats::start_recording();
    let _ = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            k_probe: KProbe::Off,
            ..probing()
        },
        Tol::witness(),
    )
    .unwrap();
    let without = k_stats::take_samples();
    // The SHARP claim, not "fewer": the driver evaluates at `Interval`,
    // and `Probe` is the only scalar that records. With the dial off
    // nothing in a drive can reach the sink at all, so the count is
    // exactly zero — a weaker `<` would pass on a drive that merely
    // recorded less.
    assert!(
        without.is_empty(),
        "the dial off recorded {} samples; it must record none",
        without.len()
    );
}

fn outcome_str(o: SampleOutcome) -> &'static str {
    match o {
        SampleOutcome::Definite(Sign::Negative) => "negative",
        SampleOutcome::Definite(Sign::Zero) => "zero",
        SampleOutcome::Definite(Sign::Positive) => "positive",
        SampleOutcome::Indeterminate => "indeterminate",
        SampleOutcome::Invalid => "invalid",
    }
}

/// The K-REPORT dump: every driver-path sample as CSV, in the M2 file
/// convention, shapes namespaced `driver/<fixture>`.
#[test]
#[ignore = "the K sweep's dump run; drives every fixture and writes a CSV"]
fn k_report_driver_dump() {
    let mut out: Box<dyn Write> = match std::env::var("CAD_K_REPORT_OUT") {
        Ok(path) => Box::new(std::fs::File::create(path).expect("the dump path is writable")),
        Err(_) => Box::new(std::io::stdout()),
    };
    writeln!(
        out,
        "shape,predicate,margin,band_zero,band_escalate,outcome"
    )
    .expect("the header writes");
    for (name, doc) in documents() {
        for s in run_doc(&doc) {
            writeln!(
                out,
                "driver/{name},{},{:e},{:e},{:e},{}",
                s.predicate,
                s.margin,
                s.band_zero,
                s.band_escalate,
                outcome_str(s.outcome)
            )
            .expect("a row writes");
        }
    }
}

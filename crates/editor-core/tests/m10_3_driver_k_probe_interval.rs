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
//! `driver/<fixture>`, one `# census …` comment line per fixture ahead
//! of that fixture's rows (see [`Population`]: a drive that certified
//! nothing has an empty population, and the census is what says so
//! instead of leaving a reader an empty file to interpret), so a
//! merged CSV stays attributable beside
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
//! service. The k-lint gate's probe-gated build row DOES build this
//! pair on every hosted run (`--features probe,interval --no-run`), so
//! a compile break here reds every PR — the row below also runs
//! locally and under `local-scripts/ci-local.sh`.
#![cfg(all(feature = "probe", feature = "interval"))]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::io::Write;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, KProbe, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, DocParam, LoopProgram, Node, ParamName, ProfileDoc,
    ProfileProgram, UnitSym,
};
use geom_core::k_stats::{self, MarginSample, SampleOutcome};
use geom_core::{Sign, Tol};

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
            display_unit: UnitSym::canonical_for(Dimension::Length),
            value: nominal,
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
    let xy_frame_0 = r.insert(Node::Datum(editor_core::Datum::Frame {
        origin: [0.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Length).unwrap()),
        u: [1.0, 0.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
        v: [0.0, 1.0, 0.0]
            .map(|v| editor_core::Expr::literal(v, editor_core::Dimension::Scalar).unwrap()),
    }));
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: xy_frame_0,
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

/// THE LEAF BUDGET IS SIZED BY THE HARDER FIXTURE, not by the easier
/// one. `slab_narrow` certifies its whole box in a single leaf and
/// would be happy at any budget; `slab_across_zero` straddles the
/// extrude's sign flip, and every box down to depth 7 is still
/// indeterminate there — so at `max_leaves: 256` the driver splits all
/// 255 of them, hits the frontier bound, and refuses the entire
/// 256-leaf frontier `Budget(Leaves)` having certified NOTHING. That
/// is not an eps effect: it is the same at 1e-6, 1e-9 and 1e-12, and
/// it is what took the `dev-probe` k-lint row red (#1343).
///
/// `4096` is the number the driver suite already uses on this same
/// fixture for its own "the witness side must certify" row, and it is
/// what the fixture measurably needs: 0 certified at 256, 344 at 4096.
/// It is a real cost, measured: the dump run goes from about a second
/// to about twenty, so about a minute over the sweep's three eps rows.
/// That cost buys the population this unit exists to sample, so it is
/// paid here rather than shrunk away by moving the fixture off the
/// flip.
fn probing() -> DriveConfig {
    DriveConfig {
        max_leaves: 4096,
        k_probe: KProbe::CertifiedMidpoints,
        ..DriveConfig::default()
    }
}

/// One document's driver sweep: what the drive certified, and the
/// samples the funnel received while it replayed those leaves'
/// midpoints.
///
/// **AN EMPTY CERTIFIED SET IS AN OUTCOME, NOT A FAULT** (issues 1296,
/// 1304, 1342 — one defect reported from three lanes). A drive that
/// refuses every leaf has certified nothing, so the certified-midpoint
/// replay has nothing to sample and this population is legitimately
/// empty. The row used to `assert!` over that state, and because
/// `scripts/k_probe_sweep.sh` runs its three ε rows in one loop, the
/// panic took the whole sweep down with it — so no branch could
/// produce a k-lint verdict at ANY ε (1304), and the failure printed
/// the ε row it happened to die on, which read as a tolerance defect
/// it was not.
///
/// **What actually emptied it, measured**: the leaf budget, not the
/// tolerance. These fixtures are ε-relative, so what they certify does
/// not move with ε — 1 and 344 leaves at every one of 1e-2, 1e-6,
/// 1e-9 and 1e-12 (measured 2026-09-03) — and the drive that certified
/// nothing was the `max_leaves: 256` one recorded at [`probing`]
/// (#1343, since raised to 4096). The panic is gone anyway: a budget
/// is a run dial, a report is not the place to die over one, and the
/// row below plants the empty population deliberately to prove it.
///
/// So the count rides BESIDE the samples, every caller says what it
/// found, and the claim the row makes is the biconditional below
/// rather than a floor that only holds when the dials are generous.
struct Population {
    /// How many leaves the drive certified — the population's size at
    /// its source, and the only thing that can explain an empty
    /// sample set.
    certified: usize,
    /// Every margin the funnel received during the certified-midpoint
    /// replay.
    samples: Vec<MarginSample>,
}

impl Population {
    /// The census line: what this fixture's drive certified and how
    /// many margins came out of it, at the ε the run was given.
    ///
    /// Written into the dump AND onto the terminal, because the two
    /// readers are different: the CSV's reader is whoever re-examines
    /// K months from now and needs to know an empty file was an empty
    /// POPULATION, and the terminal's is whoever is watching the sweep
    /// wonder why a row is quiet.
    fn census(&self, shape: &str) -> String {
        format!(
            "# census driver/{shape} eps={:e} certified={} samples={}",
            Tol::witness().eps(),
            self.certified,
            self.samples.len()
        )
    }
}

fn run_doc(doc: &ProfileDoc) -> Population {
    run_doc_with(doc, &probing())
}

/// The same drive at a caller's config — the seam the empty-population
/// row below needs, so that row can plant an empty certified set with
/// the ONE dial that produces one rather than by hunting for a
/// tolerance where the ε-relative fixtures stop certifying (they do
/// not: the fixtures scale with ε, which is why the original defect
/// was a budget and not a tolerance).
fn run_doc_with(doc: &ProfileDoc, config: &DriveConfig) -> Population {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    k_stats::start_recording();
    let v = drive(doc, &analyzed, config, Tol::witness()).expect("the fixture's nominal builds");
    Population {
        samples: k_stats::take_samples(),
        certified: v.certified().len(),
    }
}

/// **The standing row**: the funnel actually receives driver-path
/// samples, and only when the dial asks for them.
///
/// A type-check cannot see this — the dial could be read and ignored —
/// so the claim is made by counting what arrived in the sink.
#[test]
fn the_dial_puts_driver_path_margins_in_the_funnel_and_nothing_else_does() {
    let (name, doc) = documents().remove(0);
    let with = run_doc(&doc);
    // The census, on the terminal, at every run — including the run
    // where the population is empty, which is the one a reader would
    // otherwise have to guess at.
    eprintln!("{}", with.census(name));
    // **THE BICONDITIONAL, not a floor.** The dial samples certified
    // leaves and nothing else, so "the drive certified something" and
    // "the funnel received something" are one fact seen twice, and
    // asserting the equivalence keeps the claim SHARP over a drive that
    // certified nothing instead of either going vacuous or reporting a
    // run dial as a defect. A floor (`!samples.is_empty()`) is the
    // version that holds only while the dials are generous, and it is
    // the shape that reddened the sweep.
    assert_eq!(
        with.certified > 0,
        !with.samples.is_empty(),
        "{}: the drive certified {} leaves and the funnel received {} margins — the \
         certified-midpoint replay is the only path into the sink, so those two are the \
         same fact and cannot disagree",
        with.census(name),
        with.certified,
        with.samples.len()
    );
    // Named predicates, real bands: these are the kernel's own
    // decisions, not a parallel stream this unit invented. Both are
    // total over an empty population — the `any` rides the
    // biconditional above rather than restating the floor it replaced.
    assert!(with.samples.iter().all(|s| !s.predicate.is_empty()));
    assert!(
        with.samples.is_empty() || with.samples.iter().any(|s| s.band_zero > 0.0),
        "a non-empty driver population carries at least one real band"
    );

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

/// **The empty population is REPORTED, not panicked over** (issues
/// 1296, 1304, 1342 — one defect seen from three lanes).
///
/// The plant is the leaf budget, because that is the dial that
/// measurably produces an empty certified set on this fixture: at
/// `max_leaves: 256` the flip-straddling slab splits all 255 interior
/// boxes, hits the frontier bound, and refuses the whole frontier
/// `Budget(Leaves)` having certified nothing (the number recorded at
/// [`probing`]). That is exactly the state the row used to die in, and
/// dying took the sweep's whole ε loop with it.
///
/// What it asserts is the biconditional's other half: no certified
/// leaves, no samples, no panic, and a census line that says which.
#[test]
fn an_empty_certified_set_is_reported_rather_than_panicked_over() {
    let (name, doc) = documents().remove(1);
    let starved = run_doc_with(
        &doc,
        &DriveConfig {
            max_leaves: 256,
            ..probing()
        },
    );
    eprintln!("{}", starved.census(name));
    assert_eq!(
        starved.certified,
        0,
        "{}: the plant is a budget too small to reach a certificate; a drive that \
         certified something here is not exercising the empty population",
        starved.census(name)
    );
    assert!(
        starved.samples.is_empty(),
        "{}: nothing certified, so the certified-midpoint replay had nothing to sample",
        starved.census(name)
    );
    assert!(starved.census(name).contains("certified=0 samples=0"));
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
/// convention, shapes namespaced `driver/<fixture>`, each fixture's
/// rows preceded by its [`Population`] census line.
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
        let population = run_doc(&doc);
        // THE CENSUS FIRST, and unconditionally. A fixture that
        // certified nothing writes this line and no rows, which is the
        // honest shape of an empty population: the file says which
        // fixture was driven, at what ε, and that it yielded nothing —
        // where a bare header would leave a reader unable to tell an
        // empty population from a harness that died before writing.
        // It is a `#` comment line so every row in the file stays the
        // M2 convention.
        let census = population.census(name);
        writeln!(out, "{census}").expect("the census line writes");
        eprintln!("{census}");
        for s in population.samples {
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

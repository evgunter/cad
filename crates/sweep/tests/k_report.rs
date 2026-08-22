//! The K-report data harness (M2 PR 7): builds the M2 acceptance
//! shapes end-to-end at the recording scalar `Probe` (profile
//! validation → extrude/revolve → tier 1–3 validation → mass
//! properties) and dumps every recorded `MarginSample` as CSV.
//!
//! One process per ε (the `Tolerance` is a OnceLock):
//!
//! ```sh
//! CAD_TOLERANCE_EPS=1e-9 CAD_K_REPORT_OUT=docs/k-report-data/eps-1e-9.csv \
//!   cargo test -p sweep --features probe --test all \
//!     -- --ignored --nocapture k_report::
//! ```
//!
//! `sweep` sets `autotests = false` and aggregates every suite into one
//! `tests/all.rs`, so there is no `--test k_report` target; the suite is
//! selected by module prefix. `--features probe` is what compiles this
//! file — it is `#![cfg(feature = "probe")]`.
//!
//! **CI both type-checks and runs this harness, on 1 building merge
//! in 5.** It rides the `k-lint` job's `dev-probe` feature row, and
//! since 2026-08-22 that job SAMPLES one of its five feature
//! unifications per run. The draw is seeded from the head SHA under
//! its own salt, so a re-run of the same commit draws the same row and
//! "which unification gated this commit" is recoverable from the SHA
//! alone, without the run's logs. Repetition covers the matrix: at this
//! repository's measured run rate (~60 runs/hour during active work)
//! every row comes up within minutes.
//!
//! Sampling is sound HERE for one reason, and it is worth stating
//! because it does not generalise: this row is a PERSISTENCE detector.
//! A probe harness that stopped compiling, or stopped executing, stays
//! broken in the tree, so a later draw still finds it — a red is
//! deferred, never lost.
//!
//! **The ABSENCE half is not sampled and must not be confused with
//! this.** A suite that DISAPPEARS leaves no future red for a later
//! draw to catch; it merges silently, once. That is caught instead by
//! `probe-suite-census.sh`'s default mode against `CENSUS_FLOOR`,
//! sited in the `discipline` job — unconditional, on every run,
//! structurally ineligible for sampling. The census greps for the
//! `k-lint` step named
//! *"compile and list every probe-gated test target"* — a FIXED-STRING
//! per-line grep, so that quotation must not be re-wrapped — and the
//! step therefore cannot be deleted quietly either.
//!
//! When the row IS drawn, `scripts/k_probe_sweep.sh` executes exactly
//! the invocation above at all three ε, dumping to `<outdir>/m2/`. That
//! dump rides BESIDE the CSV k-lint gates, not inside it — the M2
//! shapes are not part of the distribution those thresholds were argued
//! over. The command block above is therefore the same command CI runs,
//! and it must stay that way: a runbook nothing executes cannot tell
//! you it has gone stale.
//!
//! Without `CAD_K_REPORT_OUT` the CSV goes to stdout (lines prefixed
//! by nothing — pipe as needed). Columns:
//! `shape,predicate,margin,band_zero,band_escalate,outcome`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use profile::RawLoop;
use std::io::Write as _;

use geom_core::Tol;
use geom_core::k_stats::{self, MarginSample, Probe, SampleOutcome};
use geom_core::{Point2, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{mass_properties, validate, validate_closed, validate_geometric};

fn p2(x: f64, y: f64) -> Point2<Probe> {
    Point2::new(Probe(x), Probe(y))
}

fn v(x: f64, y: f64, b: f64) -> ProfileVertex<Probe> {
    ProfileVertex::new(p2(x, y), Probe(b))
}

fn validated(loops: Vec<ProfileLoop<Probe>>) -> ValidatedProfile<Probe> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .unwrap()
}

fn axis_y() -> RevolveAxis<Probe> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(Probe(0.0), Probe(1.0)),
    }
}

/// Build one shape end-to-end at `Probe`, running the full pipeline;
/// returns everything recorded.
fn run_shape(build: impl FnOnce() -> topo::Body<Probe>) -> Vec<MarginSample> {
    k_stats::start_recording();
    let body = build();
    validate(&body).expect("tier 1");
    validate_closed(&body).expect("tier 2");
    validate_geometric(&body, Tol::witness()).expect("tier 3");
    mass_properties(&body, Tol::witness()).expect("mass properties");
    k_stats::take_samples()
}

fn shapes() -> Vec<(&'static str, Vec<MarginSample>)> {
    let theta_near_full = core::f64::consts::TAU - 0.01;
    vec![
        (
            "l_prism",
            run_shape(|| {
                let lp = ProfileLoop::polygon([
                    p2(0.0, 0.0),
                    p2(2.0, 0.0),
                    p2(2.0, 1.0),
                    p2(1.0, 1.0),
                    p2(1.0, 2.0),
                    p2(0.0, 2.0),
                ]);
                extrude(
                    &validated(vec![lp]),
                    Extrusion::Distance(Probe(1.0)),
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "holed_prism",
            run_shape(|| {
                let outer = ProfileLoop::polygon([
                    p2(-2.0, -2.0),
                    p2(2.0, -2.0),
                    p2(2.0, 2.0),
                    p2(-2.0, 2.0),
                ]);
                let hole = ProfileLoop::new(vec![v(1.0, 0.0, 1.0), v(-1.0, 0.0, 1.0)]);
                extrude(
                    &validated(vec![outer, hole]),
                    Extrusion::Distance(Probe(1.0)),
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "rounded_prism",
            run_shape(|| {
                let b = (core::f64::consts::FRAC_PI_8).tan();
                let r = 0.5;
                let lp = ProfileLoop::new(vec![
                    v(r, 0.0, 0.0),
                    v(2.0 - r, 0.0, b),
                    v(2.0, r, 0.0),
                    v(2.0, 2.0 - r, b),
                    v(2.0 - r, 2.0, 0.0),
                    v(r, 2.0, b),
                    v(0.0, 2.0 - r, 0.0),
                    v(0.0, r, b),
                ])
                // Every joint of a rounded rectangle is a fillet arc
                // meeting an edge at first-order carrier contact:
                // tangency is declared intent, verified at validation,
                // never inferred.
                .with_tangent_joints((0..8).collect());
                extrude(
                    &validated(vec![lp]),
                    Extrusion::Distance(Probe(1.0)),
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "ball",
            run_shape(|| {
                let lp = ProfileLoop::new(vec![v(0.0, -1.0, 1.0), v(0.0, 1.0, 0.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Full,
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "cone",
            run_shape(|| {
                let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Full,
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "washer",
            run_shape(|| {
                let lp =
                    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Full,
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "donut",
            run_shape(|| {
                let lp = ProfileLoop::new(vec![v(2.0, -0.5, 1.0), v(2.0, 0.5, 1.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Full,
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "wedge",
            run_shape(|| {
                let lp =
                    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Partial(Probe(core::f64::consts::FRAC_PI_2)),
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "axis_wedge",
            run_shape(|| {
                let lp =
                    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Partial(Probe(core::f64::consts::FRAC_PI_2)),
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
        (
            "near_full_wedge",
            run_shape(move || {
                let lp =
                    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
                revolve(
                    &validated(vec![lp]),
                    axis_y(),
                    Revolution::Partial(Probe(theta_near_full)),
                    Tol::witness(),
                )
                .unwrap()
                .body
            }),
        ),
    ]
}

fn outcome_str(o: SampleOutcome) -> &'static str {
    match o {
        SampleOutcome::Definite(geom_core::Sign::Negative) => "negative",
        SampleOutcome::Definite(geom_core::Sign::Zero) => "zero",
        SampleOutcome::Definite(geom_core::Sign::Positive) => "positive",
        SampleOutcome::Indeterminate => "indeterminate",
        SampleOutcome::Invalid => "invalid",
    }
}

/// The dump entry point (ignored: run explicitly, one process per ε).
#[test]
#[ignore]
fn dump_k_samples() {
    let eps = Tol::witness().get().eps;
    let mut csv = String::from("shape,predicate,margin,band_zero,band_escalate,outcome\n");
    let mut total = 0usize;
    let mut unnamed = 0usize;
    for (shape, samples) in shapes() {
        total += samples.len();
        for s in &samples {
            if s.predicate == "<unnamed>" {
                unnamed += 1;
            }
            csv.push_str(&format!(
                "{shape},{},{:e},{:e},{:e},{}\n",
                s.predicate,
                s.margin,
                s.band_zero,
                s.band_escalate,
                outcome_str(s.outcome)
            ));
        }
    }
    assert_eq!(
        unnamed, 0,
        "<unnamed> must be unreachable from shipped decide paths"
    );
    eprintln!("k_report: eps={eps:e}, {total} samples");
    match std::env::var("CAD_K_REPORT_OUT") {
        Ok(path) => {
            let mut f = std::fs::File::create(&path).expect("create CAD_K_REPORT_OUT");
            f.write_all(csv.as_bytes()).expect("write csv");
            eprintln!("k_report: wrote {path}");
        }
        Err(_) => print!("{csv}"),
    }
}

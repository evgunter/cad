//! M4 PR 8a spec D2 — rebuild-latency REPORTING (measured, never
//! gated).
//!
//! Per F8 the Band 4 corpus comes online with rebuild latency
//! *instrumented*, not contracted: this row prints a per-document
//! table of full-rebuild and incremental-recompute wall-clock times
//! and diffs it against the committed baseline
//! `tests/baseline/rebuild-latency.json`. **There is no threshold
//! gate.** A slow row is a REPORT, never a failure — PERF-PLAN stays
//! advisory, and a timing assertion would make CI's shared, noisy
//! runners into a source of false red.
//!
//! What this row *does* assert is arithmetic that has nothing to do
//! with the clock: that the incremental recompute reused exactly the
//! complement of the edit's downstream cone (the cone derived
//! independently from the recipe DAG). That is the architectural
//! property the timings are evidence *about*, and it is cheap and
//! deterministic to check.
//!
//! Variance: each figure is the MEDIAN OF 3 runs in one process.
//!
//! Reading the numbers: they come from the `dev` profile (opt-level 0
//! for the kernel crates — the workspace's deliberate default, see the
//! root `Cargo.toml` profile notes), which is what every other CI row
//! builds, so the baseline is comparable across rows and across PRs.
//! They are NOT release-representative and must never be quoted as
//! such.
//!
//! Refresh the baseline with `CAD_LATENCY_BASELINE_REFRESH=1` and
//! commit the result, saying so in the PR body. **Refresh only on a
//! quiet machine; the `refreshed_under` provenance field is how the
//! next reader audits that** (M5 PR 11 F5: a baseline refreshed under
//! two-lane cargo contention once read as a phantom 30× speedup).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::time::{Duration, Instant};

use editor_core::{CancelToken, EvalOptions, evaluate};

use corpus::{cone, documents, eval, failures};

/// The committed baseline's path, relative to the crate root.
const BASELINE: &str = "tests/baseline/rebuild-latency.json";
/// Set to `1` to rewrite the baseline from this run.
const REFRESH: &str = "CAD_LATENCY_BASELINE_REFRESH";
/// Repetitions per figure (median wins).
const REPS: usize = 3;

/// The median of an odd-length duration sample.
fn median(mut xs: Vec<Duration>) -> Duration {
    xs.sort_unstable();
    xs[xs.len() / 2]
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1e3
}

/// One document's measured row.
struct Row {
    name: &'static str,
    about: &'static str,
    nodes: usize,
    cone: usize,
    full_ms: f64,
    incr_ms: f64,
}

fn measure() -> Vec<Row> {
    let mut rows = Vec::new();
    for d in documents() {
        // --- full rebuild (cold: no prior evaluation) ---
        let mut fulls = Vec::with_capacity(REPS);
        for _ in 0..REPS {
            let t0 = Instant::now();
            let ev = eval::<f64>(&d.doc);
            fulls.push(t0.elapsed());
            let bad = failures(&ev);
            assert!(
                bad.is_empty(),
                "{}: latency row requires a green document:\n{}",
                d.name,
                bad.join("\n")
            );
        }

        // --- incremental recompute: ONE mid-DAG parameter edit ---
        let prior = eval::<f64>(&d.doc);
        let bumped = d.bumped();
        let expected = cone(&bumped, d.bump_root);
        let mut incrs = Vec::with_capacity(REPS);
        for _ in 0..REPS {
            let t0 = Instant::now();
            let ev = evaluate::<f64>(
                &bumped,
                Some(&prior),
                &CancelToken::new(),
                &EvalOptions::default(),
            );
            incrs.push(t0.elapsed());
            // The COUNTED-REUSE assertion (not a timing gate).
            assert_eq!(
                (ev.recomputed, ev.reused),
                (expected.len(), bumped.len() - expected.len()),
                "{}: incremental recompute did not reuse the cone complement",
                d.name
            );
        }

        rows.push(Row {
            name: d.name,
            about: d.about,
            nodes: d.len(),
            cone: expected.len(),
            full_ms: ms(median(fulls)),
            incr_ms: ms(median(incrs)),
        });
    }
    rows
}

/// The baseline document's JSON, as a value.
fn baseline() -> serde_json::Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(BASELINE);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("committed baseline {} unreadable: {e}", path.display()));
    serde_json::from_str(&text).expect("baseline is valid JSON")
}

/// Rewrites the baseline from `rows` (opt-in, `CAD_LATENCY_BASELINE_REFRESH=1`).
fn refresh(rows: &[Row]) {
    let mut docs = serde_json::Map::new();
    for r in rows {
        docs.insert(
            r.name.to_string(),
            serde_json::json!({
                "about": r.about,
                "nodes": r.nodes,
                "cone": r.cone,
                "full_ms": (r.full_ms * 10.0).round() / 10.0,
                "incremental_ms": (r.incr_ms * 100.0).round() / 100.0,
            }),
        );
    }
    let old = baseline();
    let out = serde_json::json!({
        "provenance": old.get("provenance").cloned().unwrap_or(serde_json::Value::Null),
        // Refresh-time provenance (F5): the figures are only
        // comparable across refreshes taken on a comparably quiet
        // machine — record what this one looked like.
        "refreshed_under": serde_json::json!({
            "cargo_rustc_procs": build_proc_count(),
            "mem_total_kb": mem_total_kb(),
        }),
        "documents": docs,
    });
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(BASELINE);
    let text = format!(
        "{}\n",
        serde_json::to_string_pretty(&out).expect("serialize")
    );
    std::fs::write(&path, text).expect("write baseline");
    println!(
        "REFRESHED {} — review the provenance block before committing.",
        path.display()
    );
}

/// Concurrent `cargo`/`rustc` processes at refresh time (the refresh
/// run's own `cargo test` is included — a count of 1-2 IS the quiet
/// state; more means another lane was building).
fn build_proc_count() -> usize {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return 0;
    };
    entries
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .chars()
                .all(|c| c.is_ascii_digit())
        })
        .filter_map(|e| std::fs::read_to_string(e.path().join("comm")).ok())
        .filter(|comm| {
            let c = comm.trim();
            c == "cargo" || c == "rustc"
        })
        .count()
}

/// Total system memory (kB), from `/proc/meminfo`.
fn mem_total_kb() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|text| {
            text.lines().find_map(|l| {
                l.strip_prefix("MemTotal:")
                    .and_then(|rest| rest.trim().trim_end_matches(" kB").trim().parse().ok())
            })
        })
        .unwrap_or(0)
}

/// **`#[ignore]` BY DESIGN — this row is REPORTING, and the ε matrix
/// paid for it 5× per run.** It is not a gate (see the module docs: no
/// threshold, a slow row is a report), so the only thing the ε battery
/// bought was ~18 s × 5 rows of wall clock printing a table nobody
/// reads per-ε.
///
/// INVARIANT: nothing is uncovered by ignoring it here. Its two
/// assertions are a strict subset of `m4_pr8_corpus.rs`, which the ε
/// matrix DOES run: the green-document check (`failures(&eval(&d.doc))`
/// empty, per corpus document) is the opening of
/// `every_document_evaluates_green`, which asserts that and more
/// (outcome, order length, cold recompute counts); and the
/// counted-reuse check (`(recomputed, reused) == (cone.len(), len -
/// cone.len())` after a `bumped()` edit against a prior evaluation) is
/// verbatim `incremental_recompute_reuses_the_cone_complement`, which
/// again asserts that and more (green after the bump, `reused > 0`).
/// Both are per-document loops over the same `documents()`.
///
/// The table itself still runs: ci.yml's dedicated `rebuild latency
/// (reporting)` job — and its mirror row in `local-scripts/ci-local.sh`
/// — pass `--ignored` so this test executes exactly once per CI run,
/// where the numbers are actually looked at. (In-tree idiom:
/// `crates/stl/tests/export.rs`'s `print_stl_hashes`.)
#[test]
#[ignore]
fn rebuild_latency_table() {
    let rows = measure();
    let base = baseline();
    let docs = base.get("documents").and_then(|d| d.as_object());

    let mut out = String::new();
    out.push_str("\n=== rebuild latency (REPORTING ONLY — no threshold gate, F8) ===\n");
    out.push_str(&format!(
        "{:<28} {:>5} {:>5} {:>11} {:>10} {:>11} {:>10}\n",
        "document", "nodes", "cone", "full (ms)", "vs base", "incr (ms)", "vs base"
    ));
    let pct = |now: f64, then: Option<f64>| match then {
        Some(t) if t > 0.0 => format!("{:+.0}%", (now / t - 1.0) * 100.0),
        _ => "  n/a".to_string(),
    };
    for r in &rows {
        let entry = docs.and_then(|d| d.get(r.name));
        let bf = entry
            .and_then(|e| e.get("full_ms"))
            .and_then(|v| v.as_f64());
        let bi = entry
            .and_then(|e| e.get("incremental_ms"))
            .and_then(|v| v.as_f64());
        out.push_str(&format!(
            "{:<28} {:>5} {:>5} {:>11.1} {:>10} {:>11.2} {:>10}\n",
            r.name,
            r.nodes,
            r.cone,
            r.full_ms,
            pct(r.full_ms, bf),
            r.incr_ms,
            pct(r.incr_ms, bi),
        ));
    }
    let total: f64 = rows.iter().map(|r| r.full_ms).sum();
    out.push_str(&format!(
        "{:<28} {:>5} {:>5} {:>11.1}\n",
        "TOTAL",
        rows.iter().map(|r| r.nodes).sum::<usize>(),
        "",
        total
    ));
    out.push_str(
        "dev profile (opt-level 0 outside spade/mesh), median of 3, one process; \
         machine-dependent — advisory only.\n",
    );
    println!("{out}");

    if std::env::var(REFRESH).as_deref() == Ok("1") {
        refresh(&rows);
    } else {
        // The baseline must at least COVER the corpus: a document with
        // no baseline row is a bookkeeping bug, not a slow document.
        let missing: Vec<_> = rows
            .iter()
            .map(|r| r.name)
            .filter(|n| docs.is_none_or(|d| !d.contains_key(*n)))
            .collect();
        assert!(
            missing.is_empty(),
            "the committed baseline has no row for {missing:?} — rerun with \
             {REFRESH}=1 and commit the refreshed {BASELINE}"
        );
    }
}

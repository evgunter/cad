//! M4 PR 8a spec D2 — rebuild-latency REPORTING (measured, never
//! gated).
//!
//! Per F8 the Band 4 corpus comes online with rebuild latency
//! *instrumented*, not contracted: this row prints a per-document
//! table of full-rebuild and incremental-recompute wall-clock times
//! and diffs it against the newest entry in the hosted-CI timing
//! history, `docs/perf-data/rebuild-latency/`. **There is no threshold
//! gate.** A slow row is a REPORT, never a failure — PERF-PLAN stays
//! advisory, and a timing assertion would make CI's shared, noisy
//! runners into a source of false red.
//!
//! What this row *does* assert is arithmetic that has nothing to do
//! with the clock: that the incremental recompute reused exactly the
//! complement of the edit's downstream cone (the cone derived
//! independently from the recipe DAG), and that every document's
//! STRUCTURE matches the committed manifest. That is the architectural
//! property the timings are evidence *about*, and it is cheap and
//! deterministic to check.
//!
//! # Two files, split along the rot line
//!
//! Timings and structure used to share one committed file, and the
//! timings rotted it: three developer-workstation refreshes disagreed
//! by 90-98% on every row with contention ruled out, so its own
//! provenance block ended up declaring cross-refresh comparison
//! meaningless (see `docs/PERF-SCAN-2026-08.md` §0). The two halves
//! now live apart, because only one of them can rot:
//!
//! - [`MANIFEST`] — `about` / `nodes` / `cone` per document. Exact,
//!   machine-independent, hand-maintained, and what the coverage and
//!   structural assertions below read. Nothing about a box can move
//!   these numbers.
//! - [`HISTORY`] — one file per merge to `main`, written by ci.yml's
//!   `rebuild latency (reporting)` job on a hosted runner and
//!   committed by it. Append-only: a run adds a filename, never edits
//!   an existing one, so concurrent merges cannot conflict and a
//!   cancelled run drops nothing.
//!
//! The `vs base` columns diff against the NEWEST history entry, so on
//! a PR they read against `main`'s last hosted measurement and on
//! `main` they read against the previous merge. There is no baseline
//! to refresh and no `*_REFRESH` env var: drift is recoverable from
//! the history by construction, which is the whole reason it is
//! append-only rather than overwritten.
//!
//! Variance: each figure is the MEDIAN OF [`REPS`] runs in one
//! process, and the table's `±` column is the half-range over those
//! runs as a percentage of the median. Read the `vs base` column
//! against that spread before believing it — a hosted 2-vCPU runner
//! has a fat tail, and a delta inside the spread is noise.
//!
//! Reading the numbers: they come from the `dev` profile (opt-level 0
//! for the kernel crates — the workspace's deliberate default, see the
//! root `Cargo.toml` profile notes), which is what every other CI row
//! builds, so the figures are comparable across rows and across PRs.
//! They are NOT release-representative and must never be quoted as
//! such.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use editor_core::{CancelToken, EvalOptions, evaluate};

use corpus::{cone, documents, eval, failures};

/// The committed STRUCTURAL manifest, relative to the crate root.
/// Machine-independent; carries no milliseconds.
const MANIFEST: &str = "tests/baseline/rebuild-latency.json";
/// The hosted-CI timing history, relative to the repo root. One file
/// per merge to `main`, named `<epoch-seconds>-<short-sha>.json` so a
/// lexicographic sort is a chronological one.
const HISTORY: &str = "docs/perf-data/rebuild-latency";
/// Set to a path to write this run's measurement there. ci.yml's
/// `rebuild latency (reporting)` job sets it; nothing else does.
const EMIT: &str = "CAD_LATENCY_EMIT";
/// Provenance the emitting job supplies (both optional).
const COMMIT: &str = "CAD_LATENCY_COMMIT";
const RUNNER: &str = "CAD_LATENCY_RUNNER";
/// Repetitions per figure — MUST BE ODD (the median is taken by
/// indexing the midpoint). Raised 3 -> 5 when the producer moved to
/// hosted CI: a shared 2-vCPU runner has a fatter tail than the
/// verified-quiet workstation this used to run on, and the `±` column
/// is only meaningful with enough samples to show it.
const REPS: usize = 5;
/// [`stats`] takes its median by indexing the midpoint, which is only
/// the median for an odd sample. A build error, not a test failure:
/// an even [`REPS`] would otherwise report a biased figure that still
/// looked plausible.
const _: () = assert!(REPS % 2 == 1, "REPS must be odd for the midpoint median");

/// The repository root (this crate lives at `crates/editor-core`).
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Median, min and max of a duration sample, in milliseconds.
fn stats(mut xs: Vec<Duration>) -> (f64, f64, f64) {
    assert!(!xs.is_empty(), "empty sample");
    xs.sort_unstable();
    let ms = |d: Duration| d.as_secs_f64() * 1e3;
    (ms(xs[xs.len() / 2]), ms(xs[0]), ms(xs[xs.len() - 1]))
}

/// One document's measured row.
struct Row {
    name: &'static str,
    about: &'static str,
    nodes: usize,
    cone: usize,
    full_ms: f64,
    full_min_ms: f64,
    full_max_ms: f64,
    incr_ms: f64,
    incr_min_ms: f64,
    incr_max_ms: f64,
}

impl Row {
    /// Half-range as a percentage of the median — the `±` column.
    fn full_spread_pct(&self) -> f64 {
        spread_pct(self.full_ms, self.full_min_ms, self.full_max_ms)
    }
    fn incr_spread_pct(&self) -> f64 {
        spread_pct(self.incr_ms, self.incr_min_ms, self.incr_max_ms)
    }
}

fn spread_pct(median: f64, min: f64, max: f64) -> f64 {
    if median > 0.0 {
        (max - min) / 2.0 / median * 100.0
    } else {
        0.0
    }
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

        let (full_ms, full_min_ms, full_max_ms) = stats(fulls);
        let (incr_ms, incr_min_ms, incr_max_ms) = stats(incrs);
        rows.push(Row {
            name: d.name,
            about: d.about,
            nodes: d.len(),
            cone: expected.len(),
            full_ms,
            full_min_ms,
            full_max_ms,
            incr_ms,
            incr_min_ms,
            incr_max_ms,
        });
    }
    rows
}

/// The committed structural manifest, as a value.
fn manifest() -> serde_json::Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(MANIFEST);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("committed manifest {} unreadable: {e}", path.display()));
    serde_json::from_str(&text).expect("manifest is valid JSON")
}

/// The newest timing-history entry: `(filename, documents map)`.
///
/// `None` when the history is empty — which is the bootstrap state and
/// the local-developer state, NOT an error. The table then prints
/// `n/a` in both `vs base` columns and the row is pure reporting.
fn latest_history() -> Option<(String, serde_json::Value)> {
    let dir = repo_root().join(HISTORY);
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".json"))
        .collect();
    // Lexicographic == chronological, by the `<epoch>-<sha>` naming.
    names.sort();
    let newest = names.pop()?;
    let text = std::fs::read_to_string(dir.join(&newest)).ok()?;
    let value: serde_json::Value = serde_json::from_str(&text).ok()?;
    Some((newest, value))
}

/// Concurrent `cargo`/`rustc` processes at measurement time (this
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

/// The build/host provenance that the `disputed_measurement` argument
/// went unresolved for want of. Recorded on EVERY entry so two
/// entries that disagree can be compared as environments, not just as
/// numbers.
fn environment() -> serde_json::Value {
    let var = |k: &str| std::env::var(k).unwrap_or_default();
    serde_json::json!({
        "runner": std::env::var(RUNNER).unwrap_or_else(|_| "local".to_string()),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "nproc": std::thread::available_parallelism().map_or(0, std::num::NonZero::get),
        "mem_total_kb": mem_total_kb(),
        "cargo_rustc_procs": build_proc_count(),
        "rustflags": var("RUSTFLAGS"),
        "rustup_toolchain": var("RUSTUP_TOOLCHAIN"),
        "cargo_profile_overrides": std::env::vars()
            .filter(|(k, _)| k.starts_with("CARGO_PROFILE_"))
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>(),
        "debug_assertions": cfg!(debug_assertions),
        "tolerance_eps": var("CAD_TOLERANCE_EPS"),
    })
}

/// Writes this run's measurement to `path` (opt-in, [`EMIT`]).
///
/// Writes the FILE ONLY — naming it, committing it and deciding
/// whether it belongs in the history is ci.yml's job, so a local run
/// with `CAD_LATENCY_EMIT` set can never touch the committed history.
fn emit(rows: &[Row], path: &str) {
    let mut docs = serde_json::Map::new();
    for r in rows {
        docs.insert(
            r.name.to_string(),
            serde_json::json!({
                "nodes": r.nodes,
                "cone": r.cone,
                "full_ms": (r.full_ms * 10.0).round() / 10.0,
                "full_min_ms": (r.full_min_ms * 10.0).round() / 10.0,
                "full_max_ms": (r.full_max_ms * 10.0).round() / 10.0,
                "incremental_ms": (r.incr_ms * 100.0).round() / 100.0,
                "incremental_min_ms": (r.incr_min_ms * 100.0).round() / 100.0,
                "incremental_max_ms": (r.incr_max_ms * 100.0).round() / 100.0,
            }),
        );
    }
    let measured_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let out = serde_json::json!({
        "commit": std::env::var(COMMIT).unwrap_or_else(|_| "unknown".to_string()),
        "measured_at_epoch_s": measured_at,
        "method": format!(
            "median of {REPS} runs in one process, std::time::Instant, sequential \
             evaluation schedule (EvalOptions::default()); dev profile (opt-level 0 \
             outside spade/mesh) — NOT release-representative"
        ),
        "environment": environment(),
        "documents": docs,
    });
    let text = format!(
        "{}\n",
        serde_json::to_string_pretty(&out).expect("serialize")
    );
    std::fs::write(path, text).unwrap_or_else(|e| panic!("write measurement to {path}: {e}"));
    println!("EMITTED measurement to {path}");
}

/// **`#[ignore]` BY DESIGN — this row is REPORTING, and the ε matrix
/// paid for it 5×.** It is not a gate (see the module docs: no
/// threshold, a slow row is a report), so the only thing the ε battery
/// bought was ~18 s × 5 rows of wall clock printing a table nobody
/// reads per-ε. That ~18 s is a dated reading and nothing re-takes it:
/// the timing register this file feeds (`docs/perf-data/rebuild-latency/`)
/// re-measures per-DOCUMENT rebuild latency, not this row's own wall
/// clock, and no assertion reads either number. The `#[ignore]` rests on
/// the coverage argument below, which holds at any wall clock.
///
/// INVARIANT: nothing is uncovered by ignoring it here. Its
/// green-document and counted-reuse assertions are a strict subset of
/// `m4_pr8_corpus.rs`, which the ε matrix DOES run: the green check
/// (`failures(&eval(&d.doc))` empty, per corpus document) is the
/// opening of `every_document_evaluates_green`, which asserts that and
/// more (outcome, order length, cold recompute counts); and the
/// counted-reuse check (`(recomputed, reused) == (cone.len(), len -
/// cone.len())` after a `bumped()` edit against a prior evaluation) is
/// verbatim `incremental_recompute_reuses_the_cone_complement`, which
/// again asserts that and more (green after the bump, `reused > 0`).
/// Both are per-document loops over the same `documents()`. The
/// structural manifest pins below are this row's OWN, and they are
/// ε-independent by construction (node counts and DAG cones do not
/// read a tolerance).
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

    // Emit BEFORE the assertions: when a structural pin fails, the
    // measurement artifact is exactly what the next reader wants, and
    // a panic here would otherwise throw the whole run's numbers away.
    if let Ok(path) = std::env::var(EMIT) {
        emit(&rows, &path);
    }

    let history = latest_history();
    let base = history
        .as_ref()
        .and_then(|(_, v)| v.get("documents"))
        .and_then(serde_json::Value::as_object);

    let mut out = String::new();
    out.push_str("\n=== rebuild latency (REPORTING ONLY — no threshold gate, F8) ===\n");
    out.push_str(&match &history {
        Some((name, _)) => format!("vs base: {HISTORY}/{name}\n"),
        None => format!("vs base: (no history under {HISTORY} — reporting only)\n"),
    });
    out.push_str(&format!(
        "{:<28} {:>5} {:>5} {:>10} {:>7} {:>9} {:>10} {:>7} {:>9}\n",
        "document", "nodes", "cone", "full (ms)", "±", "vs base", "incr (ms)", "±", "vs base"
    ));
    let pct = |now: f64, then: Option<f64>| match then {
        Some(t) if t > 0.0 => format!("{:+.0}%", (now / t - 1.0) * 100.0),
        _ => "  n/a".to_string(),
    };
    let field = |entry: Option<&serde_json::Value>, key: &str| {
        entry
            .and_then(|e| e.get(key))
            .and_then(serde_json::Value::as_f64)
    };
    for r in &rows {
        let entry = base.and_then(|d| d.get(r.name));
        out.push_str(&format!(
            "{:<28} {:>5} {:>5} {:>10.1} {:>6.0}% {:>9} {:>10.2} {:>6.0}% {:>9}\n",
            r.name,
            r.nodes,
            r.cone,
            r.full_ms,
            r.full_spread_pct(),
            pct(r.full_ms, field(entry, "full_ms")),
            r.incr_ms,
            r.incr_spread_pct(),
            pct(r.incr_ms, field(entry, "incremental_ms")),
        ));
    }
    let total: f64 = rows.iter().map(|r| r.full_ms).sum();
    out.push_str(&format!(
        "{:<28} {:>5} {:>5} {:>10.1}\n",
        "TOTAL",
        rows.iter().map(|r| r.nodes).sum::<usize>(),
        "",
        total
    ));
    out.push_str(
        "dev profile (opt-level 0 outside spade/mesh), median of 5, one process; \
         machine-dependent — advisory only. `±` is the half-range over the 5 runs; \
         a `vs base` delta inside it is noise.\n",
    );
    println!("{out}");

    // --- the ε-independent structural pins (arithmetic, not clock) ---
    let manifest = manifest();
    let pinned = manifest
        .get("documents")
        .and_then(serde_json::Value::as_object);

    // The manifest must at least COVER the corpus: a document with no
    // manifest row is a bookkeeping bug, not a slow document.
    let missing: Vec<_> = rows
        .iter()
        .map(|r| r.name)
        .filter(|n| pinned.is_none_or(|d| !d.contains_key(*n)))
        .collect();
    assert!(
        missing.is_empty(),
        "the committed manifest has no row for {missing:?} — add it to {MANIFEST} \
         (about / nodes / cone; no milliseconds live there)"
    );

    // ...and it must AGREE with the corpus. `nodes` and `cone` are
    // exact and machine-independent, so a mismatch is a real change in
    // a document's shape or in the cone derivation — never a slow box.
    for r in &rows {
        let entry = pinned.and_then(|d| d.get(r.name));
        let pin = |key: &str| {
            entry
                .and_then(|e| e.get(key))
                .and_then(serde_json::Value::as_u64)
        };
        assert_eq!(
            pin("nodes"),
            Some(r.nodes as u64),
            "{}: node count moved (manifest {:?}, measured {}) — update {MANIFEST} \
             and say why in the PR body",
            r.name,
            pin("nodes"),
            r.nodes
        );
        assert_eq!(
            pin("cone"),
            Some(r.cone as u64),
            "{}: edit cone moved (manifest {:?}, measured {}) — update {MANIFEST} \
             and say why in the PR body",
            r.name,
            pin("cone"),
            r.cone
        );
    }

    // Every manifest row must correspond to a live document, too: a
    // stale key outlives the document it described and quietly stops
    // pinning anything (M5 PR 12's die_fillet, deleted by hand).
    if let Some(pinned) = pinned {
        let stale: Vec<_> = pinned
            .keys()
            .filter(|k| !rows.iter().any(|r| r.name == k.as_str()))
            .collect();
        assert!(
            stale.is_empty(),
            "{MANIFEST} has rows for documents the corpus no longer registers: \
             {stale:?} — delete them"
        );
    }

    // `about` is prose and drifts silently; it is carried in the
    // manifest for the reader, so only require that it EXISTS.
    for r in &rows {
        assert!(
            pinned
                .and_then(|d| d.get(r.name))
                .and_then(|e| e.get("about"))
                .and_then(serde_json::Value::as_str)
                .is_some_and(|s| !s.trim().is_empty()),
            "{}: manifest row has no `about` — one line on what the document \
             proves (measured description: {:?})",
            r.name,
            r.about
        );
    }
}

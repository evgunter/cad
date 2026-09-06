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
//! **Three rows, and only one of them is `#[ignore]`d.**
//! [`rebuild_latency_manifest_pins_the_corpus_structure`] is an
//! ordinary test: it checks the manifest pins and nothing else, costs
//! no evaluation and no repeats, and therefore gates every PR through
//! the nextest archive. [`cpu_identity_degrades_rather_than_failing`]
//! is the second, and equally cheap: it pins the shapes
//! [`cpu_identity`] must survive, because a box that cannot answer
//! `/proc/cpuinfo` still owes a complete [`environment`] block.
//! [`rebuild_latency_table`] is the `#[ignore]`d
//! REPORTING row — the wall-clock measurement, the [`EMIT`] artifact
//! and the history comparison — run by its own scheduled job. The
//! first and last both check the pins, via the shared
//! [`assert_manifest_pins`], so the reporting row cannot silently
//! disagree with the gate.
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
//! - [`HISTORY`] — one file per RUN of the reporting job, written by
//!   `nightly.yml`'s `rebuild latency (reporting)` job on a hosted
//!   runner and committed by it. That job is on a nightly cron and is
//!   gated on `main` having moved since the last one, so the cadence
//!   is at most one entry per night and quiet days add none — NOT one
//!   per merge, which is what this said while the job lived in
//!   ci.yml. Append-only: a run adds a filename, never edits an
//!   existing one, so concurrent runs cannot conflict and a cancelled
//!   run drops nothing.
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
//! against that spread before believing it — a shared hosted runner
//! has a fat tail, and a delta inside the spread is noise.
//!
//! Reading the numbers: they come from the `dev` profile (opt-level 0
//! for the kernel crates — the workspace's deliberate default, see the
//! root `Cargo.toml` profile notes), which is what every other CI row
//! builds, so the figures are comparable across rows and across PRs.
//! They are NOT release-representative and must never be quoted as
//! such.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use editor_core::{
    CancelToken, ChecksConfig, DocEdit, DocParam, EvalOptions, ParamName, ProfileDoc, Subject,
    apply, evaluate, product_recorded, run_checks, run_checks_on,
};

use corpus::{cone, documents, eval, failures};
use geom_core::Tol;

/// The committed STRUCTURAL manifest, relative to the crate root.
/// Machine-independent; carries no milliseconds.
const MANIFEST: &str = "tests/baseline/rebuild-latency.json";
/// The hosted-CI timing history, relative to the repo root. One file
/// per RUN of the nightly reporting job (at most one a night, and none
/// on a night when `main` has not moved), named
/// `<epoch-seconds>-<short-sha>.json` so a lexicographic sort is a
/// chronological one.
const HISTORY: &str = "docs/perf-data/rebuild-latency";
/// Set to a path to write this run's measurement there. `nightly.yml`'s
/// `rebuild latency (reporting)` job sets it; nothing else does.
const EMIT: &str = "CAD_LATENCY_EMIT";
/// Provenance the emitting job supplies (both optional).
const COMMIT: &str = "CAD_LATENCY_COMMIT";
const RUNNER: &str = "CAD_LATENCY_RUNNER";
/// Repetitions per figure — MUST BE ODD (the median is taken by
/// indexing the midpoint). Raised 3 -> 5 when the producer moved to
/// hosted CI: a shared runner has a fatter tail than the
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

/// One document's ε-independent STRUCTURAL facts — the half of this
/// file that is a gate.
///
/// `nodes` is the recipe's node count and `cone` is the size of the
/// bump edit's downstream cone, derived from the DAG. **Neither reads
/// a tolerance and neither evaluates any geometry**, so both are exact
/// and machine-independent: no box, profile or ε can move them. That
/// is why they gate on every PR while the milliseconds below do not.
struct Structure {
    name: &'static str,
    about: &'static str,
    nodes: usize,
    cone: usize,
}

impl Row {
    fn structure(&self) -> Structure {
        Structure {
            name: self.name,
            about: self.about,
            nodes: self.nodes,
            cone: self.cone,
        }
    }
}

/// Derives [`Structure`] for every corpus document.
///
/// COSTS NO EVALUATION. `len()` reads the recipe, `bumped()` replays
/// one edit through `apply`, and `cone()` is a single forward sweep
/// over the node order — the whole corpus is milliseconds. The
/// expensive thing in this file is [`measure`], and nothing here calls
/// it.
fn structures() -> Vec<Structure> {
    documents()
        .into_iter()
        .map(|d| {
            let bumped = d.bumped();
            Structure {
                name: d.name,
                about: d.about,
                nodes: d.len(),
                cone: cone(&bumped, d.bump_root).len(),
            }
        })
        .collect()
}

/// The committed structural manifest must COVER the corpus, AGREE
/// with it, and carry no stale rows.
///
/// Shared by the gating row ([`rebuild_latency_manifest_pins_the_corpus_structure`])
/// and the reporting row ([`rebuild_latency_table`]) so the two cannot
/// drift: the table still fails on a structural mismatch when it runs,
/// it is simply no longer the only place the pins are checked.
fn assert_manifest_pins(rows: &[Structure]) {
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
    for r in rows {
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
    for r in rows {
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

/// **The gating half of this file, and it is NOT `#[ignore]`d.**
///
/// The structural manifest pins — node counts, edit cones, coverage
/// both ways, and a non-empty `about` — are this row's own claim (they
/// are not a subset of `m4_pr8_corpus.rs`), and they are ε-independent
/// AND clock-independent by construction: node counts and DAG cones do
/// not read a tolerance and do not evaluate geometry. So they belong
/// in the archive, on every PR, where a document that silently gains a
/// node is caught BEFORE it merges.
///
/// Before the split they lived inside [`rebuild_latency_table`], which
/// is `#[ignore]`d and therefore executed only by the dedicated
/// reporting job — historically main pushes. A shape change was
/// detectable after the merge, if at all. This row costs no
/// evaluation and no wall-clock repeats, so gating it is free.
///
/// The timings stay where they were: [`rebuild_latency_table`] still
/// checks the same pins when it runs, via the shared
/// [`assert_manifest_pins`].
#[test]
fn rebuild_latency_manifest_pins_the_corpus_structure() {
    assert_manifest_pins(&structures());
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
                Tol::witness(),
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

/// The fin count the registry/gather split is measured at — the point
/// the claim in `checks.rs` and `product.rs` is stated for, and the
/// only place in this file that leaves the corpus's own parameters.
///
/// The corpus heat sink is a fin pattern plus an explicit union chain,
/// so at this count its product is one chain solid and 160 pattern
/// instances. Both counts are exact and are pinned by
/// `docm5_subject::the_registry_split_is_measured_at_a_pinned_point`,
/// which gates on every PR — a wall clock is only worth reading beside
/// the size it was taken at, and that size must not drift silently.
const SPLIT_FINS: i64 = 160;

/// The registry's two terms, separated.
///
/// `checks.rs`'s cost note used to state one number for the pair and
/// could not say which term dominated, because the registry gathered
/// its own subject and the gather was inside the figure. The subject
/// door separated them, and this is what re-takes them:
///
/// - `gather_ms` — [`product_recorded`] alone, from nothing.
/// - `checks_ms` — the registry over a subject already in hand.
/// - `whole_ms` — the wrapper, which is the two together and is what
///   the old single figure measured.
/// - `census_ms` — the tier-3' census over the SAME aggregate, the
///   term this resident exists instead of. Fewer reps than the others
///   ([`CENSUS_REPS`]) because it costs seconds where they cost
///   milliseconds, and it REFUSES on this document (the fins meet the
///   base), so it is the cost of a refusing run.
struct Split {
    solids: usize,
    faces: usize,
    census_findings: usize,
    gather_ms: (f64, f64, f64),
    checks_ms: (f64, f64, f64),
    whole_ms: (f64, f64, f64),
    census_ms: (f64, f64, f64),
}

/// Repetitions for the census term alone — MUST BE ODD, same reason as
/// [`REPS`]. Three rather than five because this term is ~10^3 times
/// the others and five would be most of this job's wall clock for a
/// figure quoted to two significant digits.
const CENSUS_REPS: usize = 3;
const _: () = assert!(
    CENSUS_REPS % 2 == 1,
    "CENSUS_REPS must be odd for the midpoint median"
);

/// The corpus heat sink with its fin count driven to `fins`.
fn heatsink_at(fins: i64) -> ProfileDoc {
    let tol = Tol::witness();
    let entry = documents()
        .into_iter()
        .find(|d| d.name == "heat_sink")
        .expect("the corpus carries the heat sink");
    apply(
        &entry.doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("fins"),
            value: DocParam::Count { value: fins },
        },
        tol,
    )
    .expect("the fin count is a document parameter")
    .doc
}

/// Takes the split (module docs on [`Split`]).
fn measure_split() -> Split {
    let tol = Tol::witness();
    let cfg = ChecksConfig::default();
    let doc = heatsink_at(SPLIT_FINS);
    let ev = eval::<f64>(&doc);
    let bad = failures(&ev);
    assert!(
        bad.is_empty(),
        "the split point must be green:\n{}",
        bad.join("\n")
    );

    let subject = product_recorded(&doc, &ev, tol).expect("the heat sink gathers");
    let (solids, faces) = (subject.body.solids().count(), subject.body.faces().count());

    let mut gathers = Vec::with_capacity(REPS);
    let mut checks = Vec::with_capacity(REPS);
    let mut wholes = Vec::with_capacity(REPS);
    let mut census = Vec::with_capacity(CENSUS_REPS);
    let mut census_findings = 0;
    for _ in 0..REPS {
        let t0 = Instant::now();
        let gathered = product_recorded(&doc, &ev, tol).expect("gathers");
        gathers.push(t0.elapsed());
        drop(gathered);

        let t0 = Instant::now();
        run_checks_on(&doc, &ev, Subject::Product(&subject), &cfg, tol).expect("the registry runs");
        checks.push(t0.elapsed());

        let t0 = Instant::now();
        run_checks(&doc, &ev, &cfg, tol).expect("and so does its wrapper");
        wholes.push(t0.elapsed());
    }

    for _ in 0..CENSUS_REPS {
        let t0 = Instant::now();
        let verdict = <f64 as topo::AtRestPolicy>::gate_at_rest_declared(
            &subject.body,
            &subject.contacts,
            tol,
        );
        census.push(t0.elapsed());
        census_findings = verdict.as_ref().err().map_or(0, Vec::len);
    }

    Split {
        solids,
        faces,
        census_findings,
        gather_ms: stats(gathers),
        checks_ms: stats(checks),
        whole_ms: stats(wholes),
        census_ms: stats(census),
    }
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

/// The flag subset that actually moves these rows. Not the whole `flags`
/// line: the opt-0/opt-2 ratio measured 30% apart between an AVX-512 guest
/// and CI (2026-08-22 census), which is the discrimination worth recording,
/// and the same two names the lane's other emitters probe
/// (`scripts/criterion-emit.py`, `scripts/opt-level-calibrate.py`).
const HOST_CPU_FLAGS: [&str; 2] = ["avx2", "avx512f"];

/// Where host identity is read from. Passed to [`cpu_identity`] rather than
/// read inline, so the path is a seam a test can point at nothing.
const CPUINFO: &str = "/proc/cpuinfo";

/// Which box this is, as far as `/proc/cpuinfo` can say.
///
/// `runner`, `nproc`, `mem_total_kb` and `arch` are constant across a hosted
/// runner pool while the CPU generation underneath it is not, so without this
/// pair a sample cannot be attributed to a host at all — only to a class of
/// host.
///
/// PARITY OBLIGATION: two more copies of this parser exist, in
/// `scripts/criterion-emit.py` and `scripts/opt-level-calibrate.py`. They
/// are copies rather than one reader because no cheap home is shared across
/// Rust and Python, so the obligation is manual: a change to the field
/// names, to [`HOST_CPU_FLAGS`], or to what a null means here is owed to
/// both of them in the same diff.
///
/// `(None, None)` means the file could not be read; an empty flag list means
/// the flags are genuinely absent. A reader must be able to tell those apart,
/// and a box without `/proc` must not cost the whole environment block.
fn cpu_identity(path: &str) -> (Option<String>, Option<Vec<String>>) {
    let Ok(text) = std::fs::read_to_string(path) else {
        return (None, None);
    };
    cpu_identity_of(&text)
}

/// The parse half of [`cpu_identity`], split off so the shapes a
/// `/proc/cpuinfo` can take are testable without one.
fn cpu_identity_of(text: &str) -> (Option<String>, Option<Vec<String>>) {
    let mut model: Option<String> = None;
    let mut flags: Option<Vec<String>> = None;
    for line in text.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        match key.trim() {
            "model name" if model.is_none() => {
                let v = value.trim();
                if !v.is_empty() {
                    model = Some(v.to_string());
                }
            }
            "flags" if flags.is_none() => {
                let mut found: Vec<String> = Vec::new();
                for flag in HOST_CPU_FLAGS {
                    if value.split_whitespace().any(|f| f == flag) {
                        found.push(flag.to_string());
                    }
                }
                flags = Some(found);
            }
            _ => {}
        }
    }
    (model, Some(flags.unwrap_or_default()))
}

/// Host identity degrades rather than failing: a box whose `/proc/cpuinfo`
/// cannot be read still writes a complete environment block, and a reader can
/// tell "the file was not there" from "the flags were not set".
#[test]
fn cpu_identity_degrades_rather_than_failing() {
    assert_eq!(cpu_identity("/nonexistent/cpuinfo"), (None, None));

    // THE WHOLE POINT OF THE NULLS: the rest of the block survives them. Read
    // back the VALUES, not the keys — `serde_json`'s `get` answers
    // `Some(Null)` for a null field, so key presence would hold even if the
    // block had collapsed to nulls entirely.
    let degraded = environment_from("/nonexistent/cpuinfo");
    assert!(degraded["cpu_model"].is_null() && degraded["cpu_flags"].is_null());
    assert_eq!(degraded["os"], std::env::consts::OS);
    assert_eq!(degraded["arch"], std::env::consts::ARCH);
    assert!(
        degraded["nproc"].as_u64().is_some_and(|n| n > 0),
        "an unreadable /proc/cpuinfo cost the block its core count: {degraded}"
    );
    assert!(degraded["runner"].is_string() && degraded["rustflags"].is_string());
    assert!(degraded["cargo_profile_overrides"].is_array());
    assert_eq!(degraded["debug_assertions"], cfg!(debug_assertions));
    // `/proc`-derived, so asserted only where `/proc` is — this test must not
    // red on a developer's box for being a developer's box.
    if std::path::Path::new("/proc/meminfo").exists() {
        assert!(
            degraded["mem_total_kb"].as_u64().is_some_and(|m| m > 0),
            "an unreadable /proc/cpuinfo cost the block its memory reading: {degraded}"
        );
    }

    // And the real path fills the pair in on a box that has the file, so the
    // nulls above are the degradation and not the only thing this can produce.
    if std::path::Path::new(CPUINFO).exists() {
        assert!(!environment()["cpu_model"].is_null());
    }

    // Readable but shaped differently — an aarch64 `/proc/cpuinfo` carries
    // `Features`, not `flags`, and no `model name`. The flag list is then
    // EMPTY rather than absent, which is what says the file WAS read.
    let (model, flags) = cpu_identity_of("processor\t: 0\nFeatures\t: fp asimd\n");
    assert_eq!(model, None);
    assert_eq!(flags, Some(Vec::new()));

    // The ordinary case: first `model name` only, probed flag subset only.
    let (model, flags) =
        cpu_identity_of("model name\t: A\nflags\t: fpu avx2 sse\nmodel name\t: B\n");
    assert_eq!(model.as_deref(), Some("A"));
    assert_eq!(flags, Some(vec!["avx2".to_string()]));
}

/// The build/host provenance that the `disputed_measurement` argument
/// went unresolved for want of. Recorded on EVERY entry so two
/// entries that disagree can be compared as environments, not just as
/// numbers.
fn environment() -> serde_json::Value {
    environment_from(CPUINFO)
}

/// [`environment`] with the host-identity path as a seam, so a test can
/// build the block a box with no `/proc/cpuinfo` would write and read the
/// OTHER fields back out of it. Without the seam the claim that a missing
/// file costs only its own two fields is untestable in this emitter, which
/// is the one whose output reaches `docs/perf-data/rebuild-latency/`.
fn environment_from(cpuinfo: &str) -> serde_json::Value {
    let var = |k: &str| std::env::var(k).unwrap_or_default();
    let (cpu_model, cpu_flags) = cpu_identity(cpuinfo);
    serde_json::json!({
        "runner": std::env::var(RUNNER).unwrap_or_else(|_| "local".to_string()),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "nproc": std::thread::available_parallelism().map_or(0, std::num::NonZero::get),
        "mem_total_kb": mem_total_kb(),
        // Which box, not just which class of box: every field above is
        // constant across a hosted runner pool. See `cpu_identity`.
        "cpu_model": cpu_model,
        "cpu_flags": cpu_flags,
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
fn emit(rows: &[Row], split: &Split, path: &str) {
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
    let ms = |x: f64| (x * 100.0).round() / 100.0;
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
        // The registry's two terms at the point `checks.rs` states them
        // for — the scheduled re-measure that claim rests on.
        "registry_split": {
            "document": "heat_sink",
            "fins": SPLIT_FINS,
            "solids": split.solids,
            "faces": split.faces,
            "gather_ms": ms(split.gather_ms.0),
            "gather_min_ms": ms(split.gather_ms.1),
            "gather_max_ms": ms(split.gather_ms.2),
            "checks_ms": ms(split.checks_ms.0),
            "checks_min_ms": ms(split.checks_ms.1),
            "checks_max_ms": ms(split.checks_ms.2),
            "whole_ms": ms(split.whole_ms.0),
            "whole_min_ms": ms(split.whole_ms.1),
            "whole_max_ms": ms(split.whole_ms.2),
            "census_ms": ms(split.census_ms.0),
            "census_min_ms": ms(split.census_ms.1),
            "census_max_ms": ms(split.census_ms.2),
            "census_findings": split.census_findings,
        },
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
/// threshold, a slow row is a report), so the ε battery bought only
/// wall clock, printing a table nobody reads per-ε. The `#[ignore]`
/// rests on the coverage argument below.
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
/// structural manifest pins ARE this row's own — they are not covered
/// anywhere else — which is exactly why they no longer live only here:
/// they are ε-independent AND clock-independent by construction (node
/// counts and DAG cones do not read a tolerance and evaluate no
/// geometry), so they were split out into the unignored
/// [`rebuild_latency_manifest_pins_the_corpus_structure`] and now gate
/// every PR at no measurable cost. This row still checks them, through
/// the same [`assert_manifest_pins`], so the two cannot drift.
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
    let split = measure_split();

    // Emit BEFORE the assertions: when a structural pin fails, the
    // measurement artifact is exactly what the next reader wants, and
    // a panic here would otherwise throw the whole run's numbers away.
    if let Ok(path) = std::env::var(EMIT) {
        emit(&rows, &split, &path);
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

    // --- the registry's two terms, at the point the claim states them ---
    let split_base = history.as_ref().and_then(|(_, v)| v.get("registry_split"));
    out.push_str(&format!(
        "\n=== registry split (REPORTING ONLY) — heat_sink at {} fins, \
         {} solids / {} faces ===\n",
        SPLIT_FINS, split.solids, split.faces
    ));
    out.push_str(&format!(
        "{:<28} {:>10} {:>7} {:>9}\n",
        "term", "ms", "±", "vs base"
    ));
    out.push_str(&format!(
        "(the census refuses here: {} finding(s); \
         it is measured over {CENSUS_REPS} runs, the others over {REPS})\n",
        split.census_findings
    ));
    for (label, key, (median, min, max)) in [
        ("gather (product_recorded)", "gather_ms", split.gather_ms),
        ("registry over a subject", "checks_ms", split.checks_ms),
        ("run_checks (the two together)", "whole_ms", split.whole_ms),
        ("tier-3' census (refuses)", "census_ms", split.census_ms),
    ] {
        out.push_str(&format!(
            "{label:<28} {median:>10.2} {:>6.0}% {:>9}\n",
            spread_pct(median, min, max),
            pct(median, field(split_base, key)),
        ));
    }
    println!("{out}");

    // --- the ε-independent structural pins (arithmetic, not clock) ---
    // Also checked, unignored, by
    // `rebuild_latency_manifest_pins_the_corpus_structure` above — this
    // call is what keeps the two rows from drifting apart.
    assert_manifest_pins(&rows.iter().map(Row::structure).collect::<Vec<_>>());
}

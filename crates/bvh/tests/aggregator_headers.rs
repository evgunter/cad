//! Every aggregating `tests/all.rs` header states the no-restated-count
//! rule in ONE spelling, and none of them carries the retired one.
//! (The count of them is not written here either — this file's own
//! subject is what a hand-written count of a compiler-known set is.)
//!
//! WHY THIS EXISTS. A count of a set the compiler already knows is a
//! second, unchecked copy — and what is left to protect is not the
//! number but the **uniformity**: twelve paraphrases of one rule is the
//! same defect one level up.
//!
//! WHY IT LIVES IN `bvh`. Its subject is workspace-wide, so no crate owns
//! it and any home is arbitrary; this is the cheapest test binary in the
//! workspace and it is built by every shard, so the check costs a file
//! read and runs wherever the suite runs. It needs no CI wiring, which is
//! deliberate: a guard that has to be wired is a guard that can be
//! unwired.
//!
//! THE SURVEY THAT FOUND THIS, so a successor can re-run and widen it.
//! The candidate pool was two `rg` patterns for sentences naming a SET of
//! files, modules or tests — the first over suite headers, the second
//! over `docs/`:
//!
//! ```text
//! rg -n '^//!.*\b(every|all|each|both|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|[0-9]+)\b.{0,50}\b(suites?|files?|tests?|modules?|crates?|targets?|twins?|siblings?|probes?)\b' crates --glob 'crates/*/tests/**/*.rs'
//! rg -n -i '\b(two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|[0-9]+)\b[^.]{0,45}\b(files|modules|crates|tests|targets|suites)\b' docs/
//! ```
//!
//! **Both patterns are keyed on names, which is the class they hunt**, so
//! the pool they produce is a floor and never a census: an enumeration
//! spelled without a numeral or a quantifier does not appear in it.

// A read failure here is a broken checkout, not a test outcome: this suite's
// subject is the content of files that must exist.
#![allow(clippy::expect_used)]

use std::path::{Path, PathBuf};

/// The rule, verbatim. Every aggregating `tests/all.rs` carries exactly
/// this, so there is one spelling to change if it ever changes.
const RULE: &str = "\
//! The suite count is deliberately NOT restated in prose here:
//! `every_suite_file_is_aggregated` below checks this file against the
//! directory on every run, and a number written out beside it is a
//! second, unchecked copy of a set the compiler already knows.";

/// The spelling this replaced. Forbidden by name so a revert reds rather
/// than merely diverging.
const RETIRED: &str = "separate test targets";

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/bvh has a parent")
        .to_path_buf()
}

/// Every `tests/all.rs` that aggregates suites via `#[path]`.
fn aggregators() -> Vec<(String, String)> {
    let mut found = Vec::new();
    let dir = crates_dir();
    let entries = std::fs::read_dir(&dir).expect("crates/ is readable");
    for entry in entries {
        let krate = entry.expect("readable dir entry").path();
        let all = krate.join("tests").join("all.rs");
        let Ok(src) = std::fs::read_to_string(&all) else {
            continue;
        };
        if !src.contains("#[path = \"") {
            continue; // pncad's binary is one file, not an aggregator
        }
        let name = krate
            .file_name()
            .expect("crate dir has a name")
            .to_string_lossy()
            .to_string();
        found.push((name, src));
    }
    found.sort();
    found
}

/// A SELECTION THAT MATCHES NOTHING IS NOT A PASS. The walk is derived
/// from the directory, so an empty answer looks exactly like a clean one.
#[test]
fn the_aggregator_set_is_not_empty() {
    let found = aggregators();
    assert!(
        found.len() >= 12,
        "found {} aggregating tests/all.rs under crates/, expected at least 12 — \
         either the aggregation convention is gone or this walk stopped working, \
         and both mean the checks below covered nothing: {:?}",
        found.len(),
        found.iter().map(|(k, _)| k).collect::<Vec<_>>()
    );
}

#[test]
fn every_aggregator_header_states_the_rule_in_one_spelling() {
    let missing: Vec<String> = aggregators()
        .into_iter()
        .filter(|(_, src)| !src.contains(RULE))
        .map(|(k, _)| k)
        .collect();
    assert!(
        missing.is_empty(),
        "these crates' tests/all.rs headers do not carry the shared \
         no-restated-count rule verbatim: {missing:?}. Copy it from \
         crates/topo/tests/all.rs — a paraphrase is a second spelling, which \
         is the defect this guards."
    );
}

#[test]
fn no_aggregator_header_restates_a_suite_count() {
    let offenders: Vec<String> = aggregators()
        .into_iter()
        .filter(|(_, src)| src.contains(RETIRED))
        .map(|(k, _)| k)
        .collect();
    assert!(
        offenders.is_empty(),
        "these crates' tests/all.rs headers use the retired \
         `{RETIRED}` phrasing, which carried a hand-written suite count: \
         {offenders:?}. The count is not restated in prose; \
         `every_suite_file_is_aggregated` is what knows the set."
    );
}

/// **The build-cost measurement is not restated in any header.** The two
/// figures behind WHY ONE BINARY were measured once, on a dated run, and
/// nothing in this repo re-takes them; the LINK/DEBUGINFO note in
/// `.github/workflows/ci.yml` is the one place that carries them with
/// their date, their provenance run and the record of what has since
/// changed. A header that restates one is a copy that goes stale in
/// silence — which is what happened, in eleven headers at once.
#[test]
fn no_aggregator_header_restates_the_build_cost_measurement() {
    const FIGURES: [&str; 2] = ["494 of the 514", "1.9 s"];
    let offenders: Vec<String> = aggregators()
        .into_iter()
        .filter(|(_, src)| FIGURES.iter().any(|f| src.contains(f)))
        .map(|(k, _)| k)
        .collect();
    assert!(
        offenders.is_empty(),
        "these crates' tests/all.rs headers restate a build-cost figure: \
         {offenders:?}. State the mechanism and point at the LINK/DEBUGINFO \
         note in .github/workflows/ci.yml, which owns the numbers; copy the \
         paragraph from crates/bvh/tests/all.rs."
    );
}

//! Every aggregating `tests/all.rs` header states the no-restated-count
//! rule in ONE spelling, and none of them carries the retired one.
//! (The count of them is not written here either — this file's own
//! subject is what a hand-written count of a compiler-known set is.)
//! The last row below is about the OTHER shape: a crate with
//! `autotests = false` whose `tests/` mounts no suite at all, which is
//! legitimate only for a directory holding one file and has no in-crate
//! row of its own to say so.
//!
//! WHY THIS EXISTS. A count of a set the compiler already knows is a
//! second, unchecked copy — and what is left to protect is not the
//! number but the **uniformity**: twelve paraphrases of one rule is the
//! same defect one level up.
//!
//! WHAT EACH HALF READS. The three header rows below are about
//! PROSE, so they read [`test_utils::source::comments_only`]: a rule
//! restated in a `format!` is not a header carrying it, and a retired
//! phrase quoted in code is not a header using it. The aggregator
//! SELECTION is about a mount, whose needle is a string literal, so it
//! reads [`test_utils::source::code_and_literals`] — a `#[path]` line
//! that has been commented out aggregates nothing.
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

use test_utils::source::{code_and_literals, comments_only, rust_sources};

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
        // Comments blanked, string literals KEPT: the mount is a
        // string literal, and a `#[path]` line inside a comment does
        // not aggregate anything.
        if !code_and_literals(&src).contains("#[path = \"") {
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
        .filter(|(_, src)| !comments_only(src).contains(RULE))
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
        .filter(|(_, src)| comments_only(src).contains(RETIRED))
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
        .filter(|(_, src)| {
            let prose = comments_only(src);
            FIGURES.iter().any(|f| prose.contains(f))
        })
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

/// Every crate whose test set is NOT auto-discovered — `autotests = false`
/// in its manifest — paired with its `tests/` directory.
///
/// **That flag is the precondition, which is why the population is keyed
/// on it and not on a filename.** With it set, cargo compiles the
/// declared `[[test]]` roots and whatever they reach and nothing else,
/// so a file nothing reaches is dropped silently; without it, every
/// `tests/*.rs` becomes its own target and nothing can be dropped at
/// all. Keying on `tests/all.rs` being READABLE would have made a
/// renamed, moved or differently-named aggregator root look like an
/// absent crate and pass this row green over a live hazard.
///
/// A crate with `autotests = false` and no `tests/` directory at all is
/// not in the population: there is no file to drop.
fn autotests_off() -> Vec<(String, PathBuf)> {
    let mut found = Vec::new();
    let dir = crates_dir();
    let entries = std::fs::read_dir(&dir).expect("crates/ is readable");
    for entry in entries {
        let krate = entry.expect("readable dir entry").path();
        let Ok(manifest) = std::fs::read_to_string(krate.join("Cargo.toml")) else {
            continue;
        };
        // The manifest is TOML, so this is a line test rather than a
        // Rust view: the key at the start of a line, in the package
        // table every one of these manifests opens with.
        if !manifest.lines().any(|l| l.starts_with("autotests = false")) {
            continue;
        }
        let tests = krate.join("tests");
        if !tests.is_dir() {
            continue;
        }
        let name = krate
            .file_name()
            .expect("crate dir has a name")
            .to_string_lossy()
            .to_string();
        found.push((name, tests));
    }
    found.sort();
    found
}

/// Whether anything under `tests` mounts a suite with `#[path]`.
///
/// Over every `.rs` file in the directory rather than over `all.rs`
/// alone, for [`autotests_off`]'s reason: the aggregator root is named
/// by the manifest, not by this file, and a rename must not read as
/// "this crate aggregates nothing".
fn mounts_a_suite(tests: &Path) -> bool {
    rust_sources(tests).iter().any(|path| {
        let text = std::fs::read_to_string(path).expect("a walked test file reads back");
        // The mount is a string literal, and one inside a comment
        // aggregates nothing.
        code_and_literals(&text).contains("#[path = \"")
    })
}

/// **A test directory that mounts nothing is the SINGLE-FILE form, and
/// this is what holds it to that.**
///
/// `autotests = false` plus one `[[test]]` target means a `tests/*.rs`
/// file that nothing mounts is not compiled and does not run — from
/// outside, indistinguishable from a suite that passes. An aggregating
/// crate catches that itself, with `every_suite_file_is_aggregated`. A
/// crate that mounts nothing carries no such row: `pncad` is the
/// population, and it cannot carry one, because the row reads
/// `test_utils::source` and that file's own closure guard admits no
/// `use` root but the façade. So the claim that it does not need one is
/// exactly the claim that its `tests/` holds a single file — a sentence
/// in a header until this row, which is why it is here and not there.
///
/// **An empty population passes, deliberately**: it means every crate
/// with `autotests = false` aggregates and every one of them carries
/// its own guard, which is the stronger state, not an unchecked one.
/// What it CANNOT be is a crate that dropped out of the walk, which is
/// the whole reason the population is keyed on the manifest flag.
#[test]
fn a_non_aggregating_tests_directory_holds_one_suite_file() {
    let offenders: Vec<String> = autotests_off()
        .into_iter()
        .filter(|(_, tests)| !mounts_a_suite(tests))
        .filter_map(|(krate, tests)| {
            let files = rust_sources(&tests);
            (files.len() > 1).then(|| format!("{krate}: {} .rs files under tests/", files.len()))
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "a crate with `autotests = false` whose tests/ mounts no suite has no \
         `every_suite_file_is_aggregated` row, so nothing forces a second file in its \
         tests/ into the binary — and under that flag the file is not compiled and does \
         not run: {offenders:?}. Either mount every suite with `#[path]` and take the \
         guard with it (crates/topo/tests/all.rs is the pattern), or keep the directory \
         to one file."
    );
}

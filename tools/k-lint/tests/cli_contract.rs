//! The CLI's THREE EXIT VOICES, pinned.
//!
//! The lint is a gate: a finding fails the CI row. That only works if
//! the three outcomes stay mechanically distinguishable — a finding, a
//! harness that could not run, and a clean sweep — so each gets an
//! executable pin here rather than living in prose. The discipline
//! message is asserted by stable substrings (the wording is polish;
//! the substance — "do NOT change the geometry", the numbered
//! recourse, "the one forbidden move" — is the contract).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::{Command, Output};

const HEADER: &str = "shape,predicate,margin,band_zero,band_escalate,outcome";

/// Writes `text` to a uniquely named file under the test's scratch dir
/// and returns the path.
fn csv(name: &str, text: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("k-lint-cli-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    path
}

fn run(args: &[&std::path::Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_k-lint"))
        .args(args)
        .output()
        .expect("k-lint binary runs")
}

fn stdout(o: &Output) -> String {
    String::from_utf8(o.stdout.clone()).unwrap()
}
fn stderr(o: &Output) -> String {
    String::from_utf8(o.stderr.clone()).unwrap()
}

/// VOICE (c): a clean sweep exits 0 and says so on stdout, with
/// nothing on stderr.
#[test]
fn clean_exits_zero() {
    // Definite, far above the band; and a zero at machine noise.
    let path = csv(
        "clean.csv",
        &format!(
            "{HEADER}\n\
             demo/az,volume_backstop,2.0,1e-9,1e-8,positive\n\
             demo/az,pm_census_ee_span,1e-16,1e-9,1e-8,zero\n"
        ),
    );
    let out = run(&[&path]);
    assert_eq!(out.status.code(), Some(0), "clean sweep exits 0");
    assert!(stdout(&out).contains("clean — no margin crowds a decision boundary"));
    assert_eq!(stderr(&out), "", "a clean run is silent on stderr");
}

/// VOICE (a): a finding FAILS, and the failure leads with the
/// interpretation discipline. This is the gate flip's whole content.
#[test]
fn findings_fail_with_the_discipline_message() {
    // An in-band (indeterminate) sample — rule 1, the #99 class — plus
    // a definite margin below the baseline floor.
    let path = csv(
        "findings.csv",
        &format!(
            "{HEADER}\n\
             demo/bracket,carrier_line_circle,2.315e-6,1e-6,1e-5,indeterminate\n\
             demo/bracket,volume_backstop,3.9e-6,1e-9,1e-8,positive\n"
        ),
    );
    let out = run(&[&path]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "findings exit 2 — nonzero, and distinct from harness breakage"
    );
    // The flags themselves still print, in full, on stdout.
    let so = stdout(&out);
    assert!(so.contains("2 samples, 2 flagged"), "summary line: {so}");
    assert!(so.contains("FLAG demo/bracket:carrier_line_circle"), "{so}");

    // The verdict is on stderr, so it survives a redirected stdout.
    let se = stderr(&out);
    for pin in [
        "GATE FAILED",
        "the margin distribution changed",
        "crowd a decision",
        "do NOT change the geometry",
        "evidence ABOUT THE MARGIN DISTRIBUTION",
        "not a geometry defect",
        "Recourse, in order",
        "docs/K-REPORT.md",
        "demote this row to advisory",
        "Changing geometry to get under a lint threshold is the one forbidden move",
    ] {
        assert!(se.contains(pin), "discipline message missing {pin:?}: {se}");
    }
    assert!(se.contains("2 margin(s)"), "the count is reported: {se}");
}

/// VOICE (b): harness breakage — a malformed row — is its OWN voice.
/// Same failing row, different meaning: the lint could not run, so it
/// says nothing about geometry and must not print the discipline text.
#[test]
fn malformed_csv_is_harness_breakage_not_a_finding() {
    let path = csv(
        "malformed.csv",
        &format!("{HEADER}\ndemo/az,volume_backstop,NOT_A_FLOAT,1e-9,1e-8,positive\n"),
    );
    let out = run(&[&path]);
    assert_eq!(out.status.code(), Some(1), "harness breakage exits 1");
    let se = stderr(&out);
    assert!(
        se.contains("malformed sweep row (harness breakage)"),
        "breakage names itself: {se}"
    );
    assert!(
        !se.contains("forbidden move") && !se.contains("GATE FAILED"),
        "breakage must not wear the findings voice: {se}"
    );
}

/// An unknown outcome string is breakage too (a sweep-format drift
/// that scored silently CLEAN would disarm the whole gate).
#[test]
fn unknown_outcome_is_harness_breakage() {
    let path = csv(
        "unknown-outcome.csv",
        &format!("{HEADER}\ndemo/az,volume_backstop,2.0,1e-9,1e-8,perhaps\n"),
    );
    let out = run(&[&path]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("harness breakage"));
}

/// A missing file, and no arguments at all, are breakage as well.
#[test]
fn missing_input_is_harness_breakage() {
    let missing = csv("exists.csv", "").with_file_name("nope.csv");
    let out = run(&[&missing]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("cannot read"));

    let out = run(&[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(stderr(&out).contains("usage: k-lint"));
}

/// Findings anywhere in a MULTI-FILE run fail the whole run, and every
/// file is scanned first: the CI row lints all three ε rows in one
/// invocation, so an early clean file must not short-circuit the
/// verdict and a late one must not erase it.
#[test]
fn findings_in_any_file_fail_the_run() {
    let clean = csv(
        "multi-clean.csv",
        &format!("{HEADER}\ndemo/az,volume_backstop,2.0,1e-9,1e-8,positive\n"),
    );
    let dirty = csv(
        "multi-dirty.csv",
        &format!("{HEADER}\ndemo/az,volume_backstop,0.0,1e-6,1e-5,invalid\n"),
    );
    let out = run(&[&clean, &dirty, &clean]);
    assert_eq!(out.status.code(), Some(2));
    let so = stdout(&out);
    assert_eq!(
        so.matches("1 samples, 0 flagged").count(),
        2,
        "both clean files were scanned and reported: {so}"
    );
    assert!(stderr(&out).contains("1 margin(s)"));
}

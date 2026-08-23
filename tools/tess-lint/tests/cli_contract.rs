//! The CLI's THREE EXIT VOICES, pinned — `k-lint`'s contract test,
//! same shape and same reason.
//!
//! This lint is a gate: a finding fails the CI row. That only works if
//! the three outcomes stay mechanically distinguishable — a finding, a
//! harness that could not run, and a clean comparison. The fourth
//! thing pinned here is the one that is easy to lose: WITHOUT a
//! baseline the tool is a report, and a report is never a verdict, so
//! it exits 0 no matter how large the slack it prints.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::{Command, Output};
use tess_lint::EXPECTED_HEADER as HEADER;

/// A one-plane, one-NURBS scene. `tris` is the wall's triangle count
/// and `span_opt` the cheapest per-cell grid, which together move the
/// two gate rules independently — and, at `span_opt = 0`, produce the
/// unreadable denominator this file's harness-voice row needs.
///
/// **The twin of `tess_lint`'s own test fixture**, deliberately: an
/// integration test cannot see a `#[cfg(test)]` item, so the two
/// cannot share one. Keep them in step.
fn scene(tris: usize, span_opt: f64) -> String {
    format!(
        "{HEADER}\n\
         s/b,0,plane,2e-3,4,,,,,,,,,,,,,,,,,,,,,,,\n\
         s/b,1,nurbs,2e-3,{tris},0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,2e0,3e0,4,\
         1e2,2e2,5e1,{span_opt:e},1e-4,5e-5,99,2,1,0,3e0\n"
    )
}

fn csv(name: &str, text: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("tess-lint-cli-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    path
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_tess-lint"))
        .args(args)
        .output()
        .expect("tess-lint binary runs")
}

fn out_of(o: &Output) -> String {
    String::from_utf8(o.stdout.clone()).unwrap()
}
fn err_of(o: &Output) -> String {
    String::from_utf8(o.stderr.clone()).unwrap()
}

/// VOICE (c): an unmoved sweep exits 0, says so on stdout, and puts
/// nothing on stderr.
#[test]
fn an_unmoved_sweep_exits_zero() {
    let base = csv("clean-base.csv", &scene(100, 2.5e1));
    let out = run(&[base.to_str().unwrap(), "--baseline", base.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0));
    assert!(out_of(&out).contains("clean"), "{}", out_of(&out));
    assert!(err_of(&out).is_empty(), "{}", err_of(&out));
}

/// VOICE (a): a moved budget exits 2, names the scene on stdout, and
/// puts the discipline on stderr — where a redirected stdout cannot
/// swallow the reason the row is red.
#[test]
fn a_grown_scene_exits_two_with_the_discipline_on_stderr() {
    let base = csv("grow-base.csv", &scene(100, 2.5e1));
    let fresh = csv("grow-fresh.csv", &scene(400, 2.5e1));
    let out = run(&[
        fresh.to_str().unwrap(),
        "--baseline",
        base.to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out_of(&out).contains("FINDING s/b"), "{}", out_of(&out));
    let e = err_of(&out);
    for phrase in [
        "GATE FAILED",
        "Do NOT coarsen delta",
        "re-cut the baseline",
        "vanished",
    ] {
        assert!(e.contains(phrase), "stderr missing {phrase:?}: {e}");
    }
}

/// VOICE (b): harness breakage — a format drift is NOT a geometry
/// finding, and must not be reportable as one.
#[test]
fn a_drifted_header_exits_one_not_two() {
    let bad = csv(
        "drift.csv",
        &scene(100, 2.5e1).replacen("span_opt_cells", "span_best_cells", 1),
    );
    let out = run(&[bad.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1), "drift is harness breakage");
    assert!(
        err_of(&out).contains("harness breakage"),
        "{}",
        err_of(&out)
    );
}

/// VOICE (b) again, on the case the voices exist to separate. The
/// gate fires only on growth, so a denominator resolved in band would
/// be its own pass value: an unreadable `span_opt_cells` must reach
/// exit 1, NOT the clean 0 it would reach if the lint answered a
/// broken measurement with a number.
#[test]
fn an_unreadable_denominator_exits_one_not_zero() {
    let base = csv("broken-base.csv", &scene(100, 2.5e1));
    let fresh = csv("broken-fresh.csv", &scene(100, 0.0));
    let out = run(&[
        fresh.to_str().unwrap(),
        "--baseline",
        base.to_str().unwrap(),
    ]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a zero denominator is harness breakage, never a clean gate: {}",
        out_of(&out)
    );
    assert!(
        err_of(&out).contains("span_opt_cells"),
        "the message names the column: {}",
        err_of(&out)
    );
}

#[test]
fn no_input_exits_one_with_usage() {
    let out = run(&[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(err_of(&out).contains("usage"), "{}", err_of(&out));
}

#[test]
fn an_unreadable_file_exits_one() {
    let out = run(&["/nonexistent/tess-budget.csv"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(err_of(&out).contains("cannot read"), "{}", err_of(&out));
}

/// A REPORT IS NOT A VERDICT: without `--baseline` the tool prints
/// whatever slack it finds — here a 2x held span gain and a 100x
/// recoverable split — and still exits 0. The absolute factors are
/// known and tracked in #320; making them fail a row would only
/// pressure someone to coarsen δ, which is the one move the
/// discipline forbids.
#[test]
fn a_report_without_a_baseline_never_fails() {
    let path = csv("report.csv", &scene(100_000, 1.0));
    let out = run(&[path.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0), "a report is not a verdict");
    let o = out_of(&out);
    assert!(o.contains("no gate ran"), "{o}");
    assert!(o.contains("2.0x"), "the held span gain is reported: {o}");
    assert!(o.contains("100.0x"), "the split factor is reported: {o}");
}

/// The report's own arithmetic, end to end through the binary: the
/// attribution line comes from the rows, not from a remembered
/// constant. 4 planar triangles + 996 on the wall = 1000, of which
/// the Hessian-sized lane carries 99.6% while being half the faces —
/// which is the shape of the whole finding.
#[test]
fn the_report_attributes_the_mesh_to_the_hessian_sized_lane() {
    let path = csv("attrib.csv", &scene(996, 2.5e1));
    let o = out_of(&run(&[path.to_str().unwrap()]));
    assert!(o.contains("2 faces, 1000 triangles"), "{o}");
    assert!(
        o.contains("1 (50.0% of faces) carrying 996 triangles (99.6% of the mesh)"),
        "{o}"
    );
}

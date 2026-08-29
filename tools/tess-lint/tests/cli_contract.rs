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
        "{HEADER}\n{}\
         s/b,1,nurbs,2e-3,{tris},0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,2e0,3e0,4,\
         1e2,2e2,5e1,{span_opt:e},1e-4,5e-5,99,2,1,0,3e0\n",
        unsized_row(0, "plane", 4)
    )
}

/// A row on a lane that sizes nothing. The empty tail is COUNTED from
/// the header, never typed: a schema change must not turn a fixture
/// into a short row that fails for the wrong reason.
fn unsized_row(face: usize, chart: &str, tris: usize) -> String {
    let blanks = ",".repeat(HEADER.split(',').count() - 5);
    format!("s/b,{face},{chart},2e-3,{tris}{blanks}\n")
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

/// VOICE (a) on the rule that has no number: an ordinal the join
/// cannot call one face, in a scene where that costs the slack rule a
/// comparison. The line must name the COLUMN that disagreed and both
/// readings — "the roster moved" is a verdict with no evidence under
/// it, and "the sizing schedule got wastefuller" would be a mis-join
/// reported in the voice of a measurement.
#[test]
fn a_re_key_that_costs_a_comparison_exits_two_and_names_the_column() {
    let base = csv("rekey-base.csv", &scene(100, 2.5e1));
    // The wall rerouted off the sized lane at the same ordinal,
    // carrying the same triangles: nothing scene-granular moves.
    let fresh = csv(
        "rekey-fresh.csv",
        &format!(
            "{HEADER}\n{}{}",
            unsized_row(0, "plane", 4),
            unsized_row(1, "cylinder", 100)
        ),
    );
    let out = run(&[
        fresh.to_str().unwrap(),
        "--baseline",
        base.to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(2), "{}", out_of(&out));
    let o = out_of(&out);
    assert!(
        o.contains("FINDING s/b face 1: a different face: chart nurbs -> cylinder"),
        "{o}"
    );
    assert!(!o.contains("wastefuller"), "not a measurement's voice: {o}");
    assert!(err_of(&out).contains("re-keyed face"), "{}", err_of(&out));
}

/// …and the other side of that call, which is the one a mutation slips
/// through: the same re-key in a scene with no sized face costs no
/// comparison, so it is a NOTE — stdout, exit 0, nothing on stderr.
/// Rule 1 still runs over the scene's total, and a gate that reds
/// where it gates nothing is one people learn to route around.
#[test]
fn a_re_key_that_costs_nothing_is_a_note_and_exits_zero() {
    let plane_scene = |extra: &str| format!("{HEADER}\n{}{extra}", unsized_row(0, "plane", 4));
    let base = csv(
        "note-base.csv",
        &plane_scene(&unsized_row(1, "cylinder", 10)),
    );
    let fresh = csv("note-fresh.csv", &plane_scene(""));
    let out = run(&[
        fresh.to_str().unwrap(),
        "--baseline",
        base.to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(0), "{}", out_of(&out));
    let o = out_of(&out);
    assert!(o.contains("note: s/b face 1:"), "{o}");
    assert!(o.contains("0 finding(s)"), "{o}");
    assert!(
        err_of(&out).is_empty(),
        "a note never reds: {}",
        err_of(&out)
    );
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

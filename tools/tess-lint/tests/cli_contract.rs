//! The CLI's THREE EXIT VOICES, pinned — `k-lint`'s contract test,
//! same shape and same reason.
//!
//! This lint is a gate: a finding fails the CI row. That only works if
//! the three outcomes stay mechanically distinguishable — a finding, a
//! harness that could not run, and a clean comparison. Three voices,
//! five finding KINDS. Also pinned here, each because it is easy to
//! lose:
//!
//! * WITHOUT a baseline the tool is a report, and a report is never a
//!   verdict, so it exits 0 no matter how large the slack it prints.
//! * Rule 5 speaks in the harness-breakage REGISTER while exiting as a
//!   FINDING — the one place a voice and an exit code deliberately
//!   disagree, for the reason `main.rs`'s module docs give.
//! * Which failure LEAD prints, the both-kinds case included.
//! * That the recourse quotes `docs/TESS-BUDGET.md` verbatim. **Only
//!   half of that pin is here**: this file asserts the string the
//!   binary emits; nothing in the tree asserts the document still
//!   contains it. There is no cross-file gate, the doc is the other
//!   half, and a reflow of that sentence would break the quote in
//!   silence — said plainly rather than left looking pinned.

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
    let first = HEADER
        .split(',')
        .position(|c| c == "u0")
        .expect("the header names the first NURBS column");
    let blanks = ",".repeat(HEADER.split(',').count() - first);
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
    // COUNTED, like the findings: an uncounted channel is where
    // findings go to be forgotten.
    assert!(o.contains("1 note(s)"), "{o}");
    assert!(o.contains("s/b face 1: in the baseline, absent"), "{o}");
    assert!(o.contains("0 finding(s)"), "{o}");
    assert!(
        err_of(&out).is_empty(),
        "a note never reds: {}",
        err_of(&out)
    );
}

/// VOICE (a) on rule 5, in the voice that separates it from the other
/// four: a scene the baseline does not cover reds the row, and the
/// reason it gives is a comparison that did not happen. The lead must
/// NOT tell this author not to coarsen delta — nothing about their
/// scene's budget was read — and the recourse must name the steps.
#[test]
fn an_uncovered_scene_exits_two_in_the_harness_voice() {
    let base = csv("uncov-base.csv", HEADER);
    let fresh = csv("uncov-fresh.csv", &scene(100, 2.5e1));
    let out = run(&[
        fresh.to_str().unwrap(),
        "--baseline",
        base.to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(2), "{}", out_of(&out));
    let o = out_of(&out);
    assert!(
        o.contains("FINDING s/b: in this sweep (104 triangles), not in the baseline"),
        "{o}"
    );
    assert!(o.contains("cannot compare what the baseline lacks"), "{o}");
    assert!(!o.contains("note(s)"), "rule 5 is never a note: {o}");
    let e = err_of(&out);
    for phrase in [
        "GATE FAILED",
        "could not COMPARE 1 scene(s)",
        "scripts/tess_budget_sweep.sh",
        "check the diff is ADDITIVE",
        // Quoted from docs/TESS-BUDGET.md. This half of the pin is the
        // only half there is — see the module docs.
        "restores coverage, it does not verify it",
    ] {
        assert!(e.contains(phrase), "stderr missing {phrase:?}: {e}");
    }
    assert!(
        !e.contains("Do NOT coarsen delta"),
        "the measurement lead does not belong on a comparison that never ran: {e}"
    );
}

/// BOTH leads, when both kinds fire. The two are independent arms and
/// not an `if`/`else`: a sweep that adds one scene while another grows
/// is an ordinary PR, and printing only the first lead would leave the
/// second finding's author with advice about the other one's problem.
/// Written because flipping the second arm to `else if` passes every
/// other test in this file.
#[test]
fn a_sweep_that_grows_one_scene_and_uncovers_another_prints_both_leads() {
    // One scene in both sweeps, four times the triangles; a second
    // scene the fresh sweep alone has.
    let base = csv("both-base.csv", &scene(100, 2.5e1));
    let fresh = csv(
        "both-fresh.csv",
        &format!(
            "{}{}",
            scene(400, 2.5e1),
            scene(50, 2.5e1)
                .replace("s/b", "new/scene")
                .replace(HEADER, "")
        ),
    );
    let out = run(&[
        fresh.to_str().unwrap(),
        "--baseline",
        base.to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(2), "{}", out_of(&out));
    let o = out_of(&out);
    assert!(o.contains("2 finding(s)"), "{o}");
    assert!(o.contains("FINDING s/b: triangles"), "{o}");
    assert!(o.contains("FINDING new/scene:"), "{o}");
    let e = err_of(&out);
    assert!(
        e.contains("could not COMPARE 1 scene(s)"),
        "the uncovered lead is missing: {e}"
    );
    assert!(
        e.contains("Do NOT coarsen delta"),
        "the measurement lead is missing: {e}"
    );
    // Order matters only in that the mechanical fix comes first; what
    // must never happen is one arm swallowing the other.
    assert!(
        e.find("could not COMPARE").unwrap() < e.find("Do NOT coarsen delta").unwrap(),
        "{e}"
    );
}

/// The cut is what tells rule 5's two readings apart, so the gate
/// prints it — and says so when the baseline records none, rather
/// than leaving the reader to assume one.
#[test]
fn the_gate_names_the_tree_the_baseline_was_cut_from() {
    let stamped = csv(
        "cut-base.csv",
        &format!(
            "# tess-budget-cut: 1a2b3c4d5e6f 2026-08-30T12:00:00+00:00\n{}",
            scene(100, 2.5e1)
        ),
    );
    let out = run(&[
        stamped.to_str().unwrap(),
        "--baseline",
        stamped.to_str().unwrap(),
    ]);
    assert_eq!(out.status.code(), Some(0), "{}", err_of(&out));
    assert!(
        out_of(&out).contains("cut at 1a2b3c4d5e6f (2026-08-30T12:00:00+00:00)"),
        "{}",
        out_of(&out)
    );

    let bare = csv("nocut-base.csv", &scene(100, 2.5e1));
    let out = run(&[bare.to_str().unwrap(), "--baseline", bare.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0), "{}", err_of(&out));
    assert!(out_of(&out).contains("no recorded cut"), "{}", out_of(&out));
}

/// VOICE (b) on the provenance line: a cut the lint cannot read is
/// the sweep and the lint disagreeing about the format, which is
/// harness breakage — never a silently absent cut.
#[test]
fn a_malformed_cut_line_exits_one() {
    let bad = csv(
        "badcut.csv",
        &format!("# tess-budget-cut: nonsense\n{}", scene(100, 2.5e1)),
    );
    let out = run(&[bad.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1), "{}", out_of(&out));
    assert!(
        err_of(&out).contains("harness breakage"),
        "{}",
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

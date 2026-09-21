---
id: gating-nextest-jobs-discard-every-passing-tests-stdout
kind: issue
title: The twelve gating nextest jobs discard every passing test's stdout, so both of the tree's announce-only idioms reach no reader on the gate
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Filed by S-TINT unit TINT-2, which took the half of the repair that is
inside its own fence (`crates/*/tests/**`, `crates/test-utils/**`) and
is filing the workflow half here rather than taking it.

## The fact

`cargo-nextest` captures a passing test's stdout and `--success-output`
defaults to `never`. In `.github/workflows/ci.yml`, **`--success-output`
appears exactly once** — on the `cargo nextest run -p viewer --features
app` GPU-smoke row, whose own comment says it is there for the smoke
row's adapter. Every other `cargo nextest run` in that file, including
both archived matrix invocations that make up the twelve gating
`test (…)` jobs, passes no such flag and sets no
`NEXTEST_SUCCESS_OUTPUT`; `find . -name 'nextest.toml'` returns nothing,
so no committed profile sets a default either.

Measured on this tree (`cargo nextest run -p viewer -E
'test(/lane_skipped/)'`, no flag) — four marker rows whose entire body
is a `println!`:

```
        PASS [   0.005s] (1/4) viewer app_lane_skipped_no_app_feature_coverage_here
        PASS [   0.006s] (2/4) viewer::all panel_display::app_lane_skipped_no_panel_display_coverage_here
        PASS [   0.007s] (3/4) viewer::all error_display::app_lane_skipped_no_error_display_coverage_here
        PASS [   0.006s] (4/4) viewer::all chrome_labels::app_lane_skipped_no_chrome_coverage_here
     Summary [   0.009s] 4 tests run: 4 passed, 576 skipped
```

Not one body line appears. Adding `--success-output immediate` to the
same command prints all four.

## Why it is CIW's

Two in-tree idioms are announcements and nothing else:

- `test_utils::vacuity::stood_down` (`crates/test-utils/src/vacuity.rs`),
  a `println!` called from 22 sites in 10 test files. It is called from
  inside a row that PASSES, so neither the line nor the row's name says
  a stand-down happened. On the gate it is 100% invisible.
- the ten `#[cfg]`-gated `*_lane_skipped_*` marker rows
  (`grep -rn "lane_skipped" --include=*.rs crates/`). Their NAME reaches
  the PASS list and is read; their `println!` body is not.

TINT-2 dealt with the bodies by making them carry nothing that can go
stale, and stated in `vacuity.rs` that a stand-down has no reader on the
gate. **What it did not do, because the file is CIW's, is give the
bodies a reader.** The options and their cost:

1. `--success-output immediate` (or `NEXTEST_SUCCESS_OUTPUT`) on the
   gating jobs. Prints EVERY passing test's stdout, which on a
   ~3000-row matrix job is a log-volume decision, not just a repair —
   several rows in this tree print evidence lines unconditionally
   (`vacuity::Exposure::report`, the `[r1] …` diagnostics in
   `crates/editor-core/tests/m10_5_r1_probes_interval.rs`).
2. A committed `.config/nextest.toml` profile. Same volume effect, and
   it would also change every local run.
3. Nothing, and the announcements stay local-only. This is the status
   quo and is defensible — but it should be a decision on the record
   rather than the default nobody chose, because two idioms in the tree
   are written as though a gate reader exists.

**Not argued on cost**: `--success-output` changes no test's runtime.

`crates/test-utils/src/vacuity.rs` and the nine markers TINT-2 could
reach now say plainly that the body reaches a local run and nothing
else, so nothing in the tree overstates the channel while this is open.

## If this is repaired, the prose that states it has to move too

**No count is given here on purpose** — a hand-kept list of the places a
fact is written is the same defect this row's own subject is about. The
list is derivable, and the command is the record:

```
grep -rn 'success-output\|passing test' --include=*.rs crates/ \
  | grep -v '^crates/test-utils/src/vacuity.rs'
```

plus `crates/test-utils/src/vacuity.rs` itself, whose section *What a
passing row prints reaches nobody on the gate* is the **single** home of
the claim: TINT-2 moved the marker copies behind
`test_utils::loud_skip_marker!`, so the nine files that each stated it now
state nothing and the macro states it once. What the grep still finds
outside that section are the rows that argue their own posture from it
— at this writing `crates/geom-brep/tests/m5_pr7_ssi.rs`,
`crates/geom-brep/tests/r2_cert6_probes.rs` and
`crates/step-import/tests/cert5_r1_import_probes.rs`, each in its own
words because each is about a different row.

One site is outside TINT's fence and outside this grep's crate list:
`crates/viewer/src/lib.rs`, filed as
`work/view/viewer-lib-marker-claims-the-log-carries-its-sentence`.

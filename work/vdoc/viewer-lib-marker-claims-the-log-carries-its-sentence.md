---
id: viewer-lib-marker-claims-the-log-carries-its-sentence
kind: issue
title: viewer/src/lib.rs's loud-skip marker says to read it as a sentence the log carries; the gating log carries its name, not its sentence
status: open
opened: 2026-09-15
priority: P4
cost: E
---



Filed by S-TINT unit TINT-2. `crates/viewer/src/lib.rs` is `src/`, which
is outside that unit's fence (`crates/*/tests/**`,
`crates/test-utils/**`), so this is reported rather than taken.

## The finding

`crates/viewer/src/lib.rs` carries
`fn app_lane_skipped_no_app_feature_coverage_here`, the tree's worked
example of a loud-skip marker with no hand-kept enumeration — the
conversion held and this row does not dispute it. What is wrong is one
sentence of the rustdoc above it:

> Read it as a sentence the log carries, and keep gating to the rows
> themselves.

**The gating log does not carry the sentence; it carries the name.**
Every gating `cargo nextest run` in `.github/workflows/ci.yml` passes no
`--success-output` (default `never`) and there is no `nextest.toml` in
the tree, so a passing test's stdout is discarded. The row's `println!`
body is read on a local run and nowhere else. Measured, same file's
marker among them:

```
$ cargo nextest run -p viewer -E 'test(/lane_skipped/)'
        PASS [   0.005s] (1/4) viewer app_lane_skipped_no_app_feature_coverage_here
     Summary [   0.009s] 4 tests run: 4 passed, 576 skipped
```

The same paragraph gets the load-bearing half right two sentences
earlier — *"its whole payload is its NAME appearing in the PASS list"* —
so the defect is a trailing sentence that contradicts it, not a wrong
model of the row.

## The repair

One sentence. TINT-2 put this wording on the nine markers it could
reach, so the tenth can match:

> **This row closes no gate and cannot fail, and only its NAME
> travels.** Every gating `cargo nextest run` discards a passing test's
> stdout, so the line below is read on a local run and nowhere else.

The `#[cfg]`-gated markers as a class are
`work/tint/loud-skip-marker-is-a-hand-kept-idiom`; the channel itself is
`work/ciw/gating-nextest-jobs-discard-every-passing-tests-stdout`.

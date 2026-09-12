---
id: perf-history-writers-are-guarded-three-different-ways
kind: issue
title: the four docs/perf-data histories are written by three kinds of emitter with three different guard sitings
status: open
opened: 2026-09-11
refs: [criterion-selftest-nightly-only, criterion-selftest-fixture-is-one-scalar-in-five-fields, 2330]
---


Opened by the unit that moved `scripts/criterion-emit.py --selftest` into the
merge gate, which answered the question for ONE history and found the other
three answer it differently. `docs/perf-data/` holds four directories and this
is what writes and guards each. The lane is PERF's record; the emitters and
their wiring are CIW's, which is why this is filed here.

| history | emitter | its guard | where the guard runs |
| --- | --- | --- | --- |
| `criterion/` | `scripts/criterion-emit.py` | `--selftest` | per-PR (`ci.yml` `discipline`, mirrored) **and** nightly, as of PR 2330 |
| `opt-level/` | `scripts/opt-level-calibrate.py record` | `--selftest` | per-PR only (`ci.yml` `discipline`, mirrored); nothing before the append |
| `rebuild-latency/` | `crates/editor-core/tests/m4_pr8_latency.rs`'s `emit()`, driven by `CAD_LATENCY_EMIT` | two **non-`#[ignore]`d** tests in the same file — the corpus/manifest pin at `:322` and the cpuinfo degradation test at `:646` | per-PR, as ordinary workspace tests |
| `sccache-trial/` | none live — the raw readings of a closed trial (`docs/CI-MINUTES-2026-08.md`'s F4) | n/a | n/a |

In all three live cases the nightly's inline shell does only the COMMIT — the
`git fetch`/`reset --hard`/`push` retry loop (`nightly.yml:849-880` for
rebuild-latency, and the same shape in the criterion and opt-level jobs). The
emitter is always a program.

## What is actually uneven

1. **Where the guard sits relative to the append.** Only `criterion/` has one
   immediately before the write. `opt-level/` has none there —
   `work/ciw/calibrator-record-writes-without-its-selftest` holds that.
2. **What KIND of guard it is.** Two are script `--selftest` modes named by
   hand in both halves of CI; the third is a cargo test, which no
   `check-ci-mirror-parity.py` claim can see, because claim 4's population is
   `scripts/` and `demos/` paths. A guard that moves out of a `--selftest`
   into a test module leaves that checker's population silently.
3. **What they hold.** Measured for `criterion/` at
   `criterion-selftest-fixture-is-one-scalar-in-five-fields`; unmeasured for
   the other two. The Rust one is the strongest of the three on the shared
   cpuinfo parser and is the only copy that drives a synthetic parse
   (`calibrator-cpuinfo-parser-selftest-cannot-see-a-broken-parse`).

## What this is NOT

Not a call to unify them. A Rust measurement's emitter belongs in Rust and a
cargo test is the right guard for it. What the table is for is that the
argument *"a guard over an append-only history belongs in the merge gate"* was
settled once, for one history, and the other two were never asked — so the
next person to move one has a reading of the tree rather than one row's
precedent.

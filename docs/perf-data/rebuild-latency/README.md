# rebuild-latency timing history

One file per merge to `main`, named `<epoch-seconds>-<short-sha>.json`
so a lexicographic sort is a chronological one. Written and committed
by ci.yml's `rebuild latency (reporting)` job, on a hosted runner.

**Append-only.** A run adds a filename; it never edits an existing one.
That is what makes it conflict-free under concurrent merges (two runs
write different names), what makes it survive the workflow's
`cancel-in-progress: true` (a cancelled run drops its own entry and
nothing else), and what makes drift recoverable — an overwritten
reference would launder a slow regression, an accumulating one cannot.

## What reads it

`crates/editor-core/tests/m4_pr8_latency.rs` diffs its `vs base`
columns against the **newest** entry: on a PR that is `main`'s last
hosted measurement, on `main` it is the previous merge. An empty
directory is the bootstrap state, not an error — the row then prints
`n/a` and is pure reporting.

## What these numbers are, and are not

**REPORTING ONLY — measured, never gated** (M4-PLAN F8; PERF-PLAN
stays advisory). No CI row fails on a millisecond here. The assertions
in that test are the ε-independent structural ones, pinned separately
in `crates/editor-core/tests/baseline/rebuild-latency.json`.

They are **dev profile** (opt-level 0 for the kernel crates, opt-level
2 only for spade and mesh), which is what every other CI row builds —
comparable across rows and across PRs, and **never release-
representative**.

Read the `±` spread before believing a delta. Each figure is the median
of 5 runs in one process; a hosted 2-vCPU runner has a fat tail, and a
`vs base` move inside the spread is noise, not a regression.

## Why the history exists at all

The single committed baseline this replaces disqualified itself in its
own provenance: three developer-workstation refreshes disagreed by
90–98% on every row with contention ruled out, leaving a build/
environment hypothesis nobody captured side by side, and
`docs/PERF-SCAN-2026-08.md` §0 had to label every absolute-millisecond
claim in the repo provisional as a result. Every entry here records its
own `environment` block (runner, nproc, memory, toolchain, RUSTFLAGS,
`CARGO_PROFILE_*` overrides, debug-assertions, ε), so two entries that
disagree can be compared as environments rather than argued about.

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
own `environment` block (runner, nproc, memory, `cpu_model`, `cpu_flags`,
toolchain, RUSTFLAGS, `CARGO_PROFILE_*` overrides, debug-assertions, ε),
so two entries that disagree can be compared as environments rather than
argued about.

**What the block can now tell you**: which host CPU produced an entry, by
model string and by whether `avx2` / `avx512f` were available. Those two
fields are the ones that vary inside a hosted runner class — `nproc`,
memory, arch and toolchain are constant across the whole `ubuntu-latest`
pool, which is why the block used to be readable and still say nothing.
`cpu_model: null` with `cpu_flags: null` means `/proc/cpuinfo` was
unreadable; `cpu_flags: []` means it was read and neither extension was
present.

**What it still cannot.** *The two fields begin with the first entry
written after they were added. Every earlier entry carries the old field
set and stays unattributable — the history is append-only and nothing
retro-fits it.* Two boxes of the same model are still one reading, and
nothing here records what else the host was doing. One step change is
only half-covered: the runner class moved from 2 vCPU / 7 GB to 4 vCPU /
16 GB on 2026-09-03 (`.github/workflows/ci.yml`), so `nproc` separates
the two eras and nothing separates the boxes within either. A `vs base`
delta that straddles that date is a property of the runner, not of the
tree.

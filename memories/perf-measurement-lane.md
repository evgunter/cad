---
name: Perf measurement lane
description: Where the repo's runtime numbers come from and why they are trustworthy now — hosted CI is the canonical producer of rebuild-latency timings, the history is append-only under docs/perf-data/, and structure is pinned separately from milliseconds.
type: operational
---

# Perf measurement lane

## The rule

**A committed timing is only worth anything if you know which box
produced it.** The repo learned this the expensive way and the lane is
built around it.

## What broke (2026-07 → 2026-08)

`crates/editor-core/tests/baseline/rebuild-latency.json` used to hold
both structure and milliseconds, refreshed by whoever ran
`CAD_LATENCY_BASELINE_REFRESH=1` on their workstation. Three refreshes
of the same rows disagreed by more than any real change could explain.
Contention was ruled out by a verified-quiet re-run, leaving an untested
build/environment hypothesis nobody ever captured side by side. The file
ended up declaring cross-refresh comparison meaningless *in its own
provenance block*, and `docs/PERF-SCAN-2026-08.md` §0 had to label every
absolute-millisecond claim in the repo provisional as a result — almost
none of that scan's findings could carry a number at all.

## The lane now (2026-08-17)

Split along the rot line, because only one half could rot:

- **Structure** — `crates/editor-core/tests/baseline/rebuild-latency.json`:
  `about` / `nodes` / `cone` per corpus document. Machine-independent,
  hand-maintained, and **asserted** — a nodes/cone mismatch, a missing
  row, or a stale key fails `m4_pr8_latency::rebuild_latency_table`.
- **Timings** — `docs/perf-data/rebuild-latency/`, one file per merge
  to `main` named `<epoch>-<short-sha>.json` (lexicographic sort ==
  chronological). Written and committed by ci.yml's `rebuild latency
  (reporting)` job on a hosted runner. Every entry carries its own
  `environment` block: runner, nproc, memory, toolchain, RUSTFLAGS,
  `CARGO_PROFILE_*`, debug-assertions, ε.

**Append-only, never overwritten.** Two runs never write the same
filename, so concurrent merges cannot conflict and a cancelled run
drops only its own entry — and, the actual point, drift stays
recoverable. An overwritten reference launders a slow regression (5%
per PR over 20 PRs is 165% with no single flag); an accumulating one
cannot.

**Main pushes only.** On a PR branch the entry would be that PR's own
measurement, so its `vs base` column would diff against itself and
report nothing. PRs get an artifact instead, and their diff reads
against `main`'s last hosted measurement.

## Things to know before touching it

- **There is no baseline to refresh.** `CAD_LATENCY_BASELINE_REFRESH`
  is gone. A local run READS the history and reports against it; it
  cannot write to it. Your local milliseconds are not comparable with a
  runner's — that is the design, not a limitation.
- **Read the `±` column before believing a delta.** Each figure is a
  median over repeats (`m4_pr8_latency` owns the count); a hosted
  2-vCPU runner has a fat tail, and a `vs base` move inside the spread
  is noise.
- **Still REPORTING ONLY, still never gated** (M4-PLAN F8; PERF-PLAN
  advisory). No CI row fails on a millisecond. The assertions in that
  test are the ε-independent structural ones.
- **Not release-representative.** Dev profile, opt-level 0 for the
  kernel crates. Never quote these as release numbers.
- **The split does not resolve `disputed_measurement` retroactively** —
  the three workstation refreshes remain mutually incomparable. It
  makes the failure unable to recur.
- **Exact, deterministic data beats wall clock where it exists.**
  `docs/k-report-data/`'s predicate decision counts are
  machine-independent and immune to both the profile and contention
  problems; they are what localized the PERF-SCAN findings that had
  numbers at all. Reach for a counter before a stopwatch.

## The general shape (applies beyond this lane)

Renders went hosted for the same reason and with the same ruling
(#338 — see [[freecad-render-lane]]). When a measurement's producer is
a developer's box, the measurement decays into an argument. Move the
producer to one reproducible box class, record the environment on every
sample, and accumulate rather than overwrite.

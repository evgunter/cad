---
name: Perf measurement lane
description: Where the repo's committed runtime numbers come from and what may be done with them — hosted CI produces them, the history is append-only, and nothing gates on a millisecond
type: operational
---

**A committed timing is only worth anything if you know which box
produced it.** Workflow comments, benches and the `docs/perf-data/`
READMEs cite this file for the rules below.

- **Structure and timings live apart, because only one can rot.**
  Structure (`about` / `nodes` / `cone` per corpus document) is pinned
  in `crates/editor-core/tests/baseline/rebuild-latency.json` and
  ASSERTED. Timings accumulate under `docs/perf-data/`, one file per
  merge to `main`, each carrying its own `environment` block. The
  lane's full documentation is the header of
  `crates/editor-core/tests/m4_pr8_latency.rs`.
- **Append-only, never overwritten, main pushes only.** An overwritten
  reference launders a slow regression; an accumulating one cannot. On
  a PR the entry would diff against itself, so PRs get an artifact and
  read against `main`'s last hosted measurement.
- **There is no baseline to refresh.** A local run READS the history
  and reports against it; local milliseconds are not comparable with a
  runner's — that is the design.
- **Read the `±` column before believing a delta**; a hosted 2-vCPU
  runner has a fat tail, and a move inside the spread is noise.
- **Reporting only, never gated**, and **not release-representative**
  (dev profile, opt-level 0 for the kernel crates) — never quote these
  as release numbers.
- **Exact, deterministic data beats a wall clock where it exists**
  (e.g. `docs/k-report-data/`'s predicate decision counts): reach for a
  counter before a stopwatch.

The general shape, which applies beyond this lane: when a measurement's
producer is a developer's box, the measurement decays into an argument.
Renders went hosted for the same reason ([[freecad-render-lane]]).

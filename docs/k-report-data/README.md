# `docs/k-report-data/` — the committed K margin sweeps

Raw decision-margin telemetry: one row per predicate decision, columns
`shape,predicate,margin,band_zero,band_escalate,outcome`. They are the
**threshold provenance** for the large-K lint and the evidence behind
every distribution claim in `docs/K-REPORT.md`. Read that report for
what the numbers mean; this file is only a map of the directory, so a
reader who arrives by `grep` knows which era a row belongs to.

## What is here

| files | era | cut at | written up in |
|---|---|---|---|
| `eps-1e-{6,9,12}.csv` | M2, uncompressed | the original K = 10 study | K-REPORT §Methodology / §Results |
| `m4-eps-*.csv.gz` | M4 | 2026-07-26 | M4 addendum |
| `m5-eps-*.csv.gz` | M5, curved corpus | 2026-08-03 | M5 addendum |
| `m7-eps-*.csv.gz` | M7, current lint baseline | 2026-08-07 | M7 addendum (floor refresh) |

## The two rules that govern this directory

1. **These rows are what the sweep script wrote. Nothing is ever
   renamed, re-cut or back-filled in place.** Each file is a dated
   snapshot of a stated head, not a mirror of main. A baseline is
   re-cut — as a NEW file — when the *distribution* moves (a new floor,
   a filled gap, an ε-coupled family), never because a predicate was
   renamed.
2. **Nothing reads these files as a gate.** CI's `k-lint` runs
   `scripts/k_probe_sweep.sh` into a scratch dir and lints *that*; the
   committed files supply the thresholds in `tools/k-lint/src/lib.rs`
   and are never compared against. So a stale name here breaks nothing
   — it just has to be legible, which is what rule 1 and this file are
   for.

Together those mean a predicate name in this directory dates the row it
sits on, and the roster differs between eras in **both** directions:
later sweeps add names, and since #652 a later sweep also *drops* six.

## Worked example: `grep sector`

Eleven predicate names match `sector` in the M4, M5 and M7 files (the
M2-era `eps-*.csv` predates all of them), and they are three different
kinds of thing. Nothing on the row itself says which:

- **Retired at #652 (2026-08-19) — pre-#652 rows only.**
  `bool_sector_{arm,reflex,straight}` and
  `split_sector_{arm,reflex,straight}`. They were two spellings of one
  computation (`crates/topo/src/sector_shape.rs`, one body since #647)
  and are now the single set `sector_{arm,reflex,straight}`, which
  appears in **no committed file here** — every file predates the pool.
  The pooled names are deliberately new spellings rather than the 29:1
  majority `bool_sector_*`, so no row in this directory silently changes
  meaning. Full treatment: K-REPORT's census note (2026-08-19).
- **Still forked, correctly.** `bool_sector_{within,coplanar}` and
  `split_sector_{coplanar,extent}` are the `sector_face` twins and the
  face-extent arm — *different quantities*, still two implementations,
  the rest of smell-scan S5. These are not candidates for pooling.
- **Not a sector rung at all.** `split_bisector_side` matches the grep
  on the substring in *bisector*. It is the splitting lane's own
  subdivision-side predicate and keeps its lane prefix.

The general lesson, not the sector-specific one: **a predicate name in
this directory is only interpretable against the era of the file it is
in.** When a name changes, the change is recorded in K-REPORT as a
dated note and the old rows stay exactly as they were.

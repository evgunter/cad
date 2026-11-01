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

**M7 was still the current era at `c39a904e` (2026-09-08), and that
was checked rather than assumed.** A sweep at that tip re-derives both
calibrated constants' witnesses — `volume_backstop` at 4.79652e-5 and
`props_quad_converged` at 164.674·ε — **pointwise identical** to M7's
at all three ε rows; the zero side's ceiling (`pm_census_ee_span`,
5.32907e-15) is identical too, and nothing new landed in the gap
between them. So no new era was cut, even though the roster had grown
from 233 names to 281 and the corpus by 15%. K-REPORT's M11 addendum
is that reading and carries the extraction.

**This is a dated reading, not a standing guarantee.** Nothing re-takes
it: `k-lint (gate)` re-lints a fresh sweep on every merge but never
compares it to these files (rule 2), and no row asserts that M7's three
witnesses are still what K-REPORT says they are. The guard that would
belong beside `tools/k-lint/tests/threshold_provenance.rs` is filed
with its values and its one-line extraction in
`work/meter/k-report-era-witnesses-have-no-guard.md`. Until it lands,
a reader arriving after 2026-09-08 should re-run the sweep rather than
trust the paragraph above.

## The two rules that govern this directory

1. **These rows are what the sweep script wrote. Nothing is ever
   renamed, re-cut or back-filled in place.** Each file is a dated
   snapshot of a stated head, not a mirror of main. A baseline is
   re-cut — as a NEW file — when the *distribution* moves (a new floor,
   a filled gap, an ε-coupled family), never because a predicate was
   renamed.

   **This clause is the one home for that decision.** Concretely, at
   the current era, a re-cut is owed when any of these moves: the
   definite-side floor witness (`volume_backstop`, 4.79652e-5,
   excluding the ε-coupled family); the ε-coupled ratio's P0
   (`props_quad_converged`, 164.674 at the binding 1e-9 row); or the
   zero-side ceiling in the ambient band (`pm_census_ee_span`,
   5.32907e-15) — including the case where it crosses `band_zero/100`,
   which it sat 1.88× under at `c39a904e` and which is this
   distribution's nearest live edge. A rule-1 flag anywhere in the
   linted population is the same signal arriving through the gate
   instead. **Not triggers, and not for the same reason**: a rename is
   excluded outright, by the sentence above. A roster growth or a
   corpus growth is *unlisted* — neither excluded nor admitted — so it
   is decided by measuring the three witnesses, which is what the M11
   addendum did. `tools/k-lint/src/lib.rs`'s doc restates this rule in
   its own words; when the two disagree, this one is the rule.

   **The three quantities this clause names are re-read on every
   `k-lint (gate)` run**, value and witness NAME both, by
   `tools/k-lint/tests/threshold_provenance.rs`'s
   `the_m7_era_still_carries_the_witnesses_the_report_names`. What that
   buys is one direction only: the committed era still carries what
   this clause says it carries, so a re-cut that moved a witness cannot
   land while the prose still names the old one. It cannot say a re-cut
   is OWED — that needs a fresh sweep measured against these files, and
   rule 2 is why none runs there. So if the era is ever re-argued, this
   clause moves first and that row moves with it.
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

---
id: k-report-m11-reading-is-stale-and-unwatched
kind: issue
title: the M11 addendum's corpus figures are 39% stale eight days on and nothing re-takes them; the zero-side ceiling moved 12x
status: open
opened: 2026-09-16
priority: P4
cost: E
---



## What

`docs/K-REPORT.md`'s **M11 addendum** reports a sweep at `c39a904e`
(2026-09-08): 2 076 056 / 2 076 080 / 2 076 104 samples at
ε ∈ {1e-6, 1e-9, 1e-12} over a roster of 281 names, with the zero-side
ceiling `pm_census_ee_span` at 5.32907e-15.
`docs/k-report-data/README.md` carries the same reading and says of it
*"This is a dated reading, not a standing guarantee. Nothing re-takes
it"* — which is accurate, and this row is what that sentence predicted.

## Finding

**A sweep eight days later disagrees with three of those four figures.**
Run at `89c8766` (2026-09-16, `origin/main`'s merge base for
`instr/u12-last-round-roster`) with the same command the addendum
names — `scripts/k_probe_sweep.sh <outdir>`, dev profile, the script's
own `feats_for`:

| figure | M11 (`c39a904e`) | re-take (`89c8766`) |
| --- | --: | --: |
| samples, 1e-6 / 1e-9 / 1e-12 | 2 076 056 / 080 / 104 | 1 263 818 / 826 / 838 |
| distinct predicate names | 281 | 277 |
| `volume_backstop` min \|m\| | 4.79652e-5 | 4.796516e-5 |
| `props_quad_converged` P0 (1e-9) | 164.674 | 164.674410 |
| `pm_census_ee_span` max (zero side) | 5.32907e-15 | 4.440892e-16 |

**The verdict is unchanged and no re-cut is owed**, which is why this is
a row and not an alarm. `tools/k-lint` scores the re-take clean at all
three ε rows (0 flags, rules 1/2/3), both calibration witnesses are
pointwise identical to M7's, and `docs/k-report-data/README.md` rule 1
names the ceiling as a re-cut trigger only where it CROSSES
`band_zero/100` — this move is a decade further away from that edge, not
toward it.

**What is a finding is that the corpus lost 39% of its K population and
four names in eight days and nobody read it.** The README's rule 1 calls
a roster or corpus change *"unlisted — neither excluded nor admitted"*,
so nothing owes a re-cut and nothing owes a reading either; the M11
addendum's figures are simply wrong for anyone who arrives at them now,
and the document does not say so at the figures. A reader checking
whether M7 is still the current era reads a paragraph whose sample
counts are off by 800 000.

**Two candidate dispositions, and they are not the same unit.** Either
the addendum's figures get a re-take with today's numbers beside them
(cheap, and stale again in a week), or the thing the addendum is FOR —
"M7 is still the current era" — gets the register
`docs/k-report-data/README.md` says it has not got, sited where
`tools/k-lint/tests/threshold_provenance.rs`'s
`the_m7_era_still_carries_the_witnesses_the_report_names` already
guards the committed era's own half. The second cannot be a committed
-file test: its subject is a FRESH sweep, which only the `dev-probe`
gate produces.

**A related pointer in the same paragraph is dangling.**
`docs/k-report-data/README.md:35` sends the reader to
`work/meter/k-report-era-witnesses-have-no-guard.md` for the guard's
values and extraction, and adds *"Until it lands"*. That row closed at
METER's sweep (`docs/DOC-LEDGER.md:1858`) and the directory is gone, so
both halves of the sentence are stale: the pointer resolves to nothing
without the ledger's recovery recipe, and part of what it asked for did
land (`the_m7_era_still_carries_the_witnesses_the_report_names`).
Whoever takes this row should say which half is still open rather than
leaving the sentence to imply both are.

**Confidence:** sure of every number in the table — each was computed
from the re-take's own CSVs and the committed `m7-eps-*.csv.gz`, and
the k-lint verdict is the tool's own output. Unsure WHY the population
moved: this row does not attribute it, and the attribution is the first
thing a unit here owes.

**Sweep and its blind spot.** The re-take is the whole corpus+demo leg
at all three ε rows, so nothing in that population is unexamined. It
does not cover the M2 leg (`<outdir>/m2/`) or the E6 driver leg
(`<outdir>/driver/`), which `scripts/k_probe_sweep.sh` writes beside
the linted CSVs and which the M11 addendum does not report either.

## Was

Found by INSTR unit 12 (`k-lint-last-round-is-eps-coupled-but-unrostered`),
which ran the sweep to settle whether `props_quad_last_round` has a
distribution and read these figures off the same run.

---
id: one-test-is-the-whole-ci-critical-path
kind: issue
title: One test is 85% of the CI critical path: a_tolerance_study_end_to_end_through_the_public_doors at eps = 1e-12
status: parked
opened: 2026-09-12
blocked_on: [m10-3-chamber-row-reads-ten-times-its-recorded-cost]
refs: [nextest-shard-count-needs-remeasure]
---

Found by the shard-count re-measure
(`nextest-shard-count-needs-remeasure`), which it decided: no shard
count can cut a leg below its heaviest single test, and on today's tree
one test is most of the critical leg.

## The row

`editor-core::all r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors`
(`crates/editor-core/tests/r2_m10_6_probes_interval.rs`, the row whose
doc comment reads *"The whole consumer walk, in one row"*).

On the interval lane at **ε = 1e-12** it is the slowest test in the
whole suite by a factor of four, and it lands in whichever shard the
count partition puts it in:

| what | reading |
|---|--:|
| the row's own nextest duration, 12 hosted runs | **346.2 – 659.6 s**, median ≈ 464 s |
| the leg that carries it | 372 – 740 s |
| the row's share of that leg | **85 – 96 %** |
| the next-heaviest test in the same row | `m10_3_r2_probes_interval::a_consumer_drives_a_two_parameter_document_at_four_widths`, 88 – 105 s |
| the same test at ε = default and ε = 1e-6 | not in its leg's top four (< 20 s) |

The twelve runs are named in `nextest-shard-count-needs-remeasure`'s
measured section; the leg that carries this row is the last job to
finish on every one of them, at every shard count from 2 to 6.

## This row is the HOME of those three figures

Per run, at each shard count the probes measured — the row's own
nextest duration, the leg wall that carries it, and the share between
them. Everything above is derived from this table:

| N | the row's own duration, three runs | leg that carries it | its share |
|--:|---|---|--:|
| 2 | 458.5 / 659.6 / 470.5 s | 537 / 740 / 554 s | 85 / 89 / 85 % |
| 3 | 582.9 / 441.4 / 441.5 s | 615 / 476 / 480 s | 95 / 93 / 92 % |
| 4 | 458.6 / 576.6 / 634.9 s | 509 / 619 / 689 s | 90 / 93 / 92 % |
| 6 | 434.5 / 346.2 / 632.9 s | 454 / 372 / 660 s | 96 / 93 / 96 % |

**Nowhere else states them.** They stood in five places on the day they
were written — two `.github/workflows/ci.yml` blocks,
`scripts/ci-filter.py`, `nextest-shard-count-needs-remeasure` and this
row — and three of the five already disagreed with this table, which is
exactly the defect the shard count was re-measured for. The other four
sites now carry the conclusion and a pointer here. This row is the home
rather than the closed item because the closed item is deleted with
S-TCOST's tracker directory and this one outlives it; the six-row
per-N LEG-wall table, the per-leg fixed cost and the conservation check
are that item's own and stay there.

The shard count itself is stated in prose in each of those sites, which
is the class `work/ciw/eps-klint-and-shard-counts-are-prose` holds
(open, 2026-09-11) — its hit list is for the eps rows, the k-lint
unifications and the shard count, and the pointer sentences this PR
leaves behind are further homes for the third.

## Nothing goes red when these figures stop being true

`docs/prompts/reviewer-style-lane.md` Q6 asks a claim resting on a
measurement for a mechanical guard, a scheduled re-measure, or a
written reason it can have neither. This is the third case, and the
reason belongs here because this is where the figures live.

- **Nothing computes with them.** The shard count is a literal in
  `.github/workflows/ci.yml`; no step reads a duration to choose it, so
  there is no threshold that could red. A guard would have to be a new
  job whose only output is an assertion about this row's share — and it
  would pay the eight minutes it measures on every firing.
- **The one register that exists does not re-take them.** Q6 says to
  check `ci.yml` for a register before accepting "unguardable", and
  there is one: `nightly.yml`'s `opt-level calibration` lane commits
  durations to `docs/perf-data/opt-level/` on every firing. It does not
  cover this. Its samples carry `"tolerance_eps": ""` — the compiled
  default, where this row is under 20 s — and its measured arms record
  one whole-arm total (`E`) for an unsharded full-suite run, not a
  per-leg or a per-test duration. Neither the ε = 1e-12 leg nor this
  test appears in anything it writes.
- **The re-open condition is a chain of parked rows.**
  `nextest-shard-count-needs-remeasure` is CLOSED and names this row as
  its re-open condition; this row is PARKED on
  `m10-3-chamber-row-reads-ten-times-its-recorded-cost`, which is
  parked itself. What would notice these figures going stale is a
  reader opening this file, not a run.

That is the whole of what is owed and it is bounded: the figures decide
one literal that is already at its floor value, and a reader who finds
them stale finds them stale in the row that owns them instead of in
four copies spread across a workflow and a script.

## Two things that are new here

**It is ε-gated, hard.** The row builds its guide at
`guide(2.0 - 1.0e-11)` — a bound 1e-11 from the round number — so at
ε = 1e-12 the drive has to resolve a gap finer than the bound's own
offset, and at ε = 1e-9 (compiled default) or 1e-6 it does not. That is
why the same row is a rounding error on two of the three ε rows and the
entire critical path on the third. Nothing measured before looked at
this row per-ε.

**Its own wall varies by 1.9x run to run** (346 s to 660 s on the same
tree, same runner class, twelve runs inside three hours). Whatever
dominates it is not a fixed amount of work, and a lane quoting a single
reading of it will be wrong by a factor of two in either direction.

## Why it is parked and not dispatched

The cause is almost certainly the one already diagnosed on
`m10-3-chamber-row-reads-ten-times-its-recorded-cost`: PR #1725's
symbolic identity tier, 95 % of the M10-3 drive's cost, filed to M10 as
`work/m10/symbolic-tier-costs-95-percent-of-the-m10-3-drive`. The two
rows named in that item — `the_driven_chamber_replays_bit_identically…`
(110 – 141 s per leg here) and
`the_band_and_uniform_drives_ship_the_same_leaf_partition` — are the
second and fourth heaviest tests on this lane in the same readings, so
this row is the same family seen from the CI side rather than a
separate finding. **It is not the same row**, and that is why it gets a
file: it is in a different suite, it is four times larger than either of
them, and it is the one that sets what a contributor waits for.

Whoever takes the M10 row should re-read this one with it: if the
symbolic tier is the cause, fixing it returns the whole interval lane
to ~30 s legs and re-opens the shard-count question with it. If it is
not, this row needs its own diagnosis and it is the biggest single
lever on CI wall clock that exists today.

## What it unblocks

`nextest-shard-count-needs-remeasure` closed at N=2 **because of this
row** and names it as its re-open condition. Every other row of both
test matrices responds cleanly to more shards — from N=2 to N=4, the
interval ε = default row's slowest leg goes 148 – 191 s to 103 – 131 s
and the three f64 rows go 64 – 83 s to 40 – 59 s (that item's leg-wall
table). None of it reaches the run's wall while this row runs for eight
minutes beside it.

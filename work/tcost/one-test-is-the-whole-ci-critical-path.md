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
| the row's share of that leg | **83 – 90 %** |
| the next-heaviest test in the same row | `m10_3_r2_probes_interval::a_consumer_drives_a_two_parameter_document_at_four_widths`, 88 – 105 s |
| the same test at ε = default and ε = 1e-6 | not in its leg's top four (< 20 s) |

The twelve runs are named in `nextest-shard-count-needs-remeasure`'s
measured section; the leg that carries this row is the last job to
finish on every one of them, at every shard count from 2 to 6.

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
test matrices responds cleanly to more shards (interval ε = default:
182 s → 115 s from N=2 to N=4; the three f64 rows: 68 – 83 s → 40 –
56 s). None of that reaches the run's wall while this row runs for
eight minutes beside it.

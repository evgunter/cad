---
id: m10-3-chamber-row-reads-ten-times-its-recorded-cost
kind: issue
title: m10_3_r1_probes_interval's chamber row reads 66-83 s hosted against TCOST-6's recorded 6.7 cpu-s
status: open
opened: 2026-09-11
---


Found 2026-09-11 while reading the nightly's `gated suites (ungated
re-take)` job for
`fuzz-depth-not-existence-run-everything-at-effort-1`. Filed as a
disagreement between two numbers, not as a diagnosis: this row has not
been re-measured against TCOST-6's method and the two readings may not
be measuring the same thing.

## The two numbers

`editor-core::all m10_3_r1_probes_interval::the_driven_chamber_replays_bit_identically_names_both_wall_flips_and_reports_containment`
is the slowest test in the gated set on both nights read, and by two
orders of magnitude over the next-but-one:

| night | run | this row | next row | rest of the 419 |
|---|---|--:|--:|--:|
| 2026-09-11 | 34585964715 | **83.310 s** (`SLOW`, > 60 s) | 23.490 s | ~0.007 s each |
| 2026-09-08 | 34211844759 | **65.896 s** (`SLOW`, > 60 s) | 18.102 s | ~0.007 s each |

Against it, TCOST-6's merge entry (`work/tcost/log.md`, 2026-09-03):
*"editor-core's three chamber heads (39.3 cpu-s on the interval lane)
are one labelled row at 6.7 cpu-s hosted"*, with the leaf budget retuned
`4096 -> CHAMBER_LEAVES = 1280` on a measured ε-invariant threshold.

**6.7 against 66-83 is a factor of ten to twelve**, and the second row in
the table (18-23 s, `the_band_and_uniform_drives_ship_the_same_leaf_partition`)
is the other member of the same suite, also large.

## What could make both readings honest

Listed so whoever takes this checks rather than assumes — the point of
filing it is that nobody knows which of these it is:

- **Different ε.** TCOST-6's reading is labelled "on the interval lane"
  without an ε row; the nightly job sets no `CAD_TOLERANCE_EPS`, so it
  runs the compiled default. If the row's cost is ε-sensitive, the two
  are different measurements and neither is wrong.
- **cpu-s against wall.** The nightly's figure is nextest's per-test
  wall. For a single-threaded test these are close, but the row's name
  says it replays a chamber, and if it parallelises internally the two
  diverge. `memories/test-suite-cost.md` warns that per-leg timings are
  not comparable without normalising.
- **A regression since 2026-09-03.** `CHAMBER_LEAVES` or the drive it
  feeds may have moved under the row. This is the case that would matter
  most and is the cheapest to rule out: the constant is a literal.
- **A different runner.** Both nights are the public 4-vCPU runner; the
  TCOST-6 reading predates nothing relevant, so this is the least likely.

## Why it matters beyond bookkeeping

Whatever the cause, on today's tree **this row alone is the whole
execution wall of the gated set** — 83.599 s of `Summary` against 83.310 s
for the row, the other 418 tests finishing inside its shadow. It is
therefore the entire price of
`fuzz-depth-not-existence-run-everything-at-effort-1`, and the single
biggest lever left on what the gated population costs anybody. If it is a
regression, fixing it makes that proposal free; if it is honest, the
proposal has one row to place deliberately rather than a policy problem.

It is also an argument for what the nightly job's own header already
asks for and has never had: someone reading that job's output. Two nights
of a row marked `SLOW` at over 60 s went unread until this week.

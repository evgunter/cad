---
id: the-line-meter-fuzz-row-reds-at-interval-far-from-the-centre
kind: issue
title: sweep line_meter_fuzz: the Interval meter's low end is looser than its rounding claim far from the centre (seed 0xa9b25b00fc22acb2)
status: closed
opened: 2026-10-12
priority: P1
cost: E
pr: 5025
branch: band/line-meter-fuzz-interval-far
closed: 2026-10-13
---

(Filed by PCERT from a review probe run on PR 4897, which does not touch
`blend`. Review R1 found it.)

## What

On main (reproduced at `818e51b3ee`), `sweep`
`blend::open::end_face::line_meter_fuzz::the_line_meter_reads_the_least_of_g_on_random_segments`
fails with `CAD_FUZZ_SEED=0xa9b25b00fc22acb2 CAD_FUZZ_EFFORT=1`, at
`crates/sweep/src/blend/open/end_face/line_meter_fuzz.rs:115`:

> Interval reads [−6.7618e-8, 1.1190e-11], f64 −9.7639e-13, the least G
> is −7.2647e-13: case 80: (2778.68, −3803.69) → (−3120.38, 4271.02),
> reach 1.1684, section Some((0.0, 0.37399, 0.37399)), floors
> [((0.80747, 0.58991), −0.11488), ((−0.96659, 0.25632), 0.33183)]

The assertion that fails is the second, the TIGHTNESS claim: the
enclosure's low end must be within `1e-12·(1 + |a| + scale)` of the f64
reading (about `1.5e-8` here, at `|a| ≈ 4.7e3` and `scale ≈ 1e4`). It
sits `6.8e-8` below. The SOUNDNESS half holds: `−6.76e-8 ≤ least`, so
the enclosure still contains the least `G`, and the f64 reading is
within its slack. So this is a meter reading looser than its rounding
claim far from the centre, not an enclosure miss.

The row draws a fresh seed per run, so a red can land on any PR that
runs it. That makes it a P1 for CI, whatever the fix turns out to be.

## Repair shape

Reproduce with the seed. Find what in the Interval meter's candidate
set loses about `1e-11` relative to `|a|` on a segment this far out
(commit `dc55e8558e`, "the line meter holds its gain at Interval and far
from the centre", is where the claim was made). Then either tighten
that candidate, or show the claim's `1e-12` rate is wrong for segments
like this one and restate it with its derivation. Pin the seed's case
as a fixed row.

## Closed

PR 5025. The meter read `G` at each candidate through the point
`q0 + e·s`, which carries `s` in both coordinates. A candidate's spread
therefore lowered even a term flat along the segment by about that
spread. Here the spread was a near-0/0 radial quotient, 10⁻⁷ wide. A
second seed (`0x1281277f8dfa215d`) red the same way through a
near-parallel floor pair. `g` now reads each term with `s` once, so the
`1e-12` rate stands. Both cases are pinned in `end_face.rs`'s tests.

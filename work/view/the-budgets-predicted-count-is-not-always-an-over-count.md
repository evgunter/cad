---
id: the-budgets-predicted-count-is-not-always-an-over-count
kind: issue
title: The display budget is not a cap, and the 1/delta law's two-sided error puts the drawn picture over it
status: open
opened: 2026-09-12
---

## The finding

`scene::TRIANGLE_BUDGET` is documented as a safety net rather than a
cap, and `scene::fit_delta` does not verify the δ it commits. What the
numbers show is that the gap between those two statements is real:
**the picture the budget sizes lands over the budget on four corpus
rows.**

`C = triangles · δ` is not monotone in δ, because the grid steps are
`ceil`ed per direction: measured on `tube_ring` (release, this box),
`C` reads 64.296 at δ = 4·10⁻⁴ and 64.106 at δ = 8·10⁻⁴ — the coarser
probe gives the SMALLER constant. A fit that reads `C` on the low side
of that wobble commits a δ finer than the budget wanted, and the
picture comes in over it:

| document | requested δ | drawn triangles | over |
|---|---:|---:|---:|
| `hollow_tube_ring` | 1·10⁻⁴ | 1 002 536 | +0.25% |
| `tube_ring` | 1·10⁻⁵ | 1 001 088 | +0.11% |
| `hollow_tube_ring` | 1·10⁻⁶ | 1 003 840 | +0.38% |
| `gallery_ring` | 1·10⁻⁶ | 1 003 820 | +0.38% |

Those are main's numbers, from before the probe ladder landed; the
ladder moves the readings a little and does not change the shape of it
(`gallery_ring` at 1·10⁻⁵ then draws 1 000 036).

## What is left to decide

**The rustdoc half is already fixed** (PERF side unit, PR "the display
probe is never larger than the picture"): `FittedDelta::predicted`
promised the error was one-sided — *"under-counting would mean drawing
a picture over budget, and this cannot do that by this route"* — and
now states it two-sided, with these numbers and their size. That was
prose describing the code's own behaviour, so the lane that measured
it corrected it.

**What remains is a design question, and it is VIEW's**: should
`TRIANGLE_BUDGET` be a cap the fit must land UNDER rather than a
number it aims at? Making it one costs something real — either a
verifying tessellation at the committed δ (the second full
tessellation `fit_delta` exists to avoid) or a margin on the solved δ
(cheap, but it coarsens every budget-bound picture whether or not the
wobble went the wrong way). Doing nothing is also defensible: 0.4%
over a safety net is not a freeze, and `tests/display_budget.rs`
already holds the drawn count to the budget with a margin rather than
as a cap. What is not defensible is leaving the question unasked,
because the budget's own docs are what a reader sizes other things
against.

## Where it came from

The PERF side unit on `fit-delta-probe-can-exceed-the-picture-it-
sizes`, which re-measured the fit's committed δ and the picture it
draws across the corpus before and after changing how the probe is
placed.

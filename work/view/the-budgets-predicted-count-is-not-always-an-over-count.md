---
id: the-budgets-predicted-count-is-not-always-an-over-count
kind: issue
title: The display budget's predicted count can come in UNDER the picture, so the drawn picture lands over the budget
status: open
opened: 2026-09-12
---


## The finding

`scene::fit_delta`'s docs state the direction of the 1/δ law's error
as a guarantee, at `FittedDelta::predicted`:

> Upper in both of the ways it is wrong, which is the direction that
> makes a budget safe. […] Over-counting cost means choosing a δ
> slightly coarser than needed; under-counting would mean drawing a
> picture over budget, and **this cannot do that by this route**.

It can, and on main it does. `C = triangles · δ` is not monotone in δ,
because the grid steps are `ceil`ed per direction: measured on
`tube_ring` (release, this box), `C` reads 64.296 at δ = 4·10⁻⁴ and
64.106 at δ = 8·10⁻⁴ — the coarser probe gives the SMALLER constant.
A probe that lands on the low side of that wobble predicts a δ finer
than the budget wanted, and the picture comes in over it. Measured
pictures at the δ main commits, against the 10⁶ budget:

| document | requested δ | drawn triangles |
|---|---:|---:|
| `hollow_tube_ring` | 1·10⁻⁴ | 1 002 536 |
| `tube_ring` | 1·10⁻⁵ | 1 001 088 |
| `hollow_tube_ring` | 1·10⁻⁶ | 1 003 840 |
| `gallery_ring` | 1·10⁻⁶ | 1 003 820 |

The overshoot is ~0.25%, so nothing about the budget's job is broken
— `TRIANGLE_BUDGET` is a safety net with its own margin, and
`tests/display_budget.rs` asserts the drawn count against the budget
with a 10% margin for exactly this reason. What is wrong is the
sentence: it promises a one-sided error the tessellator's own
quantization does not provide, and a reader sizing something else
against "the picture is never over the budget" would be reading a
guarantee that is not there.

## What a fix is

Either state the error as two-sided with its measured size (the
honest reading of the numbers above), or make the fit verify — which
costs the second tessellation the fit exists to avoid, so probably
not. This is a DOC claim about a ratified law, so it is the owner's
call, not a lane's.

## Where it came from

The PERF side unit on `fit-delta-probe-can-exceed-the-picture-it-
sizes` (PR "PERF side unit: the display probe is never larger than the
picture"), which re-measured the fit's committed δ across the corpus
before and after changing how the probe is placed. The rows above are
main's, not that branch's: the defect is in the law's stated error
sign and predates the change.

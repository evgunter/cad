---
id: a-dense-floor-is-priced-off-the-law-from-the-scale-probe
kind: issue
title: A body whose floor mesh is over a sixteenth of the budget is priced by the law from its floor
status: open
opened: 2026-10-22
priority: P3
cost: M
---

## The finding

`crates/viewer/src/scene.rs`, `fit_from_probes`: the scale probe
counts the body's floor `F` and the ladder sits at
`probe_delta = extent.max(finest)`. The first descent is to
`next = (PROBE_FACTOR · F · extent / TRIANGLE_BUDGET).max(finest)`,
and the loop leaves without descending when `next >= probe_delta`
(`F ≥ TRIANGLE_BUDGET / PROBE_FACTOR`, 125 000) or, through the
halving rule, when `next > probe_delta · 0.5`
(`F > TRIANGLE_BUDGET / (2 · PROBE_FACTOR)`, 62 500). Either way it
breaks `ProbeStop::Converged` with the floor as its only reading.

`answer` then runs the 1/δ law from that floor: `C = F · extent`, a
committed δ of `F · extent / TRIANGLE_BUDGET` (over `extent / 16`),
and `requested_cost = F · extent / requested`. A floor does not scale
as `1/δ`, since it is the count nothing subdivides past, so the
sentence `FittedDelta::wording` shows ("needs about N triangles")
states a cost the body never had, by orders of magnitude on a body of
any size. What is drawn stays inside the budget, because the law
over-reads. But the δ it opens at is an extent-scale one, so a dense
planar body with a few curved faces opens with those faces at the
angular cap even when the request was affordable.

**Unreached in the corpus**: the largest floor there is
`die_composed_tour`'s 1 510 (measured by the lane that filed
`a-flat-rung-pair-is-read-as-flat-below-it`;
`tests/display_budget.rs`'s
`a_bodys_count_has_stopped_falling_by_its_own_extent` tessellates
every floor), far under 62 500. A body that reaches it is a dense import or
a heavily patterned part.

## What a fix has to decide

Whether a floor over the rung bound is read as flat (descend straight
to the request's own rung, as a flat pair does) or priced from a rung
the ladder takes anyway. Either needs a row with a fake probe over
62 500 at the floor, beside the `fit_from_probes` rows in `scene.rs`'s
tests.

## Where it came from

The FIT lane that made the budget a target
(`the-budgets-predicted-count-is-not-always-an-over-count`), checking
the ladder's descent conditions for the Flat bound.

## Moved (2026-10-26)

FIT folded back into VGEOM (Ev, in chat, 2026-10-26): FIT was cut from
VGEOM on 2026-09-20 to run in parallel, a reason that is gone at FIT's
size. Id and body unchanged.

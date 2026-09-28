---
id: union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut
kind: issue
title: A tilted rod entering a half donut's cap and poking an oval out of the inner equator reaches the volume backstop on a tier-valid planar body: the pipeline's upstream result is suspect
status: dispatched
opened: 2026-09-28
priority: P1
cost: M
branch: germ/interior-loop-measurements
---


## What

Found by PR 3336's review (2026-09-28). A tilted rod enters the half donut's
planar cap and pokes an oval out of the inner equator; no events lie on the
oval. `union` refuses at `ClassificationInvariant "volume backstop: mass
properties refused on a tier-valid planar body"`, with the interior-loop
guard on, off or mutated. The backstop catches it, so no wrong answer
reaches a caller. But the body the pipeline built before the backstop is
suspect, and the backstop is the last line. Measure what the pipeline built,
and whether it is the interior-loop class meeting the backstop first.

## Home

GERM: the torus boolean lane.

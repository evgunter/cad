---
id: union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut
kind: issue
title: A tilted rod entering a half donut's cap and poking an oval out of the inner equator reaches the volume backstop on a tier-valid planar body: the pipeline's upstream result is suspect
status: open
opened: 2026-09-28
priority: P1
cost: M
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

## The backstop's sentence changed under this row (REACH, branch `reach/volume-backstop`, 2026-10-01)

The volume backstop no longer raises `ClassificationInvariant`. It
measures through the certified quadrature, and a body it cannot
measure refuses `VolumeUnmeasured { operand, source }`.

Re-run on the half donut (`sweep::revolve` of
`revolve_common::donut_profile()` about `axis_y`, `Partial(π)`), with
a three-arc rod of radius `r` and length 1.8 on the sketch plane
through `(x0, 0.05, z0)`, axes `(0, 1, 0)` and `(−√½, 0, −dz·√½)`:
- `x0 = 2.3, z0 = −0.5, r = 0.1`:
  - union refuses `CurvedPairUnsupported { site: InteriorLoopGuard, … }`
    first.
  - subtract refuses `VolumeUnmeasured { operand: None, source: Face {
    source: Escalated { margin −1.85e-9, predicate "props_quad_converged"
    } } }`. The result's quadrature could not certify its own
    convergence on one face (`work/quad/quadrature-convergence-test-escalates-instead-of-refining`),
    so the backstop measured nothing and judged nothing.
- `x0 = 2.2, z0 = −0.4, r = 0.08`: both ops refuse at the interior-loop
  guard.
- `x0 = 2.4, z0 = −0.6, r = 0.12`: both refuse `GermFrameUnsupported`.
- `x0 = 2.3, z0 = 0.5, r = 0.1`: both build and certify.

So the "suspect body" premise is unmeasured again. Once the
quadrature fix lands, the subtract either builds, refuses
`ResultVolumeImplausible` (the suspicion confirmed), or refuses
elsewhere.

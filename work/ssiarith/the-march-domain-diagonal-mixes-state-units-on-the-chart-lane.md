---
id: the-march-domain-diagonal-mixes-state-units-on-the-chart-lane
kind: issue
title: ssi/march: MarchContext::diagonal is a length in mixed state units on the R4 lane (plane-chart metres and wall parameters), and it is now the binding step cap on straight through_seed branches
status: open
opened: 2026-10-03
priority: P3
cost: M
---

(SSI implementer on `ssi/step-max-certify`, from PR 3998's review,
2026-10-03.)

## What

`MarchContext::diagonal` (`crates/geom-brep/src/ssi/march.rs`) is the
domain box's diagonal in **state** units, and `march` caps every step
at it (`StepCap::Diagonal`). On the ℝ³ lane the state is a point, so
that is metres. On the plane × NURBS lane the state is
`(plane u, plane v, wall u, wall v)`: plane-chart metres beside the
wall's knot parameters, so the diagonal is a length in no one unit, and
it scales with the plane's window and the wall's knot domain rather than
with the branch.

With `SSI_STEP_MAX` retired, nothing else caps a straight branch marched
from an interior seed (`Ends::through_seed`, the uncertified door), so
this mixed length is now the cap that binds there. The step it allows is
then densified by the certificate (`ssi/refine.rs`), so no wrong answer
has been shown: what the march proposes is a candidate, and refinement
or the certificate decides it. The cost is a step whose length nothing
geometric sets.

## What would settle it

State the diagonal in metres through the coordinate scales, as
`distance_meters` does, and measure whether any row's sample count or
outcome moves.


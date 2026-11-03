---
id: placement-turns-exceeded-ends-as-a-defect-where-its-payload-named-a-reparameterization
kind: issue
title: PlacementTurnsExceeded ends as a stored copy's defect, where a carrier parameterized far from zero is input a reparameterization repairs
status: open
opened: 2026-11-03
---


(PCERT implementer, from `pcurve-certify-error-arms-with-no-ending`,
branch `pcert/certify-arms-end`.)

## What

`PcurveCertifyError::PlacementTurnsExceeded` (`crates/geom-brep/src/pcurve_cache.rs`)
is the refused arm of `PcurveCheck::ChartWinding`, which routes
`Unsized::Defect` (`PcurveCheck::routing`: "the placement integer and the
stored copies the kernel writes exactly"). So it ends in the door's
defect: the kernel's at a build, the kernel's or the file's at rest.

The refusal is raised where an edge's span sits more than 2³¹ whole turns
from its chart's window (`Placement::turns`): a carrier parameterized far
from zero. That is input a reparameterization repairs, and two places
said so before this branch:

- its payload ended "restate the carrier's parameter near zero" (removed
  on this branch: a payload carries no recourse, and the text contradicted
  the defect ending `render` now appends at every door);
- topo's at-rest fallback table gave it `REPARAMETERIZE`
  (`geom_brep::CARRIER_DOMAIN_RECOURSE`), though that entry was dead: the
  check's ending took precedence.

## Repair shape

Decide whether a placement integer past `i32` is the stored copy's
defect or the carrier's parameterization. If the latter, give the arm the
carrier-domain lever (`Lever::CarrierDomain` in `PcurveCertifyError::decision`)
and say why the routing of `ChartWinding`'s other arms stays a defect.

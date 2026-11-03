---
id: pcurve-winding-incidence-and-sector-refusals-end-as-a-kernel-defect
kind: issue
title: geom-brep: pcurve certification's winding, incidence and sector refusals end as a kernel defect at a build, over carriers a caller may supply
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

- `crates/geom-brep/src/pcurve_cache.rs` `PcurveCheck::routing`, `ChartWinding | ChartIncidence | Sector` → `Unsized::Defect` (~2439). `PcurveCertifyError`'s `Display` renders it at a build (~2310), for `PlacementTurnsExceeded` and for the escalations of all three checks. Doors: every pcurve mint at a build, including `EulerOpError::PcurveMint` over a caller's carrier at `Body::mev`/`mef`. **Class 1, unsure**: these checks read a carrier the caller may have supplied, and nothing excludes an in-band incidence or sector read for a carrier that edge certification passed.

## Repair shape

Decide the class first. Look for an attach-door input that reaches each of the three checks.

- **Class 1**: the check's lever (move the edge clear of the chart's singular set, or keep it within one period).
- **Class 2**, if the argument holds for a check: panic per `[failure]`, with its tests moved to `#[should_panic]`.

- 2026-11-02 — moved from PCTAIL to PCERT by `git mv` when PCTAIL merged into PCERT, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and PCTAIL had fallen far under it).
- 2026-11-03 — `SectorRefused` now routes here too (PCERT, `pcert/certify-arms-end`, PR 6448). `PcurveCertifyError::decision` ends it as `PcurveCheck::Sector`'s refused arm, so it takes `Unsized::Defect` at every door, as the check's escalation already did. The reason given there: a `SectorRefused` that reaches a door is check 4's sector condition on a STORED piece (`projected::holds`); the derivation's own sector refusal is relabelled `NoGeneralLane::Pose` by `general_lane_refusal` and keeps the pose lever. Before, topo's at-rest fallback table ended it in the lever ("re-aim the chart's axis away from the curve, or split the edge there"). The same PR removed the lever from `SectorRefused`'s, `AzimuthPeriodExceeded`'s and `TubePeriodExceeded`'s payloads, because `render` now appends the check's ending at every door. The decision of this row now covers four refusals.

## `PlacementTurnsExceeded` (2026-11-03, folded in from its own row)

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

Decide whether a placement integer past `i32` is the stored copy's
defect or the carrier's parameterization. If the latter, give the arm the
carrier-domain lever (`Lever::CarrierDomain` in `PcurveCertifyError::decision`)
and say why the routing of `ChartWinding`'s other arms stays a defect.


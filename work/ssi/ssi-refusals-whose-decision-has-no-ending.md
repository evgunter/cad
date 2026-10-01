---
id: ssi-refusals-whose-decision-has-no-ending
kind: issue
title: geom-brep: fourteen SsiError arms still end in no recourse (SsiError::ending gives None), each needing its decision named
status: open
opened: 2026-10-01
---

(SSI implementer `ssi-mend`, from the §5 sweep of PR "SSI: every march
refusal names its decision, and the march's tolerance gets the floor
door's precondition".)

## What

`geom_brep::SsiError::ending` (`crates/geom-brep/src/ssi.rs`) gives
`None` for fourteen arms, so `SsiError::render` shows their payload
with no recourse at all. D4 ¶1 (i) wants one ending per decision; the
standard in `work/chrome/error-and-check-text-overflows-its-region.md`
says the recourse is the part never to drop, and that a dead end says
so plainly. The arms, with the decision each appears to be (a guess to
check, not a ruling):

| arm | what it is | likely ending |
|---|---|---|
| `ExhaustivenessInconclusive` | a floor cell neither excluded nor accounted | geometry lever (a near-tangency or a feature below the floor), or the floor scale |
| `CellBudget` | the subdivision's cell budget spent | last resort, as `FitSampleBudget` |
| `StepBudget` | one branch's step budget spent | last resort, or the domain and extent |
| `SeedRefinementFailed` | a seed outside every basin | swallowed by both certifying doors; reaches a caller only at `idealized_trace_r3` and `trace_plane_nurbs_uncertified`, where the seed is the caller's |
| `StepRefinementFailed` | a step that would not settle | the march's limit (last resort). The march now settles to what its coordinates resolve and refuses `SettlingUnresolvable` where that is not well inside ε, which removed the far-from-origin cases measured on the cylinder × sphere and the plane × wall; a step that still will not settle is not known to be the scale |
| `TubeLadderEmpty` | every ladder rung below the floor | the carrier's extent against ε (a scale lever) |
| `TubeProbeSilent` | no rung answered | kernel defect or last resort |
| `FootPointInconclusive` | a certified foot point would not converge | last resort (the projection is an approximation) |
| `Fit(FitError)` | the fitting stack refused the trace | by the `FitError` it carries |
| `UnsupportedCertificate` | a documented per-arm boundary | a plain "no way through" ending |
| `TubeDegenerate` | the wall is constant across the locus, or the pcurve tangent is unusable | geometry lever for the wall, defect for the pcurve |
| `WrongLane` | the door was handed the wrong kinds | the caller's (pass the kinds the door traces) |
| `Band(BandError)` | the band could not be built | the caller's knobs |
| `InvalidMarchTol` | the uncertified door's march tolerance is not a length | the caller's knob; its `Display` also carries developer advice (`MarchTol::from_band`, a crate-private name) that belongs in rustdoc |

`ChartSpeed`, `OperandNotFinite`, `DomainUnusable`, `FloorUnresolvable`, `SettlingUnresolvable`,
`BranchUndersampled`, `StepCollapsed`, `StepUnusable`, `FitSampleBudget`,
`TransversalityBand`, `PairTangent`, `SelfCrossingLocus`,
`CertificateLimb`, `TubeStraddles`, `CertificateEscalated`, `Escalated`
and `MarchTolMismatch` have endings.

## Repair shape

Name each arm's decision in `SsiError::ending`, ending by an existing
table where one fits (`crate::recourse::Unsized`, a `SizedDecision`,
`crate::certify::recourse`), and shorten any payload the ending pushes
past the word budget. A row asserting every arm's ending is `Some`
over one representative payload each closes the class.

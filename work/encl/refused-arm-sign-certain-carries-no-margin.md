---
id: refused-arm-sign-certain-carries-no-margin
kind: issue
title: geom-brep: RefusedArm::SignCertain carries no reporting margin, so a definite residual miss reaches the import door by a parallel definite_*_in_file route
status: dispatched
branch: encl/sign-certain-arm-margin
opened: 2026-10-08
priority: P3
cost: M
---


(Filed by the ENCL fix-pass implementer on PR 4331, `encl/adoption-at-rest-eps-in`, on the orchestrator's ruling. This change is not taken in that PR.)

## What

`geom_brep::recourse::RefusedArm` (`crates/geom-brep/src/recourse.rs`:198) gives its band-decided arms their reporting margin: `Zero(Classified)` and `Undecided(&Indeterminate)`. Its sign-certain arm, `SignCertain` (recourse.rs:206), carries none.

So when PR 4331 gave `CertifyError::ResidualExceeded` its `margin`, that margin could not ride the arm. It reaches the import door by a second route beside `certify::recourse_in_file`:
- `CertifyError::ending_in_file` special-cases the variant (`crates/geom-brep/src/certify.rs`:678);
- `certify::definite_miss_in_file` (certify.rs:894) re-dispatches on the check's ending;
- `Unsized::definite_residual_in_file` (recourse.rs:128) builds `MissReading::Definite`.

Meanwhile `Unsized::residual_in_file` maps `SignCertain` to `MissReading::DefiniteUnvalued` (recourse.rs:120). Two routes now serve one decision's arms.

## Repair shape

Give the sign-certain arm the margin where the verdict has one: `SignCertain(Option<MarginDiag>)`, or a `Classified`. Then:
- `CertifyError::decision` returns `ResidualExceeded`'s margin on it;
- `residual_in_file` reads it;
- the `definite_*_in_file` route is deleted.

`RefusedArm::SignCertain` is matched at about 48 sites across geom-brep, topo and sweep, mostly constructing a margin-free verdict, so this is a sweep of its own. The limb follow-up (`plane-nurbs-limb-refusal-carries-no-reporting-margin-for-the-import-door`) would then fill that arm for the plane x NURBS limb too.

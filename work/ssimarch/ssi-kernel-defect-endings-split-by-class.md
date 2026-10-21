---
id: ssi-kernel-defect-endings-split-by-class
kind: issue
title: geom-brep: ten SsiError endings name the kernel alone at a build; one is reachable by a caller's domain
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

These render only through the public `SsiError::render` / `ending`,
which a caller of `cylinder_sphere_ssi`, `plane_nurbs_ssi`,
`trace_plane_nurbs_uncertified` or `idealized_trace_r3` reads. No
shipped caller renders them: `topo/src/offset_derive.rs` prints the
payload, and `edge_nurbs.rs` re-types the arms it keeps.

- `crates/geom-brep/src/ssi.rs` `SsiError::ending`, `MarchTolMismatch` (~1298). **Class 2, likely**: `seam_tol` is private, and its one caller passes the `MarchTol` that `from_band` minted from the run band.
- `SsiError::ending`, `StepUnusable { SpeedUnusable }` (~1341). **Class 2, likely**: the chart mint refuses a degenerate chart before any march.
- `SsiError::ending`, `Fit(TooFewPoints)` (~1368). **Class 2, likely**: refinement hands the fit its samples or refuses.
- `SsiError::ending`, `Fit(InvalidTolerance | KnotAlgebra | RaggedRows | BudgetExhausted)` (~1376). **Class 2, likely**: the interpolation runs none of these.
- `SsiError::ending`, `TubeProbeSilent { refused: 0 }` and `TubeDegenerate(PcurveTangentUnusable)` (~1469). **Class 2, unsure**: a fitted cubic's tangent that vanishes may be the fit's limit (class 3).
- `SsiError::ending`, `ChartRow` (~1472). **Class 2, likely**: a wall that validated re-wraps every boundary row. The spline layer refuses a weight that is not strictly positive when the net is built (geom-core `spline/knots.rs`).
- `step_budget_recourse`, `StepBound::Cap(StepCap::Diagonal)` (~2045). **Class 2, unsure**: the diagonal ends an open branch in one step, but no argument covers a closed branch held at that cap.
- `TraceDecision::ending`, a poisoned margin on `StepProgress | ClosureTangent | BranchOpenEnd` (~2288), and `BoundarySection` (~2296). **Class 2, unsure**: the operands are refused only when not finite, so finite operands at extreme magnitude may overflow a margin. If they can, these sites are class 1 and take the range lever.
- `crates/geom-brep/src/ssi/exhaust.rs` `FloorRefusal::ending`, `DomainUnreadable` (~456). **Class 1, likely**: `SsiDomain::check` tests each field for finiteness, but `Box3::around(center, half_extent)` can overflow. A transversal `cylinder_sphere_ssi` with `center.x = 1e308` and `half_extent = 1e308` passes `check`, and `SweepFloor::r3` then reads an infinite reach. The variant's doc ("every SSI door checks the caller's domain first") misses the sum.

**A second laundering channel**, repaired with these arms: `crates/geom-brep/src/edge_nurbs.rs` `of_tube` (~1609) and `refusal` (~2077) re-type `TubeProbeSilent { refused: 0 }` and a catch-all `_` into `Unsupported`. That moves a bug-only state into the not-yet-built row (`NOT_YET_ENDING`). When an arm here becomes a panic or a lever, those two re-typings must stop absorbing it.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, after answering row 0; the tests that pin the text move to `#[should_panic]`.
- **Class 1** (`DomainUnreadable`): the domain's lever, as `DomainUnusable` gives it. Better, `SsiDomain::check` refuses a slab that overflows, and the floor's arm becomes class 2.

Re-check each `likely` or `unsure` argument before acting on it. One that fails moves the site to class 1.

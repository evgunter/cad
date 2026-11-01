---
id: step-budget-curvature-recourse-advises-loosening-beside-a-domain-lever
kind: issue
title: STEP_BUDGET_CURVATURE_RECOURSE advises loosening the tolerance beside a geometry lever, against D4 ¶1 (i)
status: open
opened: 2026-10-21
priority: P3
cost: E
---

## What

`crates/geom-brep/src/ssi.rs:2061`, `STEP_BUDGET_CURVATURE_RECOURSE`
(`SsiError::StepBudget`'s ending where the curvature held the steps
short):

> "Recourse: name a domain around just the feature traced, or loosen
> the tolerance, " + `kernel_limit_last_resort!()`

DESIGN D4 ¶1 (i) (`docs/DESIGN.md:515-518`, ruled by Ev on `[ev]` PR
3352): "No refusal advises loosening ε, except one that would otherwise
name no recourse at all (a kernel approximation limit), which names it
as a last resort and says the refusal may indicate a kernel bug."

This ending names a geometry lever (a domain around just the feature
traced), so it is not a refusal that "would otherwise name no recourse
at all", and the loosening arm falls outside the exception even though
it carries the last-resort clause.

**The sibling done right.** `STEP_BUDGET_FLOOR_RECOURSE`
(`ssi.rs:2053`), the ending where the refused residual stopped falling,
loosens alone: "the arithmetic's floor, not the geometry's size". No
domain helps there, so the last resort is the only recourse, as D4
writes the exception.

## Shape

The ending names the domain lever only. If a curvature-held march is
one where no domain helps (a feature that is itself all curvature), that
case is a distinct refusal whose ending is the last resort alone, like
its floor sibling.

Found by the scan in PATHS'
`path-scene-resolution-recourses-advise-a-coarser-tolerance-beside-geometry-levers`
(closed by NURBS build A's `nurbs/scene-resolution-recourse`).

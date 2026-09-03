---
id: ssi-lever-arm-min-fold-hides-poison
kind: issue
title: ssi - the curvature lever arm is folded with f64::min, so a poisoned operand cannot reach the transversality guard's NaN arm
status: open
opened: 2026-08-29
github: 1219
refs: [762]
---

## From GitHub issue 1219

opened 2026-08-29, 0 comments.

Found by the shape sweep the CERT-2 lane owed for issue 762's second item ("`f64::max` returns the non-NaN operand, so a single poisoned derivative box cannot reach a guard's `is_nan()` arm"). That item was fixed at the two chart-speed sites; the sweep for the same **shape** — a min/max fold feeding a guard that has a NaN arm — turns up the lever-arm folds, which nobody has swept. Filed rather than fixed: the CERT-2 fence is the chart-speed guards and the two SMELL rows.

## The sites

Three folds, one chain:

- `geom-brep/src/ssi/system.rs:352` — `ImplicitPairR3::lever_arm`:
  `curvature_lever_arm(self.a, p).min(curvature_lever_arm(self.b, p))`
- `geom-brep/src/ssi/march.rs:367` — `let arm = sys.lever_arm(&x).min(ctx.extent);`
- `geom-brep/src/ssi.rs:894` — the ℝ³ finisher's
  `curvature_lever_arm(a, …).min(curvature_lever_arm(b, …)).min(domain.extent)`

`f64::min` returns the non-NaN operand, exactly as `f64::max` does.

## Why it is the same defect

`crates/geom-brep/src/implicit.rs:222` — `curvature_lever_arm` **returns poison** for `Surface::Nurbs` and `Surface::Approx`. It is not a hypothetical poison source the way `deriv_box`'s was; it is written into the callee today, one match arm down from the kinds that answer.

The consumer at `march.rs:367` is a guard with a NaN arm: `decide("ssi_transversality_arm", Margin::of(arm), band)` escalates on an indeterminate margin. So if a poisoned lever arm ever reached it, the guard would refuse — and the `.min` is what stops it reaching. A poisoned operand instead contributes *nothing* to the fold, and the surviving operand's arm is used as though it bounded both.

## What keeps it latent, and what would not

Only the lane check. `cylinder_sphere_ssi` refuses `WrongLane` for anything but a cylinder and a sphere, so no `Nurbs` or `Approx` operand reaches the ℝ³ pair today. It goes live the moment an arm retires that admits one of those kinds into the ℝ³ trace — which is the retirement direction the C5 table is built for.

## Suggested shape of the fix

The NaN-propagating fold the chart-speed guard now uses (`ssi.rs:981-986`), applied at all three sites, plus a row driving `ImplicitPairR3::lever_arm` with a `Surface::Nurbs` operand and pinning NaN rather than the sibling's radius. The change is a no-op on every input reachable today, which is what makes it cheap and also what makes it easy to keep missing.

## Home

`crates/geom-brep/src/ssi.rs` and `ssi/*` are in VERBS' `paths:` territory, and the C5 table retirements that would make this live are its charter.

---
id: merge-coplanar-refusals-open-with-a-stage-label
kind: issue
title: topo: twelve MergeCoplanarError arms still open with the merge_coplanar_faces: stage label
status: open
opened: 2026-09-30
---

(TOPO, the residue of `merged-face-role-ambiguity-ends-in-no-recourse`,
PR 3532: that unit dropped the label from `MergedFaceRoleAmbiguous`,
the arm it rewrote, and files the rest here.)

## What

Twelve arms of `MergeCoplanarError`'s `Display`
(`crates/topo/src/merge_faces.rs`) still open with the
`merge_coplanar_faces:` function label, which the refusal-shape guard
(`test_utils::refusal::stage_prefixes`) reads as a stage prefix:
`InputNotClosed`, `ResultNotClosed`, `GroupNotClosed`,
`GroupKindSplit`, `PoisonedSurfaceDescription`,
`UnsupportedConfiguration`, `PeriodClosure`, `Op`,
`InvalidDeclaration`, `DeclaredCarrierUnsupported`, `Band` and
`Pcurve`. Most also carry no `Recourse:` ending (`InputNotClosed`,
`ResultNotClosed`, `GroupNotClosed`, `UnsupportedConfiguration`, `Op`,
`InvalidDeclaration`, `Band`, `Pcurve`), so the label is all the
context they give.

## Why it was not dropped with the outline arm

Dropping the label alone trades one defect for another: "input is not
tier-2 (3 errors)" names no operation and still no recourse. Each arm
needs a subject and an ending of its own, and several are asserted in
other suites (`boolean::refusal_routes`, `sweep`'s probes), so it is a
unit, not a drive-by.

## Repair shape

Per arm: a subject clause that names what was refused, and the one
ending its class takes (an inventory limit's lever, a kernel defect's
ending, or the operator refusal's own text for `Op`). The guard rows
that already read `MergeCoplanarError` (`escalated_margin` in
`merge_faces`' winding tests, the outline rows' `assert_one_story`)
are the shape to extend to every arm.

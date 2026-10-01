---
id: euler-op-corruption-refusals-end-in-a-tag
kind: issue
title: EulerOpError's tier-1 corruption refusals end in a '(malformed body)' tag, not the shared kernel-defect ending
status: review
pr: 3621
branch: topo/euler-corruption-ends-one-way
opened: 2026-09-30
refs: [loop-cycle-broken-display-names-one-of-its-causes]
priority: P3
cost: E
---

## What

Found by the walk-proofs unit (PR 3511), which rewrote
`EulerOpError::LoopCycleBroken`'s `Display` and matched its ending to
its siblings'.

`EulerOpError`'s tier-1 corruption variants (those
`EulerOpError::reports_tier1_corruption` answers `true` for, in
`crates/topo/src/euler.rs`) end their `render` arms in a
"(malformed body)" tag: `FanOrbitBroken`, `LoopCycleBroken`,
`LoopNotCycle`, `UnclaimedHalfEdge`, `OrbitBroken`. `StaleKey`,
`StaleGeometry`, `NotSameEdge` and `EmptyAnchorsCollide` end in
nothing. Only `KillLeavesDangling` (PR 3570) and `NotOwned` (PR 3592,
`movefac`'s ownership proof) carry
`geom_core::KERNEL_OR_FILE_DEFECT_ENDING` (a body the Euler operators
refuse may have been read from a file), which
`test_utils::refusal::recourse_markers` counts; the class now ends
three ways. This is the Euler-operator instance of
`work/hone/kernel-bug-refusals-end-without-the-shared-ending`.

## The shape to give

Each corruption arm ends ". {KERNEL_OR_FILE_DEFECT_ENDING}" in place of
the tag, one decision for the whole class (D4 ¶1 (i)), and a row over
`every_euler_op_error_once` asserts that every variant
`reports_tier1_corruption` answers `true` for renders that ending.

## Correction: the ending is `KERNEL_DEFECT_ENDING`

The shape above prescribed `KERNEL_OR_FILE_DEFECT_ENDING` on the
premise that a body the Euler operators refuse may have been read from
a file. No file reaches one torn: no kernel crate takes serde
(`scripts/gates/kernel-serde-free.sh`), the document layer persists
only kernel enums (`editor-core`'s `persist::kernel_wire`:
`BooleanOp`, `ContactClass`) and replays a recipe through the doors,
topo's raw builder is crate-internal, and STEP import assembles every
body through the public doors (`mvfs`, `mev_line`, `mef_chord`,
`mekr_chord`, `kemr`, `kfmrh`, `ring_move`, `set_face_surface`,
`set_edge_curve_nurbs_lane`), each of which preserves tier 1. So the
class ends in `geom_core::KERNEL_DEFECT_ENDING`, and `KillLeavesDangling`
and `NotOwned` move to it. The one route to a torn body that is not a
kernel operation's is a caller who keeps a graft destination its `Err`
left spent, which is `S14(b)`'s open question, not a file.

`StaleKey`, `StaleGeometry` and `NotSameEdge` are reached by a caller's
mistake as well as by a torn body, and the variant does not say which,
so they end in the caller's repair before the report instead:
`stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body`.

---
id: euler-op-corruption-refusals-end-in-a-tag
kind: issue
title: EulerOpError's tier-1 corruption refusals end in a '(malformed body)' tag, not the shared kernel-defect ending
status: open
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
nothing. Only `KillLeavesDangling` (PR 3570) carries
`geom_core::KERNEL_OR_FILE_DEFECT_ENDING` (a body the Euler operators
refuse may have been read from a file), which
`test_utils::refusal::recourse_markers` counts; the class now ends
three ways. This is the Euler-operator instance of
`work/reach/kernel-bug-refusals-end-without-the-shared-ending`.

## The shape to give

Each corruption arm ends ". {KERNEL_OR_FILE_DEFECT_ENDING}" in place of
the tag, one decision for the whole class (D4 ¶1 (i)), and a row over
`every_euler_op_error_once` asserts that every variant
`reports_tier1_corruption` answers `true` for renders that ending.

---
id: placement-rule-refusals-state-no-recourse
kind: issue
title: The placement-rule refusals the feature tree draws state no recourse
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by `wire-census`'s sweep for feature-tree census lists that skip
an arm of the enum they forward: `editor_payloads`' `placement` list in
`crates/editor-core/tests/refusal_concision_chains.rs` had no
`NonRigidFrame` row. With the row added,
`PlacementRuleFault::NonRigidFrame` (`crates/editor-core/src/node.rs`,
its `Display`, which forwards `placement::FrameFault::NotRigid`) renders
"placement 2 is not definitely rigid at tolerance: …" and states no
recourse, so it joins `CountSpelling`, `ImproperFrame`, `NoPlacements`
and `NonFiniteFrame` on that file's `FILED_NO_RECOURSE`. Those four
were parked there as held ground (D10) when
`wire/wire-refusals-short-of-the-shape-guard` closed, with no row of
their own; this is that row.

## Repair shape

Each arm ends in one labelled `Recourse:` (a finite, proper, rigid
frame for the placement it names) or in the kernel-defect ending, and
its five entries leave `FILED_NO_RECOURSE`, where
`every_admission_admits_a_row_it_is_needed_for` reds on each one left
behind. Placement ground: waits on the D10 hold.

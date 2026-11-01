---
id: placement-rule-refusals-state-no-recourse
kind: issue
title: The placement, instance and face-frame refusals the feature tree draws state no recourse
status: closed
closed: 2026-10-28
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

It also holds the other entries parked there as held ground
(`wire-refusals-short-of-the-shape-guard`'s 2026-10-17 triage: they
"stand on stage 3 ground"): `PlacementsUncertified`,
`InstanceOutOfRange`, `FaceFrameNotPlanar`, `FaceFrameReadback/Dangling`,
`FaceFrameReadback/NoCanonicalFrame`, `FaceFrameReadback/NoCarrier` and
`FrameDirection/Degenerate`. Each renders with no `Recourse:` and none
of the shared endings.

## Repair shape

Each arm ends in one labelled `Recourse:` (for a placement rule, a
finite, proper, rigid frame for the placement it names) or in the
kernel-defect ending, and its entry leaves `FILED_NO_RECOURSE`, where
`every_admission_admits_a_row_it_is_needed_for` reds on each one left
behind.

## When the trigger fires

Placement ground, so it waits on the D10 stage 3 unit
`intent/transform-retires-into-a-placement`, which rebuilds the
placement-rule door, the instance placements and the authored frames
these refusals come from: re-read each arm then, since the rebuild may
already give it a recourse or retire it.

## Closed by G (`patterns-are-index-variables`)

Every arm this row held on placement-rule ground retired with `PatternKind` and `placement_rule_fault` (`CountSpelling`, `NoPlacements`, `NonFiniteFrame`, `NonRigidFrame`, `PlacementsUncertified`, `InstanceOutOfRange`), and the face-frame arms had already left `FILED_NO_RECOURSE` with D's retirement of `Transform`. None of this row's entries is left on the list.

---
id: the-unproven-coincidence-knob-is-re-read-against-ds3
kind: issue
title: The unproven-coincidence lint's knob is re-read against DS3 now that every finding is quietable or never refuses (spec §11 Q4)
status: open
opened: 2026-10-27
priority: P2
cost: M
---


**What.** `docs/INTENT-STAGE5-SPEC.md` §11 Q4 decided that the `unproven-coincidence` lint's knob stays `Advisory`. One ground for that was mechanical, under CHECKS DS3/DS6: not every finding was quietable, so `Error` could refuse what no assertion could quiet. Once `an-assertion-quiets-an-operations-unproven-row` (PR #5882) lands, every lint finding is either quietable by an assertion or never refuses (`contact_quietable`, `never_refuses`). So that ground is gone, by the same reading that makes `CheckId::AtRest` a `Severity` knob. Recorded by #5882's lane.

**What it would take to change.** Q4 is a decided question, so moving the knob to `Severity` is a design change for a designer pair and an `[ev]` PR (`memories/orchestration-model.md`), not a build. Two points the decision must weigh:
- **Flush pairs.** A flush (`SameOriented`) row never quiets, so it never refuses. Yet a flag mate can still establish it. Is a flush pair at `Error` meant to be silent?
- **Adjacency vacuity** (`a-gap-over-adjacent-cells-states-nothing-of-the-row`, P2). Some op rows (a blend's `BatteryTurn`/`BatteryJoint`, tangent and coaxial rows) name adjacent cells, so a `Gap = 0` over them holds by topology. At `Error`, a vacuous assertion would silence the row.

Blocked on #5882 merging.

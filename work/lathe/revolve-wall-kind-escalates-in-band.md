---
id: revolve-wall-kind-escalates-in-band
kind: issue
title: Revolve's classify_segment refuses SliverAxisClearance when the wall-kind reading is in band, where D3 takes the cone lane
status: open
opened: 2026-10-20
priority: P2
cost: M
---

Filed by PCERT unit E1 (`pcert/lane-choices`), from its sweep of lane
choices outside `pcurve_cache`.

Each site below picks between two forms of the same cells, a lane
choice under D3 (`docs/DESIGN.md`, `[verdicts.consumers]`).

## What

`crates/sweep/src/revolve/axis.rs:422` `classify_segment`, the Line
arm. It reads `LineRadial` and `LineAxial`:
- Zero mints a cylinder or a plane annulus;
- both definite mint a cone, the general wall over the same cells;
- in band refuses `SliverAxisClearance` (:439).

## Fix

In band should take the cone lane. If a near-degenerate cone is the
frontier, it should refuse lever-only "not yet" on both arms instead.
Add a row with an in-band pose, and a mutant that restores the refusal.

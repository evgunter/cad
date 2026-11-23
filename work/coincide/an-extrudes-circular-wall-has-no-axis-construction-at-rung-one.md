---
id: an-extrudes-circular-wall-has-no-axis-construction-at-rung-one
kind: issue
title: An extrude's circular wall has no axis construction at rung one: stated::axis_of reads an axis only off a revolve's band
status: open
opened: 2026-10-30
priority: P2
cost: M
---


Found by the designer pair on `atrest/a-gap-over-adjacent-cells-states-nothing-of-the-row` (2026-10-30, `[ev]` #6086).

**The gap.** `coincide/stated.rs` `axis_of` reads an axis construction only off a revolve's `Band` (its `axis` output). An extrude's circular wall has none at rung one. So stage 6 B's "coaxiality is one `Axis` read twice" covers revolves fully but extrudes only where a frame was read off the wall's axis; stage 6 D's trace statement reads that frame.

Example: a filleted disc's supports prove only at rung three. Two extruded circular walls whose profiles share one circle centre variable are not one `Axis` by any statement the door reads.

**Done when** an extrude's circular wall has an axis construction at rung one: its profile frame's `z` through the circle's centre, read as the wall's `Axis`. Two walls built on one centre and one heading then prove coaxial at rung one, and a centre typed alike stays unproven.

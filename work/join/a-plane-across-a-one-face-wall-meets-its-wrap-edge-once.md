---
id: a-plane-across-a-one-face-wall-meets-its-wrap-edge-once
kind: issue
title: A transverse plane across a one-face closed wall meets its wrap edge at one point, and the join refuses SingleSiteSectionLoop; not a coincidence, so not held by D10
status: open
opened: 2026-10-06
priority: P1
cost: H
branch: join/wrap-edge-section-loop
---


Split out of `closed-in-face-section-loop-has-one-site`. That row is parked on D10 because its original case, a conic lying *in* the partner's face, is a coincidence verdict, and coincidence is D10's ground. The cases appended to it later are a different class: a TRANSVERSE plane crossing a one-face closed wall. The section circle crosses the wall's single wrap edge at one point, there is no coincidence anywhere, and `boolean/join.rs` refuses `SingleSiteSectionLoop` structurally, because a record whose two germs name one locus on both operands can only match itself. This row is that class, and it is not held.

- **On main today:** a full-revolved tube under a box refuses this way (CLEAVE), because full-revolve walls already wrap `u` with one seam.
- **PATHS unit 3 (#4169):** a one-segment circle's extruded wall refuses a slab or pocket cut across it in the same way.
- **PATHS unit 4** (`circle-lowers-to-one-segment`) would make this every slab or pocket floor through a circular boss, so unit 4 is blocked on this row.

**Shape of the arm.** The split already handles the analogous case: `d9244fd60` treats a self-loop chord across a full revolve's seam as the whole section conic.

**Who found it.** Both designers on the PATHS one-segment-seam fork (`[ev]` PR #4175).

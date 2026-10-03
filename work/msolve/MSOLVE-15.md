---
id: MSOLVE-15
kind: unit
title: A mate side is a base composed with an offset; the offsets carry the roll; the primitive names only the residual subgroup
status: open
opened: 2026-10-03
priority: P1
cost: H
branch: msolve/15-frame-offset-carries-the-roll
---


Spec: `docs/MSOLVE-15-SPEC.md`. Plan item 22. It builds PLACE's row
53 (`MateFrame { base, offset }`) together with Ev's ruling on `[ev]`
PR 3681: the offsets carry the roll, `Coaxial { roll: Free | Pinned }`,
the standoff retires, and the rider and `Clocking` are deleted. Review
tier: dual. The unit changes a public type, the save format of every
mate-bearing document, and the table. Dispatches after MSOLVE-14
merges.

---
id: projectbox-section-cuts-through-bores
kind: unit
title: the projectbox section cuts through bored bosses, so its section faces are annular rings, one per half
status: review
opened: 2026-10-02
priority: P3
cost: M
pr: 3811
---

## What

CLEAVE PRs #3658 and #3714: a plane split through a bored body now
gives ONE annular section face per half (it used to give a square plus
a cancelling disc), tilted cuts and thin tubes included, and
`topo::plane_section` returns regions with holes. Evidence:
`crates/sweep/tests/split_section_rings.rs` (21 rows, among them
`a_split_through_a_bore_makes_one_annular_section_face_per_half` and
`a_thin_tube_cut_at_a_tilt…`).

`projectbox`'s cell already carries the tour's machinist's section —
the enclosure cut by a tilted plane and the halves pulled apart — but
its cut crosses no bore, so the section faces are plain polygons. The
enclosure has the bores a real one has: four floor bosses with pilot
pockets. Make the pilots real through-bores (or add the mounting bores
a real enclosure has) and place the cut so it passes through bored
bosses: the section then shows rings, which is what a machinist's
section of a bored part looks like.

## Oracle

- The section face count per half and each face's ring count, asserted
  (one outer loop plus one ring per bore the plane crosses).
- `plane_section`'s regions read back and checked against the closed-
  form section area (the tilted plane's ellipse-shaped bore holes have
  area πr²/cos φ).
- The body's volume stays on its exact oracle after the bore change.

## Constraint

Bosses are CYLINDERS unioned into a floor: if a bore through a boss
refuses on a curved door, file the refusal on the owning program and
fall back to bores through the planar walls (vent ribs or a mounting
flange), saying so.

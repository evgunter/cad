---
id: sphere-face-with-a-hole-has-no-closed-form
kind: issue
title: A sphere face with a hole has no closed form (RingOnCurvedFace), though the Gauss–Bonnet loop lane extends to rings
status: open
opened: 2026-10-05
priority: P1
cost: M
refs: [a-ring-on-a-sphere-face-has-no-island-winding]
---


Found by `reach/trimmed-sphere-escape`.

## Measured

With a sphere ring lane's island wound (the code at `cd76b9c3a3` on
that branch, `boolean::join::sphere_island_ccw`), the slab against
`ball_poled_y(0.5)` at `(0.3, 2.0, 1.2)`
(`m5_s13_review_probes::probe_edge_escape_refuses_typed_before_the_scan`'s
pose) builds ∩ and slab ∖ ball, and ∪ and ball ∖ slab refuse

```
ResultInvalid { errors: [VolumeUncomputable { solid: SolidKey(1v1),
  source: RingOnCurvedFace { face: … } }] }
```

The result is right to carry the hole: the slab's top face pierces the
ball's face in a circle no seam crosses. `topo::props::face_flux`
refuses any ring on a curved face but a rim-and-ruling cylinder wall,
and `geom_brep::props::curved_face_loops` reads one loop for every
other kind. The point-in-solid at-infinity probe reads the same
volume (`solid_contain::at_infinity_side`, `VolumeUncertified`), so an
operand with such a face cannot be classified either.

## What a fix owes

`geom_brep::props::sphere_circle_loop` already measures one loop of
circle arcs by Gauss–Bonnet. Over loops `i` with `G_i = Σ∫κ_g ds + Σε`
read against the outward normal, a face of `1 + r` loops has
`χ = 1 − r`, so `Area/R² = 2πχ − Σ_i G_i`, and the vector area adds
per loop. The rows are the probe pose's ∪ and ball ∖ slab against a
slice integral, with a ringed sphere face in tier 3 and in point
classification.

## Evidence (2026-10-07, TANG `tang/sphere-ring-island-winding`)

The sphere ring lane winds its island on main now
(`chord_join::path_island_winding`), so every pose below builds ∩ in
both member orders and box ∖ ball at the slice integral, and the ball's
side of ∪ (both orders) and ball ∖ box refuse here. The rows pin this
refusal and go red when the lane lands:

- `crates/sweep/tests/a_ring_on_a_sphere_face.rs`: the probe pose,
  mirrored below the slab, near a pole, three tilted `z`-poled balls,
  the unit ball against a box corner and a box edge, and the unit ball
  against `ball(0.6)` at `1.2·(0.6, 0, 0.8)` (there unit ∖ small refuses
  too);
- `m5_s13_review_probes::probe_edge_escape_lands_a_ring_before_the_scan`
  (renamed from `probe_edge_escape_refuses_typed_before_the_scan`), ∪;
- `four_crossings_on_one_section_circle::four_crossings_above_the_centre_wind_the_sphere_ring`,
  ∪ and ball ∖ slab.

Past the volume, `mesh::curved::tessellate_curved` and
`mesh::trimmed` refuse any ringed curved face
(`TessellateError::RingOnCurvedFace`), so these results will not mesh
either until that lane reads a ring.

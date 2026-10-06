---
id: a-notched-or-ringed-cylinder-wall-does-not-tessellate
kind: issue
title: A cylinder wall notched by rulings and an arc, or carrying a hole, builds and measures but does not tessellate
status: open
opened: 2026-10-02
---


## What

`mesh::tessellate` refuses two cylinder-wall shapes the boolean now
builds and `topo::mass_properties` measures in closed form (the
cylinder flux is its chart Green form over every loop,
`geom_brep::props::curved_face_loops`, landed by `tang/pierce-ring`):

- a wall NOTCHED by rulings and rim arcs, an iso domain that is no
  rectangle: `UnsupportedCurvedShape { source: NotIsoRectangle { what:
  "props_rim_level" } }` — the mesh curved lane cites the shape door
  `geom_brep::props::require_iso_rectangle`
  (`crates/mesh/src/curved.rs`);
- a wall with a HOLE (a pierce whose section closes inside one wall
  face): `RingOnCurvedFace` (`crates/mesh/src/curved.rs`,
  `crates/mesh/src/trimmed.rs`).

Measured on `tang/pierce-ring` with the unit pipe `r = 1`,
`z ∈ [−2, 2]` (`crates/sweep/tests/verbs_germarms.rs`'s `pipe`) and a
bar through its wall, at tolerance `1e-3`:

| bar | ∪ / pipe ∖ bar | ∩ / bar ∖ pipe |
|---|---|---|
| `(−1.1, 1.1) × (−0.3, 0.3) × (−0.3, 0.3)` | `NotIsoRectangle` | meshes, `check_mesh` Ok |
| `(0.5, 1.1) × (−0.3, 0.3) × (−0.3, 0.3)` | `NotIsoRectangle` | meshes |
| `(−1.1, 1.1) × (0.15, 0.7) × (−0.4, 0.1)` | `RingOnCurvedFace` | meshes |

Every one of those bodies passes tier 3 and meets its closed-form
volume to `1e-12` (`verbs_germarms::assert_bar_through_the_pipe`), so
the mesh lane is the only consumer left that rejects them; a viewer
showing the result of an everyday cut through a pipe wall stops here.

## The shape of a fix

The wall's chart region is a rectilinear polygon in `(u, v)` (rims
horizontal, rulings vertical), holes included, inside one branch of the
azimuth when the face window is under a period; a chart-polygon
triangulation of that region, lifted through the cylinder, would serve
both shapes.

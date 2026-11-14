---
id: a-notched-or-ringed-cylinder-wall-does-not-tessellate
kind: issue
title: A cylinder wall notched by rulings and an arc, or carrying a hole, builds and measures but does not tessellate
status: parked
opened: 2026-10-02
priority: P0
cost: H
blocked_on: [a-curved-face-is-routed-by-its-domain-not-its-carriers]
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

## More witnesses: a ring on a cone wall (GERM, `germ/cone-ring-volume`)

A cone wall with a hole now measures too: the cone's closed form reads
every loop's vector area (`geom_brep::props::cone_face_closed_form`).
`mesh::tessellate` refuses those bodies with the same
`RingOnCurvedFace` (`crates/mesh/src/curved.rs`, the ring check at the
top of the curved patch). Witness: T1's ∪ in both member orders and
cone ∖ box (`crates/sweep/tests/cone_join_lane.rs`'s
`each_poses_body_is_its_closed_form_in_every_op`, which allows this
refusal on exactly those three ops). Each body passes tier 3 and
measures `OK SOUND`. The ring there is two ellipse arcs, so a chart
polygon of rims and rulings does not cover it; the cone's chart region
is bounded by the sections' images.

## More witnesses (FLUX, `flux/ellipse-ring`)

A wall whose ring is trimmed by ELLIPSE arcs now measures too: the
cylinder quadrature reads every loop
(`geom_brep::props::quad::cylinder_cut_face`). The unit pipe ∪ or ∖ the
brick `(−1.1, 1.1) × (0.2, 0.7) × (−0.15, 0.15)` turned about the x axis
through `(0, 0.45, 0)` by 0.3, 0.7, 1.0 or 1.3 rad meets its exact volume
(`verbs_germarms::a_wall_ringed_by_ellipse_arcs_measures`), and the
mesh's trimmed lane refuses that wall `RingOnCurvedFace` before its
dispatch (`crates/mesh/src/trimmed.rs`, `tessellate_trimmed`'s ring
guard). The fix above wants the trimmed lane's chart region as well:
the same polygon with the ring's ellipse images for edges.

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Measured on main (2026-11-14, TESSLANE) — re-banded P0

Run at `d6e1eb1f` (probes `reach_probes::a_notched_or_holed_pipe_wall_refuses_at_the_curved_lanes_doors` and `a_cone_wall_holding_a_ring_refuses_at_the_curved_lanes_ring_guard` on `tesslane/measure-reach`). This row's table reproduces exactly at δ 5e-3, 1e-3 and 2e-4.

- **The notch** (bars 1 and 2; ∪ in both orders and pipe ∖ bar): `UnsupportedCurvedShape { source: NotIsoRectangle { what: "props_rim_level" } }`, built by `curved::require_iso_rectangle_face` from `geom_brep::props`' `require_rims_at_extremes`. Every half-edge of the notched wall carries a `Harmonic` image, and the trimmed cylinder arm with `cert::cert_cylinder` and its lattice exists, but nothing routes the face to it: `lane_of` reads carrier kinds, and an all-iso loop goes to the curved lane, whose door refuses a loop that is not a rectangle.
- **The ring** (bar 3; the brick turned 0.3 about x, ∪ and pipe ∖ bar; cone T1, ∪ in both orders and cone ∖ box): `RingOnCurvedFace`, raised by `curved::tessellate_curved`'s ring guard. This row's ellipse-ring paragraph names `tessellate_trimmed`'s guard; that is not the site, because the ellipse arcs are on the rings and the outer loop is iso.
- ∩ and bar ∖ pipe mesh and pass `check_mesh`; with the turned brick they mesh through the trimmed cylinder arm.

**P0**: a pipe with a boss or a slot through its wall is as ordinary as a body gets, and it cannot be drawn. With designers alongside `trimmed-tessellation-lacks-torus-and-plane-arms` (`design: true`).

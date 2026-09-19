---
id: rim-free-loop-on-a-poleless-chart-meshes-as-a-hole
kind: issue
title: mesh: a curved face whose loop is meridians only on a chart with no pole walks to zero width and meshes as a hole
status: open
opened: 2026-09-18
---


Found by TESS-1's torus measurement (spec deliverable 4) and its sweep
(discipline §5). It is the zero-WIDTH twin of the zero-height class
TESS-1 closed, and TESS-1's structural fact does not reach it.

**Measured, 2026-09-18** (default ε, local, TESS-1's branch). The body:
`witness_bodies::one_circle_cut` (`crates/mesh/tests/common/`) on a
torus about +Z, R = 2, r = 0.5, cut along the MINOR circle at `u = 0`
(`center (2,0,0)`, `axis −Y`, `radius 0.5`), both faces on the torus —
so each face's loop is two half arcs of one meridian and nothing else.

- `topo::validate` and `validate_closed` pass; `validate_geometric`
  refuses `VolumeUncomputable { NotIsoRectangle { "curved face without
  a rim (non-sphere)" } }` — the flux lane (`props::curved_face`)
  refuses the face.
- The doors `curved::require_iso_rectangle_face` cites both ADMIT it:
  `props::require_iso_rectangle` = `Ok`, `require_one_chart_branch` =
  `Ok`. That is the door's documented posture, not a props defect: its
  doc says extent is not a shape question and "a consumer that cannot
  mesh a zero-area face refuses it on its own terms".
- `mesh::tessellate`, debug assertions on: panics at the cross-face
  census, a chord segment an edge of 0 face triangles. Assertions off
  (`CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=false`): `Ok`, patches of
  `[0, 0]` triangles, and `check_mesh` = `Ok(())`
  (`check-mesh-passes-the-empty-mesh`).

**Mechanism, by reading.** Every traversal classifies `Meridian`, so
`walk::loop_polygon` takes its rim-free arm; both arcs carry one `u`,
the polygon has zero width, `curved::require_swept_rectangle` passes
(every entry is on its degenerate box) and the CDT has no inner face.

**Why TESS-1's fact does not transfer.** TESS-1 refuses a loop with no
`TravKind::Meridian`. The mirror fact — no `TravKind::Rim` — is not a
refusal: on a sphere that loop is the pole-to-pole band, which the lane
meshes, taking its u-extent from its two pole junctions. The structural
fact this member wants is narrower — no rim AND a chart with no pole
for a meridian to end on (`Chart::poles()` is empty for a cylinder and
a torus) — and a sphere band whose two meridians sit on ONE column (a
full-height slit) has zero width with poles present, so that fact would
not close the class either. The unit that takes this has to find the
statement; TESS-1 did not guess at one.

**Reach.** Unlike the rim-only sphere cap this body is NOT tier-3
valid, and no one-loop all-meridian torus face can be: a torus cut
along one circle is an annulus, which needs two loops, and a second
loop is `RingOnCurvedFace`. So the reach is the Euler door and any
caller that meshes without validating; `tessellate`'s contract does not
re-validate. The all-RIM torus loop is refused already, typed, at the
shape door (`"torus face without a meridian"` — TESS-1's row
`a_rim_only_torus_face_refuses_at_the_shape_door_on_the_same_fact`).

Not measured: the cylinder and cone members (a loop of generators
only), and the one-column sphere band.

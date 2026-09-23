---
id: rim-free-loop-on-a-poleless-chart-meshes-as-a-hole
kind: issue
title: mesh: a curved face whose loop is meridians only on a chart with no pole walks to zero width and meshes as a hole
status: open
opened: 2026-09-18
priority: P0
cost: D
parent: TESS-5
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
only).

## The one-column sphere band, measured (TESS-1 review, 2026-09-19/20)

A reviewer of TESS-1 constructed the member this row had left
unconstructed, and TESS-1's fix pass landed it as a row:
`crates/mesh/tests/loops_the_meridian_guard_admits.rs`,
`the_one_seam_sphere_is_a_zero_width_band_the_census_reports`. The
body is the one-face, one-seam sphere (V2 / E1 / F1): `mvfs` at the
north pole, one `mev` along a great-circle arc to the south pole, so
the face's loop is that meridian walked down and back. Reported by the
reviewer: tier 3 refuses the body; with debug assertions on
`tessellate` panics at the cross-face census; with them off it returns
`Ok` with `patches = [0]` and `check_mesh = Ok(())`. The landed row
asserts the tier-3 refusal and the census panic (the row is gated on
debug assertions; the assertions-off half is the reviewer's
measurement and is not re-run by any row).

**This settles the question the paragraph above left open.** Both
poles are junctions of that loop, so the narrower statement floated
there — "no rim AND a chart with no pole" — would not close the class:
here there is no rim, there ARE poles, and the domain still has zero
width. The structural fact this class wants is about the COLUMNS the
meridians stand on (all of them one column), which is a coordinate
comparison unless it can be read off incidence (one edge used twice by
one loop, or every meridian incident to the same two pole vertices
with no rim between them). The unit that takes this row starts there.

The same file also pins the spur disguises of the rim-only cap (a
meridian strut from the rim toward the pole, walked up and back): they
carry a meridian, so TESS-1's guard admits them, and the row records
what answers instead.

The title's "on a chart with no pole" is therefore too narrow — the
sphere member has poles. Left as filed (the id is the file name);
read it as "a rim-free loop whose meridians share one column".

## Correction to the two `check_mesh = Ok(())` readings (TESS-4, 2026-09-22)

The two measurements above — the two-face torus (`patches = [0, 0]`)
and the one-seam sphere (`patches = [0]`), both with debug assertions
off — record `check_mesh = Ok(())`. As of TESS-4 both are
`Err(MeshError::NoTriangles)`: every patch is empty, so the whole-mesh
arm answers first. `EmptyPatch { face }` is the verdict for a mixed
mesh, which neither of these is. Nothing else in either reading moves —
the `Ok` from `tessellate`, the patch counts and the census panic with
assertions on are unchanged, and the hole is still a hole.

Not re-executed. The assertions-off run the originals needed is not
needed for this: `check_mesh`'s first act is a `triangle_count == 0`
compare, so the verdict follows from the recorded patch counts alone.
The same mesh shape is pinned hand-built by
`survives_checkmesh_refuses_the_empty_mesh`'s all-patches-empty arm, so
the claim has a row that goes red if it stops holding.

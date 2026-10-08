---
id: a-sphere-crossing-a-sphere-face-off-every-edge-refuses-spheres-meet
kind: issue
title: Two spheres crossing in a circle no edge reaches refuse SpheresMeet: a ball whose seam lies inside another ball's face, plain or carved
status: open
opened: 2026-10-05
priority: P1
cost: H
refs: [a-ring-on-a-sphere-face-has-no-island-winding, carved-sphere-body-cannot-be-classified-or-reused-as-an-operand]
---

Found by the dual review of PR 4046 (`reach/carved-sphere-classify`,
`analysis/reach-dual/4046-r1`, NOTE 4: a third ball crossing a carved body),
measured on that PR's branch with the payloads quoted.

## Measured

Every ball here is the `y`-poled full revolve (seam meridians in the plane
`z = c_z`):

- `ball(1, origin) ∪ ball(0.3, (0, 0, 0.95))` refuses
  `SpheresMeet { operand: A, face: FaceKey(1v1), verdict: Negative { margin: -0.25 } }`;
- the lens union `U = ball(1, (2, 2, 0.5)) ∪ ball(1, (3.4, 2, 0.5))` against
  `ball(0.3, (2, 2, 1.45))` refuses the same, on `FaceKey(2v1)`.

In both, the small ball's sphere crosses the big sphere's face in a whole
circle. The small ball's seam circle lies wholly inside the big ball and the
big ball's edges lie far away, so no edge of either operand crosses a face of
the other. The reduction finds no crossing, and the containment fallback's
sphere extent scan (`boolean::ops::sphere_extent_scan`) meets two spheres
whose faces really do meet. Per `BooleanError::SpheresMeet`'s doc, the join's
sphere-pair arm has no chord to run there: the section is a closed circle
that no edge reaches, which would land as a ring on both faces.

## Not this

- A crossing that an edge does reach lands as a pierce ring and refuses
  `RingOffCylinderChart { kind: Sphere }`. That is a different door, filed as
  `work/tang/a-ring-on-a-sphere-face-has-no-island-winding.md`.
- The case is not specific to carved bodies: the plain-ball witness refuses
  identically.


## 2026-10-07 — the edge-reached door moved (TANG, PR 4211)

The first "Not this" bullet is stale: a crossing an edge reaches still
lands as a pierce ring, but the ring lane now winds it on a sphere
(`chord_join::path_island_winding`). Its ∩ and box ∖ ball build; ∪
and ball ∖ box stop at the result gate on the ringed ball face
(`work/flux/sphere-face-with-a-hole-has-no-closed-form.md`). The
refusal it named, renamed `RingIslandUnread`, is now reached on a sphere
only by a run on both sides of its section plane or bounded by a
non-circle edge. This item's whole-circle case is unchanged.

## Measured (2026-10-08, main at `872b33cc`)

Both witnesses refuse `SpheresMeet { verdict: Negative { margin: -0.25 } }`
under ∪, ∩ and both differences, in both member orders (12 of 12
runs). So do the 42 crossing poses of a sweep over the small ball's
radius (0.05 to 1.5) and its centre's height along the line of centres
(across the crossing range, and `10⁻⁴` and `10⁻⁶` in from either
tangency): 252 of 252 runs. In-band depths (`10⁻⁸` from a tangency)
escalate `Sphere(Nested)` or `Sphere(Apart)`, and equal radii closer
than `10⁻⁴` refuse before the scan.

A two-sided ring is not the shortest door. Re-charting the small ball
by hand, with its pole along the line of centres so its seam reaches
the circle, builds ∩ and small ∖ big. ∪ and big ∖ small stop at the
FLUX gate, on the big face the circle rings. A ring on both faces would
also stop small ∖ big there.

## Built (`join/sphere-pair-whole-circle`)

`boolean::ops::sphere_extent_scan`'s sphere arm now cuts the circle in
rather than refusing. Where two spheres cross in a circle the section
certificate places inside a face of each (R-loop), each operand's
pass cuts its own face along that face's chart meridian through the
circle (`SphereCutIn`, `apply_cut_ins`). Closed groups are cut too: no
re-chart, so both charts stay as built. Each pass cuts its own side, so
an edge of each operand crosses the other's face. The re-entered
crossing layer and the radical-plane join land the circle as chords on
both faces, so no face carries a ring and no run stops at the FLUX
result gate.

The circle comes from the radical plane, `ops::sphere_pair_circle`.
`SpheresMeet` is left for a decided-zero touch.

A closed group that a plane would re-chart and a sphere would cut
refuses typed (`FallbackExtentUnsupported`). The re-chart's graft
renames the face the cut names, which is the two-answers row
`closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`.

Every one of these builds at tiers 3 and 3′ and at the closed form
(`crates/sweep/tests/spheres_crossing_off_every_edge.rs`):

- both witnesses, every op in both orders;
- the 42-pose sweep;
- three tilted centre lines;
- two trimmed balls;
- REACH's pose of a ball crossing both faces of a lens (its own face
  is cut twice; small ∖ lens is two lumps, held to the census
  refusal).

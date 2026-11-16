---
id: valid-ringed-torus-faces-build-but-cannot-be-meshed
kind: issue
title: A valid body whose torus wall holds a spiric ring builds and measures, but tessellation refuses it RingOnCurvedFace as a kernel bug
status: closed
opened: 2026-10-21
priority: P0
cost: M
refs: [5567]
closed: 2026-11-14
---


Found by PR 5567's review (FLUX unit E, `flux/torus-spiric-ring`, NOTE A).

Since unit E, a boolean of a plane-bounded body with a torus whose plane
section is an oval inside a torus wall builds a valid body: the wall
keeps the oval as a ring, the spiric lane measures it (every loop read,
each ring wound against the face), and tier 3 passes it. The review's
corpus (`review/5567-01Lm5UxC`, `crates/sweep/tests/review_5567_probes.rs`,
`review_5567_pads_orders_and_mesh`) has 42 such bodies across the donut
and the one-face-pair tori.

None of them tessellates. Both curved lanes refuse a ringed face before
they read it:

- `crates/mesh/src/curved.rs:183`: `if !face.rings.is_empty()` →
  `TessellateError::RingOnCurvedFace`;
- `crates/mesh/src/trimmed.rs:220`: the same.

The refusal's Display (`crates/mesh/src/types.rs`, `RingOnCurvedFace`)
still says "no construction produces one, so this is a kernel bug". That
was true before unit E and is now false: the body is valid and a public
boolean made it. So a valid body that builds and measures can be neither
meshed nor exported through a mesh, and the refusal blames the kernel.
PR 5567 corrected the variant's doc comment; the Display and its ending
are this row's, since reclassifying the refusal from defect to
capability limit moves its ending.

What to decide: mesh a ringed torus wall (the trimmed lane's
inside/outside classification over every loop), or, until then, refuse
it as a capability limit (`NOT_YET_ENDING`) and not as a kernel bug.

## Evidence (2026-10-28, TESS sphere-arm unit): ringed SPHERE faces

The same refusal holds valid sphere faces, now that the trimmed lane's
sphere arm meshes the circle-bounded ones without rings. A ball's
section that crosses neither seam meridian of a half-band stays a ring
of that face:

- `demos/tour/src/lily.rs` wall 7: the lantern's three tepal seams
  (`seamed_lantern`) carve and measure, and the seam a third of a turn
  round lies inside one half-band of the zone, so drawing the lantern
  refuses `RingOnCurvedFace` (pinned by the wall and by
  `verbs_gate_r1_probes::wall7_the_cone_pair_is_box_looseness_the_ball_meets_the_zone`;
  the one seam that crosses the seam meridians draws);
- `crates/sweep/tests/a_tube_ending_on_a_ball.rs`,
  `a_rim_inside_one_ball_face_holes_it`, which reads its bodies without
  a mesh for this reason.

The sphere arm takes no ringed face either (`tessellate_trimmed` refuses
rings before its chart roster), so the fix this row owes covers it: the
trimmed lane's even-odd classification over every loop.

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Measured on main (2026-11-14, TESSLANE) — re-banded P0

Run at `d6e1eb1f` (probes `reach_probes::a_torus_wall_holding_a_spiric_ring_refuses_at_the_curved_lanes_ring_guard` and `a_ball_face_holding_a_tilted_ring_refuses_at_the_curved_lanes_ring_guard` on `tesslane/measure-reach`), the same at each of three δ.

- **Torus.** A donut (2, 0.5) against bar B2 of `germ_plane_torus_spiric`: ∪ in both orders and donut ∖ bar refuse `RingOnCurvedFace` on the torus face; the other three ops refuse at `trimmed::trim_frontier` on a spiric edge of a torus outer loop, which is `trimmed-tessellation-lacks-torus-and-plane-arms`.
- **Sphere.** A tube of radius 1 tilted 1.2 on a ball of radius √2: both unions and ball ∖ tube refuse `RingOnCurvedFace` on the sphere face. ∩ in both orders and tube ∖ ball mesh through the trimmed sphere arm and pass `check_mesh`. Pinned in `reach_probes::a_ball_face_holding_a_tilted_ring_refuses_at_the_curved_lanes_ring_guard`.
- **The raising site is the curved lane's, every time**: `curved::tessellate_curved`'s ring guard, never `tessellate_trimmed`'s. `trimmed::has_trim_carrier` and `has_tilted_circle` read the outer loop only, and a ringed face's outer loop is iso, so `lane_of` answers `Curved`.
- **What a ring has and lacks.** Present: every ring half-edge carries a stored image (`Spiric` and `Harmonic` on the torus rings, `Projected` on the sphere's), and the planar lane meshes rings (`planar::classify_faces`). Absent: `trim_polygon` reads `face.outer` only, both curved lanes return at their ring guard before reading a loop, and `lane_of`'s predicates do not read rings.
- The refusal's `Display` still says no construction produces a ring on a curved face and that it is a kernel bug.

**P0**: a union of a ball and a tube through it, or of a donut and a bar, is a normal verb on normal geometry, and the body cannot be drawn. It is one cell of the gap `trimmed-tessellation-lacks-torus-and-plane-arms` describes, and is with designers with it (`design: true`).

## Closed

2026-11-14 (TESSLANE unit 3, `a-curved-faces-domain-is-read-from-every-loop`,
branch `tesslane/domain-loops`). A curved face that holds a ring goes to
the trimmed lane (`tessellate::lane_of`), which reads the face's domain
from `topo::Body::face_lift` (the outer loop's lift and every ring's,
placed in its sheet) and triangulates the outer loop less the rings by
the planar lane's even-odd reading. `RingOnCurvedFace` is gone, and a
face whose loops have no reading in one sheet refuses `DomainUnread`
with the door's cause. The donut and B2 mesh in every op at δ `2e-2`
and `5e-3`, and the ball and the tilted tube, ball ∖ tube's displaced
ring included, at `2e-2`, `5e-3` and `10⁻³`
(`reach_probes::a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row`,
`a_ball_face_holding_a_tilted_ring_meshes_on_the_sphere_row`). At
`10⁻³` the ringed torus faces keep two triangles of no area beside a
spiric chord point a lattice point falls within rounding of, the class
`a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area`
holds, measured there and on main's tilted sphere pair. The lily's
seamed lantern draws; giving the scene's lanterns their seams is
`the-lily-lanterns-can-take-their-three-tepal-seams`.

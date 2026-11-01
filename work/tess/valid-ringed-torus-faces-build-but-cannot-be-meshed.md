---
id: valid-ringed-torus-faces-build-but-cannot-be-meshed
kind: issue
title: A valid body whose torus wall holds a spiric ring builds and measures, but tessellation refuses it RingOnCurvedFace as a kernel bug
status: open
opened: 2026-10-21
priority: P2
cost: M
refs: [5567]
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

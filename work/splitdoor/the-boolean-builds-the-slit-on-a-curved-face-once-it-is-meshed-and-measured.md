---
id: the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured
kind: issue
title: Build the slit form on a curved face, retiring EdgeInFaceGap::CurvedFace, once the walk meshes it and props measure its ring
status: open
opened: 2026-10-30
priority: P2
cost: E
---


`crates/topo/src/boolean/slit.rs`, `part_riders`, refuses
`EdgeInFaceGap::CurvedFace` where a line edge lies in a cylinder or
cone face and its material parts there, or a wall of it lies tangent to
the face (PR 6061). The parting itself builds on a curved face unchanged, and its
result passes tiers 3 and 3′; this was measured on PR 6061's first
commits, where `block ∖ trench`
(`crates/sweep/tests/a_keel_on_a_bores_ruling.rs`) built two keel edges
each bounding the bore. It refuses only because two consumers do not
read the result:

- `work/tesslie/the-walk-panics-on-a-slit-in-a-curved-face.md`: the open
  slit;
- `work/nearpoint/a-zero-area-ring-in-a-cylinder-face-measures-degenerate.md`:
  the closed ring.

Once both land, drop the `CurvedFace` gate and its variant, and re-pin
`a_keel_on_a_bores_ruling.rs`'s typed-refusal row and
`verbs_ga_r2_probes.rs`'s `p ∖ b` at their closed forms (`p ∖ b`:
`4π − 2·√0.91·0.36`).

## 2026-10-31 — rows PR 5793 moves onto this gap (CLEAVE)

PR 5793 reads an edge lying in a curved face by its crossings and by
vertex-on-edge records, and reads a conic on a sphere, cylinder or
cone, so these rows reach the `CurvedFace` refusal where they shipped
the one-edge form (several were red on main for it,
`work/tint/eighteen-sweep-slow-rows-are-red-on-main.md`). Each is owed
back at its parted form when the gate drops:

- `a_ruling_through_a_box_corner_edge.rs`,
  `a_wall_through_a_box_edge_builds_every_op_but_the_ring` and
  `a_post_through_the_near_wall_piece_refuses_the_ring`, `w ∖ b`: a
  ring of the near wall piece; the two v-f rests at the corner edge's
  ends land on the piece that holds them (the tests' checks before PR
  5793, under `…_records_the_rest_on_the_piece_that_holds_it` and
  `…_keeps_the_rests_on_their_piece`).
- `a_ruling_lying_on_a_wall.rs`
  `a_prism_edge_on_the_tubes_wall_builds_every_op_but_its_parting`,
  off a seam ruling, `t ∖ b` inside one face (a ring, census
  `(10, 18, 12, 2)` and two v-f rests on the one-edge form) and across
  the rim (a slit, `(9, 20, 13, 1)` with no record);
  `a_ruling_across_an_ellipse_builds_every_op_but_its_slit`, `t ∖ b`, a
  slit at `(9, 20, 13, 1)`.
- `a_pierce_held_in_a_curved_face.rs`: the plate (or slab) less the
  rod and square, in
  `a_cylinder_wall_holding_the_pierce_builds_in_every_op_and_order` and
  `a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order`
  (`parts_curved`), at their closed forms with tier 3′ on the fold.
- `edges_lying_in_a_face_part_wherever_they_lie.rs`
  `a_keel_along_a_cylinder_floors_ruling_refuses_only_its_slit`,
  `A ∖ B`: two coincident keel edges at the closed form, the rod
  through the block and ending inside it.

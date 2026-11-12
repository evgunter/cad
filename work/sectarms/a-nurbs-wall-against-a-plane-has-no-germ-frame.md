---
id: a-nurbs-wall-against-a-plane-has-no-germ-frame
kind: issue
title: A NURBS wall against a plane reaches the join and refuses GermFrameUnsupported: no frame names the germ pair's section
status: closed
opened: 2026-10-31
priority: P2
cost: M
closed: 2026-11-12
---


Filed by SPLICE's `a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`
lane. This is the stop a NURBS-wall pierce reaches once its ring
mints.

## What

`crates/sweep/tests/a_vertex_piercing_a_lens_wall.rs`
`a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_germ_frame`
unions a pyramid with a straight lofted lens blade. The pyramid's apex
pierces the blade's ridge wall, a NURBS face, at the wall's mid chart
point. In both operand orders the vertex-on-face pass mints the
pierce's ring, and the union then refuses at `boolean::join`
`pair_section_frame`'s catch-all
(`frame_refusal`, `FrameError::NoArm`):

`GermFrameUnsupported { a_kind: Nurbs, b_kind: Plane }` (and the mirror)

The germ pair is one of the pyramid's planar faces against the ridge
wall. No frame names a plane × NURBS germ pair's section.

## What it needs

A section frame for a plane × NURBS germ pair, or a typed recourse
naming the rung that would hold it. The row above is the tripwire:
it reds when the frame lands, and is re-pinned to the next stop then.

## A second witness: the wavy loft across its corners (SPLICE, 2026-11-03)

Added by SPLICE's `splice/held-pierce` lane. This row is now the stop of a P0
row's reproducer. `editor-core` `emit_topo`'s
`wavy_loft::a_union_across_a_nurbs_edge_refuses_at_the_spline_walls_germ_frame`
unions, through the recipe, a loft through four 2 × 2 squares at
z = 0, 1, 2, 3, their x offsets alternating −0.3, 0.3, with a box over
x ∈ [1, 3] whose face x = 1 crosses each of the loft's two corner edges
three times. With the spline walls' trims lifted past the sweep's null
edges, the vertex-on-face pass reads the walls, and the union refuses
in the join:

`GermFrameUnsupported { a_face, a_kind: Nurbs, b_face, b_kind: Plane }`

`a_face` is the loft's wall at x ≈ dx + 1, and `b_face` is the box's face
x = 1. The row pins both faces by identity.

**The obligation travels with this stop.** SPLICE's
`a-spline-wall-cannot-say-whether-it-holds-a-pierce-point` owes an
end-to-end row: the union builds, and each corner's crossings are
named through the boolean, the first and third, which have one sense,
tied. That row cannot be written until this frame lands. When it does,
the tripwire above goes red. Its lane should then write that
end-to-end row, or re-pin the next stop and file it.

## Closed (2026-11-12, `sectarms/join-reads-spline-implicit`)

No frame was built: the join reads a plane × spline germ pair on the
pair's section value (`GermLane::Carried`), as it reads every pair the
value carries, and cuts both chords from the carried arc. Every
tripwire above went red and is re-pinned to its next stop:

- `a_vertex_piercing_a_lens_wall_mints_its_ring_and_reaches_the_chords_certificate`
  and editor-core's
  `wavy_loft::a_union_across_a_nurbs_edge_refuses_at_a_carried_chords_certificate`
  stop at rest, where the plane × NURBS lane's own image of a carried
  chord misses it
  (`a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest`);
- `a_saddle_cut_by_its_tangent_plane_refuses_at_its_node_arcs_under_every_op`
  stops at the carry, which lists the arcs to the saddle's node and
  carries none (`a-spline-implicit-arc-is-carried-to-its-node`).

SPLICE's end-to-end row obligation travels with the first of those.

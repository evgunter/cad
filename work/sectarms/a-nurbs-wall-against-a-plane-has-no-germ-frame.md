---
id: a-nurbs-wall-against-a-plane-has-no-germ-frame
kind: issue
title: A NURBS wall against a plane reaches the join and refuses GermFrameUnsupported: no frame names the germ pair's section
status: open
opened: 2026-10-31
priority: P2
cost: M
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

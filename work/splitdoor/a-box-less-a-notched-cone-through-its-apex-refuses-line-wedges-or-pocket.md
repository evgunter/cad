---
id: a-box-less-a-notched-cone-through-its-apex-refuses-line-wedges-or-pocket
kind: issue
title: A box less the notched cone, cut through the slit's tip at the apex, refuses LineWedges or Pocket
status: open
opened: 2026-11-01
priority: P3
cost: M
---

Filed by SPLITDOOR on `splitdoor/cone-apex-boolean` (PR 6266), from its
review.

## What

The notched cone (a cone whose apex is a slit's tip, a tetrahedral
cavity cut along the slit's ruling,
`crates/topo/tests/a_slit_tip_at_a_cone_apex_reads_its_full_turn.rs`)
against the half-space through its apex builds every op but one: the
half-space less the cone, `h ∖ c`, where the cavity lies beyond the
plane, refuses. It holds the cavity as a solid of its own touching the
cone's hole along the slit's ruling, and the result's two coincident
edges there refuse `LineWedges { Overlap }`; where the plane holds a
cavity face (`n = (±0.3, −0.1, 0.1)`), the edge it leaves in that face
refuses `EdgeInFace { Pocket }`. Pinned as refusals in
`a_boolean_through_the_apex_at_the_slits_tip_builds`.

## Where to look

The two solids touch along a line, an edge-edge contact of the result:
whether `boolean::ops::coincident_wedges_disjoint` should read two
coincident edges of distinct solids as a recorded contact rather than an
overlap, and how the Pocket gap arises at a plane flush with a cavity
face.

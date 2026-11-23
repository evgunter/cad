---
id: a-plane-corner-where-a-line-runs-on-tangent-into-an-arc-reads-no-leaving
kind: issue
title: A plane face's corner where a line runs on tangent into an arc reads no leaving: the arc's rise over the line is zero at the vertex, so leaves_line spends its halvings and refuses
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

Found by EMBED's lane `embed/trace-cylinder` (2026-11-21), tracing the
cylinder buckets of the per-solid census (`measure/gate/census-c2a0c41f40.json`
on `embed/gate-measure`). **37 tests**, all of the plane/cylinder edge
bucket, all valid bodies refused at their door: the fillet sweeps of a
mixed line-and-arc rim (`band_open_mixed_chain_shapes` 10,
`band_mixed_corner` 5, `band_mixed_corner_reach` 5,
`band_open_mixed_chain` 4, `band_mixed_rim_shared_supports` 4,
`band_closed_mixed_chain` 3, `band_closed_mixed_chain_names` 3,
`band_closed_mixed_chain_shapes` 2, `review_closed_chain_junctions_r1_probes` 1).

The plane face's outline runs along a line and on into an arc tangent
to it (a rounded rectangle, an L plate's rounded corner, a notch's
rounded floor), so the vertex between them is a corner of 180° with the
two edges G1 there. The fillet's cylinder meets the plane across one of
the two. The plane's part of a cell at that vertex is read by the line
corner (`present_line_corner`, `crates/topo/src/injective.rs`), whose
step 1 is `leaves_line` (`:4632`): the arc's chart curve must leave the
line toward the face's side, its height over the line strictly growing
on each piece from the vertex, decided at
`SELF_INTERSECTION_CORNER_DEPARTS` off `rate × width × lever`. At a
tangent joint the rate is zero at the vertex: the piece that holds the
vertex straddles zero however small it is, so it is halved `LEAVES`
(24) times and refused `Local` (`:4716`). Every cell holding the vertex
reads the same, and the pair ends at `Budget(Depth)`, reported as
`Uncertified { arm: Departure }` on the shared edge.

## Evidence

- Cheapest reproducer: `sweep::all band_mixed_corner_reach::the_short_and_close_witnesses_build_at_their_closed_forms`
  (0.32 s); others `sweep::all band_open_mixed_chain_shapes::two_arcs_in_one_run_build`,
  `editor-core::all band_closed_mixed_chain_names::a_rounded_pockets_mouth_filleted_by_name_removes_pappuss_volume`.
- Measured on `embed/trace-cylinder-measure` (`CV_TRACE=1`, the `CVLL`
  row printed at the refusal): in every one of the 37 the refused piece
  is the vertex's own, `t = (0, 1.4629e-9)` (or its mirror at the arc's
  other end, `t_v = π/2`), at depth 24, with `rate` an interval holding
  zero, for example `[-3.25e-10, -3.1e-17]` or `[-8.7e-17, 1.72e-10]`,
  and `rise` likewise. The partner cylinder is presented (`Of(Side)`)
  throughout.
- Checked: the straddle at the vertex piece is the printed value in all
  37; the G1 joint is the fixtures' construction (line then tangent
  arc), read off the tests, not measured separately.

## Valid or in band

Valid. A tangent line-to-arc outline is the ordinary rounded profile;
the fillet rows build these bodies as their result.

## What a fix needs

At a vertex where the arc continues the line tangentially, the arc's
departure from the line is second order: its height is `κ·s²/2` off the
line, so a reading of the height itself (its sign off the vertex,
decided on the closed form of the circle in the line's frame), or of
the rate's sign on the open piece with the vertex's zero taken from the
joint's tangency, in place of a rate that must be strictly signed on
the closed piece. `leaves_line`, cost M. The same straight-angle joint
on a curved chart (`present_line_corner` over a cylinder's ruling and a
tangent curve) is likely the same reading; not measured here.

---
id: a-plane-cell-holding-both-sides-of-a-thin-face-is-read-only-below-its-width
kind: issue
title: present_plane reads one line or one corner, so a cell of a thin plane face, holding both its long sides or a short side with both its neighbours, is read only once split below the face's width: a thin slab refuses at the depth floor, a long needle spends the pair budget
status: dispatched
opened: 2026-11-21
priority: P0
cost: M
pr: 7334
branch: embed/strip
---


## Finding

Traced by the planar lane of the per-solid census
(`embed/trace-planar`, 2026-11-21), on `embed/gate-measure` at
`1ca743418c` with diagnostic exit lines (`embed/trace-planar-measure`).

`topo::injective`'s `present_plane` (`crates/topo/src/injective.rs`)
presents a plane face's part of a cell only when the cell holds one
line edge, or two line edges that meet at a vertex at a convex corner.
It returns `Unpresented::Local` for two line edges with no common end
(the `[e1, e2]` arm's `let Some(&vtx) = … else`) and for three or more
(the `_ =>` arm). That is right for a cell that a split can still
narrow onto one edge or one corner. On a thin face it cannot.

A face of width `w` holds both of its long sides in every cell that
reaches across it, and its short side with both neighbours in every
cell at that side. `split_by` cuts the metrically longest side
(`Axes::Metric`), so a cell is narrower than `w` only after it has
been cut down to about `w` along the face's length. The two failures
are these:

- **at a short shared edge** (the end of a thin slab), each cell of the
  pair needs about `log₂(L / w)` cuts along `L` before the first one
  across. For `L = 1`, `w = 2·10⁻⁸` that is 26 cuts a cell, 52 for the
  pair, past `max_cell_depth = 40`. The pair refuses
  `Uncertified { arm: Departure }` at an edge.
- **along a long shared edge** (a needle's or sliver's side), every
  piece of the edge splits until it is `w` long. That is `L / w` pairs,
  and the walk refuses `Budget(Pairs { max_cell_pairs: 65536 })` before
  any depth floor.

The departure margin is never reached: no sign is read on a face that
is not presented. So this is not the sliver concession's margin
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`,
PR 7095), though a fix will meet that margin once the strip is
presented. Whether the presented pair then passes is unchecked.

## Trace

Cheapest: `editor-core::all m10_3_r1_probes_interval::a_one_leaf_budget_still_covers_the_box_exactly`
(0.09 s). It is a 1 × 1 slab `2·10⁻⁸` thick, 20 ε at ε = 10⁻⁹. The
D10 rows (`sliver_shell_role::a_thin_wall_refuses_on_a_pair_below_k_eps_and_builds_above_it`)
build walls this thin, and so does every op the family drives. The gate
refuses
`Uncertified { a: 3v1, b: 4v1, shared: Edge(6v1), arm: Departure }`.
Face 3 is the side `y = 0` and face 4 the side `x = 1`, and edge 6 is
their common vertical edge, `2·10⁻⁸` long. All 41 asks of the pair exit
at the `_ =>` arm on both faces, with three edges in each cell
(`xl = [0, 2, 3]`: the shared edge and the slab's top and bottom
edges). The finest cells are `u ∈ (0.4999990, 0.5)` by the whole
thickness `v ∈ (−10⁻⁸, 10⁻⁸)`. The metric split has not yet cut across.

Second: `sweep::all review_arm_vs_reach_probes::review_lengths` has a
step `3·10⁻⁷` wide. It refuses the same way, with cells `1.2·10⁻⁴` by
`3·10⁻⁷`.

## Count

On the census of main `c2a0c41f40`, **departure, edge, plane/plane:
40 of the 111 traced** (of 116). The refusing pair's cells hold three
edges on both faces at a width below 10⁻⁶ (10⁻⁸ to 3·10⁻⁷). The
families are the `m10_*_interval` drivers, `shell_role_reader`,
`missed_ray_side`, `dsc_checks`, `sliver_shell_role`,
`review_arm_vs_reach_probes`, `a_long_bound_beside_a_short_arm` and
`fillet_h6_cap_rim`. Ten more refuse with the same three-edge exit
beside a thin curved shoulder or strut, `10⁻⁵` to `2·10⁻⁶` wide:
`wedge_end_doors` (5), `review_must_carry_rule_r*_probes` (2),
`a_short_shallow_strut_between_wide_walls_is_a_corner` (2) and
`wedge_end_doors::a_cut_across_a_tangent_seam_describes_each_piece_by_the_rule`.
Their other face exits elsewhere (`leaves_line`'s corner departure,
`decide`d negative at the vertex), so they are probably this cause
with a second one. Unchecked.

**`Budget(Pairs)`: 24 of the 98 traced** (of 124). Here the pair
budget is spent on two to four touching pairs of plane faces of a
needle, a sliver, a rod or a strut. Each runs 20,000 to 39,000 asks
that exit both faces with two line edges and no common end (the
`[e1, e2]` arm's `vtx` exit, in 8 traced tests) or past a short edge's
end (the span exit, `sliver_shell_role`'s needle). The other 16 have
the same pair counts as a traced test from their family. The cheapest
is `topo::all review_m3_pr3_consumer::tiny_real_sliver_not_wrongly_refused`
(183,590 exits at the `vtx` exit in one run). The families are
`shell_role_reader`'s needles (4), `contact9_side_codes` (6),
`germ_torus_doors`' rods (5), `sliver_shell_role` (2),
`a_corner_reads_only_its_own_sectors`, `must_carry_rule`,
`review_m4_pr2_transform`, `review_fillet_h6_r1_probes`,
`refusal_concision_chains` and `band_closed_mixed_chain_shapes`. A
rod's face is not ε-thin. `germ_torus_doors::a_rod_inside_the_tube_is_not_passed_silently`
reads cells 3·10⁻³ by 4·10⁻³ on a face about 1.3·10⁻³ wide, so what
matters is the face's width against its length, not against ε.

**`Budget(Depth)`, probably.** In 3 of the 10 depth-bucket tests re-run,
the deepest touching pairs exit at the same two arms. One is
`band_one_cycle_members::a_cup_with_a_handle_and_a_u_tube_refuse_wholly_inside`,
with 573 of 828 asks at the `vtx` exit. The census reads
`Budget(Depth)` there because a distance pair beside them reaches the
floor first. Not checked by a flip.

## What a fix needs

A planar presentation of a strip: the cell clipped to the face between
two parallel line edges, or at a short edge between its two
neighbours. The face there is an intersection of half-planes, convex,
so the same affine reading serves, with the shared side on `y = 0` and
the far lines as the rectangle's bounds. `present_chart`'s clip sides
already read several lines on one axis (`run_on`,
`sides_are_the_faces`). The plane's own arm is the one that does not.

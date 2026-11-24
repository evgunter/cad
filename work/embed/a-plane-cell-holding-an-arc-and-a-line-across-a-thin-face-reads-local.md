---
id: a-plane-cell-holding-an-arc-and-a-line-across-a-thin-face-reads-local
kind: issue
title: curved_walls reads a plane cell with one curved edge alone or at a corner, so a cell across a thin face between an arc and a line, or a curved edge and its neighbours, is read only below the face's width
status: open
opened: 2026-11-22
---


## Finding

`topo::injective`'s `present_plane` (`crates/topo/src/injective.rs`)
hands a cell holding any curved edge to `curved_walls`, before its
line arms. `curved_walls` presents one curved edge alone
(`present_curved`, `present_wall`), or a corner of it with one other
edge (`corner_of`, `present_corner`, `vertex_corner`). A cell that
reaches across a thin plane face holds both sides of it. Between an
arc and a line, or at a curved short side with both its neighbours,
the cell holds no single corner, and the read is `Local` until a split
narrows the cell below the face's width. This is the curved form of
`a-plane-cell-holding-both-sides-of-a-thin-face-is-read-only-below-its-width`,
whose strip reading covers only line edges.

## Evidence (unmeasured beyond the census)

The planar trace of the per-solid census (that row, "Count") found
ten tests that refuse with the same three-edge exit beside a thin
curved shoulder or strut, 1e-5 to 2e-6 wide:

- `wedge_end_doors` (5), and its
  `a_cut_across_a_tangent_seam_describes_each_piece_by_the_rule`;
- `review_must_carry_rule_r*_probes` (2);
- `a_short_shallow_strut_between_wide_walls_is_a_corner` (2).

Their other face exits elsewhere (`leaves_line`'s corner departure,
`decide`d negative at the vertex), so a second cause may stand behind
this one. The strip lane did not run these tests. Measure them first,
with `STRIP_TRACE` on `embed/strip-measure` or `CV_TRACE` on
`embed/gate-measure`, before reading them as this cause.

---
id: a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction
kind: issue
title: A box flush under the ringed top with a corner or edge at the ring's tip refuses ClassificationInvariant: two crossing germs of a vertex pair lie along one direction in one sector entry
status: closed
closed: 2026-10-27
pr: 5812
opened: 2026-10-25
priority: P1
cost: M
branch: cleave/flush-partners-at-a-slit-build
---


Found by PR 5545's review probe (`cleave/fate-cut-ends-at-a-slit-tip`);
the same on main `42af7c0b48` and on the branch, at ε default, 1e-6
and 1e-12, at rest, turned, and flipped and turned.

## What

`y` is the plate `PLATE` less the pyramid lying under its top
(`apex_pyramid(below(lying(0)))`), its top carrying the pyramid's ray
as a zero-area ring from `MEET` to `tip = MEET + bearing(60, 0.5, 0)`.
`x` is a box flush under the top (`[.., ..] × [.., ..] × [0.3, 1]`) or
standing on it (`× [1, 1.5]`) with a corner or edge at the ring's tip:

| box | refuses |
|---|---|
| `[1, tip.x] × [0.5, tip.y] × [0.3, 1]` (corner at the tip) | `x − y`, `x ∩ y`, `y ∪ x` |
| `[1, tip.x] × [0.5, 1.5] × [0.3, 1]` (edge through the tip) | `x − y`, `x ∩ y`, `y ∪ x` |
| `[1, tip.x] × [0.5, tip.y] × [1, 1.5]` (above, corner at the tip) | `x ∪ y`, `y ∪ x` |

each `ClassificationInvariant` "two crossing germs of a vertex pair lie
along one direction in one sector entry" (`boolean/recl.rs`), a claimed
kernel defect. The box's vertex pairs with the ring's tip, where the
top's corner is the full turn round the tip.

## Owed

Build these, or refuse them typed as a construction gap.

## Cause

The box's face flush with the top meets the void's two walls along the
ring's ray: two crossing germs in that face's one sector entry, each
along one of the ring's two edges (`B`'s locus `OnEdge`), at one
direction. `insert::walk_order` (`boolean/insert.rs:779`) orders a
vertex pair's germs by `walks_before`, which reads that pair as a tie,
and nothing else ordered them; with the walk's order broken, the
strut's facing (`leads`, `boolean/insert.rs:1390`; `walk_faces_first`,
`:2166`) met the same tie. Past those, the section's sliver between the
two germs, from the tip along the ring to its apex, read no side:
`join::sliver_along_a_slit` (`boolean/join.rs:5045`) looks for the
ring's tips at the sliver's two ends through `slit_tip_on` (`:4708`),
and the ends are the tips' null-edge copies, which it did not read, so
`resolve_roles_geometric` refused `SectionLoopUndecided`.

## Closed

2026-10-27, PR 5812. Two germs along one direction in one entry, each along an edge of the
other solid, lie on the sides of the line where that edge's flanker
flush with the entry lies: `insert::beside_order` reads them
at second order (`d + δ o`), from each germ's edge's two flankers
(`flankers_of`), and `walk_order`, `leads` and the strut facing read
it through one helper (`walks_before_beside`). `slit_tip_on` reads a vertex's
null-edge copies as the vertex. Every cell of the table builds, in all
six ops, as the closed form says, tier 3 and 3′, its edges on the line
the material's creases, at ε default, 1e-6 and 1e-12, at rest, turned
and flipped and turned; so do the box's edge through the tip along `x`
and its corner at the ring's apex, which refused `Escalated` on main.
Pinned by
`a_ring_crossed_along_its_line_places_its_germs_apart::a_box_at_the_rings_tip_builds_in_every_op`.

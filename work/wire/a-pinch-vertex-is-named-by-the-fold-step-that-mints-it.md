---
id: a-pinch-vertex-is-named-by-the-fold-step-that-mints-it
kind: issue
title: A union's pinch vertex is named by the fold step that mints it, so its name depends on the member order
status: open
opened: 2026-10-06
priority: P1
cost: M
---


## What

Found on TANG's PR 4129, and true on main for two holes as well.

The union of the plate and leaning wedges whose footprints meet at one
vertex of its top (`crates/editor-core/tests/union_pinch_member_order.rs`,
`tilted_holes`) names that vertex differently by member order. The name
comes from whichever fold step minted the vertex
(`names/emit_topo.rs`, `name_boolean_vertices`):

- **Plate first.** The plate's step against the first wedge mints a
  one-segment `Seam` name.
- **Two wedges before the plate.** The plate's step mints a junction of
  four seam lines.
- **All three wedges before the plate.** A junction of six.

Measured with two wedges on main's arms: [1, 2, 0] and [2, 1, 0] carry a
four-line junction, and every other order a one-segment name. Within
one minting configuration the junction name is identical. That much is
pinned by
`the_junction_where_the_wedges_meet_has_one_name_in_every_member_order`.

**The wedges alone** (PR 4129, review 5). A union of the leaning
wedges without the plate mints a different junction name per member
order: three wedges give 3 distinct names over their 6 orders, four
wedges 10 over 24. P − U and U − P against the plate inherit the
divergence. Main refused these unions (`PierceRunsUnordered`, then
`Naming(Emission)`), so the class is newly reached, not newly made.
Each name denotes the right vertex.

No name rebinds to other geometry. But `emit_union_member_order`'s
contract is that a union's names do not depend on the order of its
members, and this one does.

## The shape to give

A union-level canonical name for a vertex the collapse sees minted at
different fold steps. For example, the junction of every seam line
through it in the finished body, whichever step minted it. Pin it with a
row over every member order.

## Since the touch ruling's build (EMIT, 2026-10-12, branch `emit/touch-crossing`)

A union vertex where three or more member edges meet is named as the
junction of the seam lines through it in the finished body
(`emit_union::Flush::junction`). Measured over every member order:
- three and four wedges with the plate: 7 and 15 names on main, 1 each
  now;
- three and four wedges alone: 3 and 10 names on main, 1 each now.

`the_junction_where_the_wedges_meet_has_one_name_in_every_member_order`
now runs every order of three wedges with the plate, three wedges and
four wedges. What is left is two wedges with the plate (3 names, as on
main): two members' edges meeting on a third member's face. It is
filed with the touch ruling's residue as
`work/emit/a-meeting-of-three-members-at-one-point-has-no-order-free-sense.md`.

## Since the vertex-run build (branch `emit/vertex-run`, 2026-10-12)

The union's own names are order-free. Two wedges with the plate are
one name in all 6 member orders: the union names every non-corner
vertex by the lines through it in the finished body
(`emit_union::Flush::meeting`, names README N2), so the meeting point
is the run of the two legs, each `Outside`, whichever step minted it.
Three and four wedges, with and without the plate, are one run of
their legs in every order. Pinned by
`union_pinch_member_order::two_legs_on_the_plate_are_their_touch_in_every_member_order`
and `the_junction_where_the_wedges_meet_has_one_name_in_every_member_order`.

**What is left: the subtracts against the wedges' union** (this row's
"P − U and U − P against the plate inherit the divergence"), measured
over every member order of the union in
`union_pinch_member_order::a_subtract_against_the_wedges_union_names_their_meeting_alike_in_every_member_order`:

- **Three wedges.** The names P − U and U − P carry from the union's
  vertex (`From { read: U, of: <the run> }`) are one set in all 6
  orders, which the test pins. P − U's own seam junction at the point
  is not: it cites a wedge's lateral face, which is a union face piece
  `+ Fragment(Borders(…))` in some orders and bare in others. That is
  the class in
  `work/emit/a-merged-face-pieces-borders-in-a-union-depend-on-member-order.md`.
  The junction itself is
  `work/emit/a-pair-boolean-names-an-edge-bearing-meeting-as-a-seam-junction.md`.
- **Four wedges.** P − U refuses in member order `[3, 2, 1, 0]`
  (`Naming(Emission { "a crossing's edge has no piece the boolean
  classified at the crossing" })`), on main as well as on the branch,
  so a pair boolean's refusal depends on the union's member order.
  The test leaves four wedges out until this is fixed.

---
id: a-borders-join-can-link-two-obstacles-that-meet-only-at-a-notch-apex
kind: issue
title: The Borders chase links every discard one piece edge reaches through a join, which joins two obstacles that meet only at a reflex apex of a non-convex parent
status: closed
closed: 2026-10-19
opened: 2026-10-13
priority: P3
cost: M
---


## What

PR 4949 made the `Borders` chase (`names/borders.rs`, `Obstacles::walls`) follow the output stage's `EdgeJoin` from `kept` to every `gone`. Discards that one piece edge reaches are now one obstacle. The argument for this sits in the module doc and at the join chase: "both border it on its far side".

N2 defines an obstacle as "a connected part of the parent's region". The far-side argument gives connectivity only when the parent's region is locally convex at the join vertex.

**The case that breaks it** (from the PR 4949 review; not built):
- a non-convex parent, e.g. an extruded profile with a V-notch;
- the notch's apex v lies on the far-side wall line;
- the discards over [x, v] and [v, y] touch only at v, so N2 makes them two obstacles;
- v ends at degree 2 and gets joined, so the chase links them into one.

The same premise covers a later split of a joined `kept` edge. That split climbs to `kept`, then follows `joined_from` into discards bordering a stretch it does not lie along.

`Obstacles::record`'s `touches` walk has the same premise from the other end. An earlier step's seam the output stage joined is recorded as lying along the joined edge, whichever of the two the join kept (the `naming.joined_edge` extension of `d.seams`), so `earlier` holds the `kept` edge. A later chain edge whose split lineage (`Obstacles::up_lineage`) reaches that `kept` edge touches the earlier discard and is linked to it in `walls`, though it may lie along the stretch `kept` held before the join, not the joined seam's, which that discard does not border.

**What guards this today:**
- A gross over-join (join every discard of the parent) turns four `emit_union_borders` rows and `name_words_corpus` red.
- No row covers a non-convex parent at a join.
- No row pins the join rule directly. It is held only by order-consistency over `cross`, and an over-join that every order makes alike passes that.

**Owed:**
- a fixture of a non-convex parent at a join;
- a join rule that reads connectivity (N2) rather than reachability.

## Measured (2026-10-14)

Two fixtures in `editor-core` `emit_union_borders`. Both name alike in
every member order and pin today's reading.

**`regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle_today`.**
- Members:
  - the plate notched at (1.5, 1.2);
  - `m` = [−1, 2] × [1.2, 3] × [0.5, 1.5];
  - `s` = [0.6, 0.8] × [−1, 1.3] × [0.47, 1.53].
- The kernel discards the top's two regions beyond `m`'s wall as **two
  faces** in every member order, neither loop passing the apex twice
  (re-measured by both designers of fork-log row 171, 2026-10-14). What
  links them is the output stage's join at the apex: both regions'
  stretches settle on the one joined edge.
- The top's right piece reads {`m:x=2.00`, `m:y=1.20`, `s:x=0.80`}.
  With the two regions read as two obstacles it would be
  {`m:y=1.20`, `s:x=0.80`}.

**`two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle_today`.**
- `m` is split at x = 1.5 into `m1` and `m2`, so each region is its own
  member's discard.
- The right piece again cites `m2:x=2.00`.
- Two readings were tried: not linking discards that share an edge
  only through a join, and attributing a piece edge to each obstacle it
  reaches separately. Neither fixture moved. `s` splits the joined edge afterwards, and the
  left piece's piece of it climbs to the joined edge, which runs along
  both regions. So the left piece meets the right region too, and that
  region divides.

**What a fix needs, either way.** Reading the two regions as two
obstacles takes both of:
1. (Moot: the kernel never discards the regions as one pinched face.)
2. A joined edge's stretches kept apart after the join: where on the
   kept edge the joined-away vertex lay, so that a later piece of that
   edge is attributed to the stretch it lies along. The vertex is dead
   by then. This is either a parameter read on the carrier (N2's ranks
   already read one, along a line by its own parameter) or a topo
   record of the vertex's point carried through later splits.

## The question (to Ev, fork-log row 171)

N2 says "an obstacle is a connected part of the parent's region that no
piece holds". Is a region that meets another only at a point connected
to it? The `[ev]` PR proposes "connected through its closure", which
the designer pair converged on, and states it in N2.

**A wall through the apex** (measured by both designers): a splitter
`s2` whose wall ends at the apex makes today's names depend on member
order. The left piece reads {`m:y=1.20`, `s2:x=1.50`} in 2 of 6 orders
(the apex is joined away, so the join chase links the two regions) and
{`s2:x=1.50`} in the other 4 (the apex stays a live vertex, and nothing
links two discards that share only a live vertex). So "keep today's
names" is not a coherent option under either reading.

**What closure needs** (prototyped in `borders.rs` only, by one
designer): each `Discard` keeps its stretch-end vertices, canonicalised
through the step's `null_copies`, `cone_splits` and `graft_vertices`,
and `walls` joins discards that share one. All six `s2` orders then
read left {`m:y=1.20`, `s2:x=1.50`}, right {`m:x=2.00`, `m:y=1.20`,
`s2:x=1.70`}. The notch fixtures do not move. `emit_union_borders`,
`name_words_corpus`, `emit_union_rim_piece_ranks`,
`emit_union_flush_names`, `emit_shared_rim_several` and
`union_member_orders_decide_alike` stay green. No new kernel record,
no parameter read, nothing refuses.

**The cost of closure:** a boss whose corner exactly touches a divider
becomes part of it, and is cited.

## Ev's ruling: closure (PR 5153, 2026-10-19)

Ev took recommendation 1 ("the recommendation for 1 makes sense"). Two
lost regions that meet only at a point are one obstacle, and N2 says so
(PR 5153). The build is the converged design: a vertex link in
`borders.rs`, canonicalised through `null_copies`, the cone splits and
the graft's vertex maps, with all six orders of the wall-through-the-apex
probe naming alike.

## Closed

Built in `names/borders.rs` (`Obstacles`), branch `emit/borders-closure`.
Closure is two channels; each goes red without it.

- **A point the result holds as a vertex** (`Discard::meets`). Each
  stretch end is settled through the step's `Settler` (the zip's
  fusions and cone copies, the output stage's join and merge) to the
  live vertices it lies at. An edge at one of those meets another
  discard when it is, or descends from, an edge that discard borders end
  to end (`Discard::whole`), of this step or an earlier one: an edge
  `between` found for a stretch, and no edge the output stage joined,
  whose other half the discard may not border. Identity only; no point
  is compared.
  - Without the channel, or with it restricted to earlier steps'
    edges, three rows go red: `a_boss_whose_corner_touches_the_divider_is_part_of_it`,
    `a_wall_through_a_notch_apex_names_alike_in_every_order` (two
    discards of one step meeting at the point) and
    `a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider`.
  - At 1379d6859f, reading the raw stretch-end keys instead of the
    settled vertices turned the first two red. That mutant was not
    re-run after the change to `whole`.
  - **Not kept: targets read from `Discard::seams`.** That reading linked
    a corner on the far half of a joined seam to the discard along the
    near half, and in all 8 publishing orders it cited a diamond that
    touches no lost region
    (`a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`).
    Holding only `whole` costs 2 of those orders a corner on the near
    half of a seam joined in an earlier step. That case is pinned, and
    the question of which half is open in
    `borders-records-a-join-twice-and-reads-lineage-three-ways`.
- **A point the output stage joined away** (`Obstacles::joined_from`,
  the chase from a piece edge to every edge a join made it of).
  Without it, `two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`,
  `emit_union_rim_piece_ranks`' flush-union and two-orders rows and
  `union_member_orders_decide_alike`'s emit-corpus row go red.
- **Not kept: a key link with key classes.** The first build joined
  discards sharing a stretch-end key, each key widened to every key
  the step held for the point (`null_copies`, `cone_splits`,
  `vertex_merges`). Dropping the classes, or any one source of them,
  turned nothing red, and the vertex channel above, read across all
  steps, covers what the key link did within one. The key link also
  compared keys across steps, which holds only while each step's
  result is in operand A's keys.
- **A result that is B alone** (`(Absent, Direct)`, the boundary-disjoint
  fallback: a member that engulfs the accumulation) resets the record.
  None of the earlier region survives, and its keys are another
  arena's. Any other layout without A held directly refuses. Without
  the reset, `a_member_engulfing_the_others_reads_only_what_follows_it`
  trips `note_split`'s check, because an edge key of the new arena
  collides with an earlier one ("recorded with two parents"). It trips
  on origin/main too, so the reset fixes a latent defect there.
- **Fixtures** (`editor-core/tests/emit_union_borders.rs`):
  - `a_wall_through_a_notch_apex_names_alike_in_every_order`: all six
    orders give left {`m:y=1.20`, `s2:x=1.50`}, right {`m:x=2.00`,
    `m:y=1.20`, `s2:x=1.70`}. On main, order (plate, s2, m) gave left
    {`s2:x=1.50`}.
  - `a_boss_whose_corner_touches_the_divider_is_part_of_it` (the cost
    Ev accepted): a diamond boss whose corner rests on the slab's wall.
    The left piece cites the diamond's four walls beside `slab:x=1.40`
    in all six orders.
  - `a_member_engulfing_the_others_reads_only_what_follows_it`: 24
    orders, one table.
  - `regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle` and
    `two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`
    lose `_today`. Unchanged.
  - The joined seam, over the stepped plate. In each fixture with a
    diamond, 16 of 24 orders refuse, the same orders as on main
    (`a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early`).
    - `a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`:
      the 8 publishing orders give one table, with no diamond cited.
    - `a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider`:
      6 orders cite the diamond, and 2 are held apart.
    - `a_divider_beside_a_joined_seam_does_not_reach_across_it`: 6
      orders, 4 right, and 2 held apart for the chain channel's
      over-link, the same as main.
- **Measured, nothing moved:** `emit_union_borders` 22/22 (+3 new, 1
  ignored as before), `name_words_corpus`, `emit_union_rim_piece_ranks`,
  `emit_union_flush_names`, `emit_shared_rim_several` and
  `union_member_orders_decide_alike`: 56/56 with the slow set.
- N2 (`names/README.md`) states the cost: a boss or notch on one piece
  is cited when a point of it touches a divider.

---
id: borders-records-a-join-twice-and-reads-lineage-three-ways
kind: issue
title: borders.rs spells a join's seam handover twice (eager seams extension and lazy joined_from chase) and reads split lineage from three sources
status: open
opened: 2026-10-13
priority: P3
cost: M
design: true
---


## What

These are style findings from the PR 4949 review, recorded and not gating.

**`borders.rs`:**
- **One fact spelled twice.** "A join makes the gone edge's seams the kept edge's" is written eagerly, by extending each earlier discard's `seams` through `joined_edge`, and lazily, by the `joined_from` chase. A mutation shows neither alone covers what both do, and no comment says which case each owns.
- **Three lineage sources.** Split lineage comes from `body.edge_provenance_of` on the final body, from `split_from` over each step's live edges, and from `split_from` over the discard chains. For a union the first is subsumed by the second, so the two-level match is left over from the old dead-split design.
- **Duplicate seam entries.** The seam extension pushes a `kept` edge twice when two seams join into it, and its `contains` filter is O(n²).
- **Silent cross-chain cycles.** Cycle detection is per chain, so a loop that crosses chains is absorbed by `seen` rather than refused.

**`emit_union.rs`, `group_member_edges`:** the `From` arm and the new arm that lies within a member edge are near-duplicates. They differ only in `least_edge` against `least_within`.

**Tests:** `emit_union_flush_names` keeps an empty `KNOWN_REFUSING`, with its machinery, where `emit_union_rim_piece_ranks` deleted its admissions outright. One shape should be picked.

## Since (2026-10-19, `emit/borders-closure`)

The `joined_from` chase is now documented as closure's channel for a
point the output stage joined away (`Obstacles::joined_from`). Without
it, four rows of the six naming suites go red
(`two_members_regions_meeting_only_at_a_notch_apex_are_read_as_one_obstacle`
among them). The eager `seams` extension through `joined_edge` is
unchanged, so the two spellings of the handover remain.

## A joined seam is bordered only in part (2026-10-19, `emit/borders-closure`, PR 5356's delta review)

**The defect.** `Discard::seams` holds edges a discard borders only in
part. There are two ways in:
- an edge the step's own output stage joined
  (`BooleanNaming::stretch_through_joins`);
- an earlier step's seam a later join lengthened (the eager
  `joined_edge` extension above).

Every reader that takes membership in `seams` to mean "borders it end to
end" over-links whenever the join's other half is not that discard's.
The review's fixture is the stepped plate in
`a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early`.
There, `m`'s wall merges with the plate's own riser, so the lower top's
edge at `y = 1` is joined at `x = 1`, and `m`'s discard borders only
the `x ≤ 1` half.

**What is fixed.** The settled-vertex channel now targets only
`Discard::whole`: the edges `between` found, never a join's. Each is
dropped from an earlier discard once a later step's join lengthens it
or kills it. That premise is now stated in `Discard::whole` and
`Discard::meets`.
- A diamond whose corner rests on the riser's half is no longer cited,
  in all 8 orders that publish
  (`emit_union_borders::a_corner_touching_the_far_half_of_a_joined_seam_is_not_cited`;
  it was cited in all 8 before).
- `a_boss_whose_corner_touches_the_divider_is_part_of_it` and
  `a_wall_through_a_notch_apex_names_alike_in_every_order` are
  unchanged.
- No fixture reaches the drop on a later join. In a scratch probe with
  5 members, where the join at the riser can fall in a later step than
  the seam, removing the drop changed no order. Every order that would
  exercise it either refuses or names alike.

**What is not fixed: the chain channel and `reached`.** Both still read
`seams`, and so the join, as whole.
`emit_union_borders::a_divider_beside_a_joined_seam_does_not_reach_across_it`
(the plate, `m`, and `t` at `x` in `[1.5,1.7]`):
- 4 of 6 orders are right: the lower top and the merged wall are each
  {`t:x=1.50`} / {`t:x=1.70`}.
- In (plate, m, t) and (m, plate, t), `t`'s chain along the joined edge
  links it to `m`'s discard, and both parents cite the merged wall. The
  fixture holds these two orders apart.
- origin/main gives the same two outcomes. Before the fix, a third
  outcome came from the vertex channel.

**The readers of `seams`, swept** (`rg 'seams|seam_of' names/borders.rs`).
The pattern cannot match a reader through another name, and
`borders.rs` has none.
- **`touches`, the chain channel:** over-links, as measured above.
- **`reached`:** a piece edge's lineage reaches every discard holding
  the joined edge in `seams`.
- **`seam_of`'s grouping in `walls`:** two discards holding one joined
  edge are linked even when each borders a different half.

No fixture yet separates the last two from the chain channel.

**What the fix costs: a corner on the near half.**
`emit_union_borders::a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider`
moves the diamond's corner to `(0.75, 1, 1)`, on the half `m`'s
discard does border, with `t` at `x` in `[0.2,0.4]`.
- Before the fix: all 8 publishing orders cited the diamond, correctly,
  through the over-link.
- After the fix: (m, t, plate, dia) and (t, m, plate, dia) do not cite
  it. In those orders the join is made in the plate's step, before the
  diamond's, and the join's edge is not `whole` for `m`'s discard. The
  other 6 orders cite it. The fixture holds the two orders apart.

## The question

Whether a point on a joined edge lies on the half a discard borders is
not in the record. Identity cannot tell, because the joined-away vertex
is dead and a later split piece's lineage climbs to the whole joined
edge. Both the near-half under-link and the chain channel's over-link
need the answer.

The options:
1. **A kernel record.** The output stage's `EdgeJoin` would carry where
   `vertex` lay, as a parameter on `kept`'s carrier, and a later split's
   lineage would carry it on. The emitter would then keep a joined seam
   as an interval of the joined edge.
2. **A parameter read in the emitter.** Read the joined-away point off
   the stretch's live end and the step's body, and compare each later
   piece by its parameter along the carrier. N2 already reads a
   parameter for ranks. The closure build was identity-only, and this
   would make it compare points.
3. **Accept it.** Keep the under-link for a corner on the near half of
   a seam joined in an earlier step, and the chain channel's over-link,
   and document them in N2.

Both 1 and 2 also retire the eager extension and the chain channel's
over-link, because the handover becomes one spelling: an interval.

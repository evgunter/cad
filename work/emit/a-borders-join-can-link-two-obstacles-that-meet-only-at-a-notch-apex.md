---
id: a-borders-join-can-link-two-obstacles-that-meet-only-at-a-notch-apex
kind: issue
title: The Borders chase links every discard one piece edge reaches through a join, which joins two obstacles that meet only at a reflex apex of a non-convex parent
status: open
opened: 2026-10-13
priority: P3
cost: M
design: true
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
- The kernel discards the top's two regions beyond `m`'s wall as **one
  face**. Its record borders (2, 1.2)→apex and apex→(0, 1.2), both
  through the join, and the `x = 2` seam. So no join chase is involved:
  the over-join is inside one `DiscardRow`.
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
1. A discarded face pinched at a vertex of its own boundary, recorded
   per connected part: its bordered stretches and chains grouped by the
   side of the pinch they lie on. This is a topo record; the loop walk
   knows it.
2. A joined edge's stretches kept apart after the join: where on the
   kept edge the joined-away vertex lay, so that a later piece of that
   edge is attributed to the stretch it lies along. The vertex is dead
   by then. This is either a parameter read on the carrier (N2's ranks
   already read one, along a line by its own parameter) or a topo
   record of the vertex's point carried through later splits.

## The question

N2: "An obstacle is a connected part of the parent's region that no
piece holds." Does a region that meets itself only at a point count
as connected?

- **(a) Yes, the closure is connected.**
  - Today's names are right, and the two fixtures above are the pins.
  - N2's text says "connected through its closure", or "meeting at a
    point counts".
  - Cost: a piece cites the walls of a region that meets the divider
    only at a point.
- **(b) No, interior-connected.**
  - Both steps above are needed. Step 2 puts a parameter read or a
    point record into the naming record.
  - The two fixtures flip to `{m:y=1.20, s:x=0.80}`.

This row's other premise has the same answer as the question. That
premise is the `touches` walk, which links a later chain edge to an
earlier discard through a joined edge.

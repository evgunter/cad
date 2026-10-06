---
id: in-face-pierce-rings-pair-across-the-gap
kind: issue
title: A bar whose section closes inside one wall face refuses RingHomingAmbiguous when its arc is wider than the gap between its rings: the ring lane's loose-end pairing reads the germ line as planar
status: closed
opened: 2026-10-02
priority: P0
cost: M
closed: 2026-10-06
branch: tang/in-face-ring-pairing
pr: 4132
---


## What

A bar through the unit pipe (`crates/sweep/tests/verbs_germarms.rs`'s
`pipe`, `r = 1`) whose section closes inside one wall face — the bar
`x ∈ (−1.1, 1.1)`, `y ∈ [y0, y1]` with `0 < y0 < y1 < 1` — builds when
its two rings are close relative to their arcs and refuses
`Join(RingHomingAmbiguous)` when they are not. Measured by the dual
review of PR 3851 (TANG, `tang/pierce-ring`): the refusal holds exactly
when

    asin y1 − asin y0 > π − 2·asin y1,

i.e. when each ring's azimuth arc is wider than the azimuth gap between
the two rings (they sit at `±` the same azimuths about `x = 0`): 42 of
42 symmetric poses, and 42 of 150 asymmetric ones. Witnesses:
`y = (0.1, 0.9)`, `y = (0.15, 0.95)`. The row that builds,
`verbs_germarms::a_bar_whose_section_closes_inside_one_wall_face_builds`
(`y = (0.15, 0.7)`), is on the other side of the inequality.

## Cause

Upstream pairing. `find_match` and `loose_partners`
(`crates/topo/src/boolean/join.rs:721`, `:1393`) pair loose ends by the
planar nearest-facing rule (`bool_join_nearest`, chord length). On a
wall the germ line is a circle, and once the arc of each ring is wider
than the gap between them the nearest facing end of a ring is on the
OTHER ring, across the gap: the chords are laid across the gap through
both rings, and the re-homing that follows finds a ring on its run's
boundary (`RingHomingAmbiguous`).

It fails loud today. The fix is a pairing that reads the wall's germ
line along its own conic (the order along the section ellipse that
`SectionCrossings` already reads for a curved face crossed more than
twice), not nearest Euclidean distance.

## Review tier

The orchestrator's read: regression rows and doc only; the fixes
landed upstream.

## Closed

Fixed upstream before this unit was dispatched, by JOIN's PR 4008
(`join/pocket-ring-rehoming`): each germ ranks its partners by the turn
along the section conic (`bool_join_arc_travel`), and `find_match`
takes the nearest of those per-germ pairs by arm, then chord. That is
the class fix this item asked for, so no kernel change rides here.

Measured on a probe of 1320 ops (55 `(y0, y1)` pairs, two bar spans in
`x`, two heights, all six ops including both member orders): 216
refuse `RingHomingAmbiguous` at `69a8a891` (PR 3851's merge) and at
`4fbf874b` (PR 4008's parent), all on the wide side of the inequality;
none refuse and every body matches its closed form at `f8aabaae`
(PR 4008's merge) and on `46aad757`.

The unit adds
`verbs_germarms::in_face_rings_pair_along_the_wall_whatever_the_arc_and_the_gap`:
ten poses straddling `asin y1 − asin y0 = π − 2·asin y1` (two of them
0.016 and 0.014 rad either side of it), two spans in `x`, the axis at
and off the origin, the pipe and the bored block. Every op in both
member orders holds tiers 1–3 and 3′, its closed-form volume and the
family's exact census. Red at PR 4008's parent on its first wide pose;
green on `46aad757`.

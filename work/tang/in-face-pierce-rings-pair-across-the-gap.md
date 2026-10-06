---
id: in-face-pierce-rings-pair-across-the-gap
kind: issue
title: A bar whose section closes inside one wall face refuses RingHomingAmbiguous when its arc is wider than the gap between its rings: the ring lane's loose-end pairing reads the germ line as planar
status: open
opened: 2026-10-02
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

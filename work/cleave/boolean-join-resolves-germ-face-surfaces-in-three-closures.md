---
id: boolean-join-resolves-germ-face-surfaces-in-three-closures
kind: issue
title: boolean::join resolves a germ's face surfaces in three closures with one message
status: closed
opened: 2026-10-13
priority: P3
cost: E
branch: cleave/join-face-surface-one-home
closed: 2026-10-14
---

Found by PR 4863's narrow re-review (verdict MERGE); filed rather than held for.

## What

PR 4863 removed `face_pair_reading` and added a third copy of the closure
that resolves a germ's face surface. All three carry the same message,
`"germ face surface no longer resolves"`:
- `segment_arm` (boolean/join.rs, around :303);
- `bool_connect`'s `surf_of` (around :1024);
- `face_pair_section_loops` (around :2238).

`bool_connect` also resolves the same two surfaces again for each segment.

## Fix

Give face-surface resolution one home, and resolve each segment's pair
once.

Two other notes from the same re-review:
- The hoisted `segment_arm` read (before surgery) is unguarded, because
  the before-surgery and after-surgery reads never differed in the suite.
- The door pin `seams_lying_on_the_walls_...` is the only pose that reaches
  the in-band and lever arms.

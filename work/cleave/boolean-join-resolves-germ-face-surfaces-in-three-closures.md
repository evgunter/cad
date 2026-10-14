---
id: boolean-join-resolves-germ-face-surfaces-in-three-closures
kind: issue
title: boolean::join resolves a germ's face surfaces in three closures with one message
status: open
opened: 2026-10-13
priority: P3
cost: E
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

## Built (branch cleave/join-face-surface-one-home)

- One home: `join::germ_face_surface` resolves a germ face's surface and
  key, with the one desync message. `face_pair_section_loops` and
  `wrap_site_segments`' planar read (which read a non-resolving face as
  "not planar") go through it; `choose_roles`' ring-lane read (which
  read a non-resolving face as "not a quadric") is now loud too.
- One read per segment: `bool_connect` reads each segment's pair once,
  before any surgery, as `SegmentPair` (both surfaces with their keys
  and the kind arm, `segment_arm` taking the resolved surfaces). The
  per-segment re-read after earlier segments' surgery is gone; no join
  operation rewrites a face's surface key.
- The hoisted read: mutation-tested. Moving the read back into the loop
  (after earlier segments' surgery) left all 330 cylinder, ruling, rod
  and seam rows in `sweep::all` green, and a probe of one-face walls
  (seams at 0 and π, tips 0.2–2.5·zero, heights 0.3 and 0.05) returned
  byte-identical outcomes both ways. No pin was added because no pose
  found tells the two apart: the first segment always reads the pristine
  state, an in-band or apart reading refuses there before any surgery,
  and a parallel reading stays parallel at the second segment unless a
  chord end lands outside both walls' vertex ball, which no pose tried
  did. The doc on `segment_arm` states the invariant the hoist keeps:
  every segment of one face pair takes the one arm that pair reads.
- Second pose: the seams door pin now tips about `x` and about `y`, and
  `one_face_walls_read_their_axes_as_an_angle` reaches the in-band
  reading through the frame dispatch with every germ inside a face.

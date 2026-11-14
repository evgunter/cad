---
id: a-height-in-the-gap-escalates-at-metres-in-the-parallel-and-offset-arms
kind: issue
title: conic_conic's parallel arm and the router's level offset escalate a height in the gap where the pair is metres apart in-plane, the class PR 6623 fixed for transverse
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6642
---


Found by the FULL review of PR 6623 (`onecopy/needle-tilts`, bfd3b71e), finding F3. It was measured, and reads the same on base and head.

PR 6623 lets `transverse` read a height extreme in the escalation gap through its window (`height_extremes`). A height is only a lower bound on the distance, so the meeting question belongs to the window, not to the height's own escalation. Two sibling arms still escalate on the height alone:

1. **`conic_conic`'s swing-zero arm.** This is the `height?` under "A conic whose swing reads zero…". Two circles in parallel planes 3–8·z apart, centres 2.5–3 m apart (so a gap of 0.5–1 m), tilted by 1e-14, give `Esc(curve_pair_pose_parallel)`.
2. **The router's level offset.** A line parallel to a circle's plane, at a height of 3–8·z and 4 m away, gives `Esc(curve_pair_pose_offset)`.

Both are refusals, not wrong answers. They are the same class: a lower bound on distance read as the meeting question. Route each through the in-plane distance or a window, so that a height in the gap with a definite in-plane gap reads Apart.

The sibling `circle-torus-plane-height-refuses-a-graze-the-pose-places-as-a-touch` is the same shape.

**Pins:** both poses above, read in both orders.

## Resolved (PR 6642)

- **The swing-zero arm.** A height the parallel screen did not clear is a slant. The arms in one plane read the pair, and their candidates are read on both spans, the slant included.
- **The router.** `route` takes `(offset, Offset)`. A line's height above a conic's plane in the gap is level, and the arm in the plane reads it. Two lines' distance in the gap still escalates.

Pinned by `order_rows::a_height_in_the_gap_beside_a_definite_gap_in_the_plane_reads_apart`, in both orders: apart in the plane reads apart, touching in the plane escalates, and in the zero band meets once.

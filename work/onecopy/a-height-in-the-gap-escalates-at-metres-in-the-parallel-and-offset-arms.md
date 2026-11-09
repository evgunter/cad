---
id: a-height-in-the-gap-escalates-at-metres-in-the-parallel-and-offset-arms
kind: issue
title: conic_conic's parallel arm and the router's level offset escalate a height in the gap where the pair is metres apart in-plane, the class PR 6623 fixed for transverse
status: open
opened: 2026-11-09
---


Found by the FULL review of PR 6623 (`onecopy/needle-tilts`, bfd3b71e), finding F3. It was measured, and reads the same on base and head.

PR 6623 lets `transverse` read a height extreme in the escalation gap through its window (`height_extremes`). A height is only a lower bound on the distance, so the meeting question belongs to the window, not to the height's own escalation. Two sibling arms still escalate on the height alone:

1. **`conic_conic`'s swing-zero arm.** This is the `height?` under "A conic whose swing reads zero…". Two circles in parallel planes 3–8·z apart, centres 2.5–3 m apart (so a gap of 0.5–1 m), tilted by 1e-14, give `Esc(curve_pair_pose_parallel)`.
2. **The router's level offset.** A line parallel to a circle's plane, at a height of 3–8·z and 4 m away, gives `Esc(curve_pair_pose_offset)`.

Both are refusals, not wrong answers. They are the same class: a lower bound on distance read as the meeting question. Route each through the in-plane distance or a window, so that a height in the gap with a definite in-plane gap reads Apart.

The sibling `circle-torus-plane-height-refuses-a-graze-the-pose-places-as-a-touch` is the same shape.

**Pins:** both poses above, read in both orders.

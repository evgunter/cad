---
id: the-junction-boundary-guard-refuses-a-target-seam-within-its-reach
kind: issue
title: The junction end's boundary guard has no seam exemption: a periodic target face's seam edge within the spread's reach reads as a near boundary and refuses
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [the-junction-side-reading-assumes-no-other-target-boundary-within-its-reach, 7757]
---


Found on PR 7757's review, a single FULL review of `heldrecords/near-boundary-m`, and confirmed by the lane.

## What

`reduce::junction_end_boundary_clear`, which since PR 7757 is asked of every window, gives a periodic face's seam edge no exemption of its own (`reduce.rs` ~2369).
- **A seam through the end** passes only because it happens to leave on the target's side, through `leaves_on_its_side`.
- **A seam within σ that does not pass through the end** reads as a near boundary and refuses `JunctionEndUnread { NearBoundary }`. A seam is no boundary of the face's point set, so the side reading is not threatened there.
- The row it came from said in its design text that "a periodic face's seam edge through the end is not one". The code reaches that result through the geometry, not by a rule.

This is liveness, not soundness: a refusal where a build is owed. Nothing measured has hit it: all 1226 pivot-m reads in PR 7757's measurement were clear.

## Done when

A seam edge of the target face is exempt from the guard by rule. A pose with a pipe whose seam ruling lies within σ of a junction end on the other body's face, not through the end, builds at the closed form, passes t3 and t3′ at three ε, and pins the case.

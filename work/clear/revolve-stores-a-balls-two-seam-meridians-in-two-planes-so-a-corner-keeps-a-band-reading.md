---
id: revolve-stores-a-balls-two-seam-meridians-in-two-planes-so-a-corner-keeps-a-band-reading
kind: issue
title: A revolved ball's two seam meridians are stored in planes 1.2e-16 rad apart, so no separator lists both and the injectivity check keeps a band reading at a corner (against change 2)
status: open
opened: 2026-11-13
priority: P0
cost: M
---



## Finding

Found by CLEAR's lane B (`clear/own-separators`, PR 6905).

Ev's change 2 (PR 6821), as specced in `the-at-rest-gate-certifies-embedding`
`### Lane B` (approved, PR 6893), leaves no band reading at any Zero
site. PR 6905 removed the band reading at the separators the engine
builds, with one exception: a `Zero::Corner` side that the separator
does not list is still read over the side.

The exception exists because of PR 6892's two-ball union
(`a_two_ball_union_holds_with_its_poles_read`). At the pole:
- one face's corner lies on one seam meridian, and the other face's
  corner on the other;
- the two meridians are stored in planes about `1.2e-16` rad apart, so
  a plane built from either edge lists only that edge.

With membership alone, that pin moves from Holds to
`Uncertified { Departure }` at depth 40.

## What a fix needs

The fix is by construction:
1. `revolve` stores a ball's two seam meridians as arcs of one great
   circle, with one frame (`crates/sweep`, the revolve door).
2. The plane built from either meridian then lists both.
3. The corner's band reading goes.

Pin: the two-ball union holds with the corner reading removed.
Mutant: the meridians stored in two frames, which refuses.

Seam: SWEEP's revolve door, plus any stored-row or tier-3 pin that
reads the seam frames.

Banded P0: it completes lane B's part of the at-rest gate (P0).

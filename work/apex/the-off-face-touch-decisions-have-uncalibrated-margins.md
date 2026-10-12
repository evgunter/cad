---
id: the-off-face-touch-decisions-have-uncalibrated-margins
kind: issue
title: The four off-face touch decisions have uncalibrated margins and no K samples
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [4128]
---

Found by the review of PR 4128 (NOTE-2). The off-face reading adds
four decisions, none of them sampled by the K-telemetry sweep
(`scripts/k_probe_sweep.sh`): the corpus and the demo tour hold no
edge tangent to a curved carrier off its face, so their margins are
uncalibrated.

- `bool_touch_piece_off_axis` (`crates/topo/src/boolean/carrier_touch.rs:234`):
  a piece keeps clear of the carrier's medial axis, the premise of the
  second-order bound.
- `bool_touch_piece_clear` (`carrier_touch.rs:241`): a piece's lower
  bound on its distance from the carrier is positive.
- `bool_touch_ball_in_reach` (`carrier_touch.rs:275`): a touch's ball
  is inside the carrier's reach.
- `bool_touch_edge_clear` (`carrier_touch.rs:315`): a boundary vertex
  or edge is clear of the ball.

## Probe candidates

The rows of `crates/sweep/tests/pierce_tangent_off_face.rs` reach all
four, and are cheap (under a second each): the lens and its touching
bricks (below the face, by its edge alone, three balls outside the
rim, 0.3 balls outside it, three balls inside it), the 270° wall, the
torus graze and inner equator, the two touches over a quarter donut's
axis, and the rim circle. A probe set built from them would give each
margin its distribution at the three ε. The near-rim rows sit where
`bool_touch_edge_clear` decides closest to zero.

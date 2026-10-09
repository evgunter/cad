---
id: an-edge-lying-in-a-face-at-a-pierce-refuses-ring-meets-ring
kind: issue
title: A prism whose side face holds another's edge at a pierce refuses ResultInvalid RingMeetsRing in six member orders
status: open
opened: 2026-10-08
priority: P2
cost: M
---

## What

Found by PR 4346's sweep (`three-solids-touching-along-one-line-refuse-their-union`).

Row: `crates/topo/tests/three_solids_on_one_line.rs`,
`a_prism_whose_face_holds_the_line_builds_or_refuses_typed`. The fixture
is the plate `[0,3]×[0,2]×[0,1]` (member 0), upright prisms over the
0°–50° (z 0.5–2.0, member 1) and 120°–170° (z 0.47–1.7, member 2) sectors
of radius 0.4 about (1.5, 1), and a third prism (z 0.44–1.81, member 3)
over the triangle from 0.3 at 80° and 260° to 0.4 at 350°. The third's
side face holds the vertical line through (1.5, 1), in no other face's
plane.

Six left folds of `topo::union` refuse
`ResultInvalid { errors: [RingMeetsRing { .. }] }`:

- [2, 1, 3, 0], [3, 1, 2, 0], [1, 3, 2, 0] and [1, 2, 3, 0] at step 3;
- [3, 2, 0, 1] and [2, 3, 0, 1] at step 2.

Every refusing step is the one that adds the plate to a body holding
the third prism and the 120° prism. The same orders refuse identically on main (8fd03fce). Twelve orders build
sound, and three more refuse `VertexReadTwice`
(`a-touching-vertex-beside-a-partner-along-the-face-refuses`).

Unmeasured: which ring meets which, and whether the result's ring
placement or the classification before it is wrong. The tier-3 check
refusing the result is the fail-loud backstop. The row asserts the
refusal by order, and fails when an order builds.


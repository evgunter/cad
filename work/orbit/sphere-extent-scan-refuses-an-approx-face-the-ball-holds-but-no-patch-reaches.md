---
id: sphere-extent-scan-refuses-an-approx-face-the-ball-holds-but-no-patch-reaches
kind: issue
title: The sphere extent scan refuses an Approx face inside the ball a sphere group leaves, though every patch of the group is apart from it
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [boolean-gate-refuses-a-pair-the-census-reads-apart]
---

Found by `orbit/gate-reads-apart` while measuring the operand gate
(`boolean-gate-refuses-a-pair-the-census-reads-apart`).

## What

The two lumps a bar `[−2, 2] × [−0.2, 0.25] × [0.1, 0.4]` leaves cut
through the unit ball, unioned with a box whose top cap wears a
certified `Approx` (`sweep/tests/common/approx.rs`
`box_with_approx_cap`), stood on its side between the lumps with its
cap at `x = 0.5` or `x = 0.83`. Each lump's sphere patch holds neither
`x` pole, so it reaches no further along `x` than its rim at
`x ≈ ±0.88`, and the operand gate now parts the cap from every face of
the lumps (`separating::faces_apart`). The union then refuses
`CurvedBooleanUnsupported { kind: Approx }` at the sphere extent
scan's `Approx` arm (`boolean/ops.rs` `sphere_extent_scan`, through
`ball_may_reach`): the ball `center ± r` the sphere group's faces
leave holds the cap, so the scan cannot rule out an escape through it.
The same pose with a plain NURBS cap builds the three solids
(`sweep::pair_doors_read_apart::the_lumps_of_a_bar_through_a_ball_union_with_a_spline_cap_between_them`).

The ball is the closed group's extent, the right reach for the
re-chart's escape question; the open question is whether a face apart
from every face of the group (the gate's certificate) can bound an
escape at all, or whether that arm could read the group's faces'
reaches as the gate does. The arm's comment ("the pair-scoped operand
gate refuses it by kind before this scan runs") holds only for an
`Approx` face within reach of the other operand.


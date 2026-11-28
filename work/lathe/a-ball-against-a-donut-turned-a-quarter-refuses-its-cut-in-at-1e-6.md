---
id: a-ball-against-a-donut-turned-a-quarter-refuses-its-cut-in-at-1e-6
kind: issue
title: A ball against a donut turned a quarter refuses its oval's cut-in at ε 1e-6 (FallbackExtentUnsupported) where it builds at 1e-9 and 1e-12
status: open
opened: 2026-11-26
priority: P2
cost: M
refs: [close-cuts-is-not-total-on-crossings-path-cut-ins]
---

## What

`mesh::all quadric_pair_sections::a_ball_against_a_donut_turned_a_quarter_builds_in_both_orders`
(PR 7575's row) builds every op in both orders at ε 1e-9 and 1e-12.
At ε 1e-6, `A ∪ B` refuses `FallbackExtentUnsupported { operand: B }`:
"a curved face meets a face of the other solid in a closed loop
interior to both faces, which no edge event and no vertex probe can
see; the loop is certified, and cutting it in is not built". It is red
there on the one-band stack before main's merge (`a8a9d3a9b6`) and on
head after it. It sat in no table until the port's dual review found it.

The pose: the ball of radius 0.5, and the donut (0.6, 0.2) turned
1.5707963 about `x` and moved by `(0.05, 0, 0.3)`. They meet in one oval
inside both faces (the no-crossings path's R-loop). The section pass
asks for the ball's face to be cut in across it (`CutSection::Loop`).
At 1e-6 the cut-in is not made, and the section pass's refusal stands:
typed and loud, with no body shipped wrong.

## Not yet known

Which of the cut-in's readings refuses at 1e-6 is not traced: the
tube's read off the meridian half-plane (`interior_cuts::read_loop_tube`,
`OffHalf` / `NoCrossing`), or a boundary hit placed on the wrong side of
the tube. The oval is a few hundredths across, against a band of 1e-6.

## Pin

The row is pinned red at ε 1e-6 at that refusal
(`test_utils::pinned_red::row`), and runs green at the other ε.

---
id: a-tube-ending-on-a-ball-refuses-section-loop-mixed
kind: issue
title: A tube whose end rim lies on a ball, unioned with it, passes the crossing layer and refuses Join(SectionLoopMixed)
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall]
---


Found by TANG's abutting-rim lane (branch `tang/abutting-rim`), in its
sweep for transverse rim abutments.

## What

A z-axis tube of radius 1 over `z ∈ [1, 3]` (the sweep's `extrude` of a
circle) unioned, undeclared, with a ball of radius `√2` about the origin
(`sweep::test_support::ball_poled_z`). The tube's bottom rim circle lies
on the sphere, and the tube's wall meets the sphere there at 45°. The
tube's bottom disc lies inside the ball.

Since the abutting-rim lane, the rim is an ON event at the crossing
layer (`topo::boolean::reduce::lying_on`): its two ends land on the
ball's seam meridians and split them, and its interior meets the face's
boundary nowhere else. The union then refuses in the join,
`Join(SectionLoopMixed { face })`, in both member orders. A shorter tube
over `z ∈ [1, 1.5]` refuses the same way.

The section loop is the rim circle, lying inside the ball's two faces
between their seam meridians: an in-face section curve made of operand
edges, the class JOIN-1 is about.

## What the taker owes

Measure the fixture on JOIN-1's germ loci. Its closed form is the tube's
volume above the sphere plus the ball's.

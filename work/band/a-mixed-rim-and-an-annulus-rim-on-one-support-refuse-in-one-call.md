---
id: a-mixed-rim-and-an-annulus-rim-on-one-support-refuse-in-one-call
kind: issue
title: A mixed rim and an annulus rim sharing a support refuse in one call
status: open
opened: 2026-11-04
priority: P1
cost: M
refs: [a-rounded-outline-is-a-closed-chain-of-mixed-links-the-blend-refuses]
---

Filed by the band-mixchain lane (2026-11-04), from review lane band-chain-r2's Q6 on PR 6520.

`surgery::shared_support_gate` (`crates/sweep/src/blend/surgery.rs`) refuses `UnsupportedChain` "a mixed rim shares a support with another rim; blend them in SEQUENTIAL calls" when a mixed rim and a ladder or annulus rim of the same request share a face.

Witness: a rounded plate with a round hole.
- The rounded rectangle `[−1, 1] × [−0.6, 0.6]`, corner radius 0.3, 0.5 thick.
- A radius-0.2 hole runs through at the origin.
- Every edge filleted at 0.05 in one call refuses here: the cap hosts the outer mixed rim and the hole's annulus rim.
- The recourse builds. The outer rims first, then the hole's rims in a second call (the review's probe `r2_rounded_plate_with_a_round_hole_every_edge`).

The gate keeps the two walks off one face because only the annulus walk's own seam keys are re-read live (`refresh_annulus_seams`). The mixed walk reads its junction edges live itself, but nothing yet shows that either walk takes the face the other has carved. Lifting the gate needs that, and a row that proves it.

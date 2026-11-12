---
id: a-mixed-rim-and-an-annulus-rim-on-one-support-refuse-in-one-call
kind: issue
title: A mixed rim and an annulus rim sharing a support refuse in one call
status: closed
opened: 2026-11-04
closed: 2026-11-12
pr: 6865
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

Closed by the band-mixgates lane (2026-11-12). The witness's hole rim is a LADDER rim, a ring of two arcs on the cap. Lifting the mixed arm built it in one call at tier 3, bit-equal in volume to the two sequential calls. It also built with the carve order reversed, measured on scratch: no fixture the doors mint carves the ladder first. The arm is dropped.

A hole beside a corner arc then refused, because the meter read the arc's whole trim circle. A mixed link is now read at its own trim between its feet (`co_requested_trim`). Pinned at f64 and Interval with a hole moved into a strip that refuses (`crates/sweep/tests/band_mixed_rim_shared_supports.rs`).

A mixed rim and an actual ANNULUS rim on one support still refuse at the gate's remaining arm. The reason and the measurement are in `an-annulus-rim-around-a-mixed-rim-refuses-in-one-call`.

---
id: a-ball-segment-inside-its-ball-refuses-at-the-sphere-scan
kind: issue
title: A ball segment inside its ball refuses ∩ and the segment's difference at the sphere extent scan
status: open
opened: 2026-10-30
priority: P2
cost: M
---


Found by the sweep of `splitdoor/rods-two-lengths` (the rods-at-two-lengths unit), in `crates/sweep/tests/two_rims_review_probes.rs` `a_ball_segment_inside_its_ball_refuses_at_the_sphere_scan_where_the_segment_decides`, which pins the refusals.

A ball of radius 1 about the origin (B) and the segment of it below `y = 0.5` (A): the segment's sphere face lies on the ball's carrier, and its cap stands inside the ball with its rim on the ball's sphere. `A ∪ B` and `B ∖ A` build at their closed forms. `A ∩ B` and `A ∖ B` refuse `FallbackExtentUnsupported { operand: B, face: FaceKey(1v1), what: "the sphere's section circle runs near the plane face's boundary — whole-circle membership cannot be certified from the enclosures, and no crossing layer saw an event" }`. Mirrored with the operands swapped (`∪` and the segment's difference refuse). Measured at ε = 1e-6, 1e-9 and 1e-12.

- **The door.** `boolean::ops` `sphere_extent_scan`, its plane arm: the ball's sphere crosses the cap's plane in a circle, and `face_boundary_meets` finds the circle near the cap's boundary, because the circle *is* the cap's boundary (the rim). This is not the section pass's R-tan refusal, and the deferral (`OnDeferred`) never sees it: the scan refuses before any pair is held.
- **The shape.** The rods, cones and frusta of the same pose (a cap inside the other solid, the walls on one carrier) build every op since the side certificate (`OnDeferred::sided`). The sphere is the one kind whose cap-against-carrier circle the scan reads first.
- **The answer owed.** `A ∩ B` is A, and `A ∖ B` is nothing. A circle the scan finds lying on the plane face's boundary, where that boundary is an edge of the face's operand that lies on the sphere, is a touch along an edge the reduction classified, not an escape; whether the scan can read it so, or should leave it to the section pass, is the question.


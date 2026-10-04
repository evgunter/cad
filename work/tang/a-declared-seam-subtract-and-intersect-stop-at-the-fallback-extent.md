---
id: a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent
kind: issue
title: The sphere-capped tube declared a Seam builds its union, and its subtract and intersect stop at the fallback extent
status: open
opened: 2026-10-02
---


## What

The sphere-capped tube, walls declared `Seam` and discs `Rest`, builds
its union in both orders
(`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`the_sphere_capped_tube_builds_with_its_walls_declared_a_seam`). Its
`tube ∖ cap`, `cap ∖ tube` and `tube ∩ cap`, under the same
declarations, refuse `FallbackExtentUnsupported`: "the sphere's section
circle runs near the plane face's boundary — whole-circle membership
cannot be certified from the enclosures, and no crossing layer saw an
event". The row pins it.

## Why it is wrong

The answers are closed-form: the tube, the half ball, and empty (the
two share only the disc and the rim). The ops take the no-crossings
fallback, and its containment read of the sphere's section against the
disc lands on the rim itself, which is the disc's boundary. The union
never asks that question, because the declared-REST zip removes the
discs first.

## A declared sphere `Rest` stops at the same question (`reach/rest-mate-intersect-diff`)

A ball of radius 0.5 seated in a hemispherical cup (the profile
`(0, −1) → (1, 0)` arc, `→ (0.5, 0)`, `→ (0, −0.5)` arc, revolved a
full turn about `y`), the cup's inner sphere face × the ball's declared
`Rest`: the union builds at `π/6 + 7π/12 = 3π/4`, and `∩`, `cup ∖ ball` and
`ball ∖ cup`, in both operand orders, refuse `FallbackExtentUnsupported`
with this item's sentence ("the sphere's section circle runs near the
plane face's boundary"), operand the ball. Before
`ops.rs` `Exempt::Rest` answered the sphere pair they refused `SpheresMeet`
(nested margin zero) one arm earlier. The ball's equator lies in the
cup's rim plane, on the boundary of the rim annulus, so the plane arm
meets exactly this item's circle-on-a-boundary read; a ball filling a
spherical cavity has no plane face and builds every op.

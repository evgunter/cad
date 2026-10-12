---
id: a-covered-line-ending-just-off-the-face-keeps-the-door
kind: issue
title: A Seam-covered line whose end touches the partner's carrier just outside the face keeps the door
status: parked
opened: 2026-10-08
priority: P3
cost: M
blocked_on: [intent-stage4-is-built]
---

## What

The sphere-capped tube of
`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`, walls
declared `Seam` and discs `Rest`, with its hemisphere turned about `z`
so the two operands' rim vertices sit `θ·R` apart. At ε 1e-9 it
builds with `θ·R` inside the zero band (the aligned body) and from
about `5e-4` out. In the sliver band the seam cover escalates. Between,
from about `5e-8` to `2e-4` (0.000003° to 0.01°), it refuses
`CurvedPierceUnsupported` in both member orders, on the tube's seam
ruling against one hemisphere face
(`the_sphere_capped_tube_turned_within_a_hair_of_aligned` pins it).

## Where

`reduce::curved_face_arm`, the covered `(Positive, Zero)` rung. The
ruling's top end lies on the hemisphere's rim, just outside the face
being read (the face's own rim vertex is `θ·R` away). So
`vertex_on_curved_face` places it `Elsewhere`. The rung calls
`Placement::declared([Some(Elsewhere), None], false)`, and with no
interior certificate an all-`Elsewhere` pair keeps the door. Further
apart the pair never reaches the arm. The sphere face's reach over its
azimuth window (PR 4123, `boxes::sphere_window`) clears the ruling by
`R·(1 − cos θ)`, and that falls inside the sweep's pad near `θ = 0`.
With the window read as the whole latitude zone, the same door refuses
at every turn. That is what it did before PR 4123, which is why this
fixture refused at 30° and 90° when it was filed.

## Why it is not served here

The truth is clear. The cover holds the ruling in the sphere's closed
outside, and a convex residual along a line is zero there only at the
end. The end is outside the face by `θ·R`, and the ruling runs away
from the face. So a deviation that puts an incidence on the face is
about `θ·R`, which is definite across the whole window.

But only the declaration says so. Undeclared, the same tangent graze
refuses at every turn
(`the_sphere_capped_tube_refuses_undeclared_and_under_every_other_class`).
Passing `interior_clear` from the cover would make a declared lane
serve where main refuses, and the value reading does not decide the
same thing. D10 rules that out. In general the cover's certificate is
also too wide. A covered line that runs toward the face over a
tangent touch flips at about `m²/2R` (`m` the end's distance outside
the face), not `m`, and the placement door reports no `m` to lever a
decide on.

## Direction

Read again once booleans glue on Zero (`intent-stage4-is-built`). The
graze is then the Zero path's, and the row asks whether that path
places a tangent touch at an end that lies outside the face. If it
does, the near turns build. If not, they refuse there typed, with the
`m²/2R` margin named.

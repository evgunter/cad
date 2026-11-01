---
id: a-puck-and-its-rounding-ring-refuse-intersect-and-difference-on-the-tangent-top
kind: issue
title: A puck and its rounding ring refuse ∩ and ∖ where the ring's torus is tangent to the puck's top along its rim
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [rounded-stack-subtract-and-intersect-refuse-fallback-extent]
---


The puck `r = 1`, `z ∈ [0, 0.25]`, and the quarter disc of radius
0.25 revolved round it
(`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`,
`a_puck_and_its_rounding_ring_unite`). The ring's inner wall is the
puck's wall, opposed, and its floor continues the puck's. Its torus
face meets the puck's top only along the top rim, where the torus is
tangent to the top's plane. Both unions build. `A ∩ B`, `B ∩ A`,
`A ∖ B` and `B ∖ A` refuse `FallbackExtentUnsupported` on the ring's
torus face, with the section certificate's tangent sentence ("a curved
face and a face of the other solid have tangent or near-tangent
carriers and no crossing layer saw an event"). Measured on
`splitdoor/sided-followups` (origin/main `60c8f1f0` plus PR 6070's
follow-ups), 2026-11-01. The oracle: `∩` is empty, and each difference
is its minuend (`π/4` for the puck, `π·0.25²/4 · 2π(1 + 1/(3π))` for
the ring).

This pair is not one carrier, so the deferral that answers the aligned
continuations (`ops::OnDeferred`) does not reach it. What the section
pass lacks is a certificate that a torus touching a plane along a
circle on the plane face's boundary meets the face nowhere else.

Split out of `rounded-stack-subtract-and-intersect-refuse-fallback-extent`.
That row listed this pose with the aligned continuations, and those now
build.

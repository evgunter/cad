---
id: a-flipped-one-band-sphere-face-met-only-transversally-finishes
kind: issue
title: A one-band sphere face whose sense bit is flipped finishes when every neighbour meets it transversally: no check sees the flip
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---

Found by the LATHE one-band lane (2026-11-23, PR 7414's review fix pass,
the slow set's base/head diff).

## What

`crates/sweep/tests/verbs_cylsph_opening.rs`,
`a_flipped_chorded_sphere_face_is_refused_as_a_lamina`: the union of
the cylinder `r = 1`, `z ∈ [−2, 2]` and the ball of radius 1.5 about
`(0.6, 0, 0)`. Its one sphere face (outer loop: the ball's seam
meridian walked out and back, pole to pole; one ring: the section
curve, two edges shared with the two cylinder wall faces) finishes at
tier 3 with its sense bit flipped (`flipped_face_sense_for_tests`).
Every other face flipped is refused (`LaminaWedge` at its plane edges,
`LoopRoleInverted` on a cap). On main the ball was two half-bands, and a
flipped half met its twin along their shared meridian, opposed and
osculating: `LaminaWedge`. One band has no such edge.

## Why it matters

A body with a sphere face turned inside out passes tier 3: the face's
loops encode no side (a loop on a sphere bounds a face on each side), so
tier 3 exempts it from the sense cross-check, and at a transversal edge a
flipped face is a different legal wedge, not a lamina. Nothing reads the
flip. Soundness, not capability.

## Wanted

A check that reads a sense-exempt curved face's side from somewhere
other than its loops or a co-surface twin: the shell's orientation
(check 7's rays, or the solid's volume sign with that face's flux), or
the transversal wedge's convexity against the material each neighbour
states. The row is pinned to its stop while this is open.

---
id: shell-of-a-pole-touching-sphere-refuses-mapped-source
kind: issue
title: shell of a POLE-TOUCHING sphere refuses Certification(ResidualExceeded MappedSource) where the two-arc sphere shells
status: review
opened: 2026-09-09
priority: P0
cost: H
pr: 4111
branch: shell/pole-ball
---


Found while surveying demo coverage for a shell cell. SHELL-9's row
shells a sphere authored as **two cocircular arcs**; the pole-touching
one-arc ball — the shape `revolve_ball` mints and the shape a demo
would reach for — does not shell, and the refusal is not the one
SHELL-9 fixed.

## Measured

`Tol::witness()`, main at this branch's base.

| body | meridian | `topo::shell(&b, 0.1, tol)` |
|---|---|---|
| unit ball | ONE semicircular arc `(0,-1) → (1,0) → (0,1)`, pole-touching, no bore | `Face { face: FaceKey(1v1), error: Op { edge: Some(EdgeKey(2v1)), error: Certification { error: ResidualExceeded { check: MappedSource, sample: 1 } } } }` |
| bored sphere | two arcs on one circle plus an on-axis bore wall (annular, so every latitude rim is one closed edge) | **Ok**, volume `1.3808925058780275` |

Both are full revolves at the same ε; the difference is the meridian.

## Why it is a row and not a frontier note

SHELL-9 (#2223) landed the closing pcurve mint precisely so that the
sphere row flips to its closed form, and its item states the case it
measured: *"Measured on a sphere authored as two cocircular arcs …
`topo::mint_pcurves` on the assembled body makes it tier-3 valid at
`4/3·π(r−t)³`."* The pole-touching ball is the sibling shape — same
surface, one chart, two half-bands meeting at the seam meridian AND at
two poles — and it stops at a different door: a `MappedSource`
residual on the graft re-certification, not the `LoopDiscontinuity`
the closing mint retired.

So one of two things is true, and nothing in tree says which:

1. the pole-touching class is genuinely outside the offset lane's
   reach (a pole is a chart singularity, and the offset of a face
   whose boundary reaches it may have no `MappedSource` image), in
   which case the refusal wants to be typed as that rather than as a
   certification residual, and `shell`'s docs should name the class; or
2. it is the same repair SHELL-9 made, one graft further along.

## What the taker owes

One row either way: a fixture that shells the pole-touching ball and
either asserts the closed form `4/3·π(r³−(r−t)³)` or pins the typed
refusal with the class named in prose. Today the sphere reads as
"shells" from SHELL-9's row alone, and the first consumer to author a
plain ball finds otherwise.

Refs SHELL-9 (#2223).

## Premise drifted — measure first (SHELL orchestrator, 2026-10-06)

f28c201d ("full revolve builds one wall per run of cocircular arcs")
changed the fixtures: the revolved "two-arc sphere" is now one wall in
two π-bands, so the contrast this row draws is gone, and
`shell7_seam_corner::a_sphere_from_an_arc_run_hollows_to_its_closed_form`
proves only the direct door (`offset_charts_together`), not
`topo::shell`. The answering probes exist but only print
(`sf2b_r1_probes`, `shell9_r1_probes`). The first lane re-measures
`topo::shell` on a pole-touching ball: if it hollows, assert
`4/3·π(r³−(r−t)³)` and close; if it refuses, the measured refusal is
this row's evidence and the H fix proceeds.

## Measured on 4cfb4b20 (2026-10-06)

The premise drifted as suspected: `topo::shell` hollows the one-arc
pole-touching ball at every pair measured, at ε = 1e-6, 1e-9 and
1e-12, to `4/3·π(r³ − (r−t)³)` within `1e-12·r³ + volume_pad`,
tier-3 valid and watertight — the item's own `(1, 0.1)` row included
(`1.1351621454971117`). The arc-run meridians (one and three seams,
seams near both poles) hollow the same. The `Shell` recipe node over
`die_pips`' revolved ball does too. The rows:
`crates/sweep/tests/pole_ball_shells.rs` and
`lib_g17_shell_node::a_sealed_shell_over_a_revolved_ball_is_the_difference_of_two_balls`.

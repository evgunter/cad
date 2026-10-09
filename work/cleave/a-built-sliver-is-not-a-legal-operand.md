---
id: a-built-sliver-is-not-a-legal-operand
kind: issue
title: A boolean result a sliver thick passes the result gate though its at-infinity side cannot be decided, so it is not a legal operand
status: open
opened: 2026-10-09
priority: P1
cost: M
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census, 4335]
---


Found by PR 4344's dual review (r2 m5), measured on its fix pass
(`join/sphere-pair-whole-circle`, release, default ε).

## Measured

The unit ball at the origin against `ball(50)` on the `z` axis, both
`y`-poled, `2·10⁻⁶` from tangency:

| pose | op | result |
|---|---|---|
| `d = 51 − 2·10⁻⁶` (inside external tangency) | a ∩ b, b ∩ a | a lens about 1.2·10⁻¹¹ m³ and 2·10⁻⁶ thick: tier 2, tier 3′ and the certificate pass, the volume is right |
| `d = 49 + 2·10⁻⁶` (outside internal tangency) | a ∖ b | the same shape of sliver, the same readings |

Uniting either sliver with a brick far away refuses
`ShellWitnessExhausted { in_band: 26, first_in_band: Escalated {
bool_point_in_solid_infinity_enclosure } }`: the at-infinity probe's
enclosure of the volume is about ±1·10⁻⁵ around a value of 1·10⁻¹¹, so
no ray decides which side is outside. The result gate computes the
volume (`VolumeUncomputable` would refuse) but never asks whether its
sign is decided, so it ships a body the next boolean cannot read.
`crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
`a_large_ball_near_external_tangency_is_pinned`, pins the ∩ as
`operand=false`.

Main refused this pose `SpheresMeet` before PR 4344; the sliver
appears through the cut-in and the re-chart alike, so it is the gate's,
not the door's.

## What a fix owes

The result gate refuses a body whose at-infinity reading it cannot
decide (the reading `point_in_solid` and the containment fallback spend,
`solid_contain::at_infinity_side`), as `Escalated` with that reading's
margin. That is the shape [ev] PR 4335 argues for in-band results
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`).
The rows are this pose's ∩ and a ∖ b, and the near-tangent batteries'
slivers.

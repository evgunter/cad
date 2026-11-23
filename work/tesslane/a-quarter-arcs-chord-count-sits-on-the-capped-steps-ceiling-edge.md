---
id: a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge
kind: issue
title: a quarter arc's own chord count sits on the capped step's ceiling edge: 5 ulps of stored span make 3 chords where 2 serve
status: open
opened: 2026-11-23
priority: P3
cost: C
---


Found by `a-filleted-one-segment-hole-rims-band-exceeds-its-mesh-certificate` (stage 1, measured on `profile/circle-lowers-to-one-segment`).

## What

`chords::compute_chords` gives a circle edge its own count `ceil_count(span, sagitta_step(target, ρ))`. `sizing::sagitta_step` caps at `MAX_ANGULAR_STEP = π/4`, and the cap binds wherever `δ/ρ ≳ 0.17` (the sagitta step passes π/4 there) — the usual case for a small fillet at a display δ. A quarter arc's span is then exactly two steps, and the count is decided by the last bits of the stored span:

| edge | stored span | span / (π/4) | count |
|---|---|---|---|
| the one-segment hole's fillet slit (PROFILE's tree, r = 0.005, δ = 1e-3) | `1.5707963267948977` (π/2 + 5 ulp) | `2.0000000000000013` | 3 |
| the two-arc twin's slit, same r and δ | `1.5707963267948966` (π/2) | `2` | 2 |

Two builds of one geometry chord differently, and the face beside the edge does not see it: the curved lane lays `nv = ⌈(v1 − v0)/π/4⌉ = 2` rows from its box (exactly π/2), so the third chord's points sit between rows. Half and full turns sit on the same edge (multiples of π/4). The review's sweep found no trigger on a meridian arc of a turn other than a quarter.

## Why it matters, and what it no longer costs

Before the curved lane's CDT ran in the lattice metric this was a refusal (`CertificateExceeded`, 7.7 δ on PROFILE's plate at r = 0.005, δ = 1e-3; up to 83 δ over this unit's 498-band sweep on main); now it costs one chord and the triangles beside it, which the certificate holds (`the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound`). What remains is determinism of the count across builds of one geometry, and a mesh one chord finer than the theorem asks.

## What closing it takes

The count should not depend on rounding below the span's own accuracy. Options to weigh, none chosen: read the count from the edge's turn as the chart reads it (the face box's span, which is exact here), or state the cap so an exact multiple of it is not knife-edged. No ε may enter (`[tolerance.chordal]`).

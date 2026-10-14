---
id: a-tilted-plane-cut-through-a-tube-measures-1e-7-off-the-closed-form
kind: issue
title: A plane tilted 8° off the tube's cross-section cuts a revolved tube with a volume 1.2e-7 off the closed form
status: open
opened: 2026-10-04
priority: P3
cost: E
---


## What

Seen in the PR 4004 fix pass (`join/fan-end-one-spelling`), identical
on main and on the PR's head.

`fan_end_review_rim_battery` (`crates/sweep/tests/fan_end_review_probes.rs`
on branch `join/fan-end-one-spelling-review`), normal
`m0 = (0.144, 0.000, 0.990)` at the battery's default `FANREV_N=96`,
top and bottom rim: the cube's plane through the rim vertex cuts the
revolved tube (annulus `0.5 ≤ ρ ≤ 1`, `y ∈ [0, 1]`, revolved about
`y`) along a curve. All six ops and orders build and pass tiers 2
and 3′ and the certificate, but `mass_properties` reads, e.g.,
∩ `1.033119142` against the clamp oracle's `1.033119023`: `1.2e-7`
off, just over `differential::outcome`'s `1e-7` gate, so the lines
read `OK BAD` (12 lines).

The oracle is closed form for a disc (`pos_part_disc`) and was checked
against a brute slice sum on 4 000 slices in the same file. Whether
the drift is the oracle's or `mass_properties`' on a curved cut face
is not settled.

## The shape to give

Measure the same pose with a third reading (a finer slice sum, or
`mass_properties` at a tighter tolerance); then either fix the side
that is off or record why `1.2e-7` is inside that side's stated error.

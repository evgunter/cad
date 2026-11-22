---
id: rim-wedge-reduces-angles-by-a-floor-of-the-value
kind: issue
title: rim_wedge reduces its rim angles into [0, τ) by τ·floor(θ/τ), a whole-period integer read off a value (C4)
status: open
opened: 2026-11-10
priority: P3
---

Found closing `a-turn-is-still-spelled-outside-its-carriers-period`. That row takes rim_wedge's τ from the rim's period, but leaves its reductions as they are.

## What

`crates/topo/src/boolean/rim_wedge.rs` brings rim angles into `[0, τ)` as `x − τ·floor(x/τ)` in four places:
- `locus_param`'s rim arm;
- `reach`'s ride start and end cut;
- `cut_points`' arc angle.

That reads a whole-period integer off a value, which C4 (`geom_core::period::Period`'s docs) says no site does. The period's own door is `Period::at_or_above(x, 0)`.

## Why it was not folded in

`at_or_above` decides through `Period::near` and the window's end, so it moves bits. For `x` in `(π, τ)`, `near` takes `x − τ`, and the lift adds `τ` back, which is not `x` bit for bit. And a hair under zero folds onto `0` rather than onto a hair under `τ`. `locus_param`'s doc already relies on readers being invariant to that last case. The turn-spellings unit was held to moving no bits. So this row's change is a measured one: swap the reductions and run the rim rows at all three ε.

## At `Interval`, the floor can straddle (PR 6788's review, finding 3)

Since PR 6788, rim_wedge's τ is `geom::conic_period`'s, which at `Interval` is τ's enclosure rather than the point `from_f64(TAU)`. Then `x − τ·floor(x/τ)` is a floor of an enclosure. Where `x/τ` straddles an integer, `floor` reads either neighbour, and the reduction's enclosure is not that of `x`'s copy in `[0, τ)`. `Period::at_or_above` decides the window's end through the band, so it takes the straddle as a tie. That is a second reason to swap, beside C4.

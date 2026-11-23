---
id: a-blend-whose-spine-is-inside-its-ball-could-mint-a-spindle-sheet
kind: issue
title: blend: SpineIrregular refuses 0 < s ≤ r, where the rolling ball sweeps a spindle or horn sheet that Surface::Spindle now represents
status: open
opened: 2026-11-03
priority: P3
cost: M
---

(SPIRICARM, stage 1 of `horn-and-spindle-tori-have-no-representation`, the consumer rows its ruling names.)

## What

`sweep::blend` refuses `BlendError::SpineIrregular` (`crates/sweep/src/blend/mod.rs`) for a spine radius `0 < s ≤ r` against the blend radius `r`: the rolling ball's envelope there is a spindle or horn torus, which had no representation. One sheet of it is now `Surface::Spindle` (signed `R`, `−r < R ≤ r`), so the refusal is no longer "no representation" but "no lane". `sweep::blend::reach`'s `surface_distance` and its sheet functions refuse a spindle operand (`None`) for the same reason: `|(ρ − R, z)| − r` is no distance off the lemon near its apexes.

## Done when

The blend lane mints the sheet the ball sweeps where its face clears the apexes, or the row records why it should not.


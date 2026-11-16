---
id: a-sheets-tube-residual-cancels-beside-a-horns-cusp
kind: issue
title: implicit.rs's tube residual (d^2 + h^2 - r^2)/2r, shared by the torus and the sheet, cancels beside a horn's cusp
status: open
opened: 2026-11-11
priority: P4
cost: E
---


(SPIRICARM, found by the PR 6806 sweep, `a-horns-cusp-revolved-part-way-refuses-at-its-cap-meridians-certificate`.)

## What

`geom-brep` `implicit.rs` `implicit_residual`'s torus/sheet arm (`Surface::Torus | Surface::Spindle`) returns `(d² + h² − r²)/(2r)` with `d = ρ − R`. On a horn (`R = r`), beside the cusp `d ≈ −r` and `h` is small, so `d² − r²` cancels. The true residual there is about `−ρ` (the sheet runs at `ρ ≈ z²/2r`), but the `f64` form reads it with an absolute error near `ε·r²`, and an `Interval` enclosure as wide. `ρ(ρ − 2R) + h² + (R − r)(R + r)` over `2r` is the same value and keeps its digits.

`implicit_residual` has many readers (topo `held`, the boolean section and contact paths, `bound_side`, `offset_derive`). I could not show that none of them reads a sheet within `√ε·r` of a horn's cusp, so this row is unverified either way. PR 6806 wires none of them to a sheet; limb 2 reads `Meters`, not this residual.

## Done when

Either the arm is computed in the cancellation-free form (it is shared with the ring torus, where `R − r > 0` and nothing cancels near the axis), or a row shows that no reader reaches a horn's cusp with it and the site says so.

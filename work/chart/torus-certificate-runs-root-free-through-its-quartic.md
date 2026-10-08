---
id: torus-certificate-runs-root-free-through-its-quartic
kind: unit
title: The torus's rung-3 certificate runs root-free through its quartic: exclusion on F's sign, limb 2 by sup|F∘C|/(2r·inf A∘C), the tube gradient ∇F/(4rA)
status: open
priority: P2
cost: H
refs: [torus-meters-blocker-is-the-arithmetic-or-c9s-root-rule]
opened: 2026-09-29
---

Filed by CURVED from a design fork (2026-09-30; `docs/DESIGN-FORK-LOG.md`
row 30). Both designers converged on this shape; it waits on no ruling.

## What

The torus arms of `crates/geom-brep/src/ssi/enclose.rs`
(`implicit_enclosure`, `implicit_gradient_enclosure`) and of
`ssi/certify.rs`'s `composite_form` refuse `Surface::Torus`, on the
premise that its meters residual needs a square root certification
arithmetic does not take. The root is in the spelling
`m = ((ρ − R)² + h² − r²)/2r`, `ρ = |w|`, not in the surface. With the
quartic `F = A² − 4R²ρ²`, `A = |q|² + R² − r²` (`q` the point minus the
centre, `w` its component off the axis):
`m = F / (2r·(A + 2Rρ))`, and `A ≥ R² − r² > 0` everywhere on a ring
torus. So:

- **exclusion** (`exhaust.rs:sweep_r3`, the only caller of
  `implicit_enclosure`) needs only the sign of `F`;
- **the tube gradient** is `∇F/(4rA) = (q − (2R²/A)·w)/r`, root-free and
  equal to the unit normal on the surface; refuse where `A`'s enclosure
  touches zero (only a spindle or horn torus's axis points);
- **limb 2** bounds `sup|m| ≤ sup|F∘C| / (2r·inf A∘C)`, both factors
  hulls of polynomial composites (`A∘C` is already an intermediate of
  `geom_core::spline::compose`'s torus arm); `composite_form`'s constant
  `to_meters` becomes a carrier-dependent conversion, with the global
  `1/(2r(R² − r²))` only as a fallback (it can be ~`4R/(R−r)` loose).

Limb 1's `implicit_residual` keeps its evaluation-side `sqrt`; limbs 1
and 2 then bound the same `m`.

## Measure first

Enclosing `F` as `A² − 4R²ρ²` treats two dependent terms independently:
the exclusion band is ≈ `2R/r` fatter than the factored meters form,
and failing cells scale with its square. Measure exhaustion cell counts
on a thin torus (`R/r ≥ 10`) against `SSI_MAX_CELLS` with both spellings
before the arms retire; a centred or mean-value form of `F/(4rA)` is the
root-free mitigation. If Ev admits `√` into C9 (the open `[ev]` PR on
`torus-meters-blocker-…`), the factored form with `ρ = √Σw²` becomes
available and is the simplest tight spelling for the box enclosures.

## Then

Retire the torus arms one route pair at a time under C12.1, cheapest
first (sphere×torus, or the tilted plane×torus residue), widening the
lane gate (`ssi.rs`'s `WrongLane`) per pair and revisiting
`sweep_r3`'s refusal sentence. The cone is the same shape plus its
`tan α` exactness question — a separate unit. Stale "cannot"/"impossible"
wording to correct with it: `implicit_enclosure`'s "cannot bound tightly
enough to be useful", `composite_form`'s and `certify.rs`'s module doc's
"certified root, which certification arithmetic does not take", and
`intersect.rs`'s C5 notes for the cone and torus arms.

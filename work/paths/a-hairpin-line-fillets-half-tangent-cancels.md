---
id: a-hairpin-line-fillets-half-tangent-cancels
kind: issue
title: paths: the line x line fillet's half-angle tangent divides by |v1||v2| + v1.v2, which cancels at a hairpin corner and moves the fillet's effective radius
status: dispatched
branch: paths/hairpin-fillet-half-tan
opened: 2026-10-19
priority: P2
---


Found by the sweep of `tangent-arc-near-a-full-turn-loses-its-end-vertex`.
Not measured yet: the first step is a probe.

**The shape.** `sugar::line_line_fillet_trims` reads
`half_tan = (v₁ × v₂) / (|v₁||v₂| + v₁·v₂)`, tan(φ/2) of the corner's
turn φ. At a hairpin, φ = π − δ, the denominator's true value is
`|v₁||v₂|·δ²/2` and its rounding about `|v₁||v₂|·2⁻⁵²`, so `half_tan`
carries a relative error of about `2⁻⁵¹/δ²`. The fillet's stored
sweep (`4·atan` of the bulge read from `half_tan`), its setback and its
tangent points all read it. Since #5403 it is the first arm of the
home every construction reads (`sugar::split_half_tan_near`), and the
home itself (`sugar::split_half_tan`, the select on `v₁·v₂`) is the
cancellation-free reading of tan(φ/2) from the same split. A fillet
takes the first arm alone because a select is one atom to the symbolic
tier's plain walk; the fix weighs what the select costs the pad's and
the bracket's symbolic decisions.

**Measured (#5403's re-review generator, `rev2_hairpin_dump`).** At
δ = 3e-3, legs of 1e3 m, r = 1 m, 1e3 m off the origin, the stored
fillet arc misses its own end vertex by 160 units of 2⁻⁵² times the
coordinates, about 7e-11 m: under ε, and the one live end-vertex miss
the re-review's generator found once #5403 lands.

**Why it may matter, by arithmetic.** Both setbacks share `half_tan`,
so tangency survives, and what moves is the fillet's effective radius:
`r·2⁻⁵¹/δ²`. The parallel-carrier gate refuses δ under about `Kε/r`,
and the setback `2r/δ` has to fit the leg `L`. The worst case is at
`r ≈ √(L·Kε/2)`, giving a radius error of about `4.4·r³` m at the
default ε. That is about ε for `L = 100` m, and about 1e-6 m (1000 ε)
for `L = 1e4` m. The probe
`At(0,0) Toward(1,0) Fillet{r} At((0, L·tan δ)) Toward(−cos δ, sin δ) …`
reaches it. Measure the stored radius against `r` before deciding.
The fix moves the setback's bits at every corner turning past a quarter
turn, and the closed-form oracle in `tests/path_differential.rs`
(`expected_trims`) moves with them.

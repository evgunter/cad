---
id: a-hairpin-line-fillets-half-tangent-cancels
kind: issue
title: paths: the line x line fillet's half-angle tangent divides by |v1||v2| + v1.v2, which cancels at a hairpin corner and moves the fillet's effective radius
status: open
opened: 2026-10-19
priority: P2
---


Found by the sweep of `tangent-arc-near-a-full-turn-loses-its-end-vertex`.
Not measured yet: the first step is a probe.

**The shape.** `sugar::line_line_fillet_trims` reads
`half_tan = (v₁ × v₂) / (|v₁||v₂| + v₁·v₂)`, tan(φ/2) of the corner's
turn φ. At a hairpin, φ = π − δ, the denominator's true value is
`|v₁||v₂|·δ²/2` and its rounding about `|v₁||v₂|·2⁻⁵²`, so `half_tan`
carries a relative error of about `2⁻⁵¹/δ²`. The cancellation-free
choice is the one the tangent arc and `Via` took: `select_le_zero` on
`−v₁·v₂` between this spelling and `(|v₁||v₂| − v₁·v₂)/(v₁ × v₂)`.

**Why it may matter, by arithmetic.** Both setbacks share `half_tan`,
so tangency survives, and what moves is the fillet's effective radius:
`r·2⁻⁵¹/δ²`. The parallel-carrier gate refuses δ under about `Kε/r`,
and the setback `2r/δ` has to fit the leg `L`. The worst case is at
`r ≈ √(L·Kε/2)`, giving a radius error of about `4.4·r³` m at the
default ε. That is about ε for `L = 100` m, and about 1e-6 m (1000 ε)
for `L = 1e4` m. The probe
`At(0,0) Toward(1,0) Fillet{r} At((0, L·tan δ)) Toward(−cos δ, sin δ) …`
reaches it. Measure the stored radius against `r` before deciding.
The fix moves the bits of every line × line fillet at a corner turning past a
quarter turn, and the closed-form
oracles in `tests/path_differential.rs` (`expected_trims`) with them.

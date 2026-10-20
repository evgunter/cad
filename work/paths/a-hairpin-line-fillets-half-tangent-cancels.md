---
id: a-hairpin-line-fillets-half-tangent-cancels
kind: issue
title: paths: the line x line fillet's half-angle tangent divides by |v1||v2| + v1.v2, which cancels at a hairpin corner and moves the fillet's effective radius
status: closed
branch: paths/hairpin-fillet-half-tan
opened: 2026-10-19
closed: 2026-10-20
pr: 5441
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

## Outcome

Measured past ε, and fixed: `sugar::line_line_fillet_trims` reads
tan(φ/2) by the home's select (`sugar::split_half_tan`), not its first
arm.

- **What a user reached.** The probe, with the arrival anchor either
  near the origin (the row's shape, whose corner gate is levered by
  `L·δ`) or a leg's length past it (levered by `L`, so δ reaches
  `Kε/L`), over legs of 100 m and 1e4 m, δ from 0.3 rad to 1e-13 and
  radii from 1e-8 m to 1e4 m. mpmath at 60 digits from the authored f64
  literals. The stored radius is the authored `r` by construction; what
  moves is the setback, so the measure is the fillet the stored legs
  describe (setback over tan(φ/2) of the stored legs), the stored arc's
  end-vertex miss and its rim's distance off each leg's line.
- **On main, at ε = 1e-9:** 83 built hairpins past ε, 52 of them
  validated. The row's own probe peaks at 4.2e-7 m off the radius and
  8.4e-7 m off the end vertex (L = 1e4, δ = 1e-6, r = 3.2e-3), as the
  row's arithmetic said. The far anchor is worse: at L = 1e4, δ = 1e-8,
  r = 1e-4 the quotient read a setback of 6.7e3 m where the reals give
  2e4 m, on a 1e4 m leg, so a radius that does not fit was rounded
  elsewhere: 1.3e-4 m off the end vertex, 6.7e-5 m off tangency, and
  validated. ε = 1e-6: the same worst case, 7 past ε. ε = 1e-12: 254
  past. `Interval` escalated every one of them (the denominator
  straddles zero).
- **With the select:** the worst miss over the grid is 2.7e-12 m and
  the worst tangency and radius errors are 2.8e-13 and 2.1e-13 m, all
  at the 1e4 m scene's f64 resolution, at every ε row; the radius that
  does not fit refuses on the anchor fit with setback 2e4 m. `Interval`
  builds every case at 1e-9 and 1e-6 with the landing holding its
  vertex; at 1e-12 the 1e4 m scenes refuse as finer than the scene, as
  on main.
- **The symbolic tier, against main.** No decision is lost. The pad's
  `frozen` rises by two (the select is an atom where the corner's
  `v₁·v₂` is not a constant): `m10_9_pins_interval` (1389, 1102, 4299)
  → (1389, 1102, 4301), `sym11` past the ceiling [1413, 1114, 4310] →
  [1413, 1114, 4312]. The bracket and the link do not move.
  `sym_9`'s pad-by-angle leaf gains decisions: 1262/1230 → 1264/1228
  with the ladder (retried 38 → 39), 1224/1268 → 1225/1267 without. No
  form that kept the first arm was needed, so none was weighed.
- Rows: `crates/profile/tests/hairpin_fillet_radius.rs` (the grid as a
  counterexample search, two pinned hairpins against the reals at f64
  and `Interval`, and the refusal). `path_differential`'s
  `expected_trims` oracle reads the same select.

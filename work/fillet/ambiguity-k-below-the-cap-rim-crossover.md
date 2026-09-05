---
id: ambiguity-k-below-the-cap-rim-crossover
kind: issue
title: Tol accepts K below the cap-rim crossover K* = 1.272, where two doors behave differently and one refuses
status: open
opened: 2026-09-05
---

## Finding (FILLET-H6's lane, PR 1891 — recorded, deliberately not fixed there)

`Tol` accepts **any** `K > 1` from `CAD_AMBIGUITY_K`
(`crates/geom-core/src/tolerance.rs`, the predicate is `v > 1.0`), and no CI
row varies it: the whole matrix runs at the default K = 10. FILLET-H6 measured
what happens below `K* ≈ 1.272` and the answer is that two of `extrude`'s
doors stop agreeing about the same body.

**The crossover.** `extrude`'s two direction gates admit an extrusion vector
whose in-plane component is at most ε against a normal component of at least
`K·ε`, so an admitted `w` parts from the sketch normal by up to `1/K` and the
cap–wall angle obeys `sin θ ≥ K/√(K² + 1)` (the wall normal is `chord × w`).
The `Smooth` outcome needs `sin θ · arm ≤ ε` against an arm the profile door
already put at `arm ≥ K·ε`, i.e. `sin θ ≤ 1/K`. The two close exactly when
`K⁴ > K² + 1` — `K* = √((1 + √5)/2) ≈ 1.272`. Above it no admitted extrusion
can produce a smooth cap rim; below it, ordinary ones do.

**The two doors, measured end to end at K = 1.1** (`fillet_h6_cap_rim`'s
re-exec row, and `review_fillet_h6_r1_probes` / `review_fillet_h6_r2_probes`):

- the **direction gates** admit `Vector(ε, 0, K·ε)` — both are satisfied at
  their own thresholds, neither is in band;
- the **profile door** admits `rect(2, 1.002·K·ε)` — the short chord is
  definite by exactly the band the rim's arm gate reads;
- the **rim upgrade** then classifies all four short cap rims `Smooth`. Before
  this PR it minted chart images and `extrude` returned `Ok`, and
  `validate_geometric` refused that body with four
  `SliverDihedral { material_wedge_side }`. It now refuses at the door
  (`ExtrudeError::SmoothCapRim`), which is the honest local fix but does not
  answer the question below.

**The question.** Should `Tol` carry a K floor at all — and if so, is it this
crossover, or something a kernel-wide argument sets? That is kernel policy, not
one verb's, and it wants Ev: a floor changes what every predicate in the
workspace admits, and the crossover above is only the first place a
below-`K*` run was observed to split two doors' verdicts. Related, and
deliberately separate: nothing in CI exercises a non-default K, so a floor (or
its absence) is currently unmeasured everywhere except here.

Cited: `crates/sweep/src/extrude.rs` (`upgrade_rim`'s arm and
`ExtrudeError::SmoothCapRim`), `crates/geom-core/src/tolerance.rs`,
`crates/sweep/tests/fillet_h6_cap_rim.rs`,
`crates/sweep/tests/review_fillet_h6_r2_probes.rs`.

## Home

`work/fillet/` — found on FILLET-H6's lane. The decision is Ev's (kernel
tolerance policy), so it is an issue awaiting a design call, not a unit.


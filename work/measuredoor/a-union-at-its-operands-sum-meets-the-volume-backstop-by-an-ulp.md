---
id: a-union-at-its-operands-sum-meets-the-volume-backstop-by-an-ulp
kind: issue
title: A union whose true volume is its operands' sum refuses ResultVolumeImplausible at 1e-6, measured an ulp over vol(A) + vol(B)
status: open
opened: 2026-10-28
priority: P2
cost: M
---

Found by SPLITDOOR's crescent short-arc lane (branch
`splitdoor/crescent-short-arcs`). Pinned by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`short_arcs_at_a_crescents_tip` (its `Backstop` cells).

## The pose

The boss `r = 0.3`, `z ∈ [0, 1]`, over the plate `[−1, 1]² × [−0.5, 0]`
holed through by `r = 0.1` about `(0.2, 0)`, tangent inside at
`(0.3, 0)`, each rim cut at azimuth `±1e-4` (boss) and `±3e-4` (hole),
with or without a vertex at the tip, and at π. The operands meet in no
volume, so `vol(A ∪ B) = vol(A) + vol(B)` exactly.

At ε 1e-6, `A ∪ B` builds `SOUND` in one shell at the closed form;
`B ∪ A` refuses at the backstop (`boolean/ops.rs` `volume_backstop`,
the union arm's `vol(A ∪ B) ≤ vol(A) + vol(B)`):

    ResultVolumeImplausible { which: "vol(A ∪ B) ≤ vol(A) + vol(B)",
      got: "2.267035375555172", bound: "2.2670353755551624", .. }

The excess is `9.5e-15` m³, a few ulps of the sum. The bound's sign arm
decides on the exact band, so at `f64` any nonzero margin reads
certain (`volume_backstop`'s docs). Unmeasured: whether the result's measurement and the
operands' enclosures were refined at all before the sign was taken
(`bounded`, `bound_holds`), and why only `B ∪ A` reads over.

Neighbour: `main-red-reach-volume-backstop-rows-after-the-exact-sliver-bracket`
(`a_result_equal_in_truth_but_larger_at_the_midpoint_passes`) is the
same question, a result equal in truth to its bound, read larger.

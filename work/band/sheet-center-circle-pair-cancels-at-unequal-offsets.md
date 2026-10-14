---
id: sheet-center-circle-pair-cancels-at-unequal-offsets
kind: issue
title: Blend sheet_center places a round x round blend centre by differences of squares, off the smaller offset circle at offsets orders of magnitude apart and poison past a ratio of about 1e8
status: open
opened: 2026-10-13
priority: P2
cost: E
---

Found by a review of #5032 (`arc-arc-secant-arm-misses-crossings-at-unequal-radii`),
whose sweep had searched profile, geom-core, geom-brep and topo only.

## The defect

`crates/sweep/src/blend/arms.rs` `sheet_center`, the `Round × Round`
arm, places the blend centre where the two offset circles (radii
`off_a = r_a ∓ radius`, `off_b = r_b ∓ radius`, centres `dist` apart)
cross: `x = (dist² + off_a² − off_b²)/2·dist` from `c_a` and
`y = √(off_a² − x²)` (spelled `mu·√((off_a² − x²)/mu²)`). Both are
differences of squares. Where `off_a ≫ off_b`, `x` carries about
`u·off_a²/dist` and `off_a² − x²`, at most `off_b²`, carries about
`u·off_a²`, so the centre sits off the smaller offset circle by about
`u·off_a²/off_b`. Past a ratio of about 10⁸ the radicand can round
negative and the centre is NaN.

## Measured

The arm's formula in IEEE `f64`, against an 80-digit check of the
point's distance from each offset circle, 300 secant pairs per row,
`off_a` from 0.1 to 100 and `dist` drawn across the secant range, in
ulps of the largest of the radii and `dist`:

| ratio `off_a/off_b` | worst off-carrier |
|---|---|
| 1 | 1.9 |
| 10² | 182 |
| 10⁴ | 1.7e4 |
| 10⁶ | 1.7e6 |
| 10⁸, 10¹⁰ | radicand negative (NaN centre) on some pairs |

Not driven through a blend: whether a blend door reaches an offset
circle orders of magnitude smaller than its partner (a fillet radius
within a hair of a small round's radius on its concave side, say) is
the owner's to say. The banding follows the sibling rows
(`work/hone/circle-crossing-closed-forms-cancel-at-unequal-radii.md`,
`work/roots/sphere-sphere-section-radius-cancels-at-unequal-radii.md`).

## Where to look

`crates/profile/src/seg.rs` `Circles::crossings` takes both from the
clearances: `r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`, measured
there at under 3 ulps of scale at any ratio. A shared home for that
algebra is `work/issues/circle-pair-radical-chord-has-no-shared-home.md`;
without it, fixing this site mints another copy.

2026-10-14 (PATHS, `paths/circle-pair-shared-home`): the shared home has landed as `geom_core::Circles` (`crates/geom-core/src/arc/pair.rs`): built from `d` and the two radii, it hands out the identity, external and internal margins and the cancellation-free `radical` foot and `half_chord` from one `r₁ + r₂` and `r₁ − r₂`; `Circles2` places them in the plane. profile `seg.rs` and topo `validate.rs` arm 5 read it. Fix this site by reading it, deciding its margins from the same value, rather than minting a copy.

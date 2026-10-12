---
id: the-whole-body-pad-sum-rounds-to-nearest-and-the-enclosure-doc-scales-the-wrong-term
kind: issue
title: fold_runs sums flux_pad/area_pad round-to-nearest (not outward), and MassProperties::enclosure's error argument scales with |volume| where the fold's error scales with the sum of |face terms|
status: open
opened: 2026-10-12
priority: P3
cost: E
refs: [4830, the-whole-body-property-fold-depends-on-face-order]
---

## What

Both designers on the `[ev]` PR 4830 (whole-body fold determinism) found
the first; one found the second.

- **The pad sum is not outward.** `topo::props`'s `fold_runs` sums the
  per-face `flux_pad` and `area_pad` with plain `+=`, which rounds to
  nearest. So the reported half-width can understate the summed
  half-widths by an ulp. `step-import`'s `nurbs_import.rs` arc-loft row
  calls that sum "outward", and it is not. Whether the per-face pads
  carry enough slack to absorb this is unchecked.
- **The enclosure argument scales the wrong term.**
  `MassProperties::enclosure`'s doc bounds the reconstruction error at
  about `|volume|·2⁻⁵²`. The fold's own rounding error scales with
  Σ|face flux|, not with |volume|. For a body far from the origin, the
  designer estimates about 10⁷ m at ε = 1e-9 and K = 10. There the fold
  error alone reaches check 7's K·ε threshold. The conclusion probably
  still holds, since the sign decision goes through a recentred interval
  rederive, but the stated reason is wrong.

## Fix

Round the pad sum up (or state the slack the pads carry, at the site),
and restate the enclosure doc's bound in terms of Σ|terms|. If Ev picks
(S) on PR 4830, both follow from the rounded-once sum and this row
closes with that work.


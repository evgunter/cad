---
id: the-whole-body-pad-sum-rounds-to-nearest-and-the-enclosure-doc-scales-the-wrong-term
kind: issue
title: fold_runs sums flux_pad/area_pad round-to-nearest (not outward), and MassProperties::enclosure's error argument scales with |volume| where the fold's error scales with the sum of |face terms|
status: review
opened: 2026-10-12
priority: P3
cost: E
branch: flux/e-bundle-lane-honesty
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

## Resolution (FLUX, `flux/e-bundle-lane-honesty`)

Ev chose (A) on PR 4830 (storage is part of the input), so (S) does not
close this, and it is fixed here.

- **The pad sum rounds up.** `fold_runs` sums `flux_pad` and `area_pad`
  with `geom_core::interval::add_up` (new, beside `div_up`/`div_down`:
  inari's upward sum, exact sums exact, so a body of closed-form faces
  keeps pads of exactly `0.0`), and `volume_pad` is that sum's third by
  `div_up`, which the `/ 3.0` also rounded to nearest. Row:
  `topo::props::fold_pad_rows::the_folded_pads_round_up_and_zero_stays_exact`
  (`1` and a quarter of its ulp; zero pads). Both mutants (`+=` back;
  `/ 3.0` back) red it.
- **The enclosure doc** (`MassProperties::enclosure`) now bounds the
  fold's error by `≈ (n − 1)·u·Σ_f |flux_f| / 3`, which reaches check 7's
  band at `D ≈ 3·K·ε / (n·u)` from the origin, and says what reads these
  ends: check 7's certified read (`certify_role`) decides off the
  recentred interval re-derivation wherever the walk holds a lane, and
  these ends decide only a walk holding none
  (`work/tally/lane-free-volume-sign-reads-decide-on-a-rounded-sum`).
- `step-import`'s `nurbs_import.rs` comment says "rounded up".

**Re-baselined: the sweep thread-count digest**, `vpad` only, each one
ulp up (the stored third of the summed pad, now rounded up; no `apad`
moved):

| ε | body | vpad |
|---|---|---|
| 1e-6 | quintic_prism | `…f0ab` → `…f0ac` |
| 1e-6 | arc_loft_1e9eps | `…aaab` → `…aaac` |
| 1e-9 | tilted_cut_upper | `…aaab` → `…aaac` |
| 1e-9 | arc_loft_1e9eps | `…aaab` → `…aaac` |
| 1e-12 | arc_loft_1e9eps | `…5555` → `…5556` |


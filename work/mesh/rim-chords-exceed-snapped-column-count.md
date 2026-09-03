---
id: rim-chords-exceed-snapped-column-count
kind: issue
title: Rim chords can exceed the snapped column count on a malign band (residual off-lattice risk under TESS-SPLIT)
status: open
opened: 2026-08-23
github: 950
---

## From GitHub issue 950

opened 2026-08-23, 0 comments.

Filed from the TESS-SPLIT unit as the concretely scheduled followup for a disclosed residual.

## The gap

`band_schedule`'s malign-band snap aligns every near-malign band and the chord pass on ONE column family: near-malign bands take `patch_nuc` exactly, and a full-width iso rim's chord count reproduces `patch_nuc` because both derive from the same whole-patch `split_steps`. That alignment has one hole, and it predates TESS-SPLIT: `chords::nurbs_tighten` takes the MAX of the face-driven count and the edge curve's own 3-D chord bound, so a rim whose curve bound demands more chords than `patch_nuc` puts off-lattice u-points on the rim. Beside a malign (high realized `s_u/s_v`) first or last band, such a point admits the `(aspect²+1)/8·δ_s` Delaunay sliver.

Under TESS-SPAN the realized aspects in that configuration were bounded near `SAFE_ASPECT`, so the sliver certified within the measured margin. Under TESS-SPLIT's aspect-capped selection, snapped bands legitimately run to realized parameter aspects in the hundreds (protected by alignment), so a rim whose curve bound exceeds `patch_nuc` beside such a band would certify far above delta and the face would refuse typed (`CertificateExceeded`) — fail-loud, never a wrong mesh.

## Measured state

Over the whole tour and the wild corpus at this unit's head, no face hits the configuration (every rim's curve bound came in at or below the face-driven count wherever the adjacent band is malign; the budget sweep and both render corpora tessellate green). The risk is therefore a reachable-by-construction refusal, not an observed failure.

## What a fix could look like

Either teach the rim chord count to snap UP to the least multiple of `patch_nuc` at or above the curve bound (keeps chord points a superset of the columns on full-width rims), or teach `band_schedule` to raise a rim-adjacent malign band's columns to the rim's realized chord count. Both keep the one-derivation shape; neither is needed until a body actually presents the configuration, which the typed refusal will name loudly.

## Home

`band_schedule` and `chords::nurbs_tighten` are `crates/mesh/*`, S-MESH's territory, and sizing intent versus budget is its charter.

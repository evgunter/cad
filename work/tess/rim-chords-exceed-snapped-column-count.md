---
id: rim-chords-exceed-snapped-column-count
kind: issue
title: Rim chords can exceed the snapped column count on a malign band (residual off-lattice risk under TESS-SPLIT)
status: closed
opened: 2026-08-23
priority: P0
cost: H
closed: 2026-10-21
pr: 5551
branch: tess/snap-witness-or-retire
---

## From GitHub issue 950

Opened 2026-08-23; 0 comments.

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

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Superseded (2026-10-21)

**The hole is gone.** It lived in the column family the malign-band snap
made: snapped bands at `patch_nuc`, rims at the same count. The snap is
retired (`work/tess/the-lower-malign-arm-is-unwitnessed`), so no band is
aligned to `patch_nuc` and there is no family for a rim to miss.

**The remaining risk.** An off-lattice rim point beside a band whose
residual aspect over the face's lattice metric is far from 1 still admits
the `(aspect² + 1)/8 · δ_s` sliver. Since #5531 the CDT runs over that
metric, so the residual, not the raw-UV aspect, is what matters.

**The evidence against it.**
- Over the metric, the tour's worst band residual is 0.41–1.88.
- On the adversary faces (degree-2 uneven-station lofts; residuals pushed
  to 0.03–30; rims beside the thin first band), at twenty δ with the
  falsifier armed, every face meshes under δ or refuses typed, the same
  with the snap and without it. The worst ratio without it is 0.4996.
- **No adversary rim had a curve bound over the face count.** Of 4000 rim
  chord decisions on those faces (`chords::nurbs_tighten`, the curve's own
  count against the face-driven one), none exceeded it. This item's exact
  configuration was not reached, as this item measured on the tour and the
  wild corpus.

**The backstop.** The typed `CertificateExceeded` this item already named
as the outcome: a face that slivers refuses loud, never a wrong mesh.

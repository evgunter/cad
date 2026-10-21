---
id: cylinder-lofted-through-uneven-stations-refuses-at-fine-delta
kind: issue
title: A cylinder lofted through uneven stations refuses CertificateExceeded at fine δ, independent of the column snap
status: closed
opened: 2026-10-21
priority: P2
cost: M
closed: 2026-10-21
pr: 5570
branch: tess/low-residual-rows
---


## What

Found by the column-snap witness probe (`the-lower-malign-arm-is-unwitnessed`).
A constant-radius cylinder (radius 1, four arcs per circle) lofted at degree
2 through stations at z = 0, e, 2e, 0.5, 1 refuses typed at fine δ. Its first
knot span is about 1.5e tall, beside spans of about 0.25. At the review's
twenty δ (log-spaced 5e-2 → 3e-4), falsifier armed:

| e | δ | bound |
|---|---|---|
| 0.01 | 2.5860e-3 | 0.006459416449197761 |
| 0.01 | 8.8080e-4 | 0.0016305456047800168 |
| 0.01 | 6.7288e-4 | 0.001476337933837251 |
| 0.01 | 5.1404e-4 | 0.0011414792172550568 |
| 0.003 | 1.9756e-3 | 0.007687239410615543 |
| 0.003 | 1.5092e-3 | 0.0066650886200568795 |
| 0.003 | 1.1530e-3 | 0.0057624666447135 |
| 0.003 | 8.8080e-4 | 0.006151165038250064 |
| 0.003 | 6.7288e-4 | 0.0067372093702815486 |
| 0.003 | 5.1404e-4 | 0.0011589301065568335 |
| 0.003 | 3.9270e-4 | 0.0015333494198050577 |
| 0.003 | 3.0000e-4 | 0.001606308171836993 |
| 0.001 | 1.9756e-3 | 0.0022909224258081166 |
| 0.001 | 1.5092e-3 | 0.0026148025147070815 |
| 0.001 | 1.1530e-3 | 0.003227596109379822 |
| 0.001 | 8.8080e-4 | 0.0023936674079394755 |
| 0.001 | 6.7288e-4 | 0.00673720937028156 |
| 0.001 | 5.1404e-4 | 0.007078438006961101 |
| 0.001 | 3.9270e-4 | 0.0069957793410394134 |
| 0.001 | 3.0000e-4 | 0.006738578791360516 |

e ≥ 0.03 meshes at every δ. The same stations on a frustum (radii
1 → 0.5) mesh at every δ, both creased (two arcs) and crease-free (four
arcs).

**Independent of the snap.** The snap fired on 0 bands of this face, and the
twenty rows refuse identically with it and without it. Every refusal is
typed. No mesh is wrong, so this is liveness, not soundness.

## To do

Trace one row as `lofted-rational-walls-refuse-at-some-deltas` was traced:
log the offenders' positions per refinement round. Candidates:
- the thin first band's realized aspect over the metric, against the thick
  bands' (a residual the metric cannot square for both);
- the cylinder's `mvv = 0` cap arm, which sizes the thick bands to the
  aspect window's end.

That the frustum meshes points at the second candidate. Witness row: this
body at the twenty δ meshes, watertight and under δ.

## Resolution (2026-10-21): rows for narrow-column bands

**Traced.** At e = 0.003 and δ = 1.5e-3 the cylinder has 48 bands in three
groups:

| v range | band aspect | residual over the metric (4) |
|---|---|---|
| 0–0.0045 | 56.4 | 14.1 |
| 0.0045–0.253 | 1.02 | 0.26 |
| 0.253–1.0 | 0.34 | **0.085** |

All 1472 offenders (ratio up to 4.4) sit at v 0.30–0.67, inside the thick
bands. Their columns are far narrower than their rows over the metric, so an
off-row trim point fans a sliver. A column alignment could not help: those
bands already sat at the patch column count.

**Fixed.** `band_schedule` raises a band's rows until its residual reaches
`1/√8` (`MAX_RESIDUAL_SQ`, derived from the strip-sliver bound over every
split of the sizing budget at `δ_s = δ/2`). At the twenty review δ, falsifier
armed:
- the cylinder meshes at every δ for e = 0.03 down to 1e-4 (main refuses 32
  rows for e ≤ 0.01);
- worst ratio 0.4996, every mesh watertight.

The cost is filed as
`work/budget/row-raised-bands-cost-grows-with-the-band-spread`. Witness:
`lofted_circle::an_uneven_station_cylinder_meshes_at_every_review_delta`;
the no-raise mutant reds it (slow set).

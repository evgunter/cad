---
id: two-copies-of-a-pierce-carry-edges-that-run-within-the-band
kind: issue
title: A near-tangent two-run pierce leaves its copies apart on one point, and the two edges leaving them run within the band for a stretch the census passes, at 89 poses main refused
status: open
opened: 2026-10-04
priority: P0
cost: M
refs: [boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm, the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start, near-tangent-boolean-results-ship-with-an-escalated-tier-3-census, a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
---



## What

Found by PR 4026's delta review (m1) on head `f88ad91e`. **These are
near-tangent bodies that PR 4026 newly certifies**: main refuses every
pose below, and the head builds them and passes them at tier 3′.

A two-run pierce whose copies no face of the result meets keeps them
apart on their one point (the shared-point ruling, PR 3813;
`boolean::finish::weld_pierce_copies` welds only copies a face meets).
In a near-tangent pose, one edge leaves each copy, and the two edges
leave the point a few tenths of a microradian apart. They are distinct
edges on distinct vertices, so neither is backed by the other. Yet they
run within the band for a stretch, up to about 0.4 of their length.

**The witness.** The pinned pose: `sweep` `join_pierce_runs_sweep.rs`
`a_near_tangent_two_run_pierce_builds_with_edges_in_band_only_at_its_copies`
(review r1's `shallow200 nt e0 a3 d1e-7` at the default ε). Prism ∪ cube
holds two vertices at `v = (2, 0.6, 1)`, and no face meets both. The
prism top edge's piece leaves one of them, and a section edge leaves
the other. They are 3.7e-7 rad apart, so they lie within ε = 1e-9 for
about 2.7e-3 from `v`, and within εK for about 2.7e-2. The row asserts
the copies stay apart in ∪ and are welded in ∖. It does not claim the
two edges apart.

**Why the census passes them.** `census.rs` `pair_edge_edge` levers
`|ea.dir × eb.dir|` at the shorter edge's full length (about 2.0), so
it reads 7.4e-7, "not parallel", and the pair goes to the crossing
lane. There the only meeting is the shared point, which is endpoint
territory. Under D3 a collinear overlap whose far bound carries no
vertex is unbacked. The pair escapes only because the full arm reads
"not parallel". That is the arm question of
`boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm`.

The collinear-lane false positive
(`the-census-edge-edge-collinear-lane-reads-the-offset-at-the-long-edges-start`)
is a different pair at the same pose. Its fix does not reach this one,
which diverges from a shared point. The escalated class
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`)
is the census's other near-tangent residue from the same PR.

**Measure.** The review's independent in-band census
(`review-delta/delta_inband.rs` on branch
`join/pierce-two-out-runs-delta-review`) lists, by plain segment
distance, every non-adjacent edge pair within 3·εK. It was run over
PR 4026's review batteries, main against head.

Counting pairs whose two edges end on one point at two vertices:
- 116 tier-3′-SOUND lines and 17 BAD lines, at 89 poses. Of the BAD
  lines, 16 are `CensusEscalated`. The 17th is the collinear-lane false
  positive at the pinned pose (prism ∪ cube), whose finding names
  another pair;
- main refuses every one;
- `93ecd145` is identical to head;
- at `shallow200 nt e0 a1 d1e-8`, the pair stays within 3 bands for
  1.25 of its 2.0 length.

The poses, with ∪/∖/∩ lines in either operand order counted per pose,
and the longest stretch within 3·εK:
r1 `cube`:
- `Ltop nt e2 a8 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.003
- `Ltop nt e2 a9 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.012
- `Ltop nt e2 a10 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.015
- `Ltop nt e2 a11 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0085
- `Lbot nt e2 a12 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.003
- `Lbot nt e2 a13 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.012
- `Lbot nt e2 a14 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.015
- `Lbot nt e2 a15 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0085
- `Lmirror nt e2 a4 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.003
- `Lmirror nt e2 a5 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.012
- `Lmirror nt e2 a6 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.015
- `Lmirror nt e2 a7 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0085
- `notch307 nt e0 a0 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.003
- `notch307 nt e0 a1 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.013
- `notch307 nt e0 a2 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.015
- `notch307 nt e0 a3 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.0089
- `notch307 nt e1 a4 d1e-6`: 0 SOUND, 2 BAD, in band ≤ 0.0033
- `notch307 nt e1 a5 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.013
- `notch307 nt e1 a6 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.015
- `notch307 nt e1 a7 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.009
- `notch307 nt e2 a7 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0045
- `notch307 nt e2 a8 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0075
- `shallow200 nt e0 a0 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.003
- `shallow200 nt e0 a1 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.013
- `shallow200 nt e0 a2 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.015
- `shallow200 nt e0 a3 d1e-6`: 0 SOUND, 2 BAD, in band ≤ 0.0092
- `shallow200 nt e1 a4 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.0031
- `shallow200 nt e1 a5 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.013
- `shallow200 nt e1 a6 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.015
- `shallow200 nt e1 a7 d1e-6`: 2 SOUND, 0 BAD, in band ≤ 0.009
- `shallow200 nt e2 a8 d1e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0025

r1 near-tangent:
- `Ltop nt e2 a9 d1e-5`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `Ltop nt e2 a9 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0045
- `Ltop nt e2 a9 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.12
- `Ltop nt e2 a10 d1e-5`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `Ltop nt e2 a10 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.005
- `Ltop nt e2 a10 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.15
- `Ltop nt e2 a11 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.003
- `Ltop nt e2 a11 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.083
- `Lbot nt e2 a13 d1e-5`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `Lbot nt e2 a13 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0045
- `Lbot nt e2 a13 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.12
- `Lbot nt e2 a14 d1e-5`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `Lbot nt e2 a14 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.005
- `Lbot nt e2 a14 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.15
- `Lbot nt e2 a15 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.003
- `Lbot nt e2 a15 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.083
- `Lmirror nt e2 a5 d1e-5`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `Lmirror nt e2 a5 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0045
- `Lmirror nt e2 a5 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.12
- `Lmirror nt e2 a6 d1e-5`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `Lmirror nt e2 a6 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.005
- `Lmirror nt e2 a6 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.15
- `Lmirror nt e2 a7 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.003
- `Lmirror nt e2 a7 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.083
- `notch307 nt e0 a0 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.03
- `notch307 nt e0 a1 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `notch307 nt e0 a1 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.12
- `notch307 nt e0 a2 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `notch307 nt e0 a2 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.15
- `notch307 nt e0 a3 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.0033
- `notch307 nt e0 a3 d1e-7`: 0 SOUND, 2 BAD, in band ≤ 0.084
- `notch307 nt e0 a3 d1e-8`: 0 SOUND, 2 BAD, in band ≤ 0.83
- `notch307 nt e1 a4 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.03
- `notch307 nt e1 a4 d1e-8`: 0 SOUND, 2 BAD, in band ≤ 0.29
- `notch307 nt e1 a5 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `notch307 nt e1 a5 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.12
- `notch307 nt e1 a6 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `notch307 nt e1 a6 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.15
- `notch307 nt e1 a7 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.003
- `notch307 nt e1 a7 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.084
- `notch307 nt e2 a7 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0015
- `notch307 nt e2 a7 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.043
- `notch307 nt e2 a8 d3e-6`: 1 SOUND, 0 BAD, in band ≤ 0.0025
- `notch307 nt e2 a8 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.071
- `shallow200 nt e0 a0 d1e-8`: 0 SOUND, 2 BAD, in band ≤ 0.29
- `shallow200 nt e0 a1 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `shallow200 nt e0 a1 d1e-8`: 1 SOUND, 1 BAD, in band ≤ 1.2
- `shallow200 nt e0 a2 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `shallow200 nt e0 a3 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.0031
- `shallow200 nt e0 a3 d1e-7`: 1 SOUND, 1 BAD, in band ≤ 0.083
- `shallow200 nt e1 a4 d1e-7`: 2 SOUND, 0 BAD, in band ≤ 0.03
- `shallow200 nt e1 a5 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `shallow200 nt e1 a5 d1e-8`: 1 SOUND, 1 BAD, in band ≤ 1.2
- `shallow200 nt e1 a6 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.005
- `shallow200 nt e1 a7 d3e-6`: 2 SOUND, 0 BAD, in band ≤ 0.003
- `shallow200 nt e1 a7 d1e-8`: 0 SOUND, 2 BAD, in band ≤ 0.83
- `shallow200 nt e2 a8 d1e-7`: 1 SOUND, 0 BAD, in band ≤ 0.023

r2 `r2_shapes_battery`:
- `R315 ne0_1e-6k2s1`: 1 SOUND, 0 BAD, in band ≤ 0.013

## The shape to give

Decide where this belongs.
- **At the census:** read the parallelism of two edges that share a
  point at a lever that sees the stretch where they are near. A
  vertex-anchored reading would be the arm-ladder row's door. The pair
  is then a collinear overlap with no far vertex, and it is an
  undeclared finding.
- **At the boolean:** a near-tangent two-run pierce should glue or
  refuse the stretch, as it would any in-band sliver.

Either way, rerun the batteries above. The pinned row's third clause
then changes, and says so.

## 2026-10-06 — the weld this rests on is retired (JOIN, PR 4139)

`boolean::finish::weld_pierce_copies` is gone: a pinch is one vertex per
cone (Ev, PR 4057), split before the zips (`zip::split_cones`). A
pierce's copies now meet as one vertex only where the zips' own
correspondence fuses them, which is where the kernel reads them as one
cone; it no longer matters whether a face meets both. The pinned
witness is unchanged (copies apart in ∪, one vertex in ∖), but the
row's count of poses may move and was not re-measured.

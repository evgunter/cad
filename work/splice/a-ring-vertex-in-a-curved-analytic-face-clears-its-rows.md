---
id: a-ring-vertex-in-a-curved-analytic-face-clears-its-rows
kind: issue
title: A ring vertex minted in a complete curved analytic face clears all its rows: the chord strut does not certify on the chart, and the join's keys-only doors rely on the cleared face
status: closed
opened: 2026-10-31
closed: 2026-10-31
priority: P3
cost: M
pr: 6166
branch: splice/spline-ring-vertex-mint
---


Found by `a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`'s
lane, on hosted CI for PR 6166.

## What

On a non-spline chart, `boolean::vtxfac::ring_vertex_in` minted
the ring vertex by a chord strut: `Body::mev` of a line edge from the
outer loop's anchor to the pierce point, then `kemr`. On a minted
curved analytic face, the line chord does not certify against the
chart, so the strut's site mint writes `SiteRows::Clear` and the face
loses every row until the producer's closing mint. This is witnessed
by `vtxfac_spline_rows`'
`a_ring_vertex_in_a_cylinder_wall_goes_through_the_struts_site_mint`.

The pipeline depends on that clear. Routing every chart through the
row-preserving `Body::ring_vertex` (the spline arm's door) left the
face complete, and `sweep`'s
`a_closed_conic_in_the_partners_face::a_rim_inside_a_ball_face` then
refused `Join(BooleanJoin(Euler(PcurveMint { face, refusal: KeysOnly })))`.
A keys-only door later in the join owes the complete ball face a row
it has no band to derive. PR 6166's review traced it, 6 of 6 hits:
`chord_join::join_lone_ring`'s `kfmrh` (called from `SplitJoin::join`,
from `boolean::join`), which moves the hole a pierce's conic cuts onto
the old face as its ring. So the analytic arm kept the chord, and the
analytic face went rowless mid-pipeline, blinding any reader of its
stored rows between Delta 3 and the close.

## What it needs

`join_lone_ring` takes `kfmrh_minting` with the run's band, after which
`ring_vertex_in` uses `Body::ring_vertex` on every chart. Then
`vtxfac_spline_rows`' cylinder row flips: it asserts the wall's rows
stand as found.

## Closed

- 2026-10-31 — Closed in PR 6166, which does both.
  `a_closed_conic_in_the_partners_face` stays green (18 of 18). topo's
  full suite (3746) and sweep's `ci` profile (2781) are green. The
  cylinder row is now
  `a_ring_vertex_in_a_cylinder_wall_leaves_its_rows_as_found`.


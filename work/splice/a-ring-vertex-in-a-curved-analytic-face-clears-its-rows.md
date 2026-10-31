---
id: a-ring-vertex-in-a-curved-analytic-face-clears-its-rows
kind: issue
title: A ring vertex minted in a complete curved analytic face clears all its rows: the chord strut does not certify on the chart, and the join's keys-only doors rely on the cleared face
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Found by `a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex`'s
lane, on hosted CI for PR 6166.

## What

On a non-spline chart, `boolean::vtxfac::ring_vertex_in` still mints
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
it has no band to derive; my guess is the re-parenting doors in
`splitting::finish` (`mfkrh`/`kfmrh`), not confirmed. So the analytic
arm kept the chord, and the analytic face goes rowless mid-pipeline,
blinding any reader of its stored rows between Delta 3 and the close.

## What it needs

Either the join's keys-only re-parenting doors take their `_minting`
twins with the run's band, after which `ring_vertex_in` can use
`Body::ring_vertex` on every chart. Or argue that a rowless curved
face mid-pipeline costs no reader anything, and close this.

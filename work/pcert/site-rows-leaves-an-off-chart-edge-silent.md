---
id: site-rows-leaves-an-off-chart-edge-silent
kind: issue
title: an Euler op's site mint clears a face whose new edge is off its chart, so the op answers Ok and tier 3 answers [] on a body the mint refuses
status: dispatched
opened: 2026-10-01
priority: P0
cost: M
refs: [D36, S331]
parent: S331
branch: pcert/at-rest-rows-mandatory
---


Filed by the PCERT orchestrator, 2026-10-01, from the full review of
PR 3664 (`D36`).

`D36` narrowed `mint_faces`' swallow to the uncovered class, so a body
whose edge is not on its face now refuses at the mint
(`CarrierOffChart`). One route keeps the old silence:
`topo::pcurves::site_rows`, the Euler operators' mid-surgery site mint,
maps every derive or certify error to `Clear` (`map_err(|_| ItemFail::Derive)`)
and leaves the face storing nothing. Measured on
`sweep::euler_site_row_frontiers::a_tilted_circle_strut_on_a_minted_cone_leaves_the_wall_unminted`:
`mev` adds a circle strut tilted off the cone's axis — not on the
cone — and answers `Ok`; `validate_pcurves` then reads the emptied wall
as never minted and answers `[]`; only a re-run of `mint_pcurves_of`
refuses. That is the defect `D36` closed, alive through the Euler path.

Clearing mid-surgery is by design (a later door finishes describing the
face), so the fix is not simply to propagate. What closes it depends on
the ruling on PR 3617 (`S331`): if tier 3 derives a rowless face at
rest, it measures this wall and refuses `CarrierOffChart` with no change
here; if rows stay mandatory, the site mint should keep the typed reason
rather than `Clear` for a defect class. Specified after that ruling.

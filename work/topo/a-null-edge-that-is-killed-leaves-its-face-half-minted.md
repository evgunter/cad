---
id: a-null-edge-that-is-killed-leaves-its-face-half-minted
kind: issue
title: A null edge the boolean or splitting pipeline kills undescribed leaves its face half-minted, and the join's Euler operators leave their own halves rowless on it
status: closed
opened: 2026-09-30
priority: P2
cost: M
branch: topo/null-kill-remint
pr: 3508
closed: 2026-09-30
---


Found by `mev-null-leaves-a-complete-curved-face-half-minted`
(branch `topo/null-edge-remint`), whose measurement it is.

That unit closes the half-minted state `Body::mev_null`
(`crates/topo/src/null.rs`) leaves on a minted curved face at the
null edge's first description: `Body::set_edge_curve`
(`crates/topo/src/attach.rs`, `null_description_rows`) re-mints the
face through `pcurves::site_rows` once no null edge is left on it
(`StoredRows::remints_at_description`). But no production pipeline
describes a null edge. Instrumented over topo's whole suite and
sweep's `ci` profile (probes on `Body::pcurve`, `mev_null`,
`set_edge_curve_via`, `kev_describing`'s gate and `mint_pcurves`;
not committed):

- `mev_null` meets a minted face with no gap but null rows in 120
  sweep tests, 1,090 calls: `boolean::vtxfac::classify_vertex_on_face`
  (932), `boolean::insert::mint_run` (94) and
  `splitting::insert::insert_null_edges` (64). Of those, 532 meet a
  truly complete face and half-mint it (`vtxfac` 429, `mint_run` 71,
  `splitting::insert` 32); the rest meet a face already missing only
  null rows, from an earlier `mev_null` on it. None in topo's suite,
  whose boolean operands carry no rows.
- `set_edge_curve` describes a null edge only in three topo unit tests
  (`euler::tests`, `review_m3_pr1`); `kev_describing` never lists one.
  The pipelines kill their null edges undescribed.
- Between the scaffold and the kill the face is read by the Euler
  operators' site-mint plan (`pcurves::site_rows_from`): the join's
  (`boolean::join::bool_connect` and `splitting::join::split_connect`),
  and `vtxfac`'s own `mev` (16 reads, in
  `verbs_1031b_arcwind::the_boolean_after_the_merge_reaches_the_join`).
  It finds the face half-minted and leaves it as found, so the
  operators' own new halves on it get no row either: 828 of those reads see a face missing two null rows and one
  other. `split_edge`'s restriction (`pcurves::split_cache`) never
  reads such a face in these suites.
- The boolean's and `split_direct`'s closing `mint_pcurves` then finds
  half-minted faces, none of them missing a null row (the null edges
  are gone), in 100 tests. 97 are among the 120 above; the other three
  are half-minted by other doors:
  `m8_4_intersection_iso::a_boundary_column_intersection_mints_its_iso_image`,
  `shell10_r1_probes::r1_the_door_no_longer_launders_a_half_minted_out_of_scope_face`
  and `shell9_r2_probes::r2_the_closing_mint_launders_an_invalid_operand`.

So in production the state `mev_null` opens lasts until the
producer's final pass, and the retired closing-mint convention is
still what closes it there. Shapes: the kill that removes a null
edge from a face complete but for the rows the join's operators could
not mint re-mints it (the killed halves are what blocked the walk);
or the join's operators, on a face complete but for null halves, mint
the loops that no longer run through a null edge. Either needs its
own measurement of which faces are complete once the cut is done.

## Closed (2026-09-30, PR 3508)

This closes the production half of Ev's PR 2527 ruling: a
half-minted face is a state no door can produce.

**Measurement, and the shape it picked.** The measurement ruled out
the kill doors (shape a): the pipelines' kills act on rowless slivers,
and 59 of 6,385 null-edge kills touch a minted face. The fix is at the
join's `mef` (shape b), where 1,574 of 1,629 last-null-half losses
happen.

**One predicate for both doors.** `StoredRows::remints(open,
released)` serves the operators and the description alike. A face is
re-minted when:
- it stores a row;
- every loop walks;
- every missing row is on a loop a null edge holds open, unless the
  door released the face's last null edge, in which case it is
  re-minted whole.

The helper `held_open` is the one spelling, and it fails loud on an
unresolved key.

**Result.**
- 0 half-minted faces leave the boolean join. `split_direct`'s
  closing mint meets 0, where it met 78 before.
- The boolean merge still re-mints 657 faces. All 657 come from the
  seam zip's `kef`, and none is null-caused; that is ZIP's row.

**Review and fix pass.** The single review found a regression against
PR 3500 (a description on a face with a foreign gap), and the fix
pass closed it with the release clause.

**Filed:**
- `a-kill-that-releases-a-loop-from-its-last-null-edge-leaves-its-gaps`
  (P3);
- `a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass`
  (P3);
- `work/zip/the-seam-zips-kef-leaves-the-wall-it-closes-half-minted`.

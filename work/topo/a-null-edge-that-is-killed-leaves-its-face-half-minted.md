---
id: a-null-edge-that-is-killed-leaves-its-face-half-minted
kind: issue
title: A null edge the boolean or splitting pipeline kills undescribed leaves its face half-minted, and the join's Euler operators leave their own halves rowless on it
status: open
opened: 2026-09-30
---


Found by `mev-null-leaves-a-complete-curved-face-half-minted`
(branch `topo/null-edge-remint`), whose measurement it is.

That unit closes the half-minted state `Body::mev_null`
(`crates/topo/src/null.rs`) leaves on a complete curved face at the
null edge's first description: `Body::set_edge_curve`
(`crates/topo/src/attach.rs`, `null_description_rows`) re-mints the
face through `pcurves::site_rows`. But no production pipeline
describes a null edge. Instrumented over topo's whole suite and
sweep's `ci` profile (probes on `Body::pcurve`, `mev_null`,
`set_edge_curve_via`, `kev_describing`'s gate and `mint_pcurves`;
not committed):

- `mev_null` half-mints a complete face in 120 sweep tests (1090
  calls): `boolean::vtxfac::classify_vertex_on_face` (932),
  `boolean::insert::mint_run` (94) and
  `splitting::insert::insert_null_edges` (64). None in topo's suite,
  whose boolean operands carry no rows.
- `set_edge_curve` describes a null edge only in three topo unit tests
  (`euler::tests`, `review_m3_pr1`); `kev_describing` never lists one.
  The pipelines kill their null edges undescribed.
- Between the scaffold and the kill the face is read by one step:
  the join's Euler operators' site-mint plan
  (`pcurves::site_rows_from`, from `boolean::join::bool_connect` and
  `splitting::join::split_connect`). It finds the face half-minted and
  leaves it as found, so the join's own new halves on it get no row
  either: 828 of those reads see a face missing two null rows and one
  other. `split_edge`'s restriction (`pcurves::split_cache`) never
  reads such a face in these suites.
- The boolean's and `split_direct`'s closing `mint_pcurves` then finds
  half-minted faces, none of them missing a null row (the null edges
  are gone), in 97 tests, every one of them among the 120 above.

So in production the state `mev_null` opens lasts until the
producer's final pass, and the retired closing-mint convention is
still what closes it there. Shapes: the kill that removes a null
edge from a face complete but for the rows the join's operators could
not mint re-mints it (the killed halves are what blocked the walk);
or the join's operators, on a face complete but for null halves, mint
the loops that no longer run through a null edge. Either needs its
own measurement of which faces are complete once the cut is done.

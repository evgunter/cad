---
id: strut-side-follows-facing-is-spelled-three-times
kind: issue
title: A strut's mint side follows its facing, and that derivation is spelled three times (four with the ring struts)
status: closed
opened: 2026-10-04
priority: P2
cost: M
refs: [whole-orbit-fan-end-has-three-spellings, a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
branch: join/strut-side-one-rule
pr: 4294
closed: 2026-10-08
---


## What

Found in the review of PR 4004 (`join/fan-end-one-spelling`, style
finding, Q1).

The sense theorem (join module docs) fixes a null edge's mint side
and attribute from the facing of its halves: the half facing the
run's start germ is the UP half. Strut facing has one rule now
(`boolean::insert::strut_faces_first`), but the step from facing to
`NewVertexSide` and `NullEdge { below_end, above_end }` is still
written out at each minter:

- `splitting/insert.rs`, `insert_null_edges` (about `:124-140`):
  hard-codes `whole_orbit → Below` and spells the attribute from the
  same flag;
- `boolean/vtxfac.rs`, `classify_vertex_on_face` (about `:660-680`):
  derives `start_on_plus` from `strut_faces_first`, then matches it
  into a side, a half pair and an attribute;
- `boolean/insert.rs`, `mint_run` (about `:1562` and `:1594`): swaps
  the attribute side and the germ halves on
  `dangling && !spike_from_first`;
- the class's fourth: the pierce's ring struts
  (`boolean/vtxfac.rs`, `classify_vertex_on_face` step 3). They read
  their facing by the walk about the pierced face
  (`insert::strut_order`), and then match it into a side, a half pair
  and an attribute, as the second bullet does.

All three agree today (the PR 4004 batteries); they can drift apart
because nothing makes them one datum.

## The shape to give

One function from `(run side, he_plus faces start germ)` to
`(NewVertexSide, NullEdge, [start half, end half])`, called by every
null-edge minter, so the attribute, the mint side and the record's
germ halves come from one place.

## Built

`Body::mev_null_run` (`crates/topo/src/null.rs`) is the one function:
from the run's side and whether `he_plus` faces the start germ it
mints the null edge and returns the attribute and the
`[start half, end half]` pair. `insert_null_edges`, `mint_run`, the
pierce's run strut and its ring struts all call it.

Measured (release, main `b1a70019` against head): `pierce_runs_battery`
4538 lines, `pinch_runs_battery` 3026, `corner_pairs_battery` 16382 and
`j3r2_r1_reflex_battery` 1154 are byte-identical, and the seven
`join_pierce_strut_facing` rows pass on both. Negating the facing at
each caller in turn reds rows at every former site (topo `ci` profile
plus sweep's join/pierce/pinch/corner rows): splitting 35 + 6, `mint_run`
60 + 38, the pierce strut 151 + 73, the ring struts 1 + 15.

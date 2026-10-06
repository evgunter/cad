---
id: a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin
kind: issue
title: A vertex of one solid strictly inside a declared Rest contact, with no vertex of the other there, has no twin for the REST zip
status: open
opened: 2026-10-02
priority: P1
cost: H
---


Found by the REACH fix pass on `reach/fullturn-bore-mate` (PR 3814).

## Fixture

`crates/sweep/tests/full_turn_bore_mate.rs`'s `split_collar()`: the
full-turn collar with its bore split into two full-turn faces by the
circle at `y = 1.5` (a vertex `w` where it crosses the bore's seam
ruling at azimuth 0). Against `shaft(60, …)` — no shaft ruling at
azimuth 0 — `w` lies strictly inside a shaft wall third.

## Measured

`collar ∪ shaft`, both orders, identity pose:

- flush (`y ∈ [1, 2]`): `Join(UnpairedLooseEnds { count: 16 })` — the
  REST zip's `mirror_edges` returns `Ok(None)`: the split circle is an
  interior edge of A's contact patch whose ends (`w`, twice) have no
  counterpart in B, so the zip cannot give it a twin.
- through, proud above, proud below: `Join(SectionArcWindow {
  case: NoChartedRun })` — the same zip exit, surfacing the normal
  join's own refusal.

At azimuth 0 (`w` on a shaft ruling, so the reduction records a v-v
pair there) every span unions in both orders at every pose
(`a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth`).

## What would close it

The reduction places `w` inside B's face as a vertex–face contact and,
the sectors there all lumping as `Rest`, mints nothing on B. The zip
needs a vertex of B at `w` (a lone ring vertex the twin chord can
start from, the pierce-ring shape `mint_chord` already takes via
`mekr`), and a closed interior edge (`u == v`) needs a self-loop twin.

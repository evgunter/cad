---
id: a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin
kind: issue
title: A vertex of one solid strictly inside a declared Rest contact, with no vertex of the other there, has no twin for the REST zip
status: parked
opened: 2026-10-02
priority: P1
cost: H
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
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

## Measured on main (2026-10-06)

Measured on `origin/main` 3f1e3b0d: `split_collar()` against
`shaft(60, …)`, at the six `poses()`, both operand orders, and the
1e-9, 1e-6 and 1e-12 rows.

- **Flush, proud above, proud below:** the union builds in the chord
  join at its closed form (one shell, tier 3, the census). The
  declared-REST zip is not entered. Pinned by `full_turn_bore_mate.rs`'s
  `a_bore_split_on_its_own_carrier_unions_off_the_seam_where_the_shaft_ends_at_a_rim`.
- **Through:** the join refuses `Join(RingHomingAmbiguous { ring })`,
  from `ChordJoiner::rehome_rings` reading `OnBoundary`. The zip
  realizes all eight seam segments (`mint_chord`, rim arcs on `Circle`
  twins) and mirrors the shaft's six ruling pieces into the collar
  (`Line` twins). The pass mirroring the collar's interior edges into
  the shaft then returns `Ok(None)` at `mirror_edges`' no-counterpart
  exit. The edge is the collar's seam-ruling piece from `(0.5, 1, 0)`
  to `w = (0.5, 1.5, 0)`; its lower end has a shaft counterpart and
  `w` has none. So the join's refusal stands verbatim. `mint_chord` is
  not reached for that edge.
- **Undeclared:** through refuses `CurvedPierceUnsupported` in the
  reduction. Flush and the proud spans refuse `UndeclaredCoincidence`
  (`SameOriented`, the shaft's cap flush with the collar's).

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/join/log.md`, the 2026-10-03 hold). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.

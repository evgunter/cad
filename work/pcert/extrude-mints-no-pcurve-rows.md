---
id: extrude-mints-no-pcurve-rows
kind: issue
title: extrude mints no pcurve rows, alone among the producers, so its cylinder walls reach rest rowless
status: closed
opened: 2026-10-01
priority: P1
cost: E
parent: S331
branch: pcert/at-rest-rows-mandatory
closed: 2026-10-02
pr: 3759
---


Filed by the PCERT orchestrator, 2026-10-01 (the `S331` design weighing,
`[ev]` PR 3617).

`sweep::extrude` (`crates/sweep/src/extrude.rs`) never calls
`topo::mint_pcurves`; every other producer the posture docs in
`crates/topo/src/pcurves.rs` list — loft, revolve, tube, blend, shell,
booleans, offset, revert, transform — does. So an extruded arc profile
returns cylinder walls storing no row. No reason is recorded anywhere
(`chart_boundary`'s doc names "every extrude" as a body that never ran
the pass, in passing). The one structural difference is the scalar
bound (`extrude<T: Decide>`, `mint_pcurves<T: AtRestPolicy>`), and it
does not bite: the walls are cylinders, which derive in closed form at
`Decide`, as the Euler operators' site mint already does.

The fix is uniformity: extrude mints its curved walls like every other
producer (through the `Decide`-bounded site route, or by raising the
bound if that is cleaner). How much it matters depends on Ev's ruling
on PR 3617: under "rows are mandatory at rest" a rowless wall is a
tier-3 finding; under "rows are a cache" it is a cold cache. Either
way the producer posture should be one rule.

## Closed

Closed by PR 3759 (`pcert/at-rest-rows-mandatory`, merged 82b52c36c): on every curved chart a face's pcurve rows are mandatory at rest (C4). Tier 3 dry-mints a rowless face and reports `Unminted` or the derivation's refusal, re-certifies and continuity-checks a half-minted face's stored rows like a complete one's (with `RowInterval`), reads an excused face whole before excusing it, and `sweep::extrude` closes with `topo::mint_pcurves`. Residue has its own rows: the four uncovered-class `mint-has-no-route-*` rows and `at-rest-trim-containment-checks-against-the-stored-hull`.

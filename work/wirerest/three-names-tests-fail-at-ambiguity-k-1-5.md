---
id: three-names-tests-fail-at-ambiguity-k-1-5
kind: issue
title: Three names/union tests fail on main under CAD_AMBIGUITY_K=1.5, a K that Tolerance accepts
status: open
opened: 2026-10-21
priority: P3
cost: M
refs: [a-sliver-beside-a-rest-stands-outside-the-band-below-the-default-k]
---


Found by `wire-emithelpers`, 2026-10-21. I reproduced it on `origin/main` at `3ec34bb88f` with `CAD_AMBIGUITY_K=1.5 cargo nextest run -p editor-core`. All three fail by assertion, not by panic:
- `names::emit_topo::nurbs_crossings_rank_by_parameter::a_nurbs_piece_with_a_control_step_in_the_band_ties_its_crossings`. At `emit_topo.rs:7620` it gets `Some((0, 1))` where it wants `None`: the crossings rank instead of tying.
- `names::emit_topo::rim_holding_rows::an_arc_a_few_tolerances_long_escalates_its_way`, at `emit_topo.rs:9027`.
- `union_member_orders_decide_alike::the_corpus_reaches_its_classes`, at `tests/union_member_orders_decide_alike.rs:548`.

All three pass at the default K and at `CAD_TOLERANCE_EPS=1e-6`. Each fixture appears to place its witness inside the band at the default K, and at K = 1.5 the witness lies outside it. Each fixture should then either derive its witness from the band, or loud-skip at a K whose band it cannot reach. Which one is the fixture's call; the kernel's output may be right at K = 1.5.

## 2026-10-26 — still red on main; the sliver row merged in (sweep follow-up)

Re-run at `CAD_AMBIGUITY_K=1.5` on main `d621f67e`: all three still fail
by assertion, at the lines they have moved to:

- `a_nurbs_piece_with_a_control_step_in_the_band_ties_its_crossings`,
  `emit_topo.rs:7685`: `Some((0, 1))` against `None`, as before;
- `an_arc_a_few_tolerances_long_escalates_its_way`, `emit_topo.rs:9290`:
  "an arc two escalate bands long escalates its way, got
  `Err(WayInZeroBand { edge: EdgeKey(1v1), shape: Carrier })`";
- `the_corpus_reaches_its_classes`,
  `tests/union_member_orders_decide_alike.rs:553`: "a sliver beside a
  rest: Built { rows: [...] }".

The third had its own row,
`a-sliver-beside-a-rest-stands-outside-the-band-below-the-default-k`
(WIRE's `wire-bridges` lane), closed as this row's duplicate. Its
diagnosis, carried here: `a_sliver_beside_a_rest` stands its third block
`2.0 * eps` off the second's far wall, and the class test asserts the
sliver refuses in band. The band is `(ε, Kε)`, open at `Kε`, so a 2ε gap
is outside it for every K ≤ 2 that `Tolerance` accepts, and the union is
rightly decided and builds. The fix it named: place the sliver inside the
band for every K > 1 (a gap read from the ambient K, strictly between ε
and Kε), or have the class test read which side of the band the gap
falls on. The rest of that lane's selection
(`name_invariance::an_operand_order_moves_no_name`, the bridge rows in
`union_pinch_member_order`) was green at K = 1.5 and at
`CAD_TOLERANCE_EPS=1e-6`. Its band, P3, is carried here; the cost is M
because the other two fixtures are not yet diagnosed.

## Re-pointed (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). One of the three tests (`nurbs_crossings_rank_by_parameter`) is deleted. Its NURBS tripwire moved to `wavy_loft` and passes. The other two still fail at K=1.5.

## Moved to WIREREST (2026-10-27)

WIRE measured 34 budget points against 30, so this P3 row moved by `git mv` (id and body unchanged) to WIRE's second tail. No code cites its path. Signed (WIRE orchestrator).

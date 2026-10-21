---
id: three-names-tests-fail-at-ambiguity-k-1-5
kind: issue
title: Three names/union tests fail on main under CAD_AMBIGUITY_K=1.5, a K that Tolerance accepts
status: open
opened: 2026-10-21
---


Found by `wire-emithelpers`, 2026-10-21. I reproduced it on `origin/main` at `3ec34bb88f` with `CAD_AMBIGUITY_K=1.5 cargo nextest run -p editor-core`. All three fail by assertion, not by panic:
- `names::emit_topo::nurbs_crossings_rank_by_parameter::a_nurbs_piece_with_a_control_step_in_the_band_ties_its_crossings`. At `emit_topo.rs:7620` it gets `Some((0, 1))` where it wants `None`: the crossings rank instead of tying.
- `names::emit_topo::rim_holding_rows::an_arc_a_few_tolerances_long_escalates_its_way`, at `emit_topo.rs:9027`.
- `union_member_orders_decide_alike::the_corpus_reaches_its_classes`, at `tests/union_member_orders_decide_alike.rs:548`.

All three pass at the default K and at `CAD_TOLERANCE_EPS=1e-6`. Each fixture appears to place its witness inside the band at the default K, and at K = 1.5 the witness lies outside it. Each fixture should then either derive its witness from the band, or loud-skip at a K whose band it cannot reach. Which one is the fixture's call; the kernel's output may be right at K = 1.5.

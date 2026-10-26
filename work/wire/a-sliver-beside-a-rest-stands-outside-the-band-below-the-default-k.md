---
id: a-sliver-beside-a-rest-stands-outside-the-band-below-the-default-k
kind: issue
title: The corpus's sliver beside a rest stands 2ε off, outside the band at K = 1.5, so the_corpus_reaches_its_classes reds at every K ≤ 2
status: closed
priority: P3
cost: E
opened: 2026-10-21
closed: 2026-10-26
refs: [three-names-tests-fail-at-ambiguity-k-1-5]
---


Found by WIRE's `wire-bridges` lane (branch
`wire/bridged-wedges-pair-booleans`) probing D9 with
`CAD_AMBIGUITY_K=1.5`.

## What

`union_member_orders_decide_alike::a_sliver_beside_a_rest`
(`crates/editor-core/tests/union_member_orders_decide_alike.rs`) stands
its third block `2.0 * eps` off the second's far wall, and
`the_corpus_reaches_its_classes` asserts that the sliver refuses in
band. The band is `(ε, Kε)`. At `CAD_AMBIGUITY_K=1.5`, a gap of 2ε lies
past it, so the union is decided and builds:

    a sliver beside a rest: Built { rows: [...] }

That is the correct behaviour: a gap outside the band is decided. The
fixture is sized for the default K only, so the class test goes red
for every K ≤ 2 that `Tolerance` accepts (the band is open at Kε). The rest of the selection
(`name_invariance::an_operand_order_moves_no_name`, the bridge rows in
`union_pinch_member_order`) is green at K = 1.5 and at
`CAD_TOLERANCE_EPS=1e-6`.

## What it needs

Place the sliver inside the band for every K > 1 (for example, a gap
read from the ambient K, strictly between ε and Kε), or have the class
test read which side of the band the gap falls on.

## Closed (2026-10-26, sweep follow-up)

A duplicate of `three-names-tests-fail-at-ambiguity-k-1-5`, which names
the same red (`the_corpus_reaches_its_classes` at K = 1.5) beside two
other fixtures that fail at the same K. That row survives because it
holds all three; this row's diagnosis, its fix and its P3 band are
carried into it. Still red on main `d621f67e`
(`tests/union_member_orders_decide_alike.rs:553`, "a sliver beside a
rest: Built").

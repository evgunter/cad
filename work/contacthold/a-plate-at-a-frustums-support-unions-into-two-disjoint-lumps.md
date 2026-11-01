---
id: a-plate-at-a-frustums-support-unions-into-two-disjoint-lumps
kind: issue
title: Main's sweep slow set: plates at the exact support of a frustum or torus union into two disjoint lumps (Assembly) since one of 4598/4604/4597/4573/4636/4667/4671
status: closed
opened: 2026-10-11
priority: P0
cost: M
refs: [main-reds-on-plates-at-the-exact-support-of-a-frustum-and-a-torus]
closed: 2026-10-12
---


Found by FLUSH's PR 4649. Its diff only adds a test, so this red came
from main.

## What

`sweep::all operand_gate_support_plates::plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op`
(slow set; GERM's test) fails on the merge of main `6e3985cb76`. Its
failure lines read, for example:

    [ε 1e-9] frustum full s 1e-3 dir0 … δ 0e0 a∪b: Body(Assembly): touching operands built as disjoint

The union of a plate that touches the frustum at its exact support (δ 0
and δ −1e-9, both operand orders, scales 1e-3 and 1) returns two
disjoint lumps instead of one fused body. 1080 runs, 510 built.

## Range, not bisected

- Passed in the same slow set on PR 4649's earlier head `ccb997d4f`,
  whose base was main `5d3e5ea0a8`.
- Fails at main `6e3985cb76`.
- The first-parent merges between the two: #4667, #4598, #4597, #4636,
  #4573, #4604, #4671.
- The most likely is #4604 (CONTACTHOLD: the census reads touches from
  carriers, every non-incident pair is recorded, curved face–face rests
  get a stored record), with #4598 next. That is a guess from the
  symptom, a union of touching curved operands, and is not measured.

The sweep slow set runs on a PR only when its diff touches the sweep
tests, so a topo-only PR does not see this test. Owed: bisect the
range, root-cause, and fix where the cause lies.

## Worked under GERM's row (2026-10-11)

Bisected: PR 4604 (see GERM's row for the table and reading).

The same red is `work/germ/main-reds-on-plates-at-the-exact-support-of-a-frustum-and-a-torus.md`,
already dispatched by CLEAVE (`cleave/support-plates-main-red`). To avoid
two bisects, CONTACTHOLD does not dispatch a second lane. If the bisect
lands on PR 4604 or 4598, CONTACTHOLD takes the fix or supports it (the
census-by-carrier and every-non-incident-pair record are this program's).
This row closes with GERM's. (CONTACTHOLD orchestrator)

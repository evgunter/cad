---
id: ssi-prefer-the-simplest-certified-candidate
kind: issue
title: ssi: 'march first, then Hermite' is an ordering, not a rule; prefer the simplest certified candidate (Hermite first, march only if it refuses)
status: closed
opened: 2026-10-03
priority: P2
cost: M
pr: 3983
branch: ssi/hermite-first
closed: 2026-10-03
---


(SSI orchestrator, from Ev's question on PR 3862, 2026-10-03: "'march first, then hermite' also sounds sketchy unless it's a runtime optimization". Ev agreed with the direction below: "sounds great!")

## What

PR 3862 traces each branch between two known ends by marching first. Only when the march can't make progress (its step falls in the band) does it try the Hermite cubic through the two certified ends (`crates/geom-brep/src/ssi/ends.rs`).

Both are candidates, and the certificate decides, so the order can't make an answer wrong. It does choose which certified carrier is returned, and the "step falls in the band" trigger is a heuristic switch.

## Direction (Ev agreed)

Prefer the simplest certified candidate:
1. For every branch between known ends, try the Hermite cubic first. It is one span and exact at both ends.
2. Only if the certificate refuses it, march.
3. The march's own refusals stand.

This removes the step-in-band trigger. Short and nearly straight branches get one-span carriers. The cost is one cheap certification attempt per branch.

## Check

- Pairing: the Hermite currently pairs a crossing with the nearest unused one. With Hermite tried on every branch, a wrong pairing must refuse at the certificate (limbs 1/2). Verify on converging branches: PR 3862's delta reviewer found `max_by` pairings were all refused.
- Measure the cost across the suite.
- `ShortBranchUncertified`'s sizing has to stay honest.

## Done when

- The candidate order is "simplest first", stated in C3.
- No step-in-band trigger remains.
- Rows pin a one-span carrier on a straight branch, and a march on a curved one.

## Closed

PR 3983. `Ends::branches` tries the Hermite cubic first on every branch between known ends, and marches only where the certificate refuses it; `step_in_band` is gone, and C3 states the order. Rows: `a_straight_branch_is_one_hermite_span_at_any_length`, `a_curved_branch_the_hermite_cannot_certify_is_marched`, `converging_crossings_certify_no_wrong_pairing` (`m5_pr7_ssi.rs`). The 41-column zigzag's cell-budget refusal is recorded on `the-chart-sweeps-first-order-box-reads-its-derivative-off-the-whole-span-cell`.

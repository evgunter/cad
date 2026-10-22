---
id: a-body-value-consumed-by-k-nodes-is-gated-at-rest-k-times
kind: issue
title: a body value consumed by k nodes is gated at rest k times, and ops that produced an AtRestBody drop the verdict into Arc<Body>
status: open
opened: 2026-10-22
priority: P3
cost: M
---


Filed by FLUX (unit F2) on the orchestrator's ruling that carrying verdicts
in editor-core's values is the evaluation model's own question, and not
FLUX's. The typed measurement door (Ev's decision on PR 4822) does not need
it: every consumer already gates the body it reads.

## Measured

`ValuePayload::Body(Arc<Body<T>>)` (`crates/editor-core/src/eval/mod.rs`)
holds no verdict. The producers (extrude, revolve, loft, the sweeps) do not
run tier 3, and the caller re-validates at rest (`sweep/src/extrude.rs`'s
door doc). Each consumer gates on its own:

- `eval/wire.rs:1199`, `finished_operand`: once per operand per
  consuming node.
- `eval/wire.rs:3306`: each instance member.
- `checks.rs:1623`: the connectedness check, per root subject.
- `product.rs:1093`: the gathered aggregate (a new body, so not a repeat).
- `checks/at_rest.rs:556` and `:925`.

Booleans, split, shell and blends already produce an `AtRestBody`, and
`wire.rs:2550` (`SplitPart::Body(b) => SplitSide::Body(Arc::new(b.into_body()))`)
and the other op arms drop it into `Arc<Body>`. Their consumer then gates
the same body again.

**Cost on the editor-core corpus** (29 documents, `corpus::documents()`;
`evaluate::<f64>` plus `run_checks` at the default config; release build;
ε = 1e-9; a temporary, uncommitted timer at the three consumer sites):

- every consumer gate together: 4.70 s of 46.21 s, **10.2%**. This is the
  ceiling on what carrying verdicts could save, if every gated body had
  come out of a gating op;
- re-gating the very same `Arc<Body>` a consumer already gated: 0.04 s,
  **0.1%**;
- the heaviest documents are `die` (2.58 s of 12.45 s, 20.7%: 106 gates
  over 85 distinct bodies) and `die_composed_tour` (1.79 s of 29.46 s,
  6.1%: 74 gates over 51).

So the real target is the gating op's dropped verdict (up to 10%), not
repeated consumers (0.1%).

## Options

- **(A) Recommended: a memoized verdict per value.** The payload holds the
  `Body` with a once-cell for its `AtRestBody` or refusal.
  - An op that produced an `AtRestBody` (boolean, split, shell, blends)
    fills the cell at construction.
  - Every consumer reads it through one `finished(tol)` accessor, so tier
    3 runs at most once per value.
  - No node's error behaviour changes: refusals surface where they do
    today (`UnfinishedOperand`, `NotAtRest`).
  - Readers that want `&Body` get it through `Deref`.
  - The memo assumes one tol per evaluation, and asserts it.
- **(B) Gate eagerly at every producing node.** The payload becomes
  `Arc<AtRestBody>`. This changes behaviour: a node whose body tier 3
  refuses fails at its producer, and nodes no one consumes pay the gate.
- **(C) Split the payload into typed `Body` and `AtRestBody` arms by op.**
  The most churn, and consumers still gate the unfinished arm.

The owner decides, with designers if it reads this as a fork.

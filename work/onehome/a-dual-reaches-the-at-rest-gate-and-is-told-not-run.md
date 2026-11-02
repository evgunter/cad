---
id: a-dual-reaches-the-at-rest-gate-and-is-told-not-run
kind: issue
title: AtRestBody<T: Real> admits a dual, carrying AtRestOutcome::NotRunAtThisScalar, where a certification bound on T would make a dual at the gate fail to typecheck
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Raised by Ev on PR 6308 (2026-11-02): "the dual having a special error
thing also seems not quite right; isn't the point of the certification
type that it doesn't even type check to have a dual go there?"

`AtRestBody<T: Real>` (`crates/topo/src/validate.rs`) has two states.
`AtRestOutcome::Validated` is minted only by `AtRestBody::validate`,
which requires `T: Decide + CertifiedBounds + AtRestPolicy`. The other,
`AtRestOutcome::NotRunAtThisScalar`, is "what a dual's
`AtRestPolicy::gate_at_rest_kept` carries: nothing was checked". The
boolean reads it (`boolean/ops.rs`, the `kept.outcome() ==
NotRunAtThisScalar` arm), and so does `door_meter`. DESIGN `[scalars]`
says a dual may not certify (`CertifiedEnclosure` is not implemented
for `Dual`). So "not run" is a runtime state for something the type
system could exclude.

The question is how the boolean and the other consumers handle a dual
replay, which evaluates a body that has no finished-body gate, without
a finished-body type that admits "not checked". It belongs with
`the-at-rest-result-gate-and-its-typing-live-in-five-homes`, which
already owns the gate's typing. `[topology.tiers]` (PR 6308) states the
target: a scalar without certification rights cannot reach the gate.

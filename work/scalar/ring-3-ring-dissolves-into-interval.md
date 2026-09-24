---
id: ring-3-ring-dissolves-into-interval
kind: unit
title: RING-3: RingInterval dissolves into Interval — the ring's refusals become named Interval doors, Enclosure and crossing_bracket go, C9 and DL4 say what is true
status: dispatched
opened: 2026-09-24
branch: scalar/ring-3
---


## What

H5 ruling 1 cut (iii), its kernel half. `RingInterval` is deleted and
every site names `Interval`; the ring's refusal surface — `hull`'s
poison guard, `clamped_to` over the endpoints, `width`'s pad and NaN,
`mag`'s NaN, `contains`' poison-false, the NaI constructor — lands on
`Interval` as inherent methods with the ring's bodies, and the refusal
predicate is `!is_certified()` rather than a second `is_poison` that
would shadow `Real::is_poison`; `from_certified` keeps its `Def`/`Trv`
cap and moves to a sole `CertifiedBounds` bound, so
`CertifiedEnclosure::crossing_bracket` goes; `Enclosure`, its blanket
impl and its impls go; the endpoint census re-keys; C9 and DL4 are
re-written to what is true (Ev's text — the PR is `[ev]`).
Bit-preserving by construction: every endpoint bit and every refusal on
every corpus unchanged. Spec: `docs/RING-3-SPEC.md` (deleted at merge).
The feature drop, Q1 and CI are RING-4's (the split is the orchestrator's,
logged 2026-09-24).

**Review tier: DUAL** (Opus × 2, `docs/DUAL-REVIEW-PROTOCOL.md`) — an
architectural change with broad, hard-to-reverse reach: it retires the
certification substrate's separate type across 25 `src` files and
changes what three ratified clauses decide. Outside the suspended A/B
protocol (Ev, 2026-09-23): implementer, reviewers and fix pass run on
Opus, no arm, no ordinal, no blinding.

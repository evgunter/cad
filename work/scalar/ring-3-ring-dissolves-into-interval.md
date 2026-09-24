---
id: ring-3-ring-dissolves-into-interval
kind: unit
title: RING-3: RingInterval dissolves into Interval — the ring's refusals become named Interval doors, Enclosure and crossing_bracket go, C9 and DL4 say what is true
status: closed
opened: 2026-09-24
closed: 2026-09-24
branch: scalar/ring-3
pr: 3153
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

## Closed (2026-09-24) — PR 3153

Signed off by Ev on the decision section: C9 and DL4 each retire a
decision (the separate certification type, "not a `Real`", `is_poison()`
as the refusal's name, `Enclosure` as a second bracket trait), C2
re-worded naming-only. `RingInterval` and `ring_interval.rs` are gone;
the ring's refusal surface lives on `Interval` as inherent certification
doors with the ring's exact bodies (`poison`, `hull`, `clamped_to`,
`contains`, `width`, `mag`), the refusal is `!is_certified()` (231 sites,
enumerated by the compiler), `from_certified` takes a sole
`CertifiedBounds` and `crossing_bracket` went, `Enclosure` went. Bit
identity over every corpus and two reviewers' independent base-vs-head
programs (including refusals). Dual Opus review (DR row on this PR),
both APPROVE-WITH-FIXES with no MAJOR; the fix pass added the census
holders, the `nurbs.rs` white-box pin, a `hull` consumer row, and closed
a pre-existing hole: `ssi/certify.rs`'s chart tube now refuses a refused
span hull instead of certifying (no corpus row moved). Main's own red
on `geom`'s f5 elevation fuzz was fixed in the row (tolerance scaled by
the closest knot gap; kernel conditioning filed on NURBS). Landed after
RING-4 (#3154), merged in at `a0cef634a9`.

**Follow-up (Ev, 2026-09-24, in chat): RING-5** — the certification
doors move off `Interval`'s inherent surface into an extension trait in
`geom-core`, imported by name in certification files, with a gate
(allowlisted importers; no `Real` + `is_poison` there) and one `hull`
name per meaning; it absorbs
`work/scalar/certification-value-hygiene-has-no-gate.md`. Ev chose
this over a thin view type (`Certified(Interval)`, compiler-enforced but
a second type again).

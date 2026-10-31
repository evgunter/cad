---
id: a-union-and-the-pair-group-flush-edges-by-line-in-two-helpers
kind: issue
title: The union's Flush::sets_per_line and the pair's least_per_line group flush edges by line twice, and least_per_line's 'one home' doc is now false
status: closed
opened: 2026-10-20
priority: P3
cost: E
closed: 2026-10-26
---

Filed by WIRE's `wire-mintfirst` lane (branch
`wire/union-reads-not-mint-order`, PR 5488), from that PR's review.

## What

`emit_union::Flush::sets_per_line` groups a union's member edges by the
line they lie on (each group named `Merged` of its edges).
`emit_topo::least_per_line` does the same grouping for the pair boolean's
`flush_lines`, but keeps only the least of each group. The two loops
compare carriers the same way (`EdgeCarrier::runs_with` through
`ON_MEMBER_EDGE`). `least_per_line`'s doc still says it is "the one
home of the collapse the pair boolean (`flush_lines`) and the union
(`emit_union::Flush::meeting`) both read", which is no longer true: the
union does not call it.

`emit_topo.rs` was held by another lane (`wire-secface`) when PR 5488
landed, so the PR did not touch it.

## What it needs

One grouping helper in `emit_topo.rs` that returns the groups, which
`least_per_line` (or the pair's set rule, if
`a-pair-booleans-seam-vertex-cites-the-least-of-several-flush-operand-edges`
lands first) and `sets_per_line` both read; the doc corrected.

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).

Closed by PR 5778 (2026-10-26): one `emit_topo::sets_per_line`, and the union's adapter is dropped.

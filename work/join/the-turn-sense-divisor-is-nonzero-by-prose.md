---
id: the-turn-sense-divisor-is-nonzero-by-prose
kind: issue
title: Turn.sense is a bare T: the nonzero divisor turned_past and off_conic_slack read is held by call order, not by type
status: open
opened: 2026-10-09
priority: P3
cost: E
---


These items were left open by PR 4396's review and its fix-pass verification. The PR merged; none of them changes a result.

## What

- **The nonzero divisor is held by call order.** `Turn.sense` (`crates/topo/src/boolean/join.rs`, `Turn`) is a bare `T`. The fix pass added `Sense`, which carries the radial-germ refusal, but `turned_past` and `off_conic_slack` still take any `Turn`. The rule that the divisor is decided nonzero before ranking is held by call order and prose, not by type. Carrying `Sense` (or a decided-nonzero wrapper) in `Turn` would make it structural.
- **Repeated code.** The arm test opens both `nearer_along` and `nearer` (join.rs ~1298-1336), and five `let escalate` closures are defined separately.
- **An invariant that is argued, not scheduled.** The completed-null-faces row says "the reason is argued, not proven… the assertion is what holds the line". No row or schedule asks for the proof or for a witness search.

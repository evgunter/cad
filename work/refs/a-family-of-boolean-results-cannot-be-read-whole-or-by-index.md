---
id: a-family-of-boolean-results-cannot-be-read-whole-or-by-index
kind: issue
title: A family whose members are boolean results is ValuePayload::Family, so Union(xs) and xs[i] of it refuse WrongOperand
status: open
opened: 2026-11-06
priority: P1
cost: M
---


Found by a designer probe on fork `selects-indexed-member` (2026-11-06), run on a private clone at `84a1b2488`. Not yet reproduced as a committed test.

A union over a family whose members are a per-`k` `Subtract` (`Union { Family(per-k Subtract) }`) refuses with `WrongOperand { expected: "instances", found: "family" }`. The members of such a family are boolean results, so the family evaluates to `ValuePayload::Family`, not `Instances`. Two readers require an `Instances` payload:

- `wire_union`'s `whole` closure (`crates/editor-core/src/eval/wire.rs`);
- `reads_projected` (same file).

So `xs[i]` of the same family should refuse in the same way. A body seat that reads a family whole, or reads one member of it, should accept any family whose members are bodies, whatever operation produced each member.

The fix starts with a test: pin the probe's document as a test, then make both readers take the family payload. Also check `eval/members.rs` `family_value`, which already names `Body` and `Boolean(Body)` members alike.

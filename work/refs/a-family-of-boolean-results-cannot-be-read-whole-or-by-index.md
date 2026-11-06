---
id: a-family-of-boolean-results-cannot-be-read-whole-or-by-index
kind: issue
title: A family whose members are boolean results is ValuePayload::Family, so Union(xs) and xs[i] of it refuse WrongOperand
status: closed
closed: 2026-11-06
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

## Closed

**Reproduced, with a narrower cause than the probe gave.** If every member of a per-`k` subtract is a body, the family is already `Instances`: `family_value` names `Body` and `Boolean(Body)` members alike, and a union, an assemble and an indexed read all take it. The refusal comes only when a member is a boolean's **empty** result (`F[k] − F[1]`: member 1 is itself less itself). Such a family fell to `ValuePayload::Family`, the payload for families that are not of bodies, and it carried no name table. Every whole read then refused `WrongOperand { expected: "instances", found: "family" }`, and so did an indexed read of a *body* member: `cut[0]` refused because its sibling `cut[1]` was empty.

The fix makes such a family a family of bodies with an absent member.

- `family_value` names its bodies, each at its member's place. An absent member names nothing and keeps its index; that is the empty split half's rule for naming.
- `NodeValue::member_bodies` is the one door through which every evaluator reader of a family of bodies reads, whichever payload holds the family.
- `ValuePayload::member_body` is the one rule for "is this one body". The editor-core ladder copies route through it.
- A whole read that meets an absent member refuses `EmptyMember { input, at }`, naming the family and the member. That follows `[topology.conventions]` and `[loud-failure]`.
- An indexed read of an absent member is the empty value. A placement keeps it, which is `wire_place`'s rule for an empty boolean, and a subtract refuses it `EmptyOperand` at either seat.

Pinned by `g_family_of_booleans`, two tests. The empty-member test reds on the pre-fix tree with the probe's `WrongOperand`; it also asserts each name's body place. The every-member-a-body test passes on both trees and guards the case the probe suspected.

**The unification is scheduled:** `work/refs/a-family-of-bodies-has-two-payloads.md` (P1, H). It weighs one representation and holds the readers that still see only the payload.

**Rows filed for those readers:**

- `work/wirerest/a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators.md`: the product gather, the at-rest checks, `output_body` and its callers.
- `work/refs/a-selection-of-a-member-of-a-family-with-an-empty-member-refuses.md`: `read_named_body`, on PR 6555's ground.
- `work/bind/python-value-bodies-drops-a-family-with-an-empty-member.md`: Python's `Value.bodies()`.
- `work/refs/editor-core-cites-a-spec-d3-no-document-numbers.md`: the stale citation chore.

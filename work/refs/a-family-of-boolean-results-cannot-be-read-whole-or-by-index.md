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

- `family_value` names the family's bodies, each at its member's place. An absent member names nothing and keeps its index, as an empty split half does. `name_family` now takes `Option` bodies.
- `NodeValue::member_bodies` is the one door every reader of a family of bodies uses, for `Instances` and for the absent-member `Family` alike.
- The evaluator's readers go through that door: the union's and intersect's whole read (`wire_combine`), the assemble's (`wire_assemble`), the indexed read at every body seat (`reads_projected`) and the debug `own_bodies` check.
- A whole read that meets an absent member refuses `EmptyOperand` naming the family (D3: nothing ∅-absorbing).
- An indexed read of an absent member is the empty value, so a placement keeps it and a subtract refuses it `EmptyOperand`.

Pinned by `g_family_of_booleans`, two tests. The empty-member test reds on the pre-fix tree with the probe's `WrongOperand`. The every-member-a-body test passes on both, and guards the case the probe suspected.

The readers outside the evaluator that still read such a family as no body are filed:

- `work/wirerest/a-family-with-an-empty-member-reads-as-no-body-at-the-product-and-the-interrogators.md`: the product gather, the at-rest checks and `output_body`.
- `work/refs/a-selection-of-a-member-of-a-family-with-an-empty-member-refuses.md`: `read_named_body`, on PR 6555's ground.
- `work/pydoor/python-value-bodies-drops-a-family-with-an-empty-member.md`: Python's `Value.bodies()`.

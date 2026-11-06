---
id: a-selection-of-a-member-of-a-family-with-an-empty-member-refuses
kind: issue
title: A selection naming a body member of a family with an empty member refuses at read_named_body
status: open
opened: 2026-11-06
priority: P2
cost: E
---



Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; not pinned by a test.

A family whose members are each a body or a boolean's empty result evaluates to `ValuePayload::Family`, and it carries the family's names over its bodies (`eval/members.rs` `family_value`). Suppose a selection names a face of one body member, `Member { (k = 0), of }`, so it reads the family whole (`repetition::names_members`). It resolves against that table. It then reads its body through `eval/wire.rs` `read_named_body`, which goes through `placeable_operand`, and that takes `Instances` alone, so it refuses `WrongOperand { expected: "body", found: "family" }`. Before the fix the same selection refused earlier, because no table named the member at all.

`read_named_body` should read `NodeValue::member_bodies`: the member's body at its place, and `EmptyOperand` for an absent member. This is the selection path, which fork `selects-indexed-member` (PR 6555, `Select.body` becoming a `BodyRead`) is rewriting, so build it on that fork's result.

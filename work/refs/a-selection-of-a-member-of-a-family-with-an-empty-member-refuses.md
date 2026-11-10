---
id: a-selection-of-a-member-of-a-family-with-an-empty-member-refuses
kind: issue
title: A selection naming a body member of a family with an empty member refuses at read_named_body
status: closed
opened: 2026-11-06
priority: P2
cost: E
closed: 2026-11-10
pr: 6555
---



Found by the REFS lane that closed `a-family-of-boolean-results-cannot-be-read-whole-or-by-index` (2026-11-06), from reading; not pinned by a test.

A family whose members are each a body or a boolean's empty result evaluates to `ValuePayload::Family`, and it carries the family's names over its bodies (`eval/members.rs` `family_value`). Suppose a selection names a face of one body member, `Member { (k = 0), of }`, so it reads the family whole (`repetition::names_members`). It resolves against that table. It then reads its body through `eval/wire.rs` `read_named_body`, which goes through `placeable_operand`, and that takes `Instances` alone, so it refuses `WrongOperand { expected: "body", found: "family" }`. Before the fix the same selection refused earlier, because no table named the member at all.

`read_named_body` should read `NodeValue::member_bodies`: the member's body at its place, and `EmptyOperand` for an absent member. This is the selection path, which fork `selects-indexed-member` (PR 6555, `Select.body` becoming a `BodyRead`) is rewriting, so build it on that fork's result.

**Status after `a-family-of-bodies-has-two-payloads` (2026-11-06).** The unification removes the cause. Such a family is now `Instances`, `None` at the empty member, so `placeable_operand` takes it and `read_named_body` reads the member's body at its place. At an empty member's place it refuses `EmptyOperand`. A name cannot reach that place in practice, because the family names nothing of its empty member. What is still open is the pin: a test of a `Member`-spelled selection over such a family. That selection path is the one fork `selects-indexed-member` (PR 6555) decides, so the pin belongs with it.

## Closed

Pinned in its new form by PR 6555 (`crates/editor-core/tests/select_member_read.rs`, `a_selection_of_an_empty_member_refuses_empty_operand`). The `Member`-spelled selection this row described, a selection over the family whole whose names carry the member, no longer exists: Ev's ruling on PR 6555 deleted it with every producer and consumer, `repetition::names_members` among them. A selection of one member reads it by index, `Select { body: xs[i], names }`. At a member that is a boolean's empty result it refuses `EmptyOperand` naming the family (`eval/wire.rs` `selection_table`, before any name resolves), as a seat reading one empty result refuses. The test reads `F[k] − F[1]`'s empty member 1 by a fillet of `cut[1]`.

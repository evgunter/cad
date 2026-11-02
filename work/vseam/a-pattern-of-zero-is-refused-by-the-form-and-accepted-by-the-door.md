---
id: a-pattern-of-zero-is-refused-by-the-form-and-accepted-by-the-door
kind: issue
title: A pattern of zero is refused by the create form and accepted by the session door, on a cited refusal the kernel does not raise
status: closed
opened: 2026-10-31
priority: P3
cost: E
refs: [6128]
closed: 2026-11-02
pr: 6286
---

Found by PR 6128's review (NOTE-4): `SessionOp::AddPattern { count: 0 }`
is accepted and labelled at the session door.

## What

- **The form refuses 0.** `crates/viewer/src/pane/create.rs`
  `MIN_PATTERN_COUNT` is `1`. Its doc says *"A pattern of zero instances
  refuses at evaluation (`CountOutOfRange`, typed, on the node's own
  badge)"*.
- **The door accepts 0.** `DocSession::add_pattern`
  (`crates/viewer/src/session.rs`) declares `VarDecl::Index { count:
  Formula::count(count) }` and inserts one placement that reads the
  index. Neither the session nor `lower_decl`'s `IndexFault`
  (`crates/editor-core/src/edit.rs`) refuses a count of 0.
- **The kernel admits 0.** `eval::members::tuples`
  (`crates/editor-core/src/eval/members.rs`) admits
  `0..=u32::MAX` and says *"A count of zero gives no value"*.
  `CountOutOfRange` belongs to `need_count`
  (`crates/editor-core/src/eval/wire.rs`), which reads a structural
  Count *slot*. The index-driven pattern has no such slot.

So the form's cited refusal is not the one a zero pattern meets. The
form and the door also disagree on whether a zero pattern can be
authored at all. The kernel allows an empty family, so this is not a
kernel defect.

## Wanted

Decide which way the op goes:
- refuse a zero count at the session door, in the door's own words, and
  keep the form's floor; or
- admit a zero pattern as an empty family, drop the form's floor, and
  re-word its doc.

Either way, the form's doc should cite the refusal that actually
applies. What a fused pattern of an empty family evaluates to (the
`Union` over `Bodies::Family` of no members) is part of that decision,
and nobody has checked it here.

## Closed — 2026-11-02

PR 6286. The door's side: the form's floor `MIN_PATTERN_COUNT` is now
0, as `combine_ops::a_zero_count_is_a_family_of_no_member_and_a_negative_one_refuses`
and `SessionOp::AddPattern`'s doc already argued (the door judges no
count, and the index's count can be written to anything afterwards).
Its doc cites `IndexCountOutOfRange` for a count below zero. At 0 the
unfused pattern is `Instances([])` and the fused one
`Boolean(Empty)`; at -1 the family fails `IndexCountOutOfRange` and the
fused union has no value. `pane::create::tests::the_pattern_forms_floor_is_the_least_count_that_evaluates`
pins it through both doors.

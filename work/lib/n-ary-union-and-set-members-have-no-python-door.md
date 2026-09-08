---
id: n-ary-union-and-set-members-have-no-python-door
kind: issue
title: The n-ary union node and SetMembers have no Python door
status: open
opened: 2026-09-08
---


Found by LIB-B-MEASURES's node-arm sweep: with `Node.measure` and
`Node.assertion` bound, the kernel's `Node` enum has 23 variants
(`crates/editor-core/src/node.rs`) and `pncad.pyi` declares a
constructor for 21 of them. Two have none:

- **`Node::Union`** (`crates/editor-core/src/node.rs:1626`) — the
  N-ARY fold over a member list, with the same optional `declare`
  slot `Node::Boolean` carries. `Node.boolean` binds the BINARY
  operation and `Node.placed_union` / `placed_union_at` bind the
  group boolean over one prototype's placements; neither is this
  node, whose whole point is that the member LIST is data (D9: the
  fold order is the list's).
- **`Node::Sweep`** — already accounted for. `wire_sweep` refuses
  unconditionally (U4/LQ3, SWEEP_FRONTIER), so the census keeps
  `sweep_body` as `gap: G2 sweep` and binding a door that cannot
  succeed would move nothing. Named here only so the sweep's hit list
  is complete; it is not this item's subject.

**And the edit that goes with it**: `DocEdit::SetMembers`
(`crates/editor-core/src/edit.rs`) — the rewrite of a list input's
membership, with its own three refusals (`set_members_on_non_list`,
`too_few_members`, `duplicate_input`) already tagged in
`crates/pncad-py/src/tags.rs` and already in `pncad.pyi`'s
`EditError` prose. `pncad.pyi`'s `DocEdit` declares ten static
constructors and this is not among them, so those three tags are
words no Python caller can provoke through the edit they belong to.

`crates/pncad-py/src/tags.rs`'s own comment at the list-input arms
already says where this lands — "the Python SURFACE for `Node.union`
and `SetMembers` is LIB's build" — so the decision was made and the
build was never scheduled. This file is that schedule.

**Why the binding census does not see it.** `Node` and `DocEdit` are
both top-level names in `pncad.pyi`, so rule 1 accounts each WHOLE:
a missing ARM behind a bound name is invisible to
`test_binding_census.py` in both directions. The roster that does see
it is `test_north_star.py::test_the_bound_vocabulary_is_exactly_this`,
which lists the two constructor families literally — and it is a
positive list, so it says what IS bound and never what is not.

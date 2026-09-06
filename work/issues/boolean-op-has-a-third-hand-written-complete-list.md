---
id: boolean-op-has-a-third-hand-written-complete-list
kind: issue
title: BooleanOp has a third hand-written complete list, in a different order, in editor-core
status: open
opened: 2026-09-06
---


Found by the style review of PR 2046 (`view/const-all`) while checking
that PR's filed item
`work/view/hand-maintained-mirrors-of-a-kernel-enum-are-unforced.md`.
Filed here rather than on VIEW's slate because the site is in
`crates/editor-core` and no program obviously owns it.

## The finding

`BooleanOp` (`crates/topo/src/boolean/mod.rs:138`, three variants) is
listed completely, by hand, in at least three places, and two of them
are in a different order from each other:

- `crates/editor-core/src/persist/kernel_wire/boolean_op.rs:35` —
  `const ALL: [BooleanOp; 3] = [Union, Intersect, Subtract]`, the table
  `untag` searches and the refusal message quotes.
- `crates/viewer/src/forms.rs:44` — `BOOLEAN_OPS: [(BooleanOp, &str);
  3] = [Union, Subtract, Intersect]`, the boolean form's buttons.
- `crates/editor-core/src/persist/kernel_wire/boolean_op.rs:42` —
  `tag`, an exhaustive match, which IS compiler-forced and is the one
  that cannot go stale.

Neither array is forced: a fourth operation in `topo` breaks `tag`
(good) but leaves both arrays three long.

## Why it matters to the viewer's item

`hand-maintained-mirrors-of-a-kernel-enum-are-unforced.md`'s first
"answer on the table" is to have `topo` publish a `BooleanOp::ALL` and
have the viewer map over it, costed there as "a public list on a kernel
type for a chrome's benefit". Two things that item does not say:

- the cost has already been paid once for a neighbouring kernel
  vocabulary — `crates/topo/src/contact.rs:67` publishes
  `ContactClass::ALL` and `crates/viewer/src/matetool.rs:132` maps over
  it, which is exactly the shape being costed;
- a published `BooleanOp::ALL` would retire the `editor-core` copy
  above as well, so the benefit is two sites and not one.

## The neighbours, unswept

`crates/topo/src/query.rs:141` and `:227`,
`crates/topo/src/param_source.rs:146`,
`crates/topo/src/contact.rs:67` (on a `#[non_exhaustive]` enum),
`crates/editor-core/src/checks.rs:98`, `node.rs:402`, `node.rs:1812`
and `names/role.rs:198` are all hand-written `const ALL` arrays of a
kernel enum's variants. Whether any is stale was not checked here.

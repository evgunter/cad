---
id: a-one-body-node-reading-two-members-of-one-family-reaches-an-unreachable
kind: issue
title: A one-body node reading two members of one family reaches an unreachable!: mirror(xs[k], plane(xs[i].f)) projects both into one map
status: open
opened: 2026-11-24
priority: P1
cost: M
---


Found by the member-read-key lane (PR 7512) re-running a review probe
after merging `main` at `f2815a23`. Executed: the panic below is the
probe's.

`xs` a family a binder `{k in …}` yields; `reader = mirror(xs[k],
plane(xs[(k + 1) mod 2].f))`, `f` a face of a member. Evaluating
`reader` panics:

```
crates/editor-core/src/eval/wire.rs, with_replaced:
internal error: entered unreachable code: a node read two bodies of
RecipeNodeId(…) into one map: project it read by read
```

`wire::run_op` projects a one-body node's reads all at once
(`reads_projected` over `body_rows`, its `per_operand` list names the
nodes projected read by read, and a mirror is not one), and
`body_rows` hands it the mirror's body and, for a pose read directly
off a cell, that cell: two member reads of `xs` at two members, into
one map keyed by the operation. `with_replaced` announces that as a
kernel bug, and a document reaches it. Before a yield was a family to
every reader, the body seat's read was plain (the member that stands,
no projection), so only the pose's read was projected.

DESIGN `[failure.taxonomy]`: reachable by input, so a typed refusal or
a build, not row 4.

The probe that found it now mirrors a member of a second family
(`member_read_shapes::a_mirror_across_another_members_face`).

**Owed:** the mirror (and any other one-body node whose pose reads
another member of its body's family) projected read by read, or
refused typed at the door; a row.

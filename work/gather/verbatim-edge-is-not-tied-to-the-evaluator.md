---
id: verbatim-edge-is-not-tied-to-the-evaluator
kind: issue
title: verbatim_edge classifies the name pass-through set by node kind, and nothing ties it to what eval::wire actually passes through
status: review
opened: 2026-09-28
priority: P1
cost: M
branch: gather/verbatim-edge-evaluator-guard
refs: [three-walks-over-the-name-carrying-edges]
---

Recorded from PR 3321's review (S1/S5 residue), filed by the GATHER
name-edge-home lane.

**The finding.** `crates/editor-core/src/names/role.rs`, `verbatim_edge`
(~1447), is the recipe walks' one statement of N1's pass-through set: a
`Transform` is `Whole`, a `Part` is `Selected`, a `Split` is `Intact`, and
every other node kind is listed under `=> None`. The compiler holds that
match and both walks that read it (`product.rs` `placed_under_two_roots`,
`mate/member.rs` `walk`) together, but nothing holds it to the PRODUCER.
What actually passes names through is decided in
`crates/editor-core/src/eval/wire.rs`:

- `wire_transform` (~5074; the table is returned as
  `OpOut::plain(payload, Arc::clone(&value.name_table))`, ~5095);
- `wire_part` (~3219-3280);
- `wire_split` (~3126-3150, the intact entities).

A new node whose evaluation returns its input's table unchanged would
compile filed under `=> None` in `verbatim_edge`, and both walks would
silently stop at it. The product's two-roots check would then miss a
body placed under two roots through it, and the mate member walk would
refuse a reference that passes through it.

**A guard is possible** (a runtime one, which PR 3321's body first said
could not exist): over the evaluated corpus, for every node `id` with a
value, the node publishes a row whose `name.node != id` if and only if
`verbatim_edge(node).is_some()`. The "if" direction catches a new
pass-through filed under `None`; the "only if" direction catches an op
classified as verbatim whose evaluation re-mints. A `Split` publishes
both kinds of row, so the predicate is "some row carries a foreign
minter", not "every row does". Care is needed for ops that carry a
foreign `node` inside a role argument rather than as the head
(`FromTarget`, `InPart`): those rows' head `node` is the op itself, so
the head comparison is the right one.

**Cross-crate restatements** the guard would not reach, because
`verbatim_edge` is `pub(crate)`: `crates/viewer/src/marks.rs` (~514 and
~601, "a `Transform` contributes no role segment by construction") and
`crates/viewer/src/display.rs` (~433, `derives_from`'s doc). Each names
only `Transform`; whether they should read a public door onto this set
is part of the repair.

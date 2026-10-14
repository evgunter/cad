---
id: a-vanished-merged-name-is-diagnosed-a-cascade-through-its-retired-constituent
kind: issue
title: A vanished merged name is diagnosed Cascade through its first constituent, a retired name that was never a row
status: closed
closed: 2026-10-14
opened: 2026-10-07
priority: P3
cost: E
---

## What

When a merged name stops resolving, the resolver's cascade rung
(`resolve::resolve_impl`'s step 5, the `walk_names(name, Partners::Cascade,
…)` loop, `crates/editor-core/src/resolve/mod.rs:1476`) takes the first
embedded name that does not look up as the root cause. `walk_names`
(`resolve/mod.rs:2402`, through `embedded`) descends into a `Merged`
set's constituents, and a constituent is retired into the merge (N3,
`crates/editor-core/src/names/README.md`): it was never a row of the
table, before the edit or after it. So the diagnosis is
`Cascade { through: <first constituent> }`, which points at a name
that did not vanish, and the ladder below it (`prior.diagnose`, the
doc-diff rung that would name the edit) is never asked.

## Repro

A one-level union `U` of two extruded blocks `a`, `b`, overlapping
along x and declared flush on their tops (`Node::Union` with `declare`).
`U` publishes the top as `Merged{FromMember(a, top), FromMember(b,
top)}`. Hold that name, then edit `U` with `DocEdit::SetDeclare` removing
the declaration (or `DocEdit::SetMembers` dropping `b`): the merged name
vanishes, and resolving it reads `Diagnosis::Cascade { through:
FromMember(a, top) }`. The cause is the `SetDeclare`/`SetMembers` edit at
`U`, which the doc-diff rung reports when the cascade rung does not
pre-empt it.

## What a fix decides

Whether the cascade walk skips a `Merged` set's constituents (they are
not operand names the merged face derives through a row of), or reads
them only where the constituent was itself a row before the edit.
Found by the review of PR 4281.

## Closed

The cascade rung asks only of operand names, and a name that retired
into a merge (N3) is none:

- A `Merged` set's constituents are read through, never visited, by
  the cascade walk (`resolve::embedded`'s `Merged` arm under
  `Partners::Cascade`).
- A merge over a merged face lists that face's constituents re-wrapped
  (`emit_topo`'s merge-group loop through
  `merged::constituents_through_wrappers`, `emit_union`'s
  `parent_name`), so beneath a constituent sits a name the inner merge
  retired, held by a `From`. Its shape is a row's, so the table answers:
  a name a live merged row of its minting node covers
  (`resolve::retired_into_a_live_merge`) is retired, not vanished.
  The pair boolean's mint builds the same shape, and the rule reads the
  table, not the node kind, so it covers both.

`resolve::first_vanished` holds the walk and states the rule. The row's
second option (read a constituent only where it was a row before the
edit) needs a prior run, which a single-run resolve does not have.

- `tests/m4_pr4_resolve.rs`,
  `a_vanished_merged_name_is_diagnosed_at_the_edit_not_through_a_retired_constituent`:
  the sliding union's glued cap, then `SetMembers` dropping the B
  member. Before: `Cascade { through: From { read: <transform's read>,
  of: b0's Cap(End) } }` at the union. Now: `RecipeEdit {
  NodeChanged(union) }`.
- `a_vanished_merged_name_over_a_merged_operand_is_diagnosed_at_the_edit`:
  `U2 = U1 ∪ c` with c flush on U1's glued caps, then `SetMembers(U2,
  [U1])`. Before (the coverage rule disabled): `Cascade { through: [U1:
  From { a's Cap(End) }] }`. Now: `RecipeEdit { NodeChanged(U2) }` for
  every merged face of U2.
- `resolve::tests::the_cascade_walk_asks_through_a_merged_set_and_never_of_it`.

The sweep: every lookup of an embedded name in `crates/editor-core/src`
(`walk_names`, `embedded`, `for_each_inner` callers). The cascade rung is
the only one that asks the table about each name it walks.
`rebind_suggestions` matches constituents on purpose (it offers the
merge), and `derivation_nodes`/`derivation_reads` collect nodes and
reads, not rows.

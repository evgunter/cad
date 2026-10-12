---
id: a-vanished-merged-name-is-diagnosed-a-cascade-through-its-retired-constituent
kind: issue
title: A vanished merged name is diagnosed Cascade through its first constituent, a retired name that was never a row
status: open
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

---
id: a-nested-unions-merged-rows-are-not-flat-through-from-member
kind: issue
title: A union over a union publishes FromMember(u, Merged{…}) constituents: N3's flatness stops at FromMember
status: open
opened: 2026-10-06
priority: P2
cost: M
---



## What

N3 (`crates/editor-core/src/names/README.md`) says a merged name's
constituent set is FLAT: a merge of a merged face lists the faces, never
the merge. A union whose member is itself a union breaks that. The outer
union's member-keyed rows wrap the inner union's names whole, so a
merged face or a joined edge of the inner union is published inside the
outer one as `FromMember(u1, Merged{…})`, and an outer `Merged` set can
list it as one constituent.

`merged::constituents_through_wrappers`
(`crates/editor-core/src/names/merged.rs`) peels only `FromA`/`FromB`
wrappers, so neither the pair emitter's flattening nor `merged::edge_set`
sees through `FromMember`.

Found by the FUSE review of PR 4161 (step 4 of PR 3881's build). It was
already true for faces before that PR; the PR gives edges the same
shape, so they inherit it.

## What a fix decides

Whether flatness applies across a nested union's member boundary. If it
does, the outer set lists the inner set's constituents, re-wrapped as
`FromMember(u1, c)`, and `constituents_through_wrappers` learns to peel
`FromMember`. If it does not, N3's flatness sentence should say it stops
at a member boundary. Either way, a row should pin a nested union's
merged face and joined edge.

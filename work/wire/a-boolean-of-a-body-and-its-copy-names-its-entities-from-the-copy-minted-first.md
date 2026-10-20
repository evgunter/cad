---
id: a-boolean-of-a-body-and-its-copy-names-its-entities-from-the-copy-minted-first
kind: issue
title: A boolean of a body and its exact copy names each entity of the result from the copy minted first
status: closed
priority: P2
cost: M
opened: 2026-10-20
closed: 2026-10-20
pr: 5488
branch: wire/union-reads-not-mint-order
---

Filed by WIRE's `wire-nameinv` lane (branch `wire/name-invariance-checks`),
found by `crates/editor-core/tests/name_invariance.rs`
(`which_twin_was_authored_first_moves_no_name`,
`an_authoring_history_moves_no_name`).

## What

`unit_block` (`[1,2] × [0.5,1.5] × [0.5,1.5]`) authored twice, as two
separate nodes `one` and `copy` with identical content. Listed `[one, copy]`
in both documents, only the authoring order differs:

- `Union [one, copy]`: every face, edge and vertex is named
  `From { read: <one's read>, of: <one's name> }` when `one` is minted first,
  and `From` `copy` when `copy` is (40 of the table's names move).
- `Intersect [one, copy]`: the same, 40 names.
- `plate − Union [one, copy]`: the hole's 32 names move the same way, through
  the tool union.

The body is the same in both orders. The two copies' names differ only by
their node ids, so the emitter picks between two candidates by id order,
which is mint order: the author did not write it, and a recipe replayed
under another history names the result otherwise. N4 says a candidate number
is storage order, not something the author wrote; the same holds for a
mint ordinal.

## What it needs

A rule for an entity that two members hold coincidently: a name that
cites both members' entities, as `Merged` does for coincident faces (N3),
and so is keyed by the reads it cites rather than by either one's id.
Picking one by the member list's order is not open: N0 (NS1, NS4; PR
5455) rules out a name that reads the order members are listed in. The
remedy belongs with the naming-strategy fork
(`naming-has-no-single-strategy-and-each-fork-patches-one-entity`).
Unmeasured: which emitter arm picks; the union's coincident-face merge in
`crates/editor-core/src/names/emit_union.rs` is the likely site.

When it is fixed, the admission entries citing this row in
`name_invariance.rs` turn red and come out.

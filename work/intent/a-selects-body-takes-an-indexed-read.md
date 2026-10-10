---
id: a-selects-body-takes-an-indexed-read
kind: issue
title: A selection's body is a plain read, so a member read by index (xs[i]) cannot be selected from, and Rebind { body } would reach every member's selections
status: open
opened: 2026-10-10
---


FORK-DM4 unit 1 (PR 4527) builds the indexed read `xs[i]` at every body seat
(`crate::BodyRead { read, at }`, index slots `SlotId::Index { seat, k }`).
INTENT stage 2 E (#4523) landed first and made a blend's, a shell's and a face
frame's body the selection's: `VarDef::Select(var::Select { body: VarId, names })`,
whose `body` is lowered as a Body seat (`edit.rs`, `mint_selection`). By the
two implementers' agreement the merge kept `Select.body` a bare `VarId`, so a
member of a family read by index cannot be selected from: fillet `xs[2]`'s edges
has no spelling.

Building it: give `var::Select.body` the `BodyRead` form, its index slots the
select definition's own (a definition, not a node), resolve the select against
the projected member (`eval/wire.rs`'s `reads_projected` / `instance_of`), and
keep the carried names keyed by the family read (DM4).

One hazard to pin with it, from E's implementer: `DocEdit::Rebind { body }`
rewrites every selection of one body read. Two selections of different members
of one family (`xs[0]`, `xs[1]`) share that body read, so a `Rebind { body: xs }`
would reach both; it must address the indexed read (read and indices), not the
read alone.

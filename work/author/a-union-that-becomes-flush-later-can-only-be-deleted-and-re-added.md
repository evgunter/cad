---
id: a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added
kind: issue
title: A boolean that becomes flush after it is committed has no recourse but cascade delete and re-add
status: open
opened: 2026-09-30
priority: P2
cost: M
design: true
refs: [addboolean-doc-names-a-vocabulary-that-does-not-exist, no-docedit-splices-a-deleted-node]
---


Raised by AUTH-9's correctness review (PR #3543).

## What

AUTH-9 gives a flush union a recourse at the one moment the boolean
door sees it: before the union is committed. The door refuses, offers
the declaration, and commits the `Declare` and the union together. A
union that becomes flush *after* it is committed gets none of that:

- **an edit makes it flush**, e.g. a `SetSlot` or parameter edit that
  moves the boss down onto the block, or changes a height so two caps
  meet; or
- **a poisoning ancestor is repaired**. A boolean poisoned by an
  upstream failure commits (`RefusedBoolean::read` answers `None` for
  it), and when the upstream node is fixed, the boolean's own contact
  refusal appears on its row.

Either way the committed union's row shows `UndeclaredContact`, whose
recourse is to declare the pair on its `declare` input. No edit
attaches a declaration to a live node, so the author's only path is
to cascade-delete the union and everything downstream of it and author
it again through the tool.

## Why it is a design question

This is DM6's own reopening trigger. `crates/editor-core/REFERENCES.md`,
"DM6 — Splice is not added", keeps
`work/edit/no-docedit-splices-a-deleted-node` open as the record of
"the one trigger that would reopen the question: a chain that a flat
operator cannot flatten and that a user needs to edit from the middle".
A union in the middle of a chain that an upstream edit makes flush is
that shape. The candidate answers are the ones EDIT weighed on the
`[ev]` PR behind `a-declared-union-has-no-one-pass-authoring-path`:
a narrow edit that attaches a declaration to a live `Boolean`/`Union`,
or a viewer seat that replaces the node through a splice. Both reopen
DM6, so this goes to EDIT and Ev with a designer weighing before
anything is built. `needs_ev` is not set yet.

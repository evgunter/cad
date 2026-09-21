---
id: stranded-appearance-is-bound-but-unreachable-from-python
kind: issue
title: stranded_appearance is bound and tagged in pncad-py and no Python door can provoke it
opened: 2026-09-16
status: open
refs: [2784, five-doc-edit-arms-have-no-python-door]
priority: P3
cost: D
---

(Found by the style review of PR 2784, which asked for a Python row
reading a `stranded_appearance` off a real edit. Measured: there is no
such edit to read it off.)

## The finding

PR 2784 added `Maintenance::StrandedAppearance { name }` and carried it
through the bindings mechanically — `maintenance_tag` answers
`"stranded_appearance"`, the `TAG_INVENTORY` carries the word, the
`name` getter answers for it while `node` answers `None`, `pncad.pyi`
documents the arm, and the census binds
`"Maintenance::StrandedAppearance": "Maintenance.variant"`.

**No Python row can put one in `Doc.last_maintenance`, because Python
cannot paint.** A `stranded_appearance` needs a key in the document's
appearance store, and the only doors that write that store are
`DocEdit::{SetAppearance, SetAppearanceMeta}`. Neither is bound, and
that is not an oversight: `crates/pncad/src/document.rs`'s curated list
leaves `Attr`, `AttrSet` and the appearance record types out, so those
arms have no payload a Python constructor can take. The census says
so at `DocEdit::SetAppearance` (`different-shape`), `py/doc.rs`'s
`DocEdit` doc says so under "What is NOT here", and
`work/lib/five-doc-edit-arms-have-no-python-door.md` closed on the
same sentence.

So the arm is reachable in the type system and unreachable at every
door: a Python caller can match the tag and read the name, and no
Python program can make one appear.

This is the shape the census already records one arm over —
*"there is no Python door that mints a `SetAppearanceMeta`, so no
Python row can provoke this arm"* — now true of a `Maintenance` arm
rather than an `EditError` one, which is further out: a caller reading
`last_maintenance` is not reading a refusal they provoked, they are
reading what an edit they made did.

`Maintenance::Strand` is NOT in this row. `Node.fillet` takes a
`selection` of name texts and `DocEdit.delete_node` is bound, so a
payload strand is authorable from Python end to end; whether a row
exercises it is a separate question from whether one can.

## What closing it looks like

Either the appearance doors gain a Python payload — which is the
façade's `Attr` question and not this row's to settle — or the census
records the reach measurement beside the bound variant, the way it
records `SetAppearanceMeta`'s, so the next reader is not left to infer
that a bound arm is an exercised one. The second is cheap and does not
pre-empt the first.

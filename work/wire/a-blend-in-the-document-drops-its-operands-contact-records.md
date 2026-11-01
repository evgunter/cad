---
id: a-blend-in-the-document-drops-its-operands-contact-records
kind: issue
title: editor-core's wire_blend hands a fillet or chamfer an empty Carry.contacts, so a blend of a body with contact records loses them in the document
status: open
opened: 2026-11-01
priority: P1
cost: M
---


Found by the sweep on branch `wire/a-split-carries-its-operands-contacts`, which
fixed the same drop at the split and the shell. Read from the code; no
row witnesses it yet.

## What

`wire_blend` (`crates/editor-core/src/eval/wire.rs`) builds its value
through `OpOut::plain`, so the fillet's or chamfer's body carries the
default empty `Carry.contacts`. The operand's records (a Boolean's
pinch records, or the touches a `⊔`'s census kept) are lost, and any
later at-rest check or lint reads the blended body as recordless.

The kernel carry exists, `sweep::blend::Blended::carry_contacts`, but it
is a method on `Blended`, and the verbs door (`verbs::Blend::run`,
`crates/verbs/src/run.rs`) moves only the body and the naming into
`VerbOut`. `Blended::lineage` reads only those two, so the carry needs
either a lineage the door hands across (`VerbOut` or the record) or a
lineage built from `BlendNaming` alone; the split and shell fixes
rebuilt their kernel result in the wire, which `Blended`'s other fields
(`shells`, `blend_faces`, `corner_faces`, `band_faces`) do not allow
honestly.

## Direction

Carry the operand's records as `wire_split` and `wire_shell` do: read
them as the operand's door kept them (`Carry::kept`), carry them
through the blend's lineage, refuse a carry refusal as
`NodeErrorKind::ContactCarry`, and hold them on the output citing the
operand's read. heldrecords' `a-finished-body-holds-its-contact-records-at-every-door`
retires every caller-side carry, this one included, when a finished
body holds its own records.

## Done when

A fillet of an edge clear of a pinch union's touch carries the union's
records onto its result, read by a reader of the fillet.

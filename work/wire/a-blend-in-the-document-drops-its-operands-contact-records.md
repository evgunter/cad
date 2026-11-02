---
id: a-blend-in-the-document-drops-its-operands-contact-records
kind: issue
title: editor-core's wire_blend hands a fillet or chamfer an empty Carry.contacts, so a blend of a body with contact records loses them in the document
status: closed
opened: 2026-11-01
closed: 2026-11-02
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

## Findings (branch `wire/a-blend-carries-its-operands-contacts`)

- The carry is built from the birth records alone: the lineage moved
  onto `sweep::blend::naming::BlendNaming::lineage`, with
  `BlendNaming::carry_contacts` beside it taking the blended body by
  reference; `Blended::lineage` and `Blended::carry_contacts` delegate.
  `VerbOut` is unchanged. `wire_blend` reads the operand's records as
  its door kept them (`Carry::kept`), carries them, refuses a carry
  refusal as `NodeErrorKind::ContactCarry`, and holds them citing the
  operand's read.
- Witness `crates/editor-core/tests/blend_carries_contacts.rs`: a fillet
  and a chamfer of the pinch plate's left bottom edge hold the union's
  two records, read by a one-member union of the blend (red on main:
  0 of 2).
- A blend bounded by a recorded cell, read in the document: the pinch's
  second block's top edges end on its corner that rests on the first
  block's edge (record 0's vertex); both blends refuse
  `FaceClearance` before any carry (bandrule's strip rule), so no record
  on a bounded cell reaches the carry from a document. The first
  block's top edge along `y` ends on the top of the recorded edge;
  the edge keeps its key and both records ride on the union's own cells.
- Sweep of body-from-body lowerings in `crates/editor-core/src/eval/`
  (`OpOut::plain` / `per_output` / `Carry::default()` / `NodeValue {`):
  every one carries or holds its operand's records now; the blend was
  the last.
- Review pass: a `⊔` census face/face record (A's top, B's bottom)
  carries through a chamfer of A's vertical edge and a fillet of A's top
  edge. A fillet of that vertical edge refuses `ContactCarry { Unread }`,
  where on main it built with no records. The arc it puts in A's top
  trim has no chart-region reading
  (`work/charttail/a-planar-face-with-a-conic-trim-has-no-region-door.md`,
  which now records this evidence). The refusal is loud and typed, and
  the witness pins it.

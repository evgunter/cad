---
id: a-split-in-the-document-drops-its-operands-contact-records
kind: issue
title: editor-core's wire_split hands each side an empty Carry.contacts, so a split of a body with contact records loses them in the document
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Filed by SPLITDOOR (2026-10-28). Both designers on SPLITDOOR's split-door-records question found it independently while reading the code. Neither built anything to confirm it.

## What

`crates/editor-core/src/eval/wire.rs` `wire_split` builds each side's value through `OpOut::per_output` / `plain`, with the default empty `Carry.contacts`. It never calls `topo::SplitResult::carry_contacts`; only tests do. So when the operand carries contact records (a `⊔` product, or a Boolean that recorded a pinch), its sides lose them. Any later at-rest check or lint reads the sides as recordless.

## Direction

There are two ways forward:
- **Interim:** carry the operand's records through `SplitResult::carry_contacts` in `wire_split`.
- **Pending design:** SPLITDOOR's design fork on the split door's contract may move records into the finished body (`AtRestBody`), which would make this drop unrepresentable.

Check that fork's outcome (`work/splitdoor/the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime.md`) before building more than the interim carry.

First, build a fixture: a Boolean pinch union, then split it in the document, then read its sides' records.

## Built: the interim carry (2026-11-01)

Branch `wire/a-split-carries-its-operands-contacts`.

- `wire_split` reads the operand's records as its door kept them
  (`Carry::kept`, so a `⊔`'s census touches come too) and carries them
  through `topo::SplitResult::carry_contacts`. Each side's records sit
  in the split value's `Carry::members` at the side's output body,
  citing the operand's read; a port read (`split_port`) and the
  product's gather (`product::sources_of`) read that side's.
- What `carry_contacts` decides: a record whose cells lie wholly on one
  side lands on that side only; a record on cells the cut divides lands
  on each side that holds a piece of both, and the vertices the cut
  mints on the two divided cells meet in a new vertex-pair record on
  each side; a record whose two cells the cut puts on different sides
  refuses `CarryRefusal::Parted`. The wire retires a parted record
  (`sides_carry`) rather than refuse the split: the two sides meet there
  as they meet across the whole cut, a touch between two bodies that
  neither side's records state. The plate pinch cut along `y = 1`
  (`name_invariance`'s "plate pinch at y = 1") parts both of its
  union's records and refused before the wire retired them. No check
  confirms a parted record's cells meet; a bookkeeping record the cut
  parts retires too (`topo`'s `a_record_the_cut_parts_is_refused`
  refuses it). Every other carry refusal is
  `NodeErrorKind::ContactCarry`.
- Witnesses (`crates/editor-core/tests/split_and_shell_carry_contacts.rs`),
  red on main with 0 records on every side: the pinch union cut at
  `x = 0.5` (3 records above, 0 below), cut at `z = 0.5` (3 each side),
  the product of the above port (3), and the `⊔` of the same blocks cut
  at `x = 0.5` (3 above); and the plate pinch cut along `y = 1`, whose
  sides build holding neither parted record.
- The sweep fixed `wire_shell` the same way (`topo::Shelled::carry_contacts`)
  and filed `a-blend-in-the-document-drops-its-operands-contact-records`
  for `wire_blend`.

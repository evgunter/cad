---
id: a-loft-station-slot-addresses-its-section-by-position
kind: issue
title: A loft station's slot and a section's operand slot address the section by its position
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the sweep of `reads/loft-section-ids` (the unit that gave every
loft section a minted `SectionId`, `work/refs/loft-slab-and-section-names-are-positional.md`).
A loft section's NAMES now key on its id, but two edit addresses still
count sections from the start:

- `SlotId::StationSlide { section: usize, step }` and
  `SlotId::StationSpin { section: usize, step }`
  (`crates/editor-core/src/node.rs`, written by `station_rows!` and read
  by `LoftSection::rows`): the slot of one move of a section's station.
- `OperandSlot::Section(usize)` (`crates/editor-core/src/operand.rs`):
  the operand slot of one section's profile read, used by `SetParam` at
  an operand and by `SetMembers` on a loft (`crates/editor-core/src/edit.rs`,
  the `Node::Loft` arm of the `SetMembers` door).

A slot address is held only as long as one edit, so a position is not
renamed under anyone the way a name was; but an edit authored against a
section list that a concurrent or replayed `SetLoftSections` reorders
lands on another section, and D5 says slots are named, not indexed. Done
when each addresses the section by its `SectionId` (the station's `step`
is a position within one authored station, which no other edit moves),
the Python slot words (`station_slide`, `station_spin`) carry the id, and
the refusal payloads that report a section by position
(`LoftFault::{EmptyStation, StationInFirstPlane}`) are judged — they
report the list the edit wrote, so a position may be the right word
there.

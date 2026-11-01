---
id: python-set-param-reaches-no-list-member
kind: issue
title: Python's set_param has no word for one list member or loft section, where Rust's SetParam writes one
status: open
opened: 2026-10-08
---


Raised by PR 4342's review (reviewer A, NOTE-2); filed rather than
built there, by the orchestrator's ruling.

**The asymmetry.** Rust's one slot door writes a single list entry:
`DocEdit::SetParam { slot: SlotId::Operand(OperandSlot::Member(i)), value:
SlotValue::Read(_) }` re-points member `i` of a union, and `Section(i)` a
loft's section. Python's `DocEdit.set_param(node, word, value)` addresses
a slot by its word, and the slot alphabet gives `member` and `section` no
reading (`crates/pncad-py/src/slot_word.rs`, the words an address is not
completed by; `crates/pncad-py/src/tags.rs`'s `slot_id_tag` arm). The
position rides only `set_members`, which rewrites the whole list. So one
door has two reaches: Rust re-points one member, Python rewrites the list.

**Shape of a fix.** Either `set_param` takes an index beside the word for
the two list families (the refusal payload already carries `index` for
these slots, `edit_payload.rs`'s `operand_index`), or the Rust door stops
offering a single-entry write and `SetMembers` is the list's one door.
The first keeps one door with one reach; the second removes a spelling.

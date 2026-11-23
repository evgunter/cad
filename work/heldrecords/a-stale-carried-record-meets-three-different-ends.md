---
id: a-stale-carried-record-meets-three-different-ends
kind: issue
title: A stale carried record meets three different ends: the gate refuses it, a Boolean that divides its cell drops it, a split that divides it refuses the carry
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found in review of PR 7124 (`embed/records`, 2026-11-22). A record its
door carried in that no longer has a touch behind it ends three ways,
by which door reads it and whether the op divides its cells:

- **The op keeps both cells.** The carry lands the record by lineage,
  unread, and the gate reads it and refuses the result
  (`ResultInvalid`, `StaleContactDeclaration`):
  `crates/sweep/tests/a_carried_record_no_reading_covers.rs`,
  `a_record_no_reading_covers_lands_by_lineage_through_an_op_that_keeps_its_cells`.
- **A Boolean divides one of its cells.** The carry reads each piece,
  finds none touching, and drops the record; the result ships without
  it and says nothing: the same file,
  `a_record_an_op_divides_is_read_and_found_stale`.
- **A split divides one of its cells.** The carry reads each piece and
  refuses (`SplitFinishError::Carry { Stale }`).

A finished operand cannot hold such a record (its own gate confirms
every record it is handed), so the three are reached only where a
caller hands records beside the operand (`boolean_op_with`,
`BooleanInputs`) or in a test fixture. One rule is owed: a record a
carry reads and finds stale either refuses at every door or is retired
at every door, and the Boolean's silent drop is the odd one.

**A possible fourth end** (PR 7124's fix pass, 2026-11-23; seen, not
chased, no row pins it). In
`crates/sweep/tests/a_carried_record_no_reading_covers.rs`,
`a_record_an_op_divides_is_read_and_found_stale`, with the operand
`plate ∪ rod` (`rested()`) holding the union's own records beside the
planted one rather than the planted one alone, the op that divides the
cell answered a carry refusal `Unread { record: 2, cells: [Edge, Face] }`
on one of the union's own records, not `Stale` on the planted one. The
fixture was changed to hold the planted record alone
(`topo::test_support::unconfirmed::holding`) and the row is green that
way. Owed here: whether `Unread` on a record whose cell the op divides
is a fourth end of the same stale record, or a true record the divided
carry has no reading for (a different hole, with its own row).

**A fourth end: a face that kept its key and was trimmed** (EMBED
orchestrator's ruling, 2026-11-22; PR 7124's fix pass). A cut that
carves a face away under a recorded touch without dividing it leaves
the face its key; the Boolean's carry reads only the records of cells
the op divided, so this one lands by lineage, unread, and the gate
refuses the result stale. The slow set makes 109 stale `VertexOnFace`
readings, all of them this, in the lying-pyramid scenes of
`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs` and
`a_vertex_read_again_classes_every_edge.rs`. The ruling: the gate keeps
refusing; the door is where it is fixed (a kept-key cell the op trimmed
is "divided" for that record, read by the carry and retired where it no
longer touches). Its own row:
`work/heldrecords/a-cut-that-trims-a-kept-face-leaves-a-record-on-it-stale.md`.

**A second landing site beside the carry** (PR 7124, 2026-11-23). In
the Boolean, `carry_rows` places every carried vertex-on-face row
again through `place_found` with `found: None`, which lands it by
lineage, unread, whatever `carried()` read of it
(`crates/topo/src/boolean/ops.rs`). So a rule for a record the carry
reads and finds stale has two places to hold: fixing the divided-cell
reading alone would not retire the record if the second placement
lands it again
(`work/contacthold/two-readers-of-a-vertex-on-a-faces-boundary-vertex-disagree.md`).

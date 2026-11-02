---
id: a-record-the-split-parts-has-no-consumption-rule
kind: issue
title: A contact record whose cells the split's plane parts has no consumption rule in topo's carry; the document drops it after a check instead
status: open
priority: P3
cost: M
opened: 2026-11-01
---


Found by the FULL review of PR 6268 (2026-11-01). topo's carry
(`crates/topo/README.md:807-826`; `CarryRefusal` docs at
`crates/topo/src/boolean/records.rs:46-47`, "refused, never dropped")
consumes a record only when one of its cells is removed, divided with its
remainder removed, or moved, and calls any other unlanded record stale.
A record whose two cells the split's one plane puts on different sides
(`landings`, `records.rs:855-857`) is none of those, so topo refuses it
`Parted`. Such a touch lies in the cut plane: the two sides meet there as
they meet along the rest of the cut, which no single side's records can
state. A legal split refuses on it (`name_invariance`'s plate pinch at
y = 1).

PR 6268 handles it as an interim in editor-core: `wire_split`'s
`sides_carry` drops a parted record only after checking its cells meet the
cut, and refuses `ContactCarry { Parted }` otherwise. The policy's proper
home is a consumption case in topo's carry plus a sentence in that README
paragraph. The paragraph is a design page, so the change is an `[ev]` PR,
weighed by a designer pair first. heldrecords'
`a-finished-body-holds-its-contact-records-at-every-door` moves the carry
into the split's door, which must then adopt or replace the policy.

## After PR 6268 (2026-11-01)

The interim landed in topo itself as an opt-in:
`SplitResult::carry_contacts_across` (`crates/topo/src/splitting/finish.rs`)
retires a parted record only when each cell has an image on the cut (a
plane face its side holds, `SplitNaming::plane_pieces`, or a cell bounding
one), and refuses `Parted` otherwise; `carry_contacts` still refuses every
parted record, so the README's carry paragraph stays true of it. Only
`wire_split` calls the `_across` form. The `[ev]` question is unchanged:
whether the README's carry paragraph gains this as a consumption case.

---
id: range-synthetic-name-mints-a-name
kind: issue
title: range.rs mints a variable name for the slot it widens, which VR2 says the kernel never does
status: closed
opened: 2026-10-04
closed: 2026-10-06
---

`range::synthetic_name` (`crates/editor-core/src/range.rs:598`) spells a
variable name, `query_certified_range_<node id>` (with a numeric suffix
until it is free), and the certified-range query declares a variable
under it to widen a literal slot (`range.rs:688`). VARIABLES-DESIGN VR2
says the kernel mints no name, and INTENT-VARS-1's §8 Q2 ruling refuses
an anonymous variable crossing a cut for that reason.

The derived document never leaves the query, so nothing a person
authored carries the name; the INTENT-VARS-1 spec let it stay internal.
It is still the one place the kernel writes a name, and it no longer
needs to: since PR 3 a reader reads an id, so the query can declare an
anonymous variable and point the slot at its id, with no name to
choose. One thing to check when doing it: an anonymous variable must be
read (walk 6, `AnonymousVarUnread`), which the rewritten slot is.

Raised by PR 3's dual review (r1 S8).

Closed by INTENT-LITERALS PR C. A slot holds a variable, so the query
widens the slot's own free variable, an anonymous one where the value
was typed. It declares nothing and spells no name: `synthetic_name` is
gone. A slot that reads a defined variable refuses
`RangeRefusal::SlotIsDefined`, because its free inputs are the fields
to range. The rows are `docm9_range::the_slot_widens_its_own_variable`
and its `SlotIsDefined` row.

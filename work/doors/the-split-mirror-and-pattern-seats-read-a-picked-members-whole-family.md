---
id: the-split-mirror-and-pattern-seats-read-a-picked-members-whole-family
kind: issue
title: The split, mirror and pattern seats read a viewport pick on a family's drawn member as the whole family, where the README says a pick on a drawn member seats family[k]
status: open
opened: 2026-11-09
priority: P2
cost: M
---



Found by the review of PR 6593 (REFS, `viewer-boolean-tool-cannot-pick-a-read`).

**What.** The README's *A new body lands unplaced* (`crates/viewer/README.md`) says "a pick on a drawn member seats `family[k]`". After PR 6593 that holds at the boolean seats only: `Seat::reads_member` (`crates/viewer/src/seats.rs`) is true for `Member`, `SubtractFrom` and `SubtractTool`, whose door holds `BodyRead<Formula>` operands (`session::BooleanSpec`). The other body seats read what the pick reaches whole, a family included, because their ops hold an `Operand`, which has no member:
- `SessionOp::AddSplit { target }` (`Seat::SplitTarget`)
- `SessionOp::AddMirror { body }` (`Seat::MirrorBody`)
- `SessionOp::AddPattern { input }` (`Seat::PatternBody`)

`combine_ops::every_seat_reads_a_member_exactly_where_its_door_takes_one` pins the split as it stands today.

**Should read members:** the split target, the mirror body and the pattern body. A pick on one drawn copy of a pattern means that copy. Each op's field becomes a `BodyRead<Formula>`, the door takes it, and `reads_member` turns true for the seat.

**Should not:** the part seat (`Seat::PartInstance` wants a family, which it projects one member of), and the clearance seats, which read a family over its members by design (`ClearanceOutcome::over_members`).

Member reads of a family something copies wait on `a-combine-of-a-world-copied-familys-member-leaves-the-copy-drawing-it` at every seat.

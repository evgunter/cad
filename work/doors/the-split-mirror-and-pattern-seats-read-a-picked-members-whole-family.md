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

## The pattern seat routes a family its door then refuses (FORMWORDS, 2026-11-22)

Evidence added, not a second row. The pattern seat (`Seat::PatternBody`,
wanting `NodeKindWanted::Body`) admits a pattern, and a transform of
one, because a family by repetition is a body at the gate
(`session::refuse::admits`): an operation reading it at a body slot is
evaluated once per member. `SessionOp::AddPattern` with such an input
then refuses at the edit door, `IndicesUnrelated` between the family's
index and the new pattern's fresh one — measured on
`combine_ops::box_and_pattern`'s pattern and on its transform.
`SessionOp::AddFusedPattern` over the same pattern refuses the same
way, `IndicesUnrelated` between the two indices. So today
the seat steers a pick the commit refuses, which `admits`' doc says one
answer for both exists to prevent. Reading a member at this seat, as
this row asks, removes the family from the input and the refusal with
it; `combine_ops::a_transform_of_a_pattern_seats_where_evaluation_takes_it`
skips this seat and cites this row.

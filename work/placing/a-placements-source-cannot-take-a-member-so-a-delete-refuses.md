---
id: a-placements-source-cannot-take-a-member-so-a-delete-refuses
kind: issue
title: A placement's source is written as a whole read, so a delete that would hand a copy one member of a family refuses Unspliced
status: open
opened: 2026-11-08
priority: P3
cost: M
---


Found by `placing-copy-reads` (PR 6581); the re-check of its fix pass asked for it to be scheduled.

**What.** A delete by continuation (`editor_core::continuation::splice`) can need to hand a reader one member of a family. Two cases: a cut of `xs[k]` whose tool goes, read by a copy; and a fillet over a family whose copy reads `F[k]`. A list operand takes the member (`SetMembers` keeps indices). But `SetParam` at a body seat writes `SlotValue::Read(Operand)`, which has no indices. So a placement's source, a selection's or a pose's body, and the product slot cannot be handed a member, and the delete refuses `SpliceFault::MemberAtSeat` (`Refusal::Unspliced`) and deletes nothing. The refusal is honest, but the person has to delete or re-point the copy by hand first.

**Done when** a body seat can be written an indexed read, by `SetParam` with a body read or a door of its own, and the splice hands a copy the member. Alternatively, a ruling says the refusal is the behaviour wanted. Either way `gesture_placements::a_continuation_is_a_body_read_whole` must follow.

---
id: split-result-gates-have-no-row-past-the-finished-operand
kind: issue
title: No row reaches the split's result gates past the finished-operand gate: ResultInvalid and Pieces lost their witnesses to it
status: open
opened: 2026-10-06
priority: P3
cost: E
---



Left by `split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), which makes the split's doors take
`AtRestBody`.

Two of the split's result-side refusals had their only public witnesses
in operands the door now refuses at the gate:

- `SplitFinishError::ResultInvalid` (`splitting/mod.rs`, `split_direct`'s
  tier-2 gate over each side). Its rows were
  `m3_pr3_split::an_uncut_side_that_is_not_a_closed_solid_refuses` (an
  `mvfs` seed, an empty loop) and `a_cut_side_carrying_a_strut_refuses`
  (a brick with a strut). Both operands are tier-2 scaffolding, so they
  now refuse `ScaffoldingOperand` at the door
  (`a_seed_operand_refuses_at_the_door`,
  `a_strut_bearing_operand_refuses_at_the_door`).
- `SplitError::Pieces` (`split`'s `sort_into_pieces` over each side).
  Its rows were `hollow_island::every_verb_refuses_a_body_whose_pieces_cannot_be_read`
  (`Crossing`) and `a_cube_inside_a_cube_under_one_solid_refuses_as_overlapping`
  (`Overlapping`). Both operands hold two outer shells under one solid,
  which the at-rest gate refuses (`SolidOuterShells`), so neither reaches
  the split at a certifying scalar. `pieces.rs`'s own unit rows still
  read the sort.

Unmeasured: whether any finished operand reaches either gate. If none
can, each is a kernel-defect backstop and its text should say so (the
`ResultInvalid` text already ends in the kernel-defect ending); if one
can, it wants a row built from that operand.

---
id: split-result-gates-have-no-row-past-the-finished-operand
kind: issue
title: No row reaches the split's ResultInvalid gate past the finished-operand gate, and Pieces is witnessed only at a dual
status: open
opened: 2026-10-06
priority: P3
cost: E
---



Left by `split-answers-an-inside-out-operand-with-two-inside-out-halves`
(branch `cleave/split-operand-gate`), which makes the split's doors take
`AtRestBody`.

Two of the split's result-side refusals had their only public witnesses
in operands the door now refuses at the gate.

- `SplitFinishError::ResultInvalid` (`splitting/mod.rs`, `split_direct`'s
  tier-2 gate over each side) has **no public witness at any scalar**.
  - Its rows were `m3_pr3_split::an_uncut_side_that_is_not_a_closed_solid_refuses`
    (an `mvfs` seed, an empty loop) and
    `a_cut_side_carrying_a_strut_refuses` (a brick with a strut).
  - Both operands are tier-2 scaffolding, so they now refuse
    `ScaffoldingOperand` at the door. At `f64` the at-rest gate refuses
    them. At a dual the door reads tier 2 itself
    (`a_seed_operand_refuses_at_the_door`,
    `a_strut_bearing_operand_refuses_at_the_door`,
    `a_strut_bearing_operand_refuses_at_every_door_at_a_dual`).
- `SplitError::Pieces` (`split`'s `sort_into_pieces` over each side) is
  **witnessed at a dual only** (measured in the review of PR 4084).
  - `hollow_island::every_verb_refuses_a_body_whose_pieces_cannot_be_read`
    (`Crossing`) and
    `a_cube_inside_a_cube_under_one_solid_refuses_as_overlapping`
    (`Overlapping`) hold two outer shells under one solid.
  - At `f64` the at-rest gate refuses that (`SolidOuterShells`). At a
    dual the door's gate reads tier 2 and orientation but not the piece
    count, so the split takes the body, sorts it, and refuses
    `Pieces(Crossing)` or `Pieces(Overlapping)` on a plane that misses
    it. Both rows assert that at `Dual64`.

Unmeasured:
- whether any operand reaches `ResultInvalid` past the door, at any
  scalar;
- whether any finished (`f64`) operand reaches `Pieces`.

A gate no operand can reach is a kernel-defect backstop, and its text
should say so (the `ResultInvalid` text already ends in the kernel-defect
ending). A gate one can reach wants a row built from that operand.

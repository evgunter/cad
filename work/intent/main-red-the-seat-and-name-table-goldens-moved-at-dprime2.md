---
id: main-red-the-seat-and-name-table-goldens-moved-at-dprime2
kind: issue
title: main red: five committed goldens (seat4 blend and boolean, seat7 sweep, seat8 split digests; m4_pr3 die name-table digest) moved at PR 5613 (D′2) and were not re-pinned
status: parked
opened: 2026-10-25
priority: P1
cost: E
rides_with: assemble-is-an-operator-and-the-product-is-a-slot
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
---



Found by FLUX (unit F2) while separating main's reds from PR 5668's, and
filed at the orchestrator's request.

## Measured

Five committed goldens are red on origin/main at `a4f59ff66d` (ε 1e-9):

- `seat4_verb_lowering::the_blend_documents_evaluate_to_their_committed_digests`
  (`seat4_verb_lowering.rs:298`): `die_fillet` and `die_chamfer` moved;
- `seat4_verb_lowering::the_boolean_documents_evaluate_to_their_committed_digests`
  (`:525`): `crossing_slots`, `heat_sink` and `kiss_carry` moved;
- `seat7_sweep_lowering::the_sweep_documents_evaluate_to_their_committed_digests`
  (`seat7_sweep_lowering.rs:469`): `die`, `corner_table` and `cut_cylinder`
  moved;
- `seat8_split_lowering::the_split_documents_evaluate_to_their_committed_digests`
  (`seat8_split_lowering.rs:407`): `cut_cylinder`, `part_select` and
  `kitchen_sink` moved;
- `m4_pr3_names_ci::die_name_tables_are_golden`
  (`m4_pr3_names_ci.rs:168`): the die name-table digest is
  `0x7fb3623b41caf7c9` against its pin.

Bisected on main's first-parent line: all five are green at
`5ae8cf0833^1` and red at `5ae8cf0833` (PR 5613,
`intent/dprime2-mates-read-geometry`). They are one row because they share
that one cause. `cut_cylinder` reads the same moved digest
(`0x90061e7f237a73d8`) in both seat7 and seat8.

## What closes it

Two cases:
- If PR 5613 meant to move these evaluations (its mates now read geometry
  and its values chart on the target space), the pins are re-baselined,
  and the PR that does it says what moved and why.
- If it did not, the body or the name table it changed is found and
  fixed.

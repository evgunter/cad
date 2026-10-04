# PR 3987 (the boolean door takes and returns AtRestBody): LAST fix pass

The delta review is on analysis/reach-delta/3987, frozen 07ca5a8d05. Its verdict is APPROVE-WITH-FIXES with no MAJOR (MINOR 2, NOTE 5), and F1 is closed by execution. Read it in full, along with its probes under probes/delta-3987/.

This is the LAST fix pass. The orchestrator verifies it and there is no further review round. The PR still merges after #3977 (F2).

## Rulings
1. **MINOR 1.** A finished operand does reach the maximal-faces gate through the public union. The reviewer built three such operands: the at-rest kink, the split top on one surface key, and the planted disc with its circle at rest.
   - Correct the false premise in `work/reach/the-operand-gates-curved-and-maximal-face-arms-have-no-finished-fixture.md` (`:17` and `:41`), in `offer_rows.rs:1517-1528`, and in `neighbours_across_a_closed_edge.rs:104`.
   - Restore public door rows for `NonMaximalFaces` and `CoplanarNeighbours`, built from the reviewer's three probes, so those arms are pinned through the public union again. Keep the test-support door only where a public row can't reach.
   - Retitle the item to what is still open: the pair-scoped curved gate and `curved_face_arm`. The swept cone and torus fixtures it proposes are the next probe.
2. **MINOR 2.** Pin F9's type so that reverting it turns something red. Two things:
   - Add a `compile_fail` doctest showing that `boolean_reduce` and `boolean_reduce_declared` refuse a plain `&Body<T>`.
   - Add the reviewer's `d_public_boolean_reduce_at_a_dual_refuses_the_inside_out_wedge` as a row. Mutant MC must turn both red.
3. **NOTE 1.** When #3977 lands, merge main and do the real resolution in `contact9_side_codes.rs` and `edit_refusal_recourse.rs`, keeping both sides' intent. Then show the 1e-12 contact9 row green. This is part of this pass and is required before merge.
4. **NOTE 3.** Correct F8's count to the measurement against the current main base (100 flags on both, identical), in the PR body and the closed item.
5. **NOTE 5.** Correct `ResultInvalid`'s doc (`mod.rs:2378-2382`) for duals.
6. **NOTE 2 and NOTE 4.** No change. State them in the PR body.

## Constraints
- Work under the intent-refactor hold, and don't widen the PR.
- Run the full battery at all three ε, plus Python and the census. Don't merge, and post no comments.
- Update the PR body's finding→change map. Report when you have pushed. If #3977 hasn't landed by then, report anyway and say so; the merge resolution follows when it lands.

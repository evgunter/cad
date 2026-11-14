---
id: a-labelled-creation-has-no-maintenance-witness
kind: issue
title: No creation's run reports maintenance any more, so the labelled-creation netting row compares empty rows
status: closed
opened: 2026-11-08
priority: P3
cost: E
branch: placing/p3-editor-batch
pr: 6919
closed: 2026-11-14
---


Found by `placing-copy-reads` (the 6353 build).

`node_labels::a_labelled_creation_nets_the_maintenance_its_own_run_reports` (`crates/viewer/tests/node_labels.rs`) held that a labelled creation's outcome carries its run's netted maintenance and nothing of the label's own. Its witness was a combine that deleted the second operand's world copy, stranding paint keyed on it. Since PR 6353's build no feature gesture writes or deletes a placement (`DocSession::combine_in_world` drops `⊔` seats instead), and no other creation's run reports a row: `DeleteNode` and `SetProgram` are the strand-reporting edits (`edit.rs`, `door`), and a creation applies neither. The row now compares two empty lists, which `labelled_as_one_undo` still checks, but no value could make the netting half go red.

Tried and refused: a re-target strand through `in_one_space` (the slot door refuses `SpaceMismatch` on the reader rather than reporting `StrandedSpace`), and paint under a `⊔` whose operand a fillet takes over (`SetParam` reports no appearance strand).

**Done when** the row has a creation whose run reports a maintenance row, or the netting half is dropped as subsumed by `labelled_as_one_undo`'s equality with the unlabelled run.

## Looked again (2026-11-08, PR 6581's fix pass)

The review suggested two witnesses. Neither serves:
- `RepointPlacements` is no creation: `Creation::of` takes only the ops `SessionOp::creates_a_node` answers, so it cannot be labelled.
- An operand write does report the payload names it strands (`edit.rs`, `DocEdit::SetParam`'s doc; `stranded_by_repoint`, `Took::Reach`). But every creation's `⊔`/product re-point hands the seat a result that reads the taken body. That body therefore stays upstream of the seat, and no name goes out of reach.

The kernel docs' disagreement the review saw (`edit.rs`'s `SetParam` doc against the outcome contract) is about reach strands. Those are real, but no creation reaches one.

## Done (branch `placing/p3-editor-batch`)

The netting half is dropped as subsumed, the second of the two ways
this row allows: no creation's run reports a maintenance row (both
attempts above). The row's other half stays, renamed
`node_labels::a_labelled_creation_that_takes_the_product_over_is_one_undo`.
It is the only sampled creation whose run re-points the product
(`InsertNode` then `SetProduct`). It holds that run and its label to one
undo, and the product to the fillet's, without the empty-list
assertion. `labelled_as_one_undo`'s doc now says its maintenance
equality is held, not witnessed.

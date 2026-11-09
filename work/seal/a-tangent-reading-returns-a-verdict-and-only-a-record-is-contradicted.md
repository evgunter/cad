---
id: a-tangent-reading-returns-a-verdict-and-only-a-record-is-contradicted
kind: unit
title: the tangent table and the Rest door return typed verdicts; only a record read at rest is contradicted; the glue door decides a tangency once
status: open
opened: 2026-11-09
priority: P1
cost: H
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 3 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09).

## What

"Contradicted" is vocabulary from before stage 4. Once declarations were gone, the glue door claims nothing, so a reading's definite "no" is a verdict. A contradiction exists only where a record (a coincidence or contact row a body carries) claims what the reading rules out.

- **The tangent table's verdict.** `tangent_table` returns `Touch(margin) | Ruled { row: TangentRow, by: Decided } | SidesMixed { at: [Decided; 2] } | InBand(Indeterminate) | NotCertifiable`. `TangentRow` is a closed enum: Opposed, Independent, Parallel, Overlap, Gap, Off.
  - The Rest door (`rest_pair_verdict`) returns its relation with `Option<Decided>`, and rows `SensesAligned` and `TwoCarriers`.
  - Display names come from the row (`row.predicate()`). `Decided` gains no predicate field.
- **Reorder `tangent_pass`:** the first-order parallel check runs before the sides comparison, so a sample that is not tangent ends as "not parallel".
  - `SidesMixed` then means two samples each read tangent, with opposite senses: a definite crossing between them.
  - It stays a typed verdict, not an `unreachable!`. The lane admits a Villarceau rim (`tangent_certificate_lane` tests surface kinds only, and `shared_rim` asks only whether two circles are one), where n₁·n₂ varies.
  - topo README notes that rulings and lines of curvature cannot reach it.
- **The census matches the row.** `same_solid_tangent_pair`'s dispatch on `diag.predicate` strings goes (D4 ¶1 (ii)).
  - With no record, `Overlap` stays `CuspEdgesMissing`.
  - Against a record, `Ruled` or `SidesMixed` is `ValidationError::ContactContradicted { record, verdict }`, with the defect ending (D4 ¶1 (iv), `[verdicts.consumers]`).
- **The glue door decides a tangency once.** `GluedPair` carries `OneCarrier { relation, margin } | Tangent(Tangency)` per class, and `verify_glued_contacts` records what the door certified instead of re-running `glued_tangency`.
  - These go: `BooleanError::ContactContradicted`, `tangency_contradicted`, `TangencyFailure::Contradicted`, `verify_tangency`'s conformal screen and `Contradiction::OneCarrier`. All of them are debris of deciding twice.
  - `tangency_reading`'s discarded `Contradicted` becomes the pair's verdict: not glued.
- topo README C4's `ContactRefusal::{Contradicted, …}` sentence is re-worded with the code (agent text, `c7571919b0`).

## Done when

No reading on this ground builds a `Contradicted` with an `Indeterminate`. The Boolean mints no contradiction. The census has no predicate-string dispatch, and the probe below has been run:

- **The probe:** two tori sharing a Villarceau circle, with the parallel check first. Does any pair of samples read tangent with opposite senses? Record the result beside `SidesMixed`.

---
id: stage5-contact-finding-and-the-unproven-row-report-one-contact-twice
kind: issue
title: Stage 5 C's Contact finding and the unproven-coincidence row would report one at-rest contact twice
status: open
opened: 2026-10-11
priority: P2
---


Found by the census-contact designers (fork-log row 113, PR 4666).
- Stage 5 C folds the at-rest gate into `CheckId::AtRest`, which reports a `Contact` finding for each contact between copies (`docs/INTENT-STAGE5-SPEC.md`).
- From stage 4 J on, the same contact is also a `CensusAtRest` row that the `unproven-coincidence` lint reports (`checks.rs`, `unproven_coincidence`).

Ev's 2026-10-11 ruling on PR 4644 says an unproven row is ONE finding. Reconcile it in the stage 5 spec before 5 C is sliced: the quieting rule quiets the row's finding, and `Contact` is not a second finding of the same contact.

---
id: a-defect-endings-culprit-has-two-choosers
kind: issue
title: geom-brep: the culprit a defect ending names is chosen in two places, recourse::defect_ending (by Reading, cannot say damaged file) and SizedDecision::words (by ReadAt)
status: closed
pr: 5454
opened: 2026-10-19
branch: encl/one-culprit-chooser
closed: 2026-10-20
---


(Filed by the ENCL implementer of PR 5349, from its review; evidence for the closed row `kernel-defect-endings-and-repair-labels-have-no-shared-home`, which gave the endings one spelling but not one chooser.)

## What

Since PR 5349 two functions choose which culprit a defect ending names:

- `recourse::defect_ending(Reading)` (`crates/geom-brep/src/recourse.rs`) gives `KERNEL_DEFECT_ENDING` at a build and `KERNEL_OR_FILE_DEFECT_ENDING` at rest. It takes a `Reading`, so it cannot say "damaged file": `ReadAt::reading` folds the import door to `Reading::AtRest` before it is called. `Unsized::recourse`, `Unsized::residual_in_file`'s fallback and every unsized defect ending go through it.
- `SizedDecision::words(ReadAt)` (same file) chooses `RefusedReading::AsShown`, `DamagedFile` or `ContradictsStoredData` for a sized decision's refused side, and so does say `DAMAGED_FILE_ENDING` at the import door.

The two disagree at the import door past ε_in: a sized stored contradiction there names the file, an unsized or residual one names the kernel or the file (`work/encl/import-door-residual-and-unsized-endings-past-eps-in-name-kernel-or-file.md`).

## Repair shape

One chooser of the culprit, taking the door (`ReadAt`), which both sized and unsized endings read. Not restructured on PR 5349.

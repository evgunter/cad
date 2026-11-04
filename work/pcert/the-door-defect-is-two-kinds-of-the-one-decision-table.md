---
id: the-door-defect-is-two-kinds-of-the-one-decision-table
kind: issue
title: The door's defect is two kinds of the one decision table
status: open
opened: 2026-11-04
priority: P4
cost: E
---


(PCERT, from the STYLE review of `two-decision-tables-for-one-ending`, S6.)

## What

`recourse::Decision` writes the door's defect twice:
`Decision::Unsized(Unsized::Defect)` (the pcurve door's `ChartRow`, in
`PcurveCertifyError::decided`) and
`Decision::Fitted(FittedEnding::Defect)` (the lanes' `PcurveFit` and
`OutsideVocabulary`, in `PlaneNurbsRefusal::decided` and
`AnalyticRung3Refusal::decided`). Both render
`Unsized::Defect.recourse(RefusedArm::SignCertain(None), at)`:
`FittedEnding::ending` delegates there. The text is the same, and the
kind is spelled two ways.

## Repair shape

Give the lanes' arms `Decision::Unsized(Unsized::Defect)` and keep
`FittedEnding::Defect` for the fitted certificate's payload alone, or
the reverse, so that one kind names the defect. The census row
(`every_refusal_ends_through_the_one_decision_table`) needs every kind
reached, so the kind that loses its last refusal has to come off
`Decision` in the same change.

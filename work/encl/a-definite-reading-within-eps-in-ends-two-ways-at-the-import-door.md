---
id: a-definite-reading-within-eps-in-ends-two-ways-at-the-import-door
kind: issue
title: geom-brep: at the import door, a sized decision's definite refused reading within ε_in ends in the door's sentence routed as undecided, and as at rest on the sign-certain arm
status: open
opened: 2026-10-14
---


(Filed by the ENCL implementer of `a-lever-decisions-definite-arm-at-rest-drops-its-note`, from its sweep.)

## What

That unit made one definite refused reading end one way at rest and at the import door past ε_in, whichever arm routes it: both routes now end through `Shows::Refused` with the words `SizedDecision::words` picks (`crates/geom-brep/src/recourse.rs`). Within ε_in the routes still split:

- routed as `RefusedArm::Undecided` (a gate's rejection past the band), the door reads the margin against ε_in (`MarginDiag::sized_recourse_in_file`): a nearer end at or below ε_in ends in the door's one sentence ("This {size} is within the file's declared coincidence distance ... Recourse: {lever}"), with no note and no defect ending;
- on `RefusedArm::SignCertain`, `SizedDecision::recourse` never reads ε_in: it ends as at rest (the defect ending for a stored contradiction, the lever and the note otherwise). A sized verdict drops its margin on the way (`Refused::arm` maps `Refused::Negative { margin }` to `SignCertain(None)`), so the door has nothing to compare.

A past-band reading lies within ε_in whenever ε_in exceeds K·ε (a file declaring 1e-6 m read at ε = 1e-9 m).

## Question

D4 ¶1 says a size below ε_in is not one the file intends, and the ruling for the at-rest unit names only the door *past* ε_in. Should a definite refused reading within ε_in end in the door's sentence on both routes (carry the margin on `SignCertain(Some(..))` for sized verdicts and read it at the door), or in the at-rest ending on both (the door's sentence only for band-decided readings)? Either way the choice would live in `SizedDecision::words` / `Shows`, the one place both routes already share.

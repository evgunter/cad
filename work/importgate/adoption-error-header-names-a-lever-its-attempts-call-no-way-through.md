---
id: adoption-error-header-names-a-lever-its-attempts-call-no-way-through
kind: issue
title: step-import: the Adoption error names a lever in its header while each Transversality/TangentSecondOrder attempt inside it ends 'no way through'
status: open
opened: 2026-10-20
priority: P3
cost: M
---


(Filed by the ENCL implementer of `pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache`, branch `encl/sized-provenance`, from review of PR 5464; pre-existing on main.)

## What

`StepImportError::Adoption`'s `Display` (`crates/step-import/src/error.rs`, ~399) opens "no intensional description certifies (a lever applies to the model the file was exported from, which is then re-exported)", then renders each `AdoptionAttempt`'s refusal (`attempt.refusal.render(*file)`). A `CertCheck::Transversality` or `CertCheck::TangentSecondOrder` attempt certainly on its stored-contradiction side ends past ε_in in `DAMAGED_FILE_ENDING` ("There is no way through: the file is damaged; report it to the program that wrote it", `crates/geom-core/src/predicate.rs`). So one message says a lever applies and, inside it, that there is no way through and the file is damaged.

At the import door, the description that attempt refuses is not one the file states. It is a guess of the adoption ladder (`adopt_edges`, `crates/step-import/src/adopt.rs`), which the ladder rejects before trying the next rung. Calling the file damaged over a rejected guess overstates it: the next rung may adopt the edge, and when the last one refuses too, that rung's ending speaks for the edge. Here the header's lever is the reading that fits.

## Owed

Pick one reading for a ladder attempt at the door. Either render each attempt as the rejection of a candidate (no defect ending of its own), leaving the header's lever and the last rung's ending to speak for the edge; or drop the header's lever where every attempt ends in a defect. A test that renders an `Adoption` refusal with a Transversality attempt past ε_in should go red on the contradiction.

---
id: import-door-pcurve-escalation-ends-in-the-build-note
kind: issue
title: step-import: the import door renders a pcurve re-mint escalation whole, ending in the classifier's build-read coincidence menu and kernel-bug note
status: open
opened: 2026-10-10
---


(ENCL implementer, from the PR 4475 fix pass's census of at-rest readers. Pre-existing.)

## What

`StepImportError::Pcurves` (`crates/step-import/src/error.rs`) renders its `topo::PcurveMintError` whole. For an escalation, that type's `Display` (`crates/topo/src/pcurves.rs`, the `Escalated` arm) renders `{cause}`, the raw `Indeterminate`. The `Indeterminate`'s `Display` is the classifier's own, which names no door. So the refusal ends in the coincidence menu and, on a poisoned margin, the build note: "an unreadable margin may indicate a kernel bug worth reporting".

The import door reads the adopted body at rest (D4 ¶1), so this ending is wrong there:
- the note should name the file (`geom_brep::recourse::unreadable_margin_note(Reading::AtRest)`);
- the ending should be the one `topo::validate`'s `classify_pcurve` gives the same refusal at rest: `not_yet` read at rest.

The other `PcurveMintError` arms are worded in the kernel's voice too ("re-mint the body, and report the op that returned it"). That wording is wrong at a door that adopts a file.

## Repair shape

Per the fork-log row 9 ruling, the at-rest text stays with its reader. Give the import door its own reading of a pcurve re-mint refusal, as `classify_pcurve` is validate's. Do not add `ending(Reading)` to `PcurveMintError`. Texts move only on the import door's `Pcurves` arm.

## Also (ENCL, from the delta review of PR 4475)

The import door's new at-rest undecided-join text (`crates/step-import/src/error.rs` ~490–514) has no pin. `halfcap_pole.rs` tests only the readable tolerance offer. Add a step-import unit row that builds `StepImportError::Join` with a poisoned `JoinUndecided` and asserts the literal "kernel or file defect" ending, alongside this row's pcurve fix.


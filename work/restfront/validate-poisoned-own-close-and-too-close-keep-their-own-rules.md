---
id: validate-poisoned-own-close-and-too-close-keep-their-own-rules
kind: issue
title: topo::validate: own_close and too_close end a poisoned margin by rules of their own, beside MarginDiag::unreadable_note
status: open
opened: 2026-10-09
---


(ENCL implementer, from `sized-poisoned-ending-ignores-the-reading-and-the-file`. Pre-existing.)

## What

That row gave the poison→note rule one home: `geom_core::MarginDiag::unreadable_note` tests the poison, and `geom_brep::recourse::unreadable_margin_note(reading)` picks the note beside `defect_ending`. A poisoned margin keeps the decision's own ending (its lever, or the not-yet ending) and adds the unreadable-margin note its reading gives, naming the file at rest. D4 ¶1 (i) names the decision's lever "always", and a poisoned margin drops only the tolerance arm. Every sized, lever-only and not-yet ending in `geom_brep::recourse` reads it now.

`crates/topo/src/validate.rs` still ends a poisoned margin two other ways, each its own rule:

- `own_close(margin, lever)` (~:2486): a poisoned margin ends in `DEFECT` (`KERNEL_OR_FILE_DEFECT_ENDING`) and drops the lever the site passed. A readable margin at the same site ends in that lever. This is the one site where poison replaces a geometry lever.
- `too_close(margin)` (~:2463): a poisoned margin prefixes the coincidence menu with "check the inputs that built this body, then". That prefix is a third poisoned-margin wording, and it carries no file note.

`own_close` has `Unsized::LastResort` (PR 4453) as precedent, but that decision has no lever. Its only recourse is a tolerance, which a NaN cannot answer. `own_close`'s callers do pass a lever.

## Repair shape

Decide each site against the one rule. For `own_close`, either keep the lever and route the note through `unreadable_margin_note(Reading::AtRest)`, or state why this decision's lever cannot reach a poisoned margin. For `too_close`, do the same, together with `at-rest-coincidence-endings-name-no-tolerance-value`, which already rewrites that function. Texts move only on poisoned arms at rest.

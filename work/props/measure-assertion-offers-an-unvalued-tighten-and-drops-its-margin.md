---
id: measure-assertion-offers-an-unvalued-tighten-and-drops-its-margin
kind: issue
title: editor-core: a measure assertion's undecided verdict offers "tighten the tolerance" with no value, and drops the escalation (and its margin) it was decided on
status: review
priority: P3
cost: M
opened: 2026-09-29
branch: props/recourse-grammar
---


(Filed by the ENCL implementer lane for `certify-zero-verdicts-carry-the-seams-reporting-margin`, from its sweep for unvalued tighten offers outside `geom_brep::recourse`'s table.)

`editor_core::measure`'s assertion evaluation maps every escalation of the measured-versus-bound decision to `AssertionVerdict::Unevaluated { reason: UnevaluatedReason::Indeterminate }` (`Err(_) =>` in the verdict match, near `measure.rs:966`), dropping the `Indeterminate`. Its `Display` (`impl Display for UnevaluatedReason`, near `measure.rs:854`) then says "tighten the tolerance or move the bound" with no value.

- D4 ¶1 (i) wants a tighten offer to carry "the value the margin gives" (below `m/K`), and only where a smaller tolerance decides the margin. Every classify outcome now carries its reporting margin (`geom_core::MarginDiag`), and `MarginDiag::sized_recourse` computes that wording.
- A poisoned margin (`MarginKind::Invalid`) lands in the same arm and is told to tighten, which no tolerance answers.
- An enclosure straddling zero is told to tighten too, where only subdivision or moving the bound helps.

Fix shape: keep the `Indeterminate` on the verdict and end it through `MarginDiag::sized_recourse` (lever "move the bound"), as `geom_brep::recourse::SizedDecision` does.

## Resolved (props/recourse-grammar)

`UnevaluatedReason::Indeterminate` now CARRIES the escalation
(`cause: geom_core::Indeterminate`), and its `Display` ends through
`measure::ASSERT_BOUND_DECISION`, a `geom_brep::recourse::SizedDecision`
with lever "move the bound", size "difference" and
`SizedPass::AnySign` — both definite signs are verdicts, since at the
bound exactly a non-strict relation holds, so the only refused arm is
the undecided one and a smaller tolerance does decide it.

The three readings the row names now part company, from one table:
an in-band margin is offered the tolerance its own margin gives; a
poisoned margin gets the lever and the unreadable-margin note, no
tolerance; an enclosure straddling zero gets the lever alone.

`UnevaluatedReason` loses its derived `Eq` (the escalation's reporting
margin is floating point); `PartialEq` stays.

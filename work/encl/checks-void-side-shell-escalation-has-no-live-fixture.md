---
id: checks-void-side-shell-escalation-has-no-live-fixture
kind: issue
title: editor-core: no live fixture reaches the checks window's void-side (negative-margin) shell escalation
status: review
pr: 3401
opened: 2026-09-28
priority: P3
cost: E
---


(ENCL implementer, PR 3390 round 2.) The rule is D4 ¶1 (i) in
`docs/DESIGN.md`.

## What

`topo::ShellClassifyError::ending` (`crates/topo/src/props.rs`) routes
the shell-role decision through `SizedPass::NonZero`, so a void shell
whose `V/A` sits in band ends in "…, or, if this thickness is intended,
tighten the tolerance below |m|/K m". Only the outer side has a live
fixture: `dsc_checks::in_band_shell_escalates_typed_never_guessed`
(`crates/editor-core/tests/dsc_checks.rs`) builds a thin slab. The
void side is pinned on a synthetic `chk_shell_volume_sign` diagnostic
(`refusal_concision_chains::every_escalated_check_finding_ends_in_its_decisions_recourse`,
the "in band, void side" row); no document reaches it.

## Repair shape

Build a document whose product body carries a void shell of in-band
thickness (e.g. a box holding a sheet-like cavity `(1 + K)·ε`
thick, built relative to the run's ε and K as the slab row is), assert the finding is `Escalated` on
`chk_shell_volume_sign` with a negative margin, and pin its valued
ending.

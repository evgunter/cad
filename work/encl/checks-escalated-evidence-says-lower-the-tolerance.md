---
id: checks-escalated-evidence-says-lower-the-tolerance
kind: issue
title: editor-core: the checks window's Escalated evidence says 'lower the tolerance' with no decision behind it
status: dispatched
opened: 2026-09-28
priority: P3
cost: E
---

(ENCL implementer, from the §5 sweep of PR 3382.) The rule is D4 ¶1 (i)
in `docs/DESIGN.md`; `geom_brep::recourse::SizedDecision` is the shared
ending table for a sized decision.

## What

`crates/editor-core/src/checks.rs`, the finding recourse for
`CheckEvidence::Escalated` (~:647): "Recourse: thicken or remove the
degenerate geometry, or lower the tolerance". It names no decision,
and its tolerance arm is unconditional and unvalued.

## Repair shape

Route the escalated evidence to the decision that escalated. The
evidence carries its source refusal, whose own ending (certify's,
the meters', validate's) is derived from the decision. Reuse that
ending rather than a window-wide sentence. Re-pin the checks window's
text rows.

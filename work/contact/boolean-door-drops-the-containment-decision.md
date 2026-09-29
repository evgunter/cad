---
id: boolean-door-drops-the-containment-decision
kind: issue
title: topo: the boolean door maps ContainError::Escalated to BooleanError::Escalated and drops its decision
status: open
opened: 2026-09-29
---


(CONTACT-10 implementer, from the §5 sweep of `contact/10-contain-endings`;
the rule is D4 ¶1 (i) in `docs/DESIGN.md`.)

## What

`ContainError::Escalated` now carries its `ContainDecision`, but the
boolean door drops it:

- `crates/topo/src/boolean/reduce.rs`, `fn esc`, and the curved-placement
  arm of the span walk
  (`Err(ContainError::Escalated { diag, .. }) => BooleanError::Escalated { diag }`);
- `crates/topo/src/boolean/ops.rs`, the extent scan's `contfp` `map_err`.

`BooleanError::Escalated`'s `Display` (`crates/topo/src/boolean/mod.rs`)
ends in `COINCIDENCE_RECOURSE` whatever contfp was deciding. A boolean
does take a declaration, so the menu is right for a coincidence
decision. It is not right for an arc's span (`ArcSpan`), a window's
period (`WindowPeriod`) or a ray of the schedule (`Ray`).

## Repair shape

Carry the decision on `BooleanError::Escalated` (or a sibling variant
for containment), and end the containment decisions through
`ContainDecision::ending(diag, Reading::Build)`. Keep the menu only where
the refused side is a declarable coincidence.

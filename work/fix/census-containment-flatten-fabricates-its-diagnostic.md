---
id: census-containment-flatten-fabricates-its-diagnostic
kind: issue
title: the containment census flattens three ContainError arms onto CensusEscalated with a SYNTHESIZED indeterminate — the message names a margin nothing measured
status: review
opened: 2026-09-11
branch: fix/census-containment-cause
pr: 2420
---


(FIX orchestrator) Found by the
`census-flattens-the-typed-chart-region-declines` lane's sweep
(PR 2354), one lane over from the arms that unit fixed.

## The defect

`crates/topo/src/census.rs:819` maps `ContainError::ArcLoopUnsupported`,
`::RayExhausted` and `::Corrupt` onto a single `CensusEscalated` —
the same flatten the parent unit removed at the chart-region matches.

**It is worse than a flatten, and that is why it gets its own row.**
`CensusEscalated` carries an `Indeterminate`, not a cause, so the site
**synthesizes** one: `invalid(band, "pm_census_containment")`. The
finding that reaches the user therefore names a margin **nothing
measured**. That is not a refusal whose text is imprecise about its
cause; it is a refusal carrying a fabricated number in the field a
reader would use to judge how close the call was
(`memories/refusal-text-is-not-cause.md`, at the payload rather than
the sentence).

## Why it was not taken in PR 2354

Two reasons, both real:

1. **The repair is a different door-shape question.** The parent unit
   widened `CensusUnsupported` with a cause; `CensusEscalated`'s
   payload is an `Indeterminate`, so there is nowhere to put a cause
   without deciding what an escalation's cause field *is*.
2. **Re-routing those three CAN move the answer.**
   `editor_core::assembly::attribute` dispatches on the
   `ValidationError` variant, and PR 2354's central measured finding is
   that a **payload** is invisible to that dispatch while a **variant**
   is not. Sending any of these three to a different variant is
   therefore an `AtRest`/`Uncertified` change, not a rendering change —
   the exact line that unit was careful to stay on.

So this is door-shape work with an answer-moving arm in it, and it
wants its own unit with its own red-first rows, not a thread.

## What to establish first

Whether the three arms are even one class. `Corrupt` is a
kernel-invariant violation and `ArcLoopUnsupported` is an inventory
fact; PR 2354 established that the twelve chart-region arms did **not**
split cleanly along the axis their names suggest, and there is no
reason to expect these three to either. Read what each arm means from
the code, not from its name.

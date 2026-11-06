---
id: boolean-error-states-no-standing
kind: issue
title: BooleanError states no standing, so a wrapper that keeps one (OverlapRefusal::Boundary, ::Corner) must abort on every kind but Escalated
status: dispatched
opened: 2026-11-06
priority: P3
cost: M
---


Found by PR 6553's review (`onecopy/refusal-standing`).

## What

`boolean::BooleanError` (83 variants, `crates/topo/src/boolean/mod.rs`)
states no standing (`geom_brep::ladder`'s module docs). A wrapper that
keeps one in a search cannot tell a limit from a kernel fault, such as
`ClassificationInvariant` or a corrupt body.
`boolean::overlap::OverlapRefusal::refusal` therefore ranks `Boundary`
and `Corner` by margin where the error is `BooleanError::Escalated`, and
aborts on every other kind. That is sound: a fault is never kept, so no
later witness answers past it. But it gives up completeness wherever the
error was a limit a later witness could have answered past.

## What closing it takes

`BooleanError::refusal` beside its variants, each variant's class in its
doc, read through `BooleanError::kind`'s exhaustive match so that a new
arm reds it. `OverlapRefusal` then delegates, the way it already does
for `ContainError`, `BoundaryRefusal` and `NormalAtError`. Measure
whether any `ci` row keeps a limit `Corner` ahead of a witness, since
that is where the outcome would move.

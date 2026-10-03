---
id: refusal-menu-stamps-decided-coincident-on-an-in-band-coincidence
kind: issue
title: wire: refusal_menu stamps FlushRung::DecidedCoincident on every UndeclaredCoincidence, an in-band one included
status: open
opened: 2026-10-03
priority: P3
cost: E
---


(TOPO, found by the receipt sweep of
`plane-offset-rung-decided-zero-shares-invalid-with-a-poisoned-margin`.)

## What

`editor-core/src/eval/wire.rs`'s `refusal_menu` (~:3763) lifts every
`topo::BooleanError::UndeclaredCoincidence` into
`NodeErrorKind::UndeclaredContact`, and builds the finding with
`rung: names::FlushRung::DecidedCoincident` (~:3811) whatever the
refusal's margin read. Its comment reasons only that shared-source pairs
never refuse `Undeclared`, so the rung is the geometric one — true — but
`DecidedCoincident` means "definitely parallel, definitely zero offset",
and `UndeclaredCoincidence` is raised for an in-band coincidence too
(`topo::boolean::undeclared_coincidence`: `CoincidenceMeasure::Zero` and
`::Undecided` both become this variant, the margin riding `diag`).

So a refusal whose coincidence landed in the ambiguity band is offered
back as a finding the body-seat detector (`topo::flush::pair_finding`)
would never report: that detector refuses `PairInBand` on the same pair.
Detector and refusal menu then disagree on one pair, which the anti-twin
rule (SELECT-DESIGN §3b) exists to prevent.

## Shape

The kernel's variant carries the margin as an `Indeterminate`, so
`refusal_menu` cannot tell the two apart without reading the number. The
likely repair is for `UndeclaredCoincidence` to carry the
`topo::CoincidenceMeasure` the ladder now types, so the menu offers a
`DecidedCoincident` finding on `Zero` only and names the in-band arm as
what it is. That widens `UndeclaredCoincidence`'s payload across
editor-core and pncad-py, which is why it is not in the topo unit.

---
id: a-conics-belly-graze-on-a-plane-is-not-read-against-the-face
kind: issue
title: A conic grazing a plane face's carrier in band escalates BellyGraze though carrier_touch::off_face now reads a plane
status: open
opened: 2026-10-30
priority: P3
cost: E
---


Filed by `splice/a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`'s
lane, from its sweep for unsettled counts that do not go through
`carrier_touch::off_face`.

## What

The planar lane (`topo` `boolean/reduce.rs`, the conic arm of the
plane sweep) escalates `ConicPlaneMeet::Roots(Err(fault))` straight to
`BooleanError::Escalated`. Among those faults is
`ConicRootFault::BellyGraze` (`splitting/classify.rs`): a circle or an
ellipse that comes within the band of the plane without a decided
crossing. The curved lane's root doors route the same kind of answer
through `carrier_touch::off_face`, which places a touch outside the
face's trim and passes it. `off_face` now reads a plane: the spline
lane's unsettled count goes through it on both lanes. So a conic
grazing a plane face's carrier outside the face still refuses, where
the localization could pass it.

This is over-refusal, not unsoundness. No witness reaches it yet.

## What it needs

Route `BellyGraze` through `off_face` (the conic carrier is localized
like a line's, `geom::carrier_speed`), with a row of a circle grazing a
plane in band outside a face that reds without it. The other faults
(`PlaneParallel`, `CrossingInterior`, `RootOrder`) are decisions about
where a crossing lies, not whether there is one, and stay escalations.

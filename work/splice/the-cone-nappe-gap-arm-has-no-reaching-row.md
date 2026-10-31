---
id: the-cone-nappe-gap-arm-has-no-reaching-row
kind: issue
title: place_crossing's nappe reading in the band's gap has no row: no lane reaches a crossing that near a cone's apex
status: open
opened: 2026-10-31
priority: P3
cost: E
---


Filed by PR 6136's fix pass, from its review.

## What

`topo` `boolean/reduce.rs` `place_crossing` tells a certified crossing of
a cone face's carrier off the face's own nappe
(`decide("bool_cone_root_nappe")`). Where that decision falls in the
band's gap, it answers `Placed::NappeGap`, which `wall_crossing`
escalates as `ConeRoots`/`ArcConeRoots` and `spline_wall_event` as
`ConeRoots`, located at the edge and the face. No row reaches the arm.
With it mutated to `Placed::OffNappe`, topo's cone, apex and nappe rows
(184) stay green.

The reason given at the site: a crossing whose nappe the band cannot
tell lies within a few gaps of the apex. There, the line and conic root
doors refuse at the apex first (`Unsettled::AtApex`). The ring composite
cannot read past the band between such a crossing and the double cone's
other sheet, since its meter is the net hull's divisor, so it refuses
first too.

## What would close it

Either a fixture that reaches the arm: a carrier crossing the far nappe
within the gap of the apex, which neither door refuses first. Or a proof
that none exists, after which the arm can be an invariant.

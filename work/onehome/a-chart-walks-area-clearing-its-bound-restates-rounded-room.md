---
id: a-chart-walks-area-clearing-its-bound-restates-rounded-room
kind: issue
title: A chart walk's area clearing its error bound restates Rounded's room on a type of its own
status: open
priority: P1
cost: E
opened: 2026-11-02
---


Found by the sweep of `onehome/vertex-section-room` (the vertex-section
quotient's room precondition), shape "a magnitude past its own bound".

`geom_brep::chart_singularity::area_clears` decides whether a
winding-free walk's area clears its error bound as
`Margin::sign_of(reading.area.abs() - reading.error)` under
`Band::enclosure_sign()`. That is the question
`geom_core::running::Rounded::room` / `Rounded::divisor` answer for a
`Rounded` (the enclosure `value ± error` wholly off zero), asked on
`WalkSum`'s own `area` / `error` pair (`chart_singularity.rs`, the
`WalkSum` struct and its `walk()` copy), a type with a `Rounded`'s two
fields plus the per-sample `errors` the bisection reads.

Not a disagreement today: both read the sign at `Band::enclosure_sign`.
The fix is to hold the pair as a `Rounded` and ask its `room`, or to
record why the walk's sum is not one (its bound is a sum of per-sample
bounds the bisection reweighs, which `Rounded` does not carry).

P1 by `work/README.md` Priority (two implementations of one underlying logic), though small: one restated predicate, no disagreement today.

Open PR 6329 (`[ev]`) would retire `running::Rounded` as a margin
charge on the f64 lane; if it lands that way, this row closes with it.

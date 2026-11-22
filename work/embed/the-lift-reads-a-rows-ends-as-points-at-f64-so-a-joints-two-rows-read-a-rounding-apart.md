---
id: the-lift-reads-a-rows-ends-as-points-at-f64-so-a-joints-two-rows-read-a-rounding-apart
kind: issue
title: boundary_of lifts each row's ends as points at f64, so two rows at one joint read a rounding apart and every reader comparing them sees a gap or an overlap no cell narrows
status: open
opened: 2026-11-21
priority: P3
cost: M
---


## Finding

From the `embed/collinear-cover` lane (2026-11-21), measured with the
at-rest gate on `embed/collinear-cover-measure`.

`crates/topo/src/clearance.rs`, `boundary_of`, takes a use's chart ends
(`Use::image`) as `row.pcurve.eval(t)` at the row's two parameter ends,
evaluated at the body's scalar and then lifted (`lift.of`). At `f64` the
lift of a value is a point, so an end carries none of the rounding of
its own evaluation: each row evaluates its own line `α + β·t` at its
own parameter, and two rows that meet at one vertex agree to rounding,
not bit for bit. Nothing upstream promises more: the lift's contract is
that a joint "is not decided again in the chart" (`pcurves.rs`, the
loop walk).

Measured on `blend_value_decided_rows::a_disc_rim_records_its_joints_and_its_supports`
(a three-arc disc's filleted rim, the band's foot parallel on the torus
chart): at joint `9v1` the two rows' ends are `4.188790204786391` and
`4.188790204786391`, equal; at joint `8v1`, `2.0943951023931953` and
`2.0943951023931957`, one ulp apart; at joint `7v1`, across the seam,
`6.283185307179586` and `5.55e-17`. Over the 13 census tests of the
cylinder/torus bucket, every joint the cover read in a failing run
(about 20,000 reads): equal in 12,834, otherwise between `2.2e-16` and
`1.8e-15` apart (one to four ulps at those values), nothing larger.

## What reads the two ends

- `covered`, of a line's edges' ranges (`vacuous`, `sides_are_the_faces`,
  the zero side's `on_edges`): a one-ulp gap at a joint read the side
  uncovered at every depth, the cause of the bucket. Closed by reading a
  joint by its vertex (`joints`), which compares nothing.
- `run_on` and `convex`, which read a corner by its vertex too.
- `extent_of` and `face_extent`, which take hulls, where an ulp either
  way is inside `chart_slack`.
- `on_vertex`, which decides two edges' points at a corner are one in
  the band.

Not swept: every other reader of `Use::image` (the curved
presentations, `iso_params`, `Pieces::of`). A reader that compares two
rows' ends for equality or order is the shape to look for.

## What it calls for

Either each end encloses its evaluation's rounding at `f64` (so two
rows' ends at a joint are two enclosures that meet, as they are at
`Interval`), or a vertex's chart coordinate on a line is read once and
handed to both rows. The first changes what every reader above sees and
is `boundary_of`'s, shared ground with the rational-window lane
(PR 7120); the second needs a home for a vertex's chart point per face.
Neither is needed by the joint reading, which is why this is a row and
not a blocker.

## The held value is the same reading (PR 7322's review, 2026-11-22)

A row's held value (`Iso::at`, `iso_of` on the row's pcurve) is read
per row exactly as its ends are, so two rows of ONE coordinate line
can hold it a rounding apart. Measured: the three-arc filleted disc
mirrored across a generic plane (`topo::mirror_body`, point
`(0.3, −0.2, 0.1)`, normal `(0.48, −0.6, 0.64)`), torus band `9v1`,
foot-parallel rows `20v1` and `21v1`, joined at vertex `8v1`: they hold
`v = 0.0` and `v = 4.014566457044562e-16`. `present_chart`'s merge read
them as two lines on one side (`Local` at every depth, at both
scalars), and `vacuous` did not gather them.

Readers of two rows' held values: `present_chart`'s merge and `vacuous`
(by key since PR 7322 where the rows are joined at a vertex, by a
common value otherwise); `face_extent` (each line against the extent,
inside `chart_slack`); `zero_of` and the corner readings (one row
each). Bounded as the ends are: the two rows' chart points at the
shared vertex map within `4ε` in metres by the rows' envelopes and the
edge certificate's end pins; no check decides the two held values
against each other.

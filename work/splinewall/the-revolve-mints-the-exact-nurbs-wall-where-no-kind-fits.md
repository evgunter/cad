---
id: the-revolve-mints-the-exact-nurbs-wall-where-no-kind-fits
kind: unit
title: The revolve mints the exact rational NURBS wall of revolution where no kind fits: an ellipse arc off its axis or tilted, and a spline meridian
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
blocked_on: [measure-what-an-exact-nurbs-wall-of-revolution-does-today, profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader]
---



Part 3's revolve door (`an-ovoid-cannot-be-authored`, Ev PR 7289). The
ruled sentence is `docs/DESIGN.md` `[surfaces]`: "a wall swept from a
profile segment with no kind of its own is the exact rational `Nurbs`
surface, never an `Approx`". For the revolve that is an ellipse arc off
its axis or not aligned with it (a quartic of revolution), and a spline
meridian.

## What is true when it is done

- `sweep::revolve` mints, where OVOID's
  `the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis`
  refuses naming this unit, the exact rational NURBS wall of
  revolution of the meridian, built once over a rational planar curve
  so a spline meridian reuses it when
  `a-profile-segment-is-a-rational-spline-and-the-nurbs-legs-lower-to-it`
  lands.
- A meridian end on the axis builds its pole row as exactly one point,
  every control point the stored pole bit for bit (`[surfaces]`, PR
  7287: "a constructor that decides such a side collapsed at its band
  builds it exactly one point"). This unit relies on that sentence; the
  readers' half is CHARTTAIL's.
- Full and partial revolves of an off-axis ellipse arc build and are at
  rest at every lane scalar.

## Check first

The construction of the angular direction, which
`measure-what-an-exact-nurbs-wall-of-revolution-does-today` will have
measured. The standard circle (quarter or third arcs on interior double
knots) puts C⁰ knots in `u` that are smooth but read as creases (SPLICE's
`a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`);
one designer's unchecked alternative is a single-span rational cubic
semicircle (weights 1, 1/3, 1/3, 1). Which one is a choice the
measurement informs and this unit must not make alone: report the
measured costs of each and ask.

## Rows and ground

- Reaches SPLICE's three rows and CHARTTAIL's
  `a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint`
  from an everyday verb; each one this unit's bodies hit becomes its
  blocker and is re-banded P1 (`work/README.md`, a blocker inherits).
- Crosses LATHE (`revolve/*`), NURBS. Announce on LATHE's and SPLICE's
  logs.
- Review: **dual**. Difficulty **H**.

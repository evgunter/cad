---
id: a-spindle-chart-has-no-pcurve-lane
kind: issue
title: A spindle chart (Surface::Spindle) has no pcurve lane: every carrier on it, closed-form class or general, refuses UnsupportedChart at the mint and at check 4
status: open
opened: 2026-11-03
priority: P2
cost: H
---



Filed by PCERT unit E4 (`pcert/circle-general-lane`, PR 6485), from the
review's request that the C4 page's list of unrouted classes be complete.

## What

`crates/geom-brep/README.md`'s C4 page says every class of carrier a
chart can hold has a route into a certified row. On a spindle chart
none has one:

- `pcurve_cache::derive_class`'s `Surface::Spindle` arm refuses
  `UnsupportedChart` ("A spindle sheet's closed-form images are not
  built"), so no parallel or meridian of the sheet images in closed
  form;
- the projected lane (`pcurve_cache::projected`) refuses it too:
  `chart_at` answers `UnsupportedChart` for a spline net (whose
  `projected::lane_of` admits every analytic chart), and
  `projected_envelope` returns `UnsupportedChart` before its arms;
  `lane_of` admits no circle or line on a spindle.

`UnsupportedChart` ends "not yet" (`Decision::NotYet`), so nothing ends as
a defect; but a spindle face cannot hold certified rows, which C4
requires of every analytic chart at rest.

## Fix

Stage 2 or later of the spindle sheet
(`the-spindle-sheet-away-from-its-apexes`, which refuses an apex
typed): the sheet's own classes (parallels, meridians) in closed form,
and the projected lane's chart scalars, frame, form and metre conversion
for the spindle (`projected::Meters`, `implicit::conic_form_sup`), with
its two apexes as the singular set the sector condition refuses near.

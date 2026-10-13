---
id: the-wall-seam-arm-positions-its-column-off-the-start-foot
kind: issue
title: nurbs_iso_derive's wall-seam arm positions its column off the carrier's start alone, so a seam this chart does not share refuses at certification instead of taking its General image
status: closed
opened: 2026-10-13
priority: P3
cost: E
closed: 2026-10-13
branch: pcert/wall-seam-column-pick
pr: 5042
---


Found by `pcert/off-iso-line-takes-its-general-row` (the step-0 census
of `nurbs_iso_derive`'s one-foot picks); left out of that PR by its
orchestrator.

## What happens

`nurbs_iso_derive`'s wall-seam arm (`crates/topo/src/pcurves.rs`, the
`Chart(ChartCurve { pcurve: Pcurve::IsoLine { .. }, .. })` arm on the
NEIGHBOUR's chart) takes the image's shape from the neighbour's
description, a column with the neighbour's `v` map verbatim. It
positions that column with `side_pick` at the carrier's START point
against `[cu0, cu1]`, and otherwise at the start's certified chart foot
(`derive_chart_foot`). Both read the start alone, so a column through
the start foot whose `v` map this chart does not share, or a carrier
that is no column of this chart at all, is picked anyway.

## Why it is P3

The pick is a selection that the iso lane checks: the mint's
certification compares the image against the carrier over the whole
span (the arm's doc: "CERTIFIED by the full iso lane — a wrong pick
fails loudly, never silently"). So a wrong pick is a typed refusal of
the face, not a wrong row at rest. That is a false refusal wherever
the locus has a certifiable `General` image. Reasoned from the code
and not executed: the step-0 census instrumented the line-rim arm only,
and no suite was seen reaching this arm with a wrong pick.

## Fix

Judge the positioned column with `probe_pick` (the interior probe the
boundary-iso schedule and the arc rim share). Where no candidate holds,
fall through to the schedule's `General` image, with an escalated
candidate keeping its precedence over the derived image. This is the
same pattern `pcert/off-iso-line-takes-its-general-row` gave the
line-rim arm.

## Closed

`pcert/wall-seam-column-pick`. The arm offers the neighbour's stated
`v` map, as a column at each chart boundary and then at the column
the start's measured foot names, to `probe_pick` (judged a quarter span
in, label `pcurve_iso_wall_seam`). Where none holds, the carrier takes
the boundary-iso schedule an `Intersection` seam takes: its closed-form
columns and rows in both directions, the measured map, then `General`,
with a deferred escalation keeping its precedence. `side_pick` is gone.

What moved:
- a seam on a wall reversed in `v` (or in `u` and `v`) against its
  neighbour now derives the column that runs the other way, and the
  wall mints: `sweep`'s `a_wall_seam_takes_the_column_it_lies_on`;
- a rim stated as a ROW in another chart used to derive a constant
  image (`pl = (0, 0)`) that certification refused; it now derives its
  own row: `verbs_offc_consumer::a_rim_stated_as_a_row_in_another_chart_takes_its_own_image`;
- at a dual, the arm's no-lane refusal is the `General` route's
  `FittedLaneUnsupported { scalar: "dual" }`.

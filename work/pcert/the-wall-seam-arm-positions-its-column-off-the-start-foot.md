---
id: the-wall-seam-arm-positions-its-column-off-the-start-foot
kind: issue
title: nurbs_iso_derive's wall-seam arm positions its column off the carrier's start alone, so a seam this chart does not share refuses at certification instead of taking its General image
status: open
opened: 2026-10-13
priority: P3
cost: E
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

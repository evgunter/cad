---
id: a-closed-spline-charts-image-across-its-seam-is-read-on-the-extension
kind: issue
title: On a spline chart closed in a channel, an image stated past the seam is certified against the boundary cell's polynomial extension, not the chart a period back, so the knot-rectangle hold skips that channel
status: open
opened: 2026-11-25
priority: P1
cost: H
refs: [a-general-image-that-leaves-the-knot-rectangle-between-two-nodes-certifies-its-row-off-the-wall, a-lifted-copy-on-a-closed-spline-chart-has-no-placement-term]
---


Filed by PCERT (the orchestrator), from PR 7738's hosted run.

## What

PR 7738 holds a fitted or general image to its spline chart's knot
rectangle (`ssi::certify::chart_domain_limb`). The tensor composite reads
an image past the rectangle on the boundary cell's polynomial extension
(`geom_core::spline::compose::tensor`, "Domain posture"), which is no
point of the chart. In a channel the net closes in
(`geom_brep::spline_net_closes`), the mint states a side that crosses the
seam in a window past the domain: the image runs a period over, to be
read as the chart a period back.

The hold measured this on `topo`'s
`spline_region::rows::a_face_across_a_closed_charts_seam_reads_its_region_and_its_hole`
and `…::the_ring_door_moves_a_spline_faces_rings_into_its_outer_loops_region`.
On the square tube, a line's image runs to `u = 1.1` on the knot domain
`[0, 1]`, so the hold refuses it (escape `0.1`). The row is correct
there only by coincidence: the seam sits mid-side, so the last cell's
linear extension is the first cell's side.

So the hold skips a closed channel (asked only of an escape), which
keeps that channel's posture from before 7738:

- On a curved closed chart a seam-crossing image reads an extension
  that leaves the surface. Its hull refuses a carrier that is on the
  surface: an over-refusal.
- It certifies a carrier that follows the extension off the surface,
  between two nodes of the image. This is the P0's soundness hole,
  across a seam.

## What is needed

The composite reads a piece of the image past a closed channel's seam
against the chart a period back: the piece shifted by the knot period,
or the net re-expressed over the next period. The hold then applies to
every channel. `a-lifted-copy-on-a-closed-spline-chart-has-no-placement-term`
is the neighbouring question of a whole-period lift's placement.

Not probed: a curved closed chart (a round tube's net) with a face
across its seam, at rest, to measure the over-refusal; and a carrier
following the extension, to pin the hole.

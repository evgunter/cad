---
id: a-rim-arcs-pad-is-its-widest-certifying-rung
kind: issue
title: A spline x implicit rim arc's pad is its widest certifying rung, a third of the wall on a plane
status: open
opened: 2026-11-12
priority: P3
cost: M
---



Filed by the `sectarms/join-reads-spline-implicit` lane.

## What

A rim arc's pad (`CarriedComponent::pad`, `spline_implicit::pad_of`) is
the farthest the chart tube's windows, imaged on the wall, stand from
the carrier. The tube's radius ladder takes the widest rung that
certifies, so on a uniformly transverse pair the windows are wide: the
plane `z = 0.01` across the 0.02 m carry-row wall certifies at the rung
`3.6e-3` m, chart pads `0.17`, and the pad reads `6.1e-3` m, a third
of the wall. The carrier itself lies within `1e-16` m of both surfaces.

The pad is sound: limb 3 holds the section in the windows one arc. It
is loose, and every reading of a site against the component pads its
band by it, so two components of one pair closer than their pads read a
site on both and desync. No row meets that yet.

A narrower chain, certified after the wide one, holds the same arc
(a sub-arc spanning the same end slices is the arc) and would give a
pad near its rung; PR 6829's per-box narrowing toward a node is the
same move for node arcs.

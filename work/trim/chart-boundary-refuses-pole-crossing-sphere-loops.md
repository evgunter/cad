---
id: chart-boundary-refuses-pole-crossing-sphere-loops
kind: issue
title: a chart singularity has no boundary polygon — sphere and cone faces meeting one are refused, and the polar description is unbuilt
status: open
opened: 2026-09-04
---

## What

`topo::chart_boundary` (`crates/topo/src/pcurves.rs`) refuses, typed
(`PcurveMintError::SingularChartJoint`), any loop with a joint where
the chart's first channel has no lever — a sphere pole or a cone apex.
A face that meets its chart's singularity therefore gets no
description, and the polar-cap description that would serve it is
unbuilt.

## Why it is a refusal, corrected

**This file first said the case was "no description". It was worse
than that: it was a WRONG one**, and the correction is the point of
the row. `walk_loop` accepts a singular joint deliberately — the
azimuth-free arm skips the branch shift, because every azimuth agrees
at a pole and rounding on a collapsed lever would manufacture a branch
choice from a measurement that refused. That is right for MINTING: no
downstream sample reads the azimuth AT the pole.

A chord polygon does read it, as a vertex. Measured on a quarter disc
revolved 90° about its own edge (a sphere octant whose loop touches the
pole): the face's chart region is a rectangle, the chord polygon a
triangle, and **210 of 750 certified cells held material** — a
certification that is simply false. The cone twin is the
axis-touching triangle's apex face, which reached the wrap fence
instead and refused for the wrong reason.

## Who needs it

Nobody today. TRIM-3 PR-2's consumer (`clearance.rs` `window_of`)
tightens plane and cylinder windows only, and cone/sphere/torus keep
their existing carrier windows
(`work/trim/clearance-window-cone-sphere-torus.md` is that unit's
residue). This is a capability gap disclosed where it was created.

## Shape of a fix

The missing half is what a boundary polygon IS across a chart
singularity, where the azimuth chart degenerates. Two candidates:

- a polar-cap description stated in a chart that does not degenerate
  there (the pole's own tangent frame), joined to the azimuth chart's
  polygon at the latitude where both are regular; or
- a region description that is not a chord polygon at all — the
  envelope boxes read as a covering — which the round-hole row
  (`round-holes-get-no-chart-bound-benefit`) also wants.

Whichever lands, the refusal above is the safe state until it does.

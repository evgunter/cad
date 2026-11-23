---
id: a-smooth-chart-describe-edges-keeps-reads-under-determined-only-at-its-extent
kind: issue
title: describe_edges keeps a smooth chart image the rule reads under-determined at its extent; a longer edge can read it determinate
status: open
opened: 2026-10-15
priority: P3
cost: M
---


## What

Found by the probe for
`a-kept-chart-is-not-re-asked-when-a-door-changes-its-extent` (PR 5197).

`describe_edges` (`crates/topo/src/boolean/ops.rs`, the `stale` match)
keeps an edge's existing `Chart` description when the image names one
of its two sides and the must-carry rule reads the seam smooth and
under-determined. One way that reading comes out under-determined is a
zero-side second-order station: the sagitta over the edge's extent sits
in the zero band. That reading depends on the extent. A longer edge
over the same pair can read jet-determinate, and tier 3 then refuses
the kept image (`TangentNotIntrinsic`).

PR 5197 made every door that keeps a chart image over a changed extent
ask the rule again (`geom_brep::reask_chart`), so such an image is
restated when a split, join, re-anchor or offset lengthens it. What is
left is the keep itself. The image is legal where it rests, but whether
it stays legal rests on its extent, not on its pair. Coincident
carriers, the other source of a kept image on two distinct surfaces,
read under-determined at any extent.

## Witness status

None. The probe ran the six door arms over the topo, sweep, editor-core
and step-export suites at ε 1e-9, 1e-6 and 1e-12. No valid body carried
such an image into a door, and no body at rest was seen holding one.
The nearest candidate, an extruded line tangent to an arc, stores its
struts as declared `OnPair`, not as a chart image.

## Owed

A witness first: a boolean whose zipped seam between two curved
surfaces is kept as a smooth `Chart` read under-determined on a
zero-side station. Then decide whether the keep should store the pair
(`OnPair`) instead, as `describe_edges`' stale arm does, so that every
door's `OnPair` arm re-asks it.

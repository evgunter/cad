---
id: an-arrival-a-later-step-completes-draws-no-construction
kind: issue
title: a fused fillet's arrival that a later step completes draws no construction carrier
status: open
opened: 2026-11-02
priority: P3
cost: M
refs: [a-fused-fillet-verb-draws-nothing-of-its-parts-so-a-refusal-is-unreadable]
---


A fused fillet step's construction marks
(`profile::path::program::fused_constructions`, drawn by
`pane::viewport::push_constructions`) draw an arrival carrier only
where the step itself fixes it: a `Center` arrival spec (its circle
about `c` through its endpoint), or an arc the step drew as its run
out. Three arrivals are completed by a LATER step and draw none:

- `arc_fillet(spec, r)`'s line arrival, which the next leg
  (`line_to`, `.at(..).angle(..)`) binds;
- a `Radius` or `Via` arrival spec on `fillet_arc` / `arc_fillet_arc`,
  which awaits its binder (`DynTip::RadiusArrival`, `ViaArrival`);
- a bare `fillet(r)`, whose arrival side is wholly a later step's.

When that later step's fit refuses, the incoming carrier is drawn and
the arrival is not, so half of what the refusal is about is still
missing. A fix reads the arrival off the completing step's own data
(its target, or its binder's anchor and director) in the same walk, and
attributes it to the fused step it completes.

---
id: no-row-turns-on-the-bound-a-cone-pairs-apex-reach-is-read-from
kind: issue
title: step-import: no row turns on which bound a cone pair's apex reach is read from, and two neighbours are argued unkillable
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by `oneround/lever-sites-brep` (PR 7219), answering its delta
review's D1 and D2.

## The site

`step_import::build::choose` levers an edge's pose readings at how far
its two faces run, as a `LeverBounds` (`step_import::reach::faces_run`).
Where a cone is in the pair the readings are levered from its apex, so
each bound takes in the edge's reach from the apex on its own side:

```
let reach = geom_brep::edge_reach_bounds(&spec.carrier, span, apex, region.lever_from(apex));
LeverBounds::Between {
    below: extent.below().max(reach.below()),
    above: extent.above().max(reach.above()),
}
```

`reach.below()` is the edge's sampled reach where its cover is not its
own (an ellipse, a spiric, a spline) and the cover's far distance on a
line or a circle arc, where that distance is exact. Before PR 7219's
delta pass both bounds took the cover's far distance, so on the other
kinds a reading from above stood under `below`, which feeds readings
that build on a definite sign.

## The unpinned mutant

`below: extent.below().max(reach.above())`. It passes step-import's
whole suite, the slow set included (482 tests; the three reds are the
NIST FTC-09 ones, red without it).

Why nothing turns on it:

- Every cone-pair edge the suite reads there is a line or a circle,
  where `edge_reach_bounds` is `Exact` and the two are one value. The
  second delta read counted 121 apex reads: 115 on a circle, 6 on a
  line.
- An ellipse arc did not separate them on the fixture probed: a cone of
  half-angle `π/2 − δ` coaxial with a unit cylinder, the edge printed as
  an `Ellipse` of equal semi-axes over spans of 0.5 to 3 radians, read
  `Between { below: 1.00000000005, above: 1.00000000005 }` from the
  apex. Its cover is tight from there.
- The faces' run is folded in by `max`, so the apex reach matters only
  where it passes the run.

## The fixture that would turn red

A cone pair whose printed edge is a spline (a rational circle arc, say
120° in three controls, whose middle control stands `2r` from the
centre) so that the control hull's far distance from the apex passes
both the edge's true reach and the faces' run from below; and a pose
reading levered at `below` that acts on its definite sign with its
margin between the two levers (`coc_aperture_cos` on a nearly flat
cone is a candidate: `cos α` times the lever). Correct code reads it in
the file's band and refuses as ambiguous; the mutant reads it Positive
and goes on to build.

## Argued unkillable, with the reason

Two neighbours of that mutant survive the whole suite as well, and no
row can turn on either as the readers stand. Both are said at their
sites.

- **`coaxial_tangent_circle` handed `extent.below()`** (in `tangent`).
  The one row there that reads the extent is the torus's axis fork,
  `pt_axis_in_plane`, which takes its Zero for the meridian pose, so the
  contract's bound is from above. The tangent arm is reached only where
  the file read a plane tangent to a torus along a parallel; the plane
  then stands normal to the axis and that row is Positive at any extent
  past the band. Probed with a plane capping a torus: tilting the plane
  moves `pt_axis_normal`, levered at the torus's own `R + r`, and never
  a reading at the extent.
- **`above` without the cone apexes' reach.** No reader of `above` has a
  cone arm: `coaxial_tangent_circle` serves cylinder×sphere,
  sphere×torus, cylinder×torus and plane×torus.

Either stops being unkillable the day a reader of `above` gains a cone
arm or a reachable pose near the axis fork; whoever adds one owes the
row then.

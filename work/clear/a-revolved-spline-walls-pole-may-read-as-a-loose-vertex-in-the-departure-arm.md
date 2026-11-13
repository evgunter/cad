---
id: a-revolved-spline-walls-pole-may-read-as-a-loose-vertex-in-the-departure-arm
kind: issue
title: A revolved spline wall that reaches the axis has a chart that collapses at its pole with no null edge, and the departure arm reads a collapsed side only on a cone or a sphere, so its polar cells may split without end
status: open
opened: 2026-11-12
priority: P2
cost: E
---


## Finding

From CLEAR's lane clear-pairs, the sweep behind
`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`.
That lane taught `present_chart` (`crates/topo/src/injective.rs`) to
read a sphere's pole as a collapsed side of its chart, as it already
read a cone's apex. A cell holding the pole is near both seam
meridians and the pole; read as a loose vertex, the pole left every
polar strip of one half-band against every one of the other to split
until the walk ran out of pairs.

The sweep's hit list, chart collapses that carry no null edge:
- a cone's apex: read (`nappe`, the apex arm);
- a sphere's pole: read since that lane;
- a full revolve's pole on a SPLINE wall (a profile segment that is
  not a line or an arc, ending on the axis): `revolve` mints one pole
  vertex where every meridian chain meets (`Revolved::poles`,
  `crates/sweep/src/revolve/mod.rs`) and no null edge, and the
  wall's chart collapses there. Not read, and not measured: no tour
  body that runs out of pairs has one, and no fixture was built.
- a torus has no collapse while it is a ring; a spindle sheet is
  refused by kind.

## What a fix needs

First a fixture: a full revolve of a profile whose spline segment
ends on the axis, split into half-bands, asked the injectivity
question. If its polar cells split without end, read the pole as the
sphere's is read: the chart line where the wall's radius vanishes, a
collapsed side whose vertex is decided at the pole.

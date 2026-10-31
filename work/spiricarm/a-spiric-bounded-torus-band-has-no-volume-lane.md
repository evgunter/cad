---
id: a-spiric-bounded-torus-band-has-no-volume-lane
kind: issue
title: A spiric-bounded torus band (two loops winding the wall both ways) has no volume lane: the spiric wall refuses LoopsWindChart
status: open
opened: 2026-10-21
priority: P3
cost: M
refs: [5567]
---


## What

The spiric wall lane (`crates/topo/src/props/quad_lane.rs`,
`spiric_wall`; `crates/geom-brep/src/props/spiric.rs`, `wall_windings`)
reads every loop of a torus face through its lifted chart images. It
requires each loop to close on the lift and wind the torus zero times,
because the wall's Green form, summed over the loops, bounds a region of
the lifted chart only then.

A band of the wall bounded by two loops that wind the tube once each
way is a valid face: `geom_brep::props` `closure::require_boundary_windings`
admits that winding set as a boundary, as the planar and cylinder lanes
read it. The spiric wall refuses it `LoopsWindChart`, typed. It is not
misread.

## Witness

`crates/geom-brep/src/props/spiric.rs`,
`a_ring_on_a_wall_is_decided_closed_on_its_own`: an oval winding the
tube once beside the same oval run back. No body in the suite reaches it
yet: the plane × torus spiric poses keep rings that wind nothing.

## What closes it

Read a band on the lift: the Green form over a winding pair takes a
correction per winding loop (the cylinder's rim `Δu` terms are the
model), or the band is cut along a meridian into a face that closes.
Then a body row: a spiric-trimmed torus band through a public op, held
to its oracle.

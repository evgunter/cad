---
id: joinable-reads-a-chart-singularity-by-distance-alone
kind: issue
title: joinable keeps a pole or an apex a vertex by its metric distance from the chart's singular set alone; no structural backstop holds it
status: open
opened: 2026-10-07
priority: P3
cost: M
design: true
---

## The finding

`joinable` keeps a cone's apex, a sphere's poles and a torus's points
on its axis vertices by one metric reading, the vertex's distance from
the chart's singular set (`edge_join::singular_at`, predicate
`join_regular_point`), on every arm. Nothing structural backs it: a
full revolve's pole is a valence-2 vertex between two meridians of one
surface key, every structural reading the chart arm takes holds there,
and the join is refused on the distance alone
(`sweep` row `a_pole_and_an_apex_join_nothing`). A wrong surface datum
or a mis-placed vertex point would join a pole silently, and the
joined edge would run through the chart's singularity, where no image
runs one branch.

## What it needs

A structural backstop that does not read the vertex's point: a tier-2
check that no edge's chart image passes through the chart's singular
set, or a pole marked as such by the construction that mints it (the
revolve's `MeridianVertex` at the axis), read by `joinable` before the
metric reading.

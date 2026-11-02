---
id: wrap-tip-incidence-is-read-at-mint-and-again-at-tier-3
kind: issue
title: a wrap tip's incidence on the chart singularity is read twice: singular_at at the mint, and chart_singularity::locate's on-boundary decision at tier 3
status: closed
opened: 2026-10-30
priority: P1
cost: E
rides_with: tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity
closed: 2026-10-31
pr: 6097
---


Whether a vertex lies on its chart's singular point (a sphere's pole, a
cone's apex) is decided twice, by two predicates over one fact:

- at the mint, `topo::pcurves::singular_at` decides the vertex's distance
  to the nearest singular point as `pcurve_loop_pole_joint`
  (`crates/topo/src/pcurves.rs`, `singular_at`, near line 4065), and a
  joint `Singular::On` resets the loop's lift there;
- at tier 3, `geom_brep::chart_singularity::locate` decides the same
  distance as `chart_singularity_on_boundary` (`on_boundary`, near line
  312 of `crates/geom-brep/src/chart_singularity.rs`), and `Lies::AtVertex`
  is what check 6's chart-singularity arm admits.

Inside the band the two can disagree: a mint that resets at a tip tier 3
places off the vertex, or the reverse. Under `[one-way.readings]` one
question has one reading; the wrap tip (`[topology]`, the seam clause:
"the singularity is that edge's tip") is where both are read for the same
vertex. Found while landing the surface boundary / chart image unit of
`loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk`; not
fixed there.

Signed: (RESTCURVE implementer)

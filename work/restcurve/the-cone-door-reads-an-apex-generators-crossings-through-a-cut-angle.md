---
id: the-cone-door-reads-an-apex-generators-crossings-through-a-cut-angle
kind: issue
title: The cone door reads a generator's crossing with a conic edge through an angle with a cut, which no row reaches at an apex
status: open
opened: 2026-11-01
priority: P3
cost: E
refs: [the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar, torus-windows-a-hair-apart-in-tube-angle-are-undecided-at-the-certified-scalar]
---

## What

Tier 3 reads a cone apex's side through the cone door
(`crates/topo/src/validate.rs`, check 6's chart-singularity arm, through
`boolean::cycle_side`), which at the apex walks generators aimed at the
boundary (`crates/topo/src/boolean/ruled_region.rs`, `contains`'s
`at_apex` arm). A generator is a line leg, placed by a dot product, so
the sphere door's antipode reading has no twin there. But a generator's
crossing with a circle or ellipse edge is read in the edge's parameter
through two angles with a cut, the class the sphere door's fix retired
there (`the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar`):

- the first-harmonic root door's phase,
  `circle_roots::harmonic_roots`' `sin_part.atan2(cos_part)`, a whole
  turn's hull at `Interval` where the meridian plane's normal is a half
  turn from the conic's `u_ref`;
- `Conic::angle` (`ruled_region.rs`), `near_span` of an `atan2`, a whole
  turn's hull where the point lies a half turn from its edge's middle.

Each hull is read against the edge's span on
`bool_ruled_region_span`, in band. A path that reads one is set aside,
and the apex refuses only where every aimed generator does. No row
reaches that at an apex: the sweep, topo, step-import and step-export
suites at ε 1e-6, the default and 1e-12, every interval row included,
raise no `ChartSingularityUndecided` at an apex after the switch.

## Owed

Read the generator's crossings as the sphere door now reads its rays'
(`circle_roots::first_harmonic_root_directions`, and the span by the
unsigned angle from the edge's middle, `region::span_from_middle`), or
pin a row whose apex every aimed generator reads at a cut and show it
settles. Filed by RESTCURVE's pole-door lane.

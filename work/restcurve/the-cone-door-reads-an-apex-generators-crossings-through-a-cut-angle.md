---
id: the-cone-door-reads-an-apex-generators-crossings-through-a-cut-angle
kind: issue
title: The cone door reads a generator's crossing with a conic edge through an angle with a cut, which no row reaches at an apex
status: open
opened: 2026-11-01
priority: P3
cost: E
refs: [the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar, the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band, torus-windows-a-hair-apart-in-tube-angle-are-undecided-at-the-certified-scalar]
---

## What

Tier 3 is to read a cone apex's side through the cone door
(`the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band`
owes that switch, through `boolean::cycle_side`); the boolean reads a
point at an apex through it today. At the apex the door walks generators aimed at the
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
reaches that at an apex: with the switch built, the sweep, topo,
step-import and step-export suites at the default ε (and the six interval
rows of the sphere door's item) raised no `ChartSingularityUndecided` at
an apex. The switch's lane reads this again before it lands.

## Owed

Read the generator's crossings as the sphere door now reads its rays'
(`circle_roots::first_harmonic_root_directions`, and the span by the
unsigned angle from the edge's middle, `region::span_from_middle`), or
pin a row whose apex every aimed generator reads at a cut and show it
settles. Filed by RESTCURVE's pole-door lane.

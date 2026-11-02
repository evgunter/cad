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

Tier 3 reads a cone apex's side through the cone door
(`boolean::cycle_side`, since
`the-sphere-door-has-no-two-leg-path-so-a-pole-beside-a-slim-lune-reads-in-band`),
as the boolean reads a point at an apex through it. At the apex the door walks generators aimed at the
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

## Evidence off the apex (PR 6252's review)

The same reads refuse off the apex too: the review's differential of
`curved_face_containment` on cone bodies found 156 `Interval` readings
escalating on `bool_ruled_region_span` where `f64` decides.

## Owed

Read the door's conic crossings as the sphere door now reads its
rays': each root as a direction with its raw angle
(`circle_roots::first_harmonic_root_directions`), and against its
edge's span at the nearer end (`region::span_at_nearer_end`), the raw
root less the end's parameter within a quarter turn of it and the
half-width less the unsigned angle from the middle past that. Pin the
off-apex escalations above as rows, and an apex whose every aimed
generator reads at a cut. Filed by RESTCURVE's pole-door lane.

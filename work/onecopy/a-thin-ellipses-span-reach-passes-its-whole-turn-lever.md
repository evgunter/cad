---
id: a-thin-ellipses-span-reach-passes-its-whole-turn-lever
kind: issue
title: span_reach_hunt: a thin ellipse's span reach passes its whole turn's lever (seed 0xd54786f7cb0d235c)
status: open
opened: 2026-11-03
---


Found by `onecopy/angle-reads`' local full geom-brep run: the
counterexample-search fuzz
`crates/geom-brep/tests/span_reach_hunt.rs`
`a_conic_arcs_span_reach_is_never_short_of_its_farthest_point` fails its
third assertion, `span_reach_from(p) <= lever_from(p)` ("passes the whole
turn"), reproducibly with `CAD_FUZZ_SEED=0xd54786f7cb0d235c
CAD_FUZZ_EFFORT=1` (`--features test-support`):

```
Ellipse(Ellipse { frame: Frame { center: (0, 0, 0), axis: (-0.5572965940438884, 0.28438696616547604, 0.7800926609975752), u_ref: (0.6166163900257462, -0.48744344408386014, 0.6182096055313937) }, semi_u: 0.0014142132088478148, semi_v: 1.0 }) over [6.778616771278262, 18.0108665539107] from (0, 0, 0)
```

A thin ellipse (eccentricity 0.999999) over a span longer than a turn,
read from its centre. Neither side of the comparison reads anything the
angle-reads unit changed (`geom_brep::extent`'s `cover_far` and
`whole_cover`, `geom::cover`), so it is taken to be on main; the
branch's disk could not hold a second build to confirm on main itself.

## Fix

Pin the counterexample as a deterministic row, then find which of the
span's cover and the whole carrier's cover over-reads (a span over a
turn should read no farther than the whole carrier's).

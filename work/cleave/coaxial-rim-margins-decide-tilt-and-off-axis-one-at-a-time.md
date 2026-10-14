---
id: coaxial-rim-margins-decide-tilt-and-off-axis-one-at-a-time
kind: issue
title: coaxial_margins decides a rim's tilt and its centre's off-axis distance one at a time, not their sum
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

`topo::boolean::surface_group`'s `coaxial_margins`
(`crates/topo/src/boolean/surface_group.rs:71`) returns the rim's tilt
levered at its radius and its centre's off-axis distance as two
margins, and every reader decides them one at a time and serves a rim
when both are Zero: `WrapRims::holds` (`bool_wrap_rim`,
`surface_group.rs:96`), `is_wall_rim` (`bool_wall_iso_rim`, with a
radius row, `boolean/solid_contain.rs:1472`) and the sphere latitude
rim (`bool_sphere_iso_rim`, `solid_contain.rs:3251`).

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

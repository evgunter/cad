---
id: props-rim-incidence-decides-axis-and-centre-one-at-a-time
kind: issue
title: props' require_rim_incidence decides the rim's axis and centre one at a time, not their sum
status: open
opened: 2026-10-07
priority: P3
cost: E
---

## What

`geom_brep::props::curved`'s `require_rim_incidence`
(`crates/geom-brep/src/props/curved.rs:999`) requires
`props_rim_axis_parallel` (levered at `r_c`) and
`props_rim_center_on_axis` Zero as two rows, and together they certify
that the rim lies on the surface.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

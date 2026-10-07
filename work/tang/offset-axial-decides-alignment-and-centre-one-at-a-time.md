---
id: offset-axial-decides-alignment-and-centre-one-at-a-time
kind: issue
title: offset_axial decides the axis alignment and the centre-on-axis one at a time, not their sum
status: open
opened: 2026-10-07
priority: P2
cost: M
---

## What

`topo::offset_axial`'s `classify` (`crates/topo/src/offset_axial.rs:969–1036`)
decides `offset_axial_alignment` (the axis sine levered at
`frame.extent`) and `offset_axial_centre` (`centre_on_axis`, the
radial distance) as two rows, and serves a coaxial `Constraint::Wall`,
`Generator`, `Ball` or `Torus` when both read Zero.
`latitude_posture` (`offset_axial.rs:2157`, callers near 1825 and 2102)
does the same with `centre_on_axis` and `offset_axial_latitude_tilt`,
and `latitude_circle` mints from it. Each row can sit just inside the
zero band, so the served constraint stands up to about `2ε` off.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

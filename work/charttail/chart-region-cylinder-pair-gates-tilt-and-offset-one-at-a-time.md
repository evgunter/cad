---
id: chart-region-cylinder-pair-gates-tilt-and-offset-one-at-a-time
kind: issue
title: chart_region's cylinder_pair_overlap gates the axes' tilt and offset one at a time, not their sum
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## What

`topo::chart_region`'s `cylinder_pair_overlap`
(`crates/topo/src/chart_region.rs:1451–1460`) gates
`chart_region_cyl_radius`, `chart_region_cyl_tilt` (levered at `hyp`)
and `chart_region_cyl_offset` separately and serves the chart transfer
when all read Zero. The halved gate band on `Bridged` narrows each row
but does not sum them.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on. The same rows
also read each cylinder's stored origin off the reach: TANG's parked
`declared-cylinder-pair-offsets-read-off-the-reach` (`work/tang/`)
holds that half, and a fix here should land with it or after it.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

## A second site in the same file

`proper_crossings`' parallel lane (`crates/topo/src/chart_region.rs:3058`)
has the same shape in parameter space: `chart_region_parallel` (the
segments' sine levered at the longer one) decided Zero, then
`chart_region_collinear_offset` (the larger endpoint offset) decided Zero
on its own, and the collinear lane served. A point of either segment
stands off the other's line by up to the offset plus the sine levered at
its length. Found by the sweep of
`section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time` (branch
`cleave/section-cert-axis-pose`).

---
id: a-point-is-read-on-a-marched-loop-by-distance-not-by-its-tube
kind: issue
title: A point is read on a marched loop by its distance within the pad, where membership in the loop's tube is what limb 3 proves
status: open
opened: 2026-10-31
priority: P4
cost: M
---


Filed by SECT's `sect-onesix` lane, from the PR closing
`main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot`.

## Measured

`CarriedComponent::reads` (`crates/geom-brep/src/pair_section.rs`)
decides that a point lies on a marched loop when the point's foot on
the carrier is within the band padded by `pad`, the farthest point of
the loop's uniqueness tube (`Tube::of`). The tube is the hull boxes
padded by the tube radius. Limb 3 proves the tube holds this loop's arc
and nothing else, and the march's exhaustiveness puts every point of
the section inside some branch's tube. So "in this loop's tube" is
exactly what the certificate decides, and the distance reading is a
looser stand-in for it. It reads a point of another loop as within the
pad wherever a slab's far corner sits at that distance, inside the
tube's hull but outside its boxes.

At ε 1e-6 on `two_loops_keep_their_sites_and_order_near_the_node`
(`crates/geom-brep/tests/pair_section.rs`), the pose the row escalated
at under the hull-diameter bound is `d 0.2, R 0.701`. There the
escalation band was 0.1098 m (`curve_carrier_straddle`). That pose now
reads a tube radius of 5.5e-3 m, pads of 0.044 and 0.048 m, and
reaches of 0.035 and 0.036 m. The other sheet stands at least 0.100 m
from the carrier, so there is room of about 2×. The widest pads in the
row are at `d 0.1, R 0.7`: a tube radius of 4.4e-2 m and pads of 0.099
and 0.103 m. That pose passed under the old bound as well.

So no measured row needs this reading today. It is a tightening of
the same claim, worth having where two loops come within a pad of each
other.

## What closing it takes

Read a point against the loop's tube boxes padded by the band: on
where some box padded by the zero band holds it, off where no box
padded by the escalation band does, and in band otherwise. The foot is
then read on the carrier as now. The marched component carries its
boxes. Then measure what it moves: the germ join's site feet and the
counted cross-check read `reads`.

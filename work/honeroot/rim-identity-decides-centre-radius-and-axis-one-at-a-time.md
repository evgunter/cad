---
id: rim-identity-decides-centre-radius-and-axis-one-at-a-time
kind: issue
title: rim_wedge's same_circle decides a rim's centre, radius and axis one at a time, not their sum
status: open
opened: 2026-10-07
priority: P2
cost: M
---

## What

`topo::boolean::rim_wedge`'s `same_circle` (`crates/topo/src/boolean/rim_wedge.rs:160`,
mirrored by the test helper `rim_identity` near 1192) decides
`rim_circle_radius`, `rim_circle_center` and `rim_circle_axis_parallel`
(levered at `ra.radius + rb.radius`) as three rows, and reads "one rim"
when all three are Zero: two rims each term just inside the band apart
stand up to about `3ε` apart.

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: centre, radius and axis decided as three rows: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEROOT (`work/honeroot/`) by `git mv`, id and body unchanged: whose charter is the boolean's root doors and sector readers and their margins. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

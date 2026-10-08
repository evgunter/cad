---
id: section-cert-axis-pose-decides-tilt-and-offset-one-at-a-time
kind: issue
title: section_cert's axis_pose decides the axes' tilt and offset one at a time, not their sum
status: open
opened: 2026-10-07
priority: P2
cost: M
---

## What

`topo::boolean::section_cert`'s `axis_pose`
(`crates/topo/src/boolean/section_cert.rs:745`) decides
`section_axes_tilt` (levered) and then `section_axes_offset`; both Zero
gives `Pose::Coaxial`, which serves the torus×cylinder and torus×torus
coaxial section certificates from the exact coaxial formula with no
allowance for the tilt or the offset (near 668 and 692–699).

Found by the sweep of the TANG unit that closed
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` (`work/tang/`),
and filed on this slate, whose ground it lands on.

## The shape of a fix

Decide the served verdict on one margin carrying both terms, as the
section classifiers' `decide_across` (`crates/geom-brep/src/intersect.rs`)
and `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`) do:
the zero side on `|datum| + tilt·lever`, a definite side on the datum
shrunk toward zero by the tilt, the tilt row kept only to route.

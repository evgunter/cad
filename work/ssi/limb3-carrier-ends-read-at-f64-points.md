---
id: limb3-carrier-ends-read-at-f64-points
kind: issue
title: limb 3's end checks read the carrier's ends as f64 point evaluations, not enclosures of the exact ends
status: open
opened: 2026-10-04
priority: P3
cost: E
refs: [limb3-at-rest-proves-the-graph-not-the-arc]
---


## Found (review of PR 4012, nit; filed by the limb-3 lane, 2026-10-04)

`certify_branch` (`crates/geom-brep/src/ssi/certify.rs` ~1368) reads
the carrier's two ends as `carrier.eval(t)`, crossed with
`Interval::from_certified`. On the f64 lane that is the rounded point,
not an enclosure of the exact end. Limb 3's end checks
(`one_arc::reaches_end`, `one_arc::r3_reaches_end`) then read "within ε
of the end" against that point. The rounding is far below ε, so no
verdict moves at today's bands. Still, the end the check certifies is
not the one the carrier has.

## Repair shape

Enclose each end through the carrier's interval evaluation (the
certification door the limbs already use), and pass the box on.

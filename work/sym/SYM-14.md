---
id: SYM-14
kind: unit
title: the chain: four links joined with an angular error at each joint, its dispersion drawn like the plate's, and the certified lane measured on it
status: dispatched
opened: 2026-09-22
priority: P1
cost: H
branch: sym/14-chain-demo
refs: [a-widened-rotation-angle-is-unmeasured-on-the-certified-lane]
---


## What

**Requested by Ev, in chat, 2026-09-22 — P1 specifically requested:**
"a demo of error propagation like with the two holed plate but it's a
chain of 4ish elements joined with some error in the angle at each
join and so it will be visibly be more and more dispersed going down
the chain"; "if it is possible, or will be possible after some of your
planned work lands, take it on as a unit". It is possible today on the
advisory lane (the `f64` Monte-Carlo replays that draw the plate's
density sheet) through public doors — `Node::Transform` with an
`Angle`-dimensioned parameter carrying a `Distribution`, the built
bodies read back, the tour drawing the SVG itself — so the unit builds
the chain document once (`demos/tour/src/chain.rs`), draws its
dispersion sheet beside the plate's, and measures the certified lane
(`Interval`, `Sym<Interval>`, the drive) on the same document, filing
every wall at P1 with Ev's request carried: a widened rotation angle
has never been measured on the certified lane, and the derived-frame
family's walls are live. No kernel change. Protocol v7 OUT (a demo and
a measurement): OPUS implementer, one OPUS style review with a
correctness arm, no draw, no ordinal, no row. Spec:
`docs/SYM-14-SPEC.md`.

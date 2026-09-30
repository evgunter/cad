---
id: a-pinch-union-refuses-ring-homing-in-one-member-order
kind: issue
title: Two blocks meeting at a corner on a plate's top refuse RingHomingAmbiguous in one member order and publish in the others
status: open
opened: 2026-09-30
---


## What

Two blocks whose footprints meet at one corner on a plate's top: the
plate `[0,3] × [0,2] × [0,1]`, `p1` = x∈(1.0,1.5), y∈(−1,1), z∈(0.5,2.0),
`p2` = x∈(1.5,2.0), y∈(1,3), z∈(0.47,1.7). The two footprints touch at
(1.5, 1.0) only, so the plate's top is two pieces that meet at a vertex
(a pinch). The union publishes in five member orders and refuses in
`[p2, p1, plate]`:

`Boolean(Join(RingHomingAmbiguous { .. }))`: "a hole loop sits on the
divided face's outer boundary, so which piece holds it cannot be
decided".

The order dependence is the defect: the other five orders build the
same body. Found by the obstacle-mechanism measurement (branch
`emit/borders-mechanism-probe`, `crates/editor-core/tests/borders_probe.rs`,
fixture `pinch`); the refusal is raised by the ring rehoming in
`crates/topo/src/chord_join.rs`
(`SplitJoinError::RingHomingAmbiguous`). The same order-set's
`[p1, p2, plate]` body trips the tessellator's watertight census
(`work/tess/a-pinch-union-body-trips-the-watertight-census-in-one-member-order.md`).

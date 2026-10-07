---
id: measure-is-an-operation
kind: issue
title: D10 stage 2 PR D: a Measure is one primitive defining one scalar variable; its arithmetic is a Defined variable and an Assertion reads a scalar variable (VR4's exception closes)
status: parked
opened: 2026-10-07
priority: P0
cost: M
design: true
blocked_on: [the-product-is-an-explicit-list]
---

INTENT stage 2, PR D. Spec: `docs/INTENT-STAGE2-SPEC.md` §5.

A `Measure` holds one primitive and defines one scalar variable. Measure arithmetic
is a `Defined` variable, which closes VR4's interim exception. `Assertion` reads a
scalar slot. `Node::measure` stays as an authored builder returning the edit list.

`design: true`: FORK-5 (may a geometric slot read a measured value) bounds the
schedule work.

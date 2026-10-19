---
id: decide-across-and-carrier-eq-floor-are-two-spellings
kind: issue
title: the section rows' decide_across and carrier_eq's reach floor are two spellings of one bracket
status: open
opened: 2026-10-07
priority: P4
cost: M
---

## What

`decide_across` (`crates/geom-brep/src/intersect.rs`) decides a
datum read across the reach as `|datum| + swing` on the zero side and
the datum shrunk by the swing on the definite side.
`topo::boolean::carrier_eq` spells the same bracket by hand for each
kind (`reading`: `carrier_cyl_reach` with `_floor`, the sphere and
torus reaches) and decides it in `declared_reading`. Two spellings of
one rule drift.

## The shape of a fix

Give the bracket one home (a geom-brep helper both crates read), with
each kind's swing derived at its own site.

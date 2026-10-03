---
id: torus-wall-fixture-reverses-a-parallel-by-hand
kind: issue
title: The torus_wall test fixture spells a reversed parallel by hand rather than through Curve3::reversed
status: open
opened: 2026-10-03
priority: P4
cost: E
refs: [JOIN-3]
---


## What

`crates/topo/src/boolean/boxes.rs`, the test fixture `torus_wall`: its
descending parallel is the ascending circle with `axis: -axis` and its
`u_ref` moved to the arc's far end, a hand spelling of "the same circle
run back". JOIN-3 gave the reversal one home, `geom::Curve3::reversed`
(θ ↦ −θ about the flipped axis), read by every `chord_join` site; found
by JOIN-3's dual review (Q1). The fixture is not JOIN's ground, so it is
noted here rather than edited.

## Fix

Build the descending parallel as the ascending one's `reversed()`, its
interval `[−u1, −u0]`, if nothing reads the moved `u_ref`.

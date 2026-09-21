---
id: the-shape-door-could-take-the-sense-free-rim-side-residue
kind: issue
title: props: require_iso_rectangle admits a face whose rims encode different material sides; the sense-free residue unanimous_rim_side already decides it in the gate arm
status: open
opened: 2026-09-16
priority: P0
cost: D
---


Filed by the PROPS sphere-pole-side fix pass (PR 2741), answering the
dual review's question "can the shape door take the second rectangle
predicate cheaply?" — measured: it can take a WEAKER form of it, and
that form is already written.

## The measurement

`require_iso_rectangle` asks ONE predicate (`props_rim_level`: every
rim sits at an extreme). The sphere's flux lane asks two — the second
is `props_rim_interior_side`, every rim's interior side points INTO the
folded extent. **The door cannot take the second**: σ is the rim's
traversal under the face's SENSE bit, and the door is handed a surface
and a loop with no face, deliberately, so that it answers a question
about the boundary alone.

What it CAN take is the sense-free residue: **every rim encodes the
same material side**. `unanimous_rim_side` (`props/curved.rs`) is that
predicate, written for `boundary_material_sign` in this same PR, and it
needs no bit. Executed there: a sphere face carrying a rim at `lo` and
a rim at `hi` traversed the SAME way — R2's staircase face — read
`Encoded(Positive)` with the lower rim first and `Encoded(Negative)`
with the upper rim first before the change, and is an `Err` after it.
The shape door still answers `Ok(())` for that face.

## Why it was not taken here

Scope and blast radius, not principle. The door is cited by `mesh`'s
walk and by `topo`, so adding a refusal to it changes which bodies
mesh, and this unit's fence is the sphere's flux lane. It is a
strictly-stronger premise on a door whose whole job is that premise, so
the change is a small diff and a large test surface — its own unit.

## What it would NOT close

The divergence the door's own docs now record: the **L-shaped
complement of a half-cap** has ONE rim, so there is nothing for a
unanimity rule to compare, and no sense-free door can tell it from the
half-cap. That residue is `props_rim_interior_side`'s alone and stays
with the flux lane.

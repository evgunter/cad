---
id: an-inside-out-slab-thin-enough-for-its-volume-to-be-in-band-finishes-at-rest
kind: issue
title: An inside-out slab thin enough that its V/A is in band finishes at rest, so the side it bounds is the complement's
status: open
opened: 2026-10-11
priority: P2
cost: M
design: true
---



Found by CLEAVE's missed-ray unit (branch `cleave/missed-ray-side`),
measured at ε 1e-9.

## Measured

`AtRestBody::validate(brick((0,1),(0,1),(0,t)).revert())` returns
`Validated` for every `t ∈ {10, 12, 15, 20}ε`. The reverted slab bounds
the slab's complement, so its signed volume is `−t` and its `V/A` is
about `−t/2`, which is in band. Check 7, D1's positive-volume
orientation invariant, refuses only a definitely negative volume.
Its "zero and escalated exempt" clause passes this one
(`validate.rs` `plus_v_by_sign`). So `inside_out_operand.rs`'s rule,
that an inside-out body is not a finished body, holds only where `V/A`
is definite.

## What it costs

A Boolean that takes this operand reads it as the complement it
bounds. On that branch `point_in_solid` reads the side by a ray aimed
at a planar face, off the faces' own outward normals, and answers `In`
far from the reverted slab
(`crates/topo/tests/missed_ray_side.rs`). That is right for the body
as stored, and wrong for the solid it was meant to be.

## The question

What does check 7 read for a shell whose `V/A` is in band?

- **Today's exemption.** Orientation is a construction invariant that
  the probe checks only where it can measure it.
- **A crossing ray.** Read the orientation off a ray from outside the
  shell aimed at a planar face, as `solid_contain::aimed_side` does. A
  thin shell's two faces are a definite pair, and the ray reads the
  nearer face's outward normal.

This ties to FLUSH's `check-10-reads-a-cavitys-role-from-its-volume-sign`,
which asks the same question of a cavity's role.

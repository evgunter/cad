---
id: an-inside-out-slab-thin-enough-for-its-volume-to-be-in-band-finishes-at-rest
kind: issue
title: An inside-out slab thin enough that its V/A is in band finishes at rest, so the side it bounds is the complement's
status: open
opened: 2026-10-11
priority: P1
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

## Measured (PR 4725's review and fix pass, 2026-10-11)

**The at-rest gate admits an operand the Boolean must refuse.** With
the missed-ray read and no operand check, at ε 1e-9, `rev =
slab(t).revert()` finishes `Validated`, and against a brick far away:
- `brick ∩ rev` built the brick;
- `brick ∖ rev` was empty;
- `rev ∖ brick` refused `ResultInvalid { NegativeVolume }`, which reads
  as a kernel defect.

Main had refused all three on the in-band volume read. The ray reads
the operand as holding infinity, and every answer then follows the
complement.

**Handled at the Boolean, not here.** On PR 4725 the Boolean's
operand gate (`boolean/reduce.rs` `gate_sliver_orientation`) reads
each solid the at-rest gate passed as a sliver
(`AtRestBody::slivers`). It reads the solid at a point outside its
faces' certified boxes. If the material holds that point, the gate
refuses `InsideOutOperand` with
`ValidationError::SliverInsideOut { solid }`. That is pinned at every
ε for every op in both orders, against a far brick and against a brick
across the slab
(`crates/topo/tests/missed_ray_side.rs`
`an_inside_out_slab_is_refused_as_an_operand`).

The reverted thin hollow does not need this: check 10 refuses it at
rest (`ShellWinding` on both shells, at `12`–`18ε`, at every ε).

**Second review (same day).** One far point is not enough: a reverted
thin tube (profile `[r, r+t] × [0, 1]` revolved, `t = 20ε`, ε 1e-12)
read in band at the gate's point and passed, and `brick (−20,20)³ ∖
revtube` built the tube's own volume. A reverted slab at `Dual64`
skipped the gate. Fixed at the root on PR 4725: every Boolean read of
an operand's side at infinity (`SolidFaces::read_as_given`) refuses
an `In` as `PointInSolidError::InsideOut`, typed `InsideOutOperand`.
The gate now also reads every solid of a dual operand, and refuses
typed where its reading does not decide.

**Still open here:**
- Every door that takes an at-rest operand and does not read its
  orientation still admits it. The split is one.
- The question above is whether check 7 should read a sliver's
  orientation by a ray, so that `AtRestBody` never carries one. That
  would retire the Boolean's read.

## Measured (FLUSH `flush/shell-role-reader`, 2026-10-14)

Ev ruled check 7 onto the reader of a shell's role (PR 4990; DESIGN D1
tier 3). On that branch `AtRestBody::validate(slab(t).revert())`
refuses `[NegativeVolume { solid }]` at ε 1e-9 and 1e-6 for every
`t ∈ {10, 10.5, 12, 15, 19}ε`: a ray aimed at 60° at one of its faces
crosses its two faces `2t` apart and enters its material first.
`crates/topo/tests/shell_role_reader.rs`
`a_lone_slab_is_outer_and_inside_out_refuses_at_rest` pins it;
`missed_ray_side.rs` `an_inside_out_slab_refuses_at_rest` replaces the
`Validated` pin. The Boolean's operand gate for slivers
(`gate_sliver_orientation`) is retired with it. A dual's operand gate
reads the same check 7 (`AtRestBody::gate_unverdicted`'s
`wound_negative`), and refuses the slab `InsideOutOperand { NegativeVolume }`.

This row closes with that PR; RESTFRONT closes it. Its "still open"
note about other doors that take an at-rest operand no longer applies:
no inside-out slab finishes at rest.

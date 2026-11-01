---
id: the-door-gates-other-in-band-findings-are-typed-the-kernels
kind: issue
title: The finished-body gate types its other undecided findings the kernel's: a point margin in band at PlanarFace, Ring, PinchCorner, SliverDihedral or JoinUndecided refuses ResultInvalid
status: open
opened: 2026-10-09
priority: P1
cost: M
refs: [4415, a-nested-brick-k-eps-from-a-far-carrier-escalates-at-the-result-gate]
---


## What

Found by the door-typing unit (branch `join/door-types-in-band-results`).

D10's Booleans clause types a finding born of an in-band margin as the
operands' ill-conditioning (`BooleanError::Escalated`) and a definite one
as a kernel defect (`ResultInvalid`). `boolean/ops.rs` `finding_arm`
states every finding's arm. Only the shell role takes the in-band arm,
because only its margin is certified: the role read re-derives `V/A` in
interval arithmetic and says when that enclosure lies wholly in a sliver
band (`ValidationError::ShellRoleUndecided::sliver`).

The other undecided findings the gate can raise carry a point margin read
in `f64`: `DegenerateTorusEscalated`, `PlanarFaceEscalated`,
`PlanarBoundaryEscalated`, `SliverDihedral`, `JoinUndecidedAtRest`,
`VolumeSignUnresolved`, `RingContactEscalated`, `RingNestingUndecided`,
`RingPairContactEscalated` and `PinchCornerEscalated`. A point margin in
band is the geometry's only where its formula meets Q1's conditioning
premise (a few ulp at the model's extent). Nothing shows that yet for
these, so they stay `ResultInvalid`.

**Evidence.** `near_tangent_census_probe` at ε = 1e-9, d = 1e-8: three
runs refuse `ResultInvalid { RingContactEscalated }` at the gate:
`vee300 nt e1 a11 d1e-8` pc U, cp U and cp S. Each has margin −2.2566e-9
(`ring_outer_meet_side`), in band. `near_tangent_battery` refuses six
more the same way (`vee300 nt e0 a0 d1e-7 face` ac U, ca U, ca S, and
`e1 a4` likewise). TALLY's
`a-nested-brick-k-eps-from-a-far-carrier-escalates-at-the-result-gate`
was read as the same finding on a brick, and was not: there check 9
read an in-band side of two edges far apart along one line, a
misreading rather than conditioning, and reading their gap first
builds all six (closed by PR 5492).

**Two sibling sites read the same kind of finding, untyped.**
- **The split.** `splitting/mod.rs`'s finished-body gate maps every
  finding to `SplitFinishError::ResultInvalid`, the shell role
  included. D10's Booleans clause does not name the split, so whether a
  split's in-band shell is the operands' ill-conditioning is open there.
- **The pieces sort at the door.** `pieces.rs` reads the same shell role
  before the gate. Where a solid holds two decided `Outer` shells beside
  an unread one, it refuses `PieceSortError::RoleUnread`, which is
  `BooleanError::Pieces`. It drops the role read's refusal, so it cannot
  say whether the shell was a certified sliver. No probe run reaches
  this: 0 `Pieces` refusals over 109 440 runs.

## The shape to give

For each finding, either show its margin conditioned and type its
in-band arm `Escalated` in `finding_arm`, or give it a certified reading
the way the shell role has one. The census findings join this when the
door runs the census (`boolean-door-runs-the-census-over-its-result`).

**More evidence (CLEAVE, 2026-10-10).** Take the unit ball against
`ball(50)` at `d = 51 − 2e-6`, ε = 1e-9
(`crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
`a_large_ball_near_external_tangency_is_pinned`). Both `a ∪ b` and
`b ∪ a` refuse `ResultInvalid { VolumeUncomputable }`. The face's
`props_rim_level` enclosure is [0, 1.22e-9] (or [0, 1.24e-9]), in band,
`check: Inventory`. The gate types `VolumeUncomputable` as a lane that
cannot run.

Whether this margin is the geometry's is unshown. The rim sits about
2e-3 rad from the re-charted pole, which is the
`sphere-rim-level-cosine-cancels-near-a-pole` (FLUXTAIL) neighbourhood.
So it joins this inventory as another undecided finding typed the
kernel's.

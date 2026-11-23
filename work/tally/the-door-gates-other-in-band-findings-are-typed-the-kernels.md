---
id: the-door-gates-other-in-band-findings-are-typed-the-kernels
kind: issue
title: The finished-body gate types its other undecided findings the kernel's: a point margin in band at PlanarFace, Ring, PinchCorner, SliverDihedral or JoinUndecided refuses ResultInvalid
status: closed
opened: 2026-10-09
priority: P1
cost: M
refs: [4415, a-nested-brick-k-eps-from-a-far-carrier-escalates-at-the-result-gate]
closed: 2026-11-22
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

## Re-measured on main at `fd7e5f47bf` (TALLY, 2026-11-22)

**The typing this row asks for is gone.** PR 6371 (`onehome/finished-body`,
implementing Ev's PR 6326 ruling, 2026-11-02: *"i don't really want to
reify 'in-band' vs 'everything else'"*) deleted `finding_arm` and
`AtRestRefusal`. `boolean/ops.rs` `gate` maps every finding to
`ResultInvalid { findings: NotFinished }`, and each finding's own ending
types it (`[topology.tiers]`; `validate.rs` `own_close`,
`undecided_scalar_recourse`). D10's Booleans sentence still names
`Escalated` for the gate's in-band refusal.

**The row's evidence no longer reproduces.**
- `near_tangent_census_probe`, `NT_D=1e-8 NT_ONLY=vee300 NT_A=11`: all
  `e1` runs build `OK SOUND`.
- `near_tangent_battery`: 7 200 runs, 0 `ResultInvalid`; the six
  `vee300 … d1e-7 face` runs build `OK SOUND`.
- `a_large_ball_near_external_tangency_is_pinned` passes: both unions build.

**What still refuses at the gate in band.** The full census probe at its
default tilts (28 884 runs) refuses `ResultInvalid` six times, every one
`PinchCornerEscalated` on `pinch_corner_turn`, all at `d = ±1e-9`, `cp S`:
`Ltop e0 a10 d1e-9` (1.2185e-9), `Ltop e0 a11 d±1e-9` (±3.2966e-9),
`Lbot e1 a2 d±1e-9` (±1.2185e-9), `Lmirror e0 a10 d1e-9` (1.2185e-9).
Its in-band point ending is the last resort (`KERNEL_LIMIT_RECOURSE`),
as ONEHOME fork 259 (PR 6820) set for "the undecided pinch corner".

**The siblings.** `split_near_tangent_census_probe` (20 007 lines): no
in-band finding at the split's gate (3 `ResultNotAtRest
{ VolumeUncomputable: NotIsoRectangle }`, a frontier). 0 `Pieces`
refusals in the census probe.

## Closed

2026-11-22 (TALLY orchestrator): superseded. Ev's PR 6326 ruling, carried
out by PR 6371, retired the door-level in-band/definite split this row asked
to extend. A finding is typed by its own ending (`[topology.tiers]`).
D10's Booleans sentence still described the old split, so the PR that closes
this row re-words it to the ruled text. The one in-band refusal still
measured at the gate is now
`an-in-band-pinch-corner-at-the-gate-ends-in-the-kernel-limit-recourse`.
The split's in-band shell is not reached by any measured run
(`split_near_tangent_census_probe`: no in-band finding at the split's gate).

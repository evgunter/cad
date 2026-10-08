---
id: a-near-tangent-intersections-sliver-lump-reads-its-role-in-band-and-refuses
kind: issue
title: A near-tangent intersection keeps the exact answer's sliver lump as its own shell; at a tilt of a few bands its V/A reads in band and the door refuses ShellRoleUndecided
status: open
opened: 2026-10-08
priority: P0
cost: H
design: true
refs: [near-tangent-boolean-results-ship-with-an-escalated-tier-3-census]
---

## What

Found by this program's near-tangent census measurement (`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`, its `## Measured`), on main `047d10d5`, release.

Where the cube's face plane lies d rad inside a reflex corner's edge, the
exact ∩ holds a sliver lump along that edge. The lump meets the main lump
only at `v`. `zip::split_cones` gives it its own vertex and shell, as
`join_pierce_runs_sweep` `a_near_tangent_sliver_is_a_cone_of_its_own`
pins at 200 bands. At a tilt of a few bands, the lump's V/A
(`props::read_role`'s margin) falls in band, and the door refuses
`ResultInvalid { ShellRoleUndecided }`.

- **ε = 1e-9:**
  - d = +1e-8: 34 runs, all ∩, both orders: notch307 4, shallow200 8,
    vee300 12, asym 6, w345 4. The margin is the point value
    `positive_volume`, 1.1e-9 to 4.0e-9.
  - d = ±3e-8: 60 runs, margins 4.1e-9 to 9.8e-9.
- **ε = 1e-6, d = 1e-5:** 34 runs, margins 2.8e-6 to 3.7e-6.
- **Each lump is real.** On 32 of the 34 runs at 1e-8, the oracle's clip
  of one convex piece by the cube is a lump of volume 1.9e-16 to 2.0e-15,
  2.0e-8 to 2.4e-8 thick at its far end, with V/A 1.6e-9 to 3.9e-9. Its
  thickness grows linearly from `v`, so it lies within K·ε over about the
  first half of its length. (On w345 the lump spans two of the oracle's
  pieces.)
- **Witness.** `notch307 nt e0 a3 d1e-8 pc I`. The kernel's shell reads
  V/A 3.05e-9. The oracle's lump has volume 7.69e-16, area 1.99e-7,
  V/A 3.87e-9 and thickness 2.22e-8.

So the role read is right to call the lump undecided at this ε. This is
class (a): the boolean built a lump that is in band of not existing.

At ε = 1e-12 the same refusal has a different cause, filed as ENCL's
`the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell`.

**The stage.** The cone split keeps the sliver as a solid of its own. No
stage asks whether a lump that thin should exist.

The pinch unit measured the same move (see this row's parent, "Measured
(pinch unit)"): its 19 BAD → refusal lines are this.

Repro: `NT_DUMP=1 cargo run -p sweep --release --example near_tangent_census_probe | python3 scripts/oracles/near_tangent_census_classify.py`, with `NT_ONLY`/`NT_POSE`/`NT_D` to pick the pose and `CAD_TOLERANCE_EPS` the row.

## The shape to give

A design question, tied to
`a-near-tangent-split-leaves-a-face-corner-that-runs-within-the-band`. The
options:
- drop the lump: it is in band of empty, and the volume moves by
  about 1e-15;
- refuse the pose at the split, typed;
- keep the refusal as the door's answer, and say so in the door's
  contract.

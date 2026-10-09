---
id: the-forced-order-lanes-could-accept-a-ring-held-run
kind: issue
title: The forced-order join lanes refuse a ring-held run that the outer lane would take
status: closed
opened: 2026-10-07
priority: P3
cost: E
closed: 2026-10-09
branch: join/three-small-join-rows
---


## What

`crates/topo/src/boolean/join.rs` ranks a chord arc's mef run with
`capture_rank`: `Clean`, `RingHeld` or `Separates`.
- The outer lane (`best_arc`) takes a `RingHeld` arc where neither arc
  is clean.
- The two forced-order lanes accept only `Clean`:
  - `ring_order`: "derived ring role order separates a loose
    scaffolding pair";
  - the across-edge match in `choose_roles`: "a match across its
    segment's edge separates a loose scaffolding pair".

The premise that makes `RingHeld` safe in the outer lane holds for a
forced run too. A ring holding the partner of a captured half is
re-homed by geometry (`ChordJoiner::rehome_rings`) to that half's side,
since section segments in one face do not cross. So the forced lanes
are stricter than their rationale: a `RingHeld` forced run refuses
where it could build.

No witness reaches either lane's message today. Neither appears in
PR 4272's batteries (r2 `grid`, `psi`, `tilt` and `rv`, `mnotch`, the
pierce, pinch and corner-pairs batteries, `join1_r1_reflex_battery`
and `rc_wide`), nor in its review's runs. They were left on their old
rule so that PR 4272 moves an outcome only where main refused.

## Owed

Find a pose that reaches either message with a `RingHeld` forced run.
If one exists, relax the lane to accept `RingHeld`, pin the pose, and
measure it against the oracle. If none can be built, close the row.

## Closed — no pose reaches either lane with a `RingHeld` run

Neither lane is relaxed. A probe logged `capture_rank`'s verdict at
both forced-order lanes (`ring_order`, and the across-edge match in
`choose_roles`), and every verdict read `Clean`:

- **The sweep suite** at ε 1e-9, 1e-6 and 1e-12 (2 460 tests each):
  about 10 400 across-edge reads and 17 200 ring-lane reads.
- **The batteries**: `pierce_runs_battery`, `pinch_runs_battery` and
  seven `rc_wide` shards, with 678 ring-lane reads and no across-edge
  read.
- **A targeted search** for faces holding several rings, through the
  public booleans, every op in both orders: 323 poses, 1 938 runs and
  4 342 ring-lane reads.
  - F1: two- and three-pronged combs through a block's top face, blind
    and through, turned 0°, 7°, 33°, 45° and 90° (40 poses).
  - F2: a square tube through the top face, two wall thicknesses,
    blind and through, four turns (16).
  - F3: a plate with two square or two round through-holes under a bar
    over 5 × 4 placements, blind and through, three turns (240).
  - F4: comb prongs through a cylinder's wall, three prong sets, three
    depths, three tilts (27).
  The F1–F3 runs all built. Of the 162 F4 runs, 119 built, 7 answered
  empty (`B ∖ A` where the shallowest comb sits inside the rod), and 36
  refused `VolumeUnmeasured` or `ResultInvalid { VolumeUncomputable }`
  at the measurement, not in the join. Every body built passed tiers 2
  and 3′ and the certificate. The search checked the lanes, not the
  volumes.

No run, anywhere, read `RingHeld` or `Separates` at either forced lane.

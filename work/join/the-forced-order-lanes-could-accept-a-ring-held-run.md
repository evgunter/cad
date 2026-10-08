---
id: the-forced-order-lanes-could-accept-a-ring-held-run
kind: issue
title: The forced-order join lanes refuse a ring-held run that the outer lane would take
status: open
opened: 2026-10-07
priority: P3
cost: E
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

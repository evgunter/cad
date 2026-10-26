---
id: chart-ring-side-refuses-on-a-ray-level-margin
kind: issue
title: chord_join::chart_ring_side refuses the re-homing on an in-band reading of one vertex's ray, where a Zero on the same rows tries the next vertex
status: open
opened: 2026-10-06
priority: P2
cost: E
---


Unmeasured: no fixture is known to reach it. Found by the sweep of
`cleave/ray-walk` (the ray-walk driver unit), which put every
schedule walk in the crate on `ray_walk::walk` and left this one,
because its schedule is the ring's vertices and its exhaustion is an
answer, not a refusal.

`chord_join::chart_ring_side` (`crates/topo/src/chord_join.rs`) casts
one ray per ring vertex up the chart's height axis and counts its
crossings of the run's chart segments. A `Zero` on
`split_ring_chart_ray_azimuth` or `split_ring_chart_ray_height` moves
on to the next vertex (`continue 'vertex`), and a run of vertices all
grazing answers `OnBoundary`. An in-band reading on the same two rows
is `decide_m(..)?`, which refuses the whole re-homing
(`SplitJoinError::Escalated`).

Both rows are about one vertex's ray, not about the ring: the
`ray_walk` argument holds here as it does for the schedule walks, so an
in-band reading should move on to the next vertex too, the first such
reading kept. What it should end in when every vertex is set aside is
the open part: today a run of grazes is `OnBoundary`, an answer, and a
run with an in-band reading among them has no answer to fold into.
The likely shape is the walk's rule — the first in-band reading is the
refusal, else `OnBoundary` — through `ray_walk::walk` with the
`OnBoundary` fold moved to the caller.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: one ray's in-band reading refuses the whole re-homing: margins decided one at a time, or one verdict read off two decisions: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPAIR (`work/honepair/`) by `git mv`, id and body unchanged: the P2 track opened for one verdict decided on one margin at its own reach: rows that decide two margins one at a time, compose two readings, or lever a reading at the wrong extent. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

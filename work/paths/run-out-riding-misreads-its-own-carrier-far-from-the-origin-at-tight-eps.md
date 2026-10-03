---
id: run-out-riding-misreads-its-own-carrier-far-from-the-origin-at-tight-eps
kind: issue
title: PendingRunOut::rides decides a straight run out against a band that does not scale with the coordinates
status: open
opened: 2026-09-29
---


## Finding

- **Where**: `crates/profile/src/path.rs` — `PendingRunOut::rides`,
  which `Core::attribute` asks whether an emission is a pending
  fillet's run out, now for a straight arrival (the arrival ray) only.
- **What was found**: a fused verb's authored arrival arc was named by
  the same geometric reading, against the arrival circle. Far from the
  origin at ε 1e-11 and 1e-12 the arc's radial misses round at
  ε·|coordinate| past a band that does not scale with the coordinates,
  so the reading refused the chain as a near-coincidence, or named the
  arc the lowering step's `Leg` — a role no fused list holds, which the
  role-list check added with
  `work/emit/python-spells-a-piece-by-its-authoring-calls-step-handle.md`
  caught. That PR takes the arc out of the reading: the emission sites
  in `family.rs` (`resolve_arc_arrival`, `resolve_arc_close`) claim it
  as the fillet's `RunOut`, whichever step lowers it, and the circle arm
  of `rides` is gone. `path_program::a_fused_arrival_arc_far_from_the_origin_is_its_run_out`
  pins it.
- **What is left**: the ray arm decides the same kind of misses against
  the same unscaled band. A LATER step's straight segment riding a plain
  `fillet(r)`'s run out can, far from the origin at a tight ε, be named
  that step's `Leg` instead of the fillet's `RunOut`, or refuse on the
  reading. Both roles are on their verbs' lists, so no check catches the
  first. Not reproduced for the ray; this is the mechanism, read off the
  code.
- **Importance**: low (tight-ε rows, large coordinates), but it moves a
  durable name.

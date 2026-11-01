---
id: chord-and-component-directors-could-share-one-normalizing-core
kind: issue
title: Three 2-D doors normalize a vector into a Dir (unit_from_components, carrier_tangent, chord_director): weigh one shared core
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Raised by #5370's review (Q7). `profile` now has three doors that turn a
2-D vector into a `Dir` (`crates/profile/src/path.rs`):

- `unit_from_components`, the `toward` door: decides the length on the
  linear band (`path_director_norm`) and refuses `ZeroDirection` and
  `NonFiniteDirection`;
- `path::arc_fillet::carrier_tangent`: decides the radius
  (`path_arc_center_radius`) and refuses `DegenerateArcCenter`,
  `NonFiniteDirection` and `UnderflowedDirection`;
- `path::chord_director`: decides nothing, prescales by the largest
  component, stands in `+x` for the zero chord and keeps `atan2`'s
  heading past the range.

They differ on purpose in what they DECIDE (each names its own length,
or none), but the arithmetic of normalizing — and whether to prescale,
which only the chord door does and which keeps the subnormal band and
the interval scalar's ends unit — could be one function they share.
Weigh whether the two deciding doors should take the prescale too.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILETAIL (`work/profiletail/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

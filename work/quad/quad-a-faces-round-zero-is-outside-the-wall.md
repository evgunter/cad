---
id: quad-a-faces-round-zero-is-outside-the-wall
kind: issue
title: a face's round 0 is outside the work wall, and resuming past the wall is only debug-asserted
status: open
priority: P3
cost: E
parent: the-measurement-door-refines-until-a-predicate-resolves
refs: [5589]
opened: 2026-10-23
---

Raised by both DUAL reviewers on PR 5589. The fault is old in kind.

- **Round 0 is outside the wall.** `Run::stop`
  (`crates/geom-brep/src/props/quad/`) spends a round's cost
  after the lane has run it. The wall only decides whether the NEXT
  round runs, so a face whose round 0 alone costs more than
  `FACE_WORK_WALL` still pays for it in full. The round caps had the
  same gap.
- **Resuming past the wall is only debug-asserted.** `Run::enter`
  (`props/quad/`–`748`) replays the earlier rounds' cost and only
  `debug_assert!`s that the entering window's first round fits. A
  release caller that resumes a window past the wall runs that round.

Fix shape: check whether `cost(first)` fits before running round 0 or
a resumed round, and refuse typed (or answer `Wall` with no
enclosure) instead of spending.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds after PR 5815. `Run::enter`
(`crates/geom-brep/src/props/quad/run.rs:267`) replays the earlier
rounds and only `debug_assert!`s at `:272` that the entering window's
first round fits. `Run::stop` (`:293`) spends the round's cost after the
lane has run it, and reads the wall only for `round + 1`.

## Measured under per-span cells (QUAD per-span lane, PR 6092)

The rectangle lanes now lay their cells per knot span, so round 0 alone grows with the span cells, and so does the set-up before it (refined net, quotient statements, hull blocks, area pass), which no wall prices. Round 0's own cost passes `FACE_WORK_WALL` (14 250 000) at about 84 × 84 spans on the polynomial composite arm (32 × 64 cells a span), about 88 × 88 on the exact-v arm (58 × 8 × 4 a span) and about 150 × 150 on the rational composite (10 × 64 a span); PR 6092's delta review measured the same. Set-up after PR 6092's column hoist, release, one thread, uniform biquadratic (`review_6092_delta_probes::a_many_span_faces_set_up_is_linear_in_its_span_cells`): 10 × 10 polynomial 0.80 s (main 0.51 s), 30 × 30 polynomial 10.3 s (main 6.6 s), 12 × 12 rational at weight 1.5 11.0 s (main 4.6 s). It is linear in the span cells now; whether reading the surface once counts against a budget is `[ev]` PR 6102's question, and this row waits on it.

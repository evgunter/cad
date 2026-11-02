---
id: demo-tour-wall-8-no-longer-refuses-at-1e-12
kind: issue
title: demo-tour's tour_runs_green_at_eps_1e_12 is red on main: wall 8 (the loop lofted with circle sections) now tessellates, and its probe asks to be retired
status: closed
opened: 2026-10-21
priority: P1
cost: E
closed: 2026-10-28
---


Found on PR 5479 (cleave), whose `demos (tour + wild)` row went red at
`b8df543fb8`. That PR touches nothing under `demos/` or `crates/mesh/`.
Reproduced on clean `origin/main` at `5ad003f64a`, after #5511
(`tess/trimmed-retry-steps`):

    cd demos/tour && CAD_TOLERANCE_EPS=1e-12 \
      cargo nextest run --release --ignore-default-filter \
      -E 'test(=tour_runs_green_at_eps_1e_12)'

`demos/tour/src/walls.rs` panics: "wall 8 (tessellate the loop lofted
with `circle` sections) NO LONGER REFUSES — the bottle can now say
this. Retire the probe and and if wall 5 has retired too, spell
[`annulus`]'s lofted walls as `circle`
(work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late)".
That row closed on 2026-10-20. Main's push CI does not run the demos,
so nothing caught it.

## Fix

Retire wall 8's probe and its findings entry, as the message says, and
spell the annulus's lofted walls as `circle` if wall 5 has retired too.
Check the default and 1e-6 rows as well.

## Closed (2026-10-28, already fixed on main)

PR 5519 (NURBS build C) retired wall 8 and kept it live: `demos/tour/src/klein.rs` records "Wall 8, RETIRED by #5499", and the loop must tessellate at the stop's δ. No probe for wall 8 remains in `walls.rs`'s retirement path. Closed by the TESS orchestrator at the 2026-10-28 pickup.

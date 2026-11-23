---
id: germ-cone-doc-claims-one-merged-face-where-two-half-bands-stay
kind: issue
title: germ_cone_apex_closure's docs say the full revolve merges to one cone face; measured, merge_coplanar_faces leaves the two half-bands
status: closed
opened: 2026-09-29
priority: P3
cost: E
closed: 2026-11-23
pr: 7414
---


Filed by ORIGIN from PR 3430's fix pass, which measured it while
building a cone fixture. In `crates/sweep/tests/germ_cone_apex_closure.rs`
the doc on `cone()` and on `the_merged_full_cone_agrees_with_the_closed_form`
says the full revolve "merged to one cone face"; on current main,
`cone(None)` after `merge_coplanar_faces` has TWO cone faces (the two
half-bands). Either the doc is stale or the merge no longer joins them
— measure which before editing (the merge door is TOPO's; a
regression there is a finding for TOPO, not a doc fix). Signed (ORIGIN
orchestrator).

Moved from TCOST to GREEN on 2026-11-20, by `git mv`, when TCOST merged into GREEN: TCOST held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one GREEN carries.

## Closed

2026-11-23, on PR 7414 (`lathe/full-revolve-one-band`). A full revolve
now sweeps the cone wall as one face
(`work/lathe/a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote.md`),
so `cone(None)` is one cone face as built, before any merge, and the
merge has nothing to join. The test's docs say so; its "two half-bands"
body is the full revolve itself, the one face, and is named so.

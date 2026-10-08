---
id: germ-cone-doc-claims-one-merged-face-where-two-half-bands-stay
kind: issue
title: germ_cone_apex_closure's docs say the full revolve merges to one cone face; measured, merge_coplanar_faces leaves the two half-bands
status: open
opened: 2026-09-29
priority: P3
cost: E
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

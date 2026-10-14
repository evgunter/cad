---
id: corner-config-indeterminate-has-no-real-body-witness
kind: issue
title: blend: CornerConfig::Indeterminate has no real-body witness once a plane–plane end reads its end face before its neighbours
status: open
opened: 2026-10-06
priority: P3
cost: E
---

## Finding

`CornerConfig::Indeterminate` (a chain end whose neighbour no arm
resolves) lost its one real-body witness. The partial spool's planar
cap edges (`crates/sweep/tests/m5_pr12_refusals.rs`,
`a_planar_chain_ending_at_a_curved_neighbour_refuses_its_curved_end_face`)
end at the torus wall, and `battery::corner_at` now reads a plane–plane
end's end face before resolving its neighbours, so they refuse
`UnsupportedRunOut` (`END_FACE_CURVED`) instead. The tag is still
reachable — a curved open link ending beside an unresolvable edge, or a
turn whose unrequested third edge is tangential (two coplanar faces) —
but no fixture builds either, so the arm `corner_at` folds every
neighbour refusal into is now pinned by no body.

Close: build one of those witnesses and pin the tag through the front
door, or show the tag unreachable and retire it (a vocabulary change,
Ev's).

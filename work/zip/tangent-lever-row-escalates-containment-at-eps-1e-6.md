---
id: tangent-lever-row-escalates-containment-at-eps-1e-6
kind: issue
title: sweep rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts escalates Containment on bool_contact_vertex at eps 1e-6 (main red)
status: open
opened: 2026-10-06
priority: P1
cost: E
---


Reported by BAND's PR 4173 fix-pass lane (2026-10-06): at
`CAD_TOLERANCE_EPS=1e-6`, `sweep::all
rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts` fails
with a `Containment` escalation on `bool_contact_vertex`, identically on
origin/main `9319c1cf` in a clean worktree (no blend code involved). The
row is the review's from `a-flush-declared-reflex-union-ships-the-wrong-volume`.
Default eps and 1e-12 are green. Filed by the BAND orchestrator because no
item named it; ZIP owns the row.

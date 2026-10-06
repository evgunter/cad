---
id: tangent-lever-row-escalates-containment-at-eps-1e-6
kind: issue
title: sweep rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts escalates Containment on bool_contact_vertex at eps 1e-6 (main red)
status: parked
opened: 2026-10-06
priority: P1
cost: E
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---


Reported by BAND's PR 4173 fix-pass lane (2026-10-06): at
`CAD_TOLERANCE_EPS=1e-6`, `sweep::all
rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts` fails
with a `Containment` escalation on `bool_contact_vertex`, identically on
origin/main `9319c1cf` in a clean worktree (no blend code involved). The
row is the review's from `a-flush-declared-reflex-union-ships-the-wrong-volume`.
Default eps and 1e-12 are green. Filed by the BAND orchestrator because no
item named it; ZIP owns the row.

Parked on the D10 hold with ZIP's other REST-lane rows: the row exercises
the declared-REST zip, which the hold covers and which is due to retire at
INTENT's stage 4 (`the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms`).

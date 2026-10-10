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

Parked on the D10 hold with ZIP's other REST-lane rows: the row exercises
the declared-REST zip, which the hold covers and which is due to retire at
INTENT's stage 4 (`the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms`).

## Released from the D10 hold (2026-10-08)

Nothing D10 changes gates this row, so it is open: red on main; the escalation is reduce's contfp ON ladder (contain.rs), which runs before the zip and survives stage 4's Zero glue. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Renamed (2026-10-08, INTENT stage 4 A)

The row is `rest_zip_admission::the_tangent_site_keeps_building_pure_contacts`
now: the zip is deleted and the join builds the tangent site, so no
"lever" hands the union anywhere. The escalation, if it stands, is the
reduction's, before the join.

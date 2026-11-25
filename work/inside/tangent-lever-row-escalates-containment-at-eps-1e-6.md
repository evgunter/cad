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

## Re-homed at ZIP's close (2026-10-11)

Moved from ZIP to INSIDE: the escalation is the reduction's containment ladder in `contain.rs`, which INSIDE owns. The declared-REST zip it once exercised was deleted by INTENT stage 4 A.

## Measured green; fixed before it was filed (2026-11-25, lane `inside-contain-small`)

On origin/main `133ca95e02`,
`sweep::all rest_zip_admission::the_tangent_site_keeps_building_pure_contacts`
passes at `CAD_TOLERANCE_EPS=1e-6`, at the default ε (1e-9) and at 1e-12.
Probed at 1e-6: red at `9319c1cf03` (the commit the report names,
`Escalated { decision: Containment, … margin 2.2889990474188053e-6 …
predicate: Some("bool_contact_vertex") }`, as reported), green at
`a714f4e3eb` and `1484445f80` (either side of INTENT stage 4 A's merge),
so stage 4 A did not fix it.

The fix is PR 4199 (`cleave/rest-zip-lever-1e6`, merged `fa1f2a6316`
on 2026-10-06, two hours after `9319c1cf03` and before this row was
filed), as `work/reach/log.md`'s 2026-10-06 entry records:
`carrier_touch::ball_off_face` placed the ball's foot before reading the
face's boundary against the ball, so a ball holding the fillet's
boundary vertex 2.29e-6 from the foot escalated the placement's vertex
pre-pass in band; it now reads the boundary first. Its pin,
`carrier_touch_rows::a_ball_holding_a_face_vertex_in_the_band_of_its_foot_is_not_off_the_face`,
is on main. No code change here.

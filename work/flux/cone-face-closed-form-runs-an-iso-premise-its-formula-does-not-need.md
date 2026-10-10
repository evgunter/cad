---
id: cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need
kind: issue
title: cone_face_closed_form runs the S58 iso premise (require_rims_at_extremes, du_of_rims) on rim/generator loops, though apex·VA and |axis·VA|/sin α are exact for any one-nappe region
status: open
opened: 2026-10-10
priority: P1
cost: M
---


Found off-question by the sphere-arm designer pair (fork log row 108),
2026-10-10, and read from the code at origin/main `dfb193f783`, not
executed.

The cone's closed form (`geom_brep::props::curved`'s
`cone_face_closed_form`) gives the flux as `apex·A⃗` and the area as
`|axis·A⃗|/sin α`. Both are exact for any region on one nappe, because
they read only the boundary's vector area. Yet a cone face whose outer
loop is made of rims and generators still runs the S58 iso premise
(`require_rims_at_extremes`, `du_of_rims`). That premise is the one the
iso-rectangle formula needs, and it refuses a notched cone wall that the
vector-area formula would measure exactly.

It is the cone's half of the same "one closed form per face kind"
question that the sphere row settles. It lands after or beside the
sphere build, which also moves the cone's side gate onto
`props_cone_area_side`. (FLUX orchestrator)

---
id: a-closed-edge-lying-in-a-face-is-never-read-by-the-parting
kind: issue
title: The Boolean's parting skips every closed edge, so a full circle lying in a face keeps the rider form
status: open
opened: 2026-10-26
priority: P1
cost: E
---



Found by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`.
P1: D1-wrong topology shipped silently, the class of the two rows that
branch closes.

## What

`topo::boolean::slit::riders` skips an edge whose two ends are one
vertex (`if a == b { continue; }`), so a closed edge, a full circle a
full revolve or a one-segment rim mints, is never asked of any face.
Where such a circle lies in a face with both its faces going into the
face's material, the Boolean ships one edge there. Witness:
`sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
`a_tube_through_the_domes_base_builds_every_op_undeclared`: `t ∖ d`'s bowl
meets the tube's wall along the whole rim circle, one closed edge, and
it builds. The same bowl with its rim split into arcs by a turned dome
(`a_dome_sunk_across_the_tubes_seam_rulings_builds_every_op_undeclared`)
refuses `EdgeInFace` (`Curved`), as D1 says it must until the curved
slit form is built.

## Fix

Ask a closed edge of every face as an open one is asked (its ends are
one vertex, so `both` is the faces that vertex lies on, and the
carrier and the records decide the rest); a circle then refuses
`Curved` where its walls go into the face's material, and the two
witnesses agree.

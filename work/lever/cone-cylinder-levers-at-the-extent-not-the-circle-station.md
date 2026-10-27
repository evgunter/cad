---
id: cone-cylinder-levers-at-the-extent-not-the-circle-station
kind: issue
title: cone×cylinder levers its tilt at the extent, not at the coaxial circles' station
status: open
opened: 2026-10-07
priority: P3
cost: E
---

## What

`cone_cylinder_section` (`crates/geom-brep/src/intersect.rs`) reads
`coc_axes_parallel` and the coaxial row's separation range
(`separation_range` over `|s| ≤ extent`) at the operand extent from the
apex. The served circles stand at `±R·cot α`, which `coc_station_reach`
already requires to be inside the extent, so a consumed point of the
served object travels at most the station, not the extent. The excess
escalates poses whose circles stand within the band (PR 4280's
differential: 444 of the coc family's good-input escalations after the
range fix).

## The shape of a fix

Read the separation range over `|s| ≤ |R·cot α|` once the station
is decided inside the extent, so the coaxial row levers the served
circles' own travel.

## What the offset door hands it (OFFSET, PR 5875, 2026-10-27)

`topo::offset_derive::section_closed` hands `cone_cylinder_section` the
old edge's diameter bounded from above (`geom_brep::edge_extent_sup`),
because `coc_axes_parallel` acts on its Zero. That number is an edge
diameter, not a reach from the apex. So `separation_range`'s
`|s| ≤ extent` and `coc_station_reach`'s admission `extent − |R·cot α|`
read a length measured from somewhere else. The admission is the side
where a longer number admits more, so it is the bound from above that
admits a farther station there. For the old edges this door re-derives
(coaxial circles), that bound is the circle's exact diameter `2R`, or
its chord for an arc of under a half turn. The station's own reach from
the apex, `|R·cot α|`, is the length this row's fix would read.

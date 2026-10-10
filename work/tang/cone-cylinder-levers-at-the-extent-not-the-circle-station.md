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

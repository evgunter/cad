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

`topo::offset_derive::section_closed` hands `cone_cylinder_section`
both bounds on the old edge's diameter (`geom_brep::LeverBounds`), and
each row reads its own side. `coc_axes_parallel` and the coaxial swing
(`separation_range` over `|s| ≤ extent`) act on a Zero, so they read the
bound from above (`geom_brep::edge_extent_sup`). The three convention
guards and `coc_station_reach`'s admission `extent − |R·cot α|` act on
a Positive, so they read the bound from below
(`geom_brep::edge_extent_inf`). A longer number would admit a farther
station, so the admission is no looser than before this PR.

This row's own defect is unchanged. Both numbers are an edge's
diameter, not a reach from the apex, so the swing and the admission
still read a length measured from somewhere else. For the old edges
this door re-derives (coaxial circles), both bounds are the circle's
exact diameter `2R`, or its chord for an arc of under a half turn. The
station's own reach from the apex, `|R·cot α|`, is the length this
row's fix would read.

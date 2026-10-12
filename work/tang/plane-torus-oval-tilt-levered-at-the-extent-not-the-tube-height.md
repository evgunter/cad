---
id: plane-torus-oval-tilt-levered-at-the-extent-not-the-tube-height
kind: issue
title: plane×torus levers the axis-in-plane tilt at the extent, not at the tube's height
status: open
opened: 2026-10-07
priority: P3
cost: E
---

## What

`plane_torus_section`'s `pt_axis_in_plane`, and the two-oval row that
reads its tilt beside the gap (`pt_spiric_two_ovals`,
`crates/geom-brep/src/intersect.rs`), lever the axis' angle off the
plane at the operand extent. A spiric oval lies within the tube's
height `r` of the equatorial plane, so the tilt moves a minted oval
off the real plane by at most its sine times `r`, not times the
extent (`≥ R + r`).

## The shape of a fix

Lever the oval row's swing at `r`; leave the routing row's lever as it
is or re-derive it with the meridian lane's reading (the tilt turns the
meridian circles along the torus).

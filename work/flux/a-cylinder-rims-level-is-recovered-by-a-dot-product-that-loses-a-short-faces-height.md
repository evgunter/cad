---
id: a-cylinder-rims-level-is-recovered-by-a-dot-product-that-loses-a-short-faces-height
kind: issue
title: A cylinder face's rim levels are recovered as (centre − origin)·axis, which on a tilted frame far from its origin costs a short face up to 1.8e-8 of its area
status: open
opened: 2026-10-03
---


## What

`cylinder_boundary` (`crates/geom-brep/src/props/curved.rs`) reads each
rim's level as `v = (centre − origin)·axis`. The Green form now sums
against the face's mid-level, so it adds no cancellation of its own
(`cylinder_green_conditioning`, ≤ 1e-14 on frames along +Z), but the
level itself carries the rounding of that dot product: three products
of coordinates of size `|centre − origin|`. On a tilted axis with the
face far from the surface origin, a short face's height `hi − lo` is a
difference of two such levels, and its relative error grows as
`|centre − origin| / (hi − lo)`.

Measured by TANG's delta review of PR 3851: up to **1.8e-8** relative
area error on tilted, off-origin frames. The fix-pass row
`a_tilted_axis_off_the_origin_keeps_the_conditioning` (axis
`(1, 2, 2)/3` through `(3, −2, 5)`, a 4 mm face 40 m along it) holds
1e-12, not 1e-14, for this reason.

## The shape of a fix

Read a face's heights relative to one of its own rims: `v_i − v_0 =
(c_i − c_0)·axis`, a dot product of a SHORT difference, so the error
scales with the face's own extent rather than its distance from the
origin. The anchor only needs one level, and the extent check
(`props_face_extent`) the differences.

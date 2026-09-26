---
id: sphere-chart-trim-folds-any-number-of-rim-levels
kind: issue
title: sphere_chart_trim reads a trimmed sphere face as the latitude window its rim levels span however many levels there are, so a stepped outline (three or more rim latitudes) is misread; no door builds one today
status: open
opened: 2026-09-26
---


Found by both of CONTACT-3's dual reviewers. It is the sphere
sibling of the stepped-outline case that unit closed for the cylinder:
there, `wall_outline` (`crates/topo/src/boolean/solid_contain.rs`) reads
a rims-and-meridians wall as its rectangle only when the rims sit on
exactly two levels, and reads any other outline by parity along the
ruling.

## The defect, by reading

`sphere_chart_trim` (`solid_contain.rs`) checks each boundary edge's
class, rim or meridian great circle, and collects every vertex's
latitude into `levels`. `latitude_extremes` then folds `levels` to the
two extreme latitudes, however many distinct rim latitudes there are.
`point_on_sphere_in_face` answers membership in the `[azimuth] ×
[latitude]` rectangle those extremes span.

Consider a trimmed sphere face whose rims sit on three latitudes, a step
in the chart joined by a meridian. It passes the class check, and the
rectangle over-covers the step's notch. A hit there would answer `In`
from the certified walk.

## Why it is unmeasured

No door builds such a face. The sphere is split-gated
(`CurvedBooleanUnsupported`); the boolean refuses plane×sphere sections
tilted against the polar axis; and the stepped outline needs two
coaxial cuts plus a meridian cut meeting on the face. That shape was not
found to be reachable through any door. The reachability of an L-shaped
sphere face is already called unproven in `torus_chart_windows`' "this
is a class" paragraph.

## What closes it

Count the distinct rim latitudes, as `wall_outline` counts rim levels,
and keep the rectangle for exactly two (one if a pole closes the face).
Read anything else by parity along the meridian, or refuse it as
`PartialSphereFace`. Either is the cylinder's discipline.

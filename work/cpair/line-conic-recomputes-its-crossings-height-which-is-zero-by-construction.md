---
id: line-conic-recomputes-its-crossings-height-which-is-zero-by-construction
kind: issue
title: line_conic reads the plane crossing's height above the plane, zero by construction, and over a box counts the box twice in it
status: open
opened: 2026-11-24
---


Found by `boxfinish/wall-roots`'s sweep (the class of
`geom_brep::curve_face::line_quadric::wall_quadratic`'s middle
coefficient at the nearest point: a quantity zero by construction,
recomputed from a moved offset that counts the box twice). Read from
the code, not run.

`geom_brep::curve_pair::pose`, the line × conic crossing route
(`line_conic`): `point = o + û·((c − o)·n / den)` is the line's crossing
of the conic's plane, then `height = (point − c)·n` is read. Over the
reals it is zero; at `f64` it is the crossing's rounding, read where it
landed (the comment says so, and that is the point of reading it). At
`Interval`, `o`, `c` and `n` each enter twice and the cancellation is
lost, so `|h|` reads the box's width and widens the polish window
`(e + |h|)/|d̂·n̂|`. Loose, not unsound. A spelling that is the rounding
at `f64` and zero over a box would want the rounding charged rather than
re-measured.

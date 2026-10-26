---
id: the-at-infinity-volume-enclosure-has-no-row-that-reaches-it
kind: issue
title: No row reaches point_in_solid's side at infinity on the volume's certified enclosure: its one row's aimed rays now read the side
status: open
opened: 2026-10-24
priority: P3
cost: M
---


Found by CHART's `curved-face-containment-answers-none-on-a-ringed-face`
(branch `chart/ringed-curved-doors`).

## What

`solid_contain::missed_side` reads the side at infinity off the sign of
the selection's volume, certified on the interval re-derivation of the
walk's enclosures (`bool_point_in_solid_infinity_enclosure`), when no
ray aimed at a face of the selection reads it. Its one row was
`sweep/tests/a_plane_across_a_one_face_wall.rs`'s far point on the
tilted plane's wall piece. That row reached the enclosure because every
aimed ray crossed the ellipse-trimmed wall and met the cylinder chart
trim's period guard (`bool_wall_trim_period`).

The branch reads cylinder and cone faces from their boundary edges
(`topo::boolean::ruled_region`) and deletes that guard. The aimed ray
now reads the side, the enclosure is never asked, and the row is
re-pinned to that (`a_far_points_side_on_the_wall_piece_is_read_by_an_aimed_ray`).
No row in the tree now drives the enclosure's certified sign.

## Owed

A selection none of whose faces offers an aim that reads (since FLUSH's
PR 5730, a face offers none where it is a cylinder or cone whose region
cannot be read, a sphere patch with no boundary arc, or any face whose
outward normal at the aim its reader refuses, `side_reader::aim_at`;
and a selection with a spline face, or a sphere face with an edge that
is not a circle arc, casts no ray at all),
or whose aimed rays all set aside, with a far point whose rays meet
nothing. Its row would pin the enclosure's one decision, as the old row
did.

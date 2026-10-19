---
id: a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted
kind: issue
title: An edge of a Boolean result lying in a face along part of its length refuses EdgeInFace Partly instead of parting there
status: open
opened: 2026-10-19
priority: P1
cost: H
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

`sweep` `review_4831_probes::p12_a_plane_tangent_to_the_middle_of_a_hole`:
the holed plate (hole r = 1 about the z axis, `z ∈ [0, 1]`) less the
cutter brick `x ∈ [1, 4]`, `z ∈ [0.25, 0.75]`. The cutter's face
`x = 1` is tangent to the hole along the ruling `x = 1, y = 0`. In the
result the hole's ruling edge, `(1, 0, 0)` to `(1, 0, 1)`, lies in the
pocket's wall `x = 1` along its middle half only, crossing the wall's
top and bottom edges (edge–edge records at `z = 0.25` and `0.75`, and
an edge–face record on the wall). Both of the ruling's faces bend into
the wall's material there: a doubled cusp, which D1
(`crates/topo/README.md`, tier 3) parts into two coincident edges.

On main `A ∖ B` built the rider form, the ruling whole across the wall.
On the branch `topo::boolean::slit` reads it and refuses
`BooleanError::EdgeInFace { gap: EdgeInFaceGap::Partly }` ("the edge
runs on past the face's boundary"): the parting needs the ruling and
the wall's two edges divided at their crossings first, and their
crossing vertices fused, before the middle piece parts the wall as an
edge whose ends lie on its outline.

## Fix

Divide the edge and the face's boundary edges at each crossing the
edge–edge records name (the zip's `fuse_by_joint` shape), then part the
piece lying in the face (`slit::part`, both ends on the face's loop).
Return `p12`'s `A ∖ B` to its body, `30 − π`.

---
id: a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints
kind: issue
title: An arc rim covering part of its row (a split rim's child) refuses the chord schedule, which places sub-arc joints only for a rim whose arc is its edge's interval
status: open
opened: 2026-10-23
priority: P3
cost: M
---

Found by `iso/arc-rim-sub-arc` (ISO,
`iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain`).

## What happens

A `Pcurve::IsoArc` is the boundary row over the image's arc
`[t0, t0 + angle]`, and its edge's span lies in that arc. A rim the loft
built has its span AS its arc. A child of a split rim keeps the
parent's row and arc (`split_edge`'s carry, and since that PR the mint's
re-derivation, `geom_brep::arc_rim_measured`), so its span is a strict
part of the arc.

`mesh::chords`' `nurbs_tighten` (the `Pcurve::IsoArc` arm) needs a
chord point on each sub-arc joint, a C⁰ crease of the wall, and gets it
by rounding a uniform schedule to a multiple of the sub-arc count. That
lands on the joints only when the arc is the edge's interval bit for
bit, so for `spans > 1` it refuses any other row:
`UnsupportedCurve { note: "an arc-rim pcurve whose turn is not its
edge's parameter interval, so no chord point lands on its sub-arc joints
…" }`. A split bulged-profile loft prism (three sub-arcs) re-mints Ok
and validates tier 3, then refuses to tessellate there
(`sweep`'s `a_rim_covering_part_of_its_row_re_mints_the_rows_arc::both_halves_of_a_split_rim_re_mint_the_parents_row`
pins the refusal). Pre-existing: the carried rows read the same before
the re-mint could derive them.

## Fix

The joints inside a child's span sit at `t0 + angle·sub_arc_joint(k, m)`
on the IMAGE's arc, at fractions of the edge's span no uniform schedule
names. Either the schedule takes the joints in `(t_start, t_end)` as
fixed chord points and spaces each run between them (the crease point
then is the joint's own parameter, as the trimmed lane needs), or the
refusal narrows to a child that holds an interior joint (a split at a
joint, as on a two-sub-arc semicircle split in half, holds none).


## Moved to TESSSIZE (the curved and trimmed lanes' sizing: which density each lattice emits, and the theorem behind it) (2026-10-28)

Cut out of TESS by its semantic-seam split (`work/README.md`, Track size) when TESS measured 146 budget points against 30. Moved by `git mv`; the id and body are unchanged.

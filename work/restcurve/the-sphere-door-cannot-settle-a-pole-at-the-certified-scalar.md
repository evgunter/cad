---
id: the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar
kind: issue
title: The sphere door cannot settle a pole at the certified (interval) scalar, so tier 3's pole side still reads locate
status: open
opened: 2026-10-31
priority: P1
cost: M
refs: [validate-tier3-curved-boundary-containment, a-loop-the-chart-singularity-walk-cannot-certify-is-exempt, import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave]
---

## What

DESIGN `[topology.tiers]`: "whether a point lies in a face has one
reading, the point-in-face doors'". Unit 2 of
`validate-tier3-curved-boundary-containment` (the region arm) was to read
a sphere pole's or cone apex's side, off the face's boundary, through the
face's door (`boolean::cycle_side` with every surface cycle kept) in
place of `geom_brep::chart_singularity::locate`'s side reading. Built
that way, tier 3 refused six bodies the kernel's doors build, each at the
certified (interval) scalar, each `ChartSingularityUndecided` from the
sphere door; every one passes at the base, where `locate`'s stereographic
closed form decides the pole:

- `sweep` `a_tube_ending_on_a_ball::the_witness_brackets_its_closed_forms_at_the_certified_scalar`
  (North: `bool_sphere_region_at` over `[0, 4.443]`);
- `band_planar_cut_off_interval::the_box_rows_carve_at_the_certified_scalar_and_bracket_their_closed_forms`
  (South: `bool_sphere_region_span` over `[-0.236, 0.393]`);
- `fillet_h5_hostless_rim_interval::the_plane_sphere_hostless_carve_brackets_its_closed_form_at_the_certified_scalar`;
- `m5_s13_pips_interval::certified::interval_finding_union_is_bracketed`
  and `::interval_pip_pair_is_bracketed_and_additive`;
- `m6_surgery_interval::certified::interval_one_pip_composed_die_is_bracketed`.

Across them the enclosures seen were `bool_sphere_region_at` over
`[0, π]`, `[0, π/2]`, `[0, 0.283]` and `[0, 4.443]`, and
`bool_sphere_region_span` over `[-0.236, 0.393]` and `[-π/2, 3π/2]`
(band `(1e-9, 1e-8)`): a crossing's place along a great circle from the
pole, and a root's parameter against its arc's span, each read through an
angle whose interval enclosure spans most of a turn. No path settles, and
the walk answers the first in-band refusal. At `f64` the switch refused
nothing in topo's suite, step-import's, or the sweep crate's.

So the region-arm PR left tier 3's pole side on `locate` (as on its base)
and kept only the split of `locate`'s boundary placement into
`place_on_boundary`.

## Owed

The sphere door (and the cone door at an apex, which no interval row
reached) settles a singular point at the certified scalar: its crossing
places and span reads made without an angle whose enclosure spans a
turn. Then tier 3's chart-singularity arm reads the side through the
door (the switch the region-arm branch built and reverted: the face's
door at the point, every surface cycle kept; a door's in-band reading
refuses as `ChartSingularityUndecided` with `SideUnread::Banded`), and
tier 3's use of `walk_loop` retires. The row that pins the pole reading
today, and must keep passing through the switch, is
`crates/topo/tests/props_sphere_cap_door.rs`
`a_tilted_cap_holds_a_pole_by_its_region` (a cap tilted against its
sphere's axis, holding the north pole or neither). The split-out
`geom_brep::chart_singularity::place_on_boundary` is `pub(crate)` until
this row needs it from topo. This is the ratified design, one
reader and the doors, with its order changed. Import follows
(`work/foreign/import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave.md`).
Filed by RESTCURVE's region-arm lane.

---
id: circle-pair-margins-are-still-spelled-by-hand-outside-the-section-value
kind: issue
title: Four more readers decide a circle pair's reach and nest by hand rather than from geom_core::Circles: the section certificate's sphere and cylinder pairs, and the parallel-cylinder gap
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Found by the sweep of `sect/circle-crossing-once`
(`circle-pair-crossing-is-spelled-three-times`, which moved the torus
arms and profile's `circle_circle` onto `geom_core::Circles`),
2026-10-29. Filed here because the sites span several programs'
territory (`work.py territory`).

## The sites

Each writes `r₁ + r₂ − d` and `d − |r₁ − r₂|` (or `|r₁ − r₂| − d`) by
hand, which are `Circles::reach()`, `internal()` and `nested()` bit for
bit:

- `crates/topo/src/boolean/section_cert.rs` `sphere_sphere`, the rows
  `section_sphere_pair_reach` and `section_sphere_pair_nest`; its
  witness is placed by the textbook form, which HONE's
  `circle-crossing-closed-forms-cancel-at-unequal-radii` already
  carries;
- the cylinder pair's pose (`geom_brep::pair_section::cylinders`,
  `cc_crossing_gap` and `cc_crossing_nest` on `|e|`, the axes' gap along
  their common perpendicular), which replaced `section_cert`'s
  `cylinder_cylinder` and its `section_cylinder_pair_*` rows (SECTARMS,
  PR 6151);
- `crates/geom-brep/src/locus.rs` `parallel_cylinder_gaps`'s
  `tangent_locus_internal_gap` (`|r₁ − r₂| − d`), beside
  `crates/geom-brep/src/intersect.rs`'s `cc_parallel_gap`
  (`r₁ + r₂ − d`).

## What the taker owes

Read the margins from one `Circles` per pair, keeping each row's name
and its across-the-swing reading; that moves no bit. Where a site then
places the meeting, place it from the same value.

## Not this class

Swept by shape (`x − |a − b|`, sums of two radii less a distance, and
the radical form `d² ± r₁² ∓ r₂²`); these hits are other predicates:
`pair_section.rs`'s cylinder × sphere near and far reach (a sphere
spanning a wall, not a crossing); `torus.rs`'s coaxial cylinder wall
(a circle against a line); `pair_section/gap.rs`'s critical-gap levels
(`ρ₁ + ρ₂`, `|ρ₁ − ρ₂|` against a swept distance);
`topo/src/boolean/circle_torus.rs`'s contour angle (classified by its
bands, placed by the law of cosines); `topo/src/boolean/join.rs`
`parallel_radical_plane` (measured harmless,
`circle-pair-radical-chord-has-no-shared-home`); and
`geom-brep/src/curve_pair/pose.rs` `coplanar_circles` (a first-harmonic
distance, not these margins). A spelling through other names, or
assembled across functions, is outside what the patterns match.

## 2026-11-02: a cited site retired (SECTARMS stage 3)

Branch `sectarms/cylinder-join-reads-value` retires `topo::boolean::join`'s
`cylinder_pair_axes`, `cylinder_pair_frame`, `crossing_cylinder_reading`
and `parallel_radical_plane`. The join now reads two cylinders on the
section value (`join::cylinder_read`), and the parallel lane takes its
plane from `geom_brep::CylinderCylinderPose::rulings_plane`. The hit
above that cites one of those sites no longer exists. The rest of the
row stands as filed.

## 2026-11-02: the rulings plane retired too (SECTARMS stage 3b-i)

Branch `sectarms/cylinder-carried-closed-form` retires the parallel
lane's plane: `geom_brep::CylinderCylinderPose::rulings_plane` and
`topo::boolean::join`'s `GermLane::Rulings`, `rulings_are_straight` and
`rulings_normal` are gone. A parallel cylinder pair now rides its
carried rulings (`chord_join::carried_chord_spec`'s closed-form arm).
The note above that names the rulings plane no longer points at live
code. The rest of the row stands as filed.

Moved from `work/issues/` to `work/honeone/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

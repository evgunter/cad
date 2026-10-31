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
- the same file's `cylinder_cylinder`, `section_cylinder_pair_reach` and
  `section_cylinder_pair_nest` on `|δ|`, the axes' offset along their
  common perpendicular;
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

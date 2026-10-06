---
id: split-halves-volumes-sum-to-the-whole-only-within-their-pads
kind: issue
title: A tilted split's two halves' volumes sum to the whole only within their quadrature pads, where the halves' cylinder-wall quadrature refines differently
status: open
opened: 2026-10-06
priority: P3
cost: M
design: true
---

(Review of PR #4081, measured on its head `cleave/split-segment-curve`.)

## What

The two halves of a tilted `splitting::split` do not hold the whole's
volume to rounding. Their `topo::props::mass_properties` volumes add
up to the closed form only within the halves' certified pads.

Two fixtures were measured:

- **Cylinder:** the two-arc cylinder (`sweep::test_support::cylinder_of_arcs_at(2, 1.0, origin, 0, 2.5)`,
  whole volume `2.5π`), cut through `(0, 0, 1.25)` with normal
  `(sin φ cos a, sin φ sin a, cos φ)`. There were 24 poses:
  φ ∈ {0.05, 0.1, 0.15, 0.2, 0.3, 0.5, 0.7, 0.9} × a ∈ {0, 0.7, 1.4}.
- **Drum:** the drum pocketed through its wall
  (`split_section_rings::a_split_across_a_ringed_wall_mints_each_segment_on_one_curve`'s
  body), cut through `(0, lift, 0)` with normal `(t cos a, 1, t sin a)`.

Results:

- **Head, cylinder:** 18 of 24 poses meet the whole to ≤ 2e-14. Six
  miss by 1.6e-11 to 2.1e-9:
  - (0.15, 1.4): 2.8e-10
  - (0.3, 0.7): −2.1e-9
  - (0.3, 1.4): −7.0e-10
  - (0.7, 1.4): 1.4e-10
  - (0.9, 0.7): −1.6e-11
  - (0.9, 1.4): −5.6e-10
- **Head, drum at a = 0:** misses at all three tilts tried: 5.0e-9
  (t 0.2, lift 0.1), −1.7e-11 (0.1, 0.05) and −5.0e-11 (0.3, −0.1).
  These are identical on main.
- **Head, drum at a = 0.7 and 1.4:** the drum meets the whole to
  ≤ 1e-14. The t = 0.3 pose refuses in props at both azimuths: an
  escalated `props_quad_converged` at 0.7, `RingOnCurvedFace` at 1.4.
- **Main** (before #4081): 23 of the 24 cylinder poses miss, by
  1.6e-11 to 5.9e-9. #4081 mints a segment's two chords on one curve,
  and that closed most of them.

## Why it is probably not split geometry

Every miss is at least two orders under the pads. Each half's volume
is quadrature with `volume_pad` of 7e-7 to 3e-6. Where the two halves'
pads are EQUAL, the miss is ≤ 2e-14, except at the drum's a = 0. Where
they differ, the misses appear: for example (0.3, 0.7) has pads
7.5e-7 against 2.65e-6, and the drum (a 0, t 0.2) has 2.1e-6 against
8.1e-7. So the halves' cylinder walls, cut by one ellipse, are
refined to different depths by `geom_brep::props::quad::cylinder_cut_face_rounds`
(through `topo::props::quad_lane`). The sum therefore carries each
half's own quadrature error, inside its stated bound.

The question this row owes is whether a split's halves should
conserve volume to rounding: the same section, measured once and
shared. Or whether this is the bound working as stated, in which case
the row closes as such. If quadrature is the lever, the ground is
flux/quad's (`crates/geom-brep/src/props/quad.rs`), and the row moves
there.

The drum at a = 0 misses with equal pads. That residue is not
explained by the asymmetry and is the first thing to look at.

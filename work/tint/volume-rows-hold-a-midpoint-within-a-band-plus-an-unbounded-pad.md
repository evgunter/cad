---
id: volume-rows-hold-a-midpoint-within-a-band-plus-an-unbounded-pad
kind: issue
title: Volume rows hold a quadrature midpoint within a fixed band plus the reading's own pad, with the pad unbounded, so a bracket that balloons passes any body
status: open
opened: 2026-11-03
---


Found by SPLICE (`splice/far-union-volume`, PR 6429's fix pass,
2026-11-03).

## What

A row that compares a certified volume with its closed form as
`|V − want| ≤ band + volume_pad` is only as sharp as the pad, and the
pad grows with the resolution the reading met: at
`topo::test_support::pinned_resolution` (1024·ε of mean boundary
displacement) a curved quadrature face's pad at ε = 1e-6 is of order
1e-5 to 1e-3 m³, so the row passes a body off by that much while
stating a band of 1e-8 or 1e-9. PR 6429's review showed it concretely:
the spline census held at band + pad at the pinned resolution let a
lofted cylinder's cap displaced by up to 5e-4 m (500ε) through.
`spline_spiric_operands` now measures at a stated resolution and asserts
each pad within its band (`measure` there, over
`topo::test_support::volume_at`); the comparison is
`topo::test_support::assert_volume_near`.

Sites of the `band + pad` shape with no bound on the pad (grep
`volume_pad` beside an `abs()` comparison in `crates/*/tests`), as of
main `ee44bee2`:

- `crates/sweep/tests/split_across_a_revolve_seam.rs`, every
  `Vol::near` (bands 1e-8 / 1e-9; the pads at 1e-6 were 1e-7 to 1.3e-5
  when `work/cleave/revolve-seam-split-volumes-miss-their-closed-form-at-eps-1e-6`
  was closed this way);
- `crates/sweep/tests/m5_pr6_pcurves.rs`,
  `a_seam_closed_tube_split_mints_clean_halves`' volume assertion;
- `crates/topo/tests/missed_ray_side.rs` (`volume_pad + 1e-12`);
- `crates/sweep/tests/sheets_part_on_slanted_rims.rs` (two sites);
- `crates/sweep/tests/verbs_cylcyl_r1_review_probes.rs`;
- `crates/sweep/tests/boolean_tangent_rim_review.rs`;
- `crates/sweep/tests/shell9_r1_probes.rs`;
- `crates/sweep/tests/an_annular_tube_through_a_plate.rs`;
- `crates/sweep/tests/verbs_shell.rs`;
- `crates/sweep/tests/band_planar_oblique_fillet.rs` (a summed pad);
- `crates/step-import/tests/wild.rs` (`volume_pad * 1e9`).

Some of these may read only closed-form faces (pad 0), where the shape
is harmless; that is per site, unmeasured here.

## What closing it takes

Per site: measure at a resolution the row states, at which the pad lies
within the band at every ε row (measured cost stated), assert the pad
there, and compare through `assert_volume_near`; or show the site's
faces all take closed forms. A row whose pad cannot be brought within
its band at a ε row says so as a pinned stop, not a widened band.

---
id: revolved-volume-tests-carry-their-own-pappus-sums-beside-test-supports
kind: issue
title: Four sweep test files carry their own Pappus sums beside sweep::test_support::pappus
status: open
opened: 2026-11-20
priority: P4
cost: E
---

Found by PR 7177's review (SECT), 2026-11-20. PR 7177 moved its own torus
rows onto `sweep::test_support::pappus` (`triangle`, `segment`,
`pappus_volume`). Four other copies of the class remain in `crates/sweep/tests/`,
and none swaps mechanically:

- `lamina_annulus.rs` `pappus` and `revolve_axis_gaps.rs` `pappus_volume`:
  the shoelace first moment of a polygon profile. The shared module has
  no polygon piece (its `triangle` returns an unsigned area, so a fan
  of triangles loses a reflex polygon's signs). The fix is a `polygon`
  piece in `test_support::pappus`, then the two swaps.
- `the_football_revolves.rs` `pappus`: a circular segment cut by the
  axis, with an area output beside the volume; `segment` covers the
  volume half.
- `band_closed_mixed_chain_shapes.rs` `Outline::pappus`: a fillet's swept
  cross-section summed along an outline (`a·L + a·θ·r_c` per piece), not
  a profile's first moment. It may not belong to the class at all.

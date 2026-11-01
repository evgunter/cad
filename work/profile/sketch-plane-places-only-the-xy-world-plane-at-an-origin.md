---
id: sketch-plane-places-only-the-xy-world-plane-at-an-origin
kind: issue
title: SketchPlane has xy_at but no yz_at or zx_at, so a sketch on a placed world yz or zx plane reaches through RigidMotion
status: open
opened: 2026-10-21
---

## What

Found by FRAME's `frame/exact-frames-are-turns` (PR 5573). It moved
the sweep fixtures' axis-aligned planes off the Gram–Schmidt mint.
`profile::SketchPlane` has three world-plane doors at the coordinate
origin (`xy`, `yz`, `zx`), but only one at a chosen origin
(`xy_at`). So a fixture that means "the world yz plane at x = 1.5"
writes `SketchPlane::from_frame(RigidMotion::axes_yz(o))`. Eleven
sites in `crates/sweep/tests` and two in
`crates/stl/tests/review_m3_pr55_e2e.rs` spell it that way, for
example in `band_dihedral_cups.rs`, `band_planar_cut_off*.rs` and
`a_keel_ring_in_an_elliptic_section.rs`.

The unit wrote a consumer-side `sketch_yz_at`, and the review asked
for it to come out again: a library gap is filed, not filled in a
consumer.

## The two shapes

1. Add `yz_at` / `zx_at` beside `xy_at`.
2. Retire `xy_at` (`work/lib/log.md` already names it a retirement
   candidate) and let `from_frame(RigidMotion::axes_*(o))` be the one
   spelling of a placed world plane.

Note the naming trap the review found: a plane "xz" (u = x̂, v = ẑ,
normal −ŷ) and `zx` (u = ẑ, v = x̂, normal +ŷ) are the same plane with
opposite normals under transposed letters. Whichever shape is chosen
should not add an `xz` door.

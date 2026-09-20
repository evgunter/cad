---
id: the-quad-sheet-helper-is-written-three-times-across-two-chart-region-files
kind: issue
title: The xy-plane quad-sheet helper is written three times across chart_region.rs and chart_region_r2_probes.rs
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/topo/src/chart_region.rs` (two copies, in two
  separate `mod` blocks: `xy_plane`/`sheet` at `:4151`/`:4162`, and
  `xy_plane`/`sheet` again at `:4858`/`:4879`) against
  `crates/topo/src/chart_region_r2_probes.rs`
  (`face_of` `:20`, `xy_plane` `:36`, `sheet` `:51`). `face_of` is a
  fourth spelling, at `chart_region.rs:3695` and
  `chart_region_r2_probes.rs:20`.
- **Importance**: low-medium
- **Confidence**: sure by reading; not dumped
- **Raised by**: the PR 2843 fix pass, 2026-09-19, out of the
  `find_half_edge(seed.face` sweep run for
  `the-cube-sequence-is-written-five-times-and-twice-inside-src`. Three
  of that sweep's 26 hits are these, and they are **not** cubes, so they
  belong to a different class and get their own row rather than riding
  that one.

## The shape

`sheet` is one `mvfs` at a rectangle corner, three `mev_line`s round
the rim, a `find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)` and one
`mef` closing the quad onto a plane — a one-face sheet, not a solid, and
so neither a `prism_ops` body nor a member of the cube family. The two
`chart_region.rs` copies differ from each other in exactly one place:
the first takes its `FaceSurface` as an argument, the second inlines
`FaceSurface::New(xy_plane())`. `chart_region_r2_probes.rs`'s is the
first of those with `tol` threaded as a parameter instead of
`Tol::witness()` inline.

`xy_plane` is a bare `newell_plane` over the unit square's four corners,
written out three times.

## What this row is NOT

It is not "hoist them into `test_support_fixtures`". A single-face sheet
is a tier-1/2 chart fixture, and the file it would move to is the
**Euler-op fixture family**, whose doors all build closed bodies with
certified chords. The candidate homes are the two `chart_region.rs`
`mod` blocks merging their pair, and `chart_region_r2_probes.rs`
naming it from `chart_region`'s test module (or the reverse) — which is
a visibility question this row has not measured.

## What is unmeasured

- Whether `chart_region_r2_probes.rs` can name a `#[cfg(test)] mod`
  item of `chart_region.rs` at all, and at what cost.
- Whether the two `chart_region.rs` copies are in the same `cfg` arm.
- `face_of`'s two copies were read, not diffed.

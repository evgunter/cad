---
id: the-ring-paths-plane-readings-beside-plane-roots-still-decide-on-charged-heights
kind: issue
title: The ring path's readings beside plane_roots (coplanar_apart, misses_in_plane, segment_misses, split_ring_path_segment_side) still decide on sec.plane.height(x) charged path_resolution's far-carrier slack
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Filed by GERM's `germ/plane-pose-far-frame` lane (PR 7411) from its
FULL review.

## Finding

PR 7411 moved `ring_path::plane_roots` (`crates/topo/src/ring_path.rs`)
onto `curve_face::cross`, uncharged. The readings beside it still decide
on the section plane's heights charged by `slack`, which is
`path_resolution`'s fold:

- `split_ring_path_segment_side`: `sec.plane.height(x)` at a ruling
  segment's ends, charged;
- `segment_misses` (`split_ring_path_segment_apart`), `misses_in_plane`
  and `coplanar_apart` (`split_ring_path_coplanar_apart`), each charged
  `slack`.

`path_resolution` folds each conic arc's own resolution
(`geom_brep::carrier_resolution::resolution`, the far-carrier term) into
that `slack`. Part of its stated reason, "the rounding its stored
semi-axes and far centre amplify" in the plane's offset, is gone: the
offset is now read through `Frame::height`, which is
`geom::plane_offset`, posed well.

## Settles when

Each of these readings is posed well where it decides (its height
through `plane_offset`, its distances re-posed where they cancel), with
no `slack` charge. The far-carrier term leaves `path_resolution` (or the
function goes), and far-frame rows at ε 1e-12 pass uncharged.

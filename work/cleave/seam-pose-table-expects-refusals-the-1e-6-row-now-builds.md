---
id: seam-pose-table-expects-refusals-the-1e-6-row-now-builds
kind: issue
title: section_ellipse_seam_poses' table expects at-rest refusals at theta 3e-5 that build ok at eps 1e-6 (main red)
status: closed
opened: 2026-10-19
priority: P2
cost: E
refs: [a-split-leaves-an-in-band-sliver-wall-face-that-refuses-only-at-rest]
closed: 2026-10-26
---


Found by GERM's circle × torus lane running sweep at `CAD_TOLERANCE_EPS=1e-6`
(branch `germ/circle-torus-recentering`), and reproduced on main at
`c5f076966e` with that branch's change absent:
`sweep::all section_ellipse_seam_poses::planes_nearly_along_the_axis_through_a_seam_vertex_split`
(`crates/sweep/tests/section_ellipse_seam_poses.rs`, the table assertion
after the cut loop) fails with

```
poses off their table (...):
θ 3e-5 (1, 0) c 0: ok (want at-rest gate above VolumeUncomputable)
θ 3e-5 (1, 0) c 0.5: ok (want at-rest gate above VolumeUncomputable)
θ 3e-5 (1, 0) c -0.5: ok (want at-rest gate above VolumeUncomputable)
θ 3e-5 (1, 0) c 0.8: ok (want at-rest gate above VolumeUncomputable)
θ 3e-5 (1, 0) c -0.8: ok (want at-rest gate above VolumeUncomputable)
```

The five poses build at 1e-6 where the table (re-baselined by cleave's
fix pass, `5c9231d282`) wants the at-rest volume refusal, so the
expectation is ε-dependent: either the table is read per ε or these rows
are re-baselined. It is green at the default ε. Not measured at 1e-12.

## Closed (2026-10-26)

Fixed by PR 5381 (`6e23a75a6f`): `COIN_FLIP_1E6` is empty and read per ε (`row(eps, [...])` in `crates/sweep/tests/section_ellipse_seam_poses.rs`); the 1e-6 rows build since PR 5334's face merge. What remains of the sliver is `a-split-leaves-an-in-band-sliver-wall-face-that-refuses-only-at-rest`. (CLEAVE slate triage)

---
id: main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused
kind: issue
title: Main red (nightly): thin_nested_rings_build's w = 1e-3 rings now mesh at δ 5e-3, and its pin expects every op to refuse Triangulation
status: closed
opened: 2026-10-26
closed: 2026-10-28
priority: P1
pr: 5911
branch: tess/thin-rings-repin
refs: [a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta, a-thin-nested-rings-row-pins-untriangulated-where-the-mesh-now-meshes]
cost: E
---


Filed by CLEAVE's ring-tie lane (PR 5753). Found while sweeping the
`sweep` crate.

## What

`crates/sweep/tests/an_annular_tube_through_a_plate.rs`
`thin_nested_rings_build` fails on main `b2d32e9b89` at the default ε
(`cargo nextest run -p sweep -E 'test(thin_nested_rings_build)'`), and
the same on PR 5753's head. The test is in the slow set, so the
per-PR gate does not run it and the nightly is red on it.

The test pins the `w = 1e-3` rings as refusing
`TessellateError::Triangulation` on every op (`untriangulated:
&ALL_OPS`, in `every_op`). All six now mesh:

```
thin rings w = 0.001: A ∪ B: the mesh Ok("meshed")
thin rings w = 0.001: B ∪ A: the mesh Ok("meshed")
thin rings w = 0.001: A ∩ B: the mesh Ok("meshed")
thin rings w = 0.001: B ∩ A: the mesh Ok("meshed")
thin rings w = 0.001: A ∖ B: the mesh Ok("meshed")
thin rings w = 0.001: B ∖ A: the mesh Ok("meshed")
```

The pin is the evidence `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`
cites. Either the tessellator now meshes those faces, which may close
part of that row, or the booleans now hand it different faces.

## What it needs

1. Find the merge that moved it.
2. If the meshes are right (`check_mesh` passes), re-pin the test and
   update the thin-arc row. Otherwise, fix the regression.

Ownership: the pinned behaviour is TESS's (the refusal row); the test
file is TCOST's and TINT's.


## Bisect (QUAD triage, 2026-10-27)

A first-parent bisect at the default ε over `1ce7a33c..ab2b6108` names
`463956a6`, the merge of **PR 5717** (`pred/mesh-sizing-rule`). Its first
parent `5998b4f6` is green. The row is still red on main `3a0cd80f`.

## Closed (2026-10-28, PR 5911, `tess/thin-rings-repin`)

The meshes are right, so the test is re-pinned rather than the mesher
changed. PR 5717 sizes each lane at the coarsest step its own theorem
grants, and since then all 18 runs mesh: six ops × w ∈ {1e-3, 9e-4,
1e-4}. On main only w = 1e-3 had ever run, because the test stopped at
its first failure. Every one of the 18 meshes passes `check_mesh`.
`check_mesh` checks combinatorics only, so the mesh `signed_volume` was
also measured once against the closed form. It was positive and within
1.8e-5 on every op; the thin ∩ and ∖ ops come out about 0.6% low, as an
inscribed chord polygon should.

The `untriangulated` list is now empty. It was the only field of
`Known`, so `Known`, `ALL_OPS` and the `known` parameter are removed.
`every_op` now holds every mesh to `check_mesh` and counts any
tessellation error as a failure. The test is green at ε default, 1e-6
and 1e-12.

The thin-arc row this pin evidenced was re-measured in its
`## Re-measured (2026-10-28)` section and stays open. In the C-shape
table, w = 0.04 and 0.02 at δ 0.1 now mesh, and w = 0.01 still refuses
at δ 0.5 and 0.1. In the tube-alone table, one cell moved from Ok to
refused.

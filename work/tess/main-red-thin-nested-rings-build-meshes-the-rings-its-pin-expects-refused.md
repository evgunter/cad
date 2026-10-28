---
id: main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused
kind: issue
title: Main red (nightly): thin_nested_rings_build's w = 1e-3 rings now mesh at δ 5e-3, and its pin expects every op to refuse Triangulation
status: open
opened: 2026-10-26
priority: P1
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

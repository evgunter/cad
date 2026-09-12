---
id: bvh-is-the-index-after-the-memo
kind: issue
title: after the patch memo, a recomputed root's whole-mesh BVH is the pick index's largest term
status: open
opened: 2026-09-11
---


## The residue

PERF-5's memos make a recomputed root's tessellation proportional to
the faces that changed, and leave its triangle BVH (`MeshPick::build`,
`crates/editor-core/src/resolve/pick.rs`, over `crates/bvh`) rebuilt
whole. Measured on the tree that landed them (release, 4 vCPU, the
`perf/explore-gui` stage harness, medians of 3):

| document | index, memo primed | = Σ roots' tessellate_with | + Σ roots' MeshPick::build | + the rest (id map, windows) |
|---|---:|---:|---:|---:|
| `die_composed_tour` (one root, 184 034 triangles, one-pip edit: 85 of 89 faces reused) | 174 ms | 25 ms | 148 ms | 0.7 ms |
| `tube_ring` (683 672 triangles, an edit every face reads) | 2438 ms | 1689 ms | 700 ms | 49 ms |
| `gallery_ring` (160 260 triangles, likewise) | 439 ms | 311 ms | 123 ms | 5 ms |

(The `MeshPick::build` column is that call re-run on the built parts,
the tessellation column the per-root time less it; a loaded box, so
the absolute numbers are ~1.8× the PR's first table and the shares are
the reading.) On the tour die the BVH is now ~85 % of the memo'd index
build and the whole of the edit→picture wait's index share; on the
ring documents it is ~28 % and stays behind a tessellation the memo
cannot help (their bump moves every face).

## What a fix is

"Stop doing this" again, one level down: a `NodePick` that changed in
four of eighty-nine faces does not need its BVH built from nothing.
Either a per-face BVH with a top-level tree over the faces' boxes (the
unchanged faces' subtrees are reused whole, keyed like the patches),
or a refit of the existing tree's boxes where only positions moved —
the first is the shape that matches the patch memo. Out of PERF-5's
fence (spec §5); `crates/bvh` is the home.

---
id: seam-edge-between-two-merged-faces-refusal-a-legal-declared-union-reaches
kind: issue
title: A legal declared union reaches the seam-edge-between-two-merged-faces Emission refusal
status: closed
opened: 2026-09-23
priority: P0
cost: D
branch: emit/seam-edge-merged-faces
closed: 2026-09-23
pr: 3120
---


## What

`emit_topo`'s seam-edge pass refuses a seam edge whose two crossing
faces are BOTH merged faces as `NamingError::Emission("seam edge
between two merged faces (unsupported)")`
(`crates/editor-core/src/names/emit_topo.rs`, the merged-face
read-through inside `name_boolean_edges`, ~864). An
`Emission` says the kernel is broken. The documents below are legal
declared unions of ordinary blocks, so either the refusal is a
missing rule and should say so (`NamingError::SeamVertexParentage`
and `SharedRim` are the precedents,
`crates/editor-core/src/names/emit.rs`), or a rule is owed.

## Evidence

Measured 2026-09-23 on `origin/main` at `d2578ac26`, with and without
the seam junction fix (same counts both ways). Blocks from
`docm7_union_declare::block`; `a` = x∈(0,1), `b` = x∈(0.5,1.5), both
y,z∈(0,1), declared `flush_pairs((a, a), (b, b))`; every order of the
listed members:

- `[a, b, c]`, `c` = x∈(0.3,0.4), y∈(0.5,2), z∈(0.5,3.5): orders
  `[b, c, a]` and `[c, b, a]` refuse this way (2 of 6; two others
  fuse).
- `[a, b, g, g2]`, `g` = x∈(0.3,0.4), `g2` = x∈(1.2,1.3), both
  y∈(-1,2), z∈(0.5,3.5): orders `[a, g2, b, g]` and `[g2, a, b, g]`
  (2 of 24).
- the same plus `m` = x∈(0.7,0.8) slab: 4 of 120.

Every other order of those documents refuses for reasons other rows
own (`DeclareResolve` Vanished,
`work/gather/member-space-look-through-stops-at-splits-containment-and-fragmented-merges.md`;
`SharedRim`), so nothing here says the documents fuse in another
order except the first.

## Why it matters

It is P0 because a normal verb (a declared union) breaks on normal
geometry (axis-aligned blocks), and it says the break is a kernel bug.

## Found by

The document sweep for `seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`.

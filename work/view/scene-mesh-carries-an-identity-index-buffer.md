---
id: scene-mesh-carries-an-identity-index-buffer
kind: issue
title: SceneMesh's index buffer is the identity permutation and costs 139 MB on one corpus document
status: open
opened: 2026-09-15
---


## The finding

`SceneMesh::build_parts_focused` (`crates/viewer/src/scene.rs`, near
`:439`) ends its walk with

```rust
let indices = (0..positions.len() as u32).collect();
```

— the identity permutation over the corners it just emitted. It carries
no information: `SceneMesh::indices()` has exactly one production
reader, `crates/viewer/src/gpu.rs` (near `:550`), which uploads it and
takes its length as the draw count, and exactly one test reader,
`crates/viewer/tests/scene_build.rs` (near `:74`), whose assertion is
literally `mesh.indices().len() == mesh.positions().len()` — a check
that the buffer says nothing.

The scene is non-indexed geometry by construction: the walk emits three
distinct corners per triangle (each carries that triangle's face
normal, so corners are not shared across triangles even when positions
coincide). `wgpu` draws that with `draw(0..vertex_count, ..)` and no
index buffer at all — which `gpu.rs` already does one pass over, for
the edge overlay (near `:1172`), so the shape is in the file already.

## What it costs

Measured while diagnosing `ui-thread-work-after-the-index-seam`'s hit
(2) — release, `viewer`'s own corpus, `hollow_tube_ring` at δ=1e-5,
11 605 976 triangles / 34 817 928 corners:

- **139 MB** of the picture's ~1.25 GB of vertex buffers, plus the same
  again in the GPU-side buffer;
- **52–61 ms** of `SceneMesh::build_parts_focused` on a warm heap and
  **462–474 ms** on the first build at that size — 8–11 % of the step
  either way.

That is a small share of a large step, which is why it is filed here
rather than taken with the diagnosis: the step's real problem is the
other 89 %, and removing the buffer changes `gpu.rs`'s draw call, which
no headless row on this machine can exercise (the adapter test needs a
real device).

## What a taker owes

`SceneMesh::indices` and its `scene_build` row go away together, and
`gpu.rs` swaps `draw_indexed` for `draw`. The assertion to write in
place of the old one is about the draw COUNT — that the renderer draws
`positions().len()` corners — which is a claim a bug could break, where
`indices().len() == positions().len()` is not.

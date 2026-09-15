---
id: scene-mesh-carries-an-identity-index-buffer
kind: issue
title: SceneMesh's index buffer is the identity permutation and costs 139 MB on one corpus document
status: closed
opened: 2026-09-15
closed: 2026-09-15
branch: view/index-buffer
pr: 2661
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


## Closed (2026-09-15, `view/index-buffer`)

`SceneMesh::indices`, the `indices: Vec<u32>` field and its
`(0..n).collect()` went; so did the `Geometry.indices` buffer, its
`BufferUsages::INDEX` upload and `index_count`. `gpu.rs` has **two**
scene passes, not one — the shaded pass and the id pass — and both
swapped `set_index_buffer` + `draw_indexed` for `draw(0..corners)`.
`corner_count` is the one place the draw range is derived.

**The replacement assertion.** `scene_build`'s
`indices().len() == positions().len()` went with the buffer. In its
place `gpu::tests::the_draw_range_is_the_length_of_every_buffer_the_
passes_bind` asserts the draw range is `stats().triangles * 3` and is
the length of each of the four per-corner tables the two passes bind —
`flags` among them, which nothing else in the suite measured — and
`both_scene_passes_draw_the_corner_count_with_no_index_buffer` reads
`gpu.rs` through `test_utils::source::code_only` for the two call
sites. Neither reaches a device; the hosted viewer-render rows are
what judge the drawn picture.

**Not re-measured.** The 139 MB / 8-11 % figures above are the
diagnosing lane's, at δ=1e-5 on `hollow_tube_ring`; this lane did not
rebuild that harness.

`work/chrome/gpu-index-counts-substitute-u32-max.md` names two sites,
one of them this file's `index_count`. That site is **relocated, not
resolved**: `corner_count` still spells
`u32::try_from(...).unwrap_or(u32::MAX)`, now over
`scene.positions().len()`. The typed refusal that row wants is CHROME's
to write.

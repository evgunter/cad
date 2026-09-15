---
id: gpu-index-counts-substitute-u32-max
kind: issue
title: gpu.rs substitutes u32::MAX for a draw count it could not convert
status: parked
opened: 2026-09-12
blocked_on: [scene-mesh-carries-an-identity-index-buffer]
---

## Finding

`crates/viewer/src/gpu.rs`, two sites — the scene geometry's
`index_count` (~`:558`) and the edge geometry's `vertices` (~`:956`):

```rust
index_count: u32::try_from(scene.indices().len()).unwrap_or(u32::MAX),
```

`u32::MAX` is a count the function did not compute, handed to a draw
call in the shape of one it did. It is the same class as
`viewer-grid-pitch-nonfinite-fallback` (closed): a refusal spelled as
a plausible reading. The consequence is a draw over a range that does
not exist rather than a refusal a caller could act on.

**Unreachable in practice and filed anyway.** Four billion indices is
past what the pipeline could hold, so this is not a live defect; it is
here because the closing PR of the grid-pitch row disclosed it in its
census and `work/README.md` wants a disclosed residue to have a file
rather than a line in a merged PR body. The honest fix is a typed
refusal in a path that today has none, which is why it was not carried
in that PR.

## Fence

`crates/viewer/src/gpu.rs` — CHROME's and VIEW's by the territories
table.

## Parked on VIEW's index-buffer row, 2026-09-15

`gpu.rs` is ceded to VIEW under the carve-out, and this row's two sites
are not equally affected. VIEW's `scene-mesh-carries-an-identity-index-
buffer` proposes deleting `SceneMesh::indices` and swapping
`draw_indexed` for the `draw` the edge overlay already uses — **which
deletes the `index_count` site this row is about**. The `vertices` site
survives, and VIEW's replacement draw count needs the same conversion,
so that change MOVES this row rather than closing it.

Parked rather than left open because a row nobody intends to work
should not read as available on the board. The trigger is a real item
lint can see close. Whoever takes either should take both — neither
program can see that from its own slate alone.

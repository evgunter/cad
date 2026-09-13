---
id: degenerate-triangle-normal-is-substituted
kind: issue
title: A degenerate triangle's substituted +Z normal is argued locally and unowned upstream
status: open
opened: 2026-09-12
---

## Finding

`crates/viewer/src/scene.rs`, `triangle_normal`:

```rust
if len > 0.0 && len.is_finite() { … } else { [0.0, 0.0, 1.0] }
```

**The substitution is argued at the site, and the argument is good.**
An earlier draft of this row said it was unargued; that was wrong, and
the sentence is corrected here rather than left for a lane to
discover. The doc says: *"A degenerate (zero-area) triangle has no
normal; it gets `+Z` rather than a NaN, because a NaN in a vertex
buffer poisons the shading of everything the rasterizer blends it with,
while a wrong-facing sliver is invisible at the size a degenerate
triangle has."* That names the consumer and weighs the two failures,
which is the standard this crate's other substitutions are held to
(`bounds.rs`' `Probe::new` passes it the same way).

## What is actually open

Not the substitution — **the missing contract above it**. This
function is handed triangles by `mesh::tessellate`, and nothing says
whether a returned `FacePatch` may contain a zero-area triangle. So
the viewer carries a display-side answer to a question about another
crate's output, on a path where it cannot refuse cheaply.

Two candidate resolutions, and the second is the better one:

1. Skip the degenerate triangle in the scene builder — cheap, but it
   is still the viewer deciding what a tessellation may contain.
2. **State the guarantee in `mesh`'s contract and pin it there.** If a
   patch may not contain zero-area triangles, this branch is dead and
   the doc should say which claim makes it dead, the way
   `datums::unit`'s fallback names `basis`' `1/√3` bound. If a patch
   MAY contain them, the viewer's `+Z` is correct and this row closes
   with a citation instead of a change.

Either way the answer is a sentence in `mesh`, not a rewrite here,
which is why this is a CHROME row about a seam rather than a defect
report about a line.

## Provenance

Disclosed in the census closing
`viewer-grid-pitch-nonfinite-fallback`, as a same-shape hit outside
that unit; given a file at disclosure per `work/README.md`.

## Fence

`crates/viewer/src/scene.rs` — CHROME's and VIEW's by the territories
table. The resolution likely touches `crates/mesh`, which is not.

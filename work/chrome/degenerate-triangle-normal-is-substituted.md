---
id: degenerate-triangle-normal-is-substituted
kind: issue
title: scene.rs substitutes +Z for a degenerate triangle's normal
status: open
opened: 2026-09-12
---

## Finding

`crates/viewer/src/scene.rs`, the face-normal helper (~`:1053`):

```rust
let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
if len > 0.0 && len.is_finite() {
    [ (n[0] / len) as f32, (n[1] / len) as f32, (n[2] / len) as f32 ]
} else {
    [0.0, 0.0, 1.0]
}
```

`+Z` is a direction the function did not compute, reaching the shader
as if it were one it did — the same class as
`viewer-grid-pitch-nonfinite-fallback` (closed), and unargued at the
site.

**Milder than that row and genuinely awkward to fix, which is why it
is filed rather than carried.** The consequence is one mis-lit sliver,
in a per-triangle hot path, on a mesh `mesh::tessellate` has already
accepted; refusing here would be a second opinion about another
crate's contract, which `Delta::new`'s doc thirty lines up explicitly
declines to give. The two candidate answers are a refusal that skips
the triangle, or a claim in `mesh`'s contract that a returned patch
has no degenerate triangles, pinned there — the second is the better
one and is not this crate's to make.

## Fence

`crates/viewer/src/scene.rs` — CHROME's and VIEW's by the territories
table.

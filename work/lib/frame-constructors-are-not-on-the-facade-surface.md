---
id: frame-constructors-are-not-on-the-facade-surface
kind: issue
title: the frame constructors are reachable only as pncad::geom_core::linalg::frame::*, while Python meets them as Frame methods
status: open
opened: 2026-09-12
---


## Finding

**Raised by SCALAR's S393 lane** while putting the tour's sweep cells
onto the kernel's start-frame door, from the seat of a demo (the demo
rule: awkwardness met writing a demo is a library finding,
`memories/demo-purpose.md`).

`geom_core::linalg::frame` carries the frame constructors —
`path_start_frame`, `point_at`, `mirror_across_plane`. A façade caller
reaches them only by their full module path:

```rust
use pncad::geom_core::linalg::frame::path_start_frame;
```

Five segments, through a re-exported crate, past two modules the façade
never otherwise asks a caller to name. `pncad::prelude` re-exports
`Affine3`, `Mat3`, `Point3`, `Vec3`, `Tol` and the whole numbers-and-
frames tier (`prelude.rs`, section 1) — every TYPE a frame is made of,
and none of the three doors that make one.

**Python meets the same doors at a much shorter spelling.**
`crates/pncad-py/src/py/place.rs` binds them as static methods on
`Frame`: `Frame.path_start_frame(origin, tangent)`,
`Frame.point_at(...)`, `Frame.mirror_across_plane(...)` — and supplies
`Tol::witness()` itself, so the Python caller passes two arguments where
the Rust caller passes three and a module path. The standing goal is
that every demo be authorable through the bindings and that the two
surfaces agree; here the Rust side is the awkward one.

**Where**: `crates/pncad/src/prelude.rs` (section 1, "Numbers and
frames"); the callers `demos/tour/src/skinned.rs` (sweep cell,
`tube_place`) and `crates/pncad-py/src/py/place.rs` for the Python
spelling.

**Confidence**: sure (the spelling is in the tree twice, both ways).

**Verdict:**

---
id: frame-constructors-are-not-on-the-facade-surface
kind: issue
title: the frame constructors are reachable only as pncad::geom_core::linalg::frame::*, while Python meets them as Frame methods
status: open
opened: 2026-09-12
priority: P3
cost: D
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

## What the spelling costs, measured (2026-09-15, S393's fix pass)

Three readings from the seat of the tour, each one a thing the Rust
caller pays and the Python caller does not:

- **The obvious call does not compile.**
  `path_start_frame(Point3::origin(), Vec3::unit_z(), Tol::witness())`
  is `error[E0283]: type annotations needed` — the door is generic over
  `T: Decide`, `Point3::origin()` and `Vec3::unit_z()` are generic
  constructors, and nothing in the expression pins `T`, so the user
  writes `Point3::<f64>::origin()` to get past it. A caller who HAS a
  curve never meets this (the arguments come from `eval`/`deriv` and
  carry the scalar), which is why it is invisible from inside the
  kernel and is the first thing a reader reaching for the door by hand
  hits. Python's `Frame.path_start_frame(origin, tangent)` has no such
  arm.
- **Five segments of module path**, above, for a door whose two
  siblings a caller reaches the same way.
- **A caller names the door for something it is not.** The door is
  spelled for a path's START and knows nothing about one — it takes a
  point and a tangent — so a loft placing every section by the normal
  plane writes its own wrapper over it
  (`demos/tour/src/skinned.rs`'s `tube_place(path, i, roll, tol)`)
  rather than calling a door named for what it wants. That half is
  PROPS's naming row
  (`path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start`);
  what belongs here is that the wrapper is the ergonomic consequence,
  and it is written once per caller.

**One more the same lane could not fix**: the twisted cubic is authored
twice, in `demos/tour/src/skinned.rs` (`cubic_spine`) and in
`crates/sweep/tests/turning_orientation.rs` (`torsion_path`) — the same
33 points at the same coefficients. Neither can share: a demo cannot
link a dev-gated test home, and the tour is not a dependency of the
suite. It is the same shape as the copies `S393` folded, one layer out,
and it has no fix that does not go through a shared fixture the façade
can reach.

**Where**: `crates/pncad/src/prelude.rs` (section 1, "Numbers and
frames"); the callers `demos/tour/src/skinned.rs` (sweep cell,
`tube_place`, `cubic_spine`) and `crates/pncad-py/src/py/place.rs` for
the Python spelling.

**Confidence**: sure (the spelling is in the tree twice, both ways).

**Verdict:**

---
id: path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start
kind: issue
title: path_start_frame is the only (point, tangent) -> frame door, and every interior-station caller reads wrong against its name
status: open
opened: 2026-09-12
---


## Finding

**Raised by SCALAR's S393 lane** while putting the tour's sweep and loft
cells onto `geom_core::linalg::frame::path_start_frame`.

The door's signature takes a point and a tangent and knows nothing about
a path or a start:

```rust
pub fn path_start_frame<T: Decide>(origin: Point3<T>, tangent: Vec3<T>, tol: Tol)
```

Its name and its whole doc block are about the START of a swept path
("The profile frame at the start of a swept path", "the departure half
of `nurbs(curve)`"). But it is the tree's ONLY spelling for *the normal
plane at a point of a curve*, and a loft is the caller that needs that
at every station, not only the first: `demos/tour/src/skinned.rs`'s
`tube_place` places six annular sections along the twisted cubic, and
now calls `path_start_frame` at all six. A reader of that scene meets a
call named for a start, five times over, at points that are not one.

`point_at` is not the alternative: it REFUSES rather than substituting a
roll reference, by design, so a caller who wants the conventional roll
has exactly one door and it is this one.

Two shapes the fix could take, neither decided here: a name that says
what the signature does (the frame from a point and a direction, with
the ladder roll), or a second entry point for the interior-station case
that shares the recipe — noting that the module's own policy note
already argues AGAINST a second door for roll, which is a composition.
`work/comb/S35.md` carries a neighbouring row on this door (that it was
justified by a deduplication it had not performed, and duplicates
`Vec3::orthonormal_basis`'s role with a different policy); S393 gave it
its first non-Python callers, so that row's premise has moved.

**Where**: `crates/geom-core/src/linalg/frame.rs`, `path_start_frame`
and its module-doc row in the companion table; callers
`demos/tour/src/skinned.rs` (`tube_place`),
`crates/sweep/tests/common/mod.rs` (`normal_start_place`).

**Confidence**: sure (the signature and the callers are both in tree).

**Verdict:**

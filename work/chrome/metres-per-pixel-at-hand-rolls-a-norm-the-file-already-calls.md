---
id: metres-per-pixel-at-hand-rolls-a-norm-the-file-already-calls
kind: issue
title: View::metres_per_pixel_at spells a 3-D length by hand four lines from a .norm() call
status: open
opened: 2026-09-21
priority: P4
cost: E
---



## Finding

Found by VGEOM's `vgeom/deletions` lane, re-running the hand-rolled
vector-op sweep that
`work/vgeom/viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`
says #2783 under-reported. `crates/viewer/src/datums.rs` is CHROME's
under the carve-out of 2026-09-15, so this is filed and not fixed.

`View::metres_per_pixel_at` spells a 3-D length by hand:

```rust
let depth = ((point.x - self.eye.x).powi(2)
    + (point.y - self.eye.y).powi(2)
    + (point.z - self.eye.z).powi(2))
.sqrt();
```

`point` and `self.eye` are both `Point3<f64>`, so this is
`(point - self.eye).norm()` — and the file already calls that door:
`View::basis` four lines above does `forward / forward.norm()`,
`forward.cross(self.up)`, `right / right.norm()`. Same module, same
types, two spellings of one operation.

**Why it is a row and not a nit.** #2783's sweep deleted this file's
`unit`, `cross` and `dot` helpers for exactly this reason, and it
stated its own blind spot as the `sqrt()` shape. This is that blind
spot's one surviving member in the file the sweep was run on. It is
also the only member of the class with a door already in use beside
it: everything else the re-run found is array-lowered and waits on
`work/linalg/geom-core-linalg-has-no-array-doors`, while this one
needs nothing new.

**What a fix has to preserve.** The refusal is on the PRODUCT and not
on the depth, deliberately — `screen_metres_at`'s doc says a scale
that is a length does not make every multiple of it one — and
`metres_per_pixel_at`'s own doc names the two refusals (`NaN` depth, a
depth of exactly zero) and argues against a floor. `Vec3::norm` must
answer a `NaN` for a `NaN` coordinate and a zero for a zero
separation for the substitution to stay refused; check that before
swapping, because a `norm` that floors or that hypots pairwise would
move the two arms this file's closed rows
(`metres-per-pixel-swallows-a-nan-depth`) were opened for.

**Where**: `crates/viewer/src/datums.rs`, `View::metres_per_pixel_at`.

**Confidence**: sure about the duplication; unsure only whether
`Vec3::norm`'s degenerate behaviour matches, which is the one thing a
taker must measure rather than read.

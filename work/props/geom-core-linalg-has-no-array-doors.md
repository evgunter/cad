---
id: geom-core-linalg-has-no-array-doors
kind: issue
title: geom-core linalg has no [f64;3] or [[f64;3];3] conversions, so every stored-array frame in editor-core lowers to Vec3/Mat3 by hand
status: open
opened: 2026-09-11
refs: [2375]
---


## Finding

Reported by WIRE's `placement-affine-map` lane (PR 2375) from its
sweep, outside its fence; filed here by the WIRE orchestrator, because
`crates/geom-core/src/*` is PROPS's glob and PROPS's `keep_out` already
names a linalg lane.

**The absence.** There is no `[f64; 3] ↔ Vec3<f64>` and no
`[[f64; 3]; 3] ↔ Mat3<f64>` conversion anywhere in
`crates/geom-core/src/linalg/` — no `From`, no `from_array`, no
`to_array`. `Mat3::map`, `Affine3::map`, `Point2::map` and their
siblings all exist and are the crate's stated convention
(`crates/geom/src/scalar_lift.rs:12-23`: *"one name, `map_scalar` on
every geometry type and `map` on every leaf"*), but they lift between
SCALARS. Nothing lifts between the stored array form and the geometry
type, so a struct that stores coordinates as arrays — which is what the
persisted layer does — has to spell the construction out.

**Why that is a finding and not a taste.** It is the reason PR 2375
could collapse three of `placement.rs`'s hand lifts and not the rest.
Once `Frame::affine_f64` existed, `linear`, `affine`, `determinant` and
`compose` all became one call each; the two that remain remain because
the array→`Vec3` step has no door to delegate to.

**The consumers, named** (accurate at `dc251ce`, none of them PROPS's to
edit — they are the evidence, not the work):

- `crates/editor-core/src/placement.rs` — `Frame::linear_f64` and
  `rotate_then_translate`, after PR 2375. The first is now the file's
  single lowering site, which is as far as that PR could take it.
- `crates/editor-core/src/mate.rs:141-143` — `MateFrame::placement`
  lifts three stored `[f64; 3]` into `Point3`/`Vec3` by hand.
  Irreducible today for exactly this reason. `mate.rs` is claimed by
  both DOCM and MSOLVE, which is why it is named here as a consumer
  rather than filed as its own row on contested ground.
- `crates/editor-core/src/mate/solve.rs:373-377,411` — four `Vec3::new`
  over literal constants where `Vec3::zero()` / the unit constructors
  exist. A neighbour of the class rather than a member: it needs no new
  door, only the existing ones.

## Widened by PR 2375's style review: FIVE non-test sites, and one crate
## has already written the door by hand

The review swept differently — a 6-line-window scan for `[0]`/`[1]`/`[2]`
triples over **all** `crates/*/src` with `#[cfg(test)]` classified, plus
a `from_f64`-cluster scan and the inverse `[x.x, x.y, x.z]` scan — and
found the class is five sites, not the two the lane's report named.
Confidence `sure`, the reviewer's.

The one that changes the shape of this row:
**`crates/geom-brep/src/ssi/system.rs:92` and `:96` already carry the
missing function, privately and by hand** —

```rust
fn v3(x: &[f64; 3]) -> Vec3<f64>
fn p3(x: &[f64; 3]) -> Point3<f64>
```

— i.e. a third crate has written `geom-core`'s absent door for itself
rather than gone without it. That is the strongest possible evidence the
door belongs in the library: not "several sites would be shorter" but
"a consumer has already built it where it could not be shared."

The full non-test list: `ssi/system.rs:92`, `ssi/system.rs:96`,
`crates/geom-brep/src/ssi.rs:827`, `crates/viewer/src/datums.rs:550`,
`crates/editor-core/src/mate.rs:141`, plus `placement.rs`'s two
remaining lowerings.

**The scope this fixes.** The row was filed as a `Mat3`/`Affine3` array
door; the review's reading is that the schedule must carry the
**`Vec3`/`Point3` array pair with it**, because that pair is what
`ssi/system.rs` hand-rolled and what four of the five sites want. A fix
that lands `to_cols_array`/`from_cols_array` on the matrix types and
leaves the vector types alone is a **half-fix** in the style brief's
sense, and should be labelled one.

**Where else to look.** `rg 'Vec3::new\(|Mat3::from_cols\(' crates/*/src`
over any struct storing `[f64; 3]`. The lane's own blind spots, stated:
lifts through intermediate locals, variable- or slice-indexed lifts in
loops, lifts into types outside `Vec2/Vec3/Point2/Point3/Mat3/Affine3`,
and everything outside `crates/editor-core/src/`. The review's sweep
closes the last of those; the first three are still open.

**What it is NOT.** Not a request to make the arrays go away: the stored
array form is the persisted shape and is D7/D9 territory. The ask is a
door in the direction the crate already has doors in every other
direction.

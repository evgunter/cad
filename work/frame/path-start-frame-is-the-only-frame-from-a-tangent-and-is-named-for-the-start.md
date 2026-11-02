---
id: path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start
kind: issue
title: path_start_frame is the only (point, tangent) -> frame door, and every interior-station caller reads wrong against its name
status: open
opened: 2026-09-12
priority: P1
cost: M
design: true
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

**One more thing a reader of that name does not expect, and it is a
DISCONTINUITY, not a naming complaint** (added 2026-09-15): crossing
the ladder's pole band flips the frame's roll by 180°. A tangent
`(k·ε, 0, 1)` has `|Ẑ × n̂| = k·ε`, so the first rung decides
definitely for `k ≥ K` (K = 10 by default) and gives
`x̂ = normalize(Ẑ × n̂) = (0, 1, 0)`; in band — `k ≤ 5`, say — the rung
is not taken, the ladder advances to world +X, and
`x̂ = normalize(X̂ × n̂) = (0, −1, 0)`, the exact negation. The module
documents the ladder and documents that each rung needs a definite
decision, so nothing here is undocumented; what is nowhere SHOWN is
that the consequence at the crossover is a half turn of the profile,
which for an asymmetric section is a visibly different body from a
tangent that moved by `5·ε`. Worth a sentence in the demo tour's sweep
narration one day — the same place that now says roll is a
composition.

**Where**: `crates/geom-core/src/linalg/frame.rs`, `path_start_frame`
and its module-doc row in the companion table; callers
`demos/tour/src/skinned.rs` (`tube_place`),
`crates/sweep/tests/common/mod.rs` (`normal_start_place`).

**Confidence**: sure (the signature and the callers are both in tree).

**Verdict:**

## Parked under the D10 hold (2026-10-21)

FRAME orchestrator. `path_start_frame` has no kernel caller; it is an authoring door (`pncad-py/src/py/place.rs`, the tour, `pncad/examples/sweep_start_frame_and_roll.rs`). D′ (`a-placement-pins-by-written-values-without-a-mate`; the helpers fork on `[ev]` PR 5494's branch) retires the placement helpers for one constructor from the author's mates and moves, and D10 `[placement]` says no construction reads a frame, so whether this door is renamed, split or retired is decided there first. Its world +Z/+X ladder is also the shape `minted-reference-from-inputs.sh`'s KNOWN GAPS names (a reference picked off the axes without `orthonormal_basis`). Re-priced `M` with `design: true` from legacy `D`.

## Released (INTENT, 2026-10-26)

This row was parked on the stage-3 placement pivot (`a-placement-pins-by-written-values-without-a-mate`). All three of its PRs are merged: D′1 (PR 5592), D′2 (PR 5613) and D′3 (PR 5636, `29110056e8`). Re-check this row against main, then fix, narrow or close it. That is the owning track's call.

## A caller gone, and a second door for interior frames (CARVE, 2026-10-31)

Evidence from `carve/sweep-frame-is-a-minimal-rotation-from-the-start-tangent`;
nothing here is fixed by it.

- **The klein scene's hand-built interior frames are gone.**
  `demos/tour/src/klein.rs` used to loft its loop through station
  frames it minted itself (`lofted_loop`: `RigidMotion::from_aim_and_reference`
  on each station's tangent, rolled by the `path_start_frame` it read
  at the start), because `sweep_body` refused a half-turn spine. The
  sweep now carries the start frame along the path, so the loop is one
  `sweep_body` again and the scene calls `path_start_frame` at the
  spine's start only. The interior-station callers this row names
  (`demos/tour/src/skinned.rs`'s `tube_place`,
  `crates/sweep/tests/common/mod.rs`) are unchanged.
- **`sweep::sweep_places` is now a door for frames at interior points
  of a path**: it returns the author's start frame carried by the
  rotation-minimizing frame to each of `n` uniform stations
  (`crates/sweep/README.md` `[sweep.frame]`). A loft that wants the
  normal plane at its stations can read them there instead of calling
  `path_start_frame` at each, and those frames do not flip roll where
  a tangent crosses the ladder's pole band (the discontinuity above),
  because only the start frame reads the ladder. That bears on the
  row's two shapes: the interior-station case may already have its
  door, and what is left is the name.

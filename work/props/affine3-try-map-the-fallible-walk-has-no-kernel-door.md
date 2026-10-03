---
id: affine3-try-map-the-fallible-walk-has-no-kernel-door
kind: issue
title: Affine3::try_map: the fallible per-coordinate walk over an Affine3 has no kernel door, so editor-core keeps a private one
status: closed
closed: 2026-09-16
opened: 2026-09-08
pr: 2743
branch: props/affine-try-map
---

The per-coordinate walk over an `Affine3` — twelve components through
one function, the three columns and the translation kept in their
places — has an infallible door in the kernel, `Affine3::map<U>(self,
f: impl Fn(T) -> U)` (`crates/geom-core/src/linalg/affine.rs`, spelled
through `Mat3::map` and `Vec3::map`), and `SketchPlane::map` over it
(`crates/profile/src/lib.rs`). It has no fallible door.
`crates/editor-core/src/eval/anchor.rs` `map_affine<A, B, E>(a:
&Affine3<A>, f: impl Fn(A) -> Result<B, E>) -> Result<Affine3<B>, E>`
is that direction written out again, privately. Its one caller is
`eval/wire.rs` `pinned_plane`: a lane → `f64` crossing where every
component goes through `SectionScalar::pinned_f64`, which answers
`None` on any analysis scalar — a refusal `map`'s infallible `f`
cannot spell.

The argument for the kernel owning the fallible direction too is the
one `editor-core` makes for writing the walk once at all: a transposed
`c1`/`c2` is invisible in review and identical in every copy but one,
and today the walk has two copies with two owners. The proposed door:

```rust
impl<T: Real> Affine3<T> {
    pub fn try_map<U: Real, E>(self, f: impl Fn(T) -> Result<U, E>)
        -> Result<Affine3<U>, E>;
}
```

with `map` its infallible specialisation, and — since `Affine3::map`
descends through `Mat3::map` and `Vec3::map` — presumably `try_map`
on those two as well, so the fallible walk keeps the structural shape
the infallible one has (no arithmetic; exact whenever `f` is). Whether
`map` is then written as `try_map` with an `Infallible` error or kept
as its own body is PROPS' call; the two are bit-identical either way.

The geom-core vector doors are PROPS' (`work/eval/program.md`
`keep_out`), so EVAL-1 did not build this. This file is the note EVAL-1
wrote for PROPS to place; `work/eval/map-affine-retires-into-affine3-try-map.md`
is parked on it and retires `map_affine` in the PR that adopts the
door. Filed from outside PROPS' fence; the orchestrator carries it to
PROPS' board.

## Placed on PROPS' slate (EVAL orchestrator, 2026-09-08)

Moved from `work/issues/` at EVAL-1's merge: the owner is clear
(`work/README.md` — file onto the owner's slate, `issues/` is not a
waiting room), so the file is here rather than waiting. Three findings
from EVAL-1's style review belong to the same door family and are
folded in rather than filed thrice:

- **`SketchPlane::try_map` beside `Affine3::try_map`.** `pinned_plane`
  is `map_affine(&plane.placement, ..).ok().map(SketchPlane::new)` — the
  fallible direction still carries the `SketchPlane::new(<walk over the
  placement>)` shape that `SketchPlane::map` (PR 1977) exists to
  retire in the infallible one. A door on `Affine3` alone leaves it.
- **The twelve-by-name READOUT walks are the same class the other
  way.** `editor-core/src/placement.rs:169`–`:176` (`Frame::from_affine`),
  `mate/coset.rs:149` (`Coset::eq`), `tests/fixture/seat.rs:85`,
  `profile/src/lib.rs:549` (`bit_eq`), `profile/tests/sketch_plane.rs:154`,
  `topo/src/separation.rs:238` each spell the twelve components by name
  to read them out; the "transposed `c1`/`c2` is invisible in review"
  argument applies to every one. A kernel `Affine3::components() ->
  [T; 12]` (or `cols() -> [Vec3<T>; 4]`) would be one home for reading,
  as `map`/`try_map` are for lifting — worth minting together.
- **`SketchPlane::map`'s doc states a rule with no home.**
  `crates/profile/src/lib.rs:462`–`:485` (PR 1977's text) ends with "a
  caller chooses" between lifting the stored normal whole and
  rebuilding the frame at the target scalar; each caller decides in
  prose and none of the callers (`eval/wire.rs`'s `Pinned` lift,
  `anchor.rs`'s `embed_profile`, `sweep/src/loft.rs:242`,`:276`) says
  which it chose or why. Either the door decides, or the choice is a
  typed argument.

## Closed

Landed on PR #2743 (run 35078417437 green on the fix head). The kernel
owns the fallible per-coordinate walk now: `Vec3::try_map`,
`Mat3::try_map`, `Affine3::try_map`, with `SketchPlane::try_map` over
them in `profile`. First refusal returned, no arithmetic, columns and
translation in their places.

The open call this item left to PROPS — whether `map` keeps its own
body or becomes `try_map` over `Infallible` — went the second way, for
the item's own reason: two bodies would have moved the walk from two
copies with two owners to two copies with one owner, which is the same
defect smaller. Bit identity is therefore not pinned by `map` against
`try_map`, which is now a tautology, but by measuring EACH against a
hand-written third spelling that neither calls, over a corpus carrying
signed zeros, the largest subnormal, the infinities and a NaN.

The suite is what this unit should be remembered for. A transposed
column reds eleven rows; removing the short-circuit reds exactly one,
which is the sharpest evidence available that the rows are independent.
The lane's own lifting row first read the walk's output back THROUGH
the walk, so a column swap was re-applied by the readout and cancelled
— it passed under the mutation it was named for. The lane found that
itself, fixed it to read through an independent spelling, and wrote the
lesson into the row; the review lane reproduced the near-miss in both
directions.

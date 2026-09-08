---
id: affine3-try-map-the-fallible-walk-has-no-kernel-door
kind: issue
title: Affine3::try_map: the fallible per-coordinate walk over an Affine3 has no kernel door, so editor-core keeps a private one
status: open
opened: 2026-09-08
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

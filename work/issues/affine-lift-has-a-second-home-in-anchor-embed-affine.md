---
id: affine-lift-has-a-second-home-in-anchor-embed-affine
kind: issue
title: Affine3::map has a second home: editor-core's anchor::map_affine/embed_affine walk the twelve components again, and wire.rs:1132 is placement.map(T::from_f64) by another name
status: open
opened: 2026-09-05
refs: [1977]
---

The per-coordinate walk over an `Affine3` — twelve components through
one function, the three columns and the translation kept in their
places — has two homes. The kernel's is `Affine3::map`
(`crates/geom-core/src/linalg/affine.rs`), infallible, `f: Fn(T) -> U`,
with `SketchPlane::map` (`crates/profile/src/lib.rs`, PR 1977) the lift
of the type that carries a frame. The other is
`crates/editor-core/src/eval/anchor.rs:248` `map_affine` — the same
walk written out again, fallible (`f: Fn(A) -> Result<B, E>`) — and
`:268` `embed_affine`, which is `map_affine` with `T::from_f64` and an
`Infallible` error: exactly `Affine3::map(T::from_f64)` by another
name. Its two callers:

- `crates/editor-core/src/eval/wire.rs:1132` —
  `profile::SketchPlane::new(anchor::embed_affine::<T>(&placement.placement))`
  is `placement.map(T::from_f64)` since PR 1977, and is the non-tour
  consumer that door was minted for.
- `crates/editor-core/src/eval/wire.rs:784` (`pinned_plane`) — the
  lane → `f64` direction through `map_affine` with
  `x.pinned_f64().ok_or(())`, which `Affine3::map`'s infallible `f`
  cannot spell.

So the fix has two halves. `embed_affine` and the `wire.rs:1132` site
retire into `SketchPlane::map` / `Affine3::map`. `map_affine` is then
kept only for the fallible direction, and the question is whether that
direction wants a `try_map` on `Affine3` (one walk, in the kernel,
fallible; `map` its infallible specialisation) or stays where it is
as the one fallible caller's private walk — `anchor.rs`'s own doc
argues the walk is written once because a transposed `c1`/`c2` is
invisible in review, which is an argument for the kernel owning it.
Outside PR 1977's fence (editor-core); reported there and filed here
for the owner of `editor-core/src/eval/` to place.

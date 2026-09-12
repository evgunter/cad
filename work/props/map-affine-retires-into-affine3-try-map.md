---
id: map-affine-retires-into-affine3-try-map
kind: issue
title: anchor::map_affine retires into Affine3::try_map in the PR that adopts it
status: parked
opened: 2026-09-08
blocked_on: [affine3-try-map-the-fallible-walk-has-no-kernel-door]
refs: [2139]
---

`crates/editor-core/src/eval/anchor.rs` `map_affine` is the fallible
per-coordinate walk over an `Affine3` — twelve components through one
`f: Fn(A) -> Result<B, E>`, columns and translation kept in place, the
first refusal returned. It is private and has one caller,
`eval/wire.rs` `pinned_plane`, the lane → `f64` crossing where a
component refuses by the scalar's type. The kernel owns the infallible
direction (`Affine3::map`, `crates/geom-core/src/linalg/affine.rs`;
`SketchPlane::map` over it) and offers no fallible one; whether it
should is put to PROPS as
`work/issues/affine3-try-map-the-fallible-walk-has-no-kernel-door.md`,
which this row waits on.

When that door is minted, EVAL retires `map_affine` and routes
`pinned_plane` through it in a small PR of its own (`anchor.rs` and
`wire.rs` are EVAL's ground; PROPS' adopting PR may do it instead by
announced seam) — not before, and never as a third spelling beside
both. `pinned_plane` still spells `SketchPlane::new(<walk>)`, so the
door wants a `SketchPlane::try_map` beside `Affine3::try_map` or the
shape survives the retirement; the note says so. Filed by EVAL-1 at the moment the note was
written (`work/README.md`: disclosing a residue is not scheduling it).

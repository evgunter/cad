---
id: sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary
kind: issue
title: SketchPlane stores only the Affine3, so the frame witness dies at the read-back boundary
status: open
opened: 2026-09-15
priority: P0
cost: H
---

## What

Raised in review of FRAME-WITNESS (PR 2675) and filed at its fix pass.

`profile::SketchPlane<T>` is `{ pub placement: Affine3<T> }`.
`SketchPlane::from_frame(frame: OrthoFrame<T>)` is the only door that
MINTS one, and it immediately throws the witness away: it stores
`frame.to_affine()` and nothing else.

Two consequences, both live:

- **The four accessors hand back bare `Vec3`s.** `origin()`, `u()`,
  `v()` and `normal()` read columns off the stored map, so a caller
  that wants a frame again — the viewer's sketch road, the Python
  value road, any consumer that re-places geometry — either re-mints
  one from two bare vectors (the constructor the ruling refused) or
  carries the pair loose. The decision the mint made does not survive
  the read.
- **`SketchPlane::new` is `pub` and `placement` is a `pub` field**,
  with production and test callers building a plane straight from an
  `Affine3`. So the type holds whatever it is handed, and its rigidity
  is a property of the MINTING road rather than of the type. The docs
  now say this plainly (that was the fix-pass correction); making it
  true of the type is this row.

## The shape a fix would take

Store the `OrthoFrame<T>` and derive the placement, or store both with
the frame authoritative; accessors return the witnesses. `new` then
either goes away, becomes a fallible door that decides what it was
handed, or stays as a clearly-named read-back door for a placement
that came from somewhere else.

The cost is real and is why this is a row rather than a line in that
PR: `placement` is a public field read across `profile`, `sweep`,
`geom-brep`, `editor-core`, `viewer` and `pncad-py`, and
`SketchPlane::map` lifts the stored twelve components between scalars
without arithmetic — a property a stored frame would have to keep.

## Where it came from

`crates/profile/src/lib.rs` (`SketchPlane`, `new`, `from_frame`, the
four accessors); `crates/geom-core/src/linalg/ortho_frame.rs` for the
witness. The ruling is
`work/scalar/unit-vector-invariants-carried-as-prose.md` §RATIFIED.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to PATHS (opened at this exit as S-BOOL's successor for the profile lattice) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

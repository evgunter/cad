---
id: frame-witness-and-the-tube-door
kind: unit
title: A frame witness (origin plus a right-handed orthonormal pair) minted by the Gram-Schmidt ladders; from_frame and the tube door take it
status: open
opened: 2026-09-15
branch: scalar/frame-witness
---


## What

The ruling on `unit-vector-invariants-carried-as-prose` (PR 2457),
second unit, after `unit-vector-witness-in-geom-core`. A frame witness
in `geom-core` — origin plus a right-handed orthonormal triple — minted
only by the decided Gram–Schmidt ladders (`eval/wire.rs` `frame_axes`,
`geom-core` `path_start_frame` and `frame_from_unit_aim`), converting
into an `Affine3` (which stays the general affine map).
`Affine3::from_frame` takes it, and so does `sweep::tube_along_arc` /
`tube_along_arc_hollow` for `center, axis, u_ref` — retiring
`TubeError::NonUnitAxis`, `NonUnitURef` and `FrameNotOrthogonal` (two
Python refusal tags go with them; PORT announced). The wire's private
`AxisFrame` becomes the type; `SketchPlane::from_frame` and the Python
plane door route through it. Ground: PROPS, WIRE, BLEND (`sweep`),
PORT; announce each. Full v6 dual.

## Digest receipt at the merge base

Recipe: the zero-parameter one in `rate-pair-in-geom-core.md`
§"Digest receipt" — `cargo build --release` in `demos/tour`, the binary
run directly into the literal relative outdir `tour-out`, both streams
digested whole.

At the merge base `d71bb6a78` (`origin/main` at dispatch), taken before
the first code change of this unit:

- 1766 emitted files; digest of the sorted per-file digest listing:
  `87be4dd9df4cc3af9bd44593a6b981608c8e73721c746413a00322ff61e4a892`
- narration, 729 lines:
  `e930abf542c371677b2c0d87b02c2bf84eb15fbaf62ded6389f48c899ef14d49`

Both match the values PR 2657 recorded at `origin/main` (`4f71edaea`).

## Digest receipt at the head

Same recipe, same tree shape: **1766 files**, the same paths, and the
tour's **narration is IDENTICAL** —
`e930abf542c371677b2c0d87b02c2bf84eb15fbaf62ded6389f48c899ef14d49`, all
729 lines, so every census, genus, validation tier and exact-vs-meshed
mass property the tour prints is what it printed at the base.

The listing digest MOVED, to
`1389dbb20bb6c0a9efca999a587a7b5db2930b82d0ac29ed32c3643b0310d50a`, in
**20 of the 1766 files, every one of them `lily_*`**
(`lily_{bud_c,lantern,leaf_a,pedicel,sepal_a,sepal_b,sepal_c}.step`,
`lily_{lantern,pedicel}.stl` and eleven `uv/lily_*.svg`). The cause is
the unit's own subject: the lily authors its frames from directions
that are UNIT BY INTENT but not bit-exactly unit — the turtle's radial
is `(p − centre) / ring` and the blade axes come out of rotations — and
the frame mint normalizes what it is given. The moves are last-bit:
`DIRECTION('', (0.17364817766693041, ...))` becomes
`(0.17364817766693047, ...)`, ~1e-16 relative, 15 changed lines in the
largest of the twenty diffs.

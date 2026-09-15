---
id: frame-witness-and-the-tube-door
kind: unit
title: A frame witness (origin plus a right-handed orthonormal pair) minted by the Gram-Schmidt ladders; from_frame and the tube door take it
status: open
opened: 2026-09-15
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

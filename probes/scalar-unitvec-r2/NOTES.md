# SCALAR / UNITVEC — R2 review probes

Frozen head reviewed: `52424f453e56d9a477101041ee130e96f0d6346b` (PR 2646).
Merge base: `27e0132df17fbdfa590ee492974da97a44ccc832`.
Sweep base cited by the PR body: `385c01b33`.

Nothing here is proposed for the tree; it is the record of what R2 ran.

## Mutants (all applied to the frozen head, reverted after each run)

| # | mutation | result |
|---|---|---|
| M1 | drop the `is_underflowed_length` gate from `decide_unit_direction` | RED — `the_direction_door_tells_an_underflowed_length_from_a_zero_one`, `the_underflow_refusal_names_the_length_and_the_recourse` (303 passed / 2 failed) |
| M2 | `pub struct UnitVec3<T: Real>(pub Vec3<T>)` | RED — the private-field `compile_fail` doctest ("Test compiled successfully, but it's marked `compile_fail`"); the E0308 doctest correctly survives |
| M3 | re-decide in the witness door (`self.0.normalize().orthonormal_basis()`) | RED — `the_witness_orthonormal_basis_is_the_bare_construction` |
| M4 | `after_decided_length` skips the divide (`Self(v)`) | RED — 4 `linalg::frame` rows (`mirror_fixes_its_plane_and_flips_handedness`, `path_start_frame_pole_fallback_pin`, `point_at_orientation_pin`, `point_at_roll_convention_and_rigidity`) |
| M5 | `frame_from_unit_aim` reverts to `aim: Vec3<T>`, callers pass `unit.get()` | **SURVIVES** — geom-core green (305 lib + 173 integration + 11 + 13 doctests, 0 failed) |

## Mint-reach probes (`after_decided_length`, `pub(super)`)

- From `crates/geom-core/src/tolerance.rs` (geom-core, outside `linalg`):
  `error[E0624]: associated function \`after_decided_length\` is private`.
- From `crates/geom-core/src/linalg/mat.rs` (a `linalg` sibling of `frame.rs`):
  compiles and runs. `UnitVec3::after_decided_length(Vec3::new(0.0, 0.0, 0.0))`
  yields `Vec3 { x: NaN, y: NaN, z: NaN }` held in the witness — no decision
  anywhere. The reach of the ladder mint is all of `linalg`, not `frame.rs`.

## Sweeps

- `their_sweep.py` reproduces the PR body's declared pattern (a `///` line
  containing `unit` within 8 lines above a `fn` whose signature LINE takes a
  `Vec3<`, non-test sources under `crates/`, `demos/`, `tools/`). At the frozen
  head it returns 26 hits, including two the PR's 25-row table does not carry:
  `crates/geom-brep/src/ssi/jet.rs:318 axial_radial` and
  `crates/editor-core/src/eval/measure.rs:596 axis_offset`. Both files are
  unchanged between `385c01b33` and the merge base, so neither arrived with a
  merge.
- `sweep.py` is R2's own, differently shaped: any doc word in
  {unit, normalized/normalised, direction, normal, axis} above a fn whose
  MULTI-LINE signature mentions `Vec3<`/`Vec2<`/`SpanBox<`/`[f64; 3]`. 210 hits,
  used as a candidate list rather than a finding.

## End-to-end

A `crates/pncad/examples/r2_unitvec_e2e.rs` (removed before committing) authored
a `Datum::Plane` with normal `(0, 3, 4)` and a `Datum::Axis` with direction
`(1, 1, 0)` through `pncad::document::{apply, evaluate}`, read the two
`UnitVec3<f64>` witnesses off `DatumValue`, and fed them to
`UnitVec3::orthonormal_basis`, `point_at` and `path_start_frame`. Output:

```
plane normal witness: Vec3 { x: 0.0, y: 0.6, z: 0.8 }
axis dir witness:     Vec3 { x: 0.7071067811865475, y: 0.7071067811865475, z: 0.0 }
  |normal| - 1 = 0e0
witness orthonormal_basis: b1=Vec3 { x: 1.0, y: -0.0, z: -0.0 } b2=Vec3 { x: -0.0, y: 0.8, z: -0.6 }
  b1.n=0e0 b2.n=0e0 b1.b2=0e0
point_at frame +Z col: Vec3 { x: 0.7071067811865475, y: 0.7071067811865475, z: 0.0 }
path_start_frame +Z col: Vec3 { x: 0.0, y: 0.6, z: 0.8 }
double negation equal bits: true
from_angle_xy(pi/3) = Vec3 { x: 0.4999999999999999, y: 0.8660254037844387, z: 0.0 }
zero normal: error = Some(... DegenerateDirection { role: "datum plane normal" } ...)
underflowed normal: error = Some(... UnderflowedDirection { role: "datum plane normal" } ...)
```

`path_start_frame(origin, normal.get(), tol)` is the ergonomics point: the two
public frame doors still take a bare `Vec3`, so a caller holding the witness
unwraps it at the boundary and the ladder re-decides and re-divides.

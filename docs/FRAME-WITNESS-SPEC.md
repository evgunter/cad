# FRAME-WITNESS — a frame witness in geom-core, minted by the decided ladders; `from_frame` and the tube door take it

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit `frame-witness-and-the-tube-door`; deleted at
merge per `docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md`
in full first. The item is `work/scalar/frame-witness-and-the-tube-door.md`;
the ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
§RATIFIED and its refinement (PR 2457: "the tube door takes a frame
witness"); the first unit landed as PR 2646 (`geom_core::UnitVec3`,
`crates/geom-core/src/linalg/unit_vec.rs`).

## 0. The ruling this executes

A unit vector is a witness minted by a decided normalize; a FRAME is
the same idea one level up — an origin and a right-handed orthonormal
triple whose orthonormality was DECIDED where it was built, not asserted
by a caller's prose. Today `Affine3::from_frame(origin, u, v)`
(`affine.rs:56-77`) says "`u ⊥ v` unit is the caller's conventional
obligation, unchecked"; `SketchPlane::from_frame` restates it
(`profile/src/lib.rs:571-574`); the tube door re-decides unit-ness and
orthogonality under its own two funnel names with a LEVERED band
(`sweep/src/revolve/tube.rs:400-423`) and refuses `NonUnitAxis`,
`NonUnitURef`, `FrameNotOrthogonal`; WIRE's Gram–Schmidt ladder
`frame_axes` (`eval/wire.rs:1365-1373`) returns a bare pair and a
private `AxisFrame` (`:1401-1405`) names it; geom-core's own
`frame_from_unit_aim` (`frame.rs:469-479`) is already "witness in,
`Affine3` out" but private and un-named. One type, minted in one crate.

## 1. What this unit delivers

**The type.** `crates/geom-core/src/linalg/ortho_frame.rs` (a module
of its own beside `unit_vec.rs`, re-exported at `geom_core::OrthoFrame`
and `geom_core::linalg::*`):

```rust
pub struct OrthoFrame<T: Real> { origin: Point3<T>, u: UnitVec3<T>, v: UnitVec3<T>, w: UnitVec3<T> }
```

private fields, `Copy`, `Debug`, no `PartialEq`. What it means at every
scalar: `u`, `v` are witnesses (a decided normalize each, or a decided
normalize and its exact negation), `v` was produced ORTHOGONAL to `u`
by construction (a Gram–Schmidt residual or a cross product with `u`),
and `w = u × v` — unit and orthogonal to both by construction, at `f64`
the rounded cross product, at `Interval` an enclosure of it, at `Dual`
the product rule's tangent. Accessors `origin()`, `u()`, `v()`, `w()`
(the witnesses), and `to_affine(self) -> Affine3<T>` =
`Affine3::from_parts(Mat3::from_cols(u, v, w), origin − O)` — the same
columns, the same order and the same cross product `from_frame` builds
today, so it is bit-identical to `from_frame(origin, u, v)` on the same
inputs.

**The mints, and only these.**
- `OrthoFrame::gram_schmidt(origin, u_raw, v_raw, site_u, site_v, band)`
  — WIRE's ladder moved down verbatim (`u = UnitVec3::new(u_raw, site_u)`,
  `v_perp = v_raw − u·(v_raw·u)`, `v = UnitVec3::new(v_perp, site_v)`;
  `u` kept, `v` yields; the same two decisions under the CALLER's funnel
  names — `frame_axes` becomes a call to it with `DATUM_UNIT_NORM` and
  its role words, so the K predicate counts do not move). Refusal:
  `UnitVec3Error` with which axis, as `frame_axes`'s `DirectionRefusal`
  reports today.
- `OrthoFrame::from_aim(origin, aim: UnitVec3<T>, perp_raw, site, band)`
  — the `point_at`/`path_start_frame` recipe: `x = UnitVec3::new(perp_raw,
  site)` (replacing `definitely_positive` + divide under the SAME name,
  bit-identical as PR 2646 showed), `y = aim × x` by the cross mint,
  columns `(x, y, aim)`. `frame_from_unit_aim` becomes this door plus
  `.to_affine()`; `point_at` and `path_start_frame` keep their
  signatures and return the same `Affine3` bit for bit.
- The cross mint `UnitVec3::cross_of_orthonormal(a, b)` (name it in the
  crate's vocabulary): unit by construction when `a ⊥ b` are witnesses
  — `pub(crate)` to `geom-core`'s linalg, used only by `OrthoFrame`;
  its doc says why it is not a "check it is unit" constructor.
- Exact hand frames: `OrthoFrame::axes_xy(origin)`, `axes_yz`, `axes_zx`
  (the world frames `profile`'s `xy/yz/zx` build — exact unit axes,
  exact cross products, no decision; say in the doc that these are the
  only frames the type admits without a decision, because their axes
  are the exact basis vectors). If you find `SketchPlane::from_frame`'s
  callers need a general "trust me" mint, STOP: that is the
  check-it-is-already-unit constructor the ruling refused; route them
  through `gram_schmidt` with the band they hold, and where a caller
  holds no band, say so and file.

**The consumers.**
- `Affine3::from_frame(origin, u, v)` RETIRES in favour of
  `OrthoFrame::to_affine` (its "unchecked" paragraph goes with it) —
  or, if the name is load-bearing for readers, `Affine3::from_frame(frame:
  OrthoFrame<T>)`; either way no bare-`Vec3` frame constructor remains
  on `Affine3`.
- `SketchPlane::from_frame` (`profile/src/lib.rs:617-619`) takes
  `OrthoFrame<T>`; its `xy/yz/zx` sugar uses the exact mints; its
  callers in `editor-core` (`eval/wire.rs:1214-1218`, `:1325-1329` —
  already decided through `frame_axes`) hand the frame through; the
  `#[cfg(test)]` and `crates/*/tests` fixtures use the exact mints or
  `gram_schmidt` with the band they hold.
- WIRE's private `AxisFrame` (`eval/wire.rs:1401-1405`) becomes
  `OrthoFrame` (or holds one); `AuthoredFrame` (`:1114-1118`) likewise
  if it is the same thing — read `frame_from_slots` and say.
- **The tube door.** `sweep::tube_along_arc(center, axis, u_ref, …)` and
  `tube_along_arc_hollow` (`revolve/tube.rs:265-326`) take
  `frame: OrthoFrame<T>` in place of `center, axis, u_ref` — the axis is
  one witness, `u_ref` the other (say which of `u`/`v`/`w` is which in
  the doc, and keep the stored-not-normalized posture: the door stores
  the witnesses it was given). `TubeError::NonUnitAxis`, `NonUnitURef`,
  `FrameNotOrthogonal` retire with the two decides `tube_frame_unit` and
  `tube_frame_orthogonal` (`:400-423`) — the decision moved to the
  frame's mint, so the k-lint predicate counts MOVE (two names retire)
  and the PR body says so with the before/after figures; the frame
  assembly at `:450-461` (the rotation of `u_ref` by `t0`) stays. The
  Python tags `non_unit_axis`, `non_unit_u_ref`, `frame_not_orthogonal`
  (`pncad-py/src/tags.rs:1073-1086`) retire with the census row
  (`pncad-py/src/tests.rs:4732-4748`); `Node.tube`/`Node.hollow_tube`
  (`py/doc.rs:1727-1793`) mint the frame from the datum axis witness and
  `u_ref` through `from_aim` under a named site, so a degenerate `u_ref`
  refuses through the frame mint's vocabulary (a NEW tag, named; LIB
  announced). Callers: `eval/wire.rs:1995-2036` `tube_args` and
  `wire_tube`/`wire_hollow_tube`; `demos/tour` (`tube.rs`, `tubewall.rs`,
  `lily.rs`) mint with the exact or `from_aim` mints.
- `Node.datum_frame`'s evaluate path and `datum_face_frame` (`wire.rs:1483-1533`)
  already go through `frame_axes`; they hand the `OrthoFrame` on.
- `SketchPlane.from_frame(origin, u, v)` in Python (`py/doc.rs:1421-1439`)
  goes through `gram_schmidt` under `Tol::witness()`'s band and a named
  site (it was unchecked; a degenerate pair now refuses — a new Python
  refusal, named in the census; LIB announced).

**Not this unit (list with pointers):** `step-import`'s two hand-rolled
frame dodges (`adopt.rs:1031-1058`, `entities.rs:2387-2401`: a raw
`> 0` with no band — EXCH's, file); `newell.rs:156` and
`recognize.rs:228` `orthonormal_basis` on an undecided normal (the rows
PR 2646 filed); `topo/src/boolean/boxes.rs`'s six `orthonormal_basis`
calls on bare axes (file on BOOL with the class pointer);
`sweep/src/revolve/axis.rs` `AxisFrame` (the 2-D revolve frame — a
different thing sharing the name; note the clash, do not rename here);
`Mat3::rotation_about` re-dividing a witness (PROPS' row from PR 2646).

**What must not change:** every `Affine3` every ladder returns, bit for
bit (`to_affine` is `from_frame`'s arithmetic; `from_aim` is
`frame_from_unit_aim`'s); every tube body (the frame assembly is
untouched — the differential over `sweep`'s suites and the tour digest
is the proof); no golden moves except the tube's retired refusals; the
predicate counts move only by the two tube names, said.

## 2. Docs

The type's doc states what the witness means at every scalar (§1),
that `Line.dir`/`Plane.normal`/the carrier frames are NOT this type
(the at-rest rule, as `unit_vec.rs:41-48` says), and that
`editor-core`'s placement `Frame` and `sweep`'s revolve `AxisFrame` are
different things. `frame.rs`'s roll convention (`:26-34`) stays the one
home of the axis order. No history.

## 3. The pin

- D9 differential: `geom-core`, `editor-core`, `profile`, `sweep`,
  `pncad`, `pncad-py` suites green unchanged (except the retired tube
  refusal rows, rewritten to the frame mint's refusal); the tour
  digests identical at merge base and head (the zero-parameter recipe
  in `work/scalar/rate-pair-in-geom-core.md`).
- `to_affine` bit-for-bit against `Affine3::from_frame`'s columns on a
  decided frame (before the old door retires, in the same row).
- Compile-fail doctests: no `OrthoFrame` from three bare `Vec3`s; the
  tube door does not take a bare axis.
- The tube's two retired refusals: a row that a degenerate `u_ref` now
  refuses at the frame mint with the named site, and that a parallel
  `u_ref` (the old `FrameNotOrthogonal`) refuses as a degenerate
  Gram–Schmidt residual.
- `frame_axes` bit-identical: the `docm1_face_frame*` rows and the
  `pncad` façade rows PR 2646 adopted stay green unchanged.

## 4. Sweep

The class: a function that takes or builds an orthonormal pair/triple
by convention (doc words `orthonormal|right-handed|unit … perpendicular`,
`Mat3::from_cols` of three vectors, `u.cross(v)` as a third axis,
`from_frame`). Over `crates/*/src`, `demos/`, `tools/`, `benches/`;
disposition every hit; state the blind spot.

## 5. Fence

This program claims no paths. This unit reaches PROPS'
`crates/geom-core/src/linalg/*`, WIRE's `crates/editor-core/src/eval/{wire.rs,mod.rs}`
and `placement.rs`, BOOL's `crates/profile/src/lib.rs`, BLEND's
`crates/sweep/src/revolve/tube.rs` and `crates/sweep/src/lib.rs`, LIB's
`crates/pncad/*` and `crates/pncad-py/*` (tags, census, `py/doc.rs`,
`py/place.rs`), the unowned `demos/tour/src/*` and
`crates/geom-brep/src/newell.rs`, TCOST/TINT's tests. Announced by the
orchestrator; merge `origin/main` before opening the PR; territory
output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-core -p editor-core -p profile -p sweep -p pncad`
at default features and `-p geom-core --features interval`; the
`pncad-py` suite (maturin develop + pytest, as the python CI job runs
it — read `.github/workflows/ci.yml` for the exact commands); `cargo
clippy --workspace --all-targets -- -D warnings` at default AND
`--all-features`; `demos/tour` and `demos/wild` clippy; `scripts/doc-gate.sh`
and `scripts/doc-gate.sh --skip-viewer-toolkit`. Disk is shared: build
`pncad-py` last and delete the target as soon as its suite is green.
Hosted CI is the verification of record; poll to conclusion in the
foreground. Report ≤130 lines: the type and its mints, the consumers
taken (tube door signature, retired refusals and tags, the k-lint
figures), the differential's and digest's receipts, the sweep table,
deviations, rows filed and where, PR number, head SHA, CI run id and
conclusion.

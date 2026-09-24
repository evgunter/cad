---
id: frame-witness-and-the-tube-door
kind: unit
title: A frame witness (origin plus a right-handed orthonormal pair) minted by the Gram-Schmidt ladders; from_frame and the tube door take it
status: closed
opened: 2026-09-15
closed: 2026-09-15
branch: scalar/frame-witness
pr: 2675
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

Same recipe, same tree shape: **1766 files**, the same paths.

Taken at this branch's merge base with `origin/main` (`c645194c8`) and
at the branch head. The base reading reproduces the value the receipt
above records, so nothing main landed in between moved the tour:

| | files | listing digest | narration |
|---|---|---|---|
| base `c645194c8` | 1766 | `87be4dd9…` | `e930abf5…`, 729 lines |
| head | 1766 | `c678b143…` | `65da9dd7…`, 729 lines |

The listing moved in **25 of the 1766 files, every one of them
`lily_*`** — ten `.step` (`bud_a`, `bud_b`, `bud_c`, `lantern`,
`leaf_a`, `leaf_c`, `pedicel`, `sepal_a`, `sepal_b`, `sepal_c`), five
`.stl` (`lantern`, `leaf_a`, `leaf_c`, `pedicel`, `sepal_c`) and ten
`uv/lily_*.svg`.

The cause is the unit's own subject, in two layers. The lily authors
its frames from directions that are UNIT BY INTENT but not bit-exactly
unit — the turtle's radial is `(p − centre) / ring` and the blade axes
come out of rotations — and the frame mint normalizes what it is given:
that is the first twenty files. The fix pass then retired the scene's
OWN Gram–Schmidt ladders (`blade_frame`'s `dir.normalize()` +
`up.reject_from(d).normalize()`, and the bud segment's
`(dir·ct + l·st).normalize()` + `start.reject_from(a).normalize()`)
onto the mint, which spells the residual as `r − a(r·a)` where
`reject_from` spells it as `(a × r) × a / |a|²` — a different
arithmetic route to the same direction. That is the remaining five
files. The moves are last-bit:
`DIRECTION('', (…, -0.20787730316411107, …))` becomes
`(…, -0.2078773031641111, …)`, ~1e-16 relative, 137 changed lines in
the largest of the twenty-five diffs.

**The narration moved on ONE line of 729** — `lily_leaf_c`'s certified
enclosure WIDTH, `± 9.8e-16` → `± 1.0e-15`. The volume itself
(0.001265 m³), the area, the triangle count and the mesh-vs-exact
percentage are unchanged, as is every other line of the narration:
every census, genus, validation tier and mass property the tour prints
is what it printed at the base.

## Closed (2026-09-15) — PR 2675

`OrthoFrame<T> { origin, u, v, w }` in `geom-core`'s linalg, private
fields, minted only by `gram_schmidt` (one `site`), the exact
`axes_xy/yz/zx`, `from_aim_and_reference` and the one-home
`from_axis_and_reference(origin, axis_raw, reference, site, band)`;
`from_aim` is `pub(in crate::linalg)` (the dual's bilateral MAJOR: it
took perpendicularity on trust). Every mint refuses with one
`OrthoFrameError { axis, error }` naming the role (`U` kept, `V`
yields). `to_affine` is `from_frame`'s arithmetic bit for bit, pinned
on decided and aim-minted corpora; `Affine3::from_frame` retired.
`SketchPlane::from_frame` takes the frame (Python's door decides under
`Tol::witness()`; a degenerate pair now refuses — new tags
`degenerate_u_axis`/`degenerate_v_axis` and siblings); the tube doors
`tube_along_arc`/`_hollow` take `frame` (the door reads `frame.v()`),
`TubeError::NonUnitAxis`/`NonUnitURef`/`FrameNotOrthogonal` and their
three Python tags retired, the two `tube_frame_*` decides gone from
the K roster and said. Five copies of the axis-and-reference ladder
became one call each (`tube_args`, the two Python tube doors, the
tour's `blade_frame` and bud segment). Bit identity: every non-lily
scene byte-identical and every `Affine3` the ladders return; 25
`lily_*` files moved last-bit (the mint normalizes what the lily hands
it as unit-by-intent; the residual spelled `r − a(r·a)` where
`reject_from` spelled `(a×r)×a/|a|²`) and one narration line
(`lily_leaf_c`'s enclosure width `±9.8e-16 → ±1.0e-15`), re-baselined
and named. Two `compile_fail` doctests verified to fail for one reason
each (`E0451`, `E0624`). Reviews: dual, both APPROVE WITH FIXES, one
MAJOR bilateral; thirteen fix-pass items taken (mints taking `Tol`
declined — `Band::linear` is fallible and its `expect` shape is a
repo-wide convention question). Rows: INSTR
`frame-mint-funnel-names-outside-every-sweep-corpus`, BOOL
`sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary`,
BLEND `perp-unit-takes-an-unchecked-unit-axis` (widened). Routed:
PROPS' const-doors row, FIX's tour-lift row, CIW's private-link row.

---
id: recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness
kind: issue
title: step-import recognize.rs normalizes a plane normal with no length decision and plus_zero's it, so it cannot mint the unit witness — the bare Vec3::orthonormal_basis stays for it
status: open
opened: 2026-09-15
priority: P1
cost: D
---

## Where this came from

The class sweep of `unit-vector-witness-in-geom-core` (SCALAR; the
ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
§RATIFIED). The class: a function whose doc or parameter name asserts
a unit-vector precondition it does not check. `geom_core::UnitVec3<T>`
now exists to carry that fact across a function boundary — minted by
the normalizing constructor (`UnitVec3::new(v, site, band)`: decide the
length under the band, divide), by exact negation, by `sin_cos`, and by
`geom-core`'s `frame.rs` ladders. A function in the class takes the
witness the day its caller holds one; until then the precondition
stays prose, and this row is where that is recorded rather than in a
merged PR body.

The geometry CARRIER fields (`Line.dir`, `Plane.normal`, the conic
axes) stay bare under `geom/src/lib.rs`'s at-rest rule by the ruling;
a parameter that is read straight out of such a field is in the class
but its caller holds no witness, so the take waits on either a
decision at the read (a `UnitVec3::new` under a name the reader owns)
or the carrier rule changing, which is not this row's call.

## The site, and why the bare `orthonormal_basis` stays

`crates/step-import/src/recognize.rs`, the plane arm of `recognize`
(`:213`–`:227`): `normal_sum.normalize()` with no length decision —
the refutation that follows (`align.is_finite() && align != 0.0`) is
a hand-rolled check on a DIFFERENT quantity — and then `plus_zero`
on the normalized vector before `normal.orthonormal_basis()`. Two
things keep it from minting the witness: it holds no `Band` (the
function takes `eps_in: f64` and decides nothing through the funnel),
and `plus_zero` is not one of the mints (negation is exact and
minted; a signed-zero rewrite is exact too, but adding it as a mint is
a design choice this unit did not make).

So `Vec3::orthonormal_basis` keeps its bare door for exactly this
caller and `geom_brep::newell` (the props row
`a-widened-derived-placement-normalises-a-straddling-newell-sum`
carries that one). The witness door is
`geom_core::UnitVec3::orthonormal_basis`; the bare one retires when
both callers decide their length — a decision this file's owner makes,
not the sweep.

## Siblings in this crate pair, from the same sweep

- `crates/step-import/src/normalize.rs`, `half_turn` (`:111`): "rotates
  `w` a half turn about the unit direction `axis`" — exact-identity
  arithmetic that is only an involution for a unit `axis`.
- `crates/step-import/src/adopt.rs`, `line_frame` (`:1031`): takes
  `dir` from the ε_in direction reader, which validates it by hand;
  a witness minted at that reader would let this and `half_turn` take
  the type.
- `crates/step-export/src/writer.rs`, `direction` (`:166`): "unit by
  the conventions of every stored normal/axis/dir — emitted as stored,
  never renormalized". The carrier case, on the way out.


## Added at the fix pass (re-sweep at the merged base)

- `crates/step-export/src/writer.rs`, `axis2_placement` (`:189`):
  "every kernel frame stores `axis` unit, `ref_dir` unit and ⊥ `axis`
  … no renormalization" — the carrier case on the way out, beside
  `direction`; a frame, so the second unit's shape.
- `crates/step-import/src/normalize.rs`, `half_turn_curve` (`:121`):
  every point goes through `half_turn(p − origin, axis)`, so it
  carries `half_turn`'s premise on the same `axis`, one call up.

## Added by FRAME-WITNESS's §4 sweep (2026-09-15)

The two hand-rolled FRAME dodges this crate pair carries — the ladder
the unit's own `OrthoFrame::gram_schmidt` now writes once, with both
lengths decided and a typed refusal naming which axis:

- `crates/step-import/src/adopt.rs`, `line_frame` (`:1031-1058`): a
  magic-constant least-axis pick, then `perpendicular = candidate −
  dir*along`, then a raw `norm.is_finite() && norm > 0.0` — a bare
  comparison with no band — then `y = perp/norm` and
  `z = dir.cross(y)`. The residual and the cross are `gram_schmidt`'s
  two steps; the `> 0.0` is what the kernel's `decide` exists to
  replace, and this file is the last place in the tree spelling it by
  hand for a direction length.
- `crates/step-import/src/entities.rs` (`:2387-2401`): a second copy of
  the same shape — `along = z.dot(f.2)`, `x = f.2 − z*along`, the same
  `n.is_finite() && n > 0.0`, then `Mat3::from_cols(x, z.cross(x), z)`.

Both are named as "not this unit" in `docs/FRAME-WITNESS-SPEC.md` §1,
and both are one call to the mint once EXCH decides who owns the band
at the ε_in direction reader — the same unlock this row already names.

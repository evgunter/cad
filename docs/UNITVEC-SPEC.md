# UNITVEC — the unit-vector witness moves to geom-core, minted by the decided ladder

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit `unit-vector-witness-in-geom-core`; deleted at
merge per `docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md`
in full first. The item is `work/scalar/unit-vector-witness-in-geom-core.md`;
the ruling is `work/scalar/unit-vector-invariants-carried-as-prose.md`
§RATIFIED and its refinement (PR 2457).

## 0. The ruling this executes

A validating unit-vector type exists, `topo::query::UnitVec3<T>`
(`new(v, band)` decides the length under the band — finite, not
underflowed, definitely positive under funnel `DATUM_UNIT_NORM` — then
divides; `get()` unwraps; typed `UnitVec3Error`). It lives two crates
above the doors that consume a direction, so every consumer calls
`.get()` at the boundary and the fact is dropped; `geom-core`'s own
decided-normalize ladders (`frame.rs`: `definitely_positive` then
`normalize`) cannot mint it. The ruling: **the witness moves to
`geom-core`; the ladder's decided normalize is its constructor; exact
negation and `sin_cos` mint it too; `topo` does NOT re-export it —
imports are repointed; no "check it is already unit" constructor.** What
it means at every scalar: produced by a normalize whose length decided
positive under the band — an enclosure of a unit vector at `Interval`, a
unit value channel at `Dual`. No new policy.

## 1. What this unit delivers

**The move.** `UnitVec3<T>`, `UnitVec3Error` and `decide_unit_direction`
move from `crates/topo/src/query.rs` to `crates/geom-core/src/linalg/`
(a module of its own beside `vec.rs`), private field, `Copy`, `Debug`,
no `PartialEq` (the existing row about hashing a `PartialEq`-only
newtype stays as it is). The funnel-site NAME stays a caller's choice
(the `site` parameter): `DATUM_UNIT_NORM` stays in `topo` as the datum
boundary's name. Every importer (`topo`, `editor-core`, `viewer`,
`pncad`'s `document.rs`, `pncad-py`) repoints to `geom_core`; `topo`
does not re-export.

**The mints.**
- `UnitVec3::new(v, site, band)` — the normalizing constructor, exactly
  today's decision then divide (bits unchanged).
- `-u` (`Neg`) — exact.
- `UnitVec3::from_angle`-style mints where a `sin_cos` pair is already a
  unit vector by construction (2-D and 3-D as the crate needs; the
  in-plane axes of `path_start_frame`'s ladder are the first customer if
  they are built that way — check).
- `frame.rs`'s ladders (`point_at`, `path_start_frame`,
  `mirror_across_plane`) mint the witness where they decide a length and
  normalize, and hand it on.

**The consumers.** `Vec3::orthonormal_basis(self)` takes the witness
(its "Precondition (conventional, unchecked): `self` is unit" goes
away); `frame_from_unit_aim(origin, aim, perp, cross_len)` takes the
witness for `aim`. `Affine3::from_frame`, the tube door and the wire's
`AxisFrame` are the SECOND unit's (the frame witness) — leave their
signatures; where `orthonormal_basis`'s callers (`geom-brep/src/newell.rs`,
`step-import/src/recognize.rs`) already normalize just before, they mint
the witness there with the decision they already make (or, where they
just `.normalize()` with no decision, say so and file — do not add a
decision the site did not make; the rows `work/props/normalize-overflow-yields-zero-axis.md`
and `work/fix/normalize-without-the-length-question-two-more-sites.md`
already cover that class).

**What must not change:** every value every consumer computes, bit for
bit — the witness records a decision the ladder already makes and the
divide is the same divide. `topo`'s existing `UnitVec3` rows (including
the `Interval` ones: `‖u‖ − 1` decides `Zero` for tight inputs; an
overflowed enclosure stays sound) move with the type and stay green.

## 2. Docs

The type's doc states what the witness means at every scalar (§0's
sentence), and that the geom carrier fields (`Line.dir`, `Plane.normal`,
the axes) are NOT this type — they stay under `geom/src/lib.rs`'s
at-rest rule, by the ruling. `profile::path::Dir<T>` (the 2-D witness)
is named as the 2-D analogue and left alone. No history in comments.

## 3. The pin

- D9 differential: `topo`, `editor-core`, `geom-core`, `geom-brep`
  suites green unchanged.
- The moved `Interval` rows, plus one row at `Dual`: the value channel
  is the `f64` witness bit for bit and the tangent is the quotient
  rule's.
- A compile-fail doctest that a bare `Vec3` cannot be passed where the
  witness is required (`orthonormal_basis`), and that the field cannot
  be constructed outside the module.
- Negation and the `sin_cos` mint: a row each that the minted vector is
  the one the bare arithmetic produces, bit for bit.

## 4. Sweep

The class: a function whose doc or parameter name asserts a unit-vector
precondition it does not check. The PR 2457 survey found about twenty;
this unit takes the `geom-core` function parameters named in §1 and
LISTS the rest with dispositions (second unit: `from_frame`, tube;
carrier fields: out by the ruling; `splitting/mod.rs` `SplitPlane.normal`,
`boolean/boxes.rs` `slab_extent`, `implicit.rs` `axial_radial`,
`tangent.rs` `perp`, `blend/arms.rs` `perp_unit`, `mesh/cert.rs`
`dist_line_triangle`, `mate/coset.rs` `Subgroup`, `measure.rs`
`Carrier`: each either takes the witness now if it is a one-line change
whose caller already holds one, or is filed on its owner's slate as the
class's remaining members). Blind spot stated.

## 5. Fence

This program claims no paths. This unit reaches PROPS'
`crates/geom-core/src/linalg/*`, TOPO's `crates/topo/src/query.rs` and
`lib.rs`, WIRE's `crates/editor-core/src/eval/wire.rs` (imports), VIEW's
`crates/viewer/src/sketch.rs` (import), PORT's `pncad`/`pncad-py`
(imports), and wherever §4's one-line takes land. Announced by the
orchestrator; merge `origin/main` before opening the PR; territory
output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-core -p topo -p editor-core` at
default features and `-p geom-core --features interval` for the moved
rows; `cargo clippy --workspace --all-targets -- -D warnings` (a type
moved crates — every importer must compile) plus the excluded roots'
clippy (`demos/tour`, `demos/wild`) since a public type moved. Hosted
CI is the verification of record; poll to conclusion in the foreground.
Report ≤120 lines: the module and its mints, the consumers taken, the
sweep's dispositions, the differential's receipt, deviations, rows filed
and where, PR number, head SHA, CI run id and conclusion.

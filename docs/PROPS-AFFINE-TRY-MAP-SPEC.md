# PROPS affine-try-map — the kernel owns the fallible per-coordinate walk too

**Binding at dispatch** (PROPS program; items
`work/props/affine3-try-map-the-fallible-walk-has-no-kernel-door.md`
and `work/props/map-affine-retires-into-affine3-try-map.md` — read both
in full; difficulty logged at spec: **E**, an E rider — single style
review, outside the A/B experiment). Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/affine-try-map`, cut from `main`.

## What this is

The per-coordinate walk over an `Affine3` — twelve components through
one function, the three columns and the translation kept in their
places — has an infallible kernel door, `Affine3::map<U>`
(`crates/geom-core/src/linalg/affine.rs`, descending through
`Mat3::map` and `Vec3::map`), and `SketchPlane::map` over it
(`crates/profile/src/lib.rs`). It has no fallible one, so
`crates/editor-core/src/eval/anchor.rs` writes the same walk again
privately as `map_affine`, whose one caller is `eval/wire.rs`'s
`pinned_plane` — the lane-to-`f64` crossing where every component goes
through `SectionScalar::pinned_f64`, which answers `None` on an
analysis scalar: a refusal `map`'s infallible `f` cannot spell.

The argument for the kernel owning the fallible direction is the one
`editor-core` makes for writing the walk once at all: **a transposed
`c1`/`c2` is invisible in review and identical in every copy but one**,
and today the walk has two copies with two owners.

## Deliverables

- `Affine3::try_map<U: Real, E>(self, f: impl Fn(T) -> Result<U, E>) -> Result<Affine3<U>, E>`,
  with `Mat3::try_map` and `Vec3::try_map` beneath it so the fallible
  walk keeps the structural shape the infallible one has: no
  arithmetic, exact whenever `f` is, first refusal returned, columns
  and translation in their places.
- Whether `map` is then written as `try_map` with an `Infallible` error
  or kept as its own body is **this unit's call** — the items say the
  two are bit-identical either way. Decide it, say why in the PR, and
  pin the bit-identity claim with a row rather than asserting it.
- `SketchPlane::try_map` beside `SketchPlane::map`
  (`crates/profile/src/lib.rs`), because `pinned_plane` spells
  `SketchPlane::new(<walk>)` and without it the shape survives the
  retirement — the item says so explicitly.
- **The retirement, by announced seam.** `anchor.rs` and `wire.rs` are
  EVAL's ground and its item parks on ours: `map_affine` is deleted and
  `pinned_plane` routed through the new door **in this PR**, which the
  item explicitly permits ("PROPS' adopting PR may do it instead by
  announced seam"). List both files under a `## Seams crossed` heading
  with what changed, and never leave a third spelling beside the two.
- Rows: the fallible walk's component placement (a transposed column
  must red — mutate one in a scratch copy and show it does), the first
  refusal returned rather than the last, and `pinned_plane` still
  refusing on an analysis scalar exactly as it does today.

## Seam to be careful of

`crates/geom-core/src/linalg/vec.rs` is held by an unmerged unit
(`props/sign-hull`, PR #2468, holding on another program's fold) with a
large diff in `orthonormal_basis` and its tests. Put `Vec3::try_map`
beside `Vec3::map` and **do not touch that region**, so the two merge
textually when the held branch lands.

## Posture

- ε posture: none — no tolerance is read anywhere in this unit. Say so.
- Bit identity: `map`'s output must not move, whichever way you spell
  it; the row is the receipt.
- **This machine has a build mutex** (`local-scripts/with-build-slot.sh`,
  `memories/agent-lane-operations.md` §Build concurrency) and it is
  contended. Wrap every heavy cargo call, pass no `-j`, expect to lose
  the slot, and let hosted CI be the verification of record.
- Review: single style review, outside the experiment.
- **Landing: both items get `pr:` and `status: review`. DO NOT MERGE** —
  the orchestrator lands after the review and its fix pass, closes both
  items and deletes this spec at merge with its `## Per-merge deletion`
  section in `docs/DOC-LEDGER.md`. No `Co-Authored-By`, no `CI-Config:`
  trailer, no empty commits.

## Acceptance

One fallible walk in the kernel with its infallible twin, the profile
door beside it, `map_affine` gone and `pinned_plane` routed through the
door with the seam announced, a transposed-column mutation red, and
hosted CI green.

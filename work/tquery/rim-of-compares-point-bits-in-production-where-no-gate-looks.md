---
id: rim-of-compares-point-bits-in-production-where-no-gate-looks
kind: issue
title: rim_of decides one rim by a production bit compare of points and scalars, which no bit-identity gate can see
status: open
opened: 2026-09-24
refs: [rim-of-refuses-extruded-multi-arc-rims, the-re-basing-gate-refuses-m7-8-where-nothing-moves]
priority: P2
cost: M
design: true
needs_ev: true
---


## What

Found by the TOPO-B5 slot-2 review round (PR 3148), while the
re-basing gate's docs were claiming "`Point3<T>` has no door" for an
exact point comparison.

`crates/topo/src/query.rs` carries a bit comparison of kernel values:
`same_bits<T: Bounds>` (`query.rs:684`, `lo().to_bits()` and
`hi().to_bits()` of two scalars), `same_point_bits` (`query.rs:689`)
and `same_vec_bits` (`query.rs:694`) over it. `CircleId::same_circle`
(`query.rs:713-716`) uses all three to decide which arcs belong to one
rim, and `rim_of` (`query.rs:869`, the call at `query.rs:914`) is a
public door. So this is a production decision taken on bit identity of
two independently stored values: two arcs are "one circle" exactly
when their stored centre, radius and axis are the same bits.

It landed in `c512a2e34` (2026-09-04, "topo::query: rim_of, the rim
selector, with its four typed refusals"). No ratification of the bit
compare turns up (`git log --all -S'fn same_point_bits' --
crates/topo/src/query.rs` finds that commit only).

## Why it matters

- `docs/DESIGN.md`'s standing outcome: "**Production bit-identity
  coincidence checking is RETIRED** (Ev, #53; #102)", with
  `geom_core::bit_identity`'s production allowlist EMPTY and "a new
  consumer must be allowlisted and carry a retirement-scheduled note".
  Whether `same_circle` is coincidence checking (two arcs' carriers
  are one locus) or a structural read of stored data (the door's own
  doc at `query.rs:836-846` argues the second, "a total read of stored
  data — the EXACT class this module's header names") is the question;
  nothing records who answered it.
- `Bounds`' doc (`crates/geom-core/src/real.rs:999`): "A free-floating
  bounds-comparison helper would be the #701 `Enclosure` evasion with
  better manners, and stays out." `same_bits` compares two brackets'
  ends; it is a helper of that shape, though private to one module.
- **No gate can see it.** `scripts/gates/bit-identity-consumer.sh`
  matches `bit_identity::|repr_bits|eq_bits` (`:49`), so a `to_bits()`
  read of `Bounds::lo`/`hi` passes it; `bit-identity-punning.sh`
  matches the `Any`/`TypeId` idioms only; and
  `scripts/gates/bounds-allowlist.sh` fires on COMPOUND bounds
  (`Decide + Bounds`), so a bare `T: Bounds` door passes it too. The
  gate half is GUARD's ground (`scripts/gates/*`); it is carried here
  because it is only a defect if this row's answer is that the compare
  is a consumer.
- The measured cost of the rule the compare implements is already
  filed: `work/tquery/rim-of-refuses-extruded-multi-arc-rims.md` (the
  extruded arcs' centres and radii differ by ulps, so the bit compare
  refuses every extruded multi-arc rim).
- It is a precedent either way for
  `work/topo/the-re-basing-gate-refuses-m7-8-where-nothing-moves.md`,
  whose `[ev]` question is whether a kernel gate may ask "is `p_new`
  the point `p_old`, bit for bit".

## Shapes

- **It is a consumer**: route it through `bit_identity::eq_bits` and
  allowlist it in `bit-identity-consumer.sh` with a
  retirement-scheduled note, or replace the identity by a recipe-source
  (`GeomSource`) read, which is what N6 names as the ratified
  mechanism; and GUARD widens the tripwire to `to_bits` over
  `Bounds::lo`/`hi`.
- **It is not a consumer** (structural discrimination of stored
  data, not a coincidence decision): say so where the retirement is
  stated, with the ruling, so the m7-8 row's gate can cite it.

Either answer is Ev's: it reads a ratified retirement.

## Ruled (2026-09-24, PR 3156)

Ev: "(b), nice catch" — **it is a consumer.** `same_circle` decides
that two stored carriers are one locus from their bits, the
coincidence decision the retirement closed; N6 names recipe
provenance (`GeomSource`) as the channel that answers it. The repair
is this row's first shape: replace the identity by a `GeomSource`
read (preferred; it is the ratified mechanism), or, if some rim has no
recipe to read, route that case through `bit_identity::eq_bits`,
allowlisted with a retirement-scheduled note. The gate half is filed
on GUARD's slate:
`work/guard/the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds.md`.
The related cost row
(`rim-of-refuses-extruded-multi-arc-rims`, extruded arcs differing by
ulps) is decided by the same repair, since a recipe read does not see
ulps.

## The repair the ruling named has nothing to read (2026-10-02)

Measured by `tquery/rim-of-recipe` on main `5ab36cc95`, before any
build: every curve of a kernel-direct body (`extrude`, `revolve`, the
boolean, blend, `test_support`) is `GeomOrigin::KernelDirect`, so it
carries no `GeomSource`. editor-core stamps one source per curve
description (`stamp_minted_from`), so two arcs of one rim are always
distinct sources. The fallback (`eq_bits`, allowlisted) keeps the bit
identity, and with it the refusal of every extruded multi-arc rim.

## The repair: `rim_of` reads structure, not carriers

"These arcs are one rim" is already recorded in the body:
- **One surface each side.** The producer decides, through its own
  band, that a run of segments lies on one surface and shares the
  key. `extrude`'s `cosurface` → `FaceSurface::Shared`; STEP import
  dedupes by record.
- **One circle.** Two circle arcs between the same two surface keys
  that share a vertex lie on one circle, because distinct circles of
  one surface pair's intersection are disjoint.

So:

- **Membership.** The rim of `seed` is the closed chain, through
  shared vertex keys, of edges whose two side surface keys are the
  seed's two. No carrier is compared. `CircleId`, `same_bits`,
  `same_point_bits` and `same_vec_bits` are deleted, and the door's
  bound narrows from `T: Bounds` to `T: Real`.
- **Component.** The answer is the component through the seed. Other
  rims on the same surface pair are not a refusal: a plane through a
  torus, or a cylinder through a sphere, has two.
- **Order.** The walk follows the half-edges on one fixed side of the
  pair, so `rim_of(b)` is a rotation of `rim_of(a)` from any seed,
  whatever winding each arc's carrier was stored with.
- **Refusals.**
  - `CoSurface` and `NotIntact`: unchanged.
  - `NotOneRim`: names the vertex key where the chain dangles or
    branches, in place of a carrier parameter.
- **What moves.** Rows that pin the bit rule retire. The opposed-axis
  rows become answered rotations.

**Open:** whether the seed must still be a circle arc (`NotAnArc`), or
the door names any closed edge chain between two surfaces (an ellipse
rim from a tilted plane through a cylinder).

---
id: rim-of-compares-point-bits-in-production-where-no-gate-looks
kind: issue
title: rim_of decides one rim by a production bit compare of points and scalars, which no bit-identity gate can see
status: open
opened: 2026-09-24
refs: [rim-of-refuses-extruded-multi-arc-rims, the-re-basing-gate-refuses-m7-8-where-nothing-moves]
priority: P2
cost: D
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

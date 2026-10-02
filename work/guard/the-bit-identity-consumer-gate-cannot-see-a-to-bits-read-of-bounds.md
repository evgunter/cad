---
id: the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds
kind: issue
title: bit-identity-consumer.sh matches only bit_identity::|repr_bits|eq_bits, so a to_bits() read of Bounds::lo/hi — a production bit-identity consumer by Ev's ruling on PR 3156 — passes every gate
status: open
opened: 2026-09-29
refs: [3773, 3156]
priority: P2
cost: E
---

## What

Ev ruled on PR 3156 (2026-09-24, question 2: "(b)") that
`crates/topo/src/query.rs`'s `same_bits` / `same_point_bits<T: Bounds>`
(`:684-690`), which `rim_of`'s `CircleId::same_circle` (`:713-716`)
uses in production, is a production bit-identity **coincidence
check**: it decides that two stored circles are one rim from their
bits, which is the channel `docs/DESIGN.md`'s standing outcome
retired ("Production bit-identity coincidence checking is RETIRED",
Ev, #53; #102). The repair of that site is TQUERY's
(`work/tquery/rim-of-compares-point-bits-in-production-where-no-gate-looks.md`).

This row is the gate half. No gate saw the consumer when it landed
(`c512a2e34`, 2026-09-04), and none would see the next one:

- `scripts/gates/bit-identity-consumer.sh:49` matches
  `bit_identity::|repr_bits|eq_bits` only, so a `to_bits()` read of
  `Bounds::lo()`/`hi()` passes it;
- `scripts/gates/bit-identity-punning.sh` matches the `Any`/`TypeId`
  idioms only;
- `scripts/gates/bounds-allowlist.sh` fires on COMPOUND bounds
  (`Decide + Bounds`), so a bare `T: Bounds` helper passes it too.

## The shape to give

Widen the consumer tripwire to the spelling that escaped: a
`.to_bits()` on the result of `lo()`/`hi()` (or on any `Bounds`
accessor), outside `geom_core::bit_identity` and the allowlist, is a
consumer. It lands red while `query.rs`'s compare stands, so it rides
with, or after, TQUERY's repair; the selftest carries a planted
`same_bits`-shaped helper.

## Evidence (2026-10-02, TQUERY)

PR 3773 deletes the live instance cited above: `query.rs`'s
`same_bits`, `same_point_bits`, `same_vec_bits` and `CircleId` are
gone, and `rim_of` compares no carrier. The gate concern stands — the
spelling still passes every gate — so the widened tripwire now has no
live instance to land red on, and only the planted selftest helper
exercises it.

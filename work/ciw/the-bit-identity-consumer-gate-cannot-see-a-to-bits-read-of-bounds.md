---
id: the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds
kind: issue
title: bit-identity-consumer.sh matches only bit_identity::|repr_bits|eq_bits, so a to_bits() read of Bounds::lo/hi — a production bit-identity consumer by Ev's ruling on PR 3156 — passes every gate
status: open
opened: 2026-09-29
refs: [3773, 3156]
priority: P3
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

## Moved to BLIND (2026-10-26, GUARD's split)

GUARD measured 43 points against 30 and split three ways. This row is a
spelling a kernel-discipline gate's matcher misses (or refuses wrongly),
which is BLIND's charter (`work/ciw/plan.md`). Id and body unchanged.

Moved from BLIND to CIW on 2026-11-20, by `git mv`, when BLIND merged into CIW: BLIND held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

- 2026-11-22 — **Re-banded P2 → P3 by CIW triage.** There is no live instance: PR 3773 deleted query.rs's `same_bits` family, and the only incident is the 2026-09-04 landing. Widening a grep in the per-PR gates step costs very little, but it saves no fix cycles, so this is not a latency lever.

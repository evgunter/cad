---
id: split-strands-a-lone-below-bisector-as-loose-ends
kind: issue
title: split refuses UnpairedLooseEnds at a vertex whose real edges all sit above with only a wide sector's bisector below
status: open
opened: 2026-10-01
priority: P0
cost: M
---


## What

An ordinary split of an ordinary union refuses, on a plane with no
edge lying in it.

**Repro (on main at 7878b3cba, and on `origin/cleave/wrong-arc` at
6757e4b0b).** Take `brick(0..1.5, 0..1, 0..1) ∪ brick(1.2..1.3,
−1..2, 0.5..3)` and the plane through (1.2, 1, 1):
- with normal −(0.1, 1, 1)/|·|, `split` refuses
  `SplitJoinError::UnpairedLooseEnds { count: 2 }`;
- with normal +(0.1, 1, 1)/|·| it succeeds: above 0.4388, below
  1.7612, both closed.

Neither PR 3658 (planar-face pairing, on main) nor PR 3718's
`cleave/wrong-arc` (curved-face pairing) changes the answer.

## The mechanism

The plane passes through (1.2, 1, 1), where the block's rim meets
the slab's face x = 1.2. That face has a reflex corner there, so its
sector is stored twice and the duplicate carries the bisector
(`splitting/neighborhood.rs`, `classify_neighborhood`). Under the
failing normal, the classified neighbourhood is:

    Above → (1.2, 0, 1), Above → (0, 1, 1), Above → (1.2, 1, 0.5),
    Below* (the bisector duplicate)

Every real edge is Above. Only the duplicate is Below.
- `insert::above_runs` finds one Above run holding every real
  half-edge. Its `mev_null` moves all of them onto the copy, so the
  old vertex keeps only the null edge.
- The join cannot pair that end, and it refuses at
  `splitting/join.rs` (`UnpairedLooseEnds`, ~176).

The mirror is the configuration the insertion already handles: every
real edge Below and a lone Above duplicate is the dangling strut
(`insert.rs` module docs, "a lone wide-sector bisector duplicate").
The +n run is that case, and it succeeds.

`split`'s pinch lane (`splitting/mod.rs`, `split`) reruns mirrored
only on `DegenerateSection`, so it never retries this refusal.

## Pinned

`crates/topo/tests/split_tangent_edge.rs`,
`a_tangent_contact_meeting_a_real_section_cuts_only_the_slab`, pins
block ∪ slab under the tangent plane y + z = 2 with normal
(0, −h, −h) as this refusal (`count: 4`, two such vertices) for both
`split` and `plane_section`. It flips when this row is fixed. The
(0, h, h) orientation succeeds and is pinned beside it.

## Found by

CLEAVE `cleave-tangency`, measuring block ∪ slab for
`split-cannot-declare-an-exact-tangency-with-its-target`.

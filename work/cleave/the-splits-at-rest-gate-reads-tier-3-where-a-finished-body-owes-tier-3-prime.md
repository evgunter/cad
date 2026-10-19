---
id: the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime
kind: issue
title: The split's at-rest gate reads tier 3, where a finished body owes tier 3′: a curved one-end slit ships Ok and tier 3′ refuses it
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the review of PR 5350 (`cleave/keel-slit-ring`).

## What

`crates/topo/README.md` tier 3 says a finished body (`AtRestBody`)
passes tier 3′ against its own contact records, and that every door
returning one pays that gate at the door that built it. Ev folded that
bar in (`fe281ddb39`, "the finished-body bar is tier 3' against the
body's own contacts (Ev's fold, measured)"). `split`'s gate,
`AtRestPolicy::gate_at_rest` (`crates/topo/src/splitting/mod.rs`,
`split`), runs `validate_geometric`, tier 3 only.

Reached on main: the curved keel open at one end
(`crates/sweep/tests/a_curved_keel_in_the_plane.rs`,
`a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`)
splits `Ok`. On every row measured (6 of 6), the side carrying the slit
then fails `validate_pseudomanifold` with no records: one
`UnrecordedContact { EdgeEdgeOverlap }` on the slit's two coincident arcs
(`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`). PR 5350
refuses the closed form of the same keel typed (`CurvedSlitTrace`),
rather than ship it.

## Owed

The gate changes, not the doc: the doc's bar is Ev's. `split` runs
tier 3′ over each side against the records the split returns for it.
The curved slit's records (or its shared curve entry) come first, or the
one-end curved slit starts refusing at the gate. Measure what else the
3′ gate turns from `Ok` to a refusal across the split suites at three ε
before landing it.

A second witness: `a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses` (a hole left on a face of its own ships `Ok`; 3′ refuses).

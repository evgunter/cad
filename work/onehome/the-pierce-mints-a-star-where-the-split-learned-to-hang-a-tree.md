---
id: the-pierce-mints-a-star-where-the-split-learned-to-hang-a-tree
kind: issue
title: vtxfac's piercing side mints one null edge per Out run (a star) where the split had to hang a tree because two runs can bound two pieces: probe for a pinch the Boolean keeps
status: closed
opened: 2026-11-02
priority: P1
cost: M
branch: onehome/pierce-star-probe
closed: 2026-11-02
pr: 6360
---

Flagged independently by all three designers of the vertex-runs fork (cluster A, 2026-11-02). Unverified: a probe, not yet a confirmed wrong answer.

## What

The split used to mint one null edge per run at an ON vertex (a star). PR 4565 (`cb299d1009`) changed it to one per *corner* (chords paired in germ order), hung as `null::ring_tree`'s tree. The reason: two Below runs at one vertex can bound two different pieces, and a star mis-assigns material. The split's row is `a_reflex_corner_whose_below_edges_bound_two_pieces_hangs_a_path` (`splitting/insert.rs`, an L-prism reflex corner leaned through a plane).

The Boolean's pierce (`boolean/vtxfac.rs`, the runs loop in `classify_vertex_on_face`) still mints a star on the piercing side: one strut per Out run, keeping every In entry at one vertex. Its tree lives only on the pierced face's ring. One designer argues runs and corners are consistent duals (runs pair cuts adjacent on the orbit, hung in germ order; corners pair cuts adjacent in germ order, hung on the orbit), and that the Boolean minting on two bodies may make runs sound. Nothing on main pins it.

## The probe

Take the same L-prism reflex corner piercing a box's plane face, leaned as in the split's row, under ∪, ∩ and −. Check `corners_disjoint` and tier 3 on each result. If two In pieces stay joined at one vertex (a pinch) and the op returns it, or the at-rest gate misattributes it, this is a P0 wrong answer. If every op builds sound or refuses typed, close this row with the test kept as a pin.

The vertex-runs fork (`run-loops-of-split-and-pierce-are-twins`) decides where the chord choice lives; this row decides whether the pierce's current choice is sound.

## Closed (ONEHOME, 2026-11-02, PR 6360)

No wrong answer. The L-prism reflex corner pierced through a plate's face, at 5 leans × 2 sides × 9 offsets × 6 ops (420 Booleans), builds sound or refuses typed. Every built result passes tiers 3 and 3′ and `corners_disjoint`, with solid and shell counts matching the hand truth, volumes exact to ~1e-13, and one vertex per surface cone at the corner where material is pinched. The pierce's star at the piercing vertex minted two struts there and was right. The table stays as a slow-set pin (`crates/topo/tests/a_reflex_corner_pierced_through_a_plate.rs`). Not covered: several runs meeting on one ring, curved faces. On this fixture the result is consistent with the dual reading of runs and corners in the vertex-star fork ([ev] PR 6359), which keeps the chord choice a per-verb parameter.

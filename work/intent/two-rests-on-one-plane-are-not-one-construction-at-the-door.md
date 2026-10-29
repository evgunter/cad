---
id: two-rests-on-one-plane-are-not-one-construction-at-the-door
kind: issue
title: Two blocks each resting on one floor make a flush pair the door leaves unproven (over the budget), though the two rests generate their equivalence
status: open
opened: 2026-10-29
priority: P2
cost: M
refs: [tangent-and-coaxial-scenes-are-in-the-corpus]
---

Measured by INTENT stage 6 PR A on the corpus document
`blocks_on_one_floor` (`crates/editor-core/tests/corpus/blocks_on_one_floor.rs`),
pinned by `coincidence_door::two_blocks_resting_on_one_floor_are_an_unproven_flush_pair`.

Two unit cubes, each placed into a floor's space by one rest (its base
on the floor's top, `fixture::rest`) and slides written `0` and `1`
along `x`, so they abut; the two copies unioned. The union's rows:

| row | relation, site | proof |
|---|---|---|
| bases, tops (×4) | `SameOriented`, `PlaneLadder` | unproven/`Numeric`/`Budget` |
| abutting walls | `SameOpposite`, `PlaneLadder` | unproven/`Numeric`/`Budget` |
| corners on cells (×4) | `OnCarrier`, `VertexFusion` | two `OneConstruction`, two `Budget` |

and the world `⊔`'s row of the union's base on the floor's top,
`SameOpposite` at `CensusAtRest`, unproven/`Numeric`/`Budget`.

`[coincidence.structural]`: "Being one construction is the equivalence
the document's constructions and mates generate". Each rest equates a
block's base with the floor's top flipped, so the two bases are one
construction read twice through the floor, and the union's flush pair
of bases is structural by rung one. The door's rung one asks for one
mate whose two faces are the row's two cells (`stated_by_a_mate`,
`coincide.rs`, over `mate::planes_equated`) and closes over no chain
of mates, so it misses a pair whose equivalence runs through a third
face; the replay then leaves the row over the budget. The tops are flush through the two cubes' heights, which
is no mate's statement, so they are rightly rung two's or three's.

**Done when** the door proves the bases' flush pair by rung one (the
mates' generated equivalence), and the pin above flips that row.


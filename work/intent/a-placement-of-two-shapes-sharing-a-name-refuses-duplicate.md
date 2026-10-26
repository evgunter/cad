---
id: a-placement-of-two-shapes-sharing-a-name-refuses-duplicate
kind: issue
title: A placement of several shapes names each copy's entities by the shape's name alone, so two shapes carrying one name refuse Duplicate
status: closed
opened: 2026-10-22
priority: P1
cost: M
refs: [a-mirror-is-a-construction, a-placement-adds-no-name-segment-is-said-three-ways]
closed: 2026-10-26
---


Found by `a-mirror-is-a-construction` (2026-10-22, PR 5607's review pass). `Place [plate, mirror(plate)]` with no constraints refuses at evaluation:

`Naming(Duplicate { name: Placed { of: <plate's OutputBody> } })`

`names::name_placed_copies` (`crates/editor-core/src/names/emit.rs`) wraps every name of shape `k` as `Placed { of: name }` at the placement and keys nothing by the shape's read. Two shapes whose tables share a name therefore collide on it. Mirrors hit this: a mirror's image carries its source's names verbatim (`names::VerbatimEdge::Reflected`). So do a split's two halves, whose names spell no half (N1, the scope), so `Place [above, below]` should refuse the same way. A union keys each member by its read (`RoleSeg::From`) and does not collide (`crates/editor-core/tests/a_mirror_is_a_construction.rs`, `a_body_and_its_mirror_side_by_side_name_every_entity_apart`).

N0's NS1(b)/NS4 already say identical operands are told apart by their reads, a copy's `CopyKey` among them, so the fix is likely `From`-style keying in `name_placed_copies`. It meets `a-placement-adds-no-name-segment-is-said-three-ways`, which may retire `Placed` altogether: settle them together.

## Resolved by D′3 (INTENT, 2026-10-26)

D′3 (PR 5636, merged `29110056e8`) made `Place` copy exactly one source body. A placement of several shapes became a `⊔` of placements, and its operands are keyed by their reads (test 19). `names::name_placed_copies` is gone from `crates/`, so `plate ⊔ mirror(plate)` no longer collides on a shared name.

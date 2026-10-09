---
id: annular-one-segment-tube-through-a-plate-refuses-section-loop-undecided
kind: issue
title: An annular one-segment tube through a plate, its two wrap edges at different azimuths, refuses SectionLoopUndecided: both region faces of the inner loop's null face have every witness on the tube
status: open
opened: 2026-10-08
priority: P1
cost: H
---

Found by the PR 4345 dual review (r2 MINOR 5, the `F3 annulus` probe on
`join/wrap-edge-section-loop-review-r2`); measured again on the fix
pass. PATHS unit 4 (`circle-lowers-to-one-segment`) makes this shape
from every washer or annular boss.

## Witness

- The tube: two one-segment circles about the origin, `r = 1` with its
  vertex at azimuth `ao` and `r = 0.5` at azimuth `ai`, extruded
  `z ∈ [0.5, 2.5]`.
- The plate: `brick((−3, 3), (−3, 3), (0, 1))`.
- The plate's top face cuts both walls across their seams: two one-site
  loops in one face, which the wrap-edge arm joins
  (`boolean/join.rs` `wrap_site_segments`, `chord_join.rs`
  `ChordJoiner::join_lone_ring`).

| `ao`, `ai` | all six ops (∪, ∩, ∖, both orders) |
|---|---|
| 0, 0 | build, at the closed form (∩ `0.375π`) |
| 0, 1 | build, at the closed form |
| 0, π | `Join(SectionLoopUndecided)` |
| 1, 4 | `Join(SectionLoopUndecided)` |

## Where it stops

`boolean/join.rs` `resolve_roles_geometric` and `loop_roles`, on the
plate. The second null face's two loops flank:
- the inner conic's disc;
- the annulus between the two conics.

Both faces are planar, and every vertex and edge midpoint of each lies
on the tube's walls. So `complex_side`'s ladder reads both loops
undecided, and neither loop fixes the roles. The shape is the planar
cousin of `work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior.md`:
the faces are planar, and the ladder still has no witness inside them.
Why the aligned and near-aligned azimuths build has not been traced.

## What a fix would be

One of:
- a witness inside a planar region face, such as a point of the face
  off its boundary loops, sided through the same door;
- a role read that does not need one, such as each loop's winding
  against its region face's sense.

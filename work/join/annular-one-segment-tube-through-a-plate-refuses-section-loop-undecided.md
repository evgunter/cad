---
id: annular-one-segment-tube-through-a-plate-refuses-section-loop-undecided
kind: issue
title: An annular one-segment tube through a plate, its two wrap edges at different azimuths, refuses SectionLoopUndecided: both region faces of the inner loop's null face have every witness on the tube
status: closed
opened: 2026-10-08
priority: P1
cost: H
branch: join/annular-tube-roles
pr: 4397
closed: 2026-10-09
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

## Measured (`join/annular-tube-roles`)

Why the aligned pairs built: the ladder's rung 3 (`stands.rs`
`face_interior_point`) drew its candidates from vertex points alone.
The disc's one loop has one vertex, so it offered none. The annulus
offered one chord, between the two vertices. At `(0, 0)` and `(0, 1)`
that chord's midpoint lies in the annulus and decides. At `(0, π)` and
`(1, 4)` it lies in the disc, at radius 0.25, so neither region
decided. A two-arc wall has two vertices per circle and built at every
pair on main.

## Built (`join/annular-tube-roles`)

Rung 3 gains a third candidate source after the vertex ones,
`stands.rs` `across_edges`.
- From each edge's parameter midpoint `m` it runs one line inward:
  along `normal × t`, `t` the edge's direction as the face's half-edge
  walks it, since a face's interior lies to the left of its
  half-edges about its outward normal.
- Each meeting `m + s·w` of that line with a line or conic carrier of
  the face's loops, `s` decided positive, proposes `m + (s/2)·w`.
- The nearest meeting is no farther than the line's first exit from
  the face, so its midpoint is inside the face however thin the face
  is. `point_in_face` certifies each candidate as before.

A planar face offers no such witness where, for example, from every
edge the nearest boundary along the inward line is a spline or a
spiric (neither yields meetings), or the face is narrower than the
band. A spurious meeting near `m` (an unbounded line carrier, a
conic's far side) does not cost the witness: every meeting proposes a
candidate, so the first exit's midpoint is still among them. The fix does not depend on annuli or on the join: it is the one
ladder the shell witness, tier 3's check 10 and the pieces sort also
read. Pinned in `crates/topo/src/stands.rs` `rung_three_rows` and
`crates/sweep/tests/an_annular_tube_through_a_plate.rs`.

## Closed

Built as above by PR 4397 (merged `1a8170a5f7`, 2026-10-09): every
azimuth pair builds in all six ops at its closed form.

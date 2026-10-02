---
id: JOIN-1
kind: unit
title: A section germ names the cell it lies in (OnEdge or InFace) per operand, and the join matches on it
status: spec
opened: 2026-10-02
priority: P0
cost: H
branch: join/1-germ-locus
refs: [an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired, blind-d-pocket-subtract-refuses-with-join-internal-words, closed-in-face-section-loop-has-one-site, dumbbell-joint-union-leaves-four-loose-ends]
---


Spec: `docs/JOIN-1-SPEC.md`. Carries
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`,
and the typed-refusal half of `closed-in-face-section-loop-has-one-site`.

## Built (branch `join/1-germ-locus`)

- `HalfGerm` carries `a_locus` / `b_locus` (`boolean::Locus`:
  `InFace(face) | OnEdge(edge)`), derived by ONE function,
  `sectors::germ_locus`, from the attributed sector and its bounds'
  codes as first read; the vertex-on-face runs and the vertex-vertex
  insertion both call it before their surgery.
- `find_match` and `loose_partners` compare loci on both operands;
  an `OnEdge` germ whose edge is not incident to its site refuses
  `JoinDesync`.
- One fold rule, `sectors::fold_on_bound` (mixed → In): vtxfac's
  on-entry resolution, recl's edge-sector attribution (the Out-keyed
  flanker) and edge-edge record choice all read it.
- The boolean lanes' adjacency skip is `SegmentEdge::Is(locus edge)`;
  the split lane keeps `SegmentEdge::InPlane`.
- A record whose two germs share loci along a conic, left unmatched
  at quiescence, refuses `Join(SingleSiteSectionLoop { count })`.

## Measured

- `axis_lap.rs` `an_axis_lap_builds_every_op_as_its_planar_twin_does`:
  rod and diamond, ∪/∖/∩, cutter `y ∈ [0, 1]` and `[−1, 0]`: all
  twelve build, pass tiers 2, 3 and 3′, at the closed form (rod ∖
  `2.748893571891069`, rod ∩ `0.39269908169872414` = πR²/2).
- The closed tube in the box top refuses
  `SingleSiteSectionLoop { count: 2 }` under ∪, ∖, ∩.
- The merged teapot cup passes the join and refuses
  `VolumeUnmeasured` (`NotIsoRectangle { "props_rim_level" }`).
- The blind D top pose still refuses `JoinDesync { "ring-run winding
  is degenerate (zero enclosed area)" }` (JOIN-3's ground).

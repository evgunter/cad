---
id: a-cut-across-two-curved-or-partly-overlapping-edges-on-one-locus-mints-two-point-keys
kind: issue
title: A cut crossing two edges on one locus that are curved, or straight without both ends' point keys shared, mints a point key per crossing
status: open
opened: 2026-11-22
priority: P2
cost: M
---

Found by `heldrecords/split-keys`'s sweep. Ev's ruling on PR 5967
(`the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`,
"## Decided (2026-10-28)") says crossings a cut mints on two edges
recorded as one locus share one point key.

## What

`crates/topo/src/splitting/classify.rs` `insert_crossings` shares one key
only on the line lane, and only for two line edges ending on one pair of
point keys. Two line segments between the same two points are one
segment by structure, so their crossings are one point with no reading.
Two doors still mint a key per crossing:

- **Curved: the conic lane.** Two arcs on one pair of end keys are not
  one locus by structure (the two halves of a circle share both ends),
  and a conic edge can cross twice. Today no curved pair is recorded as
  one locus; plan step 3
  (`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`, the
  `SplitOn` row) records them. Once it does, a later cut across a curved
  slit or ring mints two keys at each crossing. That is the curved
  analogue of `a_later_cut_across_the_ring_builds_and_so_does_one_through_its_tip`.
- **Straight, recorded, ends not shared.** Two line edges that overlap
  only in part, under an `EdgeEdge` record of the operand
  (`ContactRecords::edge_edge`), cross the plane at one point when both
  cross it. But an `EdgeEdge` record also covers two edges crossing at a
  point, so being one line takes a reading, and `insert_crossings` is not
  handed the operand's records (`split` holds them; `reduce` takes a bare
  body).

No witness reaches either door today: no suite operand holds a curved
pair recorded as one locus or a partly-overlapping recorded line pair
that a cut crosses.

## Direction

For a pair recorded as one locus, the cut reads which crossings of the two
edges are one point. For lines that is the one point where the line meets
the plane. For a conic, match the roots by parameter on the shared
carrier, or decide the gap between them Zero, which owes a `VertexFusion`
row per `crates/topo/README.md` tier 3′. Then land the second crossing on
the first's key, as the line lane does.

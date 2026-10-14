---
id: a-face-pair-row-only-where-its-decision-shaped-the-result
kind: issue
title: A boolean records a face-pair row only where its decision shaped the result, not wherever the two faces meet
status: open
opened: 2026-10-10
---


## What

Since stage 4 B2 (`contact-records-cite-their-decision`) a boolean
keeps a face-pair row (plane or carrier ladder, tangent witness,
coaxial sphere; declared or glued) wherever the two faces' closures
meet, at a point, an edge or an area (`crates/topo/src/boolean/glue.rs`
`touched`). That is frame-independent, which was the defect E's review
measured. But it keeps rows for decisions that shaped nothing the
result holds. Measured by the B2 dual review (R2):

- two unit blocks kissing at a corner, A∖B: 3 face-pair rows (the
  three pairs of coplanar faces through the corner), 0 vertex rows,
  0 records;
- the same kiss, A∪B: the same 3 face-pair rows beside the one
  `VertexFusion` row; the faces meet at one point and the carrier
  identity glues nothing;
- two blocks in edge contact, A∖B: 4 face-pair rows.

Each such row is an `unproven-coincidence` finding.

The narrower reading: a face-pair row only where the decision shaped
the result. For a one-carrier pair, the faces share an area or the
merge glued them across an edge. For a tangent pair, the locus lies on
both faces. Spec test 15 (a flush plate on a block, ∪) must keep its
Rest row, though its result holds no merged face and no surviving
record: the touch collapses into incidences.

## First step

An area-overlap reading for a one-carrier pair. The reduction's
pending rows do not give one: two identical squares and two squares
sharing an edge both show only vertex-vertex rows. Candidates: the
merge groups (`BooleanNaming::merge_groups`), and the classification
of a fragment of one face as lying on the other.

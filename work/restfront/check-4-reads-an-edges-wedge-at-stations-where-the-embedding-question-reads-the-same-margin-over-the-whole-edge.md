---
id: check-4-reads-an-edges-wedge-at-stations-where-the-embedding-question-reads-the-same-margin-over-the-whole-edge
kind: issue
title: Check 4 reads an edge's wedge at stations, and the embedding question reads the same margin over the whole edge from a second call site: the two are one lever source and one number, not yet one reading
status: open
opened: 2026-11-22
priority: P2
cost: H
---



## Finding

From EMBED's lane `embed/remeter` (2026-11-22), filed on this slate
because the ground is check 4 (`crates/topo/src/validate.rs`,
`crates/topo/src/edge_wedge.rs`, `geom_brep::edge_wedge_over_as`).

`docs/DESIGN.md` `[topology.tiers]` says the embedding question
"reads tier 3's facts from its verdict, never re-deciding one over a
cell", and of its local reading: "That reading is the edge's wedge as
tier 3 reads it, one reading for both". **The engine does re-decide
the wedge, in every cell pair near the edge.** It did before
`embed/remeter` too, at a cell's own span; after it the two are one
lever source and, at first order, one number, read from two call
sites:

- **Check 4** reads `sin θ · arm` at the interior stations of the
  edge's certification schedule (`geom_brep::interior_stations`), the
  arm `folded_lever_arm`'s.
- **The embedding question** (`crates/topo/src/injective.rs`,
  `sign_on`) reads, in every cell pair near the edge, the sine at which
  a face leaves a separator, levered at the edge's reach
  (`wedge_reach::reach_over`). With the other face's carrier as the
  separator and the cell's zero side on the edge, that sine is check
  4's own `|n₁ × n₂| / (|n₁||n₂|)`, as an enclosure over the cell's
  stretch of the edge. The cells cover the whole edge.

They can disagree one way: check 4 definite at its stations, the
embedding question in band over a cell. That is a wedge that pinches
between stations, or an enclosure too wide to tell, and the question
refuses. So a body can pass tier 3 and then refuse to finish on the
margin tier 3 was asked for.

They also differ in the arm, and through it in class: beside a face
whose radius is far below its reach the engine signs a definite crease
where check 4 reads the smooth class at the same point
(`work/embed/the-departure-is-levered-at-the-reach-alone-where-check-4-folds-the-faces-curvature-arms.md`).

## What is needed

One reading. The state in which "decided twice" cannot be written is
check 4's first-order class folded from enclosures over pieces of the
edge, at a scalar that encloses, in place of its stations: the reading
the embedding question's carrier arm already takes per cell. What that
takes:
- a whole-edge fold beside `geom_brep::edge_wedge_over_as`'s station
  fold, over pieces of the edge's span, its escalation naming the piece;
- at `f64` a body is not an enclosure, and the embedding question lifts
  it; check 4 would read the lifted surfaces, or take the fact from the
  question, which today is asked after tier 3 passes;
- the arm's fold over a piece that ends at an apex, which reaches 0
  there (the EMBED row above).

`embed/remeter` changed nothing in `validate.rs` or in check 4.

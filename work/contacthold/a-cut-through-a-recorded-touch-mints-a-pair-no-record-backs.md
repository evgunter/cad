---
id: a-cut-through-a-recorded-touch-mints-a-pair-no-record-backs
kind: issue
title: A cut through a recorded touch mints a pair of cells at one point, and no decision backs it
status: closed
opened: 2026-10-11
priority: P1
cost: M
refs: [surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key]
closed: 2026-10-12
pr: 4809
branch: contacthold/one-record-carry
---

## What

A split whose plane crosses a recorded touch mints a cell on each
touching cell at the crossing, and the two share a point. No decision of
the split relates them, so each side refuses the minted pair
`UndeclaredContact`, and with it the half of the touch the pair now
bounds. By D1 as written the refusal is correct: the pair is a bound no
record backs, and a missing bound is never inferred. The question is
whether a cut through a recorded touch should back the pair it mints,
and how.

## Measured

On `contacthold/divided-cell-carry` (the branch that carries a divided
cell's record onto its pieces):

- **Edge kiss** (two unit cubes sharing the edge x = y = 1, two
  vertex–vertex records bounding the overlap), split at z = ½
  (`a_split_across_an_edge_kiss_refuses_both_sides`,
  `crates/topo/tests/review_4694_carry_probes.rs`). The split cuts each
  kissing edge in `classify::insert_crossings`, one `split_edge` per
  edge, each new vertex ON "by the decision that placed it" (its own
  edge's crossing root) and cached without re-measuring.
  `SplitResult::coincidences` is empty: the split decides no pinch here
  and nothing relates the two minted vertices. Each side keeps one end
  record and refuses `UndeclaredContact { VertexVertex }` at
  (1, 1, 0.5) and `UndeclaredContact { EdgeEdgeOverlap }` along its half.
  With the boolean's edge–edge record on the kiss (PR 4740), that
  record lands on both sides, on each side's pair of edge pieces; each
  side still refuses the minted pair at (1, 1, 0.5).
- **Ridge on a slab** (a cube turned 45° about x, its bottom edge on the
  slab's top from x = 1 to 3, two vertex–face records bounding the
  overlap), split at x = 2: the ridge edge and the slab's top are cut at
  (2, 2, 1), and each side refuses `UndeclaredContact { VertexOnEdge }`
  (the ridge's minted vertex on the slab's minted chord) and the half
  `EdgeFaceOverlap`. Same class, a `(vertex, edge)` pair rather than a
  `(vertex, vertex)` one.

## Why the carry cannot back it

D1 (`docs/DESIGN.md`) has result-side touching arise "only from those
coincidences propagated through the op, or from an op's copies of one
vertex at such a coincidence". The minted pair is neither: two distinct
cells minted by two independent decisions, not copies of one vertex.
And a straight overlap is "confirmed by reconstruction from its bounding
vertex records … a missing bounding record is an unbacked contact,
never inferred". The minted pair is a new bound of each half, so it
needs a record, and a record cites a decision; the split made none that
says the two are one point. A cut through a corner kiss is closed
because there the cut copies one vertex (`SplitNaming::vertex_pairs`).

## Options (a design fork; weigh before building)

1. **The split decides the pair.** Where a minted cell lies on a carried
   touch, the split reads its gap to the other side's minted cell (or to
   the other cell's piece) and records a coincidence of its own, a new
   `DecisionSite` (the boolean's precedent: `ops::ee_lineage` records a
   vertex its split minted on a carried edge–edge record against the
   other piece, with a fresh `VertexOnVertex` decision where it meets a
   vertex, under `DecisionSite::VertexFusion`). The carry then records
   the pair citing that decision and the carried records that bounded
   the touch. Needs: the new site, which carried records the pair cites,
   and the carry handing back decisions as well as records.
2. **The pair is propagated, not decided.** The cut meets a line the
   carried bounds already back at one point, so the two minted vertices
   are that point by lineage, and the pair cites the bounding records
   alone. Needs D1's "never inferred" to admit an op's inference from
   its own records, which it does not today.
3. **The split refuses typed.** A cut through a recorded touch refuses
   at the split, naming the touch, instead of building two sides the
   census refuses. Honest, and keeps the status quo's reach.

## What it blocks besides

A record landing on more than one pair of pieces (an edge–edge or
edge–face touch divided along its length) has no row that passes at
rest: a split that divides such a touch mints exactly this pair, and
neither the shell's rim surgery nor a blend divides a cell into two
live pieces that one touch spans (a hole's rim and the outer rim are
parted by the opening; a blend's carved piece dies). Once this row
backs the pair, the split's multi-pair landing gets its row.

## Resolved (PR 4809)

Ev ruled on `[ev]` PR 4749: the split's minted pair cites the carried
record alone (one plane cut both), the boolean decides such a pair at
its door, any other op refuses. The one carry lands it:
`crates/topo/tests/one_record_carry.rs` rows the edge kiss and the
ridge on a slab (the boolean writes both the edge–edge and the
edge–face row), each side passing at rest.

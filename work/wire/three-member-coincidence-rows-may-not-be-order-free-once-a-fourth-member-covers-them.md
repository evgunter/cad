---
id: three-member-coincidence-rows-may-not-be-order-free-once-a-fourth-member-covers-them
kind: issue
title: DM4's three-member coincidence row may not be one row in every member order once a fourth member covers the point (intersect pinches, union glues)
status: closed
opened: 2026-10-18
priority: P2
cost: M
design: true
refs: [a-union-fold-publishes-no-pinch-row]
closed: 2026-10-19
---


## Finding

Raised by a FORK-FOLDPINCH designer (2026-10-18), reasoned, unmeasured.
For a union every pinch is some pair's pinch (near the point the result is
the union of the members' local cones, so two members in different pieces
pinch on their own), so the judgement holds every union pinch row. For
intersect that argument fails: with non-convex local cones three members
can pinch together where no two do, and DM4's rule for such a row ("a
three-member row is the union's row too, spelled by its member cells, so
it is one row in every order") may not hold once a fourth member covers
the point: in one order the third member's step decides it, in another the
fourth has already removed the region and no step does. The same may hold
for a three-member glue row. Measure first: three intersect members whose
local cones pairwise meet in one piece each but all three together in two,
plus a fourth covering the point, over every member order. If real, the
fix likely changes DM4's sentence, which is Ev's.

The other designer added a clarification DM4 is silent on: a fold row
whose cell the fold itself minted (a member corner landing on a seam
junction of two earlier members) — spell it by its canonical head in the
node's space. Unsure whether reachable.

Signed (WIRE orchestrator).

## Measured for unions (2026-10-19, PR 5328's review)

Real for a **union**, with no cover needed. Measured on `origin/main`
(merge base f6dbcfe1f7) and on PR 5328's head; presence is the same on
both. Each fixture's published row multisets were compared across every
member order, with the union's own VarId and node id normalised out.

- `union_pinch_member_order`'s `crossing_one_wedge` (plate plus three
  prisms): 8 distinct rows; per-order counts vary from 4 to 6. Four rows
  are order-dependent (0 = plate, 1 = a, 2 = b, 3 = c):
  - The junction row at (1.5, 1, 1): b's `LateralEdge(Leg)` against the
    junction vertex, `OnCarrier` `VertexFusion`. It is published in 2 of
    24 orders (`[3,1,0,2]`, `[1,3,0,2]`), although the finished body holds
    that one junction and names it identically in all 24.
  - `Seam{a Lateral, c Lateral}` against b's `CapVertex(End)`: 8 of 24,
    every order folding a and c before b.
  - c `LateralEdge(Leg)` against a `LateralEdge(Leg)`: 6 of 24.
- `union_member_orders_decide_alike`'s `flush_onto_an_edge`: a steady 15
  rows, but three statements are each published in only 2 of 6 orders.
  The test's doc accepts this in prose.
- `pinch` (6 orders) and `three_on_a_line` (24 orders) are clean.

The cause, by the reviewer's reading: #4323's rule that a fold row is
published where a record cites it. A three-member row's presence follows
whether the step that meets it runs before the points coincide. DM4 says
"one row in every order".

Signed (WIRE orchestrator).

## Weighed (2026-10-19, FORK-ROWPRESENCE, three rounds)

Two designers, briefed with the problem only, converged after crossing
once.

- **Measured.** Every order-dependent statement in both fixtures is
  implied by judged rows, but only 2 of 6 through lineage; 4 need line
  and plane incidence theorems. Every step decision is cited by a record
  (106/0), so the citing filter is not the cause: different orders take
  different decisions at one point. A third face through a seam (the
  pencil) records no coincidence at all, in `topo` or anywhere.
- **Who reads the rows.** Every reader takes them per evaluation, in
  decision order:
  - the coincidence door and its replay, which match each row by site,
    position, cells and relation;
  - contact records' `Backing::Decided(k)`;
  - Python's `coincidences(node)`.

  Nothing compares rows across orders except this test.
- **Recommendation.** A node's rows are its evaluation's decision log:
  - the judged rows plus every decision a fold step takes, unfiltered;
  - each spelled as the set of member cells that meet, with a cell the
    fold minted resolved to the member cells it is made of;
  - each marking whether it is judged or a fold step's.

  The judged rows are the order-free subset. DM4's "so it is one row in
  every order" is withdrawn. "The fold mints no contact verdict of its
  own" is limited to carrier pairs. The author-facing findings come out
  order-free in practice: an implied fold row proves when its premises
  do, and a surviving irreducible coincidence is met in every order.
  The residue is a covered irreducible coincidence, recorded only in the
  orders that meet it before the cover. That is the same residue as a
  step's refusal.
- **Prerequisite in `topo`.** `cite_rows` emits a decision as a row only
  where a record cites it or it pinches. Every Zero decision a boolean
  takes becomes a row, in pair booleans too.

The `[ev]` PR edits DM4 (`crates/editor-core/REFERENCES.md`); design-fork
log row 169. Approved by Ev on PR 5329 (2026-10-19).

## Resolution (2026-10-19)

Answered by PR 5378, which builds DM4's decision log (Ev, PR 5329):
- a union's rows are its judged rows plus every fold-step decision, uncited included;
- each row is spelled as a sorted set of member cells with an origin, a fold-minted cell resolved to the member cells it is made of;
- `Relation::Meet` covers a meeting at a point;
- a row's margin is reported, not part of its identity.

Pinned by `union_member_orders_decide_alike` (judged rows one multiset in every order; `every_fold_row_names_every_member_cell_its_cells_are_made_of`; `the_log_is_a_function_of_the_member_order_alone`) and `union_pinch_member_order`'s 120-order cover.

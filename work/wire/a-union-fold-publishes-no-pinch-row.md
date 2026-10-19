---
id: a-union-fold-publishes-no-pinch-row
kind: issue
title: An n-ary union or intersect publishes no pinch row: the pairwise judgement drops its vertex identities and a fold step publishes only the rows a record cites
status: closed
opened: 2026-10-12
priority: P2
cost: M
pr: 5276
closed: 2026-10-18
---


Filed by CONTACTHOLD's `zero-to-distinct-cells-on-one-point-doors-audit`
(branch `contacthold/zero-to-distinct-cells-audit`).

## What

That branch makes the boolean record the vertex identity behind a
pinch: where the result keeps one vertex per cone on one point key,
`topo`'s `boolean/ops.rs::carry` emits the reduction's row for that
vertex (`pinch_rows`) as a `VertexFusion` coincidence, though no
contact record cites it. DESIGN's coincidence discipline asks for it:
an ON verdict that makes pieces of one result touch is a coincidence,
recorded.

A subtract, and a union or intersect of two members, publish the op's
coincidences whole, so the row reaches the document. A union or an
intersect of three or more members does not: both lower through
`crates/editor-core/src/eval/wire.rs::wire_combine` (`Node::Union` and
`Node::Intersect` share the arm), and its pairwise branch is shared:

- the pairwise judgement filters every `VertexFusion` row out
  (`.filter(|row| row.site != topo::DecisionSite::VertexFusion)`),
  on the premise that the fold step that keeps the touch publishes
  the row its record cites;
- a fold step publishes only `contacts.decided()`, the rows a record
  cites (`fold_rows`), and the node publishes only the fold rows its
  final records cite.

A pinch row is cited by no record, so neither path publishes it, and
the `unproven-coincidence` lint never sees the coincidence. That is
P2: the lint is the only reader that proves a coincidence holds
across the family, and a dropped row is a coincidence it cannot
fail.

## Witness

The L-prism and tilted cube of
`crates/topo/tests/records_cite_their_decision.rs::a_pinch_records_the_vertex_identity_that_made_it`
(the poses of `crates/sweep/tests/join_pierce_strut_facing.rs`'s
`TWO_RUNS`): their union and their intersection each pinch at the
prism's reflex corner. Read, not run: as a document union of the
prism, the cube and a far third member, or an intersect of the two
with a third member holding the corner inside, the node's
coincidences would hold no row naming the corner.

## What it needs

A fold step's pinch row has to be published when the final body still
pinches there, and not when a later step consumed the pinch. A later
step reads the accumulator's copies as structure and decides nothing
there, so the publication cannot be read off the last step alone.

## Measured (2026-10-18, `wire-pinch`)

The witness as an editor document (the L extruded z 0 to 1 on the xy
frame; the cube a square of half-width 2 on the frame through
(1, 1, 1) with normal (1, 1.3, 0.7) normalised, extruded 4), at
`4489bedb04`, counting `VertexFusion` rows per pairwise judgement, per
fold step, and on the node:

- `Union [prism, cube]`: the one step holds one `VertexFusion` row no
  record cites; the node publishes it; two vertices at the corner.
- `Union [far, prism, cube]`: the judgement of prism and cube holds
  one uncited `VertexFusion` row, step 2 holds one; the node publishes
  none, and the result holds two vertices at the corner. The defect.
- `Union [prism, cube, cap]` and `[cap, prism, cube]` (cap: the block
  0.8..1.2 in x and y, z 0.8 to 1.2): the judgement of prism and cube
  holds the uncited row in both; in the first order step 1 holds it
  too; the result holds no vertex at the corner. A consumed pinch the
  judgement and the first step both decided.
- `Union [prism, cube, far]` and `Intersect [prism, cube, big]` refuse
  in naming before any row is published
  (`work/refs/a-pinch-carried-through-a-later-fold-step-refuses-naming.md`).

## Decided (2026-10-18, FORK-FOLDPINCH, designers converged): DM4 already answers it

Both designers, briefed with the problem only, rejected the brief's
criterion ("the coincidences true of the finished body") and found the
answer already ratified: `crates/editor-core/REFERENCES.md` DM4 (PR 4323,
Ev-approved, fork-log row 94) — "its rows, in member space, are the
union's coincidence rows"; "a contact a third member covers is still its
pair's, recorded"; "No row names a cell the fold minted"; "drop a fold row
only when a judged row states it". A row records a decision the kernel took
from values, not a feature of the body: a covered pinch routed the build
and a replay refuses at it, so its `unproven-coincidence` finding is true.
`wire_combine`'s `VertexFusion` filter departs from DM4 for one site, and
its justifying comment is unratified agent text.

**The build:** drop the filter, so the judgement's rows are published
whole, `VertexFusion` included, in member space; spell each fold row by
member cells (the `From { read, of }` head of its collapsed name, piece
qualifiers stripped, as `NamedCell::Entity { input: read, name: of }`; a
cell the fold minted stays in the node's space under its canonical head);
and before publishing a fold row, cite the judged row that states the same
two member cells and relation instead of publishing it twice. A step's
`inherited` pinch stays silent (topo's `pinch_rows` is right). This also
fixes `a-fold-steps-coincidence-rows-are-named-without-the-end-passes`
(no row is named against an intermediate fold table), which lands in the
same unit and closes with it. Pin: the rows are one multiset across
member orders (extend `union_member_orders_decide_alike`), for the orders
that build today; REFS's `a-pinch-carried-through-a-later-fold-step-refuses-naming`
blocks the rest and is independent.

Not Ev's: no ratified text changes. One designer flagged an unmeasured
gap for intersect (three-member rows may not be order-free once a fourth
member covers them), filed separately.

Signed (WIRE orchestrator).

## Resolution (2026-10-18)

Closed by PR 5276: a union's coincidence rows are its member pairs',
named in member space, one row per statement over judged and fold rows;
pinned by `a_vertex_row_is_the_unions_once_in_every_member_order`
(`crates/editor-core/tests/union_member_orders_decide_alike.rs`). What
stays open is the minted-cell spelling,
`a-fold-row-names-a-vertex-the-fold-minted-by-the-step-that-minted-it`.

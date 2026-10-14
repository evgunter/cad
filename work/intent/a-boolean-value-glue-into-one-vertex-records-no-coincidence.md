---
id: a-boolean-value-glue-into-one-vertex-records-no-coincidence
kind: issue
title: A boolean that glues two operand vertices into one result vertex by a value decision records no coincidence: the pending row dies with no record to cite it
status: open
opened: 2026-10-11
priority: P2
cost: M
design: true
---


## What

Raised by FLUSH's PR 4728 dual review (r2, confirmation round). It
predates that PR, which only widens where the gluing is grouped.

A boolean decides vertex identities from values: `bool_contact_vertex`,
a vertex of one operand on another's vertex, edge or face. It also
groups several pierces of one face at one point into one ring
(`vtxfac::ring_groups`). Each decision is a pending row
(`crates/topo/src/boolean/reduce.rs` `PendingRow`), emitted as a
`VertexFusion` coincidence where a record citing it survives into
the result, or where the result keeps the decided vertex as copies on
one point key, a pinch the op made (`ops::cite_rows`, `ops::carry`). `PendingRow`'s rule: "only one that does
makes pieces of the result touch, and only that one is a coincidence
(D1: an ON verdict that only places topology is not)".

Where the decision glues two operand vertices into **one** result
vertex (a zip fusing a vertex pair, or one ring's copies fused into
one vertex), the two cells become one. No record survives, the row is
dropped, and the `unproven-coincidence` lint never sees it. D10's
coincidence paragraph says every coincidence the kernel infers from
values, "a boolean's glue" included, "is recorded at the one door where
structure is decided", and the lint checks it. So a value glue into one
vertex may owe a row that today's rule drops.

## Measured (PR 4728's review r2, release)

- Of 791 value-decided ring ties, 761 glue the two operand vertices into
  one result vertex. Their rows are dropped. The 30 that keep the cones
  distinct emit a `VertexFusion` row with a record citing it.
- Among the dropped are the ties in `topo`
  `three_solids_on_one_line::a_prism_moved_off_the_line_builds_or_escalates_typed_in_every_member_order`
  and `a_prism_tilted_within_the_zero_builds_as_upright`. There the
  points are not bitwise equal, which is the in-Zero offset the lint
  exists to report.
- The pinch weld that PR 4728 retired fused coincident pierces
  unrecorded as well.

## The question

Does a value decision that glues two cells into one result cell owe a
coincidence row, though no record of the result names the pair? If so,
`PendingRow`'s rule (and D1's "an ON verdict that only places topology
is not a coincidence") needs restating at the coincidence door, and
the boolean emits the row with no citing record. This is a reading of
D10 against D1 for designers, and not a lane's to decide.

## Seams

`crates/topo/src/boolean/reduce.rs` (CLEAVE, HONE, ROOTS ground) and
`ops::cite_rows` (FUSE, CLEAVE).

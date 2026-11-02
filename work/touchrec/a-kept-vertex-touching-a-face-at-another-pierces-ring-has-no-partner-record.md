---
id: a-kept-vertex-touching-a-face-at-another-pierces-ring-has-no-partner-record
kind: issue
title: A kept vertex resting on a face at a point another kept vertex pierces is recorded on the face, not paired with that pierce's ring vertex
status: open
opened: 2026-11-01
priority: P2
cost: M
refs: [a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece]
---


Found by the review of PR 6185 (`cleave/fate-touch-end-at-a-division-vertex`),
which dumped the records at the touching end it places; the same on
main `336465c32b` and on the branch.

## What

`x` is the cube `[0, 1]³` ∪ the box `[1, 2]² × [0.5, 1.5]`, touching
along `x = y = 1, z ∈ [0.5, 1]`, the union's records carried; `y` is
`[−0.5, 2.5]² × [1, 1.5]`; the op is `x ∖ y`, at rest, the cube first
(PR 6185's
`a_keel_on_the_floor_subtracts_as_a_slit::a_fate_edge_touching_a_corner_of_its_faces_division_builds_at_rest`).
At `(1, 1, 1)`, `x` holds the cube's corner `7v1` and the box's vertex
`20v1` there, two vertices on one point (a pinch). The reduction's
records there:

- `7v1` has no vertex–vertex row and none of `y` on it; its only row is
  `VfContact { vertex: 7v1, face: 2v1 }`, on `y`'s bottom face;
- `20v1` pierces that face, and its pierce ring holds the ring vertex
  `15v1`, with its null-edge copy `16v1`, at the same point;
- `7v1` is in no ring's `pierces`, and no row pairs it with `15v1`.

With three lumps (the cube and the prisms over `(1, 1), (2, 1), (2, 1.8)`
and `(1, 1), (1.8, 2), (1, 2)`, `z ∈ [0.5, 1.5]`; `y` below, `x ∩ y`)
the same: the cube's corner rests on the face (`VfContact`), the
prisms' vertices there pierce it, and their ring `21v1` (copies `22v1`,
`23v1`) names neither.

So surgery was done at the point, by the pinch-mate's pierce, but the
record that would name it as the touching vertex's partner is missing.
With it, `fate::end_at` would return `End::At([15v1, 16v1])`, and the
fate cut would end at the ring vertex through its records. PR 6185
reads the point instead (`fate::cut`'s touch placement through
`fate::lies`), which reaches the same vertices.

## Where

The vertex-on-face pass, `boolean/mod.rs` (`vtxfac::classify_vertex_on_face`
per row of `contacts.a_on_b` / `b_on_a`, `mod.rs:5961`): a touching row
returns no ring plan, and the ring grouping
(`vtxfac::ring_groups(&only, band)`, `mod.rs:6124`) groups the pierce
plans only, so a touching vertex on the point of another vertex's
pierce is joined to that ring by nothing.

## Owed

Decide whether the touching vertex should be paired with the ring vertex
(a vertex–vertex row, or a member of the ring's `pierces`), and whether
any other consumer of the vertex–face and vertex–vertex rows or of
`pierce_rings` reads this state: tier 3′'s census
(`validate_pseudomanifold`), the records carried into the next op
(`ContactRecords::carried`), `recl`'s vertex–vertex classification,
`fate::end_at`, `join`, `sectors` and `discard`. Not fixed in PR 6185.

2026-11-01: claimed from `work/cleave/` by `git mv` once PR 6185 merged, per the 2026-11-01 split note in `work/cleave/plan.md` (TOUCHREC holds the records of a result's own touches); id and body unchanged. (TOUCHREC orchestrator)

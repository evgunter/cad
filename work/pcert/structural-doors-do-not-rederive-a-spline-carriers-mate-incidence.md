---
id: structural-doors-do-not-rederive-a-spline-carriers-mate-incidence
kind: issue
title: The structural at-rest doors do not re-derive a spline carrier's incidence with its mate
status: open
opened: 2026-10-12
priority: P3
---


## What

`validate_geometric_structural` and `validate_pseudomanifold_structural`
(and their certificate forms, at f64, Interval and Sym) skip an edge
whose certificate needs the NURBS lane at check 2
(`crates/topo/src/validate.rs`, the `StructuralPhase::NoLane` arm and
the `NurbsLaneNotSupplied` skip). Until PR 4900 their check 8 still
re-derived part of that edge's certificate through the fitted row:
the row's check 4 ran `certify_rung3` / `certify_branch` against the
mate surface, so it covered limbs 1 and 2 (the carrier on the mate,
e.g. on the cap plane) and the uniqueness tube.

Since PR 4900 the row's check 4 bounds the image against its own
chart alone (C4). So at these doors nothing re-derives a spline
carrier's incidence with its mate. A plane × NURBS spline carrier
that bows off its plane between vertices passes
`validate_*_structural`. The certified doors (`validate_geometric`,
`validate_pseudomanifold`) still catch it at check 2, through
`plane_nurbs_limbs`.

The doors document that check 2 makes no claim about such an edge,
and since PR 4900 their docs say check 8 does not make it either. No
production caller outside topo reads these doors. Adding the check
would mean handing them a NURBS lane, which H5 ruling 3 rules out
(orchestrator, 2026-10-12). See also
`structural-check2-skip-reports-no-count`.

## Open

Whether the doors should state a count of the edges they made no
claim about (the sibling item), or whether the doc statement is the
whole answer.

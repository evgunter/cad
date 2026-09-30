---
id: boolean-coincidence-route-still-holds-join-and-self-check-decisions
kind: issue
title: topo: join matching, strut order, germ-line self-checks and the ray lane still end in the coincidence menu, which no face-pair declaration settles
status: open
opened: 2026-09-30
---


(TOPO, the per-site walk and §5 second pass of PR 3513.)

## What

PR 3513 gave the lever-arm gates, the section radius guards and the
wall and torus root lanes their own `BooleanDecision` variants. The walk
found more wrap sites whose decision no face-pair declaration names,
still routed to `BooleanDecision::Coincidence`, so their refusals offer
"declare the coincidence":

- the join's matching (`crates/topo/src/boolean/join.rs`: `find_match`
  near :607, `germs_face_each_other` near :1182, `loose_partners` near
  :1233): `bool_join_chord`, `bool_join_nearest`, `bool_join_facing`,
  `bool_join_arc_facing`. Where two section germs lie and which is
  nearer; the chrome triage's example: a face pair says nothing about
  where a crossing lands.
- `bool_ring_run_winding` (`join.rs` near :1533): a region's mean
  width, 2A/P.
- `bool_strut_order` (`crates/topo/src/boolean/insert.rs` near :223):
  which of two germs comes first around a strut.
- the germ-line re-reads, `bool_germ_line` at `insert.rs` near :353 and
  `crates/topo/src/boolean/vtxfac.rs` near :693: each re-reads a
  margin a caller read definite, and its definite sibling is
  `ClassificationInvariant`, so the in-band arm tells another story
  than its sibling (D4 ¶1 (iv)); `BooleanDecision::ArcSpan` is the
  precedent (a defect ending).
- `bool_sphere_recut_align` (`crates/topo/src/boolean/ops.rs` near
  :2471): a recut axis's alignment.
- the seam re-description's wedge reading (`ops.rs` near :1688,
  `Coincide::SeamWedge`): the pass runs after the zip and reads no
  declaration.
- the circle × torus root lane (`crates/topo/src/boolean/reduce.rs`
  near :1912, `circle_torus::circle_torus_roots`): its coaxial rows ask
  whether the circle lies on the torus (a coincidence the Rest
  carrier rung can settle), its quartic rows how many times it crosses
  (no declaration names that). The escalation carries no rung, so the
  wrap site cannot tell them apart; `solid_contain::WallRootFault` is
  the shape.
- two whole renders beside `Escalated`: `BooleanError::Join`
  (`crate::chord_join::UnderBoolean`, which adds the declaration back
  at the Boolean) and `PointInSolidError`'s escalated, ray-exhausted
  and loop arms (`crates/topo/src/boolean/solid_contain.rs`, its
  `Display`), which end in `COINCIDENCE_RECOURSE`. Whether a point lies
  inside a solid is not a face pair's question, and the ray lane's
  `line_wall_roots` rung is dropped at `cast_ray`.

## Repair shape

As PR 3513 did for its three families: a `BooleanDecision` variant per
decision, its subject, and an ending from its pass set (the join's
nearest and order decisions pass on any definite sign, the germ-line
re-reads are self-checks ending as defects). Where one wrap site
receives several decisions, the raiser carries which.

## Since (PR 3513's fix pass)

`ops`'s seam re-description reading (`Coincide::SeamWedge`) now
escalates as `BooleanDecision::Proximity(Coincide::SeamWedge)`, since
`BooleanDecision::of_lever` routes every lever-armed reading there: no
declaration is offered. Whether it is a self-check decision of its own
is still this row's question.

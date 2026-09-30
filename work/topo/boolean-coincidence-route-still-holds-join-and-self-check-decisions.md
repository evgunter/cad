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

## Since (PR 3513's second fix pass)

Each site listed above now states `DeclarationRead::Moot` (it reads no
declaration ahead of its question), so none offers a declaration; each
ends in `Coincide::unsettled`'s generic lever and the tolerance, which
is this row's remaining question (their own decisions and pass sets,
the self-checks as defects). The seam re-description's wedge reading
has a lever of its own (`Coincide::SeamWedge`: "move the geometry so
the faces at that seam meet either clearly creased or clearly
smooth"), and the circle × torus lane escalates as
`BooleanDecision::TorusRoots`, so its coaxial rows (whether the circle
lies on the torus) share the count's words; the carrier rung reading
a `Rest` declaration ahead of that lane only moves an in-band pose to
the covered endpoint sides
(`reduce::declaration_order_rows::the_circle_torus_lane_escalates_as_the_torus_roots_and_no_declaration_settles_it`).


## Since (PR 3513's third and fourth fix passes)

The "Since" sections above name types this head no longer has
(`Coincide::unsettled`, `Coincide::SeamWedge`,
`BooleanDecision::Proximity`); what holds now:

- **The self-checks are defects.** The third pass gave the kernel's
  own re-reads a closed decision, `BooleanDecision::SelfCheck(SelfCheck)`
  (`GermLine`, `Normals`, `ArcFacing`, `RingWinding`, `CarrierLadder`,
  `crates/topo/src/boolean/refusal_routes.rs`), which ends as a defect
  and never names the tolerance. The fourth pass pinned each at its
  site: `join::self_check_rows` (`ArcFacing`, `RingWinding`) and
  `boolean::tests::a_contradiction_at_the_undeclared_screen_is_the_kernels_own_check`
  (`CarrierLadder`, an arm the undeclared screen cannot reach). The
  pierce germ line at `vtxfac::pierce_germ_dir` is no longer one: it
  reads its margin at another arm than the transition reading that sent
  it there, so its in-band arm is the corners' overlap undecided
  (`Coincide::Sectors`, executed: `offer_rows`' `pierce_germ_line_in_band`).
- **The seam** escalates as `BooleanDecision::SeamWedge` and its arm
  gate as `LeverArm(Seam)`, each with its own lever and pass set
  (executed: `offer_rows`' `seam_*`). **The recut axis** is
  `BooleanDecision::Sphere(SphereQuestion::RecutAlign)` (executed at
  its site: `sphere_barely_leaning`).
- **The join's matching and the strut order** read no declaration
  (`DeclarationRead::Moot`), so none offers one, and end from their own
  pass sets (`Coincide::Join`, `Coincide::Sectors`); every tolerance
  they offer is executed (`offer_rows`' `germs_nearly_*`,
  `direction_just_outside_a_sector`).
- **The circle × torus lane** is `BooleanDecision::ArcTorusRoots`, on
  its lever alone until it carries its rung
  (`circle-torus-lane-escalates-without-its-rung`).

What remains of this row is its last bullet: `BooleanError::Join`
(`chord_join::UnderBoolean`) and `PointInSolidError`'s escalated,
ray-exhausted and loop arms (`solid_contain.rs`, its `Display`) still
end in `COINCIDENCE_RECOURSE`, which offers the declaration and an
unvalued tolerance.

---
id: boolean-declared-doors-still-offer-the-declare-menu
kind: issue
title: topo: the Boolean's escalations at doors whose pair is already declared still offer 'declare the coincidence'
status: open
opened: 2026-09-30
---


(TOPO, the per-site walk of PR 3513.)

## What

PR 3513 makes every coincidence wrap site name its coincidence
(`BooleanError::coincidence(Coincide, diag)`), and each renders the
coincidence sentence, which offers "declare the coincidence". D4 ¶1 (i)
offers the declaration only at a door whose declaration would change
the verdict. At these sites the pair is already declared, so no
declaration changes it:

- `boolean::insert` (`crates/topo/src/boolean/insert.rs`, the
  declared-`Tangent` germ direction, near :300): `tangent_locus`'s
  escalation, `Coincide::TangentLocus`.
- `boolean::sectors::tangent_lump` and `tangent_relative_side`
  (`crates/topo/src/boolean/sectors.rs`, near :568 and :656): the
  tangent locus and the second-order side of a declared-`Tangent`
  sector pair, `Coincide::TangentLocus` and `Coincide::SectorSide`.
- the declared-`Tangent` verification in `boolean/mod.rs` (near :2639,
  :2669 and :2773): the tangent locus, the shared rim
  (`rim_wedge::shared_rim`) and `contact_pair_verdict`'s escalation,
  `Coincide::TangentLocus` and `Coincide::Contact`.
- `boolean::reduce::curved_face_arm`'s covered circle
  (`crates/topo/src/boolean/reduce.rs`, near :1365): an endpoint's side
  on a pair the declared-cover rung already read, `Coincide::VertexOnFace`.
- the declared-REST seam walk (`crates/topo/src/boolean/rest.rs`, the
  `escalate` closure near :428): `bool_join_chord`, `bool_join_facing`
  and `bool_join_nearest`, `Coincide::Join`.

`BooleanError::RestZipUnsupported` was the definite arm of the same
door and PR 3513 gave it the geometry lever alone; these are its
in-band neighbours.

## Repair shape

Carry the door, as `PlaneDoor` does for the plane rungs: the same
decision at a declared door ends in its own lever (the tangency's, the
contact's), with the tolerance an in-band margin gives where the
decision passes on a nonzero sign, and no declaration.
`BooleanDecision::DeclaredParallel` is the precedent.

## Since (PR 3513's fix pass)

The fix pass added the decision these sites land on:
`BooleanDecision::Proximity(Coincide)`, the coincidence asked where no
declaration is read ahead of it, which ends in the geometry and the
tolerance the gap gives, and no declaration. `tangent_relative_side`'s
reading already reaches it (`BooleanDecision::of_lever` routes every
lever-armed reading there, and the pair is declared). The rest of the
list above is unchanged: each is a one-line move from
`BooleanError::coincidence` to `BooleanError::proximity` plus a row on
a declared raise.

## Since (PR 3513's second fix pass)

Every door listed above now states what it read
(`BooleanDecision::Coincidence(Coincide, DeclarationRead::Spent(class))`,
`crates/topo/src/boolean/refusal_routes.rs`), so none offers "declare
the coincidence". One more declared door was found and joined them:
the plane ladder run on a declared-`Tangent` pair
(`verify_tangent_declaration`'s conformal screen in `boolean/mod.rs`,
and `vtxfac`/`recl` through `PlaneDoor::of(class)`), whose in-band
parallelism offered the declaration to a pair already declared (the
review's union poses G2, G5 and G6, now
`reduce::declaration_order_rows::a_declared_tangent_pairs_plane_rung_offers_no_declaration`).

What remains of this row is its repair shape's second half: at a
declared door each decision ends in the coincidence's generic lever
(`Coincide::unsettled`: "move the parts so they clearly meet or clearly
stand apart there", with the tolerance), where the tangency's locus,
the contact's witness and the REST seam walk's matching may each want a
lever of their own. `Coincide::TangentSide` and `Coincide::Planes`
have theirs.


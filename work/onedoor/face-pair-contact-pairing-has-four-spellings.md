---
id: face-pair-contact-pairing-has-four-spellings
kind: issue
title: Whether two faces at a point are coplanar with their material facing is asked four ways: census ee_cross_backed, at_rest::opposed, link::pairing, contact_verify::contact_tangent_opposed
status: open
opened: 2026-11-03
priority: P1
cost: M
design: true
---


Filed by the ONEDOOR orchestrator from the wedge fork's designers (2026-11-03). Both found this independently while weighing `the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again`.

Four sites ask the same face-pair question: do two faces at a point have coincident tangent planes, and does their material face each other? None of them involves an edge or an edge direction.

- `crates/topo/src/census.rs` `ee_cross_backed`. It runs `classify_dihedral` Smooth on two declared planar faces at an edge–edge crossing point, then `folded_lever_arm` and `classify_material_pairing` at the same point.
- `crates/editor-core/src/checks/at_rest.rs` `opposed`, through `classify_material_pairing_as`.
- `crates/topo/src/link.rs` `pairing`, through `classify_material_pairing_as`. It asserts coplanarity in prose rather than reading it.
- `crates/topo/src/boolean/contact_verify.rs` `contact_tangent_opposed`. It has its own margin and its own predicate; `geom-brep/src/dihedral.rs` cites issue 1439 for it.

The wedge design gives `classify_dihedral` a `Tangency` token with `pairing_as(name, senses)`. That retires the free `classify_material_pairing`. The open question is whether these four read `classify_dihedral` and ask the token, or whether a reader whose coplanarity was proved by another route (`link`, `at_rest`) mints the token from that proof. The unit that builds the wedge tokens has to answer it, so this row follows that unit.

## Unparked (ONEDOOR, 2026-11-03)

Wedge stage 1 landed (PR 6443). `classify_dihedral` now returns a `Tangency` token with `pairing`, and the census's `ee_cross_backed` already asks the pairing of it.

The stage-1 lane did not move `link::pairing` and `at_rest::opposed` onto the token. Neither reads a class first: one proves coplanarity from a piece of one face lying on the other's plane, the other from the constant-pairing families. Reading the class through `classify_dihedral` would add a `dihedral_arm` and a `dihedral_wedge` decision at every touch check, and the arm gate could refuse where the pairing alone decides today. The dead `pairing_as` was removed.

So the question this row asks is now concrete. Either a reader whose coplanarity is proved some other way mints the `Tangency` token from that proof, through a constructor that takes the proof, or the free `classify_material_pairing_as` stays as the one named door for proved-coplanar pairs. `contact_verify`'s hand-rolled sibling stays in scope.

---
id: face-pair-contact-pairing-has-four-spellings
kind: unit
title: Whether two faces at a point are coplanar with their material facing is asked four ways: census ee_cross_backed, at_rest::opposed, link::pairing, contact_verify::contact_tangent_opposed
status: parked
blocked_on: [the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again]
branch: onedoor/face-pair
opened: 2026-11-03
priority: P1
cost: M
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

## Designed (ONEDOOR, 2026-11-12)

Weighed by two designers (reports `fork-facepair-{A,B}.md`), converged in two rounds. Nothing ratified changes, so it is a unit, not an `[ev]` fork. The four sites do not ask one question; they share one margin, the sign of the cosine between two outward normals over a lever. Each reader reads the fact its own question is about:

- **The census `ee_cross_backed`** reads the per-pair carrier relation (`carrier_pair_relation`: `SameOpposite` / `SameOriented` / `Distinct`), which `pair_contact` in the same file already reads for the same declared pairs. It no longer re-decides coplanarity and pairing at each crossing over the shorter edge's chord.
- **The link `pairing`** reads the side off its own sectors' outward normals (`Sector::outward`) as a point-against-plane reading in the link's own metric, accounting for each sector's apex offset from `p` as the link's other readings do. It no longer re-reads the carriers' normals at `p` over the folded arm. Witness: a cylinder face of radius 2ε lying on a plate refuses in band today, although the sectors plainly face each other.
- **`at_rest::opposed`** reads the facing that the `Gap` reading decides. The hand-written `constant_pairing` pose test in editor-core goes. One geom-brep pose reading answers the pose and the facing, and both `measure::gap` and `at_rest` read it.
- **`contact_verify` row 1 and certification's `tangent_gap_orientation`** share one unsensed door, `geom_brep::normal_alignment(s1, s2, p, lever, band) -> Same | Reversed | Perpendicular`. It returns `Perpendicular` only past a lever decided positive. The material pairing is that alignment turned by the sense bits.
- **`Tangency::pairing`** is sugar over `normal_alignment` at the token's arm.
- **The free `classify_material_pairing_as` retires.** A pairing is read only behind the reading that established the faces are parallel there.

To execute while building:
- `Sector::outward` agrees in sign with the carrier's outward gradient wherever the link reaches;
- `measure::gap` takes the geom-brep pose without moving gap values;
- the census suites stay green on the carrier relation.

Parked until wedge stage 2 lands (`onedoor/wedge-fold-2`), since both edit `geom-brep/src/dihedral.rs` and the census. Review tier: dual. It changes four readers across three crates and retires a public door.

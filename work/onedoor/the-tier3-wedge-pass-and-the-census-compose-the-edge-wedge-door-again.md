---
id: the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again
kind: issue
title: Tier 3's wedge pass and the census each compose the dihedral class with the material pairing, the question geom_brep::classify_edge_wedge now answers in one door
status: open
opened: 2026-11-02
---

Filed by the REFS lane (`work/refs/select-all-edges-picks-edges-a-fillet-refuses.md`), from its sweep for readers of an edge's dihedral.

`geom_brep::classify_edge_wedge` (`crates/geom-brep/src/dihedral.rs`) is now the one door that answers what an edge's dihedral is with its material side: convex, concave, smooth (sign decided Zero, outward normals aligned: wedge π) or knife (Zero, normals opposed: wedge 0 or 2π). The blend's battery reads it (`battery::wedge_at`). Three readers in `topo` still compose the same question from its parts:

- **tier 3's check 4** (`validate.rs`, the first-order pass over `geom_brep::classify_dihedral` at each of the edge's interior stations, then `MaterialStations::before_decision` calling `classify_material_pairing` inside the second-order walk), which yields `ContactMark` and `MaterialArmOutcome`;
- **the census's contact read** (`census.rs`, `ee_cross_backed`: `classify_dihedral` Smooth, then `classify_material_pairing` at the same point and arm);
- **the split's finish** (`crates/topo/src/splitting/finish.rs`, `describe_section_boundary`, ~1834-1873): `classify_dihedral` at the drafted witness at `wedge_reach`, then on Smooth against a curved wall `classify_material_pairing` at the same witness with `folded_lever_arm`, refusing `SplitFinishError::KnifeEdge` on Opposed. Its material half is the door's population exactly: one point, analytic faces, the same folded arm.

**The split's finish was not routed through the door in PR 6344, and the drift risk is stated here.** The first-order halves ask different questions. The finish decides the UNSIGNED `sin θ · arm` under `dihedral_wedge`, behind `dihedral_arm`'s gate and its `LeverEscalation`, and needs only transverse vs smooth (it describes a transverse edge either way, whatever its sign). The door decides the SIGNED `((n₊ × n₋)·τ̂)·arm`, under its caller's name. On the edge both normals are perpendicular to `τ̂`, so the two magnitudes agree; at a witness off the edge by up to the tolerance, `|(n₊ × n₋)·τ̂| ≤ |n₊ × n₋|`, so near the band's escalation edge the signed read can land in band (escalate) where the unsigned one decides transverse. Routing it would also move its K population and its escalation type (`DescribeEscalated` carries a `LeverEscalation`). So the two halves can drift: a change to the door's smooth/knife read (its arm, its pairing quote at the wedge) does not reach the split, and vice versa.

**Two vocabularies for one fact**: `topo::SplitFinishError::KnifeEdge` (with `topo::KnifeEdge { wall, at }`) and `sweep::blend::BlendError::KnifeEdge { edge, margin }` both say "a decided-zero dihedral with the material opposed", each in its own words and payload. Converging the compositions is the natural moment to give them one type.

Why the REFS unit did not move them: check 4 asks the UNSIGNED tangent-plane class (`sin θ · arm`, predicate `dihedral_wedge`, its collapsed-arm gate `dihedral_arm` and its `LeverEscalation` rungs) at every interior station, reads spline faces through `classify_seam`, and descends to the jet for the cusp/slit end; the door reads the SIGNED margin `((n₊ × n₋)·τ̂)·arm` at one point of analytic faces. Moving check 4 onto the door changes its predicate population, its escalation types and its spline arm, which is this program's ground, not a blend unit's.

What converging would take: the door taking the station's walk direction and a spline face (as `classify_seam` does), or check 4 and the census reading the door's material half (`Smooth`/`Knife`) where they now pair `DihedralClass::Smooth` with `classify_material_pairing` by hand. Either way the material pairing stays the one function it already is; what is duplicated is the composition.

---
id: the-tier3-wedge-pass-and-the-census-compose-the-edge-wedge-door-again
kind: issue
title: Tier 3's wedge pass and the census each compose the dihedral class with the material pairing, the question geom_brep::classify_edge_wedge now answers in one door
status: dispatched
opened: 2026-11-02
priority: P1
cost: H
branch: onedoor/wedge-fold
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

## Re-read (ONEDOOR, 2026-11-03)

All three composition sites are confirmed at `d1ac6881`. There is a fourth, which the row does not list: `boolean/rim_wedge.rs`. It spells its own first-order `classify_dihedral` loop, then runs check 4's `MaterialStations` inside `second_order_walk`. A related half-composer is `splitting/rules.rs` `edge_wedge`: `classify_dihedral`, then the convex/concave sign via `enters_material`, with no pairing. The census's population is two planar faces at an edge–edge crossing point, not one edge's own pair. Banded P1 (one logic composed at several sites), priced M, with `design` set because the signed and unsigned first-order reads are a real choice.

## Design (two designers, converged after three rounds, 2026-11-03)

Weighed under the design-fork protocol. The designers converged, and nothing ratified changes, so the design is not put to Ev as a fork (log, 2026-11-03). The final state:

- **One class decision.** `geom_brep::classify_dihedral(s₁, s₂, p, reach, band) -> Result<Dihedral<T>, WedgeEscalation>`, where `Dihedral = Corner(Corner<T>) | Tangent(Tangency<T>)`.
  - The class is the UNSIGNED `sin θ · arm` under `dihedral_wedge`, behind `dihedral_arm`, as today. It reads no face sense and no edge direction.
  - Each token holds the two unit normals and the folded arm.
  - Certification and the face-pair readers ask it too.
- **The sign, on the corner token.** `Corner::convexity(&Station, senses) -> Result<Convexity, WedgeEscalation>` decides `((n₊×n₋)·τ̂)·arm` by `decide_nonzero` under ONE predicate name for every reader (`dihedral_sign`). A wrong τ̂ can then only escalate, never return a wrong class. `Station<T> { p, tau }` is minted only by a carrier at a parameter. `Convexity` moves from the blend to `geom_brep`.
- **The pairing, on the tangency token.** `Tangency::pairing(senses)` is today's `material_wedge_side`, with its re-quote at the wedge. `Tangency::pairing_as(name, senses)` serves the face-pair readers. The free `classify_material_pairing` retires, so a pairing can never be asked of a corner.
- **One whole-edge fold in `geom_brep`**, beside the second-order walk: `edge_wedge_over(faces, stations, reach, band)`.
  - It reads the class at every station and the convexity at every corner station. A sign flip, or a corner/tangent mix between adjacent stations, gives `Mixed(MixedStations)`.
  - When every station is tangent, it runs the second-order walk over the `Tangency` tokens, reading side and end at each station.
  - The verdict is one of `Crease(Convexity) | Seam { bend } | Cusp | Slit | Lamina | Split { rung } | Mixed | InBand(WedgeEscalation)`.
  - It absorbs check 4's loop (`validate.rs`), `MaterialStations` and `material_arm_outcome`, `boolean/rim_wedge.rs`'s loop, and `must_carry_over_edge`'s first-order pass.
  - Its readers: check 4, rim routing, the constructors (extrude, revolve, `edge_join`) and the split finish, whose knife refusal then agrees with tier 3 station by station.
  - The point readers, the blend and `splitting/rules.rs` `edge_wedge`, call `classify_dihedral` and then the token methods.
- **A crate-visibility fence.** `second_order_walk`, `StationHook`, `material_kappa_rel` and the fold's parts are `pub(crate)`. `interior_stations` stays public, because check 5 shares the schedule.
- **One rung vocabulary.** `WedgeRung { Arm, Wedge, Sign, Pairing, Bend, End }` with one `WedgeEscalation { rung, diag, refused }`, carrying the arm's refused verdict whole.
  - It retires `EdgeWedgeEscalation`, `MustCarryEscalation`, `DihedralReading`, `MaterialStop` and topo's `WedgeCheck` (or maps it one to one).
  - `LeverRung`/`LeverEscalation` stay: `enters_material` raises them in non-wedge readings. `From<LeverEscalation>` builds the `Arm`/`Wedge` rungs.
- **What retires:** `classify_edge_wedge`, `EdgeWedge`, `EdgeWedgeEscalation`, `PositiveArm`.
- **What stays:** the two `KnifeEdge` refusals. They carry different facts and recourses, over one reading, and each carries the reading's margin.
- **K names are per question, not per caller** (orchestrator, following #652: one computation of one quantity is one population). The blend's `fillet3_convexity_sign` and `fillet3_chain_arm` populations join `dihedral_sign` and `dihedral_arm`, and `BlendDecision` stays the blend's wording key.

**Off this row.** The census's `ee_cross_backed` is a face-pair contact-pairing reader, not an edge-wedge reader; it moves to `face-pair-contact-pairing-has-four-spellings`.

**Defects the unit closes**, found by the designers:
- `rules::edge_wedge` levers by `departure_arm_lever(along)`, where tier 3 levers by `wedge_reach`;
- at a cone apex, `rules::edge_wedge` reads class and sign at two different points;
- `rim_wedge.rs` `classify_shared_rim` builds its own INVALID `Indeterminate` named `dihedral_wedge` for mixed and spline stations;
- the finish reads the knife pairing at one witness, where tier 3 reads every station;
- `boolean/edge_join.rs` reads a pair at a vertex with a neighbouring edge's tangent.

Also `work/honeword/split-dihedral-readers-drop-the-arm-rung.md` closes with the one escalation type. Spline stations stay deferred (`work/splice/a-smooth-seam-between-spline-walls-has-no-second-order-reading.md`). Check 4's by-kind spline exemption stays a reader's policy.

**Order:**
1. The tokens, the rung type and the fold in `geom_brep`, with check 4 and `rim_wedge` as its first readers. Dual review.
2. The constructors and the split.
3. The blend's predicate populations.

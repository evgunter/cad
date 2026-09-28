---
id: every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one
kind: issue
title: Every op that can mint a wedge-0/2π edge must trace it to a declared input or refuse typed at its own door; the loft is the open hole
status: review
branch: gather/wedge-end-door-audit
opened: 2026-09-28
priority: P2
cost: M
refs: [product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares, loft-between-opposite-turning-joints-reverses-a-seam-between-stations, declared-cusps-second-order-wedge-arm, self-overlapping-spines-build-and-validate]
---


Filed with Ev's ruling on PR 3317 (2026-09-28): a cusp is legal at
rest iff jet-determinate, so tier 3 no longer catches an op that mints
a knife edge nobody asked for. D1 hands that refusal to the ops. The
audit below places every op's door. The one open hole is the loft
(filed on carve).

Owed: sweep every body-producing op for a path to a definite wedge
end, list each op with its door (or add the refusal), and pin one row
per door. Future shell, draft and offset work inherits the obligation.
Designer D's optional companion: report `MaterialWedge::Cusp | Slit`
through the marks channel as a diagnostic (never a gate). The importer
option it paired with is moot: see STEP import below.

## Audited doors (branch `gather/wedge-end-door-audit`)

"Probed" means the shape was built on the branch and the verdict read
off `topo::validate_geometric` and check 4's `Tangent` mark.
"Reading" means the argument comes from the code alone. The new rows
are in `crates/sweep/tests/wedge_end_doors.rs`. The split and boolean
doors belong to `topo`, but their rows need a real curved body, which
only `sweep` builds.

| Op | Door, or why it cannot mint | Evidence |
|---|---|---|
| **Profile** (feeds extrude, revolve, loft, path sweep) | `PathError::JunctionCusp` (the path builder's near-reverse turn); `ProfileError::UndeclaredTangency` (`profile::validate`'s `judge_joints`, a raw zero-turn joint nobody declared); `ProfileError::TangentialContact` (loops or segments touching tangentially). Declared form: `.cusp()`, or the joint in `tangent_joints`. | Pinned already: `profile/tests/declared_tangency.rs` `the_cusp_verb_declares_a_joint_the_data_gate_already_accepts`, `path_property.rs` `turn_pi_refuses_as_cusp_naming_the_declaration_door`, `rejections.rs` `internally_tangent_hole_is_a_tangential_contact`. |
| **Extrude** (`sweep::extrude`) | Cannot mint past the profile. A strut's wedge is the profile joint's interior angle. A cap meets each wall at an angle strictly inside (0, π): π/2 for `Extrusion::Distance`, and oblique for `Extrusion::Vector`, but never an end. | Reading. Declared form passes: `sweep/tests/a_swept_cusp_is_legal_at_rest.rs`. |
| **Revolve**, full and partial (`sweep::revolve`) | Cannot mint past the profile. A rim's wedge is the joint's meridional angle, the caps meet the walls at π/2, and a partial revolve's on-axis cap–cap edge sits at the window angle in (0, 2π). | Reading. Declared form passes: the same file. |
| **Tube, hollow tube** (`sweep::tube_along_arc{,_hollow}`) | Cannot mint. A circular section has no joint, and the ring-torus convention (`TubeError::Revolve`) refuses horn and spindle tori. | Reading. |
| **Loft** (`sweep::loft_body`) | **Hole, needs design.** Each section passes the profile door on its own, but a joint that turns opposite ways in two sections folds that seam through wedge 0 between stations. The witness is a self-overlap (the mid-station section is not simple), not a jet-determinate cusp. No witness that stays simple at every station was found. Tier 3 exempts the NURBS seam by kind. | Probed: builds, `validate_geometric` Ok. Filed as `work/carve/loft-between-opposite-turning-joints-reverses-a-seam-between-stations.md`, which may belong with the self-overlap class. |
| **Path sweep** (`sweep::sweep_body`) | Door is the profile. Every station is the same profile, so no joint turns differently between stations. (Self-overlap is a separate class, `work/carve/self-overlapping-spines-build-and-validate.md`.) | Reading. |
| **Boolean, curved** (`topo::boolean_op_with`; `union`, `subtract`, `intersect` and their `_with` forms) | Refuses typed today, but not at a wedge-specific door: the crossing frontier (`CurvedPierceUnsupported`, `CurvedBooleanUnsupported`) refuses first. The declared route, and the undeclared-tangency refusal that must land with it, is TANG's `declared-cusps-second-order-wedge-arm` item 3. **No declared form passes.** A declared external kiss verifies and then hits `CurvedPierceUnsupported`, and an internal kiss has no declaration at all (`ContactContradicted`, from C4's opposed-senses test). Evidence added to TANG's row. | Pinned: `a_boolean_that_would_kiss_a_curved_face_refuses_typed_at_the_op` (internal cylinder kiss subtract, external kiss union, plane cutter tangent to a hole wall). |
| **Boolean, shared rim** (`boolean::rim_wedge`) | `BooleanError::RimCuspArmUnbuilt`: a wedge-0/2π rim is the declared-cusp family, defined and unbuilt. | Pinned already: `sweep/tests/mate7a_torus_rest.rs` `a_kissing_torus_rim_routes_to_the_unbuilt_cusp_family`. |
| **Boolean, planar** | Cannot mint a legal wedge end. Two planes have κ_rel = 0, so a plane–plane wedge end fails jet-determinacy (tier 3's `LaminaWedge`), and coplanar contact is the `Rest` class (`UndeclaredCoincidence`). | Reading. |
| **Union node** (editor `wire_union`) | The boolean's door: pairwise contact judgement, then the union fold. | Reading. |
| **Placed union** (editor `wire_placed_union`) | Cannot mint. `topo::Separation::certify` has to certify the placements apart before anything is built (`NodeErrorKind::PlacementsUncertified`), and the copies are grafted disjoint with nothing fused. | Pinned already: `editor-core/tests/lib_placedunion.rs` `touching_boxes_over_disjoint_solids_refuse_the_same_way`. |
| **Pattern** (editor `wire_pattern`) | Cannot mint an edge. The result is `Instances` with nothing fused. Two instances touching is a contact, which the product gate's tier-3′ census judges. | Reading. |
| **Transform, mate, part instance** (`topo::transform_rigid`) | Cannot mint. The door admits only det = +1 rigid maps, which preserve every dihedral. | Reading. |
| **Split** (`topo::split`) | **Hole, closed here.** A plane tangent to a hole wall along a vertex ruling left the hole-side piece a doubled cusp, and tier 3 passed it. `splitting::finish::describe_section_boundary`'s smooth arm now reads the material pairing of a curved wall smooth against the section (`geom_brep::classify_material_pairing`, tier 3's own reading). Opposed materials, a wedge end, refuse `SplitFinishError::SectionCusp`. Aligned materials, a π seam, cut. `split` surfaces the pinch rerun's `SectionCusp` over the direct run's `DegenerateSection`; it can only reach that past the mirror's join, so a both-sided pinch never does. The same tangency along an arc's interior already refused typed at the join (`DegenerateSection`, `SectionInvariant`). | Pinned: `a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint` (red first). Counter-rows: `a_split_through_the_hole_or_across_a_declared_cusp_still_cuts`, and `a_split_tangent_to_a_rounded_shoulder_cuts_at_a_seam`, which a kind-based guard fails. |
| **Fillet** (`sweep::blend::build::fillet_edges`) | Cannot mint. A spring edge is a π seam by construction (the ball is tangent to its support), and an end is a transverse cap or a refused run-out. | Reading. |
| **Chamfer** (`sweep::blend::build::chamfer_edges`) | Cannot mint. The strip is planar between plane–plane supports only (`ChamferArmUnsupported` otherwise), and it meets each support at a definite angle. | Reading. |
| **Shell** (`topo::shell`, `topo::shell_open`) | Not reached, and the evidence is the refusal frontier, not an argument. Shell refused every curved-seam body tried (the cusp and slit fixtures below, typed). The offset-along-normals argument (every edge keeps its dihedral class, walls that would meet refuse `WallClearance`) is a reading, and today it is not what stands between a shell and a wedge end. | Probed (the consumer rows below). |
| **Single-face offset** (`topo::replace_face{,s}_offset`) | Not reached in the one configuration probed. The topology is fixed, so a wedge end needs the offset face to become tangent to a neighbour along their shared edge. A plane moved against a cylinder neighbour already refuses at d = 0.1 (re-anchoring a moved vertex off its carrier). Not audited past that probe. | Probed, one shape. |
| **Revert, graft, void insertion** | Cannot mint an edge. `revert` maps a cusp to a slit and back (legal together), `graft_disjoint` fuses nothing, and a void touching the outer shell shares no edge with it (that is a contact). | Reading. |
| **STEP import** (`step_import::import_step`) | Cannot mint. It transcribes the file's shared-edge structure, and a knife edge is its native twin's legal state (D1 tier 3, D7 step 4; Ev on PR 3317). | Pinned: `step-import/tests/cusp_round_trip.rs` `a_native_cusp_round_trips_through_step_as_a_solid`. |

No draft op exists.

## Downstream consumers of a declared cusp body

PR 3362's review covered fillet (typed `TangentialEdge`), mesh
(watertight, volume-sane) and the STEP round trip. This audit adds
chamfer and shell, on the lune extrude (a cusp, wedge 0) and on a
plate with a lune hole (a slit, wedge 2π):

- **Chamfer and fillet on the strut** refuse `BlendError::TangentialEdge`
  on both fixtures. Pinned: `chamfer_and_fillet_refuse_a_cusp_or_slit_strut_typed`.
- **Chamfer anywhere else** on either fixture refuses typed, for
  frontier reasons: `ChamferArmUnsupported` on a curved support, the
  run-out refusal, and the ring-clearance check on the slit plate's
  non-circle ring. So no "sane output" case can be reached today.
- **Shell** refuses `ShellError::Face` on both fixtures, at 1e-3 and
  0.05: the lune's inward walls cannot re-anchor near the kiss, and the
  slit's unequal cylinders have no closed-form pose. At 0.5 the lune
  refuses `WallClearance`. Pinned: `shell_refuses_a_cusp_or_slit_body_typed`.
  None of these refusals is wedge-specific.

## STEP import

Resolved with no fork. Import transcribes the file's own shared
`EDGE_CURVE`, which is the structural record D1 names, and it mints no
tangency, just as it admits every π seam undeclared. A designer pair
weighed this independently, and Ev's PR 3317 summary already said an
imported knife edge passes.

## Sweep scope

Two passes:

- By op: every `topo`, `sweep` and `verbs` door that returns a
  `Body`, and every body-yielding `Node` arm in editor-core's
  `eval/wire.rs`.
- By shape: where a tangent contact is classified and then continued
  rather than refused. That is `splitting::rules::apply_rule_a`'s
  `tangent_sector_osculation` descent and `describe_section_boundary`'s
  smooth arm, which is where the split hole was.

Blind spots: the probes used only planes and cylinders. That is also
all the split admits today (`splitting::classify::gate_operand`
refuses every other kind), so the split door is exercised on its whole
domain. A kind admitted later reaches the same pairing read. Face-offset
tangency was probed in one shape only.

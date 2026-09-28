---
id: every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one
kind: issue
title: STEP import now admits an undeclared cusp body; every op that can mint a wedge-0/2π edge must trace it to a declared input or refuse typed at its own door
status: review
branch: gather/wedge-end-door-audit
opened: 2026-09-28
priority: P2
cost: M
refs: [product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares, loft-between-opposite-turning-joints-reverses-a-seam-between-stations, declared-cusps-second-order-wedge-arm, self-overlapping-spines-build-and-validate]
---


Filed with Ev's ruling on PR 3317 (2026-09-28): a cusp is legal at
rest iff jet-determinate, so tier 3 no longer catches an op that mints
a knife edge nobody asked for. D1 hands that refusal to the ops.

Owed: sweep every body-producing op for a path to a definite wedge
end, list each op with its door (or add the refusal), and pin one row
per door. Future shell, draft and offset work inherits the obligation.
Designer D's optional companion: report `MaterialWedge::Cusp | Slit`
through the marks channel as a diagnostic (never a gate), plus an
importer option that refuses on it.

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
| **Extrude** (`sweep::extrude`) | Cannot mint past the profile. A strut's wedge is the profile joint's interior angle, and a cap meets every wall at π/2. | Reading. Declared form passes: `sweep/tests/a_swept_cusp_is_legal_at_rest.rs`. |
| **Revolve**, full and partial (`sweep::revolve`) | Cannot mint past the profile. A rim's wedge is the joint's meridional angle, the caps meet the walls at π/2, and a partial revolve's on-axis cap–cap edge sits at the window angle in (0, 2π). | Reading. Declared form passes: the same file. |
| **Tube, hollow tube** (`sweep::tube_along_arc{,_hollow}`) | Cannot mint. A circular section has no joint, and the ring-torus convention (`TubeError::Revolve`) refuses horn and spindle tori. | Reading. |
| **Loft** (`sweep::loft_body`) | **Hole, needs design.** Each section passes the profile door on its own, but a joint that turns opposite ways in two sections reverses between stations. Tier 3 exempts the NURBS seam by kind. | Probed: builds, `validate_geometric` Ok. Filed as `work/carve/loft-between-opposite-turning-joints-reverses-a-seam-between-stations.md`. |
| **Path sweep** (`sweep::sweep_body`) | Door is the profile. Every station is the same profile, so no joint turns differently between stations. (Self-overlap is a separate class, `work/carve/self-overlapping-spines-build-and-validate.md`.) | Reading. |
| **Boolean, curved** (`topo::boolean_op_with`; `union`, `subtract`, `intersect` and their `_with` forms) | Refuses typed today, but not at a wedge-specific door: the crossing frontier (`CurvedPierceUnsupported`, `CurvedBooleanUnsupported`) refuses first. The declared route, and the undeclared-tangency refusal that must land with it, is TANG's `declared-cusps-second-order-wedge-arm` item 3. **No declared form passes.** A declared external kiss verifies and then hits `CurvedPierceUnsupported`, and an internal kiss has no declaration at all (`ContactContradicted`, from C4's opposed-senses test). Evidence added to TANG's row. | Pinned: `a_boolean_that_would_kiss_a_curved_face_refuses_typed_at_the_op` (internal cylinder kiss subtract, external kiss union, plane cutter tangent to a hole wall). |
| **Boolean, shared rim** (`boolean::rim_wedge`) | `BooleanError::RimCuspArmUnbuilt`: a wedge-0/2π rim is the declared-cusp family, defined and unbuilt. | Pinned already: `sweep/tests/mate7a_torus_rest.rs` `a_kissing_torus_rim_routes_to_the_unbuilt_cusp_family`. |
| **Boolean, planar** | Cannot mint a legal wedge end. Two planes have κ_rel = 0, so a plane–plane wedge end fails jet-determinacy (tier 3's `LaminaWedge`), and coplanar contact is the `Rest` class (`UndeclaredCoincidence`). | Reading. |
| **Union node** (editor `wire_union`) | The boolean's door: pairwise contact judgement, then the union fold. | Reading. |
| **Placed union** (editor `wire_placed_union`) | Cannot mint. `topo::Separation::certify` has to certify the placements apart before anything is built (`NodeErrorKind::PlacementsUncertified`), and the copies are grafted disjoint with nothing fused. | Pinned already: `editor-core/tests/lib_placedunion.rs` `touching_boxes_over_disjoint_solids_refuse_the_same_way`. |
| **Pattern** (editor `wire_pattern`) | Cannot mint an edge. The result is `Instances` with nothing fused. Two instances touching is a contact, which the product gate's tier-3′ census judges. | Reading. |
| **Transform, mate, part instance** (`topo::transform_rigid`) | Cannot mint. The door admits only det = +1 rigid maps, which preserve every dihedral. | Reading. |
| **Split** (`topo::split`) | **Hole, closed here.** A plane tangent to a hole wall along a vertex ruling left the hole-side piece a doubled cusp, and tier 3 passed it. It now refuses `SplitFinishError::SectionCusp` in `splitting::finish::describe_section_boundary`'s smooth arm (any curved face smooth against the section plane). `split` surfaces the pinch rerun's `SectionCusp` over the direct run's `DegenerateSection`. The same tangency along an arc's interior already refused typed at the join (`DegenerateSection`, `SectionInvariant`). | Pinned: `a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint` (red first), with `a_split_through_the_hole_or_across_a_declared_cusp_still_cuts` as the counter-row. |
| **Fillet** (`sweep::blend::build::fillet_edges`) | Cannot mint. A spring edge is a π seam by construction (the ball is tangent to its support), and an end is a transverse cap or a refused run-out. | Reading. |
| **Chamfer** (`sweep::blend::build::chamfer_edges`) | Cannot mint. The strip is planar between plane–plane supports only (`ChamferArmUnsupported` otherwise), and it meets each support at a definite angle. | Reading. |
| **Shell** (`topo::shell`, `topo::shell_open`) | Cannot mint. An offset along normals keeps normals at corresponding points, so every edge keeps its dihedral class, and walls that would meet refuse (`WallClearance`). | Reading. |
| **Single-face offset** (`topo::replace_face{,s}_offset`) | Not reached in the one configuration probed. The topology is fixed, so a wedge end needs the offset face to become tangent to a neighbour along their shared edge. A plane moved against a cylinder neighbour already refuses at d = 0.1 (re-anchoring a moved vertex off its carrier). Not audited past that probe. | Probed, one shape. |
| **Revert, graft, void insertion** | Cannot mint an edge. `revert` maps a cusp to a slit and back (legal together), `graft_disjoint` fuses nothing, and a void touching the outer shell shares no edge with it (that is a contact). | Reading. |
| **STEP import** (`step_import::import_step`) | **No door** (below). | Executed in PR 3362's review. |

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

## STEP import: the facts for Ev

- **What import admits today.** A file whose solid carries a
  jet-determinate wedge-0/2π edge ships as `StepImport::Solid` under
  default `ImportOptions` (executed in PR 3362's review). `import_step`
  gates through `gate`, which calls `topo::validate_geometric` per
  placed solid when `topo::per_part_gate_owed` asks for it, and through
  `gate3` (`validate_pseudomanifold_certificate`) on the aggregate.
  Both gates pass the arm. `crates/step-import/src` has no cusp or
  wedge logic of its own.
- **Where a door would go.** Not in `gate` or `gate3`: their contract
  is "no opinion of its own" (D9 convention 2, stated on `gate`). A
  door would be its own pass after `gate3` on the body that ships, the
  same place as the chart-coherence channel. It would need a per-edge
  wedge verdict, and there is no public one: `topo::contact_marks`
  marks both a π seam and a cusp `Tangent`, and the material arm's
  `MaterialArmOutcome` is crate-private. Designer D's companion
  (`MaterialWedge` through the marks channel) is that verdict.
- **The declaration channel already exists.**
  `ImportOptions::declared_contacts` is position-anchored
  `ImportContact`s resolved against the assembled body; only
  `VertexRest` is shipped. An `ImportContact::Tangent { at }` anchor
  resolving to a cusp edge would make "the caller declares the file's
  cusp" expressible without a new channel.
- **Options and what each costs:**
  1. *The file's cusp is the file's declared intent.* No code. A file
     with an accidental knife edge ships silently. The cost is D1's
     "never inferred from values", since a STEP file has no
     declaration vocabulary.
  2. *Measure, don't gate.* Report each wedge-0/2π edge on
     `StepImport::Solid` the way `coherence` is reported. Needs the
     marks-channel verdict. It changes no import's outcome.
  3. *Refuse by default, admit per edge through
     `declared_contacts`.* Needs the verdict plus the `Tangent`
     anchor. This is D1's reading applied literally, and it changes
     the outcome for every file that carries a cusp today.
  4. *Refuse only behind an opt-in option.* Needs the verdict. Callers
     who do not opt in keep option 1's exposure.

## Sweep scope

Two passes:

- By op: every `topo`, `sweep` and `verbs` door that returns a
  `Body`, and every body-yielding `Node` arm in editor-core's
  `eval/wire.rs`.
- By shape: where a tangent contact is classified and then continued
  rather than refused. That is `splitting::rules::apply_rule_a`'s
  `tangent_sector_osculation` descent and `describe_section_boundary`'s
  smooth arm, which is where the split hole was.

Blind spots: the probes used only planes and cylinders. Any other
curved kind the split admits would reach the same smooth arm, and the
refusal does not read the kind, but no such split was executed.
Face-offset tangency was probed in one shape only.

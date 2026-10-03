# EDIT-PLACEMENT: gauges, and a placement that is parametric

This is the unit slate for `work/place/placement-is-spelled-three-ways-node-registry-and-rule.md`. It follows Ev's rulings on `[ev]` #3437 and `[ev]` #3441, both from 2026-09-29, and `[ev]` #3505 (2026-10-01), which are now A4, A9 and A11 (2)–(5) in `crates/editor-core/ASSEMBLY.md`. Read those clauses and the row's `RULED` section first: they are the contract. The designer reports behind them are rows 17–19 and 21 of `docs/DESIGN-FORK-LOG.md`.

The ruling reaches the data model, the mate solve, evaluation, persistence, export, Python and the viewer, so it ships as three units in order. Each one leaves main green and correct. None of them builds half of a later unit.

| # | Unit | Branch | Review |
|---|---|---|---|
| P1 | The `Placement` type, held by `Node::Transform` | `edit/placement-type` | dual (`docs/DUAL-REVIEW-PROTOCOL.md`) |
| P2a | Vocabulary: the code's cluster becomes a group and its representative a root, freeing "gauge" | `edit/group-root-vocabulary` | orchestrator's read |
| P2 | Gauges; the registry and maintenance go; own space; STEP refusal | `edit/placement-gauges` | dual |
| P2-carry | Split and inline keep a carried member's checked offset (ruling 9 below) | `place/carry-keeps-offsets` | single, full |
| P2-split | Split and inline at a gauge: the verbatim move, inline's gauge, the mate-placed inline, `Promote` and `Fold` | `place/p2-split` | dual |
| P2-face | A `FromFace` side across the seam (rulings 7 and 8's `FromFace` case) | `place/p2-face` | dual |
| P3 | Viewer: the group-wide probe and "place where shown" | the viewer owner's | filed on its slate, not EDIT's |

Every premise below was read off `origin/main` by earlier surveys and may have moved. Treat each one as a hypothesis, verify it, and report any correction. This program's specs have each carried at least one false premise.

## P1: the `Placement` type (spec)

**What.** One type, `Placement`, beside `Frame` in `crates/editor-core/src/placement.rs`: an ordered chain of steps, composed left to right.
- `Step::Rigid { translation: [Expr; 3], axis: [Expr; 3], angle: Expr }`: the three components `Node::Transform` holds today. It is proper by construction; its axis is decided at evaluation by the direction door, as `Transform`'s is.
- `Step::Matrix(Frame)`: a literal proper frame, held to A6 (`Frame::admission_fault`) at every door that writes one and at load.

`Node::Transform` holds one `Placement` (`{ input, placement }`), so a body can take a `point_at` frame too. P1 does not touch `Doc::placements`, `InstantiatePart` or maintenance; P2 removes them.

**Premises.**
- `Node::Transform { input, translation: [Expr;3], rotation_axis: [Expr;3], rotation_angle: Expr }` is at `node.rs` ~2134. Its slots are `Translation(Axis3)`, `RotationAxis(Axis3)` and `RotationAngle`.
- It evaluates in lane T through `wire_transform` → `eval::transform_map` (`wire.rs`).
- The mate solve builds the same map at f64 (`mate/member.rs`).
- `Transform` appears in `corpus/tour/die_composed_tour.pncad`. That file is regenerated from source if the wire shape moves.

**Rulings.**
1. **One evaluator.** `Placement::eval::<T>(&ParamEnv<T>, …)` returns the rigid motion.
   - A `Rigid` step goes through the node's existing construction, so a single-`Rigid` `Transform`'s bits do not move.
   - A `Matrix` step goes through the literal frame.
   - `Placement::literal(&Frame)` is the one bit-exact `Frame` → `Placement` door.
2. **Slots.**
   - A `Transform`'s components are addressed per step. A single-step placement keeps today's `SlotId`s addressing the same components. Adding the step index is the unit's call; justify it, and keep the census exhaustive.
   - A `Matrix` step has no expression slots.
   - The parameter-reference and dimension checks run over every `Expr` in the chain, at the edit door and at the load door.
   - Split's and inline's parameter walks cover them.
3. **No silent wrong answer in lane T.** A `Transform` whose chain holds a `Matrix` step is literal in that step and evaluates in every lane. The mate solve still runs at the nominal only. That class is filed as `work/msolve/a-mate-through-a-parametric-placer-is-solved-at-the-nominal-in-box-and-seed-runs.md`, and P1 does not widen it.
4. **Persistence.**
   - `Transform`'s wire shape becomes the chain. An old file refuses, typed (`Unreadable`, with the regenerate recourse); it never loads with a different meaning.
   - `Doc::bit_eq` compares placements through `Expr::bit_eq`. Literal f64 bits round-trip exactly, including `-0.0`.
   - Every golden or corpus file that moves is regenerated from source and named, together with what moved it.
5. **Python.**
   - `Transform` authoring takes a `Placement`, and the existing three-component form stays as sugar for one `Rigid` step.
   - `point_at` and `path_start_frame` give a `Placement` with one `Matrix` step.
   - The census, the tag inventory and the stubs move with it. This is LIB's ground; announce the crossing.

**Rows.** Each is red on `origin/main`, then green:
- Every corpus `Transform`'s evaluated motion is bit-identical before and after.
- A `Transform` over a `point_at` frame evaluates to that frame exactly.
- A two-step chain composes in order.
- A `Matrix` step that is improper or non-finite is refused, typed, at the edit door and at load.
- A parameter drives a `Rigid` step's angle.
- An old file is refused, typed.

**Seams.**
- WIRE: `eval/wire.rs`.
- MSOLVE: `mate/member.rs`, where the f64 map is built.
- LIB: `pncad`, `pncad-py`.
- Viewer readers of `Transform`'s fields.

Announce each crossing.

## P2a: vocabulary (spec)

**What.** The code already says "gauge" for something else. `gauge_of`, `SolvedPoses::gauge`, `ClusterMaintenance`'s fields, `EditError::MaintenanceRefused { gauge }`, `SnapshotError::PlacementNotGauge` and Python's `gauge_of` all use it for a cluster's representative instance, the document-order-first member (`mate/solve.rs` `gauge_of` ~352, `solve_document` ~1102). P2 adds a gauge node, so the old word has to go first.

**Rulings.**
1. The words follow ASSEMBLY.md A9, A11 and A4:
   - a cluster is a **group**;
   - its representative instance is its **root**;
   - `SplitError::TornCluster` becomes `TornGroup`, the name A4 already uses.

   Rename every identifier, doc comment, message and Python name. Leave alone what P2 deletes outright (`ClusterMaintenance`, the `Maintenance*` arms, `PlacementNotGauge`), and say so.
2. **Python:** `gauge_of` becomes `root_of`, and `clusters` becomes `groups`. The census, the stubs and the tag inventory move with them. This is LIB/BIND's ground; announce the crossing.
3. ASSEMBLY.md's "Where in the code" table names the new identifiers.
4. **Nothing changes meaning.** No golden, pin or saved byte moves. Show it: the full editor-core, viewer and pncad suites and the Python suite run green with no expected value edited.

**Review.** The orchestrator's read. Hand back with the rename list and a grep showing that no `gauge` identifier means a cluster representative any more.

## P2: gauges (spec)

**The contract.** ASSEMBLY.md A4, A9 and A11 (2)–(5), as merged with `[ev]` #3505 (2026-10-01), and fork-log rows 17–19 and 21. This section says how to build them. Where the two disagree, the clauses win; report the disagreement.

**Premises** (surveyed on `c80090264`; verify each and report corrections):
- **The registry.**
  - `Doc::placements: BTreeMap<RecipeNodeId, Frame>` (`doc.rs` ~820) is keyed on a group's root.
  - `Doc::placement` (~1048) reads it, defaulting to the identity.
  - `DocEdit::SetPlacement` (`edit.rs` ~435, applied ~4039) writes it.
- **Maintenance.**
  - The edit door runs `maintain` / `reconcile` / `registry_after` (`mate/solve.rs` ~1418–1668) behind `edit.moves_the_mate_graph()`.
  - It records `ClusterMaintenance` rows on `LoggedEdit.maintenance` (`edit.rs` ~3392; `deny_unknown_fields`, both fields required).
  - Replay re-applies the rows (`apply_logged` ~3436, `Doc::replay` ~4281).
  - The viewer builds and replays `LoggedEdit` (`viewer/src/session.rs` ~2890, `history.rs`).
- **Instances.** `Node::InstantiatePart { doc_ref, interface }` (`node.rs` ~2470) holds no frame. Its world pose is `placements[root] ∘ relative` (`SolvedPoses::placement`, `solve.rs` ~143). It is applied in `wire_instantiate_part` (`wire.rs` ~339, ~414).
- **The solve reads the registry frame itself** (`pair_left_factor`, `solve.rs` ~938), not only through `SolvedPoses::placement`.
- **Cross-instance contact.**
  - Every declared cross-instance contact lands on `AssemblyError::Uncertified` today (`assembly.rs` ~645–685), because the census cannot certify two instances' charts.
  - Minting is role-blind (`assembly.rs` ~944).
  - Measures read any references and never check a cluster.
- **A11 (3)'s rule that pattern-placed instances cannot be roots is not implemented** (`clusters_welded_by` ~325 admits every instance).
- **STEP export.** `pncad::export::{step_for_node, export_document_step}` (`pncad/src/export.rs` ~105, ~144) read bodies that evaluation has already placed.
- **Saved files.**
  - 18 saved files carry `"placements": {}`, every one empty: 14 `bool13_goldens` and `tests/golden/golden.cad`, plus `die_tool.pncad`, `die_composed_tour.pncad` (34 `"maintenance": []` log entries), `plate_param.pncad` and `gallery_ring.pncad`.
  - There is no schema version; an old file refuses `Unreadable` with the regenerate recourse.

**Rulings.**
1. **The gauge node.**
   - `Node::Gauge { parent: Option<RecipeNodeId>, placement: Placement }`, where `None` is the world. It denotes no body, so as a root it contributes nothing (A10).
   - Its frame is its parent's frame composed with its `placement`.
   - `parent` is a reading edge (`reading_edges`), not an input, so the DAG's acyclicity check does not see it; the gauge chain is kept acyclic by `doc::gauge_ref_fault`'s walk, asked at `InsertNode`, `SetGauge` and load.
   - Its steps are addressed by the existing `SlotId::PlacementStep`, through slot edits, and the parameter and dimension checks cover them, as for `Transform`.
2. **The instance.**
   - `InstantiatePart` gains `gauge: Option<RecipeNodeId>` (`None` is the world; a reading edge) and `offset: Option<Placement>`.
   - `Node::instantiate_part` writes `gauge: None, offset: Some(Placement::IDENTITY)`, so every insert door (the viewer's `add_instance`, Python's `instantiate_part`, `InsertNode`) yields a placed instance at the world origin.
   - An absent offset arises only from a mate (ruling 4) or on purpose.
3. **Groups and roots** (A11 (2), (3)).
   - A mate places when its two instances name the same gauge reference (both the world, or one gauge id). Otherwise it declares (`MateRole`).
   - A group is a component of instances under placing mates.
   - Its root is its earliest member, in document order, that carries an offset. Pattern-placed instances cannot be roots: implement this, or show that a pattern never yields an `InstantiatePart` and say so in A11 (3)'s code table.
   - A group with no member carrying an offset, or whose gauge chain reaches a deleted gauge, is **unplaced** (ruling 6).
   - World pose = A∘F∘B, composed in one home (`Pose::compose_around`):
     - A, the derived offsets of the placers on the mate path, which is the identity bit for bit when no placer is on the path;
     - F, the gauge chain's frame composed with the root's offset;
     - B, the composed representatives.

     The solve is gauge-free; F enters only through this composition, so a parametric gauge frame does not widen the nominal-only solve class (`work/msolve/a-mate-through-a-parametric-placer-…`). (The build found that "the solve never reads a gauge frame" could not hold with a placer on the path: A conjugates through F.) The pose is evaluated in every lane, and the gauge chain and the offset feed the memo key.
   - **Checked offsets.** A non-root member's offset is a checked statement. It is verified against the solve within the solve's tolerance, never skipped: an unplaced group checks its offsets in its own space, and a member the spanning tree cannot reach faults `OffsetCheck::Unreached`, naming the mate. A mismatch faults that instance, typed, naming the offset, with the recourse "clear the offset, or change the mate". With a placer on the path, the check reads the gauge frame at the nominal; that is inside the nominal-only class above.
4. **Edits.**
   - `SetPlacement` goes. `DocEdit::SetOffset { instance, offset: Option<Placement> }` and `DocEdit::SetGauge { node, gauge: Option<RecipeNodeId> }` (an instance's gauge or a gauge's parent) replace it. Each is refused, typed, on the wrong node kind, with a recourse.
   - **The mate door.** Inserting a placing mate that joins two groups clears every offset in `a`'s group, the root's and each checked member's, in the same edit, each reported `OffsetCleared`. The mate places `a`'s group on `b`'s: the first pick moves, as the viewer's stories pick the mover first, and no ratified clause chose a side. `b`'s root, read through `root_and_cause`, roots the merged group, so `b` does not move whatever `a`'s group held, and a freshly inserted part never carries a stray checked identity offset. When `b`'s group is unplaced nothing is cleared, and `a`'s root roots the merged group. This is a compound edit, recorded as its edits, and replay re-applies them without solving.
   - **"Copy b's gauge to a, then mate"** (`regauge_then_mate`) is one compound edit: `SetGauge` on every member of `a`'s group, then the mate insert above. It refuses, typed (`EditError::WouldStartPlacing`, naming the mate), when the re-gauge would make any other mate start placing. It is one door in `editor-core`, bound in Python.
   - Deleting a gauge, a placed member or a placing mate is never refused. What stays placed is recomputed.
   - **References to a deleted gauge are kept, dangling.** Then the unplaced group can name its cause, and split can refuse a dead reference as A4 says; the recourse names `SetGauge`. Check `roots::check` and the DAG's invariants against dangling reading edges, and state what you changed.
5. **What goes.**
   - `Doc::placements` and `Doc::placement`;
   - `ClusterMaintenance`, `Maintain`, `maintain`, `reconcile`, `registry_after`, and the edit door's maintenance solve. `apply` keeps the reach only for the mate insert's clocking rider (`admit_mate`);
   - `LoggedEdit.maintenance`: the log becomes plain edits, and `LoggedEdit` goes if nothing else needs it;
   - `EditError::MaintenanceRefused` and `MaintenanceUnrecorded`, `PersistError::MaintenanceFrame`, and `SnapshotError::PlacementSite` and `PlacementNotGauge`, with every match site (`refactor.rs` ReplayTail ~857–879, `pncad/src/workspace.rs` ~784, `pncad-py` `tags.rs` and `py/doc.rs`, `edit_payload.rs`);
   - Python's `Doc.placement`, `Doc.placements` and `DocEdit.set_placement`. Their replacements are `set_offset`, `set_gauge`, the gauge node and the compound door.

   `Doc.last_maintenance` and the Python `Maintenance` class stay: DM7's strand reports live there. They carry no placement maintenance; the mate door's `offset_cleared` is added.
6. **The unplaced group** (A9, A11 (2), Ev's #3441 and #3505).
   - **One derived fact per group: its space**, either the world or its own. It is evaluated in its own frame with its tree root at the origin, and it solves and checks internally as usual.
   - **One representation:** `Space { World, Own { group, cause } }`, asked through one cross-space predicate (`Evaluation::across_spaces`).
   - **Product.** `product::product` gathers only world-space bodies. An unplaced group is not part of that body (A9). A document with no world body refuses `ProductError::Unplaced`, naming the unplaced groups, with the place-it recourse.
   - **The at-rest gate** mints pairs only within one space, and it checks every own space. The per-space loop lives in the one shared core every caller goes through (`Product::spaces`, `own_spaces`, `gate_spaces`), so the kernel, the viewer badge and Python all run it.
   - **Readers.** A measure, the flush detector, selection, clearance and pick whose references lie in two spaces refuse, typed, naming the unplaced group, with the recourse "place it (a gauge, an offset or a placing mate)". None answers across spaces.
   - **The document seam.** An unplaced group inside a part crosses as a fact (`CarriedUnplaced`, `Evaluation::unplaced_below`), routed like a carried mint refusal. The outer product holds the world only, and the outer gate does not re-run an inner part's own spaces.
   - **STEP.** `export_document_step`, and `step_for_node` on an unplaced instance, refuse naming the unplaced parts, the cause (no offset, or the deleted gauge), and how to place them (A11 (2)). An unplaced group below refuses `UnplacedBelow`, naming the part, its route and the cause.
7. **Split and inline: A4 as written.** The clause is the contract; build it. In particular:
   - split's refusals: `TornGroup`, two anchors, a severed gauge, a declaring mate that would start placing, a dead gauge reference, and a cut of unplaced material alone;
   - the two hoist shapes;
   - inline's gauge, and its sugar for a one-group part at the empty chain;
   - a mate-placed instance inlined only when its part is one group at the empty chain (otherwise refused, named, with the recourse "set the part's root offset to the empty chain, or place the instance, then inline");
   - a moved member's further offset refused;
   - the frame rule and the fold rule, each refusing the mate by name;
   - declaring crossing mates filling `InterfaceRecord` (AQ8).

   - a cut that leaves a group's placing mate behind refuses `PlacingMateLeft`;
   - every reference leaving the cut votes for an anchor, and plain geometry votes for the world; a group unplaced for lack of an offset casts no vote;
   - a `FromFace` side crosses the seam with its head (P2-split's ruling 7).

   Every refusal is typed and carries a one-edit recourse (the refusal standard), guarded by `refusal_concision_refactor.rs`. Replace `InlineError::UnplaceableFrame` only if a new arm states the same refusal more truly.

   The defects on main that the designers found close here, each pinned by a row:
   - inline reads the group's frame, not the instance's pose;
   - inline rebinds mate heads without re-coordinating their frames.
8. **Cross-gauge contact at the gate.** A declaring mate across gauges goes through the gate's existing declared-contact path. P2 neither widens nor narrows what the gate can certify. That path certifies a declared instance pair at rest within tolerance and refuses one beyond it, so a cross-gauge declaration certifies. (The premise that it answers `Uncertified` for instance pairs was false.)
9. **Persistence.**
   - `Doc` loses `placements`. Instances gain their two fields, both required on the wire, and the gauge node gets a wire shape.
   - An old file refuses, typed (`Unreadable`, with the regenerate recourse); it never loads with a different meaning.
   - A dangling gauge reference loads; it is a legal state.
   - Regenerate every file the change moves from its source, and name each one with what moved it. Re-baseline every pin that moves and list them in the PR body. A golden that changes is never a cost to weigh against the change (`docs/prompts/implementer-discipline.md`).
10. **The placement refusal arms** on `work/recipe/edit-refusals-short-of-the-shape-guard.md`.
    - The row says seven. Five remain on main: `EmptyPlacementList`, `MaintenanceUnrecorded`, `PlacementAxis`, `PlacementOnNonInstance` and `PlacementRuleMismatch` (`viewer/tests/refusal_concision_edits.rs` ~863).
    - Remove the ones P2 deletes. Rewrite the others P2 touches so they carry a recourse, and update both the census and the row.
    - `PlacementRuleMismatch` and `EmptyPlacementList` concern the explicit placement rule, not the registry. Leave them unless P2 touches them.

**Rows.** Each is red on `origin/main` (or absent there, where the door is new), then green:
- an instance on a gauge under a gauge is placed by the composed chain, and a document parameter driving the outer gauge moves both levels;
- inserting an instance places it at the world origin, and mating it to a placed instance clears its offset in the same logged edit, which replays without solving;
- the compound "copy b's gauge to a, then mate" re-gauges a's whole group, and refuses when the re-gauge would make another mate start placing;
- a checked offset that disagrees with the solve faults that instance, typed;
- deleting a gauge, a placed member or a placing mate is not refused, and the group becomes unplaced: it evaluates in its own frame, the gate mints none of its cross-space pairs, a cross-space measure refuses typed, and STEP export refuses naming the parts and the cause;
- the A4 table, one row per arm: every split refusal and both hoists; inline's gauge, its sugar, its mate-placed admission and refusal, and the moved-member refusal; and the frame-rule and fold-rule refusals;
- inline-of-split returns the document split was given, up to node ids and the regrouping of a cut whose roots a kept root separates;
- a declaring mate crossing a cut fills `InterfaceRecord`;
- the two main defects above, red on main;
- an old file refuses, typed, and a dangling gauge reference saves and loads;
- Python: a `threading.Thread` row over the new doors, and the census.

**Mutants** (plant, run, revert; report which rows go red):
- the solve reads the gauge frame;
- the mate door forgets to clear an offset in `a`'s group;
- a checked offset ignored;
- an unplaced group gathered into the product;
- STEP export of an unplaced part allowed;
- the frame rule skipped;
- the fold rule skipped;
- the hoist taking a gauge with two groups.

**Sweep.** List every reader of a world pose and of `SolvedPoses`: production, Python and the viewer. State the space each one reads, and which ones P2 changed.

**Seams.**
- MSOLVE: `mate/solve.rs`, `mate/member.rs`.
- WIRE: `eval/wire.rs`, `eval/measure.rs`.
- ASSEMBLY: `assembly.rs`.
- LIB/BIND: `pncad`, `pncad-py`.
- The viewer's readers: `matetool.rs`, `session.rs`, `history.rs`, `display.rs`. Keep them compiling and their tests green. Behaviour beyond that is P3's.
- TCOST/TINT: censuses, mechanical.

Announce each crossing in the PR body.

**Review.** Dual (`docs/DUAL-REVIEW-PROTOCOL.md`). P2 lands after P2a.

## P2-split: split and inline at a gauge (spec)

**What.** P2-core (PR 3676) built the cases of A4 that need no gauge minted or removed, and refuses the rest typed, so this unit only lifts refusals. Four arms go or narrow: `SplitError::CutHoldsGauge`, `InlineError::NeedsAGauge`, `InlineError::MatePlaced`, and `MateFaceFrameCrosses` on both sides. The row is `work/place/placement-split-and-inline-at-a-gauge-are-refused-until-p2-split.md`. The contract is ASSEMBLY.md A4 ("Identity, pins, split and inline") and A11 (2)–(5), with P2's ruling 7. Every result equals A4's document, and no lifted refusal changes a result P2-core admits. The older split and inline arms' `Recourse:` is `work/place/split-and-inline-refusals-short-of-the-shape-guard.md`'s, not this unit's.

**Premises** (read on `ef90c4dba`; verify each one and report any correction).
- **Where the four refusals are raised.** All four are in `crates/editor-core/src/refactor.rs`.
  - `CutHoldsGauge`: in `split` (~2120), right after the reading-edge rule. It fires on any gauge in the cut, before the cut groups are read.
  - `MatePlaced`: in `inline`'s match on `root_and_cause` (~2767), for any instance that is not its group's root.
  - `NeedsAGauge`: in `inline`'s `sugar_root` (~2866). It fires at a non-empty offset whenever the part is not exactly one group whose root is at the empty chain on its world, with no gauge and no member carrying an offset.
  - `MateFaceFrameCrosses`: in `split`'s crossing-mate loop (~2298) and `inline`'s frame-rule loop (~2931). Both raise it only after `frame_survives` holds, so today a `FromFace` side refuses even where its authored twin crosses.
- **What P2-core admits.**
  - The group hoist (`hoisted`, ~2223): the cut is exactly one placed group, its instances and its mates. The instance takes the root's offset, and the root lands at `Placement::IDENTITY`. A member carrying an offset refuses `HoistedMemberOffset`.
  - The verbatim move: any other cut, with the instance at the empty offset. `carry`'s `regauge` sends every gauge reference to the part's world.
  - Inline's sugar: the root takes the instance's offset.
  - The empty offset: the content lands verbatim on the instance's gauge. `regauge` sends the part's world to `host_gauge`.
  - Rows: `crates/editor-core/tests/p2_gauges.rs` (`the_group_hoist_and_the_frame_rule_at_a_split`, `inline_admits_the_sugar_and_the_empty_offset_and_refuses_the_rest_typed`, `a_from_face_side_across_the_seam_refuses_typed_at_split_and_inline`, `a_cut_of_two_placed_groups_moves_verbatim_rather_than_hoisting`).
- **Helpers to reuse.**
  - `frame_survives` (~801): the one frame-rule predicate.
  - `carry` (~223): its `regauge`/`settle` pair is the hook for every gauge rewrite and offset rewrite below.
  - `root_and_cause`, `groups`, `places` and `member_of` in `mate`.
  - `spaces_with` and the anchor vote (~2163).
  - `FaceName::part_local` (`names/role.rs` ~470): the unwrap. The wrap is a `StableName { node, path: [RoleSeg::InPart { of }] }`, as `remap_face` and the crossing loop build it.
  - `Recording` for recorded edits.
  - `refusal_concision_refactor.rs`'s wildcard-free rosters.
- **Gauge references are reading edges, not inputs** (`Node::inputs`, `node.rs` ~3088, returns nothing for `Gauge` and `InstantiatePart`; `mate::solve::reading_edges` ~618). D-2's `SeveredEdge` loop therefore never sees a kept node hanging from a cut gauge, so the severed-gauge rule needs its own check.
- **What a `FromFace` frame resolves against.**
  - `resolve_side` (`mate/solve.rs` ~1189) asks `MateReach::face_pose` of the member's instance's part (`part_of`).
  - `face_pose_over_cache` (`eval/mod.rs` ~3312) looks up the PART-LOCAL name in that part's product table and reads `topo::readback::face_pose` off the part's evaluated product body, in the part's own coordinates, with no placement applied.
  - For a part that is an assembly, that body holds its inner instances already placed. A face `R.InPart{f}` therefore resolves to R's pose in the part composed with f's pose in R's own part.
  - The face name is the state. `FaceFrame` (`mate.rs`) is spelled part-local, never in the head's qualified spelling.
- **No edit writes a mate's alignment except its insert** (`DocEdit::writes_a_mates_datum`, `edit.rs` ~510). Since the mate frame became a base and an offset (`[ev]` #3920), a slot edit at one of a frame offset's rigid steps writes it too, and asks the same admission.
  - `Rebind` rewrites `payload_names`, which are the two heads only. It never rewrites a `FaceFrame` face, whose name lives in another document's namespace.
  - Deleting and reinserting the mate is not a re-spelling: the insert runs the mate door (`clear_joined_offsets`, ~4608), which clears offsets when a placing mate joins two placed groups. It also mints a new id and moves the mate's root-list position.
- **`carry` inserts carried mates through that same door, so it clears carried offsets.** A cut that moves verbatim, holding a group whose non-root member carries a checked offset, loses that offset in the part. The edit reports `OffsetCleared` in `part_maintenance` and does not refuse. Seen on `ef90c4dba` by a probe (placed pair, top's offset set to its solved pose, plus a second placed base, cut all four): the part's top has `offset: None`. Inline's carry is the same code. A11 (2) says a further statement of where an instance sits is "never silently ignored", and A4's round trip is shape-exact, so this is a defect that closes here.
- **`split`'s `root_lands_empty` (~2256) does not read the root's gauge.** That was sound while every cut gauge reference landed on the part's world. Once the cut can hold a gauge, it is not.
- **The round-trip rows assert evaluation, not shape.** `inline_admits_the_sugar_…` says "`inline(split(d))` is `d` up to node ids" but checks the group count, the root offset and the volume. No comparator up to node ids exists in the tree.
- **A stale comment.** `inline`'s doc says spliced placements are "the host instance's frame COMPOSED onto the part's own (`Frame::compose`)", which contradicts A4 ("neither computes a frame"). Rewrite it as a drive-by.

**The cut into units.** This section builds as three units, in order:
- **P2-carry**: ruling 9 and row C1, alone. It is a live defect on main (a checked offset dropped by a split or inline that P2-core admits), so it does not wait for the rest.
- **P2-split**: rulings 1–6, 8 for authored sides, and 10; rows S1–S7, I1–I5 and R1.
- **P2-face**: ruling 7, the `FromFace` case of ruling 8, and rows F1–F5, as Ev ruled on `[ev]` #3888.

**What PR 3908 builds of P2-split, and what follows it.** `[ev]` PR 3888 asks whether a gauge in the cut moves into the part as it is or becomes the instance, and whether the group hoist and inline's sugar stay. P2-split's PR builds the part that is the same under every answer:
- **Built:** ruling 1 with D2 (an instance votes its gauge whatever its space, a dead reference having refused first; a cut gauge on a dead chain refuses `DeadGaugeReference`, whose field is `node`); ruling 2 (`SplitError::SeveredGauge`); ruling 4 for every cut that holds a gauge, so no cut gauge is hoisted and `SplitError::CutHoldsGauge` is gone; ruling 5 with D3's half for inline (the minted gauge takes the instance's label; `InlineError::MovedMemberOffset`; `InlineError::NeedsAGauge` is gone); ruling 6 (`MatePlaced` narrowed; its `part_root` and `part_gauges` name the concrete remedy where only the part root's offset or the gauges its group sits on are wrong); ruling 10. `inline`'s stale doc comment is rewritten.
- **Rows:** S2, S3, S4 with D2's, S5 and S6 re-cut to the verbatim move (a kept mate reading a root on a cut gauge refuses `MateFrameCrosses`, since that root lands on the gauge's image; a kept instance on the anchor declaring into a cut gauge refuses `WouldStartPlacing`), I1–I5, and R1 over every cut holding a gauge, nested and parametric gauges among them (`crates/editor-core/tests/p2_split.rs`). The comparator is `tests/fixture/round_trip.rs`: it reads each node off its own rendering, never through the remapping under test, and has a failing row for each field it reads.
- **Three facts the premises missed.** Gauges live in no space, so a cut of gauges, datums or other body-less nodes passed every refusal and left an instance of a part with no body: `SplitError::NoMaterial` refuses a cut none of whose roots denotes a body. `SetGauge` may put a node on a gauge inserted after it, so `carry` orders each carried gauge ahead of what sits on it (`gauges_first`) rather than relying on document order. A gauge is a sink of the recipe DAG and so a product root (A10), so the gauge inline mints joins the root list at the instance's position, ahead of the spliced roots.
**Ruling (i)** (Ev on `[ev]` PR 3888): a gauge in the cut moves into the part as it is. Split has no hoist and inline no sugar; `DocEdit::Promote` and `DocEdit::Fold` carry the convenience. A4 states it ("The cut moves as selected", and the `Promote`/`Fold` sentence). The unit that builds it:
- **Split.** Every admitted cut moves verbatim: the instance sits at the empty chain on the anchor, and every root keeps its offset. The group hoist (`hoisted`) and `SplitError::HoistedMemberOffset` are gone, so a cut root's offset counts as cut for its parameters (`UncutParamReference` where a kept node reads one too), and the frame rule's `root_lands_empty` reads the root's own offset and gauge, so a kept mate reading a cut root off the empty chain refuses `MateFrameCrosses`. Where promoting that root alone would land it at the empty chain, the refusal names it (`promote`) and its recourse is that promote; where the shared parameter is read by a cut root's offset that a promote admits, `UncutParamReference`'s recourse names the promote too. A cut root that is no instance, mate or gauge — a measure, an assertion, other recipe content — while the cut anchors on a gauge refuses `SplitError::UnplaceableRoot`, recourse "add {g} and what sits on it to the cut, or fold {g} (Fold), then split": inline refuses such a root off the world's origin (`UnplaceableFrame`, ruling 5), and the two refusals ask one predicate (`splices_onto_a_gauge`), so split admits no cut inline cannot put back. The gauge hoist (ruling 3) is not built.
- **Inline.** At the empty offset the content lands verbatim on the instance's gauge; at any other offset inline mints the gauge (ruling 5) by a `Promote` of the instance, its refusals read through `edit::promote_plan`, Promote's own, so the two cannot drift. The sugar (a root-placed instance over one such group, its root taking the offset) is gone. The mate-placed inline (ruling 6) stands: its root takes the instance's place because neither door computes a frame.
- **`Promote { instance }`.** The instance's offset becomes `Gauge { parent: <its gauge>, placement: <its offset> }`, minted as its insert would be and returned in the record; the instance and the other members of its group sit on it, the instance at the empty chain, so each placing mate still places. The gauge joins the root list just ahead of the instance, or at the end when the instance is not a root. It refuses `PromoteOnNonInstance`; `PromoteNonRoot` when the instance is not the earliest member of its group carrying an offset (`root`, whose offset states where the group sits; recourse "promote {root}"); `PromoteWithoutOffset` when no member carries one (setting the instance's makes it the root); and `PromoteMemberOffset` (another member's offset is stated in the old gauge). Root-ness is asked first, so no recourse leads back to a refusal already passed. A promote moves no label: the instance stays what its label names.
- **`Fold { gauge }`.** The gauge dissolves; every node on it hangs from its parent, and each one's chain (a gauge's `placement`, an instance's `offset` when `Some`) gets the gauge's steps in front by `Placement::compose`. An instance with no offset keeps none. The gauge goes as `DeleteNode` takes a node, through one helper (`remove_unread`): the same consumer check, root maintenance and strand report. With exactly one dependent and that dependent unlabelled, it takes the gauge's label; otherwise the label goes, reported as `Maintenance::LabelDropped`. It refuses `FoldOnNonGauge`, `FoldWouldDangle` where another node reads the gauge as an input (recourse "delete {r} (and what reads it), then fold {g}"), and `FoldWouldStartPlacing` where a declaring mate would read two instances on one gauge — one predicate with `regauge_then_mate`'s `WouldStartPlacing` (`mate_that_would_start_placing`); `SetGauge` asks none, since there the regauge is what the caller named.
- **Poses are bit-exact under both.** A gauge chain and an offset fold one step at a time (`Placement::motion_after`, read by `gauge_frame` and `group_frame`), so a frame is a function of the chain's steps and not of which gauge holds each; `Promote` and `Fold` regroup steps and move no bit of any pose.
- Both are recorded, replay with no reach, are bound in Python (`DocEdit.promote`, `DocEdit.fold`), and stand in `viewer/tests/refusal_concision_edits.rs`. `fold ∘ promote` is the identity, and `promote ∘ fold` is the identity up to node ids on a gauge with one dependent at the empty chain and no label.
- **The two conveniences.** "Make a part at this frame" is a promote of the group's root and a cut leaving the promoted gauge: the part's root sits at the empty chain, the instance on the promoted gauge at the empty offset. Ruling 3's gauge-hoist result is a cut of K's content leaving K, then a fold of K: the instance takes K's placement as its offset, and K's instances sit on the part's world at their offsets.
- **Rows** (`crates/editor-core/tests/p2_split.rs`, `p2_promote_fold.rs`): R1 over every shape split admits, the `UnplaceableRoot` refusal over a measure and an assertion root, and the root-order collapse (D1); P1 `fold ∘ promote` and P2 `promote ∘ fold`, each also under rotations comparing poses bit for bit; P3 a part at this frame; P4 content then fold, also under rotations; P5 the refusals, each with its recourse taken; a fold of a gauge an axis reads, and a label a fold drops.

The face rulings (7, the `FromFace` case of 8, rows F1–F5 with F3b and F3c) are P2-face's.

**Where the clauses disagree with themselves, ruled.**
- **D1. A4's round trip against the gauge hoist and inline's sugar.** Ruling (i) retires it: with no hoist and no sugar, `inline(split(d))` is `d` up to node ids on every shape split admits, the two shapes D1 named among them (a gauge at the empty chain holding a group, a gauge holding one group at the empty chain). One disagreement remains, and R1 pins it rather than hiding it: a cut whose roots a kept root separates in the root list comes together where the first of them was (A10's replacement rule; A4 states the regrouping), so the round trip lists the kept root after the cut's. The comparator reads root order, and that row's only disagreement is its root line.
- **D2. An unplaced cut group on a gauge other than the anchor.** A4 wins: every gauge reference leaving the cut lands on one anchor. A group unplaced for lack of an offset votes its gauge like any other instance; it only places nothing. A cut whose references then name two anchors refuses `TwoAnchors`. This narrows what P2-core admits (it sent such a group to the part's world and lost its gauge), and that admission was the defect.
- **D3. A hoisted gauge's label.** Ruling (i) retires it with the hoist. The gauge inline mints takes the instance's label under ruling 5, since the instance it stands in for is deleted; `Fold` hands a lone unlabelled dependent the gauge's label and reports one it drops. R1's comparator reads every label.

**Rulings.**
1. **The vote, with gauges in the cut.**
   - A cut gauge whose parent is outside the cut votes its parent.
   - A cut instance or cut gauge whose gauge reference is inside the cut casts no vote.
   - Every other vote is as P2-core casts it, with D2: an instance votes its gauge, its group placed or unplaced for lack of an offset, and a world-space non-instance root votes the world.
   - A cut gauge whose parent is a deleted gauge refuses `DeadGaugeReference` naming that gauge (the field holds the cut node, gauge or instance; rename it if that reads truer). Otherwise the instance left behind would name a deleted gauge, which the insert door refuses untyped as `RemainderEdit`.
2. **The severed-gauge rule** (A4: "a kept instance or gauge that hangs from a cut gauge"). A kept node whose `gauge_ref` is a cut gauge refuses with a new arm, `SplitError::SeveredGauge { gauge, kept }`, recourse "add {kept} to the cut, or set its gauge outside the cut (SetGauge)". Check it after `TornGroup`, which already speaks to a placing mate's two members, and before the vote. `CutHoldsGauge` goes.
3. **No gauge hoist** (ruling (i)). A cut holding a gauge moves verbatim (ruling 4); the gauge-hoist result is a cut of the gauge's content and a `Fold` of the gauge left behind.
4. **The verbatim move with gauges.**
   - Each cut gauge is carried. Its parent maps to the part's world when it leaves the cut, and to the carried id when it stays inside.
   - `carry`'s gauge order holds: a gauge precedes everything on it.
   - The frame rule's `root_lands_empty` reads the gauge. A root lands at the empty chain on the part's world only when its gauge reference leaves the cut and its offset is the empty chain.
5. **Inline's gauge** (A4: "the instance's frame becomes a gauge under the instance's gauge holding its offset").
   - **When:** the instance is its group's root and its offset `o` is not the empty chain.
   - **What it builds:**
     - A `Promote` of the instance before the carry: the gauge `{ parent: host_gauge, placement: o }`, with every member of the instance's host group on it, so their placing mates still place. `regauge` sends the part's world to it.
     - The rebinds then move each host mate onto the inner member it reads, already on the gauge, and the edit list is the record (replay needs no solve).
   - **A moved member with a further offset** refuses with a new arm, `InlineError::MovedMemberOffset { instance }`, recourse "clear {i}'s offset (SetOffset), then inline". Its offset was stated in the instance's gauge, and the member now sits in the new gauge's frame. `UnplaceableFrame` stays: plain recipe geometry sits on no gauge, minted or not. `NeedsAGauge` goes.
6. **The mate-placed inline** (A4: "an instance its mates place is inlined only when its part is one group at the empty chain on its world"). "One such group": one group, root at the empty chain on the part's world, no gauge, no other member carrying an offset.
   - When the instance is not its group's root and the part is one such group, the part's root takes the instance's offset, which is `None` or the checked offset it carries, on the instance's gauge.
   - The host mates that read the instance rebind onto the inner members. The frame and fold rules hold as for any inline.
   - Otherwise it refuses `MatePlaced`, narrowed to that case. Its recourse is the P2 ruling's: "set the part's root offset to the empty chain, or place the instance, then inline". Where the part is one group and only its root's offset or gauge is wrong, name the first remedy concretely (`SetOffset` in the referenced document, then `UpdateReference`). Otherwise name the second, as P2-core's text does.
7. **A `FromFace` side crosses the seam with its head** (Ev, `[ev]` #3888).
   - A `FromFace` frame names no face. Its frame is the canonical pose of the face its own head names in the member's part: the head with the member walk's qualifiers stripped, one `Instance(i)` per pattern level and then the instance's `InPart`. The member walk is the strip's one home: it reaches that name as it reaches the member, `resolve_side` reads it off the walk, `mate::head_face` answers it for a head, and the viewer's mate tool reads its pre-check's name off the same walk (`mate::member_reading`).
   - Split's and inline's head rebinds therefore carry the frame, and nothing re-spells it. `MateFaceFrameCrosses` is gone at both seams, and no edit writes a mate's frame in place.
   - `Rebind` of a head carries its frame the same way (`work/place/a-from-face-frames-face-is-never-rewritten-by-rebind.md`).
   - On the wire a face side is `{"base": "Face", "offset": <Placement>}` (the mate frame became a base and an offset, `[ev]` #3920). A file holding an older spelling — the bare `"FromFace"`, a face tag over a payload, or `{"Authored": …}` — refuses `Unreadable` with the regenerate recourse.
8. **The frame rule: one predicate, in `frame_survives`.** A mate side crosses the seam only when:
   - (a) it reads its member at the member's own instance, with no pattern copy and no placer between;
   - (b) that instance lies, in the part, in a placed group, not in a group's own space (a dead gauge reference refuses before the rule is asked: `DeadGaugeReference` at split, `PartDeadGauge` at inline); and
   - (c) the instance is its group's root at the empty chain on the part's world.

   An `Authored` side is held to (a), (b) and (c).

   A `FromFace` side is held to (b) alone (Ev, `[ev]` #3888): its frame is its head's face in whichever part the head names, and the head crosses with it, so the frame moves only if the member's place in the part's world does. A4's frame-rule sentence states it.

   Dropping (a) and (c) for a face side is deliberate, a copy read included. The head of a side on a pattern copy names that copy's face, through the copy's `Instance(i)` qualifier, and the head crosses as it is. So the face the side reads after the seam is the same face, in the same place.

   The fold rule (`MatePairSplits`) is unchanged.
9. **`carry` leaves carried offsets as they were.** The part (or the spliced host) holds exactly the source's offsets, and the edit list replays to it without a solve. Whether carried mates skip the mate door's clear or the next edit re-states each cleared offset (`SetOffset`) is the unit's call; justify it against replay. No `OffsetCleared` for a carried node survives in the outcome's maintenance.
10. **Every refusal states one recourse** and has its row in `refusal_concision_refactor.rs`. Remove `CutHoldsGauge`, `NeedsAGauge` and both seams' `MateFaceFrameCrosses` from the rosters, and add `SeveredGauge` and `MovedMemberOffset`.

**Rows.** Each is red on `origin/main` (or absent there, where the door is new), then green. `S`, `I`, `F` and `C` mark split, inline, face and carry.
- **S1. The gauge hoist.** Retired with ruling 3; its result is row P4.
- **S2. A gauge moved verbatim.** K plus an instance on the anchor: K is carried with parent world, and the instance sits at the empty offset. Red: `CutHoldsGauge`.
- **S3. The severed gauge.** A kept instance on a cut gauge, and a kept gauge whose parent is a cut gauge, each refuse `SeveredGauge` naming both nodes, with the recourse. Red: `CutHoldsGauge`.
- **S4. The vote.**
  - K on g with an instance on the world in the cut refuses `TwoAnchors`.
  - A bare cut gauge whose parent was deleted refuses `DeadGaugeReference`.
  - A cut group unplaced for lack of an offset, on a gauge other than the anchor, refuses `TwoAnchors` (D2). Admitted on main, which loses the gauge.
  - Red: `CutHoldsGauge`.
- **S5. The frame rule with a gauge in the cut.** A kept declaring mate whose cut side reads a root at the empty chain, its gauge reference leaving the cut, crosses. One reading a root on a cut gauge K refuses `MateFrameCrosses`. Red: `CutHoldsGauge`.
- **S6. Would start placing.** A kept instance on the anchor declaring against a cut instance refuses `WouldStartPlacing`. Red: `CutHoldsGauge`.
- **S7. A parametric hoisted gauge.** Retired with ruling 3. A parametric gauge moved verbatim is in R1; a parametric root offset kept in the host is a promote before the cut (`p2_gauge_offsets_and_spaces.rs`).
- **I1. Inline's gauge.**
  - A root instance at offset o on gauge g, over a part of two groups, inlines: the minted gauge has parent g and placement o, and both groups hang from it.
  - Every body's world pose equals the pre-inline evaluation.
  - The same holds for a part whose root sits at a non-empty offset, and for a part holding a gauge.
  - Red: `NeedsAGauge`.
- **I2. The members move.** A host member that the instance places through a mate moves onto the minted gauge, its mate reads the inner root, and its world pose is unchanged. Red: `NeedsAGauge`.
- **I3. A moved member with a further offset** refuses `MovedMemberOffset`, with the recourse. Red: `NeedsAGauge`.
- **I4. The mate-placed inline.**
  - An instance mated onto a host base, over a part that is one group at the empty chain, inlines: the part's root takes the instance's place, with no offset, on its gauge; the host mate reads it; and world poses are unchanged.
  - With a checked offset on the instance, the root carries it.
  - Red: `MatePlaced`.
- **I5. The mate-placed refusal.** The same over a part whose root sits at a non-empty offset refuses `MatePlaced`, naming the remedy. It holds on main; the guard is that the lift stays narrow.
- **F1. A face side reading a non-root member crosses split and inline** (`p2_face::a_face_side_reading_a_non_root_member_crosses_split_and_inline_unmoved`).
  - A kept declaring mate's face side reads the second member of a placed group. Split re-anchors its head through the instance, and inline unwraps it back onto the inner member.
  - At each of the three documents the side's resolved world frame (`face_pose` through the member's part, carried by the member's solved world pose) is the pre-seam one, bit for bit.
  - `inline(split(d))` is `d` up to node ids under R1's comparator, the mate's face sides included.
  - The authored twin refuses `MateFrameCrosses`.
- **F2. A face side reading an unplaced member refuses `MateFrameCrosses`** at split (`p2_face::a_face_side_reading_an_unplaced_member_refuses_mate_frame_crosses`). Inline's mirror is not a document the doors build: a part's product table carries no row for a body in an own space, so the host side's own insert refuses `NoSuchName`, and the row pins that.
- **F3. A face side on a pattern copy reads its master's face, at the copy** (`p2_face::a_face_side_on_a_pattern_copy_reads_the_masters_face_at_the_copy`).
- **F3b. A face side on a pattern copy crosses split and inline** (`p2_face::a_face_side_on_a_pattern_copy_crosses_split_and_inline_unmoved`).
  - A kept declaring mate's face side reads copy 2 of a three-copy pattern. Split cuts the leg and its pattern, and the side then reads the instance plainly. Inline unwraps it back onto copy 2 of the inner pattern.
  - At each of the three documents the side's world frame is the pre-seam one, bit for bit. The copy's translation carries it where the side reads a copy.
  - `inline(split(d))` is `d` up to node ids under R1's comparator.
  - The authored twin refuses `MateFrameCrosses` at split, by (a).
- **F3c. A head that names no face of its part refuses `NoPartFace`** at the insert door, with its recourse (`p2_face::a_head_naming_no_part_face_refuses_no_part_face_at_the_door`). The head is a face name at the instance with no `InPart` wrapper.
- **F4. `Rebind` carries a face side.**
  - A part-side rename, `UpdateReference`, then `Rebind` of the head: the side faults `NoSuchName` between, and after the rebind the mated block sits on the rebuilt face (`p2_face::a_rename_update_and_rebind_carry_the_face_side_with_the_head`).
  - A head rebound onto another part's instance reads that part's face (`p2_face::a_head_rebound_onto_another_parts_instance_reads_the_new_heads_face`).
- **F5. The wire.** A face side saves as the bare tag. A file whose face side carries a payload refuses `Unreadable` with the regenerate recourse, whether the payload names the head's face or another, or is `null`, `{}` or `[]` (`msolve9_from_face::an_older_file_naming_its_face_refuses_unreadable_with_the_recourse`).
- **C1. The carry keeps offsets.**
  - A verbatim split of a group whose member carries a checked offset keeps it in the part, and `part_maintenance` holds no `OffsetCleared`.
  - The same holds at an empty-offset inline.
  - Red on main: the probe above.
- **R1. The round trip.**
  - `inline(split(d))` equals `d` up to node ids for every shape split admits: one placed group, one with a checked member, a lone instance, a group at a parametric offset, D1's two shapes, a gauge on a kept gauge holding two groups and a gauge, a group on a kept gauge, plain geometry, S2, C1 and the nested-gauge shapes.
  - Compare through the composed node map: node payloads (gauge references, offsets, mate alignments and heads), the root list in order, parameters, and labels.
  - Build the comparator once, in the test substrate, as an oracle independent of the code under test: it reads each node through the composed node and step maps and never calls `refactor::remap_node`.
  - F1 and F3b join R1: `inline(split(d))` is `d` up to node ids with the mate's face sides included.
- **P1. `fold ∘ promote`.** A placed pair on a gauge g: the promoted gauge sits on g holding the root's offset, ahead of the root in the root list; the group sits on it, the root at the empty chain; the mate still places; nothing moves; the fold returns the document up to node ids; both replay with no reach.
- **P2. `promote ∘ fold`.** K on g with one instance at the empty chain: the fold puts it on g at K's placement step for step, and the promote mints K back up to node ids. A gauge on K gets K's steps ahead of its own, an instance on K with no offset keeps none, and a lone unlabelled dependent takes K's label.
- **P3. A part at this frame.** Promote a placed pair's root and cut the group leaving the gauge: the part's root at the empty chain on its world, the instance on the promoted gauge at the empty offset, the evaluation unchanged, and R1. A kept declaring mate reading the root refuses `MateFrameCrosses` before the promote and crosses after it.
- **P4. Content, then fold.** K on g with two groups and K2 under it: the cut of K's content anchors on K, and folding K gives the instance K's placement as its offset; K's instances sit on the part's world at their offsets, K2 hangs from the part's world, and nothing moves.
- **P5. The refusals.** Each of `PromoteOnNonInstance`, `PromoteWithoutOffset`, `PromoteNonRoot`, `PromoteMemberOffset`, `FoldOnNonGauge`, `FoldWouldDangle` and `FoldWouldStartPlacing` names its nodes and a recourse, and taking the recourse moves forward: the edit succeeds, or refuses an arm not yet passed.

**Mutants** (plant, run, revert; report which rows go red):
- the severed-gauge check skipped (S3);
- a cut gauge casting no vote (S4);
- `root_lands_empty` without the gauge condition (S5);
- the minted gauge's parent set to the world (I1, with g not the world);
- the members not moved onto the minted gauge (I2);
- the further offset ignored (I3);
- the mate-placed admission without "one such group" (I5);
- the strip reading the wrong qualifier level (F3);
- `frame_survives` holding a face side to (a) (F3b);
- a head that names no part face read as one (F3c);
- `frame_survives` holding a face side to (c) (F1);
- `frame_survives` not holding a face side to (b) (F2);
- `Rebind` not rewriting a mate's heads (F4);
- a face payload loading, `null` included (F5);
- the carry's re-statement skipped (C1, R1);
- the group hoist restored for a one-group cut (R1);
- `Fold` not composing the gauge's steps (P1, P2, P4);
- `Promote` dropping the parent (P1);
- `Fold`'s start-placing refusal skipped (P5);
- `Fold` skipping the consumer check (the dangle row);
- split admitting a root that sits on no gauge at a gauge anchor (R1's refusal row);
- the chain folded placement by placement rather than step by step (P2 and P4 under rotations);
- `Promote` asking for an offset before root-ness (P5);
- the comparator reading roots as a set (its root-order self-test).

**Sweep.**
- **Datums the bridge does not rewrite.** List every `StableName` or `FaceName` field a node holds outside `payload_names`: these are the names split's and inline's bridge cannot rewrite. Give each one's disposition: a `FromFace` frame holds none (ruling 7), and `InterfaceCrossing::Mate`'s `inner` is already remapped. State what that pattern misses: a name nested inside a non-name datum. Check that gap with a second pass over `remap_node`'s arms, which copy fields verbatim.
- **Side effects the carry triggers.** List every side effect `apply`'s `InsertNode` arm has beyond minting: `roots::on_insert`, which split and inline already override with `SetRoots`, and `clear_joined_offsets`, ruled here. Say for each whether `carry` must undo it.

**Seams.**
- PLACE owns `refactor.rs`.
- RECIPE: `edit.rs` (`Promote`, `Fold`, their `EditError` arms), `resolve/mod.rs` and `persist/check.rs`, the `DocEdit` match sites.
- MSOLVE: `mate.rs` (`MateFrame::FromFace`, `FaceRefusal::NoPartFace`), `mate/member.rs` (the walk, `head_face`, `member_reading`) and `mate/solve.rs` (`resolve_side`); M1 (`work/msolve/a-mate-frame-is-written-in-the-reading-instances-coordinates.md`) cites ruling 8's face branch, which leaves M1 the authored arm.
- LIB/BIND: `pncad-py` `edit_payload.rs`, `py/doc.rs`, `py/mate.rs` (`MateFrame.from_face()` takes nothing, and its `face` getter is gone), `mate_payload.rs`, `tags.rs` (`no_part_face`), and the stubs, census and tag inventory.
- The viewer: `session.rs`'s `DocEdit` match, and `matetool.rs`, which reads the name it pre-checks through `member_reading`.
- TCOST/TINT: `p2_gauges.rs`, `msolve9_from_face.rs`, `msolve10_door_admission.rs`, `refusal_concision_refactor.rs`, `tests/corpus/mod.rs`'s `DocEdit` census and `refusal_concision_edits.rs`.

Announce each crossing in the PR body.

**Review.** P2-carry: a single full review. P2-split and P2-face: dual, a concurrent Opus pair (`docs/DUAL-REVIEW-PROTOCOL.md`), class H.

## P3: viewer (filed, not EDIT's)

- G3's free-move probe is widened to a whole group.
- A group that becomes unplaced is drawn where it was last shown: the probe is seeded from the prior draw and discarded when the group's key changes, undo included.
- "Place where shown" is one edit whose frame comes from the gesture.
- This is filed on the viewer owner's slate when P2 merges, citing this file.

# EDIT-PLACEMENT: gauges, and a placement that is parametric

This is the unit slate for `work/edit/placement-is-spelled-three-ways-node-registry-and-rule.md`. It follows Ev's rulings on `[ev]` #3437 and `[ev]` #3441, both from 2026-09-29, and `[ev]` #3505 (2026-10-01), which are now A4, A9 and A11 (2)–(5) in `crates/editor-core/ASSEMBLY.md`. Read those clauses and the row's `RULED` section first: they are the contract. The designer reports behind them are rows 17–19 and 21 of `docs/DESIGN-FORK-LOG.md`.

The ruling reaches the data model, the mate solve, evaluation, persistence, export, Python and the viewer, so it ships as three units in order. Each one leaves main green and correct. None of them builds half of a later unit.

| # | Unit | Branch | Review |
|---|---|---|---|
| P1 | The `Placement` type, held by `Node::Transform` | `edit/placement-type` | dual (`docs/DUAL-REVIEW-PROTOCOL.md`) |
| P2a | Vocabulary: the code's cluster becomes a group and its representative a root, freeing "gauge" | `edit/group-root-vocabulary` | orchestrator's read |
| P2 | Gauges; the registry and maintenance go; own space; STEP refusal | `edit/placement-gauges` | dual |
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
   - `parent` is a reading edge, so the gauge chain is acyclic by the DAG's own check.
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
   - The solve runs in group-local coordinates and never reads a gauge frame (`pair_left_factor` stops reading the registry). So a parametric gauge frame does not widen the nominal-only solve class (`work/msolve/a-mate-through-a-parametric-placer-…`).
   - World pose = the gauge chain's frame ∘ the root's offset ∘ the solved relative pose. It is evaluated in every lane, and the gauge chain and the offset feed the memo key.
   - **Checked offsets.** A non-root member's offset is a checked statement. It is verified against the solve within the solve's tolerance, never ignored. A mismatch faults that instance, typed, naming the offset, with the recourse "clear the offset, or change the mate".
4. **Edits.**
   - `SetPlacement` goes. `DocEdit::SetOffset { instance, offset: Option<Placement> }` and `DocEdit::SetGauge { node, gauge: Option<RecipeNodeId> }` (an instance's gauge or a gauge's parent) replace it. Each is refused, typed, on the wrong node kind, with a recourse.
   - **The mate door.** Inserting a placing mate that joins two groups clears the offset of the `b` side's group root in the same edit. The mate places `b`'s group on `a`'s. So the merged group keeps one root, and a freshly inserted part never carries a stray checked identity offset. This is a compound edit, recorded as its edits, and replay re-applies them without solving.
   - **"Copy x's gauge to y, then mate"** is one compound edit: `SetGauge` on every member of `y`'s group, then the mate insert above. Provide it as one door in `editor-core` and bind it in Python.
   - Deleting a gauge, a placed member or a placing mate is never refused. What stays placed is recomputed.
   - **References to a deleted gauge are kept, dangling.** Then the unplaced group can name its cause, and split can refuse a dead reference as A4 says; the recourse names `SetGauge`. Check `roots::check` and the DAG's invariants against dangling reading edges, and state what you changed.
5. **What goes.**
   - `Doc::placements` and `Doc::placement`;
   - `ClusterMaintenance`, `Maintain`, `maintain`, `reconcile`, `registry_after`, and the edit door's maintenance solve. `apply` keeps the reach only for the mate insert's clocking rider (`admit_mate`);
   - `LoggedEdit.maintenance`: the log becomes plain edits, and `LoggedEdit` goes if nothing else needs it;
   - `EditError::MaintenanceRefused` and `MaintenanceUnrecorded`, `PersistError::MaintenanceFrame`, and `SnapshotError::PlacementSite` and `PlacementNotGauge`, with every match site (`refactor.rs` ReplayTail ~857–879, `pncad/src/workspace.rs` ~784, `pncad-py` `tags.rs` and `py/doc.rs`, `edit_payload.rs`);
   - Python's `Doc.placement`, `Doc.placements`, `Doc.last_maintenance`, the `Maintenance` class and `DocEdit.set_placement`. Their replacements are `set_offset`, `set_gauge`, the gauge node and the compound door.
6. **The unplaced group** (A9, A11 (2), Ev's #3441 and #3505).
   - **One derived fact per group: its space**, either the world or its own. It is evaluated in its own frame with its tree root at the origin, and it solves and checks internally as usual.
   - **Product.** `product::product` gathers only world-space bodies. An unplaced group is not part of that body (A9).
   - **The at-rest gate** mints pairs only within one space.
   - **A measure** whose references lie in two spaces refuses, typed, naming the unplaced group, with the recourse "place it (a gauge, an offset or a placing mate)". It never answers a number across spaces.
   - **STEP.** `export_document_step`, and `step_for_node` on an unplaced instance, refuse naming the unplaced parts, the cause (no offset, or the deleted gauge), and how to place them (A11 (2)).
   - No other door changes behaviour. Show this with a sweep: list every reader of world poses and state which space it reads.
7. **Split and inline: A4 as written.** The clause is the contract; build it. In particular:
   - split's refusals: `TornGroup`, two anchors, a severed gauge, a declaring mate that would start placing, a dead gauge reference, and a cut of unplaced material alone;
   - the two hoist shapes;
   - inline's gauge, and its sugar for a one-group part at the empty chain;
   - a mate-placed instance inlined only when its part is one group at the empty chain (otherwise refused, named, with the recourse "set the part's root offset to the empty chain, or place the instance, then inline");
   - a moved member's further offset refused;
   - the frame rule and the fold rule, each refusing the mate by name;
   - declaring crossing mates filling `InterfaceRecord` (AQ8).

   Every refusal is typed and carries a one-edit recourse (the refusal standard). Replace `InlineError::UnplaceableFrame` only if a new arm states the same refusal more truly.

   The defects on main that the designers found close here, each pinned by a row:
   - inline reads the group's frame, not the instance's pose;
   - inline rebinds mate heads without re-coordinating their frames.
8. **Cross-gauge contact at the gate.** A declaring mate across gauges goes through the gate's existing declared-contact path. P2 neither widens nor narrows what the gate can certify; today that path answers `Uncertified` for instance pairs. Report what a cross-gauge declaration yields on your head. If no tracker row already owns the `Uncertified` limit, file one on MSOLVE.
9. **Persistence.**
   - `Doc` loses `placements`. Instances gain their two fields, and the gauge node gets a wire shape.
   - An old file refuses, typed (`Unreadable`, with the regenerate recourse); it never loads with a different meaning.
   - A dangling gauge reference loads; it is a legal state.
   - Regenerate every file the change moves from its source, and name each one with what moved it. Re-baseline every pin that moves and list them in the PR body. A golden that changes is never a cost to weigh against the change (`docs/prompts/implementer-discipline.md`).
10. **The placement refusal arms** on `work/edit/edit-refusals-short-of-the-shape-guard.md`.
    - The row says seven. Five remain on main: `EmptyPlacementList`, `MaintenanceUnrecorded`, `PlacementAxis`, `PlacementOnNonInstance` and `PlacementRuleMismatch` (`viewer/tests/refusal_concision_edits.rs` ~863).
    - Remove the ones P2 deletes. Rewrite the others P2 touches so they carry a recourse, and update both the census and the row.
    - `PlacementRuleMismatch` and `EmptyPlacementList` concern the explicit placement rule, not the registry. Leave them unless P2 touches them.

**Rows.** Each is red on `origin/main` (or absent there, where the door is new), then green:
- an instance on a gauge under a gauge is placed by the composed chain, and a document parameter driving the outer gauge moves both levels;
- inserting an instance places it at the world origin, and mating it to a placed instance clears its offset in the same logged edit, which replays without solving;
- the compound "copy x's gauge to y, then mate" re-gauges y's whole group;
- a checked offset that disagrees with the solve faults that instance, typed;
- deleting a gauge, a placed member or a placing mate is not refused, and the group becomes unplaced: it evaluates in its own frame, the gate mints none of its cross-space pairs, a cross-space measure refuses typed, and STEP export refuses naming the parts and the cause;
- the A4 table, one row per arm: every split refusal and both hoists; inline's gauge, its sugar, its mate-placed admission and refusal, and the moved-member refusal; and the frame-rule and fold-rule refusals;
- inline-of-split returns the document split was given, up to node ids;
- a declaring mate crossing a cut fills `InterfaceRecord`;
- the two main defects above, red on main;
- an old file refuses, typed, and a dangling gauge reference saves and loads;
- Python: a `threading.Thread` row over the new doors, and the census.

**Mutants** (plant, run, revert; report which rows go red):
- the solve reads the gauge frame;
- the mate door forgets to clear `b`'s root offset;
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

## P3: viewer (filed, not EDIT's)

- G3's free-move probe is widened to a whole group.
- A group that becomes unplaced is drawn where it was last shown: the probe is seeded from the prior draw and discarded when the group's key changes, undo included.
- "Place where shown" is one edit whose frame comes from the gesture.
- This is filed on the viewer owner's slate when P2 merges, citing this file.

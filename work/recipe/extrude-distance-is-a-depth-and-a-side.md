---
id: extrude-distance-is-a-depth-and-a-side
kind: issue
title: An extrude's distance becomes a positive depth with a structural side (Ev, #3551), and a negative depth refuses with a recourse that names side
status: closed
opened: 2026-10-01
priority: P0
cost: H
branch: recipe/extrude-depth-and-side
pr: 3912
closed: 2026-10-03
---


**Ruled by Ev on #3551 (2026-10-01).** The decision record is AUTHOR's closed row `a-negative-extrude-distance-probes-as-valid` (`git show 29b8874a1:work/author/a-negative-extrude-distance-probes-as-valid.md`), its "A depth and a side" and "Ruled" sections. The designers' reports are on #3551. Fork-log row 22.

## What to build

- **Node.** `Node::Extrude { profile, distance, side }`, where `side` is structural: along or against the sketch normal. The `distance` is a depth.
- **Kernel door.** `sweep::Extrusion::Distance` takes the same pair. The door refuses a depth that is not definitely positive through its existing `extrusion_normal_component` decision: one rule in one place, the pattern the tube, blend and shell doors follow.
- **The refusal names its recourse (Ev's requirement).** A negative depth refuses with a sentence that shows how to write the extrude the other way, i.e. set `side`, not merely "the depth is not positive". A zero or ε-small depth keeps its own refusal (`DegenerateExtrusion`). Only the negative arm gains the recourse.
- **Persistence and edits.** `side` is a structural parameter, so `deny_unknown_fields`, a schema bump, `SetStructuralParam` (or its equivalent) and `pncad-py`'s `Node.extrude` all change.

## Knock-ons, measured by the designers on #3551

- `crates/editor-core/tests/docm9_range.rs` A2 (`a_branch_change_is_a_decision_flip_and_names_the_predicate`, `a_decision_flips_within_contains_a_value_that_does_not_build`) uses the depth-through-zero slab as the suite's ONLY `DecisionFlip` fixture. It needs a replacement flip that builds on both sides.
- `crates/viewer/tests/docm9_range_vs_probe.rs` ("the extrusion runs the other way through zero") needs re-wording.
- Tests that author negative distances on purpose need re-spelling with `side`:
  - `editor-core/tests/{m4_pr3_names, emit_boolean_vertex_keys, m4_pr2_eval_interval, review_m4_pr2}.rs`;
  - `sweep/tests/{extrude_acceptance, review_m2_pr4, review_m2_pr7, fillet_h6_cap_rim, bitdump}.rs`.
- The corpus `.pncad` files use no negative extrude.
- **After it lands**, `crates/viewer/tests/valid_range.rs`'s floor row and the vacuity row's "brackets only by landing a doubling" note stop depending on luck: every seed brackets a half-line floor at every ε. A row showing that the 0.8 mm and 1 m seeds both bracket would be real anti-vacuity evidence.

## Ground

- EDIT: `node.rs`, `edit.rs`, persistence.
- CARVE/STRUT: `crates/sweep/src/extrude.rs`.
- WIRE: `eval/wire.rs` `wire_extrude`.

The viewer half is DOORS' `work/doors/the-create-pane-has-no-extrude-side`, which is blocked on this row.

Filed by the AUTHOR orchestrator on Ev's ruling.

## Built (2026-10-02, PR 3912)

- **Kernel.** `sweep::ExtrudeSide { Along, Against }` and `Extrusion::Distance { depth, side }`. The door decides the depth through `extrusion_normal_component`, as before: `Zero` stays `DegenerateExtrusion` (so does an ε-small negative), a definite negative is the new `ExtrudeError::NegativeDepth { side, depth }`, and the side alone picks `±n`. Its sentence quotes the value the depth evaluated to and names the recourse true whatever drives it: "make the depth evaluate positive (its sign comes from whatever drives it, a literal, a parameter or an expression) and set the side to against the sketch normal" (or along). It meets `test_utils::refusal::problems`, and the recourse followed builds the vector door's body bit for bit (`extrude_acceptance::a_negative_depth_refuses_naming_the_other_side`); the parameter-driven case is `m4_pr2_eval::a_parameter_driven_negative_depth_refuses_with_a_recourse_that_builds`. `verbs::Verb::Extrude` carries the pair.
- **Node, edit, persistence.** `Node::Extrude { profile, distance, side }`. The side is recipe payload outside the slots, so the evaluation memo's content key writes it as its own word (`extrude_side_tag`, key format v9), and every arm of that match names its variant's fields, as `node_rows` and `Node::inputs` now do, so a field added to a node does not compile until it is stated keyed, a slot or an edge; `m4_pr2_eval::a_side_flip_recomputes_its_cone_and_undo_restores_the_body` and its python twin pin it. `side` persists as `"along"`/`"against"` through `persist::kernel_wire::extrude_side` and is a required field, so a document from before refuses `Unreadable` naming `side` (the schema bump). The structural edit is its own arm, `DocEdit::SetExtrudeSide { node, side }`, refusing `SetExtrudeSideOnNonExtrude` on another kind; the corpus's `kitchen_sink` exercises it. `pncad-py`: `ExtrudeSide`, `Node.extrude(profile, distance, side=ExtrudeSide.Along)`, `DocEdit.set_extrude_side`, the tags `negative_depth` and `set_extrude_side_on_non_extrude`, stubs and a python row.
- **Viewer.** `SessionOp::AddExtrude` builds along the normal; a negative depth typed there now refuses at evaluation with the recourse. The side control is `work/doors/the-create-pane-has-no-extrude-side`.
- **Knock-ons.** `docm9_range`'s `DecisionFlip` fixture is a notch (a profile vertex through collinear, built on both sides); the M10 driver suite's planted flip and containment chamber moved to the same notch. The negative-distance tests are re-spelled with `side`. `valid_range` gained `every_seed_brackets_the_half_line_floor`: the 0.8 mm, 1 mm and 1 m seeds all bracket.
- **What it widened elsewhere.** A depth below zero now refuses on every sub-box, which `drive::classify_replay` bisects to the floor and prices `Budget`: evidence appended to VERDICT's `coincidence-zone-priced-budget-at-the-floor`.

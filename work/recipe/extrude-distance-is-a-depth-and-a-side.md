---
id: extrude-distance-is-a-depth-and-a-side
kind: issue
title: An extrude's distance becomes a positive depth with a structural side (Ev, #3551), and a negative depth refuses with a recourse that names side
status: open
opened: 2026-10-01
priority: P0
cost: H
---


**Ruled by Ev on #3551 (2026-10-01).** The decision record is `work/author/a-negative-extrude-distance-probes-as-valid.md`, its "A depth and a side" and "Ruled" sections. The designers' reports are on #3551. Fork-log row 22.

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

The viewer half is AUTHOR's `the-create-pane-has-no-extrude-side`, which is blocked on this row.

Filed by the AUTHOR orchestrator on Ev's ruling.

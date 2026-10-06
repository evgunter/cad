---
id: a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms
kind: issue
title: A boolean result ships contact records its geometry no longer confirms, so the tier-3′ pass refuses it StaleContactDeclaration
status: parked
opened: 2026-10-02
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---


Found by the result-gate unit
(`a-boolean-result-gate-ships-a-scaffold-at-rest`), which measured
`validate_pseudomanifold(&body, &contacts)` on every result the door
builds (`boolean::ops::gate`'s four callers), at the default ε on the
`ci` profile.

## What

`BooleanBody`'s doc makes a result with non-empty `contacts` tier-3′
currency: `validate_pseudomanifold(&b.body, &b.contacts)` is its at-rest
gate. 40 results that pass tier 3 carry a record the census cannot
confirm (`StaleContactDeclaration`): the declared touching it names is
no longer a touching of the result. The door ships the record anyway.
The remap that carries records into result keys (`boolean::ops`
`remap_contacts`, `remap_carried`) keeps a record whose two sides both
survive, without asking whether they still meet.

| results | site | test |
|---|---|---|
| 1 | seamed | `topo` `boolean_covered::a_discarded_face_holds_the_edges_of_the_kept_face_that_runs_into_it` |
| 1 | seamed | `topo` `boolean_covered::a_held_edge_goes_only_to_the_fragment_it_enters` |
| 8 | fallback, two-operand arm | `sweep` `reach_wall_chord_rows::a_cube_touching_a_drum_at_a_corner_answers_its_closed_form` |
| 7 | fallback, two-operand arm | `editor-core` `docm7_union_declare::a_same_member_declared_pair_is_a_carried_record_at_its_step` |
| 8 | seamed | `editor-core` `emit_union_flush_names::a_boss_on_one_piece_of_a_covered_face_is_cited_in_no_order` |
| 2 | seamed | `editor-core` `emit_union_flush_names::a_cut_a_covered_face_meets_is_one_divider_in_every_order` |
| 2 | seamed | `editor-core` `emit_union_flush_names::a_seam_a_leftover_vertex_splits_is_published_twice_under_two_names` |
| 11 | seamed | `editor-core` `emit_union_rim_piece_ranks::a_flush_union_publishes_one_table_in_every_member_order` |

The common shape is a declared flush pair that the union consumes: the
glued faces merge, or the declared region ends inside the result, and
the record outlives the touching.

## The fix

Drop or refuse a record at remap time when the result no longer
carries its touching, and pin it on one row above. Gating tier 3′ at
the door (REACH's `boolean-door-tier-3-waits-on-the-description-gap`,
`[ev]` PR 3870) refuses all 40 until this lands.

## More evidence (2026-10-04, PR 4026's review r2, N1)

A single-run convex corner meets a cube's face in
`join_pierce_r2_probes.rs` `r2_shapes_battery` (branch
`join/pierce-two-out-runs-review-r2`), shape `Lcvx`, undeclared. Ten
results ship a `StaleContactDeclaration { VertexOnFace }`, identical on
main `45dc18f9` and on PR 4026's head:
`g16.1 pc I`, `g16.2 pc I`, `g16.3 pc I`/`cp I`, `g17.0 pc I`,
`g17.1 pc I`, `g17.2 pc I`/`cp I`, `g17.3 pc I`/`cp I`. These are
vertex-on-face records, not the flush pairs above. The undeclared
vertex-on-face findings of the same shape are
`a-kissing-convex-corner-result-ships-an-undeclared-vertex-on-face`.

## The drum rows were the census, not the record (2026-10-06, TANG)

The 8 `reach_wall_chord_rows` results above were not stale records. The
census confirmed a vertex-on-face record against planar faces only, and
the drum's wall is a cylinder, so the record read stale whatever the
geometry. On `tang/lying-on-arc-splits-at-a-ruling`,
`census::confirm_vertex_on_face` confirms one on a curved face through
the curved containment door. The drum ∖ the inner cube (a void whose
corner touches the wall) then passes tier 3′. The drum ∪ the outer cube
is left with only the undecidable cross-solid curved pair. The row now
pins both. The other 32 results were not re-measured. The change can
move one of them only where its record names a curved face.

---
id: boolean-door-runs-the-census-over-its-result
kind: issue
title: "The boolean door gates its result at tier 3 but not the census: a census at the door refuses 175 results on main, 38 of them curved unions whose solids the cross-solid lane cannot tell apart"
status: parked
opened: 2026-10-03
priority: P1
cost: H
blocked_on: [census-cross-solid-curved-pairs-undecidable-on-shell-results, a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms, a-two-pinch-union-ships-a-pinch-its-records-do-not-declare, a-boolean-drops-its-operands-own-contact-records]
refs: [boolean-door-adopts-the-finished-body-type, boolean-door-tier-3-waits-on-the-description-gap]
---


The half of `boolean-door-adopts-the-finished-body-type` that did not
land with it. Ev's finished-body bar (`docs/DESIGN.md`, tier 3) is
tier 3′ against the body's own declared contacts, so the door owes
`T::gate_at_rest_declared` over `BooleanBody.contacts` after the kept
tier-3 gate (`ops::gate`). It does not run it, because a census at the
door refuses results the door ships correctly today.

## Measured (`origin/main` 82b9ceb2, ε 1e-9, `ci` profile)

The instrument is `python3 scripts/door-tier3-meter.py` (topo's
`door-tier3-meter` feature): it ran `T::gate_at_rest_kept` then
`T::gate_at_rest_declared` over every result `boolean_op_with` returned.
Results the census alone refuses (tier 3 passed):

| corpus | gated results | census refuses | by kind |
|---|---|---|---|
| topo | 1262 | 31 | 18 `UndeclaredContact`, 6 `CensusEscalated`, 5 `CensusUndecidable`, 4 `StaleContactDeclaration` |
| sweep | 2304 | 42 | 38 `CensusUndecidable` alone, 4 with `StaleContactDeclaration`, 4 more `StaleContactDeclaration` |
| editor-core | 10271 | 102 | 69 `UndeclaredContact`, 30 `StaleContactDeclaration`, 3 `CensusEscalated` |

(A result can carry several kinds, so the kind counts sum past the
refusals.)

- **`CensusUndecidable`, sweep (38).** Every one is a union whose result
  is several solids, a curved face of one within box reach of another:
  `snowman` ×9, `run_walls_built` ×4, `shell8_r2_probes` ×4,
  `m5_s13_pips` ×3 (a ball in a wall box's corner), `germ_sphere_no_crossings`
  ×2 (a ball in a torus's hole), `germ_torus_doors`, `germ_interior_oval`,
  `m5_s10_face_sense`, `m5_s11_concave_sense_interval`, `offer_rows` ×2,
  `verbs_cylcyl_*` ×2, `verbs_pierce*` ×4. The payload is "a curved
  face of one is within reach of the other, and the kernel cannot yet
  tell whether curved faces of two parts touch" — the census's
  cross-solid backstop (`census::sweep_cross_solid_backstop`), which
  `docs/DESIGN.md` ratifies as refusing ("cross-solid pairs with a curved
  side in reach refuse as undecidable"). Since pieces are sorted one solid
  per piece, every such union reaches it. This is CONTACT's
  `census-cross-solid-curved-pairs-undecidable-on-shell-results`, which
  the finished-body sequencing says lands before any door whose results
  carry several curved solids adopts the bar.
- **`UndeclaredContact` / `StaleContactDeclaration`** on pinch unions
  and flush unions (`union_pinch_member_order` ×42,
  `union_flush_onto_edge_contact`, `emit_union_flush_names`,
  `emit_union_rim_piece_ranks`, `docm7_union_declare`, `boolean_covered`,
  `reach_wall_chord_rows`, `m3_pr6_tier3prime::closure_kiss_vs_mover`):
  FUSE's and WIRE's filed findings, `blocked_on` above.
- **`CensusEscalated`** (`contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`
  ×6, `docm2_part_interval` ×3): unfiled until this row; cause not
  measured here.

## What closes it

The four `blocked_on` rows, then `ops::gate` takes
`T::gate_at_rest_declared` over the result's contacts after the kept
tier-3 gate, and the census predicates (`pm_census_*`) get words in
`topo::decision_words` or a reason in editor-core's `WORDLESS`
(`tests/edit_refusal_recourse.rs`), since they then reach every
evaluation's decision log. Measure again with the meter first.

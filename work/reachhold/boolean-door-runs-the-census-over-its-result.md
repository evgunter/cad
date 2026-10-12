---
id: boolean-door-runs-the-census-over-its-result
kind: issue
title: "The boolean door gates its result at tier 3 but not the census: a census at the door refuses 175 results on main, 38 of them curved unions whose solids the cross-solid lane cannot tell apart"
status: parked
opened: 2026-10-03
priority: P1
cost: H
blocked_on: [census-cross-solid-curved-pairs-undecidable-on-shell-results, a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms, a-two-pinch-union-ships-a-pinch-its-records-do-not-declare]
refs: [3987, 3870]
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
- **`CensusEscalated`**, by payload: `contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex`
  ×6 escalates `pm_census_ee_span` at margins −8.79e-9 and +8.29e-9,
  inside the `escalate` band 1e-8: two result edges whose spans end
  within 9 nm of each other, in a fixture built near-coincident on
  purpose. `docm2_part_interval` ×3 (`Interval`, a widened height)
  reads enclosures [−1.0e-9, 1.0e-9] and [−2.0e-9, 2.0e-9] that straddle
  zero: the parameter's own width. Whether either is the census's
  honest answer or a door defect is not decided here.

## What closes it

The four `blocked_on` rows, then `ops::gate` takes
`T::gate_at_rest_declared` over the result's contacts after the kept
tier-3 gate, and the census predicates (`pm_census_*`) get words in
`topo::decision_words` or a reason in editor-core's `WORDLESS`
(`tests/edit_refusal_recourse.rs`), since they then reach every
evaluation's decision log. Measure again with the meter first.

## Evidence: wrong bodies tier 3′ would have refused (ZIP, 2026-10-06)

ZIP's `zip-reflex` lane measured on main at `3f1e3b0d03`, before the REST
zip's admission fix (`work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume`).
The REST zip shipped `vol a + vol b′` at all 16 runs of what is now
`crates/sweep/tests/rest_zip_admission.rs`
`a_reflex_union_behind_a_join_lever_never_ships_the_overlap_twice`
(8 poses, both orders; renamed
`a_reflex_union_at_a_tangent_site_never_ships_the_overlap_twice` when
stage 4 A deleted the zip). 12 of those 16 passed the door's tier-3 gate and
failed `validate_pseudomanifold` over their own `BooleanBody::contacts`
when the probe ran it. The other 4 passed tier 3′ as well, so the census
would not have caught every one of them. The admission fix makes the zip
decline all 16, so the door no longer ships these bodies. It still shows
that the door's tier-3-only gate lets a body tier 3′ refuses reach the caller.

## Evidence: four near-tangent results the door ships (branch `cleave/near-tangent-invariants`, 2026-10-10)

PR 4581 retired the pierce-germ `ClassificationInvariant` that refused
four of JOIN's near-tangent census runs at ε = 1e-12. Those runs now
build their closed-form volume and fail tier 3′ definitely, which this
door does not see:
- `w345 nt e1 a13 d1e-8` and `d1e-9`, prism ∩ cube: two stale `VertexOnFace` records each.
- `w345 nt e1 a14 d1e-8`, cube ∩ prism: two stale `VertexOnFace` records.
- `w345 nt e1 a14 d1e-8`, cube ∖ prism: three undeclared contacts (`VertexOnFace` at `(2, 0, 1)` and `(2, 0, 0)`, an `EdgeFaceOverlap` at `(2, 0, 0.5)`).

`crates/sweep/tests/near_tangent_pierce_poses.rs` pins them at exactly
those findings (`SHIPS_A_FINDING`), so a door census that refuses them
turns that pin red, to be re-pinned at the refusal. The findings are
FUSE's (`work/fuse/a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms.md`,
where the full list is).

## Evidence: needles the door ships with an in-band pair since thinness is judged by pairs (FLUSH, PR 4783, 2026-10-12)

Ev's ruling on PR 4690 retires the door's sole-shell `V/A` gate (PR
4636): a result is too thin only when a pair of its cells is in band.
The door's own build decides the pairs it makes (a slab, a hollow's
wall or a fin below `Kε` refuses `VertexOnFace` or a loop's `Boundary`
at margin `t`). It reads no pair the census alone reads, so results the
retired gate refused for the wrong reason now ship, some holding such a
pair. The FLUSH orchestrator ruled against a stop-gap (a census over
`slivers()`-flagged results only would refuse a needle and ship a fat
body with the same pair): this row is the fix.
- **PR 4783's own probe**, ε 1e-9, `topo/tests/sliver_shell_role.rs`
  `a_needle_ships_an_in_band_edge_pair_until_the_door_runs_the_census`:
  the `notch307` piece ∩ the census witness cube (`nt e0 a3`) builds one
  lump alone in its solid, and `validate_pseudomanifold` over its own
  contacts escalates `pm_census_ee_span` at d = 1e-8, piece ∩ cube
  (margin 2.31e-9), d = 1e-8, cube ∩ piece (2.76e-9 and 1.45e-9) and
  d = 3e-8, piece ∩ cube (1.59e-9). A door census turns those rows red,
  to be re-pinned at the refusal.
- **R2's review**, ε 1e-9 (`crates/topo/tests/review_4783_r2_probes.rs`
  on `flush/thinness-by-pairs-review-r2` @ ef12bed5): of a 512-op
  notch307 family (a0–a15, d from 5e-9 to 4e-8, both orders), 14 ops
  change; **11** now ship with `pm_census_ee_span` in band (a3 d1e-8,
  1.5e-8, 2e-8, 2.5e-8 both orders; a3 d3e-8 piece first; a2 d3.05e-8
  both orders), 3 ship clean. The same family already ships **44** such
  results on base, so the class predates PR 4783.
- **R1's review**, ε 1e-6 (the topo `review-sliver-probe` feature on
  `flush/thinness-by-pairs-review` @ a79ba445): `rc_wide_battery` moves
  **34** lines from `ShellRole` to built (`OK BAD … operand=false`),
  census clean; 25 are check 10's pieces-sort class
  (`work/flush/check-10-reads-a-cavitys-role-from-its-volume-sign.md`)
  and 9 refuse `Containment` in the operand orientation probe
  (`work/cleave/a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face.md`).

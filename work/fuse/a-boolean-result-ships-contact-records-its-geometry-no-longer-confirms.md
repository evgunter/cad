---
id: a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms
kind: issue
title: A boolean result ships contact records its geometry no longer confirms, so the tier-3′ pass refuses it StaleContactDeclaration
status: closed
opened: 2026-10-02
priority: P2
cost: M
closed: 2026-10-13
pr: 4820
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
| 7 | fallback, two-operand arm | `editor-core` `docm7_union_declare::a_same_member_declared_pair_is_a_carried_record_at_its_step` (since stage 4 B2 it refuses `DeclaredContactUnbacked`, as `a_same_member_declared_pair_with_no_record_refuses_at_every_door`, and ships no record) |
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

## Measured (2026-10-07, PR 4207: a pinched operand)

r2's pinched-operand battery (`join_pinch_cones_r2_probes.rs`
`r2_pinched_operand_battery`, PR 4139's review r2, 720 lines), release,
main `3e9d1a96` and PR 4207's head `9d7b9228`. The battery's P is a cube
minus a smaller box sharing its corner, so P is pinched at the origin.
Its C is a cube whose near face holds that point.

- 120 results are `OK BAD` on tier 3′ alone, failing only on
  `StaleContactDeclaration { VertexOnFace }`: 64 lines with one stale
  record, 56 with two, 176 records in all. Volume, tier 2, the
  certificate and the legal-operand check pass, and every one meshes.
- The lines by order and op: `pc U` 36, `cp U` 37, `cp S` 32, `pc S` 6,
  `pc I` 5, `cp I` 4. They cover poses `i` = 1, 5, 6, 7, 9, 10, 12, 13,
  14, 18, 19, 20, 22, 25, 28, 32, 35, 36, 37, 43, 44, 47, 48, 50, 51,
  52, 53, 55, 56, 57, at both ψ = 0 and ψ = 1.1.
- Main holds the same stale records on the same 120 lines, line for
  line. On 73 of them main also refused `UndeclaredContact
  { VertexVertex }` at the pinch, which PR 4207 clears.
- Examples:
  - `i=1 ψ=0 cp S`: two records on face `2v1`;
  - `i=9 ψ=0 pc U`: `VertexOnFace { vertex: 22v1, face: 21v5 }`.

Repro: put the battery file (kept with PR 4139's review r2) under
`crates/sweep/tests/` with a `mod` line in `all.rs`, limit its pose
loop to `i = 1`, and print `validate_pseudomanifold`'s error in
`common/differential.rs`'s `outcome`. Then:

    cargo test --release -p sweep --test all -- --ignored --exact \
      join_pinch_cones_r2_probes::r2_pinched_operand_battery --nocapture

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: StaleContactDeclaration comes from the declared-contact records remap_contacts carries; stage 4 retires the declared-contact seats and records found contacts at the one door. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Evidence: near-tangent wedge poses (branch `cleave/near-tangent-invariants`, 2026-10-10)

JOIN's near-tangent census (`crates/sweep/examples/near_tangent_census_probe.rs`)
at ε = 1e-12: once `pierce_germ_dir` stopped refusing them
(`ClassificationInvariant`, a translation-dependent misreading), four
runs build their closed-form volume and fail tier 3′ definitely. The
door runs no census, so it ships them. Ruled acceptable by the CLEAVE
orchestrator at PR 4581's review, and pinned at exactly these findings
by `crates/sweep/tests/near_tangent_pierce_poses.rs` (`SHIPS_A_FINDING`),
which goes red when this row or
`work/reachhold/boolean-door-runs-the-census-over-its-result.md` lands:

- `w345 nt e1 a13 d1e-8`, prism ∩ cube: two stale `VertexOnFace`
  records, `VertexKey(2v1)` and `VertexKey(9v1)` on `FaceKey(19v7)`.
- `w345 nt e1 a13 d1e-9`, prism ∩ cube: the same two, on `FaceKey(19v9)`.
- `w345 nt e1 a14 d1e-8`, cube ∩ prism: two stale `VertexOnFace`
  records, `VertexKey(23v3)` and `VertexKey(17v3)` on `FaceKey(6v1)`.
- `w345 nt e1 a14 d1e-8`, cube ∖ prism: three undeclared contacts on
  `FaceKey(8v1)`: `VertexOnFace` at `(2, 0, 1)` (`VertexKey(16v3)`),
  `VertexOnFace` at `(2, 0, 0)` (`VertexKey(22v3)`), and an
  `EdgeFaceOverlap` of `EdgeKey(20v7)` witnessed at `(2, 0, 0.5)`.

Main already ships the same findings at the neighbouring pose
`w345 nt e1 a14 d1e-9`; the census counts 6 stale and 596 undeclared
findings at 1e-12 on main, and these add 6 and 3.

## Released by stage 4 F (2026-10-11)

Stage 4 F (`declared-pairs-retire`) retired every boolean declaration: no author can declare a pair, the glue is what the margins decide, and the declare offer, the flush detector and the carried declarations are gone. What this row waited on has fired; re-read it against that tree.

## Evidence: an off-grid wedge pose at ε 1e-6 (branch `cleave/single-on-edge-record`, PR 4630)

`w345 nt e1 a3.8 d1e-7` (3.8 sixteenths, off the census's integer
grid), prism ∪ cube, at ε = 1e-6: main refuses it
`ClassificationInvariant "bisector-graze crossing without a twin
record"`. Once the twin lookup tries the reference's other twin, it
builds its closed-form volume (to 1.2e-8), passes tier 2 and the
certificate, and fails tier 3′ definitely with one undeclared contact:
`EdgeEdgeCross { a: EdgeKey(16v1), b: EdgeKey(31v3) }` witnessed at
`(2.0, 1.0187e-5, 1.0)`. That is the far end of the prism's edge `e1`,
2 m from the corner, which lies within ε of the cube's face at that
tilt. The door runs no census, so it ships it. Ruled acceptable by the
CLEAVE orchestrator at PR 4630's review (a kernel-defect refusal on
valid input traded for this class's finding), and pinned by
`SHIPS_A_FINDING` (its ε row 1e-6) in
`crates/sweep/tests/near_tangent_pierce_poses.rs`.

Measured on main at `e7828dcb0e` (main's own `recl.rs`, ε = 1e-12): the
four near-tangent runs above (`w345 nt e1 a13 d1e-8` and `d1e-9`
prism ∩ cube, `a14 d1e-8` cube ∩ prism and cube ∖ prism) now build
their closed form and ship no tier-3′ finding. `SHIPS_A_FINDING` drops
them, and their poses stay in the pin's `POSES`, held to a clean tier
3′.

## Evidence: main red on that pin at ε 1e-6 (2026-10-11, FLUX cone-vector-area lane)

At main `436bfb2a4`, `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep
--test all -E 'test(/near_tangent_pierce_poses_build/)'` fails:
`w345 nt e1 a3.8 d1e-7, prism ∪ cube: the pinned findings moved: [the
EdgeEdgeCross above, witness (2.0, 1.018657088811564e-5, 1.0), margin
2.19e-15]`. It is red on main alone and on PR 4696's merge with it, and
green at ε default and 1e-12. The pose has no cone face. Not bisected.
The last change to the pin is `e15cbbb1b` ("the pin drops the four
findings main no longer ships"). Main's push CI does not run sweep's
1e-6 row, so only a PR that touches an eps-sensitive crate sees it.

## 2026-10-13, closed: fixed by INTENT stage 4 F and PR 4820 (FUSE, carry-rows-after-4820)

Measured on main `0d2117d0` with a temporary probe (never committed)
in `boolean_op_with` that ran `validate_pseudomanifold(body, contacts)`
on every body the door ships, over the topo, sweep and editor-core `ci`
suites (8,390 tests, about 18,000 results). Across all of it there is no
`StaleContactDeclaration`.

- **This row's tests:**
  - `boolean_covered`'s two rows: 6/6 pass.
  - `emit_union_flush_names`: about 300/300 pass.
  - `emit_union_rim_piece_ranks`: about 2,800/2,800 pass.
  - `near_tangent_pierce_poses`: 108/108 pass.
  - `reach_wall_chord_rows`' other rows: 172/172 pass.
  - `docm7_union_declare`'s test no longer exists; stage 4 F retired
    declarations.
- **The drum.** `reach_wall_chord_rows::a_cube_touching_a_drum_at_a_corner_answers_its_closed_form`:
  8 of 32 results fail tier 3′, on `CensusUndecidable` alone. That is the
  census's curved lane, not a stale record: it belongs to
  `work/fuse/census-curved-record-lanes-read-circles-only.md`.
- **The pinched-operand battery** (`join_pinch_cones_r2_probes.rs`,
  commit `66adba296c`), re-run in a scratch copy adapted to the current
  API: 676 bodies pass tier 3′, and the other 44 lines are correct empty
  results. The 120 stale lines measured on main `3e9d1a96` are now 0.
- **The `Lcvx` battery's ten stale poses** (`g16.1` to `g17.3`), every
  op in both orders: 60/60 pass.

Stage 4 F retired the declared pairs, so no record outlives a flush
pair any more. PR 4820's one carry lands each record only where the
census's record confirm reads the pair meeting (`records::landings`).

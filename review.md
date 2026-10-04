# Delta review of PR #3987 after fix pass 1, frozen head 07ca5a8d05

Lane `reach-delta3987`. I read the brief, `fix-3987-1.md`, both dual reports (`analysis/reach-dual/3987-r1`, `-r2`), the PR body (`get`), and the CI check runs and job log of the head. I read no PR comments or reviews. All runs were local on 07ca5a8d unless a row says otherwise. Probes, the mutant driver, the k-sweep driver and both k-lint outputs are in `probes/delta-3987/`.

**Verdict: APPROVE-WITH-FIXES**, still under the F2 merge order (after #3977, with #3977 merged in). MAJOR 0 · MINOR 2 · NOTE 5.

F1, the blocking regression, is closed by execution: every op, both orders, the several-solid case and the public reduction. Both MINORs are about what the tests can see, not what the door ships:
- a premise the fix pass wrote into a filed item and two row docs is false;
- F9's type change has no row that goes red if it is reverted.

## Finding table

| F | status | evidence |
|---|---|---|
| F1 inside-out operand at a dual | **CLOSED** | B's `r2_inside_out_wedge_at_dual`, mounted verbatim on the head: ∖ and ∩ refuse `InsideOutOperand { operand: B }`. The ccw control gives ∖ 0.9647859044860531 and ∩ 0.035214095513947156, matching B's polygon clip. PR row `inside_out_operand::an_inside_out_operand_refuses_in_every_op_at_a_dual` passes. Class probes (`delta3987_probes.rs`), all refusing `InsideOutOperand` naming the right operand: an inside-out solid inside a two-solid operand at `Dual64` (∪/∖/∩, both orders); `boolean_reduce` / `boolean_reduce_declared` at `Dual64` (3 ops × 2 orders × 2 doors). Mutant MA (orientation read dropped) reddens the PR row and both probes. Result gate at duals: `ops::structural_gate` runs (MB reddens its unit row; see NOTE 2). Stranded operand at a dual: see NOTE 4. |
| F2 merge order / 1e-12 row | **PARTLY**, as ruled (awaits #3977) | At 1e-12, topo+sweep+verbs+editor-core give 6897/6898. The only red is `contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex` (∩ refused: "a solid encloses negative volume"). Hosted CI on 07ca5a8d (`test` job) has the same single red: 11529/11530. Merging #3977's head 6b8e4cad8 into a copy of this head and running `contact9_side_codes` at 1e-12 gives **11/11 green**, so it is the failure #3977 fixes. The note sits at `contact9_side_codes.rs:282` and the merge-order line at the unit item `:128`. See NOTE 1 on the merge itself. |
| F3 conflict / no CI | **CLOSED** | Head is mergeable (`unstable`). Hosted run 37161937487 on 07ca5a8d: every job green except `test`, whose only failure is the F2 row (`gate ok` red for that reason alone). Python, demos, lint and tools are green. |
| F4 non-curved census findings | **CLOSED** (pinned stated-red) | `reach_wall_chord_rows`, `docm7_union_declare` and `emit_union_flush_names` pin the exact census findings, and each goes red when fixed (read: the `Ok` arms assert the other node). BooleanBody docs (`ops.rs:179-191`) say tier 3. |
| F5 result gate unpinned | **CLOSED** | M3′ (sort-only at the two-operand fallback, `finish_fallback`, and REST) reddens `result_gate_sites::every_result_kind_carries_tier_3s_verdict`, `full_turn_bore_mate::a_shaft_off_the_bores_seam_{is_built_by_the_zip,unions}`, and the box-inventory meta-row (a mutant artefact, as A saw). M2 (main's gate) reddens 13 rows, including the F5 unit row `the_result_gate_refuses_what_only_tier_3_sees` and the door row `review_cleave_farplane::booleans_beside_a_far_carrier_never_answer_wrong`. "Revert the F5 pins", read as a mutant: every red row above except `verbs_germarms` and the meta-row is an F5 addition, so with them reverted M3′ is caught by nothing and M2 only by `verbs_germarms`, as at b8eb4dd. |
| F6 seat re-gate | **CLOSED** | `work/reach/the-evaluator-carries-kept-bodies.md` is open. A's meter (14,417 gates, 42.2 s) is in the PR's Cost section. |
| F7 backstop `≥ 0` arms | **CLOSED** (filed) | `work/reach/the-backstops-positivity-arms-retire-behind-the-result-gate.md` is open. |
| F8 nightly k-lint row | **CLOSED** (red on main by the same flags) | Re-run below: **100 flags on both** the head and its main base 46ce5d4d4, identical in shape, predicate, margin and rule. The lane's 104 was taken against the older main 11d9c7a7f (NOTE 3). |
| F9 `boolean_reduce` took `&Body` | **PARTLY** | Both take `&AtRestBody` and run `gate_unverdicted_operand` (`mod.rs:3395`, `:3413`/`:3422`). A's r1 probe no longer compiles. Its premise, a stranded `AtRestBody<f64>`, cannot be built: `AtRestBody::validate` and `f64::gate_at_rest_kept` both refuse it (`d_stranded_operand_cannot_be_finished_at_f64`). Not closed for two reasons: MINOR 2 (nothing pins the type) and MINOR 1 (the lost-coverage item's premise is false). |
| F10 stale CLEAVE split item | **CLOSED** | `split-answers-…:28-45` now names `gate_unverdicted_operand` and the `AtRestBody` route. |
| F11 49 vs 38/151 | **CLOSED** | Reconciled in the PR body and the unit item. |
| F12 one name, two bars | **CLOSED** (description, per ruling) | BooleanBody docs and `DESIGN.md:292-295` state tier 3. The ratified sentence above it (`:288-291`, "passes tier 3′ … pays that gate once") is unchanged, so the type still names two bars; the ruling says this is not a design change. |
| F13 copies of `finished` | **CLOSED** with a stated residue | Copies remain in `demos/tour/src/booleans.rs:30` and `demos/tour/tests/verbs_teapot_r2_probes.rs:71`, with the reason given (binary crate). The second is `AtRestBody::validate`, identical to the policy gate at `f64`. |
| F14 `ResultInvalid` doc | **CLOSED** | Reworded; but see NOTE 5 at duals. |
| F15, F16 | not taken | Optional. |
| F17 stranded row named kinds only | **CLOSED** | `offer_rows.rs` asserts every finding's face, edge or vertex is the stranded half's. |
| F18 sphere re-cut | **CLOSED** (filed) | `the-sphere-recut-recharts-operand-clones-outside-the-kept-verdict.md` is open. |

## MINOR

1. **A finished operand does reach the maximal-faces gate, through the public union. The unreachability premise the fix pass wrote into the tree is false, and the public rows it retired could have stayed public.** Demonstrated by execution, sure (`probes/delta-3987/delta3987_probes.rs`).
   - **The in-band kink (coincv5's pose):** `d_kinked_prism_with_its_kink_at_rest`.
     - Setup: the prism whose wall turns by `KINK` = 5.5e-10 at a short edge, height 0.1.
     - It refuses at rest only on its kink's `ScaffoldAtRest`. With that edge set at rest in one wall's chart (`EdgeCurveSpec::line_between(..).at_rest_in_chart(chart, false)`, one public call), it passes `AtRestBody::validate`.
     - `topo::union` with a far brick then refuses `CoplanarNeighbours`, margin −5.4999999999999913e-11. That is exactly main's public-row value, −KINK·KINK_HEIGHT. 2·KINK does the same.
   - **The structural arm:** `d_split_top_sharing_one_surface_key_against_the_finished_gate`.
     - Setup: a top split by `mef_chord`, both halves on the top's own surface key, the chord at rest in that chart.
     - It finishes, and the public union refuses `NonMaximalFaces`.
   - **The closed-edge lever:** `d_coplanar_neighbours_with_an_at_rest_edge_against_the_finished_gate`.
     - Setup: the disc planted in a top, its circle set at rest in the top's chart and traversed with axis −z. (With +z it refuses `LoopRoleInverted`.)
     - It finishes, and the public union refuses `CoplanarNeighbours` on (top, disc).
   - **What this contradicts:**
     - `work/reach/the-operand-gates-curved-and-maximal-face-arms-have-no-finished-fixture.md:17` ("the arm is reached by no row through a public door") and its closing option `:41` ("a statement per arm that it is unreachable … and its retirement"). Retiring these arms would be wrong.
     - `offer_rows.rs:1517-1528`: "Two faces bent apart by an angle in the band are not a finished body … the edge between the faces has no honest intersection description at that angle". True of the fixture, not of the pose: tier 3 certifies the at-rest description.
     - `neighbours_across_a_closed_edge.rs:104`, the "(b)/(c)" witnesses: "a disc planted in a top keeps its circle a scaffold, so `a` is not a finished body and no boolean door takes it". Fixture-true, pose-false, and this is the merge commit's own claim that the brief asked about.
   - Also: at a dual every operand is unverdicted, so these arms stay reachable there with unfinished operands, and no dual row covers them.
   - **Class:** I did not try the pair-scoped curved gate (`CurvedPairUnsupported`) or `curved_face_arm`. The item's proposed swept-cone/torus fixtures are the obvious next probe.
2. **F9's type change has no row that can go red.** Demonstrated by execution, sure.
   - Mutant MC: `boolean_reduce` / `boolean_reduce_declared` take `&Body<T>` again, with the `gate_unverdicted_operand` loop removed.
   - It **compiles**: every caller passes `&AtRestBody` and deref-coerces.
   - topo + sweep + editor-core + verbs at 1e-9 go 6900/6901. The only red is my probe `d_public_boolean_reduce_at_a_dual_refuses_the_inside_out_wedge`.
   - So the "public reduce takes finished operands" fence, and the dual orientation read through the public reduction, hold only by the current source text. Nothing in the PR pins either. Under `Deref`, `&AtRestBody` at a call site does not prove the callee's parameter type (Q3).

## NOTE

1. **Merging #3977 is not clean.** Executed: merging 6b8e4cad8 into a copy of 07ca5a8d conflicts in `crates/topo/tests/contact9_side_codes.rs` (the `body_of` tier-3 block against this PR's `into_body`) and in `crates/editor-core/tests/edit_refusal_recourse.rs`. I resolved both to this head's side only to run the row; the real merge needs a real resolution. Sure.
2. **`structural_gate` is pinned at unit level only.** Executed: mutant MB (the call skipped) reddens only `ops::tests::at_a_dual_the_result_gate_still_refuses_a_scaffold_at_rest`, which calls `gate` directly. No door row at a dual ships a body through it. Its subject is a construction that stopped half-way, which no dual pose here produces, so this is likely acceptable. Likely.
3. **F8's count.** Executed: the nightly's dev-probe dumps (`m4_pr8_k_probe`, `demo-tour k-probe`) at 1e-6 and 1e-9, then `tools/k-lint` over both rows, on the head and on main 46ce5d4d4 (`delta3987_ksweep.sh`).
   - Both trees give `GATE FAILED`, 100 margins (rule 1: 82, rule 2: 3, rule 3: 16; one `table:volume_backstop` margin trips two rules, so 101 FLAG lines).
   - The flags are identical once CSV line numbers are stripped. Samples per row: head 2.87 M, main 1.58 M. So the gate adds about 1.29 M samples and no flag.
   - Not exercised:
     - the 1e-12 row (the lane reports it stops at `lily_leaf_b`);
     - the M2 and E6-driver dumps;
     - the plain probe-suite section, which aborts on main. Main's latest hosted nightly (run 37116455100, 36acda19) shows the dev-probe row red at `tilted_sphere_pair_k_rows` before the lint step runs.
   - The lane's "104" is the same measurement against 11d9c7a7f, and the PR body and closed item still say 104.
   - Sure.
4. **Stranded operand at a dual.** Executed: `d_stranded_operand_at_a_dual` (a scalar-generic copy of `top_split_redescribed`; it strands at `f64` too, 8 findings). At `Dual64`, ∪/∖/∩ and `boolean_reduce` in both orders end at `ClassificationInvariant { "contfp ray schedule exhausted" }`, as on main, and no body ships. HONE's note that no public dual construction strands a face holds for the public rechart: `set_face_surface` with the lifted plane refuses `RechartStrandsDescriptions` at `f64` and at `Dual64` (`d_does_a_public_rechart_strand_at_a_dual`). Other public routes were not swept. Likely.
5. **`ResultInvalid`'s doc premise is false at duals** (`mod.rs:2378-2382`). It says "The operands are finished bodies, so no finding is carried in from an operand". At a dual the operands carry no verdict; they pass tiers 1–2, edges and orientation only. Likely.

## ε runs (nextest `ci` profile, local)

| run | 1e-9 | 1e-6 | 1e-12 |
|---|---|---|---|
| topo + sweep + editor-core + verbs, which hold the PR's new and changed rows plus B's mounted probes (at 1e-6 and 1e-12, all but the rotated-box E2E; at 1e-9 all probes were filtered out and B's ran separately, 3/3) | 6891/6891 | 6898/6898 | 6897/6898 (F2 row only) |
| mesh + pncad + step-export + stl + viewer (the rest the PR touches) | 1403/1403 | 1403/1403 | 1403/1403 |
| topo + sweep + editor-core, **default profile (slow set included)** | 6955/6955 (probes filtered out) | — | — |

Every red was checked against origin/main and #3977 (F2).

The 1e-9 row excludes one red that was my own artefact: `every_suite_file_is_aggregated`, from my first `#[path]`-less mount of B's file, fixed before the 1e-6 run. B's `r2_rotated_boxes_every_op_both_orders_reused` ran once at 1e-9: 308 answered, 0 wrong. The demos tour and Python suites were not run locally; hosted CI on the head is green for both.

## Mutants (1e-9, topo + sweep + editor-core + verbs, `ci`)

| mutant | red rows |
|---|---|
| MA: dual orientation read dropped (`reduce.rs` `gate_unverdicted_operand`) | `inside_out_operand::an_inside_out_operand_refuses_in_every_op_at_a_dual`, plus my two class probes |
| MB: `structural_gate` skipped | `ops::tests::at_a_dual_the_result_gate_still_refuses_a_scaffold_at_rest` only |
| MC: public reduce takes `&Body` (compiles) | none of the PR's; my probe only (MINOR 2) |
| M3′: sort-only at the fallback sites and REST | `result_gate_sites`, `full_turn_bore_mate` ×2, box-inventory meta-row |
| M2: main's gate | 13 rows: `result_gate_sites`, `the_result_gate_refuses_what_only_tier_3_sees`, `review_cleave_farplane`, `full_turn_bore_mate` ×5, `m5_s1_rest_zip` ×4, `verbs_germarms` |
| F5 pins reverted | by attribution of the rows above (not a separate run): M3′ is caught by nothing, and M2 only by `verbs_germarms` |

## Style (Q1, Q3, Q4, Q5 and Q6 exercised; Q2 and Q7 lightly; Q8 not done: I read `ops.rs` 470–560 and 2860–2930, the touched tests and `reduce.rs` 370–480, not the whole ~4000 lines)

- **Q3, sure.** MINOR 2 and NOTE 2: of the three dual guards this pass added, only the orientation read has a door-level row.
- **Q4, sure.** MINOR 1 is the class this lane exists for. A premise ("no finished operand reaches X") was written into an item and two row docs on the evidence of the fixtures at hand, and one public call refutes it. The sweep shape that would have caught it: for each retired public row, try to finish its body before relabelling it.
- **Q1, sure (disclosed).** At a dual, `gate_operand`'s tiers 1–2 run twice per operand per op: `gate_unverdicted_operand` (`reduce.rs:459`), then `gate_operand_pairs` (`:377`) on the merged body. The filed item says so. At f64 the pair pass runs on `Validated` operands, also filed.
- **Q7, unsure.** `gate_unverdicted_operand` keys on `outcome() != Validated`, not on the scalar. That is right today, since only a dual mints `NotRunAtThisScalar` in production. But `reduce.rs`'s own tests mint `AtRestBody::<f64>::not_run` to feed a torn body through, so "unverdicted" already means two things in-crate.
- **Q4, likely.** `DESIGN.md:288-295`: the ratified clause and the new sentence beside it now give one type two bars. This is accepted by ruling F12, and recorded so the census unit retires the sentence.
- **Q6, likely.** The k-lint item (closed) and the PR body still carry 104. The current figure against the head's own main is 100 (NOTE 3).

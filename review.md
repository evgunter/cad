# Review of PR #3987, frozen head b8eb4dd0eb

Lane `reach-dual3987-r1`. Wall clock 18:51–20:06 UTC, 2026-10-03. No glimpse: I read only the brief, the PR body (`get`), PR 3977's body (`get`, for the merge order) and the repo. I read no comments, reviews or `analysis/reach-dual/*` branches.

**Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 2 · MINOR 5 · NOTE 4. The door's tier-3 half does what it says, checked by execution. Two things block: the merge order against #3977, and a head that cannot merge and has no CI.

## MAJOR
1. **The head cannot merge and no gate ran on it.** Demonstrated by execution. `git merge-tree HEAD origin/main` (474d36d) conflicts in `crates/topo/src/boolean/ops.rs`, the file that holds the gate, against main's merge-door re-description (`af6c6fcdd`…`911e57dc4`). GitHub shows `mergeable_state: dirty`, and `get_check_runs`/`get_status` on b8eb4dd return **0 runs and 0 statuses**. The PR's "11420 passed after merging main" describes a merge that is not this head. Any verdict conditioned on green has no green to rest on.
2. **The door lands before check 7's interval re-derivation, so a valid body gets a wrong verdict at ε 1e-12.** Demonstrated by execution. `contact9_side_codes::a_vertex_pair_reads_a_dipping_chord_at_its_far_vertex` is red at 1e-12 on the head (`NegativeVolume` on a +5.83e-19 m³ body). Under mutant M1 (the gate's `gate_at_rest_kept` replaced by `AtRestBody::not_run`) it goes **green**, so the result gate is the cause. Ev's ratified sequencing (`work/reach/boolean-door-tier-3-waits-on-the-description-gap.md:125-128`, written in the ratification commit 5a42ebe13) says check 7's interval re-derivation "must [land] before the boolean door's" adoption, and the unit's own §Sequencing 1 says "each lands before or with the gate". Claim 5 holds as worded: the row is stated red and not suppressed, with no nextest exclusion, `#[ignore]` or ε skip. Stating it does not make merging ahead of #3977 sound. This must merge after #3977, with #3977 merged in.

## MINOR
1. **Claim 1's premise is wrong: the census half is not built.** Demonstrated by execution, and disclosed by the PR. `ops::gate` (`ops.rs:2702`) runs `gate_at_rest_kept` only. Probe `e2e_door_ships_a_result_its_own_tier_3_prime_refuses`: an off-axis disjoint union of two unit balls (gap 0.26, and 0.05) ships with outcome `Validated` and volume equal to the closed form. `validate_pseudomanifold` over its own contacts then refuses it `CensusUndecidable`.
   - For the curved cross-solid case this is the ratified sequencing.
   - The non-curved `UndeclaredContact`/`StaleContactDeclaration` results that the unit's §Sequencing 3 says "must be fixed or pinned" are neither; they are parked (`work/reach/boolean-door-runs-the-census-over-its-result.md`). "A result the door ships is never below tier 3′" is false at this head.
2. **The stranded-operand invariant is still reachable through a public door.** Demonstrated by execution. `topo::boolean_reduce` and `boolean_reduce_declared` (`boolean/mod.rs:3274,3292`, re-exported at `topo::` and so `pncad::topo::`) still take `&Body`. The stranded split top crossed by the brick ends on `ClassificationInvariant { what: "contfp ray schedule exhausted" }` for ∪, ∖ and ∩ (probe `probe_stranded_operand_through_public_boolean_reduce`). HONE's item is closed as "retires".
3. **Claim 6 is only partly true: the gate at three of the four sites goes unpinned.** Demonstrated by execution. Mutant M3′ keeps `sort_into_pieces` and skips tier 3 at the fallback two-operand arm (`ops.rs:3490`), `finish_fallback` (`:3549`) and the declared-REST union (`rest.rs:396`). topo + sweep + editor-core run 6756/6757; the one red is the box-reader inventory meta-row reacting to the mutant's helper. M1 (the whole gate) does turn 4 rows red: `the_result_gate_refuses_a_scaffold_at_rest`, two contact9 seam rows that then ship a scaffold-at-rest body (`got None`), and `verbs_germarms`. The census mutant cannot be run, because there is no census call to remove.
4. **The editor's re-gate is unmeasured in the PR, and it is the larger cost.** Demonstrated by execution.
   - `finished_operand` (`editor-core/src/eval/wire.rs:855`) clones each operand and runs full tier 3 on it.
   - A boolean result is gated at the door, `into_body`'d and stamped, then gated again at the next seat. So "nothing is gated twice" is false for editor chains.
   - Metered over the editor-core suite (ci profile, ε 1e-9): **14,417 seat gates, 42.2 s summed**, median 0.82 ms, p90 5.4 ms. The PR's door tier-3 gate on the same suite is 27.2 s.
   - This replaces CLEAVE's 87 µs check-7 read with a full tier 3. The "later step where the evaluator carries kept bodies" has no `work/` item (grep: none).
5. **An open item now prescribes a removed function.** By inspection. `work/cleave/split-answers-an-inside-out-operand-with-two-inside-out-halves.md:28-36` (open) prescribes `validate::inside_out_solids` and the orientation read in `reduce::gate_operand`. This PR removes both, updates SHELL's twin item, and leaves this one stale.

## NOTE
1. **Claim 4's count is not reconciled.** The unit cites 49 sub-tier-3 fixtures; the PR measures 31 topo + 7 sweep = 38 tests (151 calls) and never reconciles the two. The refusal tables otherwise match what I ran (unsure whether the 11 extra are counted elsewhere).
2. **End-to-end through `pncad`** (`probes/review3987_e2e.rs`; all `Validated`, every result re-validated):
   - Fixtures: box×box, box×cylinder and box×ball (pole normal to the cut face), at ×1e-3, ×1 and ×1e3, identity and a 0.7 rad skew rotation, both operand orders, ∪/∩/∖.
   - Results reused as operands: (A∖C)∩B and (A∖C)∖D.
   - Volumes match the closed forms (box arithmetic, πr²h, cap πh²(3r−h)/3) to 1e-9 relative.
   - `point_in_solid` was sampled 300× per result against the analytic oracle (120 checks): **0 wrong answers** at 1e-9 and 1e-12.
   - At 1e-6 the ×1e-3 pose shows only typed `Escalated` refusals inside 10ε, never a wrong In/Out.
   - Typed, pre-existing refusals: a ball whose polar axis is tilted against the cut plane gives `SectionNotPolar`; at 1e-12, `transform_rigid` of the ×1e3 rotated cylinder refuses on the pcurve envelope (fixture side; that pose was skipped).
   - The inside-out operand refuses at `validate` with `NegativeVolume`.
3. **Suites at this head** (nextest `ci`):
   - topo: 2162/2162 at 1e-9 and at 1e-6; 2161/2162 at 1e-12 (contact9 only; `rigid_map_near_eps_plane_nurbs` sits in the slow set).
   - sweep + verbs + editor-core + pncad at 1e-9: 4714/4714.
   - Not run: those four at 1e-6/1e-12, the tour, k-lint (the PR reports 108 k-lint flags and a nightly that will go red).
4. **Claim 3 holds.** By inspection, sure.
   - `AtRestBody`'s fields are private (`validate.rs:7908`) and `Body` has no interior mutability (`body.rs:156`).
   - Access is `Deref` only; `into_body` gives up the verdict; `Tol` has one inhabitant (`tolerance.rs:683`).
   - `not_run` is `pub(crate)` and called only from the dual's policy (`props.rs:3334`).

## Style (Q1–Q8 exercised; Q8 partly: I read `ops.rs` 1–110, 120–660, 2690–2720 and 3460–3700, not all 3,700 lines)
- **Q1, sure.** The fix mints four copies of `finished` beside `topo::test_support::finished` (`lib.rs:306`): `stl/tests/common/mod.rs:20`, `verbs/tests/run_door.rs:142`, `demos/tour/src/booleans.rs:30` and `demos/tour/tests/verbs_teapot_r2_probes.rs:71`. Two spellings differ at duals: `AtRestBody::validate` always validates, while `T::gate_at_rest_kept` returns `not_run` at a dual. Sweep re-exports the shared copy correctly, so the class can be swept.
- **Q3, sure.** `review_cleave_farplane.rs:213` accepts any typed refusal. The PR's new ∪/∖ refusals at every k pass silently, and the row cannot go red on over-refusal. Nothing asserts `outcome() == Validated` on a fallback-site result, which is the cheap pin M3′ shows is missing.
- **Q4, sure.** The stale CLEAVE split item (MINOR 5). Also: `BooleanError::ResultInvalid`'s doc (`mod.rs:2281-2284`) says "the findings are the door's own". That is true only of a correct kernel, and the 1e-12 row shows the gate's own verdict can be wrong.
- **Q5, likely.** `BooleanBody.contacts` is documented as "the tier-3′ declarations" (`ops.rs:186-187`), and nothing at the door has checked them. The type reads as tier 3′ while carrying tier 3.
- **Q6, likely.** The editor re-gate deferral is disclosed but unscheduled (MINOR 4). So is the backstop's `encloses_material` retirement in the unit's final state: it is neither done nor filed.
- **Q7, unsure.** `VerbOut<T, B = Body<T>>` (`verbs/src/run.rs:37`) adds a defaulted type parameter with one non-default user. I would have typed the pair arm directly.
- **Q1, likely.** `sweep::test_support::assert_legal_operand` validates the body, then unions it with a far brick whose door gates again: three gates for one question.

## Probes
`probes/review3987_e2e.rs` (pncad, public API) and `probes/review3987_topo_probes.rs` (in-crate, stranded operand). The mutants were edits to the head, now reverted:
- M1: `ops.rs` `gate`, with `gate_at_rest_kept` replaced by `AtRestBody::not_run`.
- M3′: sites `ops.rs:3490`, `ops.rs:3549` and `rest.rs:396` routed through a sort-only gate.
- Seat meter: a timer around `finished_operand`, appended per call to a file (10 of 14,427 lines were interleaved by concurrent writes and dropped).

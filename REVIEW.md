# Review r2: PR 4274, a nested pairing at a shared vertex builds

Frozen head `4d985b572`, base `517b81a24`. Lane isolation kept: no other review branch or
session read, no glimpse to disclose.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 3.

**Method.** Release `sweep --test all` on base and on an instrumented head
(`REVIEW-r2-probes.patch`: `R2_PROBE` prints each `hang_at_shared` result, `R2_MUT` switches
the mutants; with neither set, head's path is unchanged). On head the whole suite passes
(2416 rows). Every build was read through `outcome` and `pierce_point_finding` (keys,
cones, `check_mesh`). Diffs ignore `v=`/`want=`; `SOUND` still needs the volume to 1e-7.

| battery / probe | lines | moved | moves |
|---|---|---|---|
| `r2_pinch_findings` (pinch battery + findings) | 3024 | 102 | `ba U/I/S` × 34 `SharedVertexCrossings` → `SOUND Some(None)` |
| `four_pairs_battery` | 5333 | 307 | `SharedVertexCrossings` → 182 `SOUND Some(None)`, 120 `PinchConesOnSeparateKeys`, 5 `SharedVertexCrossings` |
| `r2_many_pairs` 4 cubes, tilts −0.05 / −0.8 (new) | 1376 | 61 | 50 → `SOUND Some(None)`, 10 → `PinchConesOnSeparateKeys`, 1 → `SharedVertexCrossings` |
| `r2_many_pairs` 5 cubes, tilts −0.1 / −0.4 / −0.7 (new) | 1082 | 1 | → `SOUND Some(None)` |
| `near_tangent_battery` / `join1_r1_reflex_battery` / `corner_pairs_battery` | 7200 / 1152 / 16380 | 0 | byte-identical |

Refusal → `BAD`: 0 everywhere. `SOUND` → refusal: 0 everywhere.

## Claims

1. **Laminar arcs, innermost holder: HOLDS** (inspection plus probe).
   - B's arcs are non-crossing chords (`b_runs`), A's are consecutive pairs, and a turn
     keeps the chord. Any sides of two non-crossing chords give disjoint, nested or
     covering arcs, so the crossing arm (`insert.rs:619-621`) is live only in `b_runs`.
   - Positions are a permutation (`insert.rs:429`), so no two germs coincide in one.
   - Two turned runs in one plan: on A's side they always cover and refuse (NOTE-2).
   - Wrap past the start: about 70 shared-vertex plans with wrapping arcs (`[5,0]`,
     `[4,1]`, …) all built `SOUND Some(None)`. No wrong-holder configuration found: in a
     laminar family the holders are totally ordered, and the shortest is innermost.
2. **B's nesting is unchanged: HOLDS** (executed).
   - A wrapping B arc is adjacent and holds nothing, and holding one end means holding
     both, so the holders match main's; `held_by` is main's loop body, moved.
   - The reflex, corner-pairs and near-tangent batteries are byte-identical, and
     `b_runs_nest_every_non_crossing_matching_…` passes on head.
3. **Depth above one at a shared vertex: UNSURE.** It is not exercised anywhere (MINOR-1).
   I did not get a depth-2 pose to build.
4. **No wrong body ships: HOLDS within what I probed** (executed).
   - All 182 four-pairs and 102 pinch lines give `Some(None)`; the new 4- and 5-cube
     families ship no new `BAD`.
   - Not probed: a curved face at the vertex (parked: `a-curved-face-at-a-shared-pinch-vertex-…`).
5. **The 120 lines are the parked class: HOLDS** (executed).
   - All 120 are `ba U`/`ba S` at 60 poses. At every one, main already refuses `ab U`
     `PinchConesOnSeparateKeys` (the operand's own pinch on separate keys), and `ba I` now
     builds `SOUND Some(None)`. None is a body the ruling would build on one key.
6. **The remaining refusals: crossing arm unreachable; cover arm not reached** (MINOR-2).
7. **The mutants are real: HOLDS** (executed). Rows that go red, from the `join` sweep rows
   plus the `boolean::insert` unit tests:

   | mutant | red rows |
   |---|---|
   | `guard` (restore the up-front refusal) | `a_six_crossing_pair_…_in_both_orders` |
   | `once` (holders read before the reconcile, my own) | 4 rows: `…holds_the_rest_of_its_pair`, `…holds_two_siblings`, `a_six_crossing_…`, `three_pairs_whose_hang_…` |
   | `cover` | the unit test only |
   | `outer` | the unit test, `b_runs_nest_every_…`, `eight_crossing_corners_nest_two_deep_…` |
   | `depth1` (my own) | the unit test, `eight_crossing_…` |
   | `nostrut` (my own) | the unit test, `eight_crossing_…` |

## Findings

- **MINOR-1: the shared-vertex depth chain and `by_strut` are never reached** (test gap,
  executed).
  - Every `held_by` result in `hang_at_shared` (`insert.rs:1048`), over the whole suite
    and every battery above, has depth ≤ 1 and `by_strut` false.
  - So `depth1`/`nostrut` go red only through `eight_crossing_corners_…` (no shared
    vertex); confined to `hang_at_shared` they would survive every row. "Depth above one
    is allowed" has no row, nor does a held strut at a shared vertex.
- **MINOR-2: the cover refusal has no reachable pose** (executed).
  - `R2_MUT=cover` changes 0 lines in the four-pairs, pinch and 4-cube batteries and 0
    sweep rows; only the synthetic unit test pins it. The doc (`insert.rs:1017-1019`,
    `mod.rs` `SharedVertexCrossings`) describes a configuration no probe produced.
    Refusing is the right direction; whether it is live is unknown.
- **NOTE-1:** 8 lines in the 4-cube family at tilt −0.05 (`ab U`/`ba U`) are `OK BAD`,
  `t3p=false`, with three keys. They are identical on main, are the parked
  `a-pinch-the-seams-do-not-link-…` class, and are not this PR's.
- **NOTE-2:** the PR body says the laminar reading "covers two turned runs in one plan".
  On A's side two turned runs always cover each other, so they refuse. "Covers" there
  means "refuses typed" (inspection).
- **NOTE-3:** the five `SharedVertexCrossings` lines that remain in four-pairs come from
  the reconcile's both-ways arm. The cover mutant leaves all five unchanged, which confirms
  the PR's attribution.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 not exercised)

- **Q1 (likely):** at a shared vertex, two readings now meet in one pass.
  `reconcile_pass` turns by sector geometry (`held_cut`, `insert.rs:1166`), then
  `hang_at_shared` reads holders by walk combinatorics (`insert.rs:1043`). Main's
  `sibling_holds` read the reconcile's geometry; nothing now checks the two agree at a tie.
  The `## Since` note names both spellings, not that they now feed each other.
- **Q1 (unsure):** `arc_holders`' `len`/`past` closures (`insert.rs:608-612`) are a third
  spelling of forward walk distance mod n, beside `run_order`'s `(p + 1) % n`
  (`insert.rs:544`).
- **Q2 (likely):** `arc_holders`' doc (`insert.rs:595-606`) promises a crossing refusal
  that is unreachable from `hang_at_shared` and is all `b_runs` uses it for.
- **Q4 (likely):** `insert.rs:619` accepts `o` as holder when `k` holds one of `o`'s ends
  (a crossing); correct only because the loop later visits `(o, k)` and refuses.
- **Q4 (unsure):** `n = 2 * runs.len()` (`insert.rs:1043`) is not carried with `walk`; it
  rests on the planner's `n = survivors.len()` (`insert.rs:415-424`).
- **Q7 (unsure):** `held` has two writers, `plan_null_pairs` and `hang_at_shared`.
- **Q3 (sure):** the cover arm's only row is synthetic (MINOR-2). The depth and strut chain
  at a shared vertex has no row that can go red (MINOR-1).
- **Q5 / Q6 (likely):** the body's deviation from the row's route is an improvement and owes
  nothing. "Two turned runs" is overstated (NOTE-2).
- **Q8:** not exercised; `insert.rs` (3159 lines) was not read end to end, on budget.

REVIEW COMPLETE

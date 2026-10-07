# Review (fix pass, whole unit) — PR 4274, frozen head 7464d6c0

**Verdict: APPROVE** · MAJOR 0 · MINOR 1 · NOTE 4

**Method.** I built release `sweep --test all` twice, in separate target dirs:
- head `7464d6c0`;
- main at the merge base `21c7b0c4`, with head's `join_pierce_runs_sweep.rs` copied in so the probes run there too.

The head build carries an env-gated patch (`REVIEW-fix-mutants.patch`, not applied on this branch). `R3_MUT` switches the mutants: `oldkey`, plus `depth1`, `nostrut` and `outer`, each confined to `hang_at_shared`. `R3_TRACE` prints every `hang_at_shared` reading and every mint key with a nested strut. With neither variable set, head's path is unchanged.

**My family.** A new ignored row, `review_fix_strut_chain_with_many_cubes_battery` (`crates/sweep/tests/join_pierce_runs_sweep.rs:2795`, built on `:2718`), pinches two or three narrow parallelepipeds onto the eight-crossing poses' shared point:
- they are cube frames whose edges are pulled toward the diagonal, so several fit disjointly;
- sides are 4, 2 and 1, at 12 tilt/azimuth directions and 2 spins;
- 1 041 lines in all, 483 of them `SKIP` where the slivers overlap.

Every line is read through `differential::outcome` plus `pierce_point_finding` (one key, `cones_at`, `check_mesh`).

I first tried two side-4 corner-on cubes. All 575 of those sets overlapped (`SKIP`), so I switched to the slivers. I read the r1 and r2 review branches, as the dispatch asked.

## Batteries (executed, main vs head, `v=`/`want=` ignored)
| battery | lines | moved | moves |
|---|---|---|---|
| `pinch_runs_battery` | 3 024 | 102 | `SharedVertexCrossings` → `SOUND` |
| `four_pairs_battery` | 5 333 | 307 | → `SOUND Some(None)` 182, → `PinchConesOnSeparateKeys` 120, `SharedVertexCrossings` unchanged 5 |
| r1 probe `eight_crossings_with_a_pinched_cube_battery` | 576 | 276 | → `SOUND Some(None)` 276 |
| `review_fix_strut_chain_with_many_cubes_battery` (mine) | 1 041 | 228 | → `SOUND Some(None)` 128, → `PinchConesOnSeparateKeys` 100 |
| `pierce_runs_battery` / `join1_r1_reflex_battery` | 4 536 / 1 152 | 0 | byte-identical |
| non-ignored `join_` rows (head) | 59 | — | all pass |

- No line on head reaches `ClassificationInvariant`, and none moves from a refusal to `BAD`.
- My family's 44 `BAD` lines and 105 two-key `SOUND` findings are identical on main.
- Not run, for budget: `corner_pairs_battery`, `rc_wide`, r2's `many_pairs`, and the topo unit tests. CI is green on this head.

## Claims
- **A. The mint order: HOLDS (executed for depth 1; inspection beyond).**
  - **Trace.** In 152 of my family's readings (76 two-sliver configs), the M1 strut chain `[2,5] ⊃ [3,4]` with `fan: None` sits at the shared vertex beside two sliver struts. `R3K` keys:
    - outer strut `d0`;
    - held strut `d1`;
    - both slivers `d2` (nested in both), or `d1` as siblings of the held strut.

    All build `SOUND Some(None)`. The old key (`oldkey`) sends 74 of these lines to `ClassificationInvariant` ("In end … Out end"), so my family pins the fix independently of the PR's row.
  - **A fan-held run beside a strut-held run** at one vertex: the trace shows both, e.g. `r3:h1 [1,Some(0)] r2:h2 [2,Some(0),by_strut]`, and they build.
  - **Deeper chains, by inspection.** If geometric nesting (`nests` → `holds_whole`) is transitive and agrees with the arcs, then everything that nests an outer strut also nests its inner one. Sorting by that count is then a linear extension of the nesting at any depth.
  - **Depth ≥ 2 with `fan: None`** (strut ⊃ strut ⊃ strut) is reached by no family: not the PR's, not r1's, not mine (MINOR-1).
  - **No reordering of rows that sorted correctly on main.** The key differs from main's only when `held.fan` is `None`, and every differential above is 0 outside the moved refusals.
- **B. The crossing arm in `hang_at_shared` is unreachable: HOLDS (inspection; executed).**
  - Chords are fixed at plan time:
    - A pairs consecutively (`insert.rs:443`);
    - B's chords are checked non-crossing by `b_runs` (`insert.rs:583`);
    - the reconcile's only alternative is `reversed()`, which swaps `walk` and keeps the chord (`insert.rs:717`, `:1205`);
    - nothing else writes `walk`.
  - Any sides of non-crossing chords are disjoint, nested or covering. No battery line carries "cross in its walk".
- **C. No wrong body ships: HOLDS within my family (executed).**
  - Every one of the 128 new `SOUND` lines is `Some(None)`.
  - **The 100 new refusals** are 34 `ba U`, 34 `ba S` and 32 `ba I` over 66 configs. Every one of the 66 already builds, in `ab` order on both trees, a body whose vertices at the point "do not share one point".
    - That is the parked operand-pinch class (`a-pinch-the-seams-do-not-link-…`), which the refusal names.
    - Unlike four_pairs, main refused none of these configs `PinchConesOnSeparateKeys`, and `ba I` is among the newly refused ops (NOTE-1).
- **D. The batteries: HOLDS.** pinch, four_pairs and r1's probe reproduce the PR's table exactly. I did not re-run r2's `many_pairs`.

## Findings
- **MINOR-1 (test gap / unenforced premise; inspection plus trace).** A strut-only held strut sorts correctly only if its own strut holders nest it geometrically (`nests`, `insert.rs:882`), so that its `d` exceeds each holder's.
  - Those holders come from the walk arcs (`arc_holders`). The comment "which counts its own holders" (`insert.rs:794`) asserts that agreement, and nothing checks it.
  - A tie or reversal would mint the held strut before its holder, at the vertex, with `by_strut` set.
  - The only strut-only chain any family reaches is depth 1. A `debug_assert` beside the `keyed` sort would make it loud: each strut-only held run's `d` must exceed its holder's.
- **NOTE-1.** In my family the newly refused lines include `ba I`, which in four_pairs builds `SOUND`. They are still the parked class: the `ab` order shows the operand's point on separate keys on both trees. **likely**
- **NOTE-2.** Ties at equal `d` between the held strut and another pair's strut break by run index `n` (`insert.rs:800`). Executed cases are true siblings. Whether a `holds_whole` within the band can call nested runs non-nested, and so leave a false tie, I could not tell. **unsure**
- **NOTE-3.** I did not execute the cover arm here. It now refuses `SharedVertexCrossings` in `hang_at_shared`, and its docs call it a backstop no known pose reaches (`insert.rs:619-624`, `mod.rs:2318`).
- **NOTE-4.** Corner-on cubes cannot pinch two at once above the posed face: their 35° face clearance overlaps within the 0.6 rad tilt range. That is why my family uses slivers.

## Adjudicated findings
- **r1 M1: CONFIRMED FIXED** (executed). The r1 probe is 276 → `SOUND Some(None)` with 0 `ClassificationInvariant`. My two-sliver family reaches the same chain and builds.
- **Pins (r1 m1 / r2 MINOR-1): CONFIRMED FIXED** (executed). The new row `a_strut_chain_…_depth_three_hangs` goes red under every mutant:
  - `oldkey`: 2 runs;
  - `depth1`: 2;
  - `nostrut`: 4;
  - `outer`: 2;
  - unmutated: it passes.
- **Cover arm (r1 m2 / r2 MINOR-2): CONFIRMED FIXED as documentation.** The arm is documented as an unreached backstop. The two refusals are told apart:
  - `ArcRefusal::{Crossing, Cover}` (`insert.rs:606`);
  - in `hang_at_shared`, a cover is `SharedVertexCrossings` and a crossing is `ClassificationInvariant` (`:1082`);
  - in `b_runs`, a crossing is `PairingMismatch` and a cover is `ClassificationInvariant` (`:444-453`).
- **r1 S1 / r2 Q4 (walk length): fixed.** `NullPlan::walk_len` (`insert.rs:307`), asserted at `:1080`.
- **r1 S5 (laminarity assert): fixed** (`insert.rs:647`).

## Style lane
I exercised Q1, Q2, Q3, Q4 and Q6. Q5 and Q7 I skimmed. I did not exercise Q8: `insert.rs` was not read end to end, for budget.
- **Q1 (likely).** Who holds whom is now read twice for one strut-only chain:
  - by walk arcs (`held_by`), which picks the mint site;
  - by sector geometry (`nests`), which picks the mint order.

  This is r2's sector-vs-walk note, now with a consumer that needs the two readings to agree (MINOR-1).
- **Q2 (likely).** The `mint_plans` comment (`insert.rs:787-795`) now carries the ordering argument, and its key premise is unenforced (MINOR-1). Its length roughly matches the code it defends.
- **Q3 (sure).** The new row can go red: all four mutants turn it red, executed. The ignored r1 probe stays unpinned in CI, but the row covers its M1 pose.
- **Q4 (likely).** I ran `rg` over `crates/topo/src/boolean` for `hang_in_turned`, `sibling_holds` and "never shares its vertex": 0 hits.
- **Q6 (likely).** The deviation ("build, not refuse") is an improvement: r1 M1 now builds a correct body instead of refusing typed.

REVIEW COMPLETE

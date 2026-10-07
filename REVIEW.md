# Review r2 — PR 4249 at frozen head `cb1af2e55` (base `fb8c7cbb`)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 2 · NOTE 3. The 217 are
right and nothing else in the PR's batteries moves. Outside the battery's
shape, three pairs at one vertex move 18 refusals to `BAD`. Lane
isolation: I read no other review branch, session or PR comment.

**Method.**
- Release builds, with separate target dirs for base, head and mutants.
- Every line goes through `differential::outcome`. Every built body also
  gets `vertices_at(v)`, `cones_at`, `faces_through_two_vertices_at`,
  `pierce_point_finding` and a separate `tessellate` + `check_mesh`.
- The probes are committed here: `join_pierce_runs_sweep::r2_shared_vertex_probe`
  (`R2_PART=tripod|skew|fine|both`; `R2_ONLY=<tag>` prints
  `validate_pseudomanifold`) and `r2_pinch_battery_detail`.
- The mutants are env-switched in `review-r2-mutants.patch`
  (`R2_MUT=off|nofan|onecut|flip|isup`).

## Claims

1. **The 217 are right bodies — holds (executed, all 217, not a sample).**
   - `pinch_runs_battery` base vs head: exactly 217 lines move, all
     `ClassificationInvariant` → `OK SOUND` (`ab U` 76, `ab S` 76,
     `ba U` 23, `ba I` 19, `ba S` 23).
   - The other 2 807 lines are byte-identical, vertex keys included.
   - All 217 have `pierce_point_finding = None` and `f2 = 0`: 175 have
     2 vertices at `cones_at = (2,0)`. **The 42 one-vertex bodies all have
     `cones_at = (1,0)`**, so they truly hold one cone.
2. **Beyond the battery — falsified for three pairs (MAJOR-1); holds
   elsewhere.**
   - `fine`, the pinch at 72 spins per direction (36 288 runs): 2 586
     refusal → `SOUND`, all finding-free, 0 `BAD`, 0 SOUND → refusal.
   - `skew`, two corners 115–160° apart (6 048 runs): 386 refusal →
     `SOUND`, finding-free; its 30 `BAD` lines are identical to base.
   - `both`, both operands pinched (2 016 runs): byte-identical, all
     `SOUND`; the hang is never reached.
   - **Not exercised:** a curved shared vertex, since `cones_at` is
     planar-only.
3. **The geometric hold test — holds (inspection), with one half
   untested.**
   - For a fan, `held_cut` reads `walks_before` from the run's first
     entry, so the wrap is handled. A fan with `lo.0 == hi.0` is a strut,
     and a strut never turns (same segment).
   - Siblings cannot tie: `walk_order` refuses.
   - "Only a turned run holds a sibling" holds: A's pairs are
     consecutive; B's non-nested runs are adjacent; a nested B plan at a
     shared vertex refuses up front (`insert.rs:1085`); two survivors make
     one run.
   - **`onecut` survives** (MINOR-1).
4. **New refusals — unreached; `is_up` refuses nothing legal (executed).**
   - No line in any family moves to or from `SharedVertexCrossings`, so
     the arms at `insert.rs:1012` and `:1034` are never reached.
   - By inspection, two turned runs already refuse in `reconcile_pass`:
     each complement holds the other's blocker. Refusing is right.
   - `R2_MUT=isup` restores `(true, true) => Ok(true)` (`join.rs:457`):
     rows, the pinch battery and tripod are byte-identical.
5. **Nothing else moved — holds (executed).** This is base vs the true
   head (with `is_up`; NOTE-1), all byte-identical:
   - `pierce_runs_battery` 4 536;
   - `corner_pairs_battery` 16 380;
   - `join1_r1_reflex_battery` 1 152;
   - `rc_wide_battery` shards 7, 41, 60 and 83 (480 each).

   The 20 `join_pierce_runs_sweep` rows pass. The 468 two-key `SOUND`
   bodies are the same 468 on both trees with identical findings, and none
   is among the 217.
6. **The mutants are real — holds (executed).**
   - `off` (the PR's), `nofan` (hang at `v`, not the copy) and `flip`
     (mark the turned run as held): both new rows go red. In the pinch
     battery exactly the 217 return to `ClassificationInvariant`
     "In end … Out end". Tripod moves 376 lines and skew 386.
   - `onecut` (read one cut as two) survives everything.
7. **The sweep — holds.**
   - Conflicting ends:
     - `mint_directed`'s `hung.ends`, which covers only shared VV runs;
     - the new `is_up` refusal, a global backstop for every producer
       (vtxfac, rest, zip), though it fires only when a half from that
       vertex is read.
   - `Held` and depth readers, none of which misreads A-side `Held`:
     - `mint_plans`' sort key (depth 1 works);
     - `MintSite::by_strut`, now `Some(false)` for an A plan, where the
       unheld run is a fan that never takes the strut path;
     - `reconcile_pass`'s slot-1 refusal, which runs before the hang;
     - `SideRun::reversed`, reconcile only.

     The only caller is `mod.rs:4454`, once per reduction.

## Findings

**MAJOR-1 — three pairs at one vertex: 18 refusal → `BAD` (executed).**
- **The pose.** `tripod`: the notch against three side-4 cubes whose
  corners at `v` open 120° apart, tilted 0.25 or 0.6 toward the grid
  direction. A pose is kept only if the kernel's union of the cubes holds
  all their volume (110 skipped).
- **The move.** 16 `ClassificationInvariant` and 2 `Euler` refusals on
  base become `OK BAD t3p=false`, all union:
  - `ab U` at `i=0 j=3 t=2 k=0`, `0 4 1 0`, `1 3 2 0`, `1 4 1 0`,
    `3 3 2 0`, `3 4 1 0`, `4 3 2 0` and `4 4 1 0`;
  - both `ab U` and `ba U` at `1 4 2 1`, `2 3 2 0`, `2 4 1 0`,
    `2 4 2 0` and `11 4 2 1`.
- **What fails.** Volume is exact to 9 decimals, there are 3 vertices for
  3 cones, and `check_mesh` passes, but the cones sit on two point keys.
  Tier 3′ refuses `CensusUndecidable` "one passes into the other where they
  touch" (`R2_ONLY="tripod i=0 j=4 t=1 k=0"`).
- **Attribution.** `R2_MUT=off` reproduces base line for line, so this is
  `hang_in_turned`'s (`insert.rs:972`).
- **Mitigation.** Base already ships 10 `BAD` lines of this signature at
  other tripod poses. It reads as the parked separate-keys class
  (`a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys`, D10),
  census-escalated rather than a definite wrong body. The PR widens that
  class; it does not mint one.
- **Why it blocks.** "refusal → `BAD`: 0" is true only of the PR's own
  batteries. The fix pass should keep these refusing, or disclose and file
  them with the poses (adjudicator's call).

**MINOR-1 — unpinned hold test (executed).** At `insert.rs:1003-1012` the
hold test's second cut and all three new arms are unpinned: `onecut`
survives the rows, the pinch battery, tripod and skew. The claim "no
battery line reaches them" has no guard at the claim site.

**MINOR-2 — stale text (inspection).**
- `insert.rs:141`: `Held` is "read once at the plan ([`b_runs`])", but
  the hang writes it after the reconcile.
- The header, `insert.rs:53-56`: the reconcile leaves the runs
  "disjoint", but one plan's runs now nest.
- `insert.rs:1084`: "Only B's runs nest".
- The row `a_nested_pairing_at_a_shared_vertex_refuses_typed`
  (`join_pierce_runs_sweep.rs:1434`) now asserts `ab` builds `SOUND`.

**NOTE-1.** The PR's head batteries left out `is_up`. Re-run on the true
head: no difference.

**NOTE-2.** Base's 10 tripod `BAD` lines are pre-existing and outside
this PR. File them.

**NOTE-3.** Uncovered: a curved shared vertex. `both` never reaches the
hang.

## Style

Questions exercised: Q1–Q7. For Q8 I read the `insert.rs` header and
`mint_plans` through `mint_directed`, not all 3 054 lines.

- **Q1** `insert.rs:972-1037`, `:581` (`b_runs`' `holds`) and `:867`
  (`holds_whole`): three spellings of "run m holds run k whole". — likely
- **Q1** `OtherCut::owner` (`insert.rs:1159`) is documented as "the pair",
  a plan index in `reconcile_pass`. `hang_in_turned` passes a run index
  (`:1000`). — sure
- **Q1** `insert.rs:977-991` re-derives `reconcile_pass`'s `others`
  filter, the `key` closure and the slot's orbit: the third copy of
  `if slot == 0 {..0} else {..1}`. — likely
- **Q2** `insert.rs:964-971` justifies refusing "two turned runs of one
  plan", which `reconcile_pass` already refuses by my reading. That is
  prose defending a likely-dead arm. — unsure
- **Q3** The `is_up` both-sets check (`join.rs:457`) has no row that can
  go red; `isup` survives. — sure
- **Q4** The `MintSite::by_strut` doc (`insert.rs:1329`, "in a plan whose
  runs nest") predates A-side plans reaching it (`:766-770`). That is
  harmless today by inspection. — likely
- **Q7** `Sides::is_up` refuses lazily, per half read. `Sides::new` could
  name every conflicting vertex once. — unsure
- **Q7** `Held { depth: 1, by_strut: false }` is hard-coded, while
  `b_runs` builds chains. The filed nested+shared issue needs both, and
  nothing types the difference. — unsure

REVIEW COMPLETE

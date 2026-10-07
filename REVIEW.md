# Review r1 — PR 4249 at `cb1af2e5` (base `fb8c7cbb`)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 2 · NOTE 5.
Lane isolation kept: I read no other review branch or session, and saw no glimpse. All runs were on the frozen head, in a release build (base and head in separate target dirs). Mutants and instrumentation ran in a third, debug tree. Probes: `crates/sweep/tests/join_pierce_runs_sweep/review_r1_4249.rs`, `r1_*`, ignored. Each built body goes through `outcome`, `cones_at` against `vertices_at(v)`, `shared_point_finding`, `faces_through_two_vertices_at` and `check_mesh` at δ 0.05.

## Claims
1. **Holds** (executed, `r1_pinch_battery_detail`). All 217 moved lines are `SOUND`. Their volume equals the oracle at 9 decimals, they hold one point key, mesh cleanly, and have `faces2=0`.
   - The vertex count at `v` equals the exact cone count with 0 free cones: 175 bodies at 2/2 and 42 at 1/1 (`ab U` 19, `ab S` 23). So the 42 truly hold one cone.
2. **Falsified, outside the battery** (MAJOR-1, MINOR-2).
   - Three pairs at `v`: 22 refusal → `BAD`.
   - Curved face: 98 refusal → `BAD` in pre-existing classes.
   - Wedge pinches (a turned run holding two siblings; `R1HANG n=3` 344 times): 261 moves, all `SOUND`, cones exact.
   - Both operands pinched: no hang is reached, and 0 lines move.
   - Near-tangent (19 of the 76 moved poses, ψ ±1e-12…1e-3, tilt 1e-7/1e-4): 1 350 moves, all `SOUND`, cones exact.
   - No moved body has a face through two vertices at `v`.
3. **Holds** (inspection plus instrumentation).
   - A fan's `lo` and `hi` never share an entry: `run_fan` empty ⇒ strut. So the same-entry short-cut in `walks_before` (`insert.rs:1287`) cannot hit a wrapping fan.
   - Struts never turn: the reversed segment is identical, so `held_cut` agrees both ways.
   - Ties need two germs along one direction, which distinct crossings exclude. A nested B plan at a shared vertex refuses before the hang (`:1085`).
   - So "only a turned run holds a sibling" holds. `hang_in_turned` reads the geometry anyway, which also makes it independent of that premise.
4. **Arms unreachable on my poses** (executed). Instrumented eprintln at `insert.rs:1012`, at `:1034` and at `join.rs:457`'s both-sets arm, over ~8 000 runs of four probe families: 0 fires, against 787 hangs.
   - Refusing is right for each arm.
   - `is_up` refused nothing legal on any line I ran: every battery is byte-identical on the frozen head, which includes the check.
   - The `_` arm also refuses a **strut** holding both cuts; the doc and PR body do not list that case (NOTE-4).
5. **Holds** (executed; frozen head vs `fb8c7cbb`, lines moved):
   - `pinch_runs_battery`: 217 of 3 024, the PR's breakdown exactly.
   - 0 moved: `pierce_runs_battery` (4 536), `corner_pairs_battery` (16 380), `join1_r1_reflex_battery` and `j3r2_r1_reflex_battery` (1 152 each), `rc_wide_battery` shards 0/7/13/22/29/36/41/50/57/63/70/83 of 84 (5 760).
   - The non-ignored `join_pierce_runs_sweep` rows: 20/20 pass.
   - The 468 two-key `SOUND` bodies (`ab U`, `ba U`, `ba S`, 156 each) are the same lines on both trees.
6. **Partly holds** (executed, debug, the 20 `join_pierce_runs_sweep` rows):
   - Red, with both new and flipped rows failing as `ClassificationInvariant`: M0 (no `hang_in_turned`), M1 (hang at `v`, `fan: None`) and M3 (flip: mark the holder held by the sibling).
   - **Survive:** M2 and M2b (the hold test reads only the first, or only the second, cut) and M4 (drop the holder-held refusal) (MINOR-1).
7. **Holds, with one class it cannot see.** Readers of `Held` and depth:
   - `mint_plans` sort key (`:729`), `fan_copy` (`:745`) and `by_strut` (`:766`); `mint_directed`'s geometry cross-check (`:1393`). All are slot-generic, and A's `held` is `None` before the hang.
   - `reconcile_pass` `:1085` reads B only, but it runs before the hang, so it is safe.
   - Conflicting ends: `mint_directed`'s guard (shared runs only) and `Sides::new`/`is_up`.
   - The sweep's blind spot is a moved body whose cones land on **separate point keys**. That is not an ends conflict, and it is how MAJOR-1 escapes.

## Findings
**MAJOR-1 — refusal → `BAD` with three pairs at one vertex** (executed). File/line: `insert.rs:972-1040` (`hang_in_turned`) reaching the parked separate-keys class.
- Probe: `r1_three_cubes_at_one_vertex`, 168 poses. `notch343` against three side-2 cubes whose corners touch only at `v`, with diagonals 120° apart.
- 134 lines move from `ClassificationInvariant` "In end … Out end": 101 `SOUND` with exact cones, 11 `SOUND` but on two keys, and **22 `OK BAD`**.
- The 22 are `ab U` ×12, `ba U` ×10, e.g. `i=4 j=3 k=1`. Each has `t3p=false` with the volume exact, and three vertices at `v` on two point keys. `r1_three_one` gives tier 3′ as `CensusUndecidable` "one passes into the other where they touch".
- Base already ships this signature on 8 lines of the same probe (`i=2,3 j=2,4 k=1 U`). So the hang does not invent the class; it hands 22 more refusals to it.
- The PR's "refusal → BAD: 0" holds only on its own batteries.
- Fix pass: either refuse where the hang leaves the shared point on several keys, or file the moved lines against `a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys` with these counts and a red-able row, and say so in the PR body.

**MINOR-1 — the hold test's second cut and the holder-held refusal are pinned by nothing** (executed).
- M2, M2b and M4 pass all 20 rows; the arms at `insert.rs:1004-1012` and `:1023-1035` were never reached in ~8 000 instrumented runs.
- Disclosed as unreached, but with no row that goes red if they go, and no unit test that builds a sibling holding one cut.

**MINOR-2 — curved face at `v`: 98 refusal → `BAD`, plus 4 → `JoinDesync`** (executed). Probe: `r1_curved_notch_pinch`.
- Setup: the notch's face `v→(2,1.15)` bowed into an arc. The oracle is Richardson-extrapolated over 48 and 96 chords; it matches the kernel's volume of the curved notch to 9 digits.
- Both trees ship a large curved `BAD` population on unmoved lines (`operand=false` and/or tier 3′ curved `CensusUndecidable`; base has 873 `BAD`). The 98 moves land in those patterns, with cones exact and one key.
- Exception: `i=9 j=4,5 k=4 ab S` have `t3p=false operand=true`, a pattern absent from the unmoved lines. Tier 3′ there reads "a curved face of one is within reach of the other" (census-escalated, not definite).
- 4 lines go to `JoinDesync` "null-edge copies have not exactly one kept end": typed, so acceptable.
- Owed: disclose and measure, as with MAJOR-1.

**NOTE-1** — The acceptance batteries ran on a tree without the `is_up` check (PR body). My runs on the frozen head close that: 0 moved.
**NOTE-2** — Stale docs:
- `insert.rs:141`, the `Held` doc, says "([`b_runs`]), read once at the plan"; now two writers.
- `:1084` says "Only B's runs nest." (true only before the hang).
- `:9` says "in A the run that holds no third germ" (a turned A run holds its siblings).

**NOTE-3** — `a_nested_pairing_at_a_shared_vertex_refuses_typed` (`join_pierce_runs_sweep.rs:1434`) now asserts `SOUND` for every `ab` run; the name covers only the `ba` half.
**NOTE-4** — The `_` arm at `insert.rs:1012` also refuses a strut holder (`held == 2`, `is_strut`) and a second holder. The doc at `:969` and the PR's three-refusal list do not name these cases.
**NOTE-5** — The row `a-vertex-two-crossing-pairs-cut-…` keeps `status: open` while its build is in review. The PR body's "the other route builds the same tree" is unmeasured prose.

## Style
Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was partial: I read `insert.rs` lines 1-200, 440-1600 and 2040-2130, not end to end.
- **Q1 (likely).** `hang_in_turned` (`insert.rs:977-990`) re-derives `reconcile_pass`'s skeleton (`:1050-1067`): the slot loop, `key`, find-other-plan and the `secs` pick. That is a second copy of "plans at one shared vertex", with no shared home. `Held` is also built two ways, as `b_runs`' chain depth (`:488-496`) and as a hard-coded `depth: 1` (`:1016`). A third nesting reading beside `held_cut`, `holds_whole` and `nested` is now "both cuts `held_cut`" (`:1003-1006`); `holds_whole`'s comment (`:880`) already documents one divergence between them. The class needs a sweep.
- **Q2 (likely).** `mint_plans`' comment (`:724-728`) asserts that a held run is "clear of every other pair's cut, so its strut nests none of theirs". Nothing checks it at mint; it rests on `reconcile_pass`'s fixed point.
- **Q3 (sure).** MINOR-1's surviving mutants.
- **Q3 (likely).** The new row `a_run_turned_…` (`join_pierce_runs_sweep.rs:1472`) has only two poses, both with `n=2`. The `n=3` hang my wedge probe reached (344 times) is pinned by no row.
- **Q4 (sure).** NOTE-2's three sentences. The doc rotted; the code is right.
- **Q5 (likely).** `hang_in_turned`'s doc reads "Two turned runs of one plan would each hold the other's cuts"; the code refuses on held counts, not on turned-ness (NOTE-4).
- **Q6 (sure).** The measurement "`is_up` fires on no line" has no guard or register at the claim site (`join.rs:444`). The PR's "no battery line reaches" the new refusals sits only in prose at `insert.rs:969`, and nothing goes red if a line starts to reach them.
- **Q7 (unsure).** I'd have marked the turned run during `reconcile_pass` and asserted the geometry against the mark, rather than re-reading all runs of every plan at every shared vertex with no record of which turned.

REVIEW COMPLETE

# Review r2: PR #4036 at frozen head 266264cc

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 4 · NOTE 4. Lane isolation: I read no other review branch or session, and saw no glimpse.

## Method

- **Builds.** Release builds of head 266264cc and of the merge base 8793177b ("main"), each in its own CARGO_TARGET_DIR. An instrumented copy of head adds four things, all env-gated or eprintln-only: the survivor count `n` when it is above 2, a line when `walk_order`'s tie fallback fires, a line when `try_rest_union` is entered, and mutants (`R2_MUTANT`).
- **My probe:** `crates/sweep/tests/review_r2_vv_probes.rs` (small target `r2probe`). The corner shapes are:
  - wedges of a square at 270° (L), 345° (notch), 355°, 315°, 195°, 181°, 179°, 120°, 90° (two twists) and 20°;
  - a 4-valent pyramid apex (roof ∩ roof);
  - a 4-valent cross valley (roof ∪ roof).
- **Placements:** each shape's corner on a cube's edge and at its corner, every op, both orders. The sweeps:
  - `grid`: 84 directions × 5 ψ;
  - `tilt`: the 24 axis-aligned frames at eps = 0, 1e-2, 1e-4, 1e-6 and 1e-8, about two axes;
  - `psi`: 120 turns at the PR's four-crossing directions;
  - `hex`: the cube's corner sharing a cone axis with the shape's corner, which crosses up to 6 times.
- **Oracle.** Every line is read by `differential::outcome`: tier 2, tier 3′, certificate, legal operand, and volume against a kernel-free oracle (a signed sum of convex pieces clipped by the cube's half-spaces). Every BAD line was re-checked with an exact-rational (`Fraction`) oracle in Python.
- **Scale:** 145 080 head runs; main ran the same poses.

## Claims

1. **No wrong body: holds, with one qualification (MINOR 1).** Head shows 138 BAD lines:
   - **48 are my f64 oracle's error at eps=1e-6.** The exact oracle agrees with the kernel to 1e-9.
   - **57 are flat181 at eps=1e-6, right volume, with t2, cert and operand true.** Tier 3′ fails with `CensusEscalated` (`pm_census_ef_residual`, margin ±3.54e-9 in band). Main ships 42 of these bodies identical. The other **15 are refusal→BAD**: on main they were `PairingMismatch`, `JoinDesync` or `Euler` (e.g. `flat181 corner frame=11 eps=1e-6 ax=1 cs U`).
   - **33 are valley4, which builds on head only** (main refuses roof ∪ roof). 21 are `CensusEscalated` and pass the n=4 path. 12 are exact eps=0 poses with undeclared or stale contacts in the result; none of these 12 reaches n>2.
   - **No definite refusal→BAD.** Every BAD has the exact volume, and t2, cert and operand are true. Each one fails only the census, the class of the parked `work/reach/boolean-door-runs-the-census-over-its-result`.
   - **Main → head:** 0 GOOD→refusal. Refusal or panic→SOUND: hex 1 152, grid 6 751, psi 5 489, tilt 1 221.
2. **Walk-order argument: holds for four crossings. Its general form is falsified (MINOR 2).**
   - **Four crossings.** For two Jordan curves crossing four times, the only non-crossing matchings on each side ({12,34} and {14,23}) force one cyclic order on both curves. On my corpus every n=4 pose passed the guard.
   - **Six crossings interleave.** With the side-X arcs {16,25,34} and the side-Y arcs {12,36,45}, B's link reads 1 6 3 4 5 2, and neither start pairs adjacently.
   - **Executed:** head refuses `PairingMismatch` 1 422 times, on notch, notch355 and reflex315 on a cube's edge or corner. Every one is n=6 with distinct germs, and the tie fallback never fired (serial attribution, `R2PROBE`). All 1 422 were `PairingMismatch` on main too, so nothing regressed. The guard is still witnessed, though: the module doc's "cannot interleave their crossings … holds wherever the walk orders read distinct germs" is false at n=6 (`insert.rs:27-31`), and so is the saddle doc (`m3_pr6_saddle.rs:29-33`).
   - **Convex six-crossing links build.** hex: 1 026 n=6 runs, all SOUND.
   - **Tie fallback (`insert.rs:412`):** 0 firings across all serial runs, including the exact eps=0 frames, where an edge lies in a face and the poses refuse `UndeclaredCoincidence` upstream.
   - **Degenerate sector:** an exactly flat (180°) corner is not a legal operand (`ScaffoldAtRest`). flat179 is clean: 0 GOOD→worse.
   - I could not make the fallback fire.
3. **Op-dependent start: holds (executed).**
   - **Mutant M4** (B's kept side, which differs only on ∖) and **mutant M4b** (the inverted start) move only GOOD→refusal: M4 by 1 666 runs over hex, psi and tilt on 7 shapes, M4b by 4 998.
   - There are 0 refusal→GOOD under either mutant. So no pose on my corpus where the other start builds and head's refuses.
   - Both ∖ orders and ∩ are covered; M4's red lines are ∖ only.
4. **REST zip not needed: holds (executed).**
   - The instrumented `join1_r1_reflex_battery` on head enters `try_rest_union` 0 times.
   - The row's three wrong-volume poses are SOUND at the closed form: `sqQ1 (−0.5, 0.25) U` gives 15.979166667, and `eBot (−0.5, −0.25)` and `eBot (−0.25, −0.5) U` are SOUND too.
   - The residue is InvalidDeclaration 144, ContactContradicted 24 and ContinuationContradicted 24.
   - No declared-contact code is in the diff (`insert.rs` and one line of `mod.rs`). Declared-flush outcomes do move: 49 + 49 refusal→SOUND. See NOTE 1.
5. **Nothing else moved: holds (executed).**
   - `pierce_runs_battery`: 469 refusal→SOUND and 57 panic→SOUND, which is 526. 0 GOOD→worse, and the 17 face refusals are identical.
   - `join1` and `j3r2` reflex: 47 refusal→SOUND and 2 panic→SOUND each, which is 49. The rest is identical.
   - `rc_wide_battery`, sharded 84 ways:
     - main panics in 8 shards (`orbit_step_at`);
     - where main finished: 1 456 refusal→SOUND, 0 GOOD→worse, and 20 `JoinDesync`→`Escalated{Coincidence(Join)}` (eLeft at 0.003°);
     - the 3 368 runs in main's panicking shards are all SOUND or EMPTY ok on head;
     - head has 0 BAD.
6. **Goldens and mutants: holds.**
   - **Goldens (inspection).** In `perf12_census_*` the witness coordinates are unchanged and only VertexKey 38↔39 and FaceKey 17→18 swap. perf2's second, persisted-text column is unchanged. CI `test` is green on 266264cc.
   - **My mutants, executed:**

     | Mutant | What it changes | Goes red |
     |---|---|---|
     | M1 | within-entry order reversed | four_germ, subset, saddle, cleave, three-corners |
     | M2 | B runs the other way | four_germ, eLeft, subset, saddle, cleave, three-corners |
     | M3 | `shared` only for more than 2 pairs | four_germ, subset, f12 unit (my probe: 564 runs → PANIC at `orbit_step_at`) |
     | M4 | B's kept side | four_germ, eLeft, subset, saddle, cleave, three-corners |
     | M4b | inverted start | the same as M4, plus f12 |

## Findings

- **MINOR 1 (executed). The PR's "0 refusal→BAD" does not hold under `outcome` on wider poses.** 15 flat181 runs at eps=1e-6 go from refusal to BAD bodies that fail tier 3′ with `CensusEscalated`. Every one is escalated, not definite: the volume is exact-oracle right and t2, cert and operand pass. They fall in the parked door-census class, which already holds main's 42 identical bodies, so this is exposure, not a new defect. Probe: `R2_SWEEP=tilt R2_SHAPES=flat181`.
- **MINOR 2 (executed; claim/doc). The general non-interleaving argument is false at six crossings.** The guard fires on distinct germs 1 422 times on head (`insert.rs:27-31`, `m3_pr6_saddle.rs:29-33`; the PR body and the cleave and P0 rows' "fires nowhere"). Typed and pre-existing, so no wrong body. The text should say "four", and the n=6 meander wants a row. No row covers it: the FUSE tie/interleave row is about several vertex pairs. Pose: `notch corner frame=1 eps=1e-2 ax=1 sc U` (tilt sweep).
- **MINOR 3 (inspection). `walk_order` runs for every pair, including n=2, where its order is never read.** At n=2, `run_order` gives `Some(None)` either way and the start rotates only when n>2. `walks_after` refuses in-band as a coincidence (`insert.rs:291-302, 918-935`), so a two-germ pair whose germs share one entry within the band now refuses where main read nothing. I measured 0 GOOD→refusal, so this is latent.
- **MINOR 4 (inspection; Q6). The tie-fallback blind spot is disclosed but not scheduled.** The PR body has a "Blind spot" paragraph. `insert.rs:412` still orders by the other solid's entry, which is the mechanism fix 1 removed elsewhere, and no work row names it.
- **NOTE 1 (D10 hold).** `four-germ-vertex-pairs-run-b-in-a-order` is `status: parked`, `blocked_on: [d10-…]` (`work/join/four-germ-vertex-pairs-run-b-in-a-order.md:5,10`) and listed under plan.md's hold. The PR builds its fix and asks the orchestrator to close it. Its stated reason, the REST-zip wrong body, is gone (claim 4), but closing a held row is the orchestrator's/Ev's call.
- **NOTE 2 (inspection).** `join.rs:72` still cites `plan_null_pairs`' `record_dir`, which this PR removed (now `dirs`, `insert.rs:270`).
- **NOTE 3 (executed).** At the merge base, main panics mid-battery (`insert.rs:1573`) in `pierce_runs_battery` (57 poses) and the JOIN-1 reflex batteries (2). The PR's "line for line" diff is against 81dde823. With panics caught (test harness only) the PR's counts reproduce exactly.
- **NOTE 4 (executed).** The roof ∪ roof valley (`conflicting seam vertex correspondence` on main) builds on head, and its 4-valent corner is mostly SOUND (above).

## Style

Questions I exercised: Q1, Q2, Q3, Q4, Q6, Q7, and Q8 (the parts of `insert.rs` the diff touches, plus reconcile/mint/strut paths, ~1 000 of ~2 000 lines). Q5 was read against the module doc only.

- **Q1 (likely).** `walk_order` (`insert.rs:396-425`) re-spells `precedes` (`insert.rs:1226`): by entry, then `walks_after`. It uses a different entry origin, which is fine for cyclic use, and adds a tie fallback that `precedes` refuses (`None`). That makes two spellings of "walk order round a vertex", one of them with a different answer on ties.
- **Q2/Q4 (sure).** The module doc asserts an invariant that nothing enforces and that is false at n=6 (MINOR 2). The same sentence is copied into the saddle doc and into four `work/` rows.
- **Q3 (likely).** The test-only `insert_null_pairs` hard-codes `BooleanOp::Union` (`insert.rs:110`), so no unit test through it can exercise the op-dependent start. The f12 unit test reddens under M4b but not under M4.
- **Q4 (unsure).** The `run_ends` doc says "its run order follows A's walk order, not necessarily this solid's" (`insert.rs:788-789`). After `run_order`, each solid's direction is its own when n>2, so the parenthetical reads stale for n>2.
- **Q7 (likely).** `let a_run = run_order(n, 0, 1).flatten()` (`insert.rs:328`) computes a constant (`Some(false)` when n>2, `None` when n=2) through the adjacency helper. `shared = pairs.len() > 1` (`:331`) tags a single pair's runs with a flag whose field doc is about other null edges.
- **Q7 (unsure).** The pairing start compares only A's first germ's `sa.0` against `kept_side` (`insert.rs:312`), and B's kept side is never consulted. My corpus found no pose where that matters (claim 3), but the asymmetry is argued only in prose.
- **Q6 (likely).** The measured "0 ties / 0 PairingMismatch across 27 batteries" backs a doc claim but has no guard or scheduled register. My probe's n=6 notch poses are a ready guard row.

REVIEW COMPLETE

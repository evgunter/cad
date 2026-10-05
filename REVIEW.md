# Review r1 — PR #4036 at frozen head 266264cc

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 5. No wrong body found in about 90k probe runs of my own,
none refusal→BAD that is definite, and no pose where the other pairing start is right. Claim 2's argument is false at
six crossings, and the guard it covers now has no committed witness.

Lane isolation: I did not fetch or read any other `…-review-*` branch, session or PR comment. No glimpse.
Builds were release, each in its own target dir: head, the merge-base `8793177b`, the PR's measured main `81dde823`,
and an instrumented head (env-gated `CAD_TRACE` and `CAD_MUT` switches, in a worktree off the branch).
Every probe line goes through `differential::outcome`: tier 2, tier 3′, the certificate, a far-brick union (the
`assert_legal_operand` question), and the closed-form volume.

## Probe rows (this branch)
`crates/sweep/tests/join_vv_review_r1_probes.rs`. Corners: L270, a 343° notch, a 203° shallow reflex, a 90° and a
45° convex corner, and a 4-valent pyramid apex (wedge ∩ wedge, volume 4/3 checked). Each sits on a cube's edge and
corner. Five batteries, about 90k runs, all ops in both orders:
- `grid`: the pierce sweep's 12×7 directions × 4 turns;
- `tilt`: aligned frames tilted 1e-2…1e-8 about two axes, both signs;
- `turn`: three four-crossing directions, turned in 72 steps of 5°;
- `tie`: a cube edge exactly in the top-face plane;
- `hex`: two corner octants turned about a diagonal, reaching six crossings.

Pinned row `a_six_crossing_notch_corner_refuses_pairing_mismatch_on_distinct_germs` (see MINOR-1).

## Claims
1. **No wrong body ships — HOLDS (executed).** Head against `81dde823`: 9 073 refusal→SOUND, 0 SOUND→refusal, and
   259 BAD lines byte-identical. Against the merge-base: 8 560 refusal→SOUND, 0 SOUND→refusal, and 525 base panics
   that head turns into 513 SOUND and 12 typed `Escalated`. The only refusal→BAD are 3 runs, all *escalated*:
   `notch343 edge base=4 psi=0 p=1 t=-1e-7`, pc I, pc S and cp I. Tier 3′ there is `CensusEscalated`
   `pm_census_ef_cut_gap` at margin 8.4e-9, inside the band [1e-9, 1e-8]. Checked without the census, the volume is
   right and t2, the certificate and the legal-operand check all pass (`join_vv_review_r1_tilt_detail`). That is the
   same class as main's 259 BAD lines at t=±1e-7 on the other shapes, which are census or legal-operand failures at
   the right volume. Four crossings at every rotation: 11 487 n=4 plans across the batteries. Of 15 552 `turn` runs,
   15 438 are SOUND or rightly empty. The rest are 108 notch `PairingMismatch` (MINOR-1) and 6 L270 runs at turn=36
   (180°), census-only BAD at the right volume and byte-identical on `81dde823`.
2. **The walk-order argument — FALSIFIED (executed), see MINOR-1.** Two simple links cannot *cross* their pairing,
   but non-crossing forces cyclic adjacency only at n=4. At n=6 a nested matching is legal, and the guard fires on
   distinct germs: 324 runs, all on notch343, with 0 tie fallbacks. The tie fallback fired 0 times in all my batteries
   and in the PR's batteries, including the dedicated `tie` battery. I could not make it fire (NOTE-3).
3. **The op-dependent start — HOLDS (executed).** Mutant M4i inverts the start on every n>2 plan. Across 75k
   comparable runs, every op and both orders, it builds 0 runs that head refuses and breaks 9 132 that head builds,
   with no BAD. No pose prefers the other start. pyr4 is excluded: under M4i its wedge ∩ wedge operand itself fails
   (`JoinDesync` "conflicting seam vertex correspondence").
4. **Un-parking needs no REST-zip ruling — HOLDS (executed; no positive control).** The three poses run through the
   REST zip on main (sqQ1 (−0.5, 0.25) ∪, eBot (−0.5, −0.25) ∪, eBot (−0.25, −0.5) ∪) are all SOUND at the oracle in
   both batteries. The REST door (`ops.rs:817`) is entered 0 times on head across both reflex batteries, rc_wide,
   pierce and join1_r1, and no join refusal occurs there. No declared-contact, REST or flush code is in the diff.
   Declared-flush *outcomes* move only through the shared planner (49+49 refusal→SOUND). See NOTE-4 on the hold.
5. **Nothing else moved — HOLDS (executed).** rc_wide in 84 shards, pierce, both reflex batteries and join1_r1
   against `81dde823`: pierce 526, the reflex batteries 49 each and rc_wide 2 288 refusal→SOUND. join1_r1 is
   byte-identical. 0 SOUND→refusal and 0 refusal→BAD. The PR body says rc_wide 2 300 and 20 Escalated; I count 2 288
   and 19.
6. **Goldens and mutants — HOLD (executed).** A key-free dump of `kitchen_sink` is byte-identical on main and head:
   every name, with every arena key replaced by its entity's geometry (447 rows). The pinned digests pass on their own
   tree. So only keys moved. My mutants, one per fix, each redden `four_germ_vertex_pairs_build_every_op`:
   - M1x, within-entry order reversed: `PairingMismatch`;
   - M2x, B's run inverted: `ClassificationInvariant` and `Euler`;
   - M3x, `shared` on A only: a panic at `orbit_step_at`;
   - M4x, start read from B's kept side: `Euler` and `JoinDesync`.

   M1x, M2x and M4x also redden the cleave, saddle and three-corners rows. M3x survives the topo rows.

## Findings
- **MINOR-1 (claim/doc; executed)** `insert.rs:27-31`, `m3_pr6_saddle.rs:29-34`, PR body fix 1. "The guard holds
  wherever the walk orders read distinct germs" is false. notch343 crosses the cube six times at 54 poses (324 runs).
  Traced: A pairs `(0,3)(2,4)(5,1)` and B reads them at `(2,5)(0,1)(4,3)`, nested and non-crossing. Every op refuses
  `PairingMismatch`, as on main. It is fail-loud, not BAD. The doc claim and the deleted "guard unwitnessed" note need
  correcting, and the six-crossing nested pairing has no work row.
- **MINOR-2 (test-gap; executed)** `insert.rs:318-328`. No committed row drives `plan_null_pairs` into the adjacency
  guard: the PR deleted `f12_pairing_mismatch_guard`, and `f12_run_order_reads_cyclic_adjacency` tests `run_order`
  alone. Mutant M5 makes a non-adjacent pair fall back to `run_degenerates`. It survives 899 topo integration tests,
  396 `boolean` unit tests and the PR's sweep rows. Only the pinned row above goes red. Without the guard the n=6
  poses refuse later, as `ClassificationInvariant` "a vertex at a shared point is the In end of one null edge…".
- **NOTE-1 (executed)** `insert.rs:1168`. Fix 3's safety net is a panic. Without fix 3 (M3 or M3x),
  four-germ poses panic in `orbit_step_at`'s `unreachable!` instead of refusing. On the merge-base main, 525 of my
  probe runs and 11 battery files panic there. "Now reached only at a vertex holding one null edge" is argued, not
  enforced. Already filed on TOPO per the brief.
- **NOTE-2** The PR's "main" is `81dde823`, not its merge-base `8793177b`. On the merge-base, main panics (NOTE-1)
  partway through pierce, both reflex batteries and 8 rc_wide shards. The diffs above use both.
- **NOTE-3 (executed)** The tie fallback (`insert.rs:410-412`) is unreached by every probe, the dedicated `tie`
  battery included. So it stays an unwitnessed path that orders by the other solid's entry (style S3).
- **NOTE-4 (process)** The parked `four-germ-vertex-pairs-run-b-in-a-order` (blocked_on D10) has its fix built,
  because fix 2 is load-bearing for the undeclared P0 row: M2 reddens it. The row stays `parked` with `## Built`
  evidence. Closing it under the hold is the orchestrator's or Ev's call.
- **NOTE-5 (executed)** The 3 escalated refusal→BAD of claim 1 sit in the census's escalate band, and on main the
  same tilt is census-BAD for four other shapes. That is CONTACT's census class, not this PR's.

## Style (exercised: Q1 Q2 Q3 Q4 Q5 Q6 Q7, and Q8 by structure only)
- S1 Q1 `insert.rs:334-352` — likely. The run direction now has two rules: the walk order for n>2, and
  `run_degenerates`' orbit-swallow test for n=2. They are two spellings of "the run that holds no third germ".
- S2 Q1 `insert.rs:396`, `:1226`, `:816` — likely. There are three spellings of "position round the vertex":
  `walk_order` (entry index then `walks_after`), `precedes` (physical-sector rel then `walks_after`) and `held_cut`'s
  `rel`. The file has about 12 order or hold predicates for one vertex. Look at `nests`, `nested` and `tied_held` too.
- S3 §1-trap `insert.rs:410-412` — likely. The fix that removes "order by the other solid's sector" keeps exactly
  that as its silent tie fallback, rather than refusing typed.
- S4 Q4 `join.rs:72` — sure. It cites `plan_null_pairs`' `record_dir`, which this PR removed.
- S5 Q4 `insert.rs:786-789` — likely. `run_ends` says the run order "follows A's walk order, not necessarily this
  solid's". After fix 2, B's `from`/`to` follow B's own `run_order`.
- S6 Q2/Q5 `insert.rs:129-132`, `:698-701` — likely. The `shared` doc and the PR body route a pair's own runs
  "through `reconcile_shared`". But `reconcile_pass` reads only *other* plans' cuts, so a plan's runs are never checked
  against each other. That rests on fix 2 alone.
- S7 Q3 `insert.rs:1870` — sure. The renamed unit test cannot go red if the guard is dropped (MINOR-2).
- S8 Q3 `insert.rs:110` — sure. The test-only `insert_null_pairs` hard-codes `BooleanOp::Union`, so no unit row
  reaches fix 4 for ∩ or ∖.
- S9 Q6 PR body — likely. "0 ties" and "0 PairingMismatch across 27 batteries" are one-off instrumented counts with
  no guard or register. The six-crossing refusals have no row.
- S10 Q7 `insert.rs:309-314` — unsure. Fix 4 is asymmetric: A's runs go to A's kept side and B's fall where they
  fall. The PR body's defence of the B side is longer than the rule. S_ba rides on the operand swap.
- S11 Q4 `m3_pr6_saddle.rs:193-222`, `review_m3_pr6.rs:400-408` — sure. Those hunts treat any `PairingMismatch` as
  "lands here", but the notch fires it outside both of them.

REVIEW COMPLETE

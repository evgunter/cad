# Review r1 — PR 4274, frozen head 4d985b57

**Verdict: APPROVE-WITH-FIXES** · MAJOR 1 · MINOR 2 · NOTE 4

Method: release builds of head and of main at the merge base `517b81a2`, each in its own
target dir, plus a third build of head with env-switched mutants and a holder trace
(`R1_MUT`, `R1_TRACE`; not pushed). The new probe row is
`review_r1_depth_two_at_a_shared_vertex_probe` at
`crates/sweep/tests/join_pierce_runs_sweep.rs:2566` (ignored, prints lines).
- **The probe.** It takes the two eight-crossing poses of
  `eight_crossing_corners_nest_two_deep_and_build_every_op`, `notch343×notch343 r2` and
  `wedge343×wedge330 r1`. A side-4 cube is pinched onto the posed corner at the shared
  corner. Its octant sits above the posed corner's top face, tilted 0/0.3/0.55/0.61 rad
  (0.6155 rad is tangent), at 4 azimuths × 3 spins.
- **The lines.** That gives 48 cube poses × 2 orders × 3 ops = 576 lines. Each line is read
  through `differential::outcome` plus `pierce_point_finding` (one key, `cones_at`
  count, `check_mesh`).
- **Lane isolation:** nothing read from any other review branch or session.

## Battery reproduction (executed, main vs head)
| battery | moved | moves |
|---|---|---|
| pinch_runs | 102 | SharedVertexCrossings→SOUND, all `ba` (claim holds) |
| four_pairs | 307 | →SOUND 182 (all `Some(None)`, one key), →PinchConesOnSeparateKeys 120, →SharedVertexCrossings 5 (claim holds) |
| pierce_runs, corner_pairs, join1_r1_reflex | 0 | byte-identical |
| r1 probe (576 lines) | 276 | →SOUND 192 (all `Some(None)`), **→ClassificationInvariant 84** |

There are 12 probe SOUND lines whose finding is "vertices do not share one point". All 12
are `notch343` lines, U or `ba` S, at t=2/3 a=1, and they are byte-identical on main, so
they are pre-existing and not moved.

## MAJOR

**M1. The PR routes a legal pose that main refused typed into the In/Out-end invariant.
Demonstrated by execution.**
- **Where:** `wedge343×wedge330 r1` + cube, `ba` U and `ba` S, at 42 of 48 cube poses
  (every tilt, including t=0).
- **Main:** `SharedVertexCrossings` (the up-front guard).
- **Head:** `ClassificationInvariant { "a vertex at a shared point is the In end of one null
  edge and the Out end of another" }`. That is the exact symptom `hang_at_shared`
  (`insert.rs:1021`) exists to prevent, and the PR's mutant 2 names it.
- **Trace of B's plan** (n=8, after the reconcile): arcs `[[6,1],[7,0],[3,4],[2,5]]`, struts
  `[F,F,T,T]`, holders `[None, Some(0), Some(3), None]`.
  - The reconcile turned **two** runs of one plan: the outer fan `[0,7]`→`[7,0]` and the
    inner fan `[1,6]`→`[6,1]`.
  - That leaves the strut chain `[2,5]⊃[3,4]` outside every fan. The inner strut is held
    by a strut with `fan: None`, so it mints at the shared vertex itself.
- **Same holders, different outcome.** The notch pose reaches the identical laminar
  family (`[[2,5],[6,1],[7,0],[3,4]]`) in `ba` U/S and builds SOUND on one key. The arc
  reading alone therefore does not decide buildability here.
- **What the PR removed.** It dropped `sibling_holders`' "a strut holder" refusal and the
  premise that the reconcile "never turns two". `mint_plans`' comment (`insert.rs:761`,
  "its holders, which the reconcile cleared of every other pair's cut, so its strut nests
  none of theirs") is unverified for a strut-only chain left at a shared vertex. I suspect
  it, but have not root-caused it.
- **Severity.** No wrong body ships; the invariant catches it. But this is a refusal
  downgraded from typed to an internal-invariant error on legal input, which this PR newly
  reaches.
- **Fix wanted:** either root-cause it, or refuse it typed (for example, a strut-only chain
  outside every fan after a turn). Add the probe's wedge `ba` pose as a row.

## MINOR

**m1. Test gap: the PR's three new claims are unpinned at a shared vertex (depth > 1, the
innermost holder, `by_strut`). Mutants executed.**
- Cap the depth at 1 (`held_by`, `insert.rs:510`), drop `by_strut`, or pick the outermost
  holder (`arc_holders`, `insert.rs:607`). Each goes red on only two things: the unit test,
  and `eight_crossing_corners_nest_two_deep_and_build_every_op`, which is a **non-shared**
  vertex.
- `pinch_runs_battery`, `four_pairs_battery` and every shared-vertex row are byte-identical
  under all three.
- With `R1_TRACE`, four_pairs never exceeds depth 1 (840 plans at depth 1, 14 297 with no
  holder).
- My probe does catch them, as SOUND→`ClassificationInvariant`:
  - depth1: 60 lines, incl. the 12 depth-3 `wedge ba` lines;
  - nostrut: 108;
  - outer: 164.
- So claim 3 holds by execution: a depth-3 chain `[0,7]⊃[1,6]⊃[2,5]⊃[3,4]` (fan, fan,
  strut, strut) at the shared vertex builds SOUND on one key in `wedge ba` U/S at 6 poses.
  But no committed row protects it.

**m2. The cover arm is reached by no pose I or the PR could find. Executed.**
- Replacing the cover refusal with acceptance (`R1_MUT=cover`) moves 0 lines in pinch,
  four_pairs and the probe. Only the unit test goes red.
- `hang_at_shared` never returned `None` (cover or crossing) in four_pairs or the probe
  (trace: 0 `holders=None`).
- The 5 remaining `SharedVertexCrossings` come from the reconcile arm, as the PR says.
- So the claim "pinned by the unit test" is literally true, but it is unexercised by any
  geometry.
- The arm is right to refuse: two arcs that each hold the other's ends cannot both be
  minted at nested copies.

## NOTES

- **N1 (claim 1, inspection; holds).** Within a plan, the chords are non-crossing:
  - A pairs consecutively;
  - B is checked by `arc_holders`, which refuses a crossing the same way main's span test
    did.
  Any choice of side per chord is then disjoint, nested or covering, so the laminar
  statement is a theorem. Holders of one run are mutually nested, so the lengths are
  distinct and `min_by_key` is the unique innermost. The `successors` chain strictly
  lengthens, so it terminates. The wrap is checked: `len`/`past` are modular, and the unit
  test's `[4,1]` and `[5,0]` plus the trace's `[6,1]`/`[7,0]` agree. Germs cannot share a
  walk position (`walk_order` refuses a tie).
  Two turned runs in one plan **are** reachable (M1's trace and the notch probe), and the
  reading handles them laminarly.
- **N2 (claim 2; holds).**
  - By inspection: for B's non-wrapping intervals plus length-1 wrap arcs, `arc_holders`
    gives main's holders exactly, and a cover is impossible there.
  - Executed: pierce, corner_pairs and reflex are byte-identical, and the eight-crossing
    depth-two row passes.
  - Not run: rc_wide shards (budget).
- **N3 (claim 5; holds, likely).** All 120 lines are `ba U`/`ba S` at 60 poses. At those
  same 60 poses, `ab U` already refuses `PinchConesOnSeparateKeys` on main, and
  `ab S`/`ba I` build SOUND. The refusal lands exactly on the ops that keep the pinch
  operand's cones: the parked class, not a newly refused buildable body. I did not rebuild
  without the no-turn `Hang` feed to see what would ship.
- **N4 (claim 7).** The PR's four mutants: 1 (guard, `R1_MUT=guard`) reddens
  `…builds_in_both_orders`. 2 (holders read once, `R1_MUT=once`) reddens 4 shared-vertex
  rows. 3 (cover) and 4 (outer) redden the unit test. Mine are in m1.

## Style
Questions exercised: Q1, Q2, Q3, Q4 and Q7. Q5, Q6 and Q8 were skimmed only: I did not
read the whole 3159-line `insert.rs`, for budget (unsure).
- **S1 (Q1/Q2).** `hang_at_shared` re-derives the walk length as `2 * runs.len()`
  (`insert.rs:1043`), while `plan_null_pairs` had `n = survivors.len()`. One quantity has
  two spellings, held equal only by `chunks(2)` pairing. **likely**
- **S2 (Q4).** The `mint_plans` comment at `insert.rs:761` now asserts that a held run's
  holders were "cleared of every other pair's cut" by the reconcile. A strut holder that is
  never turned, and that `holds_whole` lets keep another pair's strut, would falsify it.
  M1 may be that case. **unsure**
- **S3 (Q7).** `arc_holders` evaluates `holds(o, k)` twice per pair and encodes "cover"
  as a guard failure falling into `_ => return None`. The cover arm and the crossing arm
  share one silent `None`, so the caller cannot name which refused, and both become
  `SharedVertexCrossings` (`insert.rs:619`). **likely**
- **S4 (Q3).** `arc_holders_nest_the_runs_a_turn_reroots_and_refuse_a_cover` feeds
  hand-made arcs. No row ties a real plan's `SideRun::walk` to them, so a wrong `walk`
  swap in `directed`/`turned` would go red only through geometry rows (see m1). **likely**
- **S5 (Q2).** The doc on `arc_holders` argues laminarity at length in prose. It is
  correct (N1), but nothing asserts it: a `debug_assert` would. **unsure**

REVIEW COMPLETE

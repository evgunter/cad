# Review r1: PR 4050 at frozen head `0e0d871a`

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 4.
No wrong body ships anywhere I could reach, including n = 8 and n = 10 vertex pairs and two levels of nesting. Every change from main
is a `PairingMismatch` that now builds SOUND or refuses typed. Two invariant refusals and the new `SharedVertexCrossings` guard are reached on paths the PR calls
unreached, and one mutant of the holder rule survives every committed row. Lane isolation: I read no other `*-review-*` branch of this PR.

**Method.** Release builds of head and of main at the merge base `6e57858c`, plus a trace build (head with env-gated `eprintln`s,
uncommitted) used only to classify runs. Its lines are byte-identical to head's. Every line goes through `differential::outcome`
(tier 2, 3′, certificate, legal operand, volume). Probes: `crates/sweep/tests/join_6x_review_r1_probes.rs`. Each operand is a signed
sum of convex half-space pieces, so `vol(A∩B) = Σ sᵢtⱼ vol(Pᵢ∩Qⱼ)` holds kernel-free. A second cube or a second reflex prism is
posed rigidly about the shared corner. Sets: `tilt` (the pin's 3 poses + 3 more six-crossing poses, tilted 1e-3…1e-11 about 6 axes),
`turn` (n330, n350, sheared s343 on a cube corner/edge, 504 frames each), `rand` (6 000 seeded rotations over 16 shape pairs incl.
notch vs notch, notch vs L, sheared vs shallow, valley4 vs notch): 55 260 runs, plus 3 024 in `r1x_shared_vertex` (pinch).

## Claims

**1. No wrong body ships: HOLDS** (executed). Head: 54 338 SOUND, 854 rightly EMPTY, 68 typed refusals; 0 BAD/PANIC/EMPTY WRONG.

The trace counts 3 432 n=6, 42 n=8 and 12 n=10 plans, and 2 490 runs with a held run. Those span every shape in the brief:
notch 300/330/343/345/350/357°; sheared notches s343 and s320; valley4 (4-valent) on a cube's corner and edge; reflex vs reflex.
Near-tangent tilts down to 1e-11 build or refuse `Escalated`, identically on main. There is no refusal→BAD, definite or escalated,
so nothing needed checking without the census. A mirrored prism corner is a rotation of itself (a planar wedge is mirror-symmetric),
so the sheared notches are the chirality cases. Both orders and all three ops were run throughout.

**2. The nesting argument: HOLDS for what it states** (executed, `REVIEW-r1-meander.py`, an independent model).
- **What the model covers.** Every closed meander at n = 4, 6, 8 and 10 (2/8/42/262), both choices of B's interior, all four
  kept-side pairs, and every B walk start and direction. A port of `b_runs` (insert.rs:417) never returns `None`. Its runs are
  laminar, `holder` is the innermost holding run, and every kept B region holds arcs of exactly one result-link cycle. 0 failures.
- **What it does not cover.** Like the PR's, it checks regions, not the join's merging of A and B copies. "One vertex per cycle"
  stays an inference.
- **Brief correction: the holder is NOT always a fan.** Notch vs notch reaches strut holders: 38 runs, 36 SOUND at depth 1. My poses
  also reach shapes no committed row reaches: depth-2 nesting (24 runs), two siblings held by one run (≥19), and n = 8 and 10.
  Every one is SOUND except MINOR-1.

**3. The guard: PARTLY.**
- **`PairingMismatch` is dead on realizable input.** Guard 1's crossing branch and guard 2 fire 0 times in 58k traced runs (NOTE-1).
- **The shared-vertex refusal is REACHED, contrary to "no battery reaches this".** A pinch operand (two cubes touching only at a
  corner, united undeclared) against p343 refuses `SharedVertexCrossings` in 102 runs, `ba` order, all three ops. That is right and
  typed. The guard covers only the case where the nested side's vertex is the shared one; see MINOR-3.

**4. Nothing else moved: HOLDS** (executed, main `6e57858c` vs head, release).
Byte-identical: `pierce_runs_battery` (4 542 lines), `join1_r1_reflex_battery`, `j3r2_r1_reflex_battery` (1 158 each),
`rc_wide_battery` in 28 shards (40 320). Mine: exactly 2 490 lines differ, all main `PairingMismatch` → 2 488 SOUND + 2 refusals
(MINOR-1). Pinch: 204 differ, all main `PairingMismatch` → 34 SOUND, 102 `SharedVertexCrossings`, 68 refusals (MINOR-3).
0 SOUND→refusal, 0 →BAD.

**5. The mutants are real: HOLDS** (executed; built, run on the `join_pierce_runs_sweep` rows and `rand`, removed). root (PR):
pin red, 1 268 not SOUND. order (PR): pin red, 1 057 "holder was not minted before it". corner (PR): pin red, PANICs as reported.
outer (mine, holder rule): **pin green**, 24 `rand` lines change (MINOR-2). No mutant ships a BAD body.

## Findings

**MINOR-1. A strut held by a strut that a fan holds mints at the wrong vertex** (executed). `insert.rs:621-636`, `:1330`.
`fan_holder` reads only the immediate holder. When that holder is a strut, the run mints at `vertex(i)`, but the strut itself was
minted at its fan holder's copy, where the corner now lives. `corner_bound` then refuses `ClassificationInvariant` "an earlier run at
the vertex carried a strut's corner to its copy".
Run: `n343 n330 k=105 ba U`/`ba S` (n=8; trace nest=2, holder_fan=false); main `PairingMismatch`. Not BAD, but it falsifies
"a run held by a strut keeps the hang-at-the-tip path" and is an invariant refusal on valid input. With the outer mutant (holder =
the fan) both build SOUND, which confirms the cause. Unscheduled.

**MINOR-2. Nothing committed pins the innermost holder, or any nesting at n ≥ 8** (executed). `insert.rs:459`.
`min_by_key`→`max_by_key` survives `six_crossing_corners_build_every_op`, `f12_b_runs_nest_a_pair_and_refuse_a_crossing` and
every `join_pierce_runs_sweep` row (all n=6, one holder, where innermost = outermost); only my n=8 notch-vs-notch poses catch it.
The PR's meander model is not committed either, so nothing re-measures the argument the change rests on.

**MINOR-3. Nested pairing against a pinch, with the nesting solid's partner shared** (executed). Guard: `insert.rs:838` reads holders
in the slot whose vertex is shared, which is never A's. Clash: `:1290`.
`p343 pinch … ab` (notch = A, its vertex shared by two plans; the pinch cube's B vertex nests): I SOUND; U and S refuse
`ClassificationInvariant` "a vertex at a shared point is the In end of one null edge and the Out end of another", raised in operand
A (trace): 68 runs, main `PairingMismatch`. The PR says such a plan refuses `SharedVertexCrossings`; here an invariant error
refuses it. Cause unsure (A's senses at the shared vertex may follow B's arbitrary run direction for the nested pair). Unscheduled.

**NOTE-1. Guard 1's crossing branch and guard 2 are unreachable in practice** (executed: 0 hits in 58k traced runs; my model never
yields a crossing). That is right for a fail-loud guard. The unit row witnesses `b_runs`' `None`, but nothing witnesses guard 2 at
this head.

**NOTE-2. The pinch probe runs without the pinch's carried records.** Its operand is an undeclared union of two cubes, and every
built body still passed 3′. No declared contact or REST zip is touched, so the D10 hold is respected.

**NOTE-3. Pre-existing refusals, identical on main:** `valley4 cubeE` 36 `JoinDesync` "every chord arc" (the PR's filed
`a-roof-cross-valley-…` row); `valley4 cubeC k=52` 6 `Escalated`; p343 tilts at 1e-9, 24 `Escalated`.

**NOTE-4. Unconfirmed claim.** The PR's 373k-run battery diff was not re-run in full. I re-ran the four batteries named above, plus my own.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only in part: I read `insert.rs` 1–1360 of 2 367 (the header, plan, mint,
reconcile and `mint_directed`), not the tail.

- **S1 (Q4/Q5, sure).** Two passages still say every run "holds no third germ in that solid's own walk order": the module doc at
  `insert.rs:8-10` and the comment in `plan_null_pairs` at `:330-331`. Both are false for B's nested runs, which hold the held pair's
  germs. This is doc rot; the code is right.
- **S2 (Q4, sure).** The `SharedVertexCrossings` doc at `mod.rs:1973-1981` still describes only "no run … that holds none of another
  pair's cuts". The new trigger at `insert.rs:838` fires before any cut is read, on a nested plan alone. The Display text is broad
  enough to cover it.
- **S3 (Q1, likely).** Two nesting vocabularies in one file: within a plan `holder`, `b_runs`' `holds`, the `nest` closure
  (`:580`); across plans `nests` (`:698`), `holds_whole`, `nested`. `nest` and `nests` differ by one letter and answer different questions. MINOR-1 sits exactly at their seam. The PR's own note
  files `b_runs` as a third run rule beside `run_order`. Look also at `reconcile_shared`'s turning rule.
- **S4 (Q2, likely).** The comment at `:619-620`, "one a strut holds hangs at the strut's tip ([`nests`])", asserts a path the PR
  says nothing reached. It is the path MINOR-1 breaks at depth 2.
- **S5 (Q6, sure).** Measured claims with no guard or register: "no battery reaches" the strut-holder path and the
  shared-vertex refusal (both reached, above), "every holder was a fan", and the uncommitted meander model.
- **S6 (Q7, unsure).** The copy is recovered after the mint by asking which end is not `at_vertex` and searching the halves
  (`:655-667`), where `mint_directed` could return it. Separately, the sort key puts nest depth before the shared-vertex
  "struts before fans" rule for every plan. That is safe only because B-slot nesting at a shared vertex is refused and A never
  nests: an invariant held by convention.
- **S7 (Q3, likely).** The pin is strong (every op SOUND at a closed form), but its premise excludes n ≥ 8, so it cannot go red on
  holder selection or on two-level nesting (MINOR-1 and MINOR-2).

REVIEW COMPLETE

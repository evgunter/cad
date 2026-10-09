# Review R2 — INTENT stage 4 PR A (#4364), frozen head c43e146c

Reviewer R2 (Opus), concurrent pair, `docs/DUAL-REVIEW-PROTOCOL.md` at da5dbd153.
Base `git merge-base c43e146c origin/main` = 4908d4725. I read no other lane's
branch, scratchpad or PR comment.

## Verdict: APPROVE-WITH-FIXES

No MAJOR. The join builds every union the zip built, with the same census. Every
zip-refused union it now builds passes tier 3′. Every mutant of the three new rules
goes red, except one inert branch. The fixes are stale text, an over-stated
digest claim and a test gap.

## Findings

**MINOR-1 (claim/doc). Present-tense "REST zip" text survives; the PR's sweep
pattern could not match it.** — `demos/tour/src/crosslap.rs:88-92` (a code comment,
and the stop label string `"declared mated union (M5 S1 REST zip)"`),
`demos/tour/src/lily.rs:2273`, `crates/topo/tests/review_m3_pr5.rs:379`,
`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs:898`. The PR swept
`declared-REST|REST lane|rest lane|zip lane|rest.rs|RestZip`, and none of those
patterns match "REST zip". Found by `rg 'REST zip'` at c43e146c. The `crosslap.rs` file
had its header and captions rewritten in this PR and keeps the label two hunks
away. Confidence: **sure**.

**MINOR-2 (claim). "Every other body digest is bit-equal" (spec §11 row 2, the
work item) does not hold, and the PR measured only the pinned ones.** — My probe at
`boolean_op_with` ran on both trees over all of sweep, keyed per test by
(op, operand fingerprint, declared). Undeclared tangent-site unions that the join
built on main also changed bits on the head. Their volumes differ at the last ulp,
deterministically:
`join2_r1_probes::unions_through_ring_vertices…` 24.848699434776332 → …336 (×2),
`join2_r2_probes::a_channel_whose_arm_ends…` 40.285398163397446 → …44,
`a_like_far_ends_tie…` 42.58539816339744 → …45, plus one call that moved the other
way. F/E/V/shell counts, contact records and tier 3′ are identical. This is the
expected reach of `across_tangent` into undeclared joins, so it is benign. But
"No golden moved" is the PR's true but narrower claim, and the spec row needs
restating as "pinned digests". Confidence: **sure** that the bits moved, **likely**
that it is benign.

**MINOR-3 (test gap). No topo row pins any of the three new rules, and one branch
is inert.** — Under each mutant below, topo's 580 `chord_join|boolean` lib tests
stayed green. Only sweep integration rows bite, so a topo-only change can regress
them without a topo signal. The seam shift
`chord_join.rs` `Ok(Sign::Zero) => u_p - tau` (in `chart_ring_side`, under `full`)
survives every row and every probe of mine (M4). Either it is dead or its
case is unrowed. Its comment ("past which the ray's degenerate rows put it")
claims it is needed. Confidence: **sure** (unpinned), **likely** (inert).

**NOTE-1. Ruling (3)'s peg-in-socket is not this branch's.** — On main 4908d4725
with the zip door intact, my probe shows the declared peg ∪ socket built by the
**join** (`zip=false`, tier 3′ ok, vol 0.048854541785392246) at ε = 3e-7, 1e-6
and 2e-6. The doc in `mate7a_torus_rest.rs:227-233` was already stale on main, and
closing `join/peg-in-socket…` is right. The "beyond the spec, unbisected" credit
belongs to main's merges, not to `across_tangent`. Confidence: **sure**.

**NOTE-2. Ruling (4)'s shape-2 evidence is not in any row.** — No test in
`rest_zip_admission.rs` asserts the line kiss's tier-3′ failure, so the "20 runs"
come from the implementer's probe. In my probe over every sweep suite, none of
main's 125 zip-built unions failed tier 3′ (structural census). The only
`UndeclaredContact` failures in `rest_zip_admission` are **undeclared** unions,
which never reached the zip and are identical on both trees. So "as the zip
shipped" is not corroborated by any suite row. The re-pointed row should name a
fixture. Confidence: **unsure**.

**NOTE-3.** The `just over one period` refusal arm (`chart_ring_side`, `Sign::Negative`)
could not be exercised: `cyl_wall_sheet` over (0, τ+1e-3) panics `WindingExceeded`
before re-homing. Confidence: **sure** (not exercised).

## Claims

**C1 soundness — HOLDS, exercised beyond the PR's 74.** I paired by operand content,
not call ordinal, so divergence does not break the pairing. Main produced 125
zip-built unions over 16 tests, which are **113 distinct operand pairs**. All 113
match on the head:
- contact records identical per kind;
- F/E/V/shell/solid counts identical;
- tier 3′ ok on both trees;
- worst relative volume difference 4.66e-16.

**Main refused, head builds: 60 distinct operand pairs**, all with tier 3′ ok on
the head. On main these were 45 `Join` (the zip reproduced the join's refusal),
8 `ResultInvalid` and 7 `RestZipUnsupported`. They come from `rest_zip_admission`
(56), `rest_nested_strut` (2) and `join2_r2_probes` (2). Each is also asserted at
its closed-form volume by its own test. No non-zip union flips between build and
refuse. The tier-3′ failure profile across all of sweep is identical on both trees.

**C2 half-open / one period — HOLDS.** A scratch row on the
`sibling_escalation_rows` island fixture (run [0.5,1]×[0.3,0.7]) read:
- at the high ruling's azimuth, v=0.5 → `Undecided`; at the run vertex (1,0.3) →
  `Undecided`; v=0.2 and v=0.8 → `Out`;
- the same at the low ruling (0.5): `Out`/`Undecided`/`Out`;
- (0.75,0.5) → `In`; 0.999 → `In`, 1.001 → `Out`;
- ±1e-13 in band → `Out` (correct).

On a full sheet [0,θ]×[0,1]:
- θ = τ and θ = τ−1e-13 (in band) read as one period. Seam azimuth mid → `Undecided`
  (on the seam strut); below → `Out`; just before and after lo → `In` (correct, the
  azimuth wraps); interior → `In`; above → `Out`.
- θ = τ−1e-3 reads not-full, and the gap just before lo → `Out`.

On main, every on-ray case above returned `Undecided` (M1 reproduces this).

**C3 across_tangent — HOLDS by execution of the suites at three ε, plus a
reading.**
- The neighbour index is right: `start` is the next orbit chord, so
  `i.start == (i+1).end` (`sectors.rs:105-111`, `insert.rs` docs l.37-40).
- The crossing codes are forced by the clean In/Out pair across the shared bound.
- The face guard `sectors[k].face == f` keeps a wrong neighbour from moving.
- Swapping the direction (M7) turns 11 sweep rows red, so the side is pinned.
- I built no curved-bound-specific scene beyond the suites. They include
  plate-on-rounded-plate (curved rim bounds) and are green at default, 1e-6 and
  1e-12.

Confidence **likely**.

**C4 refusals — HOLDS for the suites.** The 7 `RestZipUnsupported` and 53 other
main refusals that the head builds are all sound (C1). Every other refusal on main
refuses identically on the head. `pncad-py` and `.pyi` carry no `rest_zip`.
`carrier_pair.rs`'s ten items are byte-identical to `rest.rs`'s at base (diffed
per item).

**C5 pins — HOLD (tour not run).** At c43e146c, `--profile default`, private target:

| ε | topo | sweep | editor-core |
|---|---|---|---|
| default | 2643/2643 | 2483/2483 | 2918/2918 |
| 1e-6 | 2643/2643 | 2483/2483 | 2918/2918 |
| 1e-12 | 2643/2643 | 2482/2483 | 2918/2918 |

The 1e-12 failure is `parallel_cylinder_join::a_tipped_rod…`, which fails identically
on main 4908d4725 (I re-ran it there). The timing row passed here. The PR's
editor-core ε gap on its own head is now closed: green at both ε. I did not run
the tour, so main's tour red (#4353) is unexamined.

**C6 the rows bite.** Each mutant was rebuilt and run against the sweep subset
(112 tests over 13 suites) and topo's 580 `chord_join|boolean` lib tests:

| Mutant | sweep red | topo red |
|---|---|---|
| M1: half-open reverted to main's bail | 6 (full_turn_bore ×4, mate2 r1/r2) | 0 |
| M5: half-open direction flipped | 0 — equivalent, every probe identical | 0 |
| M6: along-ray row bails | 6 | 0 |
| M3: one-period window refuses | 1 (`a_blind_shaft_unions_on_and_off_the_bores_seam`) | 0 |
| M4: seam shift dropped | **0 — survives** (MINOR-3) | 0 |
| M2: `across_tangent` → identity | 11 (join2_r2 ×4, rest_zip_admission ×4, reach_continuation ×2, rest_nested_strut) | 0 |
| M7: `across_tangent` direction swapped | 11 | 0 |

## Implementer rulings

1. **chart_ring_side, half-open + one period — AGREE.** The rule is correct (C2).
   Its direction is a free choice that no row pins (M5); that is fine for a
   half-open rule, but say so. Drop the seam shift or row it (MINOR-3).
2. **across_tangent, partner-edge chord removed — AGREE.** This is an improvement
   on the spec's letter, since nothing reached the chord with the germ in the right
   face. Mutants M2 and M7 confirm the arm carries the 11 rows.
3. **PinchApex now builds — AGREE** (tier 3′ ok both orders, in my probe too).
   **peg_in_socket — AGREE to close, credit misplaced** (NOTE-1).
4. **Dip row re-pointed to E — AGREE with the re-point**. The evidence for "as
   the zip did" is not in any row (NOTE-2).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only in part: I read
`chart_ring_side` and `insert::plan_null_pairs` end to end, and not the whole of
`chord_join.rs`.

- **S1 (Q4/Q7).** `sectors.rs:766-769` documents `PairRecord.sa/sb` as "the
  A-sector (start, end) bound codes", the sector `a` names. After `across_tangent`
  (`insert.rs:308-334`), `r.a` names sector k while `sa` still holds sector i's
  codes. `insert.rs:450` then pairs the moved `r.a` with the unmoved `raw[i].sa`.
  That is safe today only because OnEdge loci never move. An invariant held by
  convention, with a field doc that is now false for moved records. **likely**
- **S2 (Q7).** `insert.rs:322-325` tests `(On, _)` before `(_, On)`. A record
  with both bounds On silently takes the start side. I did not find it reached.
  **unsure**
- **S3 (Q1 class).** MINOR-1's blind spot is a class: the "M5 S1" prose tag
  marks the zip's era across tour and tests; `crates/topo/tests/crosslap_rest.rs:73`
  ("The M5 S1 lane is reached exclusively through the…") is a fifth hit. Sweep
  `M5 S1` as well as "REST zip".
  **likely**
- **S4 (Q2).** The comment justifying the seam shift (`chart_ring_side`, "a vertex
  at the seam's azimuth reads at `lo`, past which the ray's degenerate rows put
  it") is longer than the one line it defends, and no verdict depends on the
  line (M4). **likely**
- **S5 (Q5).** `ops.rs:100-106` was rewritten in this PR and re-asserts "What
  still refuses, typed: undeclared mates". Undeclared tangent-site unions build on
  both trees, and some ship bodies that fail tier 3′ (`UndeclaredContact`,
  `rest_zip_admission` and `join_pierce_runs_sweep`). That is pre-existing, but
  the sentence is the PR's. **unsure**
- **S6 (Q5).** README C7 now says the join makes union volume "exactly additive
  at full engagement". The join's volume differs from the zip's by up to 4.7e-16
  relative, so the two cannot both be exact. Which one the C7 claim describes is
  unmeasured. **unsure**
- **S7 (Q3).** Every new rule is rowed only at sweep level (MINOR-3). The
  `RingSide::Undecided` doc and `chart_ring_side`'s doc were each rewritten,
  but no unit row in `chord_join/` was added beside them. **sure**
- **S8 (Q7).** `carrier_pair::flush_pair_relation` moves with "NO in-tree
  consumer" stated in its own doc (`carrier_pair.rs:28-30`). It is a published
  door kept alive by a move. Disclosed and pre-existing. **sure**
- **S9 (Q6).** "unbisected" (ruling 3) was a disclosed deviation with no schedule.
  NOTE-1 settles it. **sure**

## Method, cost

- Private `CARGO_TARGET_DIR`s (head and a base worktree), all foreground.
- Scratch-only edits, all reverted; nothing was pushed to the PR branch.
- The probes were:
  - an `r2probe` module at `boolean_op_with`, identical in both trees, with a
    zip-built flag on main;
  - a `chart_ring_side` row module;
  - seven mutants.
- Tokens about 270k. Wall-clock about 2 h 50 m.

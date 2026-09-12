# S-TCOST log — test-suite cost

Narrative record; the plan is `docs/S-TCOST-PLAN.md`. Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-09-02)

Opened on Ev's direction (in-chat, 2026-09-02) by a fresh
orchestrator on a remote container. Charter and the three rulings
Ev gave on the orchestrator's questions are in the plan.

**Operational facts, recorded once:**

- **Branch prefix `tcost/`**; orchestrator branch `tcost/orchestrator`;
  the harness-designated session branch
  `claude/test-suite-opus-optimization-q8u962` carries the opening PR
  and is otherwise unused.
- **A/B ordinal band: S-TCOST = 1400–1499**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same PR. The band is
  used only by kernel-logic units; test-only units record no row.
- **Three censuses dispatched at opening** (Opus, read-only, reports
  under `~/tcost-work/`): CI red history, CI per-test timing history,
  local build profile. Their findings are summarised here when they
  land; the raw material stays lane-private.

**Decisions taken unilaterally:**

- The program name and prefix (`S-TCOST` / `tcost/`); the band
  1400–1499 as the next free band per the banding entry.
- The gate mechanism's shape (TCOST-1 spec): marker-at-the-suite,
  derived selection, fail open — argued from the same siting rule the
  nightly demotion marker and `scripts/nightly-only-selection.py` use
  (a central roster drifts; a marker at the test cannot).

## Seam: the red-history census landed (2026-09-02)

Read-only census of every hosted-CI failure since the repo opened
(report lane-private; the numbers below are the ones that decide
units). Coverage: 623 failed runs; 43 % of failed-job logs from the
first week are gone from GitHub, and `--no-fail-fast` only landed
2026-08-31, so before that a red job's failure list is a prefix of the
real one. The census is solid from 2026-07-31 on.

- **397 (test, run) reds over 162 distinct tests** — by (test, run):
  219 defect caught, 77 stale pin updated, 46 ε-band/lane sensitivity,
  40 red inherited from main (five tests × one red base tree), 11
  infra, 4 planted.
- **Only 25 % of failed runs failed on a nextest test at all**; the
  rest are clippy, rustfmt/rustdoc, discipline gates, k-lint, compile
  errors and the render lanes' `refs/pull/N/merge` race (~130 jobs,
  the single largest non-test failure cause — outside this program,
  filed as an issue).
- **Cost against return**: of the 145 tests that ever appeared in a
  `Slowest 20` table, 21 have ever gone red. Never-red expensive
  families: `cert5_r2_probes` (11 tests, ~208 cpu-s summed, the single
  most expensive thing in the suite), `m10_3_r1_probes_interval`,
  `cert5_offgrid_knot_rational`, `cert7_r1_probes`,
  `solid_contain::r1_probes`, `r2_cert6_probes`, `m8_3_rational_volume`,
  `offset_fit`, `cert5_r1_patch_probes`, `cert7_r2_probes`,
  `r2_probe_cert8`. Their `*_r1_*` siblings did go red on their own
  PRs. At file level 287 of 322 probe/review suite files (1 790 tests)
  have never produced a red.
- **Genuine wins to protect**: `verbs_pierce_r1/r2_probes` (a silent
  wrong volume), `review_r1_tier_gate_probes` (an in-band wedge that
  shipped), the de-vacuumed `spline::knots` guards, `props::quad`'s
  multiplicity ladder, `review_s2`'s collapsed-offset lever.
- **Class finding (durable home here)**: three probes dump fixtures to
  a path that does not exist on the runner (`Os { code: 2 }`) and cost
  a full red run each — `m4_pr6_eps_diff`, `review_m2_pr4::
  dump_for_cross_profile_diff`, `r2_m10_2_probes`. A content unit
  retires the dump or points it at a runner-valid path.

Reading for the units: "never red" alone retires nothing (a suite can
guard stable code), but a family whose cost is measured and whose
return is zero is where lever 4–6 work starts; the R1/R2 asymmetry
says the second reviewer's suite of a pair is the first candidate to
gate or fold.

## Seam: the timing census landed (2026-09-03)

Per-test cpu-s over 104 green hosted runs (298 shard legs; all six
lane × ε rows covered 23–27 times), parsed with `scripts/slowest-tests.py`'s
own reader, shares normalised inside each run. Coverage caveat: the
Actions run-listing API caps at 1000 results and this repo makes
~150 runs a day, so PR runs are reachable only from 2026-08-27 and
main pushes from 2026-08-09; the trend is anchored on the latter.

- **Snapshot 2026-09-02**: default lane 459 cpu-s over ~5 700 tests
  (~113 s wall per shard), interval 535 cpu-s (~138 s). Top 20 tests
  = 50 % / 61 %; top 40 = 64 % / 80 %. **89 % of tests cost under
  20 ms and are 8 % of the bill** — the tail is free; the head is the
  work.
- **One file is a quarter to a third of the suite**:
  `crates/geom-brep/tests/cert5_r2_probes.rs` (11 tests: 118 cpu-s =
  26 % default, 163 cpu-s = 30 % interval). `geom-brep` as a crate is
  ~50 % of both lanes. Files whose own headers call them review probes
  are 60 % of the cpu-s on ~32 % of the tests.
- **The suite doubled in the last week** (255 → 459 cpu-s on the
  default lane, 08-27 → 09-02) from two suites landing (`cert5_r2_probes`,
  `cert7_r1_probes`) and one row slowing.
- **Rows that cannot fail**: `cert7_r1_probes::hunt_for_a_genuine_refinement_stall`
  (29 s, 6.4 % of the default suite alone) and
  `cert7_r2_probes::r2_stall_hunt` (4.7 s) hold no assertion — every
  arm is an `eprintln!`; the file header says "local, not for merge".
  `cert5_r2_probes::many_offgrid_knots_cost` / `many_dyadic_knots_cost`
  (24 s + 23 s) are cost-measurement probes printing `R2 COST`.
- **The dominant mechanism** is an independent dense Gauss–Legendre +
  Cox–de Boor oracle rebuilt per test (cert5_r2's `drive()` evaluates
  it twice, at 16 and 32 cells per span, only to decide `converged`);
  second is a ladder inside one `#[test]` (`width_versus_gap_from_a_block_edge`
  = 7 `drive()` calls, 43–59 s; the topo torus probe = 5 shapes × ~262
  ray poses, 23 s). ε barely moves the suite total but swings single
  certification rows 4–7× (1e-6 is the cheap row).
- **Interval-only**: `editor-core/tests/m10_3_r1_probes_interval.rs`
  is 42 cpu-s (7.9 % of the interval suite); its head row runs the E6
  driver at a 4096 budget three times.

**Units cut (test-only track, Opus lanes, batched style review):**
TCOST-2 geom-brep cert5/cert6 probes; TCOST-3 geom-brep cert7 +
rational review probes + offset_fit + offb_r2; TCOST-4 topo
`solid_contain::r1_probes` and the boolean in-src rows; TCOST-5 the
sweep + step-import rational family; TCOST-6 editor-core's interval
probes and the three fixture-dump infra rows. TCOST-2/3/4 dispatched
first (one heavy cargo at a time on this box; 5 and 6 follow when
disk frees).

## Seam: the build profile landed (2026-09-03)

`cargo build --workspace --tests --timings` under CI's profile env on
this 4-core container (shares carry to the 2-vCPU runner, seconds do
not; the hosted archive step's median is 609 s over five code-tier
runs on 2026-09-02, `docs/perf-data/opt-level/`).

- **82 % of the workspace's own compile time is test targets** (the
  ~225 dependency crates ride `rust-cache`; workspace libs are 18 %).
  Within the test half: the `all` integration binaries 74 %, the
  `--lib` unit-test binaries 26 % (the `crates/*/src` `#[cfg(test)]`
  rows — 1 396 tests in ~44 400 lines — are a fifth of the bill a
  `tests/`-only programme would miss).
- **Four crates are 70 % of test-target compile time**: editor-core,
  topo, sweep, mesh. `editor-core::all` alone spans the last 55 % of
  the build's wall — an indivisible ~170 s unit on this box, the
  plausible reason the hosted step is ~2× this box's wall.
- **What predicts a test target's cost is the number of items it
  instantiates** (r = 0.94), not its line count, and the intercept is
  zero: no per-binary constant is left to harvest after #179/#387.
  70–89 % of every test binary's symbols are library code pulled in
  by the link. Halving topo's suites shrank its binary only 17.6 % but
  cut the target's compile TIME 32 % (ten repeats, quiet-box clusters
  42 s → 28 s): about a third of a test target is fixed link and
  dependency instantiation and two thirds scales with test content,
  so deleting test content pays roughly two thirds of proportional on
  the build.
- **The largest single finding: shared helpers are compiled once PER
  SUITE.** Every aggregated suite keeps its own `mod common;` /
  `mod fixture;` / `mod corpus;`, so the binaries compile ~2.2× the
  workspace's actual test source — 342 533 redundant lines, 197 762
  of them in editor-core (`corpus` ×37 at 3 502 lines, `fixture` ×108),
  where `fixture::desc` is codegen'd 168 times. The aggregator headers
  declined this cost ("the alternative is editing the suites"); it is
  now measured, and it is the one build lever that removes compile
  work without removing a test or an assertion.
- Levers restated for the build side: simpler objects (lever 6) is a
  build lever too (fewer distinct instantiations); deleting a test
  pays ≈ 129 ms of compile on this box, flat; merging rows sharing a
  fixture is build-neutral; comments and `#[ignore]` are not levers.

**Unit cut: TCOST-B1** — deduplicate the per-suite helper modules,
prototyped on editor-core (the tail-owning unit, 58 % of the
redundant lines) with the hosted archive-step time before/after as the
measurement of record, then the same pass over topo, mesh, sweep,
viewer and profile if the prototype pays. Dispatched after a content
lane frees this box (disk and the one-cargo rule).

## Seam: first content lanes back (2026-09-03)

- **TCOST-4** (PR 1608, green on the drawn point default/1e-6): the
  topo torus probe 23 cpu-s → under the hosted top-20 cutoff; its
  1 310-ray lattice was an edge-value table plus a deterministic
  lattice, re-formed as a 13-regime enumeration plus a properly seeded
  counterexample-search row (a TCOST-1 gate candidate, specific to
  `crates/topo/src/boolean/solid_contain.rs`); two assertion-free
  profile dumps retired with named owners. The three fixture-dump reds
  from the census were already fixed on main.
- **TCOST-2** (PR 1609, green on default/1e-12): the cert5/cert6
  family 189 → ~100 cpu-s hosted (~47 %): the two cost probes deleted,
  the gap ladder 7 → 3 drives with the duplicated rung's owner named,
  the oracle ladder halved with the silent `converged` gate turned into
  a labelled assertion, three cert6 rows merged. **The brief's premise
  was wrong for 9 of 11 rows**: the cost is the KERNEL, not the test's
  oracle — `nurbs_patch_face` costs 22–33 s per call (local) when it
  exhausts its round budget against 3–5 s when it certifies or refuses
  early, and **the rational lane costs ~90× the integral lane on the
  same face** (a 3×3 dome: unit weights 0.23 s, one weight at 1.25 →
  21.7 s). Both go to the A/B track as **TCOST-K1** (spec to follow).
  Open question left by the lane: the merged cert6 row hard-expects
  the unit dome to certify at the run's ε while its comment admits a
  tighter band may refuse; preserved as-is (green at all three ε
  locally), flagged for the style batch.
- **TCOST-3** (PR 1614, green at default/1e-6, lane asked): the cert7
  and offset family 88 → 38 cpu-s hosted (−56 %; the whole default
  suite at that point 458 → 402): six assertion-free rows deleted (the
  two stall hunts among them), two rows deleted with `offset_fit`'s
  far-from-origin row named as owner, the recentring ladder trimmed to
  one station per distinct trajectory, the skinned loft shrunk.
  `review_r1_rational_probes.rs` left unchanged on measurement: its
  oracle is 0.5–13 % of the rows; `nurbs_patch_face` is the rest
  (TCOST-K1). **Second kernel finding for the A/B track**:
  `offset_fit::fit_offset` is 99.9 % of the recentring row at 3.5–3.7 s
  per station against 0.004 s for its 437-point oracle, and 13.8 s for
  one call at an unreachable tolerance — **TCOST-K2 candidate**, spec
  after K1's Phase 1 says whether the same exhausted-budget shape is
  at work in the fit loop.

## Seam: batch style review 1 adjudicated (2026-09-03)

One Opus reviewer over PRs 1608 / 1609 / 1614 (no builds; every
cited run resolved to its head SHA and read). All three MERGEABLE
WITH FIXES; fix passes run as the implementers' own lanes.

- **1609 MAJOR-1**: the two retired cost rows had carried a real
  containment assertion on 12+12-knot patches and the named owner was
  a different fixture class and predicate — one heavily-knotted row
  comes back with the assertion and without the timing print (its
  cost is TCOST-K1's to cut; it is TCOST-1's first gate candidate).
  **MAJOR-2**: the merged cert6 row's baseline `expect` contradicts
  its readings' tolerated refusal — the baseline takes the readings'
  posture (loud skip, never red on an honest refusal).
- **1614 MINOR-1**: two of the "assertion-free" deletions asserted
  through `unwrap`; two liveness residues (a cylinder at tol 1e-4, an
  elliptic wall at d 0.1) get an owner. Eight deletions, not six.
- **1608**: a stale "not part of the shipped tree" header; an
  anti-vacuity floor on the re-formed enumeration (65 rays that all
  come back `Uncertain` would be green).
- **Class finding, durable home here**: `crates/geom-brep/tests/`
  carries six spellings of `quarter_cylinder`, four `drive`s and three
  dense-oracle constructions across the certification probe files,
  found only by grepping constants — the prose sweep for "verbatim /
  re-derived / kept in step" returns nothing. A test-helper dedup
  unit for that crate follows TCOST-B1's pattern (**TCOST-7**).

## Unit: TCOST-3 merged (2026-09-03, PR 1614 at 68960021)

geom-brep's cert7 probes and the offset fixtures. Eight deletions
(six assertion-free, two that carried liveness through `unwrap` — the
review's finding; both liveness claims re-owned as labelled arms in
`offset_fit` and a 0.03 cpu-s cert7 row), the recentring ladder
trimmed to one station per trajectory with its coverage argument
marked unguarded at the site, the skinned loft shrunk. Family
88 → 42 cpu-s hosted at default/1e-6 (−52 %); roughly 46–50 cpu-s
off a ~458 cpu-s suite. Lane asked for default; interval covered
locally only (no interval semantics in the diff).

## Unit: TCOST-4 merged (2026-09-03, PR 1608 at 06e4d74a)

The topo torus-oracle probe: 1 310 rays → a 13-regime enumeration
(65 rays) with an AGREEMENT assertion and a DECIDEDNESS floor whose
failure names the regime (the fix pass measured 40/41/38 decided at
the three ε rows and found membership moves with ε, so the floor is
a floor over the rows, not a per-regime pin), plus a seeded
counterexample-search row over the generic poses (ungated until
TCOST-1; specific to `crates/topo/src/boolean/solid_contain.rs`).
Two assertion-free cross-profile dumps retired with owners; a
dangling module-path citation in `solid_contain.rs` corrected. The
row left the hosted top-20 at both ε rows drawn (from 23 cpu-s, rank
2); topo's lib suite locally 77 → 10 s.

## Unit: TCOST-2 merged (2026-09-03, PR 1609 at e8922ec1)

geom-brep's cert5/cert6 probes. The gap ladder 7 → 3 drives (the
duplicated 1-ulp rung's owner named), the dense oracles lazy and
halved with the silent `converged` gate turned into labelled
assertions, three cert6 rows merged with the baseline now taking the
readings' posture (a 1e-4 row added so the bit reading is live at
every ε — the fix pass found main's own reading had been standing
down at 1e-6 silently). The two heavily-knotted containment rows
came back from the review's MAJOR without their timing prints, at
8.2 and 4.7 cpu-s against 24 and 22 on main; both are TCOST-1 gate
candidates (specific to `crates/geom-brep/src/props/quad.rs`) and
their residual cost is TCOST-K1's. Family ≥ 44 % down hosted
(interval/1e-6 drawn on the fix head; default/1e-12 on the first);
row count 24 → 22, nothing deleted.

## Tracker migration (2026-09-03)

The plan and this log moved from `docs/S-TCOST-PLAN.md` / `docs/S-TCOST-LOG.md`
to `work/tcost/plan.md` / `work/tcost/log.md`. The slate now lives in this
directory's item files and in `work/STATUS.md`; this log stays the
narrative. Items at migration, matched to the entries above: TCOST-1
(dispatched, PR 1612), TCOST-2 / TCOST-3 / TCOST-4 (closed, merged as
PRs 1609 / 1614 / 1608), TCOST-5 and TCOST-6 (open, cut at the timing
census), TCOST-7 (open, cut at style review 1), TCOST-B1 (dispatched,
PR 1616), TCOST-K1 (spec) and TCOST-K2 (open, candidate).

## Seam: TCOST-5 back (2026-09-03)

PR 1621, green at interval (asked) / 1e-6 (drawn). The sweep and
step-import rational family: one duplicated arc-loft row deleted
(the step-import native row owns every assertion and adds tiers
1/2), the cross-crate twin balloon rows merged into the import-door
row with every assertion labelled and a bare early return turned
into a named loud skip, two assertion-free digit dumps deleted with
owners, and the `r2_probe_cert8` sweep put on the fuzz harness (it
had a fixed seed, a private LCG and no replay line). Family −10 %
hosted at the cheap ε row, −21 % locally at 1e-12. The
four-quadrature import row measured irreducible from the test side
(the imported body is not the native body; `ImportOptions` cannot
skip the at-rest gate). Awaiting style batch 2.

**Third kernel finding for the A/B track — TCOST-K3 candidate:**
`topo::validate_geometric` recomputes the enclosure its caller just
computed and hands nothing back, so three rows in this family (and
the real import path) pay two rational certificates per body. An API
that lets the gate consume or return the mass properties removes one
certificate per body. Separate from K1 (a schedule exit) and K2 (the
fit loop); its own spec after K1 lands.

## Seam: usage-limit interruption (2026-09-03, ~03:00–04:22 UTC)

Evan's session usage limit bound at ~03:00 UTC; the four lanes then
running (TCOST-1, TCOST-K1, TCOST-B1, TCOST-7) were killed mid-turn
by the 429 and no new work was dispatched until the reset (Evan's
ask, in-chat). State at the kill, verified from the pushed branches:
TCOST-1's head 4cba1468 fully green on the interval lane (asked) with
the report unwritten; TCOST-B1 pushed through its five-crate widening;
TCOST-7 pushed its first consolidation commit; TCOST-K1 had two
uncommitted instrumentation files and no branch. All four resumed
from their own transcripts at 04:25 UTC with the cwd-reset rule in
the message; a lane that shows no progress by the next check-in is
re-spawned fresh from its pushed state rather than resumed again.

## Seam: TCOST-1 and TCOST-B1 back (2026-09-03)

- **TCOST-1** (PR 1612, green on both compile modes — interval asked
  on the final head, default drawn on the same code one commit
  earlier; evidence PR 1613 shows the skip in a run: 35 `gated: …
  skipped` notices, 2 663 tests run against 5 611 unfiltered).
  42 markers over 299 tests (5.3 % of the suite): every `fuzz::`/
  `effort()` caller plus two hand-rolled xorshift sweeps. Deviations
  disclosed: the marker is a no-op macro without `include_str!`
  (directories cannot be expressed that way; the discipline gate
  covers both), an unresolvable marker fails open for its own suite
  only, the nightly row builds `--features interval` once, and **the
  nightly row has never executed hosted** (the lane's token could not
  dispatch nightly.yml). The `proptest!` population (22 files, 15 of
  them `#[cfg(test)]` modules in production files) is the disclosed
  next batch. Under review.
- **TCOST-B1** (PR 1616, green; interval asked on the six-crate head):
  353 per-suite helper `mod` lines → one declaration per binary,
  329 772 redundant compiled lines gone (96 % of the class); local
  editor-core test-target compile −20 % over three alternating pairs,
  binaries −4.9 %. The lint policy those helper trees inherited from
  whichever suite loaded them is now stated per tree (159 clippy
  errors surfaced and were resolved by naming, not widening). A
  finding for the program: **the hosted archive-step duration is a
  function of the change filter's tier and package set** — at one
  identical configuration it ranges ±25 %, wider than any single
  unit's effect — so build-side units quote the tier with the number
  or compare post-merge distributions; the 609 s figure in the plan's
  brief was a median over mixed tiers. Under style batch 2 with
  TCOST-5.

## Seam: batch style review 2 and the TCOST-1 review adjudicated (2026-09-03)

- **TCOST-1 (PR 1612)**: MERGEABLE WITH FIXES — the selection right in
  both directions (every arm planted with the full filterset string
  asserted), the composition one `-E` on every leg, 35 + 7 = 42
  notices in the demo run, the discipline gate wired in both halves.
  One MAJOR in marker CONTENT: `mesh8r2_probes`' set named only
  `crates/mesh/src/` while its subject is `topo::examine_chart_coherence`.
  Fix pass: widen that set and `r1_p2_onb_probes`', re-audit every
  set against "does it name the module the CLAIM rests on", move the
  two production files' fuzz rows into their own `#[cfg(test)]` files
  so 44 deterministic pins stop being gated with them, tighten the
  reader census's marker recogniser to the python's shape with a
  unit test, state the nightly row's cost shape in-file.
- **TCOST-B1 (PR 1616)**: MERGEABLE — every claim reproduced
  independently, the lint delta strictly narrower than before, the
  matched hosted pair better matched than claimed (same cache key,
  members evicted on both sides). Three cheap fixes (a counted token
  in six header comments; one home for the allow-block prose; the
  two inert `duplicate_mod` allows dropped, the two live ones kept).
  **TCOST-B2 cut**: the same pass over step-export, step-import, stl,
  geom and geom-core (11 933 redundant lines, ~3 % of the class).
- **TCOST-5 (PR 1621)**: MERGEABLE WITH FIXES — one assertion
  narrowed (the ORACLE pad check moved inside the certified path, so
  it no longer runs at 1e-12), contradicting "nothing was weakened";
  the loud skip not in the tree's named-`#[test]` idiom; a copied
  const doc calling a 60° arc 150°; the deleted suite's measured
  negative findings dropped; the fuzz row still the floor-plus-search
  trap the memory names — split per the memory's remedy.

## Seam: container restart (2026-09-03, ~04:50 UTC)

The session's container restarted with five lanes live (TCOST-1 and
TCOST-5 mid fix-pass, TCOST-B1 with a fresh clippy red on its fix
head, TCOST-7 with PR 1635 open, TCOST-K1 with its first kernel commit
pushed and no PR). The disk survived — worktrees, uncommitted fix-pass
edits (15 files in TCOST-1's lane, 3 in TCOST-5's), warm target dirs —
and only the agents' transcripts were lost, so each lane was
re-created FRESH from its on-disk and pushed state with the fix list
restated in the brief (the death-recovery rule: fresh over resume when
the remainder is specifiable from what is pushed).

## Seam: TCOST-7 back and reviewed (2026-09-03)

PR 1635 (green at default/1e-6 drawn; no interval suite touched):
`crates/geom-brep/tests/shared/` holds one home each for the quarter
cylinder, the sphere band, the knot vectors, the dense grid and the
offset-residual loop, and the `Patch` oracle; bit-identity of every
caller's fixture was checked at the merge base by the finisher and
again by the reviewer. Review verdict MERGEABLE WITH FIXES: six
call sites of the very loop the unit consolidated were left
un-migrated in two files the PR edited, `review_r1_rational_probes`
keeps a `dense` and a `dbasis` character-identical to the shared
ones, a minted "cited by" entry cites nothing, and `face_posture`
carries a parameter both callers pass identically. Fix pass in the
lane. **TCOST-8 cut**: the helper families the unit deferred —
`band` (47 spellings), `edge`/`great` (16), `p3`/`v3`/`pt` (38) —
after TCOST-7 merges.

Operational: a reviewer ran `git checkout` inside the orchestrator's
own checkout (`/home/user/cad`), moving it onto the reviewed branch;
restored from the pushed `tcost/orchestrator`. Every brief template
now carries the rule (read other branches with `git show`, or your
own worktree; never touch the orchestrator's checkout).

## Unit: TCOST-5 merged (2026-09-03, PR 1621 at b2babf84)

The sweep + step-import rational family: the duplicated arc-loft row
deleted (the step-import native row owns every assertion and adds
tiers 1/2), the cross-crate twin balloon rows merged into the
import-door row with labelled assertions (the ORACLE pad check
restored to every ε after review), the deleted suite's measured
negative findings carried into the merged header, the arc-loft
posture printed by variant, `r2_probe_cert8` split into a
written-chart witness row and a floorless varying-seed search on the
fuzz harness. The in-row stand-down uses `test_utils::vacuity::stood_down`
(the tree's door for a run-time ε condition; the named-`#[test]` idiom
is the whole-binary shape). Family −10 % hosted at the cheap ε row,
−21 % locally at 1e-12; the four-quadrature import row measured
irreducible from the test side. Interval asked on the final head.

## Seam: the one-declaration pattern becomes an invariant (2026-09-03, Evan's ask)

Evan (in-chat): can the existing aggregation guard enforce TCOST-B1's
pattern? Yes — folded into B1's fix pass: each converted crate's
`every_suite_file_is_aggregated` gains a second assertion over the
same walk, **a suite file declares no modules** (every `mod` item in
a suite's code view is a violation; helper trees are directories with
a `mod.rs`, which the walk already excludes), so a shared helper has
exactly one home in `tests/all.rs`. Per crate, so B1's six crates
carry it now and TCOST-B2 carries it to the rest as it converts them.

## Seam: CI-posture units cut (2026-09-03, Evan's ask)

Evan asked which jobs, or expensive parts of them, can go to the
nightly, and whether caching can avoid compiling the kernel so many
times. Assessment (per the ratified rule — a persistence detector may
be demoted, an absence detector may not — and the audit's billing
model): a code-tier run compiles the kernel ~9–10 times in ~8
distinct profile/feature unifications that share no artifacts, so
only fewer jobs collapse them; the cross-RUN lever is content-keyed
sccache (F4), wired since #852 and currently off with its reading
still owed. Evan approved the plan and kept the render lanes.

Units, CI-infrastructure track (Opus lanes, style review, no A/B row,
one PR each with its own hosted measurement and the billed-minute
entry in `docs/CI-MINUTES-2026-08.md`):

- **TCOST-C1** — `corrupt input (release profile)` → the nightly
  (persists; the only lane running the two `cfg(not(debug_assertions))`
  rows keeps running daily).
- **TCOST-C2** — the rustdoc gate's six excluded roots and its third
  pass → the nightly; the workspace pass stays on PRs, scoped to the
  closure the build already uses.
- **TCOST-C3** — the python suite seed-keyed like the viewer toolkit
  (PRs only when `pncad`, `pncad-py` or `editor-core` seeds move;
  ungated nightly).
- **TCOST-C4** — the sccache trial re-read: on for a window, per-crate
  hit stats read on warm runs (workspace crates and test binaries are
  the hypothesis; dependency hits prove nothing), verdict written under
  F4 either way.

## Unit: TCOST-1 merged (2026-09-03, PR 1612 at e293af83)

The per-file test gate is live: an in-file `gated_to!` marker names
the source paths a suite is specific to; `scripts/ci-filter.py`
derives one nextest filterset from the markers and the diff and both
CI halves consume it on every run leg; a discipline gate reds on a
path that does not resolve; every skip is a notice in the run's log;
the nightly re-takes the whole gated set ungated (its first execution
is the first nightly after this merge — neither the lane's nor the
orchestrator's token can dispatch nightly.yml). 42 markers over 257
tests (4.6 % of the suite — every `fuzz::`/`effort()` caller plus two
hand-rolled sweeps; the review's re-audit widened 16 path sets to the
module each claim rests on, and the two production-file markers moved
into their own `#[cfg(test)]` files so 44 deterministic pins stay
ungated). Merged with one red annotated as inherited from main —
M10-4's bore-pin row is red at interval/1e-6 on any tree, issue 1646
filed for the M10 lane. Next batch: the `proptest!` population (22
files), and the two content units' gate candidates (TCOST-2's
heavily-knotted rows, TCOST-4's torus sweep).

## Unit: TCOST-7 merged (2026-09-03, PR 1635 at 6d7f577a)

`crates/geom-brep/tests/shared/`: one home each for the quarter
cylinder, the sphere band, the knot vectors, the dense grid, the
offset-residual loop (nine call sites, three `None` policies, one
routine) and the `Patch` oracle, with `dense_over`/`dbasis_over`
parameterised on each oracle's own `basis` so the three deliberately
independent seedings stay three (the price — a shared-recurrence typo
now mirrors in all three — stated in the PR). Bit-identical fixtures
at every caller, checked by the finisher and the reviewer; 509 rows
byte-identical in listing; green on both lanes (interval asked on
an empty trailer commit because one touched suite is
`cfg(feature = "interval")`). Compile-time delta inside this box's
noise. TCOST-8 holds the deferred families.

## Tracker adopted (2026-09-03)

Ev (in-chat): GitHub issues are retired; `work/` is the tracker. The
two issues this program filed (1607 render-lane checkout; 1646 the
M10-4 ε-band row) are now `work/issues/` files carrying their numbers,
and every open unit has an item: TCOST-6 (blocked on B1), B2, 8, C1–C4
(dispatched), K3 (candidate); TCOST-1/5/7 closed at their merges.

## Unit: TCOST-B1 merged (2026-09-03)

PR 1616 at `913b1f04`, run 33722014086 (interval lane, ε default,
all green). Each shared test-helper tree is declared ONCE in its
crate's `tests/all.rs` for editor-core, topo, mesh, sweep, viewer and
profile (324 `mod` → `use crate::…`, 29 `mod` lines deleted; 329 772
redundant compiled lines gone, ~96 % of the class the build profile
measured). `every_suite_file_is_aggregated` in those six crates gained
the one-declaration assertion (a suite file declares no `mod <name>;`),
read through the new `test_utils::source::file_module_decls`; it fires
on exactly the five unconverted crates, which TCOST-B2 takes.
`clippy::duplicate_mod` allows dropped from eight `all.rs` (kept in the
five). Measurement: local alternating A/B −20.5 % on editor-core's test
target (66.5 → 52.9 s median), binaries −4.9 % total; hosted archive
step ±25 % at fixed tier, so the effect of record is the post-merge
distribution (a reading for a later census, not one run). Inherited
red annotated on the PR: the M10-4 bore-pin row at ε 1e-6
(`work/issues/m10-4-bore-pin-row-red-at-interval-1e-6.md`). TCOST-6
and TCOST-B2 were dispatched on B1's head before the merge (Ev's call)
and share its target dir; they retarget onto main from here.

## Seam: B1's merge reddened main's default lane; hotfix (2026-09-03)

TCOST-B1's merge turned `editor-core/tests/m10_p_lift.rs`'s `mod
fixture;` into a module-scope `use crate::fixture;` whose only use is
inside an `#[cfg(feature = "interval")]` row, so the default-feature
build carries an unused import and `-D warnings` reds every default-
lane draw on main (first seen on PR 1647's run 33723972788). B1's own
gating run drew the interval lane, which compiles the row — the
sampled point missed it, exactly the shape the sampling accepts. The
TCOST-C2 lane found it on its merge run and carried the fix; the
orchestrator cherry-picked it as PR 1658 (one file, 4 lines) on B1's
behalf per the inherited-red rule (the causing lane owes the fix).
Lesson for build units that touch per-feature suites: a `use` that
replaces a `mod` must sit where the `mod`'s uses are, and the lane
should ask for BOTH lanes on its final head, not one.

## Seam: second usage-limit interruption (2026-09-03, ~07:55–09:20 UTC)

Five lanes (TCOST-6, TCOST-B2, TCOST-K1, C1..C3, C4) died on a 429 at
about 07:55; nothing was lost — every lane's worktree was clean and
its last commit pushed (TCOST-6's two local commits excepted, still on
disk). Resumed from their transcripts at 09:22 with the cwd-reset rule
and the state each needed (the m10_p_lift hotfix on main; K1's
incremental cache dropped). TCOST-8 reported before the cut (PR 1659).

## Seam: CI-posture batch review adjudicated (2026-09-03)

C1 (PR 1650), C2 (PR 1654), C3 (PR 1655): all MERGEABLE WITH FIXES;
C4 (PR 1648) not yet delivered, joins the same reviewer by message.
Verified by the reviewer against the jobs API: the corrupt-input job
is absent from C1's head run; C2's warm-vs-warm reading (179 s / 110 s
gate step / 69 s non-gate) is the batch's strongest measurement; C3's
seed axis has the planted failure a closure-keyed implementation
would fail. Fixes owed: C2's widened `OUTLIER_GATES` matcher admits a
bare `--print-roots` invocation as a real run (reproduced; the tree
is saved only by the `$(` spelling of the roots step) — drop it in
the same pass as `--selftest` and plant the failure; the scoping of
pass 1 is a second demotion that gets its own sentence; `--scope ""`
silently means `--workspace`; the nightly side of C2's ledger is
unpriced. C3's re-take can report green having run nothing
(`unittest discover` on an empty match exits 0) — a nonzero-count
guard, as a class across the three call sites. C1: two prose sites
the sweep missed, an honest rationale for the dropped output. All
three owe a row in the nightly budget table (~8 → roughly double).
Landing order C2 → C1 → C3 (C2 is the only one not touching
`ci-filter.py`; C1's delta there is the smaller), main merged and one
green run each before landing. Two candidate `work/issues/` items
filed by the lane: D107's prose now points at the wrong workflow;
the pin-reading `sed | head -1` idiom's fourth copy in nightly.yml.

## Seam: style batch 3 adjudicated (2026-09-03)

TCOST-8 (PR 1659), TCOST-6 (PR 1666), TCOST-B2 (PR 1669, found open
by the reviewer before its lane reported): all MERGEABLE WITH FIXES.
Every cited chain verified character-exact (the `Band::linear` chain,
`Real::from_f64` on f64 and `Probe`, the `sin(±0)` merge, the
turbofish sites). Fixes owed — TCOST-8: a `Band::linear` spelled
inline inside the shared tree it made a home for; reasons AT the copy
for `control`/`tmer`/`wall_domain`/`width` (the reviewer's ruling: the
R1/R2 pair and the carrier-id stamp are the reason, "file-local
tokens" is a preference); the ~29 inline `Band::linear(Tol::witness())`
sites taken since the home now exists; a default-lane run before
merge (64 of 71 suites compile there — the B1 lesson). TCOST-6: the
new frontier-width claim lacks its column and its `>= 64` floor its
argument; `LEVEL_CEILING = 128`'s justification is false for f64 (the
bound is `DEFAULT_MAX_DEPTH = 24`); the single-axis assumption of
`widest_frontier` made a stated precondition; no default-lane run
needed (whole-file `cfg(interval)`, textual filterset term — the first
gated marker inside such a suite). TCOST-B2: the hosted evidence sits
two commits behind the head, whose own default-lane run was still
running; a false "no path-relative read" sentence (step-import's
`common/mod.rs` reads `CARGO_MANIFEST_DIR`; conclusion holds).
Candidates filed by the lanes: the rename-only blind spot of the
body-hash census; the one-declaration assertion's nine verbatim copies
wanting one home in `test_utils`, with `pncad/tests/all.rs` outside it.

## Seam: TCOST-K1 review dual dispatched (2026-09-03, ~10:20 UTC)

PR 1652 green on its final head `554f2b0f` (interval 1e-12 asked;
the default-lane head's own clippy red was main's m10_p_lift, since
fixed). Ordinal 1400 claimed on main (PR 1672) with the v6 draw and
the brief hashes; both reviewers dispatched concurrently on the
frozen head, briefs identical modulo lane name, neither sees the
other's report. The unit's own log entry waits for the dual to
conclude. Hosted reading of record for the program: default-lane
shard totals 219.7 + 300.1 → 135.0 + 127.6 cpu-s at 1e-12; the
digest instrument found every certified bound byte-identical across
six lane×ε rows.

## Unit: TCOST-6 merged (2026-09-03)

PR 1666 at `5666f015` (run 33743321953, interval 1e-12 asked, green).
editor-core's three chamber heads (39.3 cpu-s on the interval lane)
are one labelled row at 6.7 cpu-s hosted: every assertion kept, two
strengthened (receipt identity on the D9 drive; the widest frontier
the schedule had to spread, ≥ 64, with the single-axis precondition
asserted). The budget constant 4096 → `CHAMBER_LEAVES = 1280` on a
measured, ε-invariant threshold at 1024 (sweep table in the doc);
the two partition rows keep `FULL_PARTITION_LEAVES = 4096` with the
measured cost of not cutting them; the ulp-wide box row gets
`GRID_FLOOR_LEAVES`. The suite is gated (`gated_to!`) to the driver,
analysis, distribution, resolve and tolerance modules its claims
rest on — the first marker inside a whole-file `cfg(interval)` suite
(no default-lane run needed: the filterset term is derived from the
path). Findings: no slow-kernel candidate (cost linear in the leaf
budget); the four other interval-only census rows are corpus-shaped
and want a corpus unit, not this mechanism.

## Unit: TCOST-B2 merged (2026-09-03)

PR 1669 at `2b4bf85e` (run 33742913933, default lane, all green; the
parent head's interval run red only on main's M10-5 row, filed as
`work/issues/m10-5-e2e-channel-slider-reds-at-eps-1e-6.md`). The one-
declaration pass over step-export, step-import, stl, geom and
geom-core (36 `mod` → `use`, 11 948 redundant compiled lines); the
guard assertion now in all fourteen guarded crates and demonstrated
to fire on the merge-base shape; `clippy::duplicate_mod` allows gone
from every `all.rs`; geom's helper inside the `curves/` group directory
reached through an inline `mod curves { pub mod n1r2_fixtures; }` so
the `#[path]` census stays clean. Honest size reading: at ~3 % of the
class the guard's own ~100 KB per crate outweighs the dedup in three
of the five (net −1.4 MB across the five after the control), dev
profile, not comparable with B1's table. main regrew the class within
a day (five lib_tube suites, 21 965 lines) and B1's guard caught it —
the argument for the guard. Filed: the fourteen-copy assertion wants
one home in `test_utils`, and `pncad/tests/all.rs` sits outside it.

## Unit: TCOST-C2 merged (2026-09-03)

PR 1654 at `b6a85c32` (run 33742313504, default 1e-12 drawn, green;
interval covered by the pre-fix run 33730257957 — every fix sits in a
lane-independent seat). The rustdoc gate's excluded roots and pass 3
leave the PR gate for the nightly; pass 1 is scoped to the dependent
closure (a second demotion, argued per row: a member whose sources
changed is its own seed; a rename/delete break lands in a dependent
by construction; prose-only breaks in unrelated members are the
nightly's). `fmt` job 222 s / 4 billed → 179 s / 3, warm against warm
(run 33727294346); the nightly side priced at ~3 billed a night,
DERIVED, to be re-read from the first nightly run. The review's hole
closed: `gate-roster.sh` no longer counts a bare `--print-roots`
invocation as a run of the gate (planted failure added); `--scope ""`
is refused rather than silently widening to `--workspace`. Landing
order C2 → C1 → C3; C1 re-merges main now.

## Seam: C4 reviewed (2026-09-03)

PR 1648 MERGEABLE WITH FIXES: the verdict (sccache refuses
`--crate-type bin`, so its ceiling is the libs' 18 %) confirmed
from the stats themselves; the control's miss verified from its
job (a 275 MB save on the post step, against `Cache up-to-date.` on
the warm run). Owed: F4's 82 % figure cited a lane-private path —
the seconds go inline or the table into `docs/perf-data/`; one
number that does not reconcile (90 − 47 = 43, not "62-odd"); the
miss rows lack the rust-cache key they missed; the "compiles from
scratch on most runs" claim narrowed to what one branch's runs
show (a branch's first build job restores nothing; main never runs
the job under F3). C4 lands last of the four. TCOST-B3's body
narrowed the same way.

## Seam: TCOST-K1 dual delivered — correspondence pre-note (2026-09-03)

Both reviews on frozen `554f2b0f`, reports stored under
`~/tcost-work/ab/`. Verdicts DIVERGE in letter, converge in
substance: R1 MERGEABLE-AFTER-FIXES 2/8/3, R2 MERGEABLE 0/4/2.
Neither could construct a certifying face the exit refuses; both
re-derived the bound and measured it per round; both verified every
hosted number against the cost reports. BILATERAL: the exit's
decision sits outside the named predicate / k-funnel (R1 MAJ A1 ≡
R2 MINOR 4 — severity-divergent: R1 reads a shrunken ledger
population and a stale justification sentence, R2 telemetry only);
the suite cannot red when the exit stops firing on the integral
lane and `refused_early…` cannot detect absence (R1 MAJ A2 + B1 ≡
R2's mutant table and Q3, which R2 scored in the rubric rather than
as a finding); the monotonicity across rounds unstated (B4 ≡ M1);
the pad term dead under the test surface (B8 ≡ M2); the NaN
sentence (B5 ≡ NOTE); second spellings of the last piece count and
block assignment (B3 ≡ style); no default-lane run on the final
head, both judging it adequate (B7 ≡ claim 4). UNILATERAL R1: the
threshold-to-zero mutant (escalation pre-emption unguarded), the
stale "measured width" prose class, the undisclosed 1.1461e-6 pin,
the TARGET_LEN_FACTOR/K mirrors, the half-vacuous certify row.
UNILATERAL R2: the pinned width's `#[ignore]` retake is registered
nowhere (R1 has it inside C2), the QUAD2_RATIONAL_MAX_ROUNDS doc,
DESIGN.md:603. DISPATCHER FAULT (R1 C3): the shared target dir let
one review lane execute the other's mutated binary — cargo's lock
serialises writes, it does not isolate artifacts; R1 re-took every
result privately, R2's mutant matrix is flagged as possibly
contaminated (its outcomes agree with R1's on every shared mutant).
Union fix pass dispatched to the implementing lane, same arm; the
row records at merge.

## Unit: TCOST-C1 merged (2026-09-03)

PR 1650 at `f593881e` (run 33745119482, default 1e-6 drawn, green;
interval covered by the pre-fix run 33722960963). `corrupt input
(release profile)` leaves the PR gate: −2 billed minutes on every
code-tier run whose closure holds topo (89 of the last 128 merges),
re-taken nightly with its absence detectors (count guard + name
greps) moved with it. The nightly job-insertion conflict with C2 was
resolved by rebuilding nightly.yml from main and re-inserting C1's
job (both parse; both jobs present; no markers). The nightly budget
total is edited once, by C3 as the last to land.

## Unit: TCOST-8 merged (2026-09-03)

PR 1659 at `d13b23a9` (default 1e-12 asked, run 33746172065, green
on the post-merge head; interval 1e-6 asked, run 33728831063, on the
pre-fix head, the three interval-only edits re-verified locally
against a byte-identical 553-name listing). Fourteen helper families
of `crates/geom-brep/tests/` into `tests/shared/` — 43 `fn` bodies
plus, after the review, 27 inline `Band::linear(Tol::witness())`
sites, 70 sites for one home; the nextest listing `cmp`-clean
against the merge base on both lanes throughout. Nineteen "kept
apart" copies each carry their reason at the copy, the R1/R2 pair
and the carrier-id stamp among them, and `shared/mod.rs` enumerates
them with a grep-checkable count. Compile-time deliberately not
measured (TCOST-7's precedent: noise). Filed: the inline canonical-
frame surfaces (34 sites at merge-base numbers) and the body-hash
census's rename-only blind spot (a sweep obligation across geom-brep
and the B1/B2 crates). Operational: the session scratchpad is shared
across sibling lanes — one lane's PR draft was overwritten by
another's; lane notes belong under `~/tcost-work/<lane>-notes/`.

## Unit: TCOST-C3 merged (2026-09-03)

PR 1655 at `82cdff58` (run 33746592219, interval 1e-6 drawn, green;
default covered by the fix-head run 33742871112). The python suite is
seed-keyed on {pncad-py, pncad, editor-core}: −2 billed minutes on
every code-tier run whose seeds miss, with the verdict step sited in
the `filter` job because a skipped job runs no step. The review's
hole closed as a class: `unittest discover` over an empty match exits
0, so a nonzero-count guard now stands at all three call sites
(ci.yml's job, the nightly re-take, `run-python-tests.sh` — no single
seam exists for them, and that is filed). As the last of the three
to land, C3 set the nightly budget total once (~8 → ~15; calibration
night ~34 → ~41), dropped the three per-row "not yet included" notes,
and corrected the paragraph under the table that had gone false.
Ledger after the three: PR side −1 (C2, measured) −2 (C1, per
topo-closure run) −2 (C3, per seed-miss run); nightly side +~7, of
which C2's ~3 is derived and wants a re-read from the first nightly
that executes it. C4 lands last after its final main merge.

## Unit: TCOST-C4 merged (2026-09-03)

PR 1648 at `eb7a78a2` (run 33748099185, interval 1e-6 asked, all
green; the default lane on earlier heads of the same tier). The
sccache trial re-run under its own rules — seven runs, two lanes,
tier=all, 18 packages, lane asked by trailer — and its verdict
written into CI-MINUTES F4: sccache 0.16.0 refuses `--crate-type
bin`, so the 47 refusals on a warm run are the workspace's test
binaries and build scripts, 82 % of compile time by the build
profile's census (now inline in F4 with provenance: test targets
726.3 s, libs 159.5 s, 297 units); dependency hits are zero on any
warm run because a restored rust-cache means rustc never runs for
deps; the object cache barely persists. Rig kept, inverted to
`vars.SCCACHE == '1'` (inert with nothing set). The finding that
outgrew the unit — a branch's first build job restores nothing
(branch-scoped caches + F3's "main never runs the job"), the control
verified by its 275 MB save — is TCOST-B3, dispatched. The four
CI-posture units are landed: PR side −5 billed minutes on a typical
code-tier run, nightly side +~7.

## Seam: hosted CI down account-wide — the Actions spending limit (2026-09-03, from 11:52 UTC)

Every `ci.yml` run created after 11:52 UTC fails within seconds with
no job started, on main and every branch ("The job was not started
because an Actions budget is preventing further use"); the last run
to get a runner was 33751935948 at 11:51. Verified against the run
list, not a lane's clock (an earlier "stall" report from the TCOST-9
lane was a clock error and is retracted). Routed to Ev as an `[ev]`
PR carrying `work/issues/actions-budget-denies-job-starts.md`
(`needs_ev: true`; the B3 lane filed it). Lanes told: no empty
commits to re-gate until CI is back. State at the cut: TCOST-9's
head green on both test legs (only the runner-less archive-cleanup
job red); TCOST-B3's default run green on every job that started,
test legs never started; K1's fix pass had a green default-lane run
at 11:45. TCOST-B3 is in review (PR 1684) — its finding is the
program's biggest CI-minutes lever so far: ~45 billed minutes an
hour going to dependency rebuilds on a cache no branch could read,
against ~7 for a main-scoped primer.

## Seam: style batch 4 adjudicated (2026-09-03)

TCOST-9 (PR 1681) MERGEABLE WITH FIXES — everything load-bearing
reproduced (the 54 terms, both proof counts by hand from the filter
logs, byte-identical moved rows, no `#[path]` mount on the two new
files, the census re-derived). One MAJOR on the batch-2 bar, one
level over: a marker's own file is implicit but a sibling test-helper
module is not, and three sets omit the helper their fixtures and
tolerance come from (`profile/tests/common/` for two, `editor-core/
tests/fixture/` for one) — a PR editing that helper would skip the
suites that depend on it; the class swept across all 54 markers, and
a mechanical guard (marker set vs the suite's own helper imports)
filed as a candidate. `curves/boxes.rs` omits `linalg/` where its bvh
siblings name it. Minors: an orphaned doc paragraph from the split, a
stale fuzz label, a title contradicting its body, and §0's wrong
diagnosis of the red housekeeping job (it is the budget denial).
TCOST-B3 (PR 1684) MERGEABLE WITH FIXES — the mechanism proved from
the tree, the YAML coupling verified character-identical, the parity
selftest's seven plants confirmed; two arithmetic lines (a rate whose
denominator does not close; commits vs pushes), `runs-on` added to
the parity check, the never-executed primer and the cancel-in-
progress interaction stated. Landing ruling: on a re-gate of the head
when runners return, not before. Both fix passes dispatched locally.
## Unit: TCOST-K1 fix pass landed (2026-09-03)

The union of both reviews of PR 1652 (frozen 554f2b0f) applied on the
branch: the budget exit decides through the k-funnel under its own
predicate (`props_quad_last_round`), asked once after round 0, on a
definite negative only; `QuadratureBudget` carries the rounds it paid
for, and the suite's rows are gated on that receipt on both lanes
(four kernel mutants — exit never fires, exit refuses in-band, pad
dropped, bound × 4 — each red on the rows that own them); one home
for block assignment, cut count, hull selection and the remainder's
cell factor; the bound's monotonicity written at the site; the stale
"measured width" prose swept, the Display and its five matchers with
it; the area pad filed as `tcost-area-pad-lever`. Refusal payloads
bit-identical to the frozen head on the digest's probe subset.

## Unit: TCOST-K1 merged (2026-09-03) — the program's first A/B row

PR 1652 at `b28b734f` (interval 1e-12 with the dev-probe k-lint row
asked, run 33766712819, every job green; the fix pass's default-lane
run 33750172871 green at 1c78f3c9). Merged at 5e668ba6, recorded as
ordinal 1400 / sample #117 in `docs/MODEL-AB-LOG.md`; block TCOST-KB1
slot 0 concluded (record branch-side). The NURBS-patch lanes refuse at
the budget without running the schedule: a lower bound on the last
round's flux width, decided through the k-funnel under
`props_quad_last_round` once after round 0; `QuadratureBudget` carries
the rounds it paid. Hosted at 1e-12: default-lane shard totals
219.7 + 300.1 → 135.0 + 127.6 cpu-s; the block-edge gap row 27 → 2.9.
Every certified bound byte-identical across six lane×ε rows; no pin
re-baselined. `docs/TCOST-K1-SPEC.md` deleted at merge per the
DOC-LEDGER rule (the PR body and this entry are its record).

## Unit: TCOST-B3 merged (2026-09-03)

PR 1684 at `cf9392f4` (default 1e-6, run 33766730064, all green
including the new `cache-prime` parity step). The build jobs'
rust-cache lived in a scope no branch could read (main never saved
one; 40 of 49 first build jobs missed; ≈45 billed minutes an hour on
dependency rebuilds). Two primer jobs on `push: main` save under the
shared keys the build jobs now restore, at ~7 billed minutes an hour;
`scripts/check-cache-prime-parity.py` reds if the primer and build
jobs' env, key or runner drift. OWED: the after-reading — the first
PR run after the next push to main, `build + archive`'s restore line
and archive duration against 820/840 s cold — and the first push to
main pays one full dependency build per lane.

## Unit: TCOST-9 merged (2026-09-03)

PR 1681 at `b09a0b33` (default 1e-6, run 33766720238, all green).
The second gating batch: the proptest population (8 whole suites, 2
splits so pins stay ungated, cert5_r2_probes whole) plus TCOST-2's and
TCOST-4's gate candidates — gated set 43 → 54 suites, 257 → 384 tests;
both hosted proofs (54 skip notices on an unrelated diff; the named
suite running on a touched path). The batch-4 MAJOR closed as a class:
ten markers (three of this unit's, seven of TCOST-1's) now name the
sibling test-helper module their fixtures come from. Left ungated with
a filed question for Ev: 14 in-src proptest modules at 0.62 cpu-s
total. Filed: the `#[path]`-mount blind spot in `ci-filter.py`'s
src-module derivation; the marker-set-vs-imports guard. Evidence PRs
1679/1680 closed unmerged.

## Seam: the outage ended (2026-09-03, 14:24 UTC)

Ev raised the spending limit; the three re-gate runs got runners
within seconds and every one went green. Issue closed.

## Seam: API overload after the K1 merge (2026-09-03, 14:47–15:22 UTC)

The K2 and K3 implementer lanes (block TCOST-KB1 slots 1 and 2) each
died three times on server-side 529 overloads before doing any work;
resumed after a 10-minute backoff at 15:22. They stay on the block's
drawn arm. The census re-run, which is read-only aggregation of run
logs and no unit's work, was moved to a different model to get the
readout; its report lands under `~/tcost-work/timing-history/`.
Interim reading from it: the suite's cost distribution is much
flatter (the old 42.6 cpu-s head row is gone; no shard's top row now
exceeds ~11 % of its shard); K1's interval total at 1e-12 is ~355 cpu-s
against the 535 baseline (tier differs); the build critical path is
unchanged inside its own ±25 % noise; most of today's runs were
tier=all because they touched the workflow, which makes the per-file
gate a no-op in that sample; the nightly has not yet run on a tree
carrying C1–C3, and its 09:54 run's gated-suite re-take row failed at
a pin-read step (being read); B3's first primer run on main was
cancelled mid-save by the next push, as its own entry predicted.

## Seam: the program's before/after readout (2026-09-03, post-K1)

Re-taken from hosted runs only (`~/tcost-work/timing-history/
REPORT-after.md`, every number labelled with its run id). What is
clean: K1's own controlled pair at 1e-12 on the default lane,
219.7 + 300.1 → 135.0 + 127.6 cpu-s (−48 %); the head of the
distribution is flat — the 2026-09-02 top row (42.6 cpu-s) is gone,
the largest row on a comparable run is 9.8 cpu-s, and no row exceeds
~5 % of a shard. What is indicative: tier=all runs at ε 1e-6 read
333–337 cpu-s per default-lane run against the 459 baseline (~−27 %,
but 1e-6 was already the cheap row); the cleanest like-for-like at
1e-12 (447) shows only a small drop; K1's interval run at 1e-12 reads
356 against 535 (−33 %, tier=closure). What is noise: the build
critical path (631–793 s default, 647–685 s interval at tier=all) sits
inside B1's own ±25 % envelope; a one-pair billed-minutes comparison
was swamped by the k-lint draw and the render-lane scene, so the
CI-posture saving of record stays CI-MINUTES' own matched pairs,
−5 billed minutes per code-tier PR run. Two structural facts the
readout surfaces: (1) a diff touching `ci.yml` forces tier=all, which
makes the per-file gate a no-op — 4 of 6 default and 5 of 6 interval
runs sampled were such runs, so the gate's steady-state saving is not
in this sample (TCOST-9's closure-tier run skipped 33 suites; TCOST-8's
41); (2) the nightly has not run on a tree carrying C1–C3 or
TCOST-9, and its last run (09:54, pre-TCOST tree but carrying
TCOST-1's row) FAILED the gated-suite re-take at "read the nextest
pin from ci.yml" before any suite ran — the pin-reading idiom the C3
lane filed as `nightly-pin-reading-idiom-four-copies` biting first on
TCOST-1's own row; a fix is owed before tomorrow's nightly. B3's
primer: the first push to main after it was cancelled mid-save by the
next push (predicted); its restore-hit reading is still owed.

## Seam: the nightly row fixed; the primer primed (2026-09-03, 15:47 UTC)

PR 1693 merged at c5263958: the gated-suite re-take's "read the
nextest pin" step had unbalanced quotes (one of five copies of the
pin-reading idiom — the instance C3's filed issue predicted) and had
never run; all five copies now execute locally against ci.yml. Tonight's
nightly is the first that can execute TCOST-1's and TCOST-9's re-take
and C1–C3's three new jobs. B3's primer (census agent's audit of the
six pushes to main after 14:45): the first two pushes were cancelled
before or during the save; the 14:56 push completed a real save on the
default lane (762 s) and the interval save's steps succeeded under a
job-level cancel; the 15:11 push RESTORED on both lanes (its dependency
build skipped) — the mechanism works; the owed reading is the first
PR-context `build + archive` restore under the new keys.

## Seam: main's inherited red on the board; the K2 dual dispatched (2026-09-03, 18:40 UTC)

`main` has been red at the code tier since the M10 measure tags landed
ahead of the gate that pins them: `pncad-py`'s `TAG_INVENTORY` misses
`measure_clearance_refused` and `measure_selection_kind`, and nobody
saw it because `main`'s own push runs are docs-tier. The K2 lane found
it on the merge of `main` into its branch and filed it rather than
repairing it (the repair is a public-Python-surface call — M10's);
the issue file was cherry-picked onto `main` as PR 1705 (merged
601977d1) so it is on the board before either kernel lane merges.
Every K2/K3 run that carries `main` past that point shows ONE red row
on the `2/2` shard of both lanes; that row is this inherited red and
is annotated as such at merge, not chased.

TCOST-K2 reported (PR 1697, final head `efeb59f22`): L1 landed in
`geom-core::compose` (the Bernstein weight table memoized per degree
pair, one product row reused per cell block, elevation hoisted the
same way); digest md5-identical on six lane×ε rows; `offset_fit::`
−29.7 % normalised on both lanes, `geom-brep` whole-crate −8.7 %; L3
NOT landed — the census reproduced but `measure` reads its poison
verdict off `X`, so deferring `X` moves which value decides a
refusal; filed as `offset-composite-lazy-sign-gate`. The lane raised
the stop clause's ambiguity ("the crate suite" vs the suite the spec
measured L1 on) rather than reading it its own way. Rulings at the
claim: the bar reads against the `offset_fit::` suite; L3's argument
is a finding either way; the inherited red is verified, not
re-opened. Ordinal 1401 claimed on `main` (PR 1706 from
`tcost/k2-claim`); byte 128 parity 0 ⇒ R1 OPUS, R2 FABLE; briefs
hashed and stored under `/home/user/tcost-work/ab/`; both reviewers
dispatched concurrently on the frozen head with PRIVATE target dirs
(R1's is the finished implementer lane's, moved; R2's fresh).

## Seam: TCOST-K3 reported; its dual dispatched (2026-09-03, 18:50 UTC)

TCOST-K3 reported (PR 1703, final head `d5cb0f5b1`): spec option 3
on both doors — `validate_geometric_certificate[_declared]` and
`validate_pseudomanifold_certificate` return the `MassProperties`
the gate decided on, the old doors are `.map(|_| ())`, `PlusVCheck`
yields the derivation and the battery applies `plus_v_invariant`
itself, `StepImport::Solid` carries the aggregate gate's enclosure.
Phase 1 confirmed the spec's correction at the instrument (the pair
sits behind the tier-3′ door on single-solid imports), the pairs
bit-identical in all four fields; digest md5-identical on the
certified subset at six lane×ε points; no pin moved. Two facts the
spec did not have: dm1-id-214 pays ONE certificate (multi-instance;
the per-solid gate refuses first) and the saving is largest at
ε = 1e-9 (−½/−⅓/−¼ on the three rows) and flat at 1e-12, because K1
already refuses those schedules after round 0. Disclosed against the
program's own arithmetic: the unit's suite costs ≈9.6 cpu-s at 1e-12
across the two shards where the unit saves nothing there. The PR
body's Hosted CI section is unfilled at the frozen head (owed by the
fix pass). Ordinal 1402 claimed on `main` (PR 1707 from
`tcost/k3-claim`); byte 124 parity 0 ⇒ R1 OPUS, R2 FABLE; briefs
hashed and stored; both reviewers dispatched concurrently on the
frozen head with PRIVATE target dirs, alongside K2's dual on the
same box (four review lanes, one heavy cargo at a time). Rulings at
the claim: the 1e-12 acceptance line is falsified by measurement and
the unit proceeds; the suite's own cost is in scope for findings,
the per-file gate included; the inherited red is verified, not
re-opened, and the branch's duplicate issue file for it is dropped
at the fix pass.

## Seam: the K2 dual concluded; the fix pass dispatched (2026-09-03, 19:15 UTC)

Both K2 reviewers reported within 40 minutes of each other. R1
MERGEABLE-AFTER-FIXES (0 MAJ / 2 MIN), R2 MERGEABLE (0 MAJ / 2 MIN);
verdicts diverge in letter and converge in substance. Both re-derived
the identity argument and then MEASURED it with an independent bitwise
probe over every `bern_mul_row` consumer — `ch_mul`, the tensor
composites, elevation, asymmetric patch products, past-cap degrees —
and both found base and head md5-identical; neither could construct
a differing pair; both reproduced every hosted number from the raw
job logs. The one finding both hold: the patch-level key-order
mutants (tables swapped, `lo` from the wrong degree, the u-weight read
from the v-table) survive every `geom-core` row and die only in
`geom-brep`'s `offset_fit` rows — the unit's own patch row uses a
symmetric (2,2)×(2,2) fixture and compares the head against itself,
and the weight table is a bare slice whose pair is a convention
nothing enforces (a swapped table has the same row widths). Unilateral
R1: two misquoted numbers in the body (the added rows' cost quoted
from the shard that did not run them; the crate-suite figure raw
where the headline is normalised — −6.7 % under the PR's own
normalisation). Unilateral R2: the L3 obligation is one sentence, not
a soundness argument; the follow-up should be sized so. v6 item-3
unilateral instrument: no candidate. Union of 7 items dispatched to a
fresh fix lane on the implementer's arm (asymmetric guard rows against
the old loop spelled inline, a typed table carrying its pair with one
home for `lo(k)`, docs at the site, the oracle as the old loop,
`elevate_block` hoisted, the two numbers, the L3 file resized); three
declined with reason (`Cow`/names, `binom_row`'s clones for non-memo
callers, committing the instruments). Correspondence and both reports
under `/home/user/tcost-work/ab/`.

## Seam: the K3 dual concluded; the fix pass dispatched (2026-09-03, 19:35 UTC)

Both K3 reviewers MERGEABLE-AFTER-FIXES, 2 MAJ each, converging: the
kernel change is sound (both traced every path, both killed the same
mutants, both found the `unreachable!` input-unreachable) and the
unit's EVIDENCE is not — every hosted run on the branch was at
ε = 1e-12, the ε where K1 already removed the cost, so the headline
−½/−⅓/−¼ is local-only and the acceptance's hosted before/after row
is unmet; at that ε the new suite costs ≈9.6 cpu-s per lane (the #1
and #2 slowest tests of their shards) against nothing saved, ungated
though its subject persists. One MAJOR was UNILATERAL: R2 showed by
mutation that the suite is vacuous at 1e-12 — the certifying fixture
refuses after round 0 there, the identity row takes its `Err` arm and
compares nothing, and three kernel mutants live at 1e-12 that die at
1e-9. That is a v6 item-3 unilateral-instrument candidate (test-gap,
demonstrated); adjudication is recorded at merge, blinded here. R2
also showed the import path's "one certificate" has no guard (a
`gate3` that recomputes leaves every row green). R1 identified the
digest's "noise floor" as a likely ungated fuzzer (`R1_SEED` from
`SystemTime`, unpinned by `CAD_FUZZ_SEED`) and the identity as held
by convention at the non-f64 scalars. R1 corrected the dispatch: the
suite's 1e-12 cost was in the implementer's report to the
orchestrator, not the PR body, so it is undisclosed on the PR. Union
of 10 items to a fresh fix lane on the implementer's arm: the suite
certifying at every ε with its arm asserted, `gated_to!` plus a
cheaper fixture, hosted evidence at 1e-9 on both lanes with a
post-K1 merge-base probe (the plan's 2026-09-02 runs are PRE-K1 and
are not K3's before), an import-path certificate count, the planted
row's assertions made real, the digest's blind spots named and the
fuzzer filed, the identity pinned across scalars, the needle's
coverage stated, docs and one home, three files (two filings, one
duplicate dropped). Program-level lesson, recorded: a kernel unit in
a TEST-COST program has to price its own suite at every ε in the
draw, and the orchestrator's brief should have asked for a hosted
run at the ε where the saving lives.

## Seam: B3's owed after-reading, taken (2026-09-03, 19:35 UTC)

The first PR-context `build + archive` restore under B3's shared keys:
run 33791378275 (K3's `lane=both` head, 18:36 UTC), both lanes —
`Cache hit for: v0-rust-build-default-Linux-x64-fa41882e-fd5fb1c1`
and its interval twin, `full match: true`, restore 14 s / 12 s;
`build test binaries + archive` 271 s (default) / 325 s (interval)
against the 820 / 840 s cold figure B3 recorded — the build critical
path −67 % / −61 % on a branch's first build job, which is the case
F4's premise was false for. The parity step is green on the same run.
B3's after-reading is closed; the primer's ~7 billed minutes an hour
buy this on every PR run after a push to main.

## Unit: TCOST-K2 merged (2026-09-03) — the program's second A/B row

PR 1697 at `6108d6f82` (`lane=both eps=1e-12` asked, run 33796624357:
eighteen jobs green on both lanes; the two `2/2` shards red on the
inherited `pncad-py` inventory row only, annotated on the PR). Merged
at 87d33648c, recorded as ordinal 1401 / sample #119 in
`docs/MODEL-AB-LOG.md`; block TCOST-KB1 slot 1 concluded (record
branch-side on `tcost/kb1-block`). The Bernstein product's structural
weight is built once per degree pair and memoized; every coefficient,
hull and certificate bit-identical (both reviewers measured it
independently over every consumer); `offset_fit::` −29.7 % / −29.9 %
normalised on the two lanes, `geom-brep` −8.7 % raw / −6.7 %
normalised. The fix pass added the asymmetric-bidegree guard rows the
dual asked for and a typed weight table that makes a swapped key a
debug panic instead of a wrong bound. `docs/TCOST-K2-SPEC.md` deleted
at merge per the DOC-LEDGER rule (ledgered this time, with K1's).
Remaining in the block: TCOST-K3 (slot 2), fix pass in flight.

## Unit: TCOST-K3 merged (2026-09-03) — the program's third A/B row; block TCOST-KB1 concluded

PR 1703 at `497b1b48e` (two gating runs on the final tree, both
`lane=both` asked: 33802978677 at ε default and 33803928081 at 1e-12,
twenty jobs green on both lanes at each point; the two `2/2` shards
red on the inherited `pncad-py` inventory row only, annotated on the
PR; the merge-base probe PR 1709 showed the same shape on the base
tree and was closed unmerged). Merged at 6381ebdd9, recorded as
ordinal 1402 / sample #120; block TCOST-KB1 concluded — its
branch-side record (pre-draw fields, the byte-126 draw, the three
slot conclusions) merged into `docs/MODEL-AB-LOG.md` with this entry.
The tier-3 doors return the certificate they computed; the fix pass
turned the dual's two MAJORs into hosted numbers: the adopted rows
give back −13.4 / −6.7 cpu-s (default / interval lane) at the default
ε and nothing at 1e-12 (K1 already took it), against a suite now
gated to the five paths it is specific to and costing 6.0 / 6.9 cpu-s
where it runs. The v6 unilateral-instrument candidate (R2's "vacuous
at 1e-12", demonstrated by mutation) was TALLIED for R2 at merge: the
fix pass reproduced the gap from its own side. One instrument lesson
came out of the digest re-take — the shared append log tears lines
under concurrent writers, so a body measured once can vanish from a
`sort -u` roster; the "closed-form noise" was that, not a fuzzer —
and one filing on the rule alone (`R1_SEED` from `SystemTime`,
unpinned by `CAD_FUZZ_SEED`). `docs/TCOST-K3-SPEC.md` deleted at
merge and ledgered.

## Seam: after the block — two content units and a census (2026-09-03, 21:20 UTC)

With TCOST-KB1 concluded (three kernel units, three A/B rows), the
board's open items are two kernel candidates (`offset-composite-lazy-
sign-gate`, `tcost-area-pad-lever` — each a spec and a new block if
taken), one kernel finding filed by K3's fix pass (`edge-nurbs-
computes-the-chart-image-and-discards-it`), two test-only dedups and
the parked shard-count question. Dispatched now: TCOST-10 (the blend
suite's fixture builders and volume oracles homed in
`sweep/tests/common/`) and TCOST-11 (the one-declaration guard's
fourteen copies homed in `test_utils`, pncad decided) as content
units on the style-batch track; and a read-only shard-count census
over the post-K3 hosted runs (fixed cost per leg, imbalance, the
longest test, N = 2/3/4 modelled), which is the measurement the
program's keep_out asks for before any sharding knob moves.

## Seam: the shard-count question answered (2026-09-03, 21:30 UTC)

The parked `nextest-shard-count-needs-remeasure` item (issue 461 of
2026-08-13, "blocked on the test-speedup work") is closed on the
post-K3 census: test legs are 34–74 s wall (were 250–430 s), the fixed
cost per leg is unchanged at ~15.6 s and is now a quarter to a half of
a leg, the 296 s floor is a 30 s floor, and the shard imbalance the
old audit found structural is now noise around 1.15×. Stay at N=2:
every added shard costs a billed minute for a 20–45 % wall cut on legs
already under a minute; the interval rows' boundary case (~1 billed
minute per row, modelled) is not worth a measured PR. One finding for
the CI-minutes arithmetic: saturation reads ~3.9× on every leg, so the
runner presents ~4 vCPUs now, not the 2 the 2026-08 figures assumed.

## Seam: TCOST-10 reported; style batch 5 dispatched (2026-09-03, 21:45 UTC)

TCOST-10 (PR 1715, head `4da8f030`, interval 1e-6 asked, run
33809383460 green but the inherited row): the blend suite's builders
and the two oracle families homed in `sweep/tests/common/`
(`cavity.rs`, `oracles.rs`), builders retired in six suites and
oracle spellings at seven sites; `nextest list` byte-identical at
both feature sets; no execution win claimed (constructors). The lane
applied a narrower rule than the candidate asked for — "a spelling
that could not disagree comes home; one that could stays and says
so" — and kept the die/surgery Steiner spelling in five files because
its association differs at the bit level; reviewer suites gave up
their BUILDERS on the argument that the audit, not the constructor,
is their independence. Both calls go to the batch review as
questions. Filed: the same brick/prism shape outside the blend family
(`sweep-boolean-suite-brick-and-prism-copies`). Style batch 5
dispatched on TCOST-10 now; TCOST-11 joins by message when it reports.

## Seam: batch 5 on TCOST-10; TCOST-11 reported (2026-09-03, 22:05 UTC)

Batch 5 read TCOST-10 MERGEABLE WITH FIXES: nothing moved (the
reviewer re-took the test-name multiset itself, 1100 = 1100, and
checked the seven retired oracle spellings token by token), but the
reason written at seven die/surgery copy sites — "different
association, so not the same f64" — is false at the constants those
rows use (bit-identical at all but one, and that one 8.9e-16 against a
1e-9·volume tolerance); two headers went stale; a constants sweep
found four copies of the homed forms outside `sweep` that the lane's
name-based census could not see; and the rule's "could disagree"
discriminator is never exercised (a = b = c has measure zero; θ = π/2
is never visited). Ten fixes sent to the lane, including the two
agreement rows that turn the rule into a gate. The reviewer also
found the two test-support trees holding opposite rules for the same
class — `geom-brep/tests/shared` refuses reviewer-pair rebuilds "even
when the text is identical", `sweep/tests/common` absorbs them on the
argument that blend fixtures are bodies under audit, not derivations
— a reading of a ratified memory that goes to Ev (`needs_ev`), filed
by the lane with its fixes. TCOST-11 reported (PR 1716): the guard's
fourteen copies are one `test_utils::source::aggregation_violations`
call each (−807 lines of test source), a selftest plants four
violations, `pncad` gets no guard for a stated reason and a new `bvh`
row that reds if any non-aggregating crate's `tests/` grows a second
file; binary size at CI's profile +0.02 % (neutral, as the unit
says), the dev-profile +2.1 MB recorded and explicitly not quoted.
Joined to batch 5 by message.

## Unit: TCOST-10 merged (2026-09-03)

PR 1715 at `ab47d903c` (interval lane asked, ε default drawn, run
33812074765, every job green but the inherited row). The blend
suite's cavity/cube builders and the two closed-form volume oracle
families have one home in `sweep/tests/common/`; builders retired in
six suites, oracle spellings at seven sites; `nextest list`
byte-identical at both feature sets; no execution win claimed. Batch
5's fixes landed: the false "not the same f64" reason at seven copy
sites retracted with the measurement (bit-identical at all but one
constant, 1 ulp at that one against 1e-9·volume) and the copies kept
as conservatism; the two agreement rows that make the "could
disagree" rule a gate (a planted 1e-7 perturbation of each home form
reds exactly those two rows); a distinctive marker (``NOT `common::``)
at every kept copy so the disclosure list is grep-comparable (14 = 14);
the four out-of-crate copies named and filed as
`chamfered-cube-and-steiner-oracles-outside-sweep` (a CONSTANTS
sweep, not names); the home's header states it takes the opposite
line from `geom-brep/tests/shared` on reviewer-pair rebuilds, and the
policy question is on the board for Ev
(`reviewer-pair-rebuilds-two-trees-two-rules`, `needs_ev`).

## Seam: batch 5 on TCOST-11 (2026-09-03, 22:40 UTC)

MERGEABLE WITH FIXES. The reviewer re-derived rather than read: the
fourteen merge-base guard bodies hash identical (so "nothing
crate-specific was lost" is true because there was nothing to lose),
the RULE paragraph is untouched, the source-reader census's predicate
recomputed on all fourteen heads, the new `bvh` row's population
enumerated across the tree ({pncad}, and `test-utils` confirmed a
non-hazard), and TCOST-10's merge proved non-conflicting by
`merge-tree`. Findings, all cheap: the new row passes green if
`pncad`'s aggregator is renamed or moved (it keys on a readable
`all.rs`, not on `autotests = false`); the census's detection margin
fell from two tells to one and nobody is told; fourteen byte-identical
guard doc paragraphs arrived with the unit that kills fourteen
byte-identical guard bodies; the quoted binary-size figure is a local
build unlabelled; check 2's message lost `assert_eq!`'s left/right
lines and the body calls it unchanged. Six fixes sent to the lane.

## Unit: TCOST-11 merged (2026-09-03)

PR 1716 at `52ab13d1d` (default lane, ε 1e-6 drawn, run 33814096354;
earlier heads on the interval lane at 1e-6 and 1e-12; every job green
but the inherited row on each). The one-declaration guard's fourteen
byte-identical bodies are one `test_utils::source::aggregation_violations`
call each (−891 lines across the fourteen `all.rs`, +204 in the
library compiled once), with a selftest that plants four violations
and reports every one rather than the first; `pncad` gets no guard
for a stated reason and a `bvh` row keyed on `autotests = false` that
reds if a non-aggregating crate's `tests/` grows a second file — the
review's receipt showed the first spelling of that row passing green
over a renamed aggregator, and the fix closed it. Batch 5's other
fixes landed: the census's detection margin (now one tell per
`all.rs`) stated at the helper, the fourteen doc paragraphs cut to
pointers, the size figure labelled LOCAL. Binary size at CI's profile
+0.02 %: the unit buys one home, not bytes, and says so.

## Readout: the 2026-09-03 stretch (22:55 UTC)

What landed today on top of the post-K1 readout (15:47): the two
remaining kernel units of block TCOST-KB1 and two content units, all
through their reviews, plus the shard-count question closed on a
census.

- **TCOST-K2** (ordinal 1401, sample #119): the Bernstein product's
  structural weight memoized per degree pair — `offset_fit::`
  −29.7 % / −29.9 % normalised on the two lanes, every coefficient and
  certificate bit-identical (both reviewers measured it independently
  over every consumer). The dual found the suite's patch-level blind
  spot; the fix pass closed it with asymmetric guard rows and a typed
  table.
- **TCOST-K3** (ordinal 1402, sample #120): the tier-3 doors return
  the certificate they computed — the three adopted rows give back
  −13.4 / −6.7 cpu-s (default / interval lane) at the default ε and
  nothing at 1e-12 (K1 already took it). The dual found the unit's
  evidence lane wanting (no hosted run at the ε where the saving
  lives; a suite that was vacuous at 1e-12 and cost 9.6 cpu-s there);
  the fix pass bought the evidence hosted at both ε on both lanes with
  a post-K1 merge-base probe and gated a cheaper suite. The
  unilateral-instrument candidate was tallied for R2 at merge.
- **TCOST-10 / TCOST-11**: one home each for the blend suite's
  fixtures and oracles and for the aggregation guard (−891 lines of
  test source); no execution win claimed by either, none expected.
- **Shard count**: stay at N=2; legs are under a minute; the runner
  presents ~4 vCPUs now.

Where the suite stands (hosted, code-tier, both lanes, the runs of
record 33803928081 / 33802978677): test legs 46–74 s wall against the
program's opening picture of 250–430 s; the longest test 30 s at the
default ε (was 296 s); the per-file gate skipping 33–54 suites on
closure-tier diffs; the build's rust-cache restore a full-match hit on
every PR run after a push to main (271 / 325 s against 820 / 840 s
cold); −5 billed minutes per code-tier run from the CI-posture units.
`main` is red at the code tier on one `pncad-py` row (M10's; on the
board), which every PR of the day carried as an inherited red on its
`2/2` shards.

Open on the board: two kernel candidates that would each need a spec
and a new A/B block (`offset-composite-lazy-sign-gate`, ~15 % of the
recentring call in the all-or-nothing case, bounded by the mixed
rounds; `tcost-area-pad-lever`, the area pass a refused patch face
still pays), one kernel finding (`edge-nurbs-computes-the-chart-
image-and-discards-it`), two test-only dedups (`sweep-boolean-suite-
brick-and-prism-copies`, `chamfered-cube-and-steiner-oracles-outside-
sweep`), two parked items, and one question for Ev
(`reviewer-pair-rebuilds-two-trees-two-rules`: `geom-brep/tests/shared`
refuses reviewer-pair rebuilds even when identical; `sweep/tests/common`
absorbs them on the argument that blend fixtures are bodies under
audit — one reading should win and be written into the memory).
Recommendation: the two test-only dedups are cheap and can go as one
content unit; the kernel candidates are worth a second block only if
Ev wants the A/B sample to grow — the suite's seconds are no longer
where they were, and the next lever on the real program's cost (the
area pass) is a kernel question more than a test-cost one.

## Seam: Ev's ruling on reviewer-pair rebuilds (2026-09-04)

Neither tree's rule survives. The policy memory was overstated — Ev
had thought it revised to "reviewer tests are ordinary tests" and the
revision had never reached the in-repo copy — so the memory is
rewritten to that, geom-brep's "must never absorb even when identical"
bullet is now the reason-at-the-copy rule (a claim that needs its own
derivation keeps it; authorship is not a reason), and sweep's
"opposite line" paragraph is the ruling. The orchestrator's own
"fixture drift" argument for a compensating row was retracted as a
nuisance change detector: the unit suites' pinned volumes and oracles
already red on a changed body. Issue closed; nothing else moves.

## Seam announced by TOPO (2026-09-05)

TOPO's `D261` (`topo/d261-reader-collapse`) converts four
`crates/topo/src` readers onto `test_utils::source` and edits
`crates/test-utils/tests/reader_census.rs` for exactly those entries,
re-deriving `UNCONVERTED_TODAY` from the table at its landing (S-BOOL's
`D287` lowers the same constant; whichever lands second re-counts).
Nothing else under `crates/test-utils/` moves.

## Filed by TOPO; one more seam (2026-09-05)

`source-lacks-an-item-body-carve-and-shared-means-any-mention` filed on
this slate from D261's style review. The seam already announced widens
by ONE function: D261's fix pass adds an item-body carve beside
`balanced_end` in `crates/test-utils/src/source.rs`, with its rows, and
converts its own caller; the other four copies are this item's.

## Announced seam from PROPS (2026-09-05): two editor-core test files

The k-stats unit (PR #1969, `work/props/k-stats-escalation-channel-and-redo.md`)
converts every caller of the retired `start_verdict_log`/`take_verdict_log`
pair to the new `k_stats::Bracket`, and two of them are TCOST's:
`crates/editor-core/tests/m4_pr4_resolve.rs` (mechanical) and
`crates/editor-core/tests/asm2a_instantiate.rs`, which also gains the
unit's red-first nesting row (an instantiate node records its own op's
decisions whichever instance ran the part) and the parallel-schedule rows
adopted from the review. The spec's fence named the callers as a class;
the review found these two unlisted in the body and the fix pass lists
them. Signed (PROPS orchestrator).

Addendum (2026-09-05, PROPS orchestrator): the k-stats bracket also
adds `crates/editor-core/tests/kstats_bracket_rows.rs` (the
schedule-independence, part-in-part, memo, cancel and pre-pass rows the
review adopted) and one row in `tests/asm_r2a_mate_solve.rs` (a
mate-solve escalation sits on no node's log), and registers both in
`tests/all.rs`; `m10_7_r2_probes_interval.rs` and its goldens are M10's
by that program's own seam note. Signed (PROPS orchestrator).

Addendum (2026-09-05, PROPS orchestrator): the rotation-floor rider
(#1980) rewrote one doc block in
`crates/geom-brep/tests/revolved_point_anchor.rs` — a sentence that
unit itself made false ("still recorded as an audit member … a sixth")
now points at the paragraph at `Mat3::rotation_about`; no assertion
moved. The vec3-doors rider (#1977) adds `crates/profile/tests/sketch_plane.rs`,
which the tool lists as tcost's as well as bool's. Signed (PROPS
orchestrator).

## One census line from TOPO's D50 (2026-09-06)

`crates/topo/src/live.rs` now reads Rust source (the `Live` guard row),
so `reader_census.rs` gained one `Shared` line for it, forced by the
census's own arrival detector; `UNCONVERTED_TODAY` untouched. Landed
with PR 1949.

## Seam announced by TOPO (2026-09-06): the Euler-counts door

Ev ratified `readback::euler_counts` / `genus()` (PR 2010). The unit
converts the hand-written identity in `crates/topo/tests/*` (four
files) and `crates/sweep/tests/*` (ten files) to the door — test rows
as ordinary tests, one call replacing five `.count()`s and a sum per
site; nothing else in those files moves. Lands on
`topo/census-door`; say here if a TCOST lane is live on any of them.

## Finding relayed by TOPO (2026-09-06, from S69's review)

`crates/test-utils/src/fuzz.rs:283-288`: `fuzz::replay()` reports the
PROCESS's random seed while `fuzz::pinned()` ignores it, so a pinned
row's failure message says "reproduce with CAD_FUZZ_SEED=<random>"
naming a seed the row never used (executed on
`review_m1_pr4.rs:1544`'s coverage row). This program's file; not
filed by TOPO.

(BLEND orchestrator, 2026-09-08) Seam announced for BLEND-6
(`docs/BLEND-6-SPEC.md`, block BLEND-B1 slot 0): the repaired boss and
its dimple twin are homed once in `crates/sweep/src/test_support.rs`
and their copies in `crates/sweep/tests/review_fillet_h5_r1_probes.rs`
and `fillet_h5_hostless_rim.rs` deleted in favour of it; the ring-
clearance refusal pins in those files and `fillet_h5_r2_probes.rs`
flip to carves. A new suite `blend6_ring_clearance.rs` is added.

## Announced from LIB (2026-09-09): one row added to `profile/tests/sketch_plane.rs`

LIB-MIRROR (PR #2271) adds `the_partial_eq_impl_is_bit_eq_and_answers_the_same_on_the_two_zeros` beside the existing `bit_eq` row, pinning that `SketchPlane<f64>`'s new `impl PartialEq` answers exactly what `bit_eq` answers; one test, no fixture and no runtime cost.

## Seam: the whole slate re-sorted against the public repo (2026-09-11)

Ev asked, in chat, for every entry on this program's board to be sorted
three ways now that `evgunter/cad` is public and standard-runner minutes
are free: still wanted because its subject hurts LATENCY; unwanted
because it was only ever buying billed MINUTES; and unclear. All 65
entries were read. The finding that decides most of them is structural
and worth stating once.

**Latency here is one chain, and it is not the tests.** `ci.yml` has
exactly two dependency edges — `test` needs `build`, `test-interval`
needs `build-interval` — and every other job hangs off `filter` in
parallel. A run's wall is therefore the max over chains, and the max is
`build + archive (interval)` at **388 s median** plus a test leg at
**34–74 s**, against a whole-run median of **442 s** / **482 s** at tier
`all` (`work/ciw/f3-recosting-on-a-public-repo` §M2, 4-vCPU runner;
`nextest-shard-count-needs-remeasure`'s 2026-09-03 re-measure for the
legs). So: the BUILD is the pole, the test legs are the short end of it,
and anything that trims a parallel job returns nothing to the person
waiting. That is the whole sort, applied per row.

**Three of this program's landed units are minutes and nothing else.**
TCOST-C1, C2 and C3 each moved a check off the PR gate and each priced
itself in billed minutes: −2, −2, and 43 s off a 222 s parallel `fmt`
job. None of the three was ever on the critical path; C1's own nightly
header records the job at 93 s. What they cost is attribution, plus two
rows (`review_d18`'s `cfg(not(debug_assertions))` pair) that now compile
in no PR lane at all. `work/ciw/f3-recosting-on-a-public-repo`
§*What is therefore open* item 2 named these three for re-costing on
2026-09-04 and left them to this program; this program never looked.
Filed as `nightly-demotions-c1-c3-were-bought-with-billed-minutes` — a
re-cost with a hosted wall reading per row, not a revert on sight, since
C3's job has never had one and is the one that could plausibly approach
the pole.

**One open row closed.** `skip-eps-battery-by-observing-oncelock`
(parked since 2026-08-19 on Ev's *defer, not reject*) is now REJECTED.
Its entire prize was ~90 cpu-s spread across ε legs that run in
parallel, so it never shortened anything; and it would take 251 tests
off two of three ε rows, which were put BACK on every run on 2026-09-04
for exactly this reason (`scripts/ci-filter.py` §CONFIGURATION
COVERAGE, `work/ciw/reinstate-full-configuration-runs`). The mechanism's
design is kept in the file; the cost case is dead.

**One closed row re-opened, and it is the only latency work the billing
change unlocks.** `nextest-shard-count-needs-remeasure` closed at N=2 on
2026-09-03 — the day the repo went public, and the fact did not reach
the verdict. Read in full, that verdict is one quantity: *"every added
shard costs a full billed minute for a 20–45 % wall cut"*, and for the
interval rows *"≈1 billed minute per row"* against N=4 at ~40 % less
wall. The cost side is now zero and the benefit side is untouched, on
the legs that carry the last job of the critical path. It still needs
its own measured PR under the program's `keep_out`; what changed is that
the arithmetic no longer refuses it.

**Everything else on the board survives untouched, and mostly because it
was never a cost lever.** Of the ~28 open rows, the fourteen Track W
units (C18, D70, D72, D113, D380–D386, H12, S216, S230) and most of the
slugs arrived here in the tracker-wide re-home of 2026-09-04 by PATH
GLOB, not by a cost argument — `malformed-ambient-eps-reds-review-m2-pr7-k`
says so in its own body. They are test-integrity, dedup and citation
rows; free minutes change nothing about them, and several (S216's
compile-per-row gate, D381's missing pin) get slightly EASIER to justify
now that the compute they would add is free. The four genuine latency
rows on the board are `rust-cache-never-restores-across-branches` (which
already re-stated its own case on latency grounds when the repo went
public), `tcost-area-pad-lever`, `offset-composite-lazy-sign-gate` and
the re-opened shard count; the first cuts the pole itself and the middle
two are kernel constant factors that the shipped program pays too.

**Two rows are genuinely unclear and neither is this lane's to settle.**
`proptest-modules-in-src-ungated` asks Ev to close the gap between
*"a fuzzer that is not gated is a defect in the fuzzer"* and that rule's
stated rationale, which is per-run cost — free minutes WIDEN that gap
rather than close it, so the question is more live, not less, and the
row stays open unchanged. And the two gate-defect rows
(`gated-marker-omits-sibling-helper-imports`, `gated-marker-path-mount`)
are real bugs for exactly as long as the per-file gate stands; whether
it stands is downstream of the same ruling.

## Seam: the non-cost half of the board leaves for S-TINT (2026-09-11)

Ev's direction in the same conversation as the re-sort above: the fourth
bucket — the rows that are not cost levers in either currency — gets a
track of its own rather than sitting on a cost program's slate. **Thirty
of this program's 42 live rows moved** by `git mv` to `work/tint/`
(S-TINT — test-suite integrity, band 3500–3599), ids, titles and bodies
unchanged, each carrying a `## Moved to S-TINT (2026-09-11)` record.
The fourteen Track W units (`C18`, `D70`, `D72`, `D113`, `D380`–`D386`,
`H12`, `S216`, `S230`) and sixteen slugs this program's lanes filed while
measuring. `work/tint/log.md`'s opening entry carries the full split and
the list of what stayed.

**Twelve rows stay, and they are the whole of this program's live
board:** the four latency rows (`rust-cache-never-restores-across-branches`,
the re-opened `nextest-shard-count-needs-remeasure`, `tcost-area-pad-lever`,
`offset-composite-lazy-sign-gate`), the unmeasured kernel candidate
(`edge-nurbs-computes-the-chart-image-and-discards-it`), the demotions
row filed above, the two per-file-gate defects, the two fuzz-gating
policy rows, the parked proptest question, and the `ci-filter.py`
citation fix. This program is now what its charter says it is and
nothing else.

**The territories overlap deliberately.** S-TINT claims
`crates/*/tests/*` and `crates/test-utils/*` as well, because the fence
between the two programs is the QUESTION and not the path, and no glob
expresses it. `work.py territory` will warn on branches of either;
`work/tint/plan.md` §*The fence with S-TCOST* is the rule a lane reads
instead of guessing, and it names the three things that stay this
program's whatever they look like — the gate mechanism, the fuzz-gating
policy, and everything under `scripts/`. A row on either slate that turns
out to be on the wrong side moves back by `git mv`, not by a copy.

## The unclear kernel row is measured, and it closes negative (2026-09-11)

`edge-nurbs-computes-the-chart-image-and-discards-it` was the one row
the re-sort could not place: a compute-and-discard candidate whose own
body said the cost had never been measured. It is measured now, and
both halves of the row fail.

**The premise is false.** The image is not discarded —
`edge_nurbs.rs:354-362` passes it into `certify_rung3` as
`Some(&pcurve)`, and `ssi/certify.rs` refuses `UnsupportedCertificate`
without it at `:810-815` and `:851-854`. Every `PlaneNurbsLimbs` field
but `min_sin_theta` comes out of that certificate. What exists is a
redundancy ACROSS PASSES (the mint derives its own), and collapsing that
means the certifier trusting the stored cache, which
`topo/src/validate.rs:3297-3299` ratifies against.

**And the prize is small.** LOCAL, single-threaded medians on this
container, an iteration tool and not a result of record: `chart_image`
is **9.5 %** of an edge certification in release (0.461 of 4.86 ms) and
**12.6 %** in dev, once per plane×NURBS edge per validation pass.

Recorded because it is the general lesson of this re-sort, not just this
row's: **the row was filed off a sentence in a spec, and the sentence
had gone stale.** `docs/PCURVE-P2-SPEC.md:55-62` still says
`edge_nurbs` *"THROWS IT AWAY"*, and the advice that sentence gives —
prefer the existing producer to writing a third — had already been taken
by the refactor that made `chart_image` shared. Eight days on this board
and one instrumented lane to find that out. Filed to TRIM, whose
territory both files are:
`work/trim/pcurve-p2-spec-says-edge-nurbs-throws-the-image-away`.

The board is eleven live rows: the four latency rows, the demotions row,
the two gate defects, the two fuzz-policy rows, the parked proptest
question, and the `ci-filter.py` citation fix.

## Unit: the gate's converse check, and two rulings (2026-09-11)

**`gated-marker-omits-sibling-helper-imports` is closed with the arm
landed.** `--gated-check` now requires that every sibling helper module a
gated `tests/` suite imports is covered by its marker's path set, by the
same match `selected_by` makes. Two findings worth the log:

- **The item's own fix sketch was wrong in the load-bearing place.** It
  said to resolve `use crate::<h>` "the way `_all_rs_modules` already
  does" — but that reader records `#[path]` PAIRS, and every helper tree
  in this repo is mounted by a bare `mod common;`. Built on it the check
  resolves nothing and passes every tree in green: this row's own defect,
  one level up. Caught by looking at the output (zero suites resolved on
  a tree the census said had ten), not by reading. `_tests_sibling_files`
  is a second reader; `_all_rs_modules` is untouched and no term moved.
- **It found three offenders on main, and they are not the ten.**
  `review_chamfer_r1_probes`, `review_verbs_rim_lever_probes` and
  `verbs_rim_r1_probes` in `crates/sweep`, all importing
  `use crate::common;` and naming no part of
  `crates/sweep/tests/common/`. All three markers were written
  **2026-09-09** (`8ee8cf1c`), six days AFTER the census swept all 54 and
  widened the ten it found. The class regrew at the predicted rate under
  the review the row said would not catch it — which is the argument for
  a mechanical check over another sweep. Fixed in the same commit.

Planted both directions in `scripts/gates/gated-suite-paths.sh` (an
announced cross-fence edit; that file is code-quality Track K's and calls
`--gated-check` rather than deciding anything): the gate must red on an
unnamed helper import, and must pass the near miss this directory's
convention owes — the same import with the directory named.

**`gated-marker-path-mount` is SIZED, not taken.** Its two candidate
fixes are now priced against the code: "make the silence loud" is ~15
lines on the reader just built; "resolve the mount" needs a module-tree
walk of `crates/*/src` whose nested-mount case would reproduce this row's
own failure if done by halves. The two buy different things and the
choice is Ev's; the recommendation on the item is candidate 2 now,
candidate 1 only when a row genuinely wants gating in a mounted file.

**`consider-proptest-for-randomized-sweeps` is closed on Ev's ruling**
(in chat, 2026-09-11): *"the fuzzers have found things only rarely and it
seems like understanding the problem found by the fuzzer hasn't been too
burdensome."* The whole case for migrating was shrinking — the issue says
so itself — and shrinking is a tool for making diagnosis cheaper. The
harness it was parked on already exists (`test_utils::fuzz`), the
memory's fuzzing rules are untouched, and `r1-probe-seeds-are-not-on-the-
fuzz-dial` stays open: it is the one place the reproducibility floor this
ruling assumes is not actually met.

Board: eleven live rows (corrected 2026-09-12 — the count here was
wrong; see the correction at the tail).

## Unit: the gate's mount arm (2026-09-11)

`gated-marker-path-mount` closed on candidate 2, the cheap one, on Ev's
direction. `--gated-check` refuses a marker on any `crates/*/src` file
another `src` file `#[path]`-mounts, naming the mounting file and line.
Candidate 1 (resolve the mount) is on the record as sized and declined:
refusing is exact, resolving has to be RIGHT, and a one-level resolver
would derive a wrong prefix on a nested mount — this defect one level
deeper.

**The census had grown, and re-deriving it is what caught a reader bug.**
The row named three mounts on 2026-09-03; there are seven at `76d4bb0d`
(the original three, plus three new `geom-core/src/sym/` ones, plus the
`mesh` lib mount the row had set aside). None carries a marker, so the
tree is green from the first run — the guard changed, not the tree.

Two shapes the reader had to get right, both live:

- **Intervening attributes.** `crates/mesh/src/lib.rs:288` writes
  `#[cfg(test)]`, `#[path]`, `#[allow(...)]`, then `mod`. The first draft
  required the two to be adjacent, missed it, and reported a clean tree.
  The census — seven expected, six found — is what caught it. Same lesson
  as the sibling row the same day: **on both arms the first draft was a
  reader that silently saw nothing, and both were caught by counting the
  output against a census rather than by reading the diff.**
- **A mount target need not be under `src/`.** That mesh mount reaches
  `crates/mesh/tests/common/witness_bodies.rs`, which is now refused too
  — stricter than the row's own note, and it names the mounting line,
  which is the fact an author needs.

Planted both directions in `gated-suite-paths.sh`, announced cross-fence
as before: a marker on a mounted file must red (its paths otherwise
valid, so the red can only be this arm), and a mounted file with no
marker must pass — the live tree's own state, and what a gate firing
there would red `main` over.

Both gate-defect rows are now closed. Board: ten live rows (corrected
2026-09-12 — "six" here counted only the latency rows, the demotions row
and `r1-probe-seeds-are-not-on-the-fuzz-dial`, and silently dropped four;
see the correction at the tail).

## Seam: Ev's EFFORT proposal, and the measurement that prices it (2026-09-11)

Ev, in chat: run every fuzzer at EFFORT = 1 always, and let the marker
select which ones run DEEPER — plus the observation that compilation time
is a factor. Drafted as an `[ev]` PR
(`fuzz-depth-not-existence-run-everything-at-effort-1`, `needs_ev`); the
`memories/test-suite-cost.md` clause rides it and nothing is wired until
Ev signs off.

**Half of it already exists and the other half exists nowhere.**
`fuzz::effort()` already defaults to 1 and the harness already calls the
shipped counts a smoke level, so "run at EFFORT = 1" is what every kernel
fuzz row already does when it runs at all. And NOTHING in the kernel ever
runs above 1: the only `CAD_FUZZ_EFFORT` in CI is the
`interval-transcendentals` oracle job's `"8"`, a different workspace. The
nightly re-take runs the gated set at EFFORT = 1 too, so it buys breadth,
not depth. The proposal is two edits, not a redesign.

**The measurement was already being taken and nobody had read it.** The
nightly's `gated suites (ungated re-take)` runs the whole gated set,
ungated, at EFFORT = 1 — exactly the population and exactly the dial the
proposal would add to every PR. Its own header says the reading was owed
from its first firing and never taken. Two nights:

| night | `Summary` wall | tests | slowest row | 2nd |
|---|--:|--:|--:|--:|
| 2026-09-11 | **83.599 s** | 419 | 83.310 s | 23.490 s |
| 2026-09-08 | **66.359 s** | 419 | 65.896 s | 18.102 s |

**The gated set's entire execution wall is ONE test** — 83.599 against
83.310 — with the other 418 finishing in its shadow at ~0.007 s each.
Cost concentrates savagely, measured. So the proposal's price is not a
policy question but a single row, filed as
`m10-3-chamber-row-reads-ten-times-its-recorded-cost`: that row reads
66-83 s against TCOST-6's recorded 6.7 cpu-s, a factor of ten nobody has
explained, and it is now the largest lever on this program's board.

**Ev's follow-up found the carve-out**: the `interval-transcendentals`
oracle fuzzer is NOT in the 419 and must not be swept into the rule. That
root is outside the workspace, so a PR does not compile it anyway —
~234 s of build to buy ~7 s of cases at EFFORT = 1. The rule's premise is
that the binary is compiled regardless; where that is false the job-level
gate stands. That lane is the proposal's precedent rather than its
exception: it is the only thing in the repo already buying depth on the
changes that reach it.

Also: `work/tcost/program.md`'s `keep_out` now names S-TINT, making that
territory overlap two-sided (22 warnings from 23; the rest pre-date this
and mirror overlaps S-TCOST already had).

## The M10-3 cost discrepancy is diagnosed: a 15x regression the gate hid (2026-09-11)

Ev asked for the diagnosis; it is a regression, it is attributable, and
the recorded figures were never wrong.

**Measured**, one box, same profile and command, two trees: the suite
goes **21.041 s at TCOST-6's merge (`a4439fbef`) to 319.373 s at
`origin/main`** — 15.2x, with the three rows at 15.2x / 9.0x / 6.3x.
TCOST-6's recorded 6.7 cpu-s is CORRECT for the tree it was taken on:
21.0 s at opt-0 over the 3.8x local:hosted ratio measured on this same
row is ~5.5 s. Nothing was mis-measured.

**Bisected** over `a4439fbef..origin/main`, 2 512 revisions, 11 steps, on
the cheapest discriminating row: **PR #1725 (`m10/m10-7-symbolic`)**, at
the commit that names its own mechanism — *"driver: replay at
`Sym<Interval>`, the dials and the receipt; re-cut the M10-3 limit
rows"*. The leaf budgets did not move; the arithmetic under every box of
the subdivision did. Nothing in `work/m10/` or in the test file records a
runtime cost for that change, and the file still carries TCOST-6's
measured prose ("1.46 s here against 0.98 s at 1024") beside constants it
no longer describes.

**The gate is why it sat eight days.** The suite is `gated_to!`
editor-core's modules; #1725 changed `geom-core`, which is not in that
set — so the gate SKIPPED the suite on the pull request that made it 15x
more expensive, and on nearly every one since. A skipped test contributes
no row to the `Slowest N tests` report, so the instrument that exists to
catch this could not see it. The one lane that ran it is the nightly
ungated re-take: eight nights at 66-83 s, flagged `SLOW`, green, unread.

Two open arguments now have this as evidence rather than reasoning: that
a gate deciding EXISTENCE hides what a dial deciding DEPTH would show,
and that a detector nobody reads is not a control (Ev, 2026-09-07).
Filed with both dispositions on
`m10-3-chamber-row-reads-ten-times-its-recorded-cost`; whether
`Sym<Interval>` is worth its seconds is M10's call and this program does
not reopen it.

## The M10-3 cost is 95 % symbolic tier; filed to M10 (2026-09-11)

Ev asked for an issue on speeding up the symbolic form. Quantified first,
one line changed (`SymbolicDials::default()`'s `enabled`), same box and
command: **tier off 15.364 s, tier on 319.373 s** — the tier is 304 of
319 seconds, **95.2 %, a 20.8x multiplier**. The rest of the kernel did
not regress: with the tier off the suite is faster today than the whole
suite was before E12 existed (21.041 s at `a4439fbef`). The entire delta
is E12.

Two corrections to yesterday's reasoning fall out of it. The degree-16
result is **explained and the dial exonerated** — `drive.rs` predicts it
in as many words (*"at 16 it freezes and the row does not move"*), so a
frozen form sends work back to subdivision and 16 is slower than 128.
And `CHAMBER_LEAVES` should NOT be re-cut: the budget was measured
correctly, the rows assert what they assert, and the seconds belong to a
kernel tier the real program pays too.

Filed as `work/m10/symbolic-tier-costs-95-percent-of-the-m10-3-drive` —
M10's slate, because M10 designed the tier (M10-7), owns `drive.rs` where
both budget constants live, and owns the rows that pay. Named in it:
`geom-core/src/sym*` is PROPS's by glob, so a fix inside the normal form
is an announced cross-fence change. The row asks for a PROFILE first —
which of degree growth, the `num-bigint` coefficient ring or per-term
allocation dominates — because the one hypothesis taken from reading the
code (the degree constant) was refuted by measurement, and a patch
written the same way would have made the row slower.

Recorded there without an argument attached: **all nine rows pass with
the tier off**, which is a coverage question for M10 and not a case for
turning it off.


## Correction: the board counts in the two entries above were wrong (2026-09-12)

Read off `work.py status --program tcost` rather than counted by hand,
which is how the error happened: both figures were derived from the
enumeration a lane had in its head at the time, and both enumerations
were short. **Ten rows are live**, and here they are in full so the next
reader does not have to re-derive them either:

| row | what it waits on |
|---|---|
| `fuzz-depth-not-existence-run-everything-at-effort-1` | **Ev** — the `[ev]` PR carrying the `memories/` clause; nothing wired |
| `m10-3-chamber-row-reads-ten-times-its-recorded-cost` | M10's `symbolic-tier-costs-95-percent-of-the-m10-3-drive`; diagnosed, no work left on this side |
| `nightly-demotions-c1-c3-were-bought-with-billed-minutes` | a hosted wall reading per job, C3's especially — never taken |
| `nextest-shard-count-needs-remeasure` | its own measured PR (N=3/N=4 on the interval legs) |
| `rust-cache-never-restores-across-branches` | a unit; the pole itself |
| `tcost-area-pad-lever` | a spec, then a kernel unit |
| `offset-composite-lazy-sign-gate` | a spec, then a kernel unit |
| `r1-probe-seeds-are-not-on-the-fuzz-dial` | a unit; more load-bearing if the `[ev]` clause lands |
| `proptest-modules-in-src-ungated` | closes WITH the `[ev]` clause, not before |
| `ci-filter-cites-a-path-the-ledger-recipe-cannot-open` | two comment lines |

The lesson is small and general enough to keep: **a board count belongs
to `work.py status`, not to a log entry's prose.** A hand-written total
in a narrative goes stale the moment the next row lands, and this program
wrote two of them wrong in one day while auditing other people's stale
figures.

## C3 measured; all three demotions are clear to restore (2026-09-12)

The reading `nightly-demotions-c1-c3-were-bought-with-billed-minutes`
asked for and nobody had ever taken. Hosted, over the 32 most recent
completed pull-request runs, every run where the job actually executed:
the **`python suite` job is 106 s median (n = 15), range 88-127 s**,
against a code-tier run wall of 845 s median in the same window — **13 %
of the run**. It was the one of the three that could plausibly have been
the pole. It is not.

With C1 at 93 s and C2 at 43 s off a 222 s `fmt` job, all three are now
measured on one axis and none of them is on the critical path: the wall
is the `build + archive` -> `test` chain and all three hang off `filter`
beside it. The row's ask 2 therefore stands for all three — restore
them; nothing here costs a contributor a second, and each pays for that
in attribution.

Noted on the row rather than reconciled: this window's 845 s wall median
sits above the 442-482 s in `work/ciw/f3-recosting-on-a-public-repo` §M2,
which measured a different window and a different endpoint pair. The
conclusion is a ratio and holds on either denominator; whoever restores
the jobs takes the before/after from their own PR.

## rust-cache closed on its after-reading; the budget half split out (2026-09-12)

`rust-cache-never-restores-across-branches` is closed. Its premise —
*"restored nothing on five of seven build jobs"* — is false on today's
tree, and the fix is the one the row itself proposed: **TCOST-B3's
`push: main` primer landed and works.** Over the 32 most recent completed
PR runs, every `build + archive` job that executed restored — **0 of 16
miss-shaped per lane**, restore step 13 s median — and the jobs sit at
**251 / 279 s** against the row's own 820 / 840 s cold figure. Grounded
rather than inferred: one job's log read directly prints
`Cache up-to-date.` The row had sat open for nine days describing a tree
that had moved.

**Half of it did not close.** The primer works by REFRESHING a shared key
on every main push, so eviction costs one push's staleness; that says
nothing about an entry written ONCE under a hash key — which is what
TCOST-C4 measured churning out of the 10 GB budget inside the hour, and
what `work/ciw/cache-rendered-cells-on-input-hash` is parked on. Filed as
`actions-cache-budget-under-a-hash-key` and the CIW row re-parked onto
it in the same commit, because a closing row may not un-park another
program's item by leaving its blocker dangling — lint caught exactly that
and the rule says fix the stale row rather than soften the check.

**The pattern is now worth naming.** Five rows this session were figures
describing a tree that had moved: TCOST-6's 6.7 cpu-s, this file's own
1.46 s in-file timing, `PCURVE-P2-SPEC`'s "THROWS IT AWAY", C3's reading
that was never taken at all, and this row's five-of-seven. Each was
written accurately and none was re-read at the change that falsified it.
The gate cannot catch this class — a stale NUMBER in prose reds nothing —
so the only thing that does is a lane re-deriving a figure before acting
on it, which is what found all five.

## The EFFORT policy is ratified and unwired; the row says so now (2026-09-12)

Ev signed the clause off and it merged at PR #2363. The row that carried
it still read `needs_ev: true` and was titled around the measurement,
so the board reported "1 on Ev" for a question Ev had already answered —
the board lying about its own state, which is the class this program
spent two days fixing elsewhere.

Corrected: `needs_ev` dropped, retitled to name the live work
(**`Wire the EFFORT policy: ci-filter.py selects a raised EFFORT instead
of excluding suites`**), and the wiring section promoted from "if Ev
signs off" to THE WORK. Five steps, unchanged in substance.

**Nothing is wired.** `scripts/ci-filter.py` still emits an EXCLUSION and
still decides existence; no lane in the kernel runs above EFFORT = 1.

**Not blocked, but ordered.** The gated set's execution wall is one suite
whose cost is a kernel regression now on M10's slate. Wiring before that
is fixed puts a 66-83 s row on every pull request; after, the same step
costs about a second of leg time. A lane may go first and owes the
measurement of what it lands.

## New orchestrator; the A/B protocol is off and the review axis changes (2026-09-12)

The program changed hands. Ev's instruction on the handover, in chat, is
the whole of the new review rule: *"don't use the AB protocol and just do
style reviews unless it's a unit with high risk of being wrong which
deserves a full review."*

**What that retires.** The 2026-09-02 split decided the review track by
**what the diff touched** — test-only work got a batched style review and
no A/B row, kernel-logic work got the standard v6 dual from band
1400-1499. Both halves go. There is now one default (a style review) and
one exception (a full review), and the axis between them is **risk of
being WRONG**, which is a different question from whether a diff reaches
the kernel. A test-only diff that changes what CI RUNS can qualify; a
kernel diff whose every claim a digest already checks can fail to.

Written down in `work/tcost/plan.md` §Review with what raises the risk,
because "high risk of being wrong" is a judgement and a judgement left
unelaborated is re-derived differently by every dispatch. The
orchestrator names the track **in the dispatch**, with its reason, so a
reviewer knows what standard is being applied to them.

**The band stays claimed and closed.** S-TCOST drew three ordinals —
1400, 1401, 1402, for TCOST-K1/K2/K3 — and draws no more.
`docs/MODEL-AB-LOG.md`'s banding entry records the ruling rather than
releasing the range, on the VIEW precedent (Ev, 2026-09-04): recorded
rows keep their ordinals for life, so releasing the band would put a
future program's ordinals on top of three that already exist. Its entry
also cited `docs/S-TCOST-LOG.md`, a path `work/README.md` now refuses
outright; repointed to `work/tcost/log.md` in the same commit.

## The board re-read cold, and one row was lying (2026-09-12)

Ten rows live, read off `work.py status --program tcost` and not
enumerated by hand — this program wrote two board counts wrong in one day
by doing the latter.

**`m10-3-chamber-row-reads-ten-times-its-recorded-cost` is parked**, not
open. Its diagnosis is complete, its fix is a kernel change inside a tier
M10 designed and owns, and it has had **no dispatchable work on this side
since it was filed**. It was reporting as available work it is not.
Parked on `work/m10/symbolic-tier-costs-95-percent-of-the-m10-3-drive`,
which is the thing that can actually fire.

That is the same class this program has been auditing in other people's
trees all week — a board figure describing a state that has moved — and
it was on our own board. The general lesson holds and is already written
in this log: a row's status is a claim, and a claim nobody re-derives
goes stale silently.

## Three lanes dispatched (2026-09-12)

All three get **style reviews**; none is a full-review unit, and the
reason is recorded per lane rather than assumed.

- **`tcost/ci-filter-gui-log-citations`** — two comment lines repointing
  `scripts/ci-filter.py`'s dead `docs/GUI-LOG.md` citations at the recipe
  that opens them. No behaviour change.
- **`tcost/c1-c3-restore`** — restore TCOST-C1/C2/C3 to the PR gate and
  rewrite their demotion notes off billed minutes. Ask 1 of that row (a
  hosted wall reading per job) was discharged on 2026-09-12; asks 2 and 3
  are the work. The lane takes its own before/after rather than quoting
  the row's table, per the row's own closing sentence.
- **`tcost/shard-n-remeasure`** — the measured N the shard row has been
  waiting for since its verdict was re-opened on a currency that no
  longer exists. Explicitly allowed to land "stay at N=2": the
  deliverable is a measurement, not a change.
- **`tcost/r1-seeds-on-the-harness`** — route the R1 probe rows through
  `test_utils::fuzz`. The `viewer` site the row calls "a third site" is
  **in scope**: fixing two of three identical instances is the half-fix
  this project's standing failure is made of.

**Not dispatched, and why.** `fuzz-depth-not-existence-run-everything-at-effort-1`
is the biggest live row and is ordered behind the M10 regression — wiring
it now puts a 66-83 s row on every pull request, wiring it after costs
about a second of leg time. Nothing forbids going first; the ordering is
worth more than the week. `proptest-modules-in-src-ungated` closes with
it. The two kernel units (`tcost-area-pad-lever`,
`offset-composite-lazy-sign-gate`) each want a spec first and are the two
rows that WILL take full reviews — both rest on bit-identical
certificates and refusal classes that only a digest can check, which is
the risk shape §Review names.

## The two ci-filter GUI-LOG citations resolve now (2026-09-12)

`scripts/ci-filter.py` carried the fullest paraphrase of Ev's
viewer-CI-posture ruling in live code and cited `docs/GUI-LOG.md` for
it — a path the ledger's own recovery recipe cannot open, because the
file was renamed to `work/gui/log.md` before the `gui` directory was
deleted. Every other CI site points HERE for the argument, so this was
the one citation a reader checking whether the code still applies the
ruling would dead-end on.

Both sites now carry the house spelling that CIW unit 7 landed on the
other six, copied from the landed text rather than re-invented. Comments
only. That closes the CI-code class at 8 of 8. The four left are the
provenance class the CIW item separated out and declined —
`docs/MODEL-AB-LOG.md`'s banding entry and three tracker items, none of
them ours — and they wait on the ledger's rename note rather than on a
re-point.

## The shard count is measured, and it stays at 2 (2026-09-12)

`nextest-shard-count-needs-remeasure` closed on twelve hosted code-tier
runs — N = 2 (control), 3, 4 and 6, three full runs each, four probe
branches off one commit differing only in the shard literal, all four
counts running in the same wave so a wave's runner weather is shared.
Every run id is on the row. The probe PRs (#2428-#2431) were measurement
only and are closed; their `tcost/probe-n*` branches are still on the
remote, because this lane's token is refused ref deletion — whoever can,
should delete them. The landed diff is the row's verdict, two rewritten
argument blocks in `ci.yml` and one corrected citation in
`scripts/ci-filter.py`.

**The re-opening was right about the currency and wrong about the
answer**, and the thing that decided it is the one clause the re-opening
explicitly declined to re-open: *"no single test binds any N up to 4."*
It does now.
`editor-core::all r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors`
runs **346-660 s by itself** at ε = 1e-12 — 85-96 % of the leg that
carries it — and that leg is the last job to finish on all 30 runs read,
at every count. Five of the six matrix rows cut cleanly with more shards
(f64 82 s → 54 s, interval ε = default 182 s → 115 s, N=2 → N=4); the
sixth does not respond to the count at all, and it is the one that sets
what a contributor waits for. Cutting a 78 s leg beside a 554 s one is
not a cut.

**Filed with it**: `one-test-is-the-whole-ci-critical-path`, parked on
the M10-3 row. It is the same family as the symbolic-tier regression
already diagnosed there — the chamber replay and the band/uniform drives
are the second and fourth heaviest rows in the same readings — but a
different suite, four times larger, and newly shown to be **ε-gated**:
the row builds its guide at `guide(2.0 - 1.0e-11)`, so it is the whole
critical path at ε = 1e-12 and under 20 s at the other two. Nothing had
looked at it per-ε before.

**Three method notes worth keeping.** The conservation check was done at
the level of test IDs, not counts: every test is named in its leg's log,
so the shards' name SETS were compared directly and came back identical
at every N (`missing 0, extra 0`; 6 948 f64 and 7 669 interval). The
per-leg fixed cost is **15.9 s median over 270 legs** and is flat in the
count — 15 s in August, 15.6 s on 09-03, unmoved. And run WALL is the
weakest of the three instruments here: running four probe runs at once
pushed job queue times from a 2 s median to 108 s, which is visible in
the run walls and absent from the leg walls, because a leg's wall starts
when its runner does.

**Reported, not filed** (`ci.yml` carries several more billed-minute
arguments outside this knob; the orchestrator places them): the block
that decides THIS knob is rewritten on wall clock, per
`work/ciw/plan.md` §The 2026-09-04 re-read, and none of the others was
swept.

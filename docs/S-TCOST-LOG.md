# S-TCOST log — test-suite cost

Narrative record; the plan is `docs/S-TCOST-PLAN.md`. Convention as in
the other programs: seam entries at pipeline seams, unit entries at
merges, the tail is the live state.

## Opening state (2026-09-02)

Opened on Evan's direction (in-chat, 2026-09-02) by a fresh
orchestrator on a remote container. Charter and the three rulings
Evan gave on the orchestrator's questions are in the plan.

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

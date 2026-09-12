---
id: nextest-shard-count-needs-remeasure
kind: issue
title: Determine the right nextest shard count: the N=2 verdict was priced in billed minutes
status: closed
opened: 2026-08-13
closed: 2026-09-12
github: 461
refs: [449, one-test-is-the-whole-ci-critical-path]
---

## From GitHub issue 461

Opened 2026-08-13; 0 comments.

The 2-way `--partition count:N/2` sharding of the `test` and `test-interval` matrices was sized against a much slower suite. **The arithmetic that justified it has changed, and the arithmetic that would justify changing it isn't stable yet.**

## Do not act on the numbers below yet

**There is a parallel effort to speed up the tests themselves. That needs to land before this is measured** — otherwise the shard count gets tuned against a test suite that is about to change underneath it, and we redo the work. This issue exists to hold the analysis, not to be actioned now.

## What was measured (2026-08-12, pre-opt-2)

Across 11 full runs, per-leg:

* **Fixed cost per leg: 15 s** (job wall minus nextest's own reported elapsed, median over 20 legs). Checkout ~3 s, nextest binary from cache ~2 s, artifact download ~3 s, archive extraction 0.7 s. No compile, no rust-cache.
* Modelled 2 → 4 shards: **wall ~17.8 min → ~9.6 min (−46%), billed ~137 → ~146 min (+7%)**. Most of the +7% is per-job minute *rounding*; real added work was only 10 × 15 s = 2.5 min.
* **nextest already saturates both vCPUs** — Σ(per-test time)/wall was 1.97–2.00× on every one of the 20 legs. There is no packing slack; shards are the only lever.

## Two findings that survive regardless of speed work

**1. The imbalance is structural, not luck.** 10 of the 12 slowest tests land in shard 2 — identically, in both lanes, in every run sampled. `--partition count:N/2` is deterministic by list position and reads no timings. Median imbalance:

| matrix row | shard 1 | shard 2 | ratio |
|---|---|---|---|
| interval, eps = default | 509 s | **828 s** | **1.63×** |
| eps = default | 484 s | 753 s | 1.56× |
| eps = 1e-12 | 442 s | 667 s | 1.51× |
| eps = 1e-6 | 476 s | 482 s | 1.01× |

Rebalancing the *existing* 2-way split was worth ~160 s of wall on the critical leg, for free. `--partition hash:N/M` randomises placement but does not balance by weight; neither mode reads timings.

**2. There is a hard floor.** `step-import::all rw2_probes::probe_round_trip_bit_identity_and_reorder` was 296 s — no sharding scheme goes below its longest single test. Modelled N=6 bought only 27 s more wall than N=4 for +48 billed minutes.

## Why #449 changed the premise

opt-level 2 on the archive jobs (#449) took the critical leg from **828 s to 117 s**. Against a 15 s per-leg fixed cost, that constant is now ~13% of a leg instead of ~2%. More shards buy much less and cost proportionally more — the 2→4 case may now argue the *other* way. This is a re-measure, not a re-derive.

## What to do when the speedup work lands

1. Re-measure the per-leg fixed cost (it may also have moved — artifact size changed with opt-2).
2. Re-measure per-test times and the shard-2 imbalance; check whether the same tests still dominate.
3. Only then decide N, and consider whether weight-aware partitioning is worth it versus just picking a better N.

Full write-up and method: `docs/GENERICS-BUILD-COST.md`, and `docs/LOCAL-BUILD-PERF.md` for the machine-variance caveats.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Home

S-TCOST: the issue's own block — "the parallel effort to speed up the tests themselves" — is this program, and CI sharding is named in its `keep_out` as out of scope unless a unit's measurement makes the case in its own PR, so the analysis parks here rather than travelling.

## Re-measured (2026-09-03, S-TCOST, after TCOST-K1–K3 landed)

Read-only census over 20 test legs of the five latest code-tier runs
(33803928081, 33802978677, 33796624357 and their merge-base probes
33804838111, 33800419245; ε default and 1e-12, both lanes). Report and
raw data: `/home/user/tcost-work/shard-remeasure/REPORT.md` on the
orchestrator's box; the numbers of record are in the runs' own cost
reports.

- **Legs are 34–74 s wall** (were 250–430 s). Fixed cost per leg
  **15.6 s median** (unchanged from the 15 s of 2026-08-12), now 25–60 %
  of a leg instead of ~2 %.
- **The hard floor collapsed 296 s → 30 s** (`nurbs_import::arc_loft_
  natively_computes_its_rational_volume` at ε default; ~2.5 s at
  1e-12; next-highest ~6 s). No test binds any N up to 4.
- **The imbalance is no longer structural**: shard2/shard1 ratio
  0.65–2.29 (median ≈1.15) against the old stable 1.51–1.63; the top-5
  of a shard shares only 1–3 tests run to run, because no single test
  is heavy enough to anchor a ranking any more.
- **Saturation Σ(per-test)/wall ≈3.9×** on every leg (was ~2.0×): the
  `ubuntu-latest` runner now presents ~4 vCPUs, or nextest's thread
  count resolves differently — inferred, not read. Still no packing
  slack; shards remain the only lever.
- **Model, not measurement**: the f64 rows sit at 46–63 s on N=2 and
  every added shard costs a full billed minute for a 20–45 % wall cut
  — **stay at N=2**. The two interval rows sit at 70–74 s, both legs
  over the 60 s billing boundary (4 billed min); N=3 models to ~52 s
  legs (3 billed min), N=4 billed-neutral for ~40 % less wall.

**Verdict: closed at N=2.** The only case a re-shard could make is
the interval rows' ≈1 billed minute per row, modelled at a boundary
where a model is wrong in either direction, and paid only on runs
that draw the interval lane; under the program's keep_out that case
would need its own measured PR, and it is not worth one now. Re-open
if the interval legs grow past the boundary by a margin a model does
not need. `hash:` partitioning buys nothing over `count:`; a
weight-aware split is more machinery than the saving.

## RE-OPENED (2026-09-11): the verdict was decided by a currency that no longer exists

The 2026-09-03 re-measure above closed this at N=2, and read in full its
closing paragraph rests on one quantity: **billed minutes.** *"every
added shard costs a full billed minute for a 20-45 % wall cut"* for the
f64 rows; for the interval rows, *"the only case a re-shard could make
is the interval rows' ~1 billed minute per row, modelled at a boundary
where a model is wrong in either direction."* Both halves are a price
in Actions minutes weighed against wall clock.

`evgunter/cad` went public on **2026-09-03** — the same day that
re-measure was taken, and the fact did not reach it. Standard-runner
minutes are free (`scripts/ci-filter.py` §CONFIGURATION COVERAGE;
`work/ciw/f3-recosting-on-a-public-repo`). Every figure on the cost side
of that verdict is now zero, and the benefit side is untouched, so the
verdict inverts on its own numbers:

- **f64 rows**: 46-63 s legs, "a 20-45 % wall cut" per added shard,
  against a cost that was a rounding artefact of per-job minute billing.
- **interval rows**: 70-74 s legs; the re-measure's own model puts N=3
  at ~52 s legs and N=4 at **~40 % less wall**, and N=4 was already
  called *billed-neutral* before minutes went free.

The interval legs sit on the run's critical path — `test (interval,
eps = default, 1/2)` is named as the last job on it in
`scripts/ci-filter.py` §WALL CLOCK IS NOT FREE — so this is a cut to
what a contributor actually waits for, not to aggregate compute.

**What this does NOT re-open.** The two structural findings are
unchanged and still say what they said: the imbalance is no longer
structural (ratio median ~1.15), and no single test binds any N up to 4
now that the hard floor collapsed 296 s -> 30 s. Weight-aware
partitioning is still more machinery than the saving; `hash:` still buys
nothing over `count:`. The work is a measured N, not a new mechanism.

**It still needs its own measured PR**, per the program's `keep_out`
("CI build knobs (profile/cache/sharding) are out unless a unit's
measurement makes the case in its own PR"). That keep-out is about
evidence and is unaffected by the billing change; what changed is that
the case can now be made, where before the arithmetic refused it.
Every N modelled above is a MODEL — the re-measure says so — and the
before/after must come from hosted runs on the 4-vCPU runner.

## MEASURED (2026-09-12): N stays at 2, and the re-opening's own premise is what broke

The re-opening above is right that the closing verdict was priced in a
currency that no longer exists, and right that this needed a hosted
measurement rather than another model. It is wrong about which way the
measurement lands, and the thing that decides it is the one finding the
re-opening explicitly declined to re-open: **"no single test binds any N
up to 4" stopped being true.**

### Method

Four shard counts — the live N=2 as a control, plus N=3, N=4 and N=6 —
each on its own probe branch off the same commit, differing from main in
exactly the shard literal (and, for the control, one comment line so the
diff classifies TIER=all). Three full code-tier runs each, in three
waves, all four counts running in the same wave so a wave's runner
weather is shared. Twelve runs, 270 test legs:

| N | test jobs/run | run ids |
|--:|--:|---|
| 2 | 12 | 34681503322, 34682495317, 34683422606 |
| 3 | 18 | 34681505853, 34682495342, 34683423324 |
| 4 | 24 | 34681508890, 34682494764, 34683422743 |
| 6 | 36 | 34681511479, 34682496147, 34683422386 |

The job count was confirmed on each run (12 / 18 / 24 / 36 — 6N), and
all 270 legs' `run archived tests` STEP concluded `success`; none was
skipped. A second population, for the N=2 spread alone, is the 18
code-tier runs of the same morning on other lanes' branches.

### Test-count conservation, at the level of test IDs

Not a count check — a SET check. Every test the run executed is named in
its leg's log, so the per-shard name sets can be compared directly. At
N=2, 3, 4 and 6, on both matrices:

| row | union of the shards | sum of per-shard `tests run` | listed (`run` + `skipped`) |
|---|--:|--:|--:|
| f64, ε = default | **6 948, identical set at every N** | 6 948 | 6 984 |
| interval, ε = default | **7 669, identical set at every N** | 7 669 | 7 753 |

`missing 0, extra 0` against the N=2 set at every other N; the run-sum
and the listed total are identical at every N on all six rows. The two
name collisions between interval shards are the deliberate re-execution
of `m4_pr8_corpus_interval::every_document_evaluates_green_at_interval`
and `m4_pr6_roundtrip_interval::interval_replay_identity_across_save_load`
by the `band 4 corpus (interval)` and `persistence save/load/replay-
identity (interval)` steps, which ride on shard 1 of the first eps row
at every count. Nothing is dropped by re-sharding.

### Step 1 of the old plan: the per-leg fixed cost has NOT moved

**15.9 s median over 270 legs** (p25 12.6, p75 18.5, p90 20.2, max 47.7)
— against 15 s on 2026-08-12 and 15.6 s on 2026-09-03. It is also flat
in the count (N=2 15.2, N=3 15.7, N=4 16.8, N=6 15.3), so the "more
shards buy less as the fixed cost grows" worry is not what decides this.

### The leg walls, per row, per N — the slowest leg of each row, three runs each

| row | N=2 | N=3 | N=4 | N=6 |
|---|---|---|---|---|
| f64, ε = default | 82 / 83 / 78 | 57 / 61 / 60 | 54 / 59 / 56 | 64 / 55 / 44 |
| f64, ε = 1e-6 | 74 / 68 / 68 | 54 / 54 / 55 | 42 / 48 / 43 | 39 / 41 / 41 |
| f64, ε = 1e-12 | 66 / 64 / 68 | 51 / 47 / 58 | 46 / 49 / 40 | 36 / 40 / 44 |
| interval, ε = default | 191 / 148 / 182 | 153 / 150 / 181 | 115 / 103 / 131 | 102 / 128 / 135 |
| interval, ε = 1e-6 | 170 / 179 / 168 | 220 / 209 / 128 | 127 / 125 / 120 | 111 / 108 / 118 |
| **interval, ε = 1e-12** | **537 / 740 / 554** | **615 / 476 / 480** | **509 / 619 / 689** | **454 / 372 / 660** |

Five of the six rows behave exactly as the re-opening predicted: a real,
clean, monotone cut down to N=4, flattening at N=6 where the fixed cost
is half a leg. **The sixth row does not respond to the count at all**,
and it is the row that decides the run.

### Why it does not: one test

The ε = 1e-12 interval leg is the last job to finish on all 30 runs
read, at every count. Inside it,
`editor-core::all r2_m10_6_probes_interval::a_tolerance_study_end_to_end_through_the_public_doors`
is, on its own:

| N | the row's own duration, three runs | leg that carries it | its share |
|--:|---|---|--:|
| 2 | 458.5 / 659.6 / 470.5 s | 537 / 740 / 554 s | 85 / 89 / 85 % |
| 3 | 582.9 / 441.4 / 441.5 s | 615 / 476 / 480 s | 95 / 93 / 92 % |
| 4 | 458.6 / 576.6 / 634.9 s | 509 / 619 / 689 s | 90 / 93 / 92 % |
| 6 | 434.5 / 346.2 / 632.9 s | 454 / 372 / 660 s | 96 / 93 / 96 % |

A count partition cannot go below its longest single test, and that test
is 85-96 % of the leg. What is left after it — 30 to 80 s — is all any
shard count has to work with, and splitting it further changes the leg
by less than the test's OWN run-to-run spread, which is **346 s to
660 s, a factor of 1.9 on the same tree**. The twelve critical legs, in
count order, are 537 / 740 / 554 · 615 / 476 / 480 · 509 / 619 / 689 ·
454 / 372 / 660; the N=2 population of the same morning (18 other runs)
spans 409-645 s with a median of 534. **Every count's readings sit
inside every other count's spread.** Run walls say the same and say it
more weakly, because four simultaneous probe runs pushed job queue times
from a 2 s median to as much as 108 s: 915 / 1279 / 1029 at N=2,
1251 / 1007 / 1139 at N=3, 916 / 1226 / 1332 at N=4, 1113 / 890 / 1209
at N=6, against 825-1124 for the 18 unperturbed N=2 runs.

### Verdict: N stays at 2, on wall clock and not on a minute

The re-opening's inversion does not survive contact with the runner —
not because its billing argument was wrong (it was right; minutes are
free and every cost figure in the 2026-09-03 verdict is now zero) but
because its benefit argument was measured on a tree where no test bound
the split. On today's tree one does, at the exact row that holds the
critical path, and `20-45 % less wall` per added shard is available
everywhere it cannot be spent. A contributor waits for the maximum over
the legs, not their sum; cutting a 78 s leg to 54 s beside a 554 s one
is not a cut.

Going to N=4 would therefore buy no wall and cost twelve more legs of
fixed time, twelve more runner slots per run (measurably worse queueing
when runs overlap), and a job list a reader has to scan at 24 rows
instead of 12. Both matrices stay at 2; nothing here argues for
different counts on the two, because the f64 rows' improvement is real
and unreachable for the same reason.

**What this measurement could not see.** It is one morning on
`ubuntu-latest` (~4 vCPU) with the suite as of `9b3959849`; it says
nothing about a tree where the floor test is gone. It reads job and step
timings from the Actions API and nextest's own per-test durations, so a
cost inside the runner image (a slow artifact download, a cold page
cache) is inside the "fixed cost" figure rather than attributed. And the
three-runs-per-count design can separate a 2x effect from noise, not a
10 % one — which is enough here only because the effect it had to see is
zero.

**Re-open when** `work/tcost/one-test-is-the-whole-ci-critical-path`
closes — i.e. when the ε = 1e-12 interval leg is no longer floored by a
single test. At that point five of six rows already say N=4, and the
sixth would stop dissenting. Not before: while one row runs for eight
minutes, the count is the wrong knob. What this does NOT re-open is
unchanged from the 2026-09-11 section — `hash:` still buys nothing over
`count:`, and a weight-aware split is still more machinery than the
saving (it could not help here either: no scheme balances a shard that
is one test).

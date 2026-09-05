---
id: tree-wide-guards-outside-the-change-closure
kind: issue
title: Tree-wide guards are unreachable from the change closure: two main breaks in one night, and restoring F3's push test rows would have caught neither
status: open
needs_ev: true
opened: 2026-09-05
refs: [f3-recosting-on-a-public-repo, merge-order-semantic-break-reaches-main, main-latently-red-at-tier-all, 1829, 1859, 1871, 1884]
---

On 2026-09-04 `crates/test-utils/tests/reader_census.rs:538`
(`every_site_that_reads_rust_source_is_in_the_ledger`) reddened `main`
twice in three hours and forty-eight minutes of exposure, each time
repaired by a CIW lane. This item measures both incidents and then says
what the measurement actually implicates, **which is not what it looks
like from the outside**.

**The headline correction, established below and not argued.** The
obvious reading is "a PR was green on the census and `main` was red on
it minutes later". That reading is FALSE. DOCM's PR 1871 ran fully green
— all 37 checks, including the `test (eps = 1e-12, 1/2)` row — and
**the census never executed in that run at all**. It was not built, was
not archived and was not run, because `scripts/ci-filter.py`'s closure
put `test-utils` outside the run's `CARGO_SCOPE`. The guard is tree-wide;
its reachability is not.

The consequence lands squarely on F3 and is the reason this is an `[ev]`
item: **restoring `build` + `test` to `main`'s push run would not have
caught either break.** A push run classifies the merge's own diff, so it
draws the same `CARGO_SCOPE` the PR run drew, and `test-utils` is scoped
out again. Measured below.

## The two incidents

### Incident 1 — `docm1_face_frame.rs`

`crates/editor-core/tests/docm1_face_frame.rs` arrived reading Rust
source without a ledger line in `reader_census.rs`'s `LEDGER`.

| fact | value |
|---|---|
| introduced | PR **1829**, merge `fde85c50`, merged **2026-09-04T18:20:26Z** |
| repaired | PR **1859**, merge `5a1317e5`, merged **2026-09-04T21:21:16Z** |
| **exposure** | **3 h 00 m 50 s (180.8 min)** |
| first red PR run | run **33905591338** created **18:23:04Z** — **2 m 38 s** after the break |

### Incident 2 — `docm5_subject.rs` and `landing_gathers.rs`

`crates/editor-core/tests/docm5_subject.rs` and
`crates/viewer/tests/landing_gathers.rs`, same shape, same guard.

| fact | value |
|---|---|
| introduced | PR **1871**, merge `2a924eb2` (branch head `9f34220e`, DOCM's fix-pass commit `f84280bf4`), merged **2026-09-04T23:15:09Z** |
| repaired | PR **1884**, merge `5d711eea`, merged **2026-09-05T00:02:35Z** |
| **exposure** | **47 m 26 s** |
| first red PR run | run **33929481424** created **23:26:02Z** — **10 m 53 s** after the break |

Total exposure **3 h 48 m 16 s**; the two breaks span 18:20:26Z →
00:02:35Z, 5 h 42 m.

**Detection was fast; repair was not.** 2 m 38 s and 10 m 53 s to the
first red, against 180.8 min and 47.4 min to the repair. Whatever this
costs, it is not a detection-latency cost — which matters, because the
obvious fixes are all detectors.

### The red PR runs, counted from the Actions API

`pull_request` runs of `ci.yml` **created inside each exposure window**,
whose failure is a `test (…, 1/2)` row (the shard that carries the
census — the census is at index 2302/3017 and 2308/3001 of shard 1, and
every red run failed **all six** `1/2` rows across both ε and both
lanes, which is what a deterministic source-text census looks like):

| window | run | created | branch | failed jobs |
|---|---|---|---|---|
| 1 | 33905591338 | 18:23:04Z | `ciw/unsample-klint` | 7 |
| 1 | 33906662373 | 18:34:56Z | `view/ev-tracker-fired-trigger` | 7 |
| 1 | 33906985606 | 18:38:29Z | `ciw/unsample-klint` | 7 |
| 2 | 33929481424 | 23:26:02Z | `ciw/delete-config-trailer` | 7 |
| 2 | 33929608386 | 23:28:08Z | `ciw/issues-readme` | 7 |

**5 red runs, 35 failed jobs, 4 distinct innocent branches.** Each run
failed six `test (…, 1/2)` rows plus `gate ok`. Verified by name in the
logs of job **101134962625** (run 33906662373, window 1) and job
**101206518889** (run 33929608386, window 2), both reading

```
FAIL [ 0.574s] (2308/3001) test-utils::reader_census every_site_that_reads_rust_source_is_in_the_ledger
FAIL [ 0.477s] (2302/3017) test-utils::reader_census every_site_that_reads_rust_source_is_in_the_ledger
```

The other three carry the identical failing-row signature (exactly one
failed test in shard 1/2, six lanes, deterministic) and were not opened
individually; that is stated as an inference, not as a reading.

### It did NOT block every open pull request, and the difference is the finding

The population in window 1 is **27 `pull_request` CI runs, of which 3
went red**; in window 2, **13 runs, of which 2**. The rest were green
and merged: **5 PRs merged during window 1** (1861, 1862, 1860, 1863,
1865) and **3 during window 2** (1881, 1866, 1882).

A green run in those windows is not a run that passed the census. It is
a run that **never built it**. Only a run whose closure contains
`test-utils` runs the census, and only 5 of 40 runs did. The
compensating control F3 named — "the next PR's merge ref" — is therefore
not a control that fires on the next PR. It fires on the next PR *that
happens to draw a closure containing the guard's home crate*, which
tonight was 12 % of them.

## The mechanism, established

`scripts/ci-filter.py:20` defines TIER=closure's `PKGS` as *"the changed
members plus every member that transitively DEPENDS on them,
dev-dependencies INCLUDED"*, and `:739` emits
`CARGO_SCOPE=-p a -p b …` from it. `ci.yml:2557` archives exactly that
scope (`cargo nextest archive ${{ needs.filter.outputs.cargo_scope }}`),
and the `test` matrix only runs what the archive holds.

Run both breaking merges through the real classifier
(`git diff --name-only <merge>^1...<merge>` into `ci-filter.py --files`):

| merge | TIER | CARGO_SCOPE |
|---|---|---|
| `fde85c50` (PR 1829) | closure | `-p editor-core -p mesh -p pncad -p pncad-py -p step-export -p step-import -p stl -p sweep -p topo -p verbs -p viewer` |
| `2a924eb2` (PR 1871) | closure | `-p editor-core -p pncad -p pncad-py -p viewer` |

**`test-utils` is in neither.** The archives confirm it at the runner:
PR 1871's green job **101201002362** logs
`Extracting 8 binaries` and `808 tests run: 808 passed, 809 skipped`,
where the red runs log `Extracting 35 binaries` and `3001`/`3017` tests.
Run **33927923370** is `event: pull_request`, `conclusion: success`,
37/37 jobs green, and the census is simply not in it.

### Why `test-utils` in particular

The closure is over **dependents**, so a crate is selected by a change
to itself or to anything it depends on. Reachability is therefore
`|deps-closure| + 1`, computed from `cargo metadata` over the 18
workspace members:

| guard's home crate | deps-closure | selected by a change to |
|---|---|---|
| **`test-utils`** | **0** | **1 of 18 members** |
| `geom-core` | 1 | 2 of 18 |
| `editor-core` | 13 | 14 of 18 |
| `viewer`, `pncad-py` | 16 | 17 of 18 |

`crates/test-utils/Cargo.toml:10` says why, and says it as a virtue:
*"ZERO dependencies, deliberately. This crate is a LEAF: it sits below
every other crate in the tree"*. That is exactly right for the layering
and exactly wrong for reachability. **The repository's most tree-wide
guard is housed in its least reachable crate**, and nothing in the tree
notices the pairing.

## What `main`'s push run actually is today

Read off `.github/workflows/ci.yml` on `origin/main` (`5d711eea`) rather
than from F3's prose, and confirmed against three real push runs
(33931643072, 33930577432, 33929563356). Sixteen of the twenty-one jobs
carry `github.event_name != 'push'`. What survives:

| job | ci.yml | gate | observed |
|---|---|---|---|
| `change filter` | :391 | no `if` | 21–27 s |
| `CI half parity + gate wiring (every tier)` (`mirror`) | :930 | no `if` | 37–46 s |
| `prime the build cache (default)` | :2184 | `push \|\| workflow_dispatch` | 28–37 s |
| `prime the build cache (interval)` | :2237 | `push \|\| workflow_dispatch` | 29–37 s |
| `render lanes` | :4554 | not dispatch/merge_group **and** `run_k_lint` | 3 + 238 + 82 s, code-tier only |

Plus `work-status.yml` (`.github/workflows/work-status.yml:16`),
which regenerates `STATUS.md`.

**The census is not among them, and no test row is.** F3's prose names
`filter` + `rebuild-latency` + `renders`; the tree has since replaced
`rebuild-latency` (demoted to `nightly.yml`) with the two `cache-prime`
jobs. The list above is the current one.

**And `cache-prime` does not build.** `ci.yml:2230` gates its only
compile step on `steps.cache.outputs.cache-hit != 'true'`, so on all but
a key rotation a push run compiles **nothing at all**. Whatever is added
to the push side is being added to a run that today has no Rust build in
it.

## The candidate fixes, priced

Measured in this worktree on a 4-core box, i.e. the same shape as the
hosted runner (`483212ef`: 4 vCPU / 16 GB), with `CARGO_TARGET_DIR`
outside the tree and deleted after.

### The narrow guard row — **it forces no build**

`test-utils` has one test target, `reader_census`, and zero
dependencies. From a **deleted** target directory:

| measurement | value |
|---|---|
| `cargo test -p test-utils --test reader_census --no-run`, empty target dir | **2.69 s compile / 2.80 s wall** |
| cold end-to-end, build + the census test alone | **4.86 s** |
| the test itself, warm | **3.65 s** (all 5 rows in the file: 3.6–3.9 s) |
| the same census inside CI's prebuilt archive | **0.477 s / 0.574 s** (jobs 101206518889, 101134962625) |

**This is the answer to the question the brief expected to be bad news,
and it is the opposite: adding this guard to a `main` push run does NOT
mean compiling the workspace where today it does not.** `test-utils`
depends on nothing, so the row is a checkout, a toolchain and ~7 s of
cargo. Against the observed fixed cost of a cheap push job (21–46 s), it
is **~40 s wall, one billed minute, and zero on a public repo**. It does
not touch the critical path, which on a code-tier push is `render lanes`
at 238 s, and at ~40 s it sits at the length where M3's cancellation
rate is 15 %, not 57 %.

**The same row is even cheaper on the PR side, and that is where it
would actually have helped**: it would have red PR 1829 and PR 1871 on
their own runs, before either merged, with the red landing on the author
who caused it. Neither incident would have existed.

### Restoring F3's push test rows — **measured ineffective for this class**

`ci.yml:582` sets the push run's base to `HEAD^1` and diffs
`HEAD^1...HEAD`, which for a merge commit is the PR's own diff. So a
restored `build` + `test` on push draws the `CARGO_SCOPE` in the table
above — `test-utils` scoped out both times — and reds nothing.

Its cost is the one `f3-recosting-on-a-public-repo` M2 measured:
`build + archive` at 336 s / 388 s median, a 442 s run, and M3's 57 %
cancellation at that length. **Expensive and, for this defect,
ineffective.** Only `--force-all` on every push would reach it, which is
the scheduled-full-run shape Ev declined on 2026-08-22.

### The other guards with the same shape

Not one guard — at least **five rows across four files**, all walking
the tree from the repository root or across `crates/*/src`, each
reachable only through its own home crate's closure:

| guard | home | reachability |
|---|---|---|
| `crates/test-utils/tests/reader_census.rs:538` `every_site_that_reads_rust_source_is_in_the_ledger` | `test-utils` | **1/18** |
| `crates/test-utils/tests/reader_census.rs:566` `every_shared_entry_actually_reaches_the_shared_lexer` | `test-utils` | **1/18** |
| `crates/geom-core/tests/bounds_census.rs:529` `every_sole_bracket_bound_door_is_in_the_roster` | `geom-core` | **2/18** |
| `crates/geom-core/tests/flagged_census.rs:251,264` (`shipped_sites()` at :233 walks `crates/*/src`) | `geom-core` | **2/18** |
| `crates/editor-core/tests/fix_loop_polygon_expr.rs:140` `the_polygon_close_is_written_once_in_shipped_src` | `editor-core` | 14/18 |
| `crates/pncad-py/src/prose_census.rs:183` `scanned_files` (whole-repo `src/**`) | `pncad-py` | 17/18, but separately gated on `RUN_PNCAD_PY` |

**A fix aimed only at `reader_census` leaves `geom-core`'s two at 2/18.**
The `geom-core` rows are not free the way `test-utils`'s are: its tests
are one mounted binary (`--test all`), so reaching them costs a cold
build of `geom-core` + deps — measured **17.79 s** from an empty target
dir, with the two `bounds_census` rows then running in **0.94 s**.
Cheap, but no longer negligible, and `editor-core`'s binary is much
larger. A "tree-shaped guards" row that covers `test-utils` and
`geom-core` is ~25 s of cargo; one that covers `editor-core` too is a
real build.

## What is being decided

`work/ciw/program.md`'s `keep_out`: *"what a main push re-gates is an
[ev] ruling before any change to the F3 trim"*. **Nothing about F3 is
changed by this item.** The question for Ev is which of these to take:

**(a) Add a test row to `main`'s push run.** Measured above: it does not
work for this class, because the push run re-draws the merge's own
closure. The variant that does work is `--force-all` on every push —
the scheduled full run declined 2026-08-22, at 442 s and a 57 %
cancellation rate.

**(b) Leave F3 as it stands and accept the fleet as the detector.**
Tonight's measured price: 3 h 48 m of exposure, 5 red PR runs, 35 failed
jobs, 4 innocent branches, two CIW lanes consumed. Detection cost 2–11
minutes; the rest was repair. This is a real option — the price is small
and the detector, though only 12 % efficient tonight, was fast twice.

**(c) A narrow unscoped guard row that runs the tree-shaped guards
without a workspace build.** For `test-utils` this is measured at ~7 s of
cargo and ~40 s of job, on the push side, the PR side, or both — and on
the PR side it prevents the class rather than detecting it. It does not
touch F3, does not touch the archive, and does not touch the tier.
Extending it past `test-utils` costs a real build per crate added, so a
row scoped to the two leaf-housed guards (`test-utils`, `geom-core`) is
the natural stopping point at ~25 s of cargo.

There is a fourth shape — **make the closure reach guards that are
tree-wide**, by pinning `test-utils` into every closure or by rehoming
the guards. That is `scripts/ci-filter.py`, which this program's
`keep_out` assigns to S-TCOST, so it is named and not proposed.

**CIW's reading**, offered as a reading and not as a decision: (c) on
the **pull-request** side is the only option that puts the red on the
author who caused it, and the only one whose cost was measured at
seconds rather than minutes. (a) is measured not to work. But the
decision is Ev's, and (b) is defensible on tonight's numbers.

## What is not established here

- The failing test is verified **by name in 2 of the 5** red runs; the
  other three are inferred from an identical failing-row signature.
- Two incidents in one night are **two samples, not a rate**. Nothing
  here says how often a tree-wide guard's home crate falls outside a
  closure that would have caught the arrival.
- The ~40 s figure for a hosted guard row is **composed** from this
  repo's observed cheap-push-job overhead (21–46 s) plus a locally
  measured 7 s of cargo. No such job has been run on a runner.
- Whether the `geom-core` and `editor-core` guards have ever actually
  missed an arrival is **not measured** — only that their reachability
  admits it.

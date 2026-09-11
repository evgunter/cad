---
id: fuzz-depth-not-existence-run-everything-at-effort-1
kind: unit
title: Run every fuzzer at EFFORT=1 on every run; the marker selects DEPTH — the measurement, and the wiring it implies
status: open
opened: 2026-09-11
needs_ev: true
---


Opened 2026-09-11 on Ev's proposal, in chat: *"everything is run at
EFFORT=1 always (and what EFFORT=1 quantitatively is is set to be a
rounding error time wise, perhaps literally by having a timeout?), stuff
that otherwise would've passed the gate gets a higher EFFORT run"* —
with the observation, the same message, that *"compilation time is a
factor here"*. `needs_ev`: the clause rewrite rides the `[ev]` PR that
carries this file, and nothing below is wired until Ev signs it off.

## Half of it already exists, and the other half exists nowhere

Read before proposing, so the change is smaller than it sounds:

- **`fuzz::effort()` already defaults to 1** (`crates/test-utils/src/fuzz.rs:207`),
  and the harness's own doc already calls the shipped counts *"a smoke
  level: enough to catch a gross regression in the seconds a gated CI job
  should cost, and no more"*. `scaled(n) = n * effort()`, never below `n`.
  So "run at EFFORT=1" is not a new mode — it is what every kernel fuzz
  row already does whenever it runs at all.
- **Nothing in the kernel ever runs above EFFORT = 1.** The only
  `CAD_FUZZ_EFFORT` in CI is `"8"`, on `ci.yml:3935`, and that is the
  `interval-transcendentals` oracle job — a different workspace. The
  nightly's ungated re-take runs the gated set at EFFORT = 1 too, so it
  buys BREADTH and not depth. The harness doc says *"set
  `CAD_FUZZ_EFFORT=100` to restore roughly the depth these sweeps had
  before 2026-08-13, or higher to actually hunt"* — and no lane in this
  repository does.

So the proposal is two edits, not a redesign: **stop the gate deciding
existence**, and **start something deciding depth** — which today is
decided by nothing.

## THE MEASUREMENT (hosted, and it was already being taken)

The `gated suites (ungated re-take)` job in `nightly.yml` runs the WHOLE
gated set, ungated, at EFFORT = 1, every night main moved. That is
exactly the population this proposal would add to every PR run, under
exactly the dial it would run at. Its own header says the reading was
owed and never taken — *"THE READING WILL COME FROM THIS JOB'S OWN
DURATION on the row's first firing... Whoever reads that run should
replace this paragraph with what it cost."* Read now, two nights:

| night | run | `Summary` wall | tests | slowest single test | 2nd |
|---|---|--:|--:|--:|--:|
| 2026-09-11 | 34585964715 | **83.599 s** | 419 | 83.310 s | 23.490 s |
| 2026-09-08 | 34211844759 | **66.359 s** | 419 | 65.896 s | 18.102 s |

**The gated set's entire execution wall IS one test.** 83.599 against
83.310; 66.359 against 65.896. The other 418 tests finish inside its
shadow and contribute nothing to the wall — most of the visible PASS
lines read `0.007s`. Both nights name the same row:

`editor-core::all m10_3_r1_probes_interval::the_driven_chamber_replays_bit_identically_names_both_wall_flips_and_reports_containment`

This is `memories/test-suite-cost.md`'s *"cost concentrates savagely"*
in its purest form, and it makes the proposal cheap to price:

- **Everything except that row is free.** 418 tests at EFFORT = 1, spread
  over 12 test legs (2 shards x 3 eps x 2 lanes), against legs that are
  34-74 s wall today. Their summed contribution is under a second per leg.
- **That row alone is the whole cost**, and it does not divide: it lands
  in one shard of one leg and roughly doubles it, 66-83 s onto 34-74 s.
  The leg it lands in becomes the run's critical path.

The job totals (456-625 s over five nights, of which 424-575 s is the one
step) are **build-dominated and are not this number** — that step compiles
`--workspace --features interval` before running anything. The `Summary`
line is the execution, and it is the figure this proposal turns on.

## What the numbers therefore say

**Take the proposal, and the price is one row.** The wall a skip was
buying back is, for 418 of 419 tests, indistinguishable from zero — so
for them the gate is trading a silent failure mode against nothing. For
the 419th it is trading it against 66-83 s, which is real, and which is a
question about THAT ROW rather than about the policy.

**Compilation is untouched either way**, which is Ev's point and it is
decisive about magnitude: the test binaries are compiled into the archive
whether or not they execute (`build + archive (interval)`, 388 s median,
the run's longest job), so neither policy moves the pole. Everything
being argued about here lives in the cheap half of the run.

## Wiring, if Ev signs the clause off

1. **`ci-filter.py`: invert what `TEST_FILTER` means.** Today the gate
   emits `not (A | B | ...)` excluding untouched gated suites. Under the
   ruling it emits nothing for the ordinary legs — every suite runs — and
   the same derived set becomes the SELECTION for a deep leg.
2. **A deep leg, or a deep step.** The selected suites re-run with
   `CAD_FUZZ_EFFORT` above 1. Cheapest shape is a step in the existing
   test job rather than a new job: the archive is already there, and a
   new job pays the per-leg 15.6 s fixed cost for nothing. The EFFORT
   VALUE is a measurement, not a guess — the oracle job's note
   (`ci.yml:3922-3931`) is the precedent: *"the cases are ~7s at effort 1
   ... and scale linearly — 8.2x measured locally from effort 1 to 8"*.
3. **The nightly's re-take changes meaning**, and its header must say so:
   it stops being "the coverage a PR skipped" and becomes "the depth a PR
   did not buy", so it should run at the higher EFFORT rather than at 1.
4. **`fuzz.rs`'s doc has one sentence to fix** — *"in the seconds a
   gated CI job should cost"* assumes the gate decides existence.
5. **The gate's three guards stay and get cheaper to be wrong about**
   (`--gated-check`'s marker resolution, the helper-import arm and the
   `#[path]`-mount arm, both landed 2026-09-11). A broken marker under
   the ruling costs depth, not existence — which is the argument for the
   ruling, not an argument for deleting the guards.

## What this closes and what it does not

- **`proptest-modules-in-src-ungated` closes with it.** That row exists
  because the 14 in-src modules mix deterministic pins with property rows,
  so a file-level marker would gate the pins too. Under the ruling nothing
  is gated for existence, so there is nothing to split: the pins run, the
  property rows run shallow, and its 0.62 cpu-s table stops needing a
  threshold written down. It is the strongest evidence the design is
  right — an open question stops existing rather than getting an answer.
- **Shape-3 rows are untouched** and the clause says so: where the count
  IS the coverage claim, EFFORT must not scale it below its floor.
- **`r1-probe-seeds-are-not-on-the-fuzz-dial` stays open and gets MORE
  load-bearing**: two rows seed from the clock under a private `R1_SEED`
  that `CAD_FUZZ_SEED` cannot pin. Under a policy where every fuzzer runs
  on every PR, an unreproducible red on an unrelated branch is the exact
  cost the policy accepts — and it is only acceptable while every red IS
  reproducible. That row is the one place it is not.
- **The attribution cost is real and is accepted, not waved away.** A
  find at EFFORT = 1 lands on whoever drew the unlucky seed rather than on
  the PR that touched the code. Ev's ruling of 2026-09-11 on
  `consider-proptest-for-randomized-sweeps` is the evidence it is small:
  finds are rare and diagnosis has not been burdensome.

## The one place the premise breaks: a separate cargo root

Ev asked (in chat, 2026-09-11) whether the measurement includes *"the
fuzzer that only runs on changing interval-transcendentals"*. **It does
not**, and the reason is the carve-out the clause now carries.

`interval-transcendentals/` is its own cargo root, excluded from the
workspace, so the nightly's `--workspace --features interval` re-take
never reaches it and it is not among the 419. It is gated at the JOB
level by the change filter's `ORACLE_PATHS` — four paths under that root
(`ci.yml`'s `interval backend crate` note) — and not by a `gated_to!`
marker at all. Two different mechanisms that read alike from a distance.

It is also where this proposal's arithmetic stops working. The whole
reason "run it anyway at EFFORT = 1" is nearly free for a kernel row is
that `build + archive` compiles the row into the archive whether or not
it executes. That root is not in the archive: `ci.yml:3922-3931` records
**~234 s of build to buy ~7 s of cases at EFFORT = 1**. Running it on
every pull request would spend the expensive half of a run — compile —
to buy the cheap half.

So the job-level gate there stands, and the depth argument applies to the
LANE instead of to the row. Which that lane already does: it is the only
thing in this repository that runs above EFFORT = 1
(`CAD_FUZZ_EFFORT: "8"`, `ci.yml:3935`), on exactly the reasoning this
proposal generalises — *"Depth is nearly free on this lane and that is
the whole reason to spend it... a lane that fires twice a year with a
pinned seed re-certifies one fixed sample forever, however deep it is.
Fresh millions each firing is worth more than the same millions
faster."* It is the precedent, not the exception.

## Depth keys on the NAMED PATHS, not the closure (Ev, 2026-09-11)

Ev, in chat: *"the fuzzers should run for longer if the PR touches the
files they're actually about, rather than only including them by
closure"*. This is charter lever 3 of this program, restated for the
depth side — and the good news is that **the mechanism already works
this way and needs nothing built.**

`GatedSuite.selected_by` (`scripts/ci-filter.py`) compares the marker's
declared paths against the DIFF's own file list: equality for a file,
prefix for a directory written with a trailing `/`, plus the suite's own
file implicitly and — since 2026-09-11 — its sibling helper directory.
Nothing in that reads the crate closure. The closure decides which
crates' tests are BUILT (`CARGO_SCOPE`); the marker decides which suites
are selected, and those are different questions. So "run deeper when the
PR touches the files it is about" is `selected_by` unchanged, pointed at
a different consequence.

**What does have to change is the fail-open direction, and it inverts.**
Ratified by Ev in chat, 2026-09-11 (*"depth fails closed is good"*), so
the clause states it as a rule rather than deriving it here.
Today `gated_filter` fails open — emits no filter, so everything runs —
on tier `all`, tier `docs`, an unreadable diff, a change to the
derivation's own inputs, and any exception. That is right for EXISTENCE:
failing open means running MORE, which is the safe direction.

For DEPTH it is the wrong direction twice over. Tier `all` fires on most
merges (`memories/test-suite-cost.md`: *"demos/, .github/ and scripts/
dominate"*), so a depth selection that failed open would buy the deep run
on most merges — turning the exception into the rule and spending the
thing the dial exists to ration. And it is not needed for safety: under
this ruling a run that resolves nothing still executes every sweep at
EFFORT = 1, so failing CLOSED on depth costs depth and never existence,
which is the whole shape of the ruling.

So: **depth is selected by the marker's named paths against the diff,
with no tier fail-open at all.** An unresolvable marker, an unreadable
diff and a tier-`all` run all get the smoke level and no deep leg — and
the deep run is then something a lane asks for deliberately, which is
what the nightly's retirement leaves room for.

**One live instance of the residue, found by checking rather than
assumed away.** A marker may name a path in a crate its own crate does
not depend on. The change filter's closure follows dev-dependency edges
UPWARD — a changed crate pulls in its DEPENDENTS — so such a crate's
tests are never built on that diff, and the marker's promise cannot be
kept in either direction: the suite does not run today, and could not
run deep tomorrow, on exactly the change it names.

Swept at `486557f5` over all 56 markers, resolving each named path's
crate against its home crate's dependency set from `cargo metadata`.
**Exactly one:**

`crates/sweep/tests/tcost_k3_certificate.rs` names
`crates/step-import/src/lib.rs`, and the dependency runs the other way
(`crates/step-import/Cargo.toml:56` takes `sweep`), so a change to that
file never builds `sweep`'s tests.

**It is harmless today, and the reason is worth keeping**: TCOST-K3
wrote TWO suites, one per crate, and the sibling
`crates/step-import/tests/tcost_k3_import_certificate.rs` names the same
path from inside `step-import`, which IS in the closure on that diff. So
the import-path claim is covered — by the other suite, not by this
marker's entry, which is inert.

The fix if it ever bites is to widen the run's scope for that suite, not
to key depth on the closure. Recorded here rather than filed because it
is one inert entry with a working sibling; if a second appears without
one, it is a row.

## Retiring the nightly re-take (Ev, 2026-09-11)

Ev, in chat: *"can we get rid of the nightly job in favor of just
turning the number of runs on that one test way down"*. **The first half
is right and the argument for it is stronger than cost. The second half
is the one operation that row was written to refuse.**

### The job should go, and Ev's own ruling says so louder than the seconds

`nightly.yml`'s `gated suites (ungated re-take)` has exactly one purpose
and says so: *"what the gate gives up is LATENCY, and this row is the
bound on it: at most a day, and only for a break reached by a path the
marker did not name."* Under this ruling nothing is skipped, so there is
no latency to bound and the job has no second job to fall back on.

Three things make retiring it better than merely harmless:

1. **The PR gate strictly dominates it.** The job is ONE LANE — it runs
   `--features interval` only, and admits the gap: *"What this does NOT
   re-take is a gated suite that breaks only in the DEFAULT compile
   mode."* The PR legs are both lanes x three ε rows. Moving the
   population to the PR gate upgrades its configuration coverage from
   one point to six.
2. **A detector nobody reads is not a control** — Ev's ruling of
   2026-09-07 on `work/ciw/f3-recosting-on-a-public-repo`, and this job
   is a live instance of it. Its own header asked for a reading from its
   first firing (*"Whoever reads that run should replace this paragraph
   with what it cost"*); the first reading was taken on 2026-09-11, by an
   orchestrator going deliberately to the jobs API, and it found a row
   marked `SLOW` at over 60 s that two nights had reported green and
   unread. `work/ciw/nightly-demotions-have-never-run` found the same
   shape one lane over.
3. **It removes a whole `--workspace --features interval` build a day**
   (424-575 s of its 456-625 s), which is the job's real cost and buys
   only what the PR gate would then already have.

### But the count is not the knob

`CHAMBER_LEAVES = 1280` is not a dial with slack in it:

- **1024 is a MEASURED THRESHOLD**, not a preference — the first budget
  at which every boundary-touching box refines onto a flip rather than
  refusing for `Budget`. Below it the row stops asserting what it says.
- **The margin is already minimal and deliberate**: pinned at 1024 the
  row *"would go red on any kernel change that costs the drive one box
  of refinement, for a reason that has nothing to do with what the row
  asserts."*
- **The cut has already been taken** — TCOST-6 moved it 4096 -> 1280.
- **The row reds if you cut it further, by construction**: an
  anti-vacuity floor on `widest_frontier` at 64 exists so that *"a
  budget cut that dropped the frontier under it reds here rather than
  quietly turning the D9 comparison into two sequential runs."*

Cutting it is the operation the file was written to refuse, and doing it
anyway would trade a silent skip for a silently weaker assertion — the
same failure shape this whole ruling is trying to remove.

### So the sequencing, and it is short

`m10-3-chamber-row-reads-ten-times-its-recorded-cost` is the blocker, and
it got bigger when this was checked: **both** expensive rows in that file
read 10-16x their recorded figures (6.7 cpu-s against 65.9/83.3 s; an
in-file measured 1.46 s against 18.1/23.5 s). Two rows, two independent
sources, one factor — a common cause rather than two coincidences.

1. **Diagnose that first.** If it is a regression, both rows return to
   their recorded costs, the whole gated set runs on every PR for about
   a second of leg time, and the nightly job is deleted with nothing to
   weigh. **This is the likely case and it makes everything else free.**
2. **If the figures are honest**, the recorded ones are wrong and those
   two rows need re-justifying at their real cost — which is its own
   piece of work, not a budget cut — and until then the job stays or the
   two rows are placed deliberately.

Either way the nightly job's retirement rides the diagnosis and not this
clause, so it is named here as the consequence and left unwired.

## Not taken: the timeout

Ev's message floats *"perhaps literally by having a timeout?"*. The
clause refuses it and says why: a time-based cutoff makes what the test
explored depend on the machine, so it differs per leg, cannot be
reproduced from the logged seed, and manufactures apparent ε-sensitivity
— the last hazard in the memory's own closing bullet. A wall-clock ceiling
OVER the whole EFFORT = 1 population is a good tripwire and is proposed
as one; it is not the dial.

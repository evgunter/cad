---
id: m10-3-chamber-row-reads-ten-times-its-recorded-cost
kind: issue
title: DIAGNOSED: the M10-3 interval suite is 15x slower since M10-7's symbolic normal form (PR 1725) — a KERNEL regression its own gate hid
status: parked
opened: 2026-09-11
blocked_on: [symbolic-tier-costs-95-percent-of-the-m10-3-drive]
---


Found 2026-09-11 while reading the nightly's `gated suites (ungated
re-take)` job for
`fuzz-depth-not-existence-run-everything-at-effort-1`. Filed as a
disagreement between two numbers, not as a diagnosis: this row has not
been re-measured against TCOST-6's method and the two readings may not
be measuring the same thing.

## The two numbers

`editor-core::all m10_3_r1_probes_interval::the_driven_chamber_replays_bit_identically_names_both_wall_flips_and_reports_containment`
is the slowest test in the gated set on both nights read, and by two
orders of magnitude over the next-but-one:

| night | run | this row | next row | rest of the 419 |
|---|---|--:|--:|--:|
| 2026-09-11 | 34585964715 | **83.310 s** (`SLOW`, > 60 s) | 23.490 s | ~0.007 s each |
| 2026-09-08 | 34211844759 | **65.896 s** (`SLOW`, > 60 s) | 18.102 s | ~0.007 s each |

Against it, TCOST-6's merge entry (`work/tcost/log.md`, 2026-09-03):
*"editor-core's three chamber heads (39.3 cpu-s on the interval lane)
are one labelled row at 6.7 cpu-s hosted"*, with the leaf budget retuned
`4096 -> CHAMBER_LEAVES = 1280` on a measured ε-invariant threshold.

**6.7 against 66-83 is a factor of ten to twelve**, and the second row in
the table (18-23 s, `the_band_and_uniform_drives_ship_the_same_leaf_partition`)
is the other member of the same suite, also large.

## What could make both readings honest

Listed so whoever takes this checks rather than assumes — the point of
filing it is that nobody knows which of these it is:

- **Different ε.** TCOST-6's reading is labelled "on the interval lane"
  without an ε row; the nightly job sets no `CAD_TOLERANCE_EPS`, so it
  runs the compiled default. If the row's cost is ε-sensitive, the two
  are different measurements and neither is wrong.
- **cpu-s against wall.** The nightly's figure is nextest's per-test
  wall. For a single-threaded test these are close, but the row's name
  says it replays a chamber, and if it parallelises internally the two
  diverge. `memories/test-suite-cost.md` warns that per-leg timings are
  not comparable without normalising.
- **A regression since 2026-09-03.** `CHAMBER_LEAVES` or the drive it
  feeds may have moved under the row. This is the case that would matter
  most and is the cheapest to rule out: the constant is a literal.
- **A different runner.** Both nights are the public 4-vCPU runner; the
  TCOST-6 reading predates nothing relevant, so this is the least likely.

## Why it matters beyond bookkeeping

Whatever the cause, on today's tree **this row alone is the whole
execution wall of the gated set** — 83.599 s of `Summary` against 83.310 s
for the row, the other 418 tests finishing inside its shadow. It is
therefore the entire price of
`fuzz-depth-not-existence-run-everything-at-effort-1`, and the single
biggest lever left on what the gated population costs anybody. If it is a
regression, fixing it makes that proposal free; if it is honest, the
proposal has one row to place deliberately rather than a policy problem.

It is also an argument for what the nightly job's own header already
asks for and has never had: someone reading that job's output. Two nights
of a row marked `SLOW` at over 60 s went unread until this week.


## Widened (2026-09-11): it is BOTH expensive rows, and the file's own
## in-file figures disagree too

Ev asked whether the nightly re-take could be dropped in favour of
"turning the number of runs on that one test way down". Checking what
knob that row actually has turned the finding from one row into a class,
and the class is what decides the answer.

**The second row disagrees by the same factor.**
`the_band_and_uniform_drives_ship_the_same_leaf_partition` is the
18.102 s / 23.490 s row in the table above. Its budget's own doc comment
(`crates/editor-core/tests/m10_3_r1_probes_interval.rs:358-366`) records
a MEASURED cost for it: *"the pair of band/uniform drives costs 1.46 s
here against 0.98 s at 1024, and the escalation row 0.45 s against
0.21 s."*

So the file carries two independent in-file figures and both are
10-16x under the hosted reading:

| row | recorded | hosted (two nights) | factor |
|---|--:|--:|--:|
| `the_driven_chamber_replays_bit_identically_…` | 6.7 cpu-s (TCOST-6 merge entry) | 65.9 / 83.3 s | ~10-12x |
| `the_band_and_uniform_drives_ship_the_same_leaf_partition` | 1.46 s (in-file, `:364`) | 18.1 / 23.5 s | ~12-16x |

Two rows, two sources, one factor. **That is a common cause, not two
coincidences**, and it makes the "different ε" and "cpu-s vs wall"
hypotheses above weaker than they looked: a 6.7-cpu-s row reading 83 s
of WALL would have to be blocked rather than busy, and both rows ran at
the END of the nightly's list (418/419 and 419/419) with nothing left to
contend with. Whoever takes this should start from "what is common to
this file" rather than from either row.

## And the budget is not the knob Ev's question assumes

Stated here because it is the reason the nightly cannot simply be
deleted in favour of a smaller count:

- **`CHAMBER_LEAVES = 1280`, and 1024 is a MEASURED THRESHOLD** — the
  file says so at `:340-354`: *"1024 is the exact threshold: it is the
  first budget at which every boundary-touching box has been refined
  onto a flip rather than refused for `Budget`"*. Below it the row stops
  asserting what it asserts.
- **The 1280 margin is deliberate and already minimal**: *"a row pinned
  there would go red on any kernel change that costs the drive one box
  of refinement, for a reason that has nothing to do with what the row
  asserts."*
- **The cut was already taken.** TCOST-6 moved this row 4096 -> 1280,
  about a third of what it used to pay, and documented why not 1024.
- **The row reds if the budget is cut too far, by design.** `:480-495`
  carries an anti-vacuity floor on `widest_frontier` at 64, *"the point
  below which a `par_iter` over the frontier stops being a schedule at
  all"*, precisely so that *"a budget cut that dropped the frontier
  under it reds here rather than quietly turning the D9 comparison into
  two sequential runs."*
- **`FULL_PARTITION_LEAVES = 4096` is not cuttable on its own terms**:
  for its two rows *"more leaves is more of the claim rather than more
  of the same claim, so the budget is not cut."*

So turning these counts down is the operation this file was written to
refuse. If the 10-16x is a regression, fixing it returns both rows to
their recorded figures and no cut is needed at all; if it is honest,
then the recorded figures are wrong and the rows need re-justifying at
their real cost — which is a different piece of work from a budget cut,
and a bigger one.

## DIAGNOSED (2026-09-11): a real regression, and the gate is why nobody saw it

Ev asked for the diagnosis. It is a regression, it is attributable, and
the recorded figures were never wrong.

### The measurement

Same box, same profile (`test` / opt-0), same command
(`cargo nextest run -p editor-core --features interval --test all -E
'test(/m10_3_r1_probes_interval/)'`), two trees:

| row | at TCOST-6 (`a4439fbef`, 09-03) | at `origin/main` (09-11) | factor |
|---|--:|--:|--:|
| `the_driven_chamber_replays_bit_identically_…` | 21.016 s | 319.280 s | **15.2x** |
| `the_band_and_uniform_drives_ship_the_same_leaf_partition` | 9.869 s | 88.936 s | **9.0x** |
| `a_wrapped_escalation_never_certifies_inside_the_band` | 2.972 s | 18.612 s | **6.3x** |
| whole suite | **21.041 s** | **319.373 s** | **15.2x** |

**The recorded figures were RIGHT.** TCOST-6 recorded 6.7 cpu-s hosted
for the chamber row. Today's local:hosted ratio, measured on the same
row, is 319.3 / 83.3 = 3.8x (opt-0 against the archive's optimised
build). Applying it to the pre-change 21.0 s gives **~5.5 s hosted**,
which is TCOST-6's 6.7 cpu-s within noise. Nothing was mis-measured and
no hypothesis in the section above survives: not ε, not cpu-s-vs-wall,
not the runner. The row changed under its own figures.

### The cause, by bisect

`git bisect` over `a4439fbef..origin/main` (2 512 revisions, 11 steps) on
the cheapest discriminating row (`a_wrapped_escalation`, 2.97 s good /
18.6 s bad, cut at 8 s) lands on two adjacent commits, both in
**PR #1725, `m10/m10-7-symbolic`**:

- `bd64a9cdc` *"geom-core: the symbolic identity tier (Sym\<T\>, the
  normal form, the session)"* — **3.030 s, good**
- `ddbbd5a24` *"sym: the normal form is a quotient of polynomials…"* —
  does not build standalone, skipped
- `1979777f4` *"driver: **replay at Sym\<Interval\>**, the dials and the
  receipt; re-cut the M10-3 limit rows"* — **21.313 s, bad**

### KERNEL, not test — and the obvious mechanism is refuted

Ev asked which side it is on. **The kernel.** Neither candidate commit
touches `m10_3_r1_probes_interval.rs`, or any assertion, fixture or
budget in it:

- `1979777f4` touches `crates/editor-core/src/drive.rs` (1 line),
  `tests/m10_3_driver_interval.rs` (a DIFFERENT suite) and a new
  `tests/m10_7_probe_interval.rs`.
- `ddbbd5a24` touches `geom-core/src/sym.rs` (+290), `editor-core/src/`
  `drive.rs` (+245), `analysis.rs`, `measure.rs`, `eval/memo.rs`, plus
  `geom-brep` and `topo` — and `tests/all.rs` only to register a suite.

So the shipped program pays this, not only CI. That is the
"kernel-logic unit" class in this program's own charter: *where the
census shows a test is slow because the CODE it exercises is slow in a
way the real program pays too, the fix is a kernel change.*

**The one-line hypothesis is REFUTED, by experiment.** `1979777f4`'s
only kernel change is a shipped dial:

```
-pub const DEFAULT_SYM_MAX_DEGREE: u32 = 16;
+pub const DEFAULT_SYM_MAX_DEGREE: u32 = 128;
```

An 8x raise on a default reads like the whole answer. It is not. Setting
it back to 16 on today's `origin/main` and re-running the suite gives
**467.7 s against 319.4 s at 128** — the constant does not explain the
regression and the tighter budget is *worse*, presumably because a
degree refusal sends work back to subdivision. Recorded because it is
the trap: the diff that looks like the cause here is not, and a fix
written from reading it would have made the row slower.

**By elimination, the cause is `ddbbd5a24`** — *"the normal form is a
quotient of polynomials, so a normalized-and-remetered carrier
cancels"*, a rework of the symbolic normal form in `geom-core/src/sym.rs`
and of the driver that consumes it. `git bisect` could not test it
directly (it does not build standalone, which is why it was skipped),
so this is elimination within a two-commit window rather than a direct
measurement, and it assumes the degree constant's effect is no different
on the 09-03 tree than on today's. Whoever takes the fix should confirm
it against that commit's own build before acting.

**Caveat, stated rather than papered over**: the bisect ran on the
escalation row alone, so `1979777f4` is established as the cause of THAT
row's 7x. The chamber row's 15.2x is measured at the endpoints only and
may have later contributions; anyone re-cutting the budget should
re-measure it rather than assume one cause.

### Nothing records that this cost anything

- The test file still carries TCOST-6's measured prose beside the
  constants — *"the pair of band/uniform drives costs 1.46 s here
  against 0.98 s at 1024"* — with no mention of `Sym` anywhere in it.
  Those sentences are now wrong by 10-60x and read as current.
- `work/m10/` records the symbolic tier's API costs
  (`sym-registration-flattens-two-axes`, the `COEFF_BITS` trade) and no
  runtime cost for the replay at all.
- `CHAMBER_LEAVES = 1280` was chosen as a *measured* margin over a
  *measured* threshold of 1024. Both measurements were taken against the
  old arithmetic. The margin may still hold — the threshold is about
  refinement, not speed — but the cost half of that trade is void.

### The gate is why it sat for eight days

This is the part worth carrying beyond the row. The suite is
`gated_to!` editor-core's driver/analysis/distribution/resolve/tolerance
modules. **PR #1725 changed `geom-core`**, which is not in that set — so
the gate skipped the suite on the pull request that made it 15x more
expensive, and on nearly every one since. A skipped test contributes no
row to the `Slowest N tests` cost report, so the instrument that exists
to catch exactly this could not see it.

The one lane that did run it is the nightly ungated re-take, which has
run it every night at 66-83 s, flagged `SLOW` at over 60 s, in green,
unread — until an orchestrator went to the jobs API on 2026-09-11 for a
different reason. `work/ciw/nightly-demotions-have-never-run` found the
same shape one lane over, and Ev's ruling of 2026-09-07 is the general
form: **a detector nobody reads is not a control.**

So this row is now evidence in two open arguments rather than only a
cost bug:

- for `fuzz-depth-not-existence-run-everything-at-effort-1`, that a
  gate deciding EXISTENCE hides what a dial deciding DEPTH would have
  shown — the cost report would have carried this row on every run;
- for retiring the nightly re-take, that its output has never been read
  and a row screaming `SLOW` for eight nights is what that costs.

### What is NOT concluded

That the change was wrong. Replaying at `Sym<Interval>` is M10-7's
design and may be worth every second of it; this row does not reopen it
and has not read that argument. What is wrong is that a 15x cost change
landed with no figure attached, under a gate that hid it. Two honest
dispositions, and the choice belongs to whoever owns the rows:

1. **Re-cut the budget against the new arithmetic.** The 1024 threshold
   and the 1280 margin were measured on the old one; re-derive both and
   the row may come back near its old cost with its claim intact.
2. **Record the new cost and keep the budget.** Then the row is a
   deliberate 66-83 s on the interval lane, the file's prose is corrected
   to say so, and it is placed on purpose rather than by accident.

Either way the file's stale measured sentences are fixed in the same
change — they are the reason this took a bisect to find rather than a
read.


## Quantified and handed to M10 (2026-09-11)

The remaining question — how much of the 15.2x is the symbolic tier
itself — is measured. Same box, same profile, same command, one line
changed (`SymbolicDials::default()`'s `enabled`):

| configuration | suite wall |
|---|--:|
| tier OFF | **15.364 s** |
| tier ON, shipped dials | **319.373 s** |
| the suite before the tier existed (`a4439fbef`) | 21.041 s |

**The tier is 304 of 319 seconds — 95.2%, a 20.8x multiplier** — and the
rest of the kernel did not regress at all: with the tier off the suite is
FASTER today (15.4 s) than the whole suite was before E12 (21.0 s). The
entire delta is the symbolic tier.

Two things that fall out, both now on M10's slate as
`work/m10/symbolic-tier-costs-95-percent-of-the-m10-3-drive`:

- **The degree-16 result is explained and the dial is exonerated.**
  `drive.rs`'s own note predicts it: *"the endpoint identity the tier's
  headline row depends on needs degree >= 32 to cancel; at 16 it freezes
  and the row does not move."* A frozen form sends work back to
  subdivision, so 16 is slower than 128. The budget is correctly set and
  this row is not about the dials.
- **All nine rows pass with the tier off**, which is a coverage question
  for M10 rather than a case for turning it off, and is recorded there
  as one.

So the disposition for THIS row narrows: nothing here should re-cut
`CHAMBER_LEAVES`. The budget was measured correctly, the rows assert what
they assert, and the seconds belong to a kernel tier that the real
program pays too. Either M10's row makes the tier cheaper and this one
closes with it, or the M10-3 suite is deliberately placed at ~84 s
hosted and the file's stale prose is corrected to say so.

## Parked on M10's row (2026-09-12)

Status moved `open` -> `parked`. The diagnosis is complete and the fix is
a kernel change inside a tier M10 designed and owns, so **nothing on this
side is dispatchable** — the row was sitting on the board as available
work it is not, which is the board lying about its own state.

Trigger: `work/m10/symbolic-tier-costs-95-percent-of-the-m10-3-drive`.
When that closes, this row re-reads the suite's cost and either records
the new figure or closes.

What stays this program's, and is why the row is parked rather than
re-homed: the cost figure the gate hid is S-TCOST's instrument, and
`fuzz-depth-not-existence-run-everything-at-effort-1` is ordered behind
this — wiring the EFFORT policy before the regression is fixed puts a
66-83 s row on every pull request.

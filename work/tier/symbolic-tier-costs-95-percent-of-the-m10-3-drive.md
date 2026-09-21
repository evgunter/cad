---
id: symbolic-tier-costs-95-percent-of-the-m10-3-drive
kind: issue
title: The E12 symbolic tier is 95% of the M10-3 interval drive: 20.8x measured, and nothing has profiled inside the normal form
status: open
opened: 2026-09-11
parent: SYM-1
priority: P1
cost: H
---


Filed 2026-09-11 by S-TCOST, on Ev's direction in chat, out of the
diagnosis of `work/tcost/m10-3-chamber-row-reads-ten-times-its-recorded-cost`.
**Nothing here says the tier is wrong or should be off.** It says the
tier is expensive, nobody has measured where inside it the time goes, and
the first work is that measurement.

## The measurement

One box (4 vCPU / 15 GB, shared), `test` profile (opt-0), same command
each time — `cargo nextest run -p editor-core --features interval
--test all -E 'test(/m10_3_r1_probes_interval/)'`. LOCAL readings, an
iteration tool and not a result of record
(`memories/perf-measurement-lane.md`); a unit would need a hosted pair.

| configuration | suite wall | note |
|---|--:|---|
| `SymbolicDials::default()` with `enabled: false` | **15.364 s** | all 9 rows still pass |
| shipped (`enabled: true`, degree 128) | **319.373 s** | `main` at `486557f5` |
| shipped, degree forced back to 16 | 467.731 s | *slower* — see below |
| the same suite before the tier (`a4439fbef`, 2026-09-03) | 21.041 s | pre-M10-7 |

**The tier is 304 of 319 seconds — 95.2% of the suite, a 20.8x
multiplier.** And the rest of the kernel did not regress: with the tier
off the suite is *faster today* (15.4 s) than the whole suite was before
the tier existed (21.0 s). The entire delta is E12.

Hosted, applying the 3.8x local:hosted ratio measured on this same row:
~84 s with the tier (which matches the nightly's 83.3 s reading) against
~4 s without.

**The degree dial is not a lever and the design doc already said so.**
Forcing `DEFAULT_SYM_MAX_DEGREE` back to 16 makes it *slower*, exactly as
`drive.rs`'s own note predicts — *"the endpoint identity the tier's
headline row depends on needs degree >= 32 to cancel; at 16 it freezes
and the row does not move."* A frozen form sends the work back to
subdivision. The dial is correctly set; this row is not about the dials.

## What is known about the mechanism, from the design's own words

`drive.rs`'s `SymbolicDials` note is unusually explicit and is the best
starting evidence:

> the form the tier actually builds is not the written one. It is a
> QUOTIENT of polynomials reached by repeated cross-multiplication, and
> a metered extrusion contributes `‖w‖` and its reciprocal to every term
> it touches, so the degree that matters is the degree AFTER those
> denominators have been carried up the DAG.

And `sym.rs`'s own header records that the coefficient ring moved off
`i128` to arbitrary precision — `Rat` over `num-bigint`, bounded at
`COEFF_BITS` — *"They were an in-tree `i128` through M10-7"* — with a
`Small(i128)`-inline / `BigInt`-on-overflow hybrid already in place
because *"measured, a `BigInt` for every one of them"* was worse.

So the plausible cost centres, in no order and **none of them measured**:

1. **Degree growth from carried denominators.** Cross-multiplication up
   the DAG is the mechanism the doc names; a form that reaches degree 32+
   in several symbols has a term count that grows fast, and every
   normalisation walks it.
2. **The coefficient ring.** `Rat` over `num-bigint` past the `i128`
   inline path — how often the promotion actually fires on this fixture
   is a counter nobody has read.
3. **Term storage and allocation.** `BTreeMap`/`HashMap` per form, rebuilt
   per normalisation.

## What this row asks for, in order

1. **Profile inside `geom_core::sym` on the M10-3 slab.** Which of the
   three above dominates, with numbers. Everything else is guesswork
   until this exists — including any patch written from reading the code,
   which is exactly how the degree-16 hypothesis above got refuted.
2. **Read the freeze and promotion counters that already exist.** The
   design says the evidence for the budget is *"the FROZEN COUNT on the
   verdict"*; whatever that reports on this fixture is free evidence and
   nobody has looked.
3. **Only then, a change** — and it is a kernel unit under S-TCOST's
   charter clause for exactly this shape (*where a test is slow because
   the CODE it exercises is slow in a way the real program pays too, the
   fix is a kernel change*), so it runs under whatever review posture
   M10 sets, with a hosted before/after.

## One observation, recorded without an argument attached

**All nine rows pass with `enabled: false`.** That is a fact, not a
case for turning the tier off: `drive.rs` says the tier is on because
*"a certifier that can only certify boxes narrower than its own ε is the
state M10-3 pinned and E12 exists to leave"*, and these rows may simply
not be the ones that exercise what it buys. But if the M10-3 suite is
green either way, then on today's tree **nothing in it would red if the
tier regressed to its pre-E12 answers** — which is a coverage question
for M10 and is why it is written down here rather than left in a
terminal.

## Territory

Filed on M10's slate: M10 designed the tier (M10-7, PR #1725), owns
`crates/editor-core/src/drive.rs` where `SymbolicDials` and both budget
constants live, and owns the M10-3 rows that pay the cost.
**`crates/geom-core/src/sym.rs` and `sym/` are PROPS's** by its
`crates/geom-core/src/*` glob, so a fix that lands inside the normal form
is an announced cross-fence change to PROPS rather than M10's to take
silently. Named here so the first lane does not discover it at
`work.py territory`.

## Provenance

The regression that surfaced this is
`work/tcost/m10-3-chamber-row-reads-ten-times-its-recorded-cost`: the
M10-3 interval suite went 21.04 s -> 319.37 s between 2026-09-03 and
2026-09-11, bisected to PR #1725, and the gate that was supposed to
watch it skipped the suite on that very PR (the marker names
editor-core paths; #1725 changed `geom-core`). The cost was never
recorded anywhere — `work/m10/` records the tier's API costs and the
`COEFF_BITS` trade, and no runtime figure at all.

## Re-homed at M10's exit sweep (2026-09-13)

Here because the work it asks for — a profile INSIDE the normal form — is in this
program's file. **The measurement stays S-TCOST's**: that program filed it on Ev's
direction, its figures and its successor rows
(`work/tcost/m10-3-chamber-row-reads-ten-times-its-recorded-cost`,
`work/tcost/one-test-is-the-whole-ci-critical-path`) are named in the body, and no
row here is justified by a cpu-second without S-TCOST's measurement behind it.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## The profile (SYM-1)

Taken 2026-09-13/14 on the SYM-1 lane's box (4 vCPU / 15 GB, shared
with one other lane; valgrind 3.22.0; the workspace's pinned
toolchain). LOCAL readings, an iteration tool and not a result of
record (`memories/perf-measurement-lane.md`) — but the instrument is
in tree and every number below is a count a hosted re-run
reproduces, so a later unit's before/after is one dispatch away.
Instruction shares are callgrind's and are contention-proof; wall
times are the profile's `Instant` clocks and are not (the drive's wall
read 349 s and 255 s on two takes of the same binary).

**Instrument.** Two halves, as the spec asked. `geom_core::sym::profile`
(behind the test-only cargo feature `sym-profile-testing`, forwarded on
`editor-core`'s dev-dependency edge exactly as `identity-pass-testing`
is; with the feature off none of it compiles) records per op the forms
built and their sizes; every FREEZE with the cause noted at the refusal
site (`Terms` / `Degree` / `Coefficient` / `Overflow` / `ZeroDivisor` /
`Unrecorded`, and `Unnoted` asserted zero); the ring's promotions off
the `i128` path and the widest coefficient kept and refused; and per
walk (plain / early / door) AND PER ORIGIN the calls, the forms built
and the wall time — the origin being WHO asked: the decision's own
discharge (`Decision`), the contradiction assertion the `Decide` impl
runs on every DEFINITE margin wherever debug assertions are on
(`Assertion` — dev, test, and this workspace's release profile, so
every profile measured here), or the shape report's rendering
(`Report`). Inside the early walk the per-node rule A/B reduction and
rule D's fold are timed. `valgrind --tool=callgrind` gives the
instruction split by function. The rows are
`crates/editor-core/tests/m10_sym_profile_interval.rs`, all
`#[ignore]`d evidence.

**What the existing counters said first.** `SymCounts::frozen` on the
slab's chamber drive: **0**, at every leaf of 2,559. On the plate at
its nominal: 1,044 (the header's 1,056 was M10-8's reading; the zero
normalisation and A1's folds since moved it) — and `frozen` counts the
plain walk whoever asked it, so 360 of those 1,044 are the assertion's.
The shape report sizes a residual but not the population, and neither
says WHY a form froze, who asked for it, or what it cost — which is
what the profile adds.

### Method — re-run it

```
# the row: bounded_chamber(60ε, 30ε, 100ε), the M10-3 chamber S-TCOST bisected on
# (`m10_3_r1_probes_interval::the_driven_chamber_replays_bit_identically_…`)
export CARGO_TARGET_DIR=…                                  # a private one
cargo test -p editor-core --features interval --test all --no-run            # test profile, as S-TCOST measured
cargo test --release -p editor-core --features interval --test all --no-run  # release, beside it
B=<the executable each prints>
# structural profile (feature on through the dev edge; nothing to enable)
$B --ignored --exact m10_sym_profile_interval::sym_profile_slab_replays --nocapture   # nominal / leaf (±ε) / root
$B --ignored --exact m10_sym_profile_interval::sym_profile_slab_drive   --nocapture   # the whole row, 1280 leaves
$B --ignored --exact m10_sym_profile_interval::sym_profile_plate_nominal --nocapture
# callgrind over a BARE replay (no shape report, no profile; one replay per repeat, nothing else)
CAD_SYM_PROFILE_DOC=slab  CAD_SYM_PROFILE_BOX=nominal CAD_SYM_PROFILE_REPEATS=4 \
  valgrind --tool=callgrind --callgrind-out-file=cg-slab.out \
  $B --ignored --exact m10_sym_profile_interval::sym_profile_callgrind_replay --nocapture --test-threads=1
CAD_SYM_PROFILE_DOC=plate CAD_SYM_PROFILE_BOX=nominal CAD_SYM_PROFILE_REPEATS=1 \
  valgrind --tool=callgrind --callgrind-out-file=cg-plate.out $B --ignored --exact … --test-threads=1
callgrind_annotate --threshold=100 cg-slab.out                  # self Ir per function
callgrind_annotate --threshold=100 --inclusive=yes cg-slab.out  # inclusive Ir per function
```

The self-cost classes below partition EVERY instruction the process
retired, by function name (`num_bigint::*` is the BigInt ring;
`<Int>`/`<Rat>` the `i128` path with its gcd; `collections::btree` the
term maps; `malloc`/`free`/`memcpy` the allocator; `Poly`/`Form`/
`mono_mul`/`within` the merge loops; `form_in`/`intern`/`discharge` the
walk and the DAG build; `hashbrown` the memo maps), and the rows below
sum to 100 % because the residual row `other` is shown. A test-profile
listing spreads a quarter of the count over `core` iterator and
`ub_checks` glue that release inlines away (chiefly `Hash128::word`'s
byte loop, 31.7 % inclusive at opt-0 against 0.6 % self in release) —
the ranking is read from release and the test profile is given beside
it because the wall S-TCOST measured is the test profile's.

### 1. The ranking

Self instruction share of one bare replay at the nominal, release
profile (test profile in brackets). Slab: 141 M Ir per replay (831 M
at opt-0); plate: 1,305 M (6,827 M). Every row of the partition:

| class | slab | plate |
|---|--:|--:|
| **term storage** — allocator + memcpy | 34.0 % (9.5) | 30.3 % (9.5) |
| **term storage** — `BTreeMap<Mono, Rat>` walk / insert / clone / drop | 16.4 % (16.5) | 21.3 % (19.5) |
| **term storage** — `Vec<(u128,u32)>` monomials, `Rc<Form>`, clone/drop glue | 6.0 % (6.6) | 5.1 % (9.4) |
| **term storage, total (candidate 3)** | **56.4 %** (32.6) | **56.7 %** (38.3) |
| **the ring** — `Rat`/`Int` on the `i128` path, `from_parts`' gcd and `strip_twos` | 8.7 % (3.7) | 14.6 % (5.3) |
| **the ring** — `num-bigint` | 1.3 % (1.1) | 12.8 % (12.4) |
| **the ring, total (candidate 2)** | **10.0 %** (4.8) | **27.4 %** (17.7) |
| polynomial merge loops (`Poly::mul`/`insert`/`add` self, `within`) | 9.7 % (5.0) | 7.8 % (6.6) |
| the walk and the DAG build — `form_in`, `intern` (node ids + table) | 15.5 % (2.2) | 3.7 % (0.7) |
| memo maps (`hashbrown`) + content hashing (`Hash128`, digests) | 3.0 % (16.7) | 1.0 % (4.4) |
| `Sym<Interval>`'s `Real` ops (the DAG build above `intern`) | 1.0 % (1.9) | 0.3 % (0.9) |
| rules A/B (`algebra`) self; rule D and rule C | 0.3 % (0.1); 0.0 | 1.0 % (0.6); 0.0 |
| the numeric channel (`Interval`) | 2.6 % (1.6) | 0.6 % (0.4) |
| `core`/`std` generic glue (iterators, `ub_checks`; opt-0 only) | 0.1 % (28.6) | 0.1 % (22.4) |
| the kernel above the scalar | 0.1 % (0.2) | 0.1 % (0.1) |
| other (libc, the harness, unclassified) | 1.4 % (6.3) | 1.4 % (8.1) |

Inclusive, release (test): slab — `sign_within` 72.4 % (55.8),
`discharge` 72.2 % (55.7), `plain_form` 56.6 % (42.7), `early_form`
14.7 % (12.4), `intern` 12.9 % (35.6), `Poly::mul` 25.3 % (19.0),
`Poly::insert` 9.6 % (5.5), `Poly::add` 5.9 % (3.2), `Rat::mul` 8.1 %
(5.7), `Rat::add` 1.8 % (1.4), `Rat::from_parts` 6.6 % (5.6),
`reduce_steps` 4.2 % (2.6); the session's TEARDOWN, which no walk clock
sees — `drop_glue::<Session>` 10.0 %, of which `Rc<Form>::drop_slow`
9.4 % (the memos' forms). Plate — `sign_within` 92.2 % (90.3),
`discharge` 85.3 % (82.9), `plain_form` 11.3 % (10.9), `early_form`
60.5 % (61.4), `door_form` 20.2 % (17.9), `reduce_steps` 53.0 %
(47.2), `Poly::mul` 44.0 % (51.0), `Poly::insert` 19.3 % (19.1),
`Rat::mul` 23.4 % (30.1), `Rat::add` 5.6 % (7.1), `Rat::from_parts`
24.2 % (34.6), `intern` 2.1 % (6.4), `trig::fold` 0.4 % (0.5);
teardown 3.7 % (`Rc<Form>::drop_slow` 4.4 %).

**The ranking, on the slab: (3) term storage and allocation, 56 %;
then the walk's own overhead and the DAG build, 19 %; then (2) the
coefficient ring, 10 %, of which `BigInt` is 1.3 %; (1) degree growth
is not a cost on this document at all.** The slab's forms are TINY:
mean 1.5 terms out (max 10), total degree up to 68, `SymCounts::frozen
= 0` on every one of the drive's 2,559 leaves, no form within a factor
of 400 of the term budget. What the slab pays for is VOLUME times a
fixed cost per form: 10,604 plain forms per nominal replay — 9,686 of
them the decision's, 918 the assertion's — for 1,490 decisions (964
theorems, 526 numeric), at ~7.5 k instructions per form in release
(`plain_form` inclusive ÷ forms) — one `BTreeMap`, one heap `Vec` per
monomial per term, one `Rc`, one hash of the node, one gcd per
coefficient, for a form that is mostly `q^k` over `sqrt(q²)^j`. On the
plate the same storage share (57 %) sits beside a real ring share
(27 %, `BigInt` 12.8 %), and BOTH ride inside the per-node A/B
reduction (`reduce_steps`, 53 % inclusive), whose substitutions build
the products that freeze on degree — so on the plate (1) is the
freeze MECHANISM and (3)+(2) are the cost of reaching it.

### 2. The two scales, the assertion's share, and the whole row

Per walk and origin, one slab replay (release clocks; test clocks in
brackets). `D` is the decision's discharge, `A` the assertion's, `R`
the shape report's (calls only, every one a memo hit):

| slab replay | decisions (theorem / numeric) | plain calls D / A / R | plain forms D / A | early calls D / A | early forms D / A | `reduce_steps` calls | wall, plain D / A, early D / A |
|---|---|---|---|---|---|--:|---|
| nominal, 56.0 ms (166) | 964 / 526 | 980 / 510 / 16 | 9,686 / 918 | 16 / 510 | 36 / 1,958 | 1,994 | 27.4 / 3.4, 0.1 / 7.1 ms (69.9 / 9.3, 0.3 / 19.2) |
| leaf-sized, nominal ± ε, 32.8 ms (142) | 964 / 526 | 980 / 510 / 16 | 9,686 / 918 | 16 / 510 | 36 / 1,958 | 1,994 | 14.2 / 2.0, 0.1 / 4.0 ms (62.3 / 7.9, 0.3 / 17.9) |
| root box (±100 ε), 1.0 ms (5.6) | 100 / 86 | 118 / 68 / 18 | 82 / 91 | 18 / 68 | 41 / 102 | 143 | — |

A form is charged to the origin that FIRST built it; the assertion
walks a definite margin's DAG whose kids the decision path then finds
memoized, so with the assertion removed (the reviewer's mutation) the
decision's plain forms read 9,938 and early 66 — the 252 and 30
between are shared nodes. Read either way: **the decision path builds
nine tenths of the slab's plain forms and next to none of its early
ones; the early walk on the slab — its 1,994 `reduce_steps` calls, its
36 atoms of the 56 — is the assertion's.** The answer is the same at
the nominal and over a leaf: over a box 2 ε wide an identity margin's
enclosure `[0, c·w]` is still not definite, so the decision path builds
exactly the forms it builds at the nominal, and the assertion runs on
the same 510 definite margins. The root box refuses at its second node
(the extrusion vector straddles zero over ±100 ε).

The drive over the whole row (test profile, sequential, profile on):
2,559 sessions (1,280 leaves + 1,279 splits), 21.7 M nodes interned
(8,488 per session against 12,208 at the nominal), wall 254.9 s:

| origin | plain calls | plain forms | early calls | early forms | plain wall | early wall |
|---|--:|--:|--:|--:|--:|--:|
| decision | 1,569,376 | 17,528,640 (6,850 / session) | 43,236 | 171,363 | 117.6 s | 1.4 s |
| assertion | 803,738 | 1,571,279 | 803,738 | 3,182,424 | 13.7 s | 29.2 s |

0 frozen; `reduce_steps` 3.35 M calls 5.8 s, `trig::fold` 20 k calls
87 ms, top-residual reduce 847 k calls 1.0 s; 33.0 M `Rat` operations,
418 k on the heap path (1.27 %), 104.6 k promotions (0.32 %), widest
coefficient 201 bits; max terms 10, max degree 68. The assertion is
8 % of the drive's plain forms, 95 % of its early forms, and 43 s of
the 162 s the walks take.

### 3. The freeze population

Slab, at every scale and over the whole drive: **none** — no `Terms`,
no `Degree`, no `Coefficient`, no `Overflow`. Plate at its nominal,
1,312 freezes over the three walks (plain 1,044 = `SymCounts::frozen`;
early 164; door 104 — a node the plain walk freezes is frozen again by
each later memo), split by who asked:

| origin | plain `Degree` | plain `Coefficient` | early `Coefficient` | door `Coefficient` | total |
|---|--:|--:|--:|--:|--:|
| decision | 672 | 0 | 48 | 104 | 824 |
| assertion | 360 | 12 | 116 | 0 | 488 |

(With the assertion removed the decision's plain count reads 684: 12
of the assertion's `Degree` freezes are on nodes the decision path
then meets in the memo.) By cause and op, both origins, with the kids'
sizes at the freeze:

| cause | op | count | kid total degree min / mean / max | kid terms min / mean / max |
|---|---|--:|---|---|
| `Degree` | `Powi` | 516 | 70 / 96.3 / 117 | 1 / 3.0 / 7 |
| `Degree` | `Mul` | 408 | 40 / 85.9 / 117 | 1 / 2.1 / 4 |
| `Degree` | `Sub` | 108 | 66 / 70.1 / 74 | 1 / 2.0 / 3 |
| `Coefficient` | `Powi` | 232 | 0 / 6.8 / 34 | 2 / 7.0 / 10 |
| `Coefficient` | `Add` | 48 | 4 / 5.3 / 8 | 14 / 37.8 / 90 |
| `Terms`, `Overflow`, `ZeroDivisor`, `Unrecorded`, `Unnoted` | — | 0 | | |

Widest coefficient kept 249 bits (the bound is 256), widest refused
401 bits. **The derived-frame mechanism is visible here and absent on
the slab**: 1,032 of 1,312 freezes are on DEGREE, on a square or a
product whose kids already stand at degree 40–117 with one to seven
terms — a monomial-shaped quotient whose degree the carried
denominators doubled once too often (`Powi`'s degree in → out on the
plate is 18.9 → 13.8 mean but 122 max; on the slab 11.4 → 22.8, never
past 68). The 280 coefficient freezes are the other shape M10-8 named:
low degree, many terms, 53-bit mantissa products past 256 bits.
`drive.rs`'s note — degree binds, terms never do — is confirmed on
both documents by count: no form on either is within 40× of
`DEFAULT_SYM_MAX_TERMS`. The decision path alone freezes 824 (672 on
degree); the assertion's 488 are the same two shapes on the definite
margins' DAGs.

### 4. The promotion count

| | `Rat` ops | heap-path `Int` ops | promotions (`Small`→`Big`) | `BigInt` self share, release |
|---|--:|--:|--:|--:|
| slab, one replay | 19,568 | 288 (1.5 %) | 72 (0.37 %) | 1.3 % |
| slab, the whole drive | 33,022,715 | 418,392 (1.27 %) | 104,598 (0.32 %) | — |
| plate, nominal | 232,169 | 21,132 (9.1 %) | 2,967 (1.28 %) | 12.8 % |

The `BigInt` arithmetic's share of the instructions matches its share
of the operations on the slab (1.3 % against 1.5 %) and is ~1.4× it
on the plate (12.8 % against 9.1 %: a heap op costs more, and most of
that 12.8 % is `biguint_shr2` and `sub_assign` — `strip_twos` and the
gcd inside `Rat::from_parts`, run after every product). The `i128`
inline path holds: promotions are under half a percent of the ring's
operations on the slab and just over one percent on the plate. (The
ring counts are not split by origin; on the slab the assertion's forms
are a fifth of all forms built, on the plate a quarter.)

### 5. Where the walks spend it

Release wall by the profile's clocks (test profile in brackets), the
decision's share then the assertion's:

| | plain walk D / A | early walk D / A | door walk D / A | of the early walk: `reduce_steps` | rule D `trig::fold` | top-residual A/B |
|---|---|---|---|---|---|---|
| slab nominal, 56.0 ms (166) | 27.4 / 3.4 ms (69.9 / 9.3) | 0.1 / 7.1 ms (0.3 / 19.2) | — | 3.1 ms (4.4), 1,994 calls, 36 the decision's | 6 µs, 8 calls | 0.3 ms, 526 calls |
| plate nominal, 256.7 ms (1,119) | 40.2 / 7.1 ms (141.6 / 25.8) | 93.4 / 16.6 ms (498.2 / 86.4) | 46.0 / 0.0 ms (213.0 / 0.1) | 97.6 ms (452.8), 23,001 calls | 1.1 ms (5.0), 553 calls | 0.5 ms, 610 calls |

On the slab the plain walk is the tier (57 % of the replay's
instructions inclusive; 980 decision calls building 9,686 forms), and
the early walk is the assertion's: the decision path asks it 16 times
for 36 forms, the assertion 510 times for 1,958. Rules A/B per node
are 4 % of the replay and almost entirely the assertion's — the slab
has 56 atoms and eight trig nodes. On the plate the early and door
walks are 80 % of the replay and `reduce_steps` — rules A/B per node
— is 53 % alone, 89 % of the early walk's wall, five sixths of it the
decision's (early D 93.4 ms against A 16.6); rule D's fold is 0.4 %,
its closed forms memoized per argument. Outside every clock: the
session's teardown, 10 % of a slab replay and 4 % of a plate replay
inclusive (release), and the DAG build (`intern`, 13 % and 2 %).

### What a fix would target — one proposal per candidate, the number behind it

Ask 3 (the change) is NOT taken here; these are the next unit's
inputs, each with the count that argues for it, none with a design.

- **Candidate 3, term storage — the largest number on both documents
  (56 % of instructions in release).** A form averages 1.5 terms and
  never exceeds 10 on the slab (90 on the plate); every one is a
  `BTreeMap<Vec<(u128, u32)>, Rat>` — a tree node plus one heap `Vec`
  per monomial, cloned on every `add`, `mul`, `neg` and memo insert,
  and the allocator alone is a third of the count; the teardown that
  frees them is another tenth. The proposal: a small-vector polynomial
  — terms as a sorted `Vec` with the monomial inline for the
  one-to-three-indeterminate case — so a form of ten terms is one
  allocation, not eleven. The bound on the win is the storage share
  itself; the merge loops it feeds are 9.7 %.
- **The assertion — not one of the three, and the second number on
  the slab's early walk.** `Decide for Sym<T>` runs `discharge` inside
  a `debug_assert!` on every margin the numeric channel proved
  non-zero, and this workspace's release profile keeps
  `debug-assertions = true`, so in every profile it builds the plain
  AND early forms of every definite margin: a tenth of the slab's
  plain forms and 95 % of its early forms (1,958 of 1,994 nominal
  forms; 3.18 M of 3.35 M over the drive; 43 s of the drive's 162 s in
  the walks), and 488 of the plate's 1,312 freezes. The design
  question it raises, for the next unit and not decided here: a
  soundness cross-check that is a second full walk — whether it
  belongs on every definite decision, on a sample of them, or behind
  its own feature, and what the published build (where the release
  stanza "comes back OUT") loses when it goes.
- **The volume behind it — not one of the three, and the third
  number on the slab.** 9,686 decision-built plain forms per nominal
  leaf for 980 decisions the numeric channel could not answer, and a
  DAG interned afresh in each of 2,559 sessions (21.7 M `intern`s,
  8,488 per session; `intern` is 13 % of a nominal replay in release,
  36 % at opt-0 where `Hash128::word`'s byte loop is not inlined). The
  plain form reads no value — it is a function of the node's content
  hash alone — so its memo is valid across leaves of one drive; today
  the session "holds nothing across leaves" by design (module header,
  D9 section). The proposal is a drive-scoped plain memo keyed by
  `SymId`, which would remove up to the plain walk's 57 % from every
  leaf after the first — with the premises a design has to meet: the
  plain walk has SIDE EFFECTS a memo hit would skip (`combine`
  registers every atom in `sess.atoms`, which the top-residual reduce
  and `reduce_steps` read; `frozen` is incremented inside the walk, so
  a hit would leave the receipt first-leaf-only), and the opaque
  sequence (`OPAQUE_SEQ`, per replay by D9's argument) becomes
  load-bearing across leaves. Inputs, not a design; it is a
  session-model change and needs the design conversation before the
  code.
- **Candidate 2, the ring (10 % slab, 27 % plate).** `Rat::from_parts`
  runs a gcd and two `strip_twos` after EVERY `add` and `mul` (self
  3.5 % slab / 7.5 % plate; the `BigInt` shifts and subtractions
  under it are most of the plate's 12.8 %). Promotions are rare
  (0.3–1.3 % of ops), so the inline path is right; the proposal is
  to make the normalisation cheaper on the common shape — skip the
  gcd when the denominator is one (a dyadic coefficient, which
  `exp2` already carries), and reduce lazily where the product is
  about to be refused anyway. `Poly::degree` walks every term twice
  per product and twice per `within` (7.8 % inclusive at opt-0,
  1.4 % `within` self in release): a cached degree is the small
  version of the same idea.
- **Candidate 1, degree growth from carried denominators — no cost
  on the slab; the freeze mechanism on the plate.** 1,032 of the
  plate's 1,312 freezes are `Degree` on kids at 40–117 (672 of them
  the decision's), inside the per-node A/B reduction that is 53 % of
  the plate's instructions. The fix that targets it is the one
  `derived-frame-placement-freezes-on-the-symbolic-lane` names — a
  degree-resetting atom for a value-exact norm, or normalisation
  simplified before squaring — and that row owns it; the number this
  unit adds is that the term budget is never the wall (no `Terms`
  freeze anywhere; max 90 terms against 4,096) and the coefficient
  bound is a fifth of the freezes (280, widest refused 401 bits).

## The change (SYM-4)

Ask 3, on the two in-session levers the profile ranked first and
third: **a `Poly`'s terms are one sorted vector in the `BTreeMap`'s
own order**, and **the ring skips the gcd and the products by one on
the dyadic shape**. Every count identical before → after on both
documents — forms, atoms, frozen, decisions by outcome, and the digest
chain of every form the walks build, which
`m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`
now pins per ε row on the slab and once on the plate (captured on the
merge base, asserted since). The one profile line that moved is the
ring's `big-path int ops` (slab 288 → 144, plate 21,132 → 10,584):
the heap gcds against one and products by one the second lever
removed. Method as SYM-1's, re-taken on the SYM-4 lane's box (4 vCPU,
shared with one review lane); instruction counts are callgrind's over
one bare replay at the nominal and reproduce to within 500 Ir across
takes; the walls are local and say so.

| instructions, release (one replay) | before | vector terms | + dyadic ring |
|---|--:|--:|--:|
| slab, total | 141.5 M | 104.3 M (−26 %) | 99.0 M (−30 %) |
| slab, term storage (allocator + `BTreeMap` + monomial/`Rc`/glue) | 75.7 M · 53 % | 39.8 M · 38 % | 39.7 M · 40 % |
| slab, the ring (`i128` path + `num-bigint`) | 13.2 M · 9.4 % | 13.2 M · 12.7 % | 8.6 M · 8.7 % |
| slab, `Rat::from_parts` inclusive | 6.6 % | 9.0 % | 5.7 % |
| slab, `plain_form` inclusive ÷ plain forms | 7.6 k | 5.0 k | 4.7 k |
| plate, total | 1,300 M | 976 M (−25 %) | 713 M (−45 %) |
| plate, term storage | 728 M · 56 % | 371 M · 38 % | 365 M · 51 % |
| plate, the ring | 346 M · 27 % | 346 M · 35 % | 109 M · 15 % |
| plate, `num-bigint` alone | 169 M · 13 % | 169 M · 17 % | 5.0 M · 0.7 % |
| plate, `Rat::from_parts` inclusive | 24.3 % | 32.3 % | 10.4 % |

| local walls (an iteration reading, not a result) | before | vector terms | + dyadic ring |
|---|--:|--:|--:|
| slab nominal replay, release, profile on | 47.2 ms | (not taken) | 35.1 ms |
| plate nominal replay, release, profile on | 283 ms | (not taken) | 149 ms |
| the chamber drive row, test profile, sequential harness, one take | 368.0 s | 239.7 s | 225.0 s |

**The hosted before/after** (`memories/perf-measurement-lane.md`: a
hosted runner's numbers, read with the spread in mind; the instruction
counts above are the numbers of record). Base: run 34812106311, the
nearest code-tier PR run to the merge base — which is TRIM-3 PR-2's
HEAD (`8e53655d`), not the merge base itself, so its cpu-s carry that
PR's change beside the runner's contention; no run of the merge base
alone exists (main pushes carry no test matrix). After: SYM-4's PR
run 34823472408 on head `3d7f3526b` (all twelve `test` jobs and five
`k-lint` unifications green). The nextest `count:*/2` partition re-cut
around the new row and the change filter bought a different set of
suites (2,406 → 3,810 tests in the interval `1/2` shards), so
per-shard durations do not compare and per-test cpu-s carry
contention from whatever shared the runner — the spread between two
runs of one tree is itself large (the frozen head's second run read a
shard at 92 s that its first run read at 169 s, ~1.8×), so the
DIRECTION of the per-test drops below survives it and their
magnitudes do not. The rows are the ones in both runs' top-20 tables:

| interval lane, cpu-s per test | base (shard) | SYM-4 (shard) |
|---|--:|--:|
| `m10_7_r2 … r2_the_drive_is_bit_identical_across_repeats_and_the_rayon_schedule` | 302.9 (default 2/2); 302.0 (1e-12 2/2) | 212.2; 248.7 |
| `m10_7_r2 … r2_the_tier_off_serialization_carries_no_symbolic_line` | 278.5; 259.4 | 189.8; 203.2 |
| `m10_10_r2_probes::r2_rule_d_agrees_with_a_by_hand_closed_form…` (geom-core) | 70.0; 52.7 | 44.1; 42.5 |
| `m10_10_pins … eps_relative_ceilings_under_the_shipped_set` | 30.4 (default 1/2); 29.9 (1e-12 1/2) | 16.1 (default 2/2); 17.4 (1e-12 2/2) |
| `m10_3_r2 … my_own_drive_is_bit_identical_across_repeats_and_schedules` | 19.6; 19.7 | 13.1; 12.3 |
| `m10_3_driver … a_sliver_wrapped_in_the_ops_own_error…` | 14.1; 13.9 | 9.4; 10.6 |
| **the chamber row** `m10_3_r1 … the_driven_chamber_replays_bit_identically…` | **121.96 (default 1/2); 130.98 (1e-12 1/2)** | **did not run** — the suite is gated to the driver's paths, not the tier's (`work/tcost/m10-3-chamber-probes-gated-away-from-the-symbolic-tier`) |

Shard walls, for the record and not for comparison: `test (interval,
eps = default, 1/2)` 147 → 169 s, `2/2` 355 → 248 s, `1e-6 1/2` 129 →
123 s, `1e-6 2/2` 138 → 136 s, `1e-12 1/2` 558 → 118 s, `1e-12 2/2`
355 → 826 s (the 479-cpu-s tolerance-study probe moved shards and
read 760 cpu-s beside the two `m10_7_r2` drives).

**Disclosed deviations.** (1) `unary_at_zero`'s `Acos` arm — `acos(0)
= π/2` — was `Poly::indet(π)` with its coefficient multiplied by ½ and
is now `Poly::term(π, ½)`: the same polynomial to the bit, one
`Rat::mul` fewer, so the profile's `rat ops` is one lower per
`acos(0)` fold (zero on the slab and the plate at their nominals,
whose rat-ops lines are identical; it would read on a document that
folds `acos(0)`) — an instrument-only change, no decision. (2) Four
files outside the spec's list changed only where they read the map's
API or spelt a one-term polynomial: `sym/algebra.rs`, `sym/signed.rs`,
`sym/trig.rs`, `sym/report.rs`. (3) The fix pass adopted the reviews'
rows — the canonical-`Rat` row in `rational.rs` (red under a gcd
skipped on every shape, green on the tree), three `form.rs` rows on
the vector's invariant, the six-document walk-ledger evidence row
(`the_walk_ledger_on_the_unmeasured_documents`: the plate, both
brackets, the annulus, the pad, the link — the coverage this record
did not claim; both reviewers ran that differential identical), and
the largest-form growth guard on the pinned row (slab 10, plate 90 —
a guard that fails LEGIBLY, not fast: it is read after the replay
returns, so a compounding growth mutant still times out before the
assertion is reached, as the delta measured; a non-compounding one
names itself) — and made `Poly::terms` private behind an accessor.
`mul` shrinks its product to fit (+0.14 % instructions, inside the
spread); `add` still allocates `|a| + |b|` and keeps what the merge
drops (~1.8 MB of slack on the plate's nominal, the delta's probe) — a
half-fix on the slack class, memory and not a decision, left as is.

**What was measured and not taken, with its number.** The monomial
inline (`smallvec` at width four, the slab's maximum and nine tenths
of the plate's monomials): 413 M against 417 M on the slab and 971 M
against 976 M on the plate — under one percent, because a 224-byte
term entry costs in memmove and clone most of what the allocator
saves and the crate's non-inlined `cmp` eats the rest; a new shipped
dependency in the kernel crate does not buy one percent. The product
loop as collect-then-sort-and-merge: 436 M and 1,020 M, a fifth worse
on both. A scratch monomial reused across the product loop: a wash
(418 M / 980 M), because a product here is mostly one term by one. A
cached degree: `within`'s self cost is 1.1 % of the slab and
`Poly::degree`'s own row 1.2 % of the plate where it is not inlined,
so the cache's whole gain is bounded by about one percent and it was
not written.

**What remains, as the next input.** The storage class is still the
largest number on both documents (40 % / 51 %), and it is now the
allocator's per-form traffic — one `Vec` per monomial, one `Rc<Form>`
per memo entry, the term vector's clone on every `add`, `neg` and
`recip` — not a tree. The walk's own overhead and the DAG build are
the second number on the slab (24 %, `intern` 18 %): the volume, and
the drive-scoped plain memo that would remove it is the session-model
decision this unit was cut not to take. The ring is 9 % of the slab
and 15 % of the plate, its `from_parts` 5.7 % and 10.4 % inclusive,
`strip_twos` 1.5 % and 3.1 %. The assertion's share is unchanged as a
fraction (a tenth of the slab's plain forms, 95 % of its early forms)
and is the item's separate question.

## Coverage: which rows DO red with the tier off

The observation above ("all nine M10-3 rows pass with `enabled:
false`") has its answer: those nine rows' subject is the DRIVER —
its accounting, the flips it names, containment, the grid floor —
on boxes sized in ε where the tier's theorems change no verdict, so
they pass either way by construction and pin nothing about the tier.
What pins the tier's answers is elsewhere (`rg -n
"SymbolicDials::off|without_the_algebra|SymBudget::none"
crates/*/tests`, 21 files): **`m10_10_pins_interval`** asserts the
plate's per-predicate theorem / gated / registered / numeric split at
the nominal (180/0/0/0 and the door's 72) and the plate's and the
annulus's ceilings as fractions of their REAL studies at three ε rows
— with the tier off, or regressed to pre-E12 answers, every one of
those reds; **`m10_9_pins_interval`** pins the registered column and
that the door is inert on straight geometry; **`m10_8_pins_interval`**
pins the bracket's discharge under the shipped set and the A0 set's
inertness on straight geometry; `m10_3_driver_interval::the_tier_off_reproduces_the_pre_e12_refusal`
and `m10_7_r2_probes_interval`'s byte-identity row pin the OFF lane
itself (the pre-E12 bytes), so a regression that made the tier a
no-op would leave those green and the pins above red. The tier's
answers are therefore pinned in the tree — in the M10-8/9/10 pin
suites and not in the M10-3 suite that pays for them, which is the
row S-TCOST's cost question is about.

## Decision for Ev (2026-09-14, `[ev]` PR from the SYM orchestrator): the plain form's memo and the leaf

**Where the cost stands after SYM-4.** In release, one nominal replay
of the M10-3 slab is 99 M instructions (was 141.5 M); storage 40 %,
the walk and the DAG build 24 %, the ring 9 %. Nothing freezes on the
slab. What the slab pays for is VOLUME: ~10,000 plain forms per leaf
for 1,490 decisions, and the same 12,208-node DAG interned afresh in
each of a drive's 2,559 sessions (21.7 M `intern`s over the drive; 19
M plain forms). The plain walk is half of every leaf's replay and it
recomputes, leaf after leaf, forms that are the SAME function of the
same content hash.

**The fact the proposal rests on.** A node's id is a content hash of
`(op, children, payload)`, leaf-invariant (a `Param` carries only its
symbol, a `Lit` its bits, an `Opaque` the per-leaf sequence — the
same on every leaf of one drive by D9's fixed single-threaded walk),
and the PLAIN form reads no value: it is a function of the id and the
session's budget alone. So a plain form computed on one leaf is valid
on every other leaf of the same drive.

**D3 — may the tier's plain-form memo outlive the leaf?** Today the
hash-consing table is per-leaf-replay, holds nothing across leaves,
and is dropped with the leaf (`sym.rs`, the D9 section — the tier's
own module docs, not ERROR-DESIGN E12, which says only "memoized per
node").

- **(1) A drive-scoped plain memo**, keyed by `SymId`, installed by the
  drive around its leaves and dropped with the drive; the early and
  door walks stay per leaf (they consult the registry and the leaf's
  rules). Three side effects, each with a definition to choose:
  `sess.atoms` are registered inside the plain walk and read by the
  top-residual reduce and by `reduce_steps` — the memo carries its
  atoms; `SymCounts::frozen` is incremented inside the walk — it
  becomes "distinct nodes frozen over the drive" on the drive's
  receipt while each leaf's receipt keeps its decision counts (the
  per-leaf accounting goldens' `frozen` column moves and is
  re-blessed as the acceptance's own move); the opaque-sequence
  argument becomes load-bearing across leaves (a pin). **Recommended**:
  it removes up to the plain walk's half from every leaf after the
  first — the largest lever left on the slab — and its soundness
  argument is the one the tier already makes for two occurrences of a
  node inside one leaf, applied across leaves.
- **(2) Keep the per-leaf session** (status quo, ratified as not-now):
  the leaf stays a self-contained unit with a self-contained receipt;
  the cost is the volume above, paid on every drive.
- **(3) A subtree memo**: a child leaf inherits its parent's memo at
  the split and drops it with the subtree — (1) confined to the
  bisection tree, the same side effects one level down. Not
  recommended over (1): it buys most of the win with a second scope
  to reason about.

The orchestrator's pick is (1), as a unit in block SYM-B2 (H,
STRUCTURAL — the receipt semantics are the design, the memo is the
code), unless Ev prefers the leaf to stay self-contained, in which
case this proposal closes with (2) on the record. The assertion
discharge (the `Decide` impl's `debug_assert!` — a tenth of the
slab's plain forms, 95 % of its early-walk forms) is a separate
question and is not asked here.

## Ev's answer (2026-09-14): (1), the drive-scoped plain memo

"(1) sounds good!" on #2581. Taken as a unit of block SYM-B2 (H /
STRUCTURAL): the plain-form memo keyed by `SymId`, installed by the
drive and dropped with it; the early and door walks per leaf; the
three side effects defined as above (the memo carries its atoms;
`frozen` becomes distinct-nodes-over-the-drive on the drive's receipt,
the per-leaf goldens' column re-blessed as the acceptance's own move;
the opaque-sequence argument pinned across leaves). The assertion
discharge stays a separate question.

## What the drive-scoped plain memo took, and what it left (SYM-7, 2026-09-15)

The volume ask above closes with SYM-7 (`geom_core::sym::DriveMemo`,
`DriveConfig::plain_memo`). What it moved, measured on the box this
item's other readings were taken on:

| document | leaves | schedule | memo off | memo on | ratio |
|---|--:|---|--:|--:|--:|
| slab | 1,280 | sequential | 157.13 s | 78.17 s | 2.01× |
| slab | 1,280 | parallel | 40.12 s | 21.04 s | 1.91× |
| plate | 256 | sequential | 247.47 s | 205.35 s | 1.20× |
| plate | 256 | parallel | 65.01 s | 51.42 s | 1.26× |

Test profile, one take each. In RELEASE both reviewers re-took the slab
at **2.7–3.05×**, which is the number to quote for a shipped build: the
test profile spreads a quarter of the count over glue that release
inlines away, and that glue is in both lanes.

### The ceiling it was sized against

| document | leaves | plain forms | per leaf | distinct ids | ratio |
|---|--:|--:|--:|--:|--:|
| slab (M10-3 chamber, 1,280-leaf budget) | 2,559 | 19,099,919 | 7,464 | 18,833 | 1,014.2× |
| plate (256-leaf budget) | 511 | 9,005,864 | 17,624 | 17,624 | 511.0× |

### Phase 3 — the hash-consing table shared across the drive: MEASURED, NOT TAKEN

`intern` is the largest remaining share of a memo-on slab drive:
**101,014,539 of 297,806,769 Ir inclusive, 33.9 %** (callgrind, release,
8-leaf sequential drive). Almost all of that is `SymNode::id()`, the
128-bit content hash, which a shared table does not remove — the id has
to be computed before any table can be consulted. What sharing could
remove is the TABLE work: `rustc_entry` 10,176,058 + `reserve_rehash`
6,573,305 + `insert_no_grow` 5,286,294 = **22,035,657 Ir, 7.4 % of the
drive** — an upper bound taken before any lock cost, against 21.7 M
interns over a drive each of which would need one. SYM-7's spec gated
the change at **≥ 10 % of wall time**, so it is not taken; both
reviewers accepted the partition. The number stays here rather than in a
merged PR body, which is where the spec said it should live.

### What the memo does NOT help

A drive whose leaves are dominated by the EARLY walk, which stays per
leaf. R2 measured the boss-on-a-derived-frame-over-a-tilted-datum
document at 48 leaves: **275.8 s with the memo on against 276.1 s with
it off** — every leaf refused, the per-node rule A/B reduction is the
cost, and the plain memo saves nothing there. The class the memo helps
is plain-walk-dominated drives, which is what the slab is and what the
plate half is.

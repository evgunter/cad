---
id: symbolic-tier-costs-95-percent-of-the-m10-3-drive
kind: issue
title: The E12 symbolic tier is 95% of the M10-3 interval drive: 20.8x measured, and nothing has profiled inside the normal form
status: open
opened: 2026-09-11
parent: SYM-1
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

Taken 2026-09-13 on the SYM-1 lane's box (4 vCPU / 15 GB, shared with
one other lane; valgrind 3.22.0; the workspace's pinned toolchain).
LOCAL readings, an iteration tool and not a result of record
(`memories/perf-measurement-lane.md`) — but the instrument is in tree
and every number below is a count a hosted re-run reproduces, so a
later unit's before/after is one dispatch away. Instruction shares are
callgrind's and are contention-proof; wall times are the profile's
`Instant` clocks and are not.

**Instrument.** Two halves, as the spec asked. `geom_core::sym::profile`
(behind the test-only cargo feature `sym-profile-testing`, forwarded on
`editor-core`'s dev-dependency edge exactly as `identity-pass-testing`
is; with the feature off none of it compiles) records per op the forms
built and their sizes, every FREEZE with the cause noted at the refusal
site (`Terms` / `Degree` / `Coefficient` / `Overflow` / `Unrecorded`),
the ring's promotions off the `i128` path and the widest coefficient
kept and refused, and per walk (plain / early / door) the forms built
and the wall time; inside the early walk the per-node rule A/B
reduction and rule D's fold, each timed. `valgrind --tool=callgrind`
gives the instruction split by function. The rows are
`crates/editor-core/tests/m10_sym_profile_interval.rs`, all
`#[ignore]`d evidence.

**What the existing counters said first.** `SymCounts::frozen` on the
slab's chamber drive: **0**, at every leaf of 2,559. On the plate at
its nominal: 1,044 (the header's 1,056 was M10-8's reading; the zero
normalisation and A1's folds since moved it). The shape report sizes a
residual but not the population, and neither says WHY a form froze or
what it cost — which is what the profile adds.

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
# callgrind, profile NOT installed (one replay per repeat, nothing else in the process)
CAD_SYM_PROFILE_DOC=slab  CAD_SYM_PROFILE_BOX=nominal CAD_SYM_PROFILE_REPEATS=4 \
  valgrind --tool=callgrind --callgrind-out-file=cg-slab.out \
  $B --ignored --exact m10_sym_profile_interval::sym_profile_callgrind_replay --nocapture --test-threads=1
CAD_SYM_PROFILE_DOC=plate CAD_SYM_PROFILE_BOX=nominal CAD_SYM_PROFILE_REPEATS=1 \
  valgrind --tool=callgrind --callgrind-out-file=cg-plate.out $B --ignored --exact … --test-threads=1
callgrind_annotate --threshold=100 cg-slab.out                  # self Ir per function
callgrind_annotate --threshold=100 --inclusive=yes cg-slab.out  # inclusive Ir per function
```

The self-cost classes below are a partition of every instruction the
process retired, by function name (`num_bigint::*` is the BigInt ring;
`<Int>`/`<Rat>` the `i128` path with its gcd; `collections::btree`
the term maps; `malloc`/`free`/`memcpy` the allocator; `Poly`/`Form`/
`mono_mul`/`within` the merge loops; `form_in`/`intern`/`discharge` the
walk; `hashbrown` the memo maps). A test-profile listing spreads a
third of the count over `core` iterator and `ub_checks` glue that
release inlines away — the ranking is read from release and the test
profile is given beside it because the wall S-TCOST measured is the
test profile's.

### 1. The ranking

Self instruction share of one replay at the nominal, release profile
(test profile in brackets). Slab: 145 M Ir per replay (838 M at
opt-0); plate: 1,312 M (6,839 M).

| class | slab | plate |
|---|--:|--:|
| **term storage** — allocator + memcpy | 34.1 % (9.6) | 30.2 % (9.5) |
| **term storage** — `BTreeMap<Mono, Rat>` walk / insert / clone / drop | 16.8 % (16.5) | 21.6 % (19.4) |
| **term storage** — `Vec<(u128,u32)>` monomials, `Rc<Form>`, clone/drop glue | 6.0 % (6.7) | 5.1 % (9.3) |
| **term storage, total (candidate 3)** | **56.9 %** (32.8) | **56.9 %** (38.2) |
| **the ring** — `Rat`/`Int` on the `i128` path, `from_parts`' gcd and `strip_twos` | 8.4 % (3.6) | 14.5 % (5.2) |
| **the ring** — `num-bigint` | 1.3 % (1.1) | 12.7 % (12.3) |
| **the ring, total (candidate 2)** | **9.7 %** (4.7) | **27.2 %** (17.6) |
| polynomial merge loops (`Poly::mul`/`insert`/`add` self, `within`) | 9.5 % (4.9) | 7.8 % (6.6) |
| the walk — `form_in`, `intern` (node ids + table) | 15.1 % (2.2) | 3.7 % (0.7) |
| memo maps (`hashbrown`) + content hashing (`Hash128`, digests) | 3.0 % (16.6) | 1.0 % (4.4) |
| rules A/B (`algebra`) self | 0.3 % (0.1) | 1.0 % (0.6) |
| rule D (`trig`), rule C (`signed`) | 0.0 % | 0.0 % |
| the numeric channel (`Interval`) | 2.6 % (1.6) | 0.6 % (0.4) |
| `core`/`std` generic glue (iterators, `ub_checks`; opt-0 only) | 0.3 % (28.7) | 0.1 % (22.5) |

Inclusive, release (test): slab — `sign_within` 72.2 % (55.9),
`discharge` 71.1 % (55.3), `plain_form` 55.7 % (42.4), `early_form`
14.4 % (12.3), `intern` 12.6 % (35.3), `Poly::mul` 25.0 % (18.9),
`Poly::insert` 9.7 % (5.4), `Poly::add` 5.8 % (3.1), `Rat::mul` 7.9 %
(5.7), `Rat::add` 1.7 % (1.3), `Rat::from_parts` 6.5 % (5.6),
`reduce_steps` 4.1 % (2.6). Plate — `sign_within` 92.2 % (90.3),
`discharge` 85.2 % (82.9), `plain_form` 11.4 % (10.9), `early_form`
60.3 % (61.3), `door_form` 20.2 % (17.9), `reduce_steps` 52.8 %
(47.1), `Poly::mul` 44.2 % (51.0), `Poly::insert` 19.8 % (19.1),
`Rat::mul` 23.3 % (30.2), `Rat::add` 5.6 % (7.1), `Rat::from_parts`
24.1 % (34.7), `intern` 2.1 % (6.4), `trig::fold` 0.4 % (0.5).

**The ranking, on the slab: (3) term storage and allocation, 57 %;
then the walk's own overhead and the DAG build, 18 %; then (2) the
coefficient ring, 10 %, of which `BigInt` is 1.3 %; (1) degree growth
is not a cost on this document at all.** The slab's forms are TINY:
mean 1.5 terms out (max 10), total degree up to 68, `SymCounts::frozen
= 0` on every one of the drive's 2,559 leaves, no form within a factor
of 400 of the term budget. What the slab pays for is VOLUME times a
fixed cost per form: 10,604 plain forms per replay for 1,490 decisions
(964 theorems, 526 numeric), at 7.6 k instructions per form in
release (`plain_form` inclusive ÷ forms) — one `BTreeMap`, one heap
`Vec` per monomial per term, one `Rc`, one hash of the node, one gcd
per coefficient, for a form that is mostly `q^k` over `sqrt(q²)^j`.
On the plate the same storage share (57 %) sits beside a real ring
share (27 %, `BigInt` 12.7 %), and BOTH ride inside the per-node A/B
reduction (`reduce_steps`, 53 % inclusive), whose substitutions build
the products that freeze on degree — so on the plate (1) is the
freeze MECHANISM and (3)+(2) are the cost of reaching it.

### 2. The two scales, and the whole row

| slab replay | decisions (theorem / numeric) | plain forms | early forms | frozen | wall release (plain / early) | wall test |
|---|---|--:|--:|--:|---|--:|
| nominal (degenerate box) | 964 / 526 | 10,604 | 1,994 | 0 | 64.5 ms (35.9 / 8.4) | 259 ms |
| leaf-sized box, nominal ± ε | 964 / 526 | 10,604 | 1,994 | 0 | 45.6 ms (21.2 / 5.3) | 224 ms |
| root box (±100 ε) | 100 / 86 | 173 | 143 | 0 | 1.4 ms | 9 ms |

The answer is the same at the nominal and over a leaf: over a box
2 ε wide an identity margin's enclosure `[0, c·w]` is still not
definite, so EVERY identity's form is built exactly as at the nominal
— the same 10,604 forms, the same 964 theorems. The root box refuses
at its second node (the extrusion vector straddles zero over ±100 ε),
which is the whole of what "skipped where the numeric channel already
answers" buys on this document: nothing until the box is too wide to
evaluate. The drive over the whole row agrees: 2,559 sessions
(1,280 leaves + 1,279 splits), 21.7 M nodes interned, **19.1 M plain
forms and 3.35 M early forms, 0 frozen**, 7,463 plain forms per box on
average; wall 349 s sequential at opt-0 with the profile installed, of
which the plain walk is 176.8 s, the early walk 41.5 s, `reduce_steps`
7.4 s, `trig::fold` 0.13 s, the top-residual reduce 1.4 s; 33.0 M
`Rat` operations, 418 k on the heap path (1.27 %), 104.6 k promotions
(0.32 %), widest coefficient 201 bits; max terms 10, max degree 68.

### 3. The freeze population

Slab, at every scale and over the whole drive: **none** — no `Terms`,
no `Degree`, no `Coefficient`, no `Overflow`. Plate at its nominal,
1,312 freezes over the three walks (plain 1,044 = `SymCounts::frozen`;
early 164; door 104 — a node the plain walk freezes is frozen again by
each later memo), by cause and op, with the kids' sizes at the freeze:

| cause | op | count | kid total degree min / mean / max | kid terms min / mean / max |
|---|---|--:|---|---|
| `Degree` | `Powi` | 516 | 70 / 96.3 / 117 | 1 / 3.0 / 7 |
| `Degree` | `Mul` | 408 | 40 / 85.9 / 117 | 1 / 2.1 / 4 |
| `Degree` | `Sub` | 108 | 66 / 70.1 / 74 | 1 / 2.0 / 3 |
| `Coefficient` | `Powi` | 232 | 0 / 6.8 / 34 | 2 / 7.0 / 10 |
| `Coefficient` | `Add` | 48 | 4 / 5.3 / 8 | 14 / 37.8 / 90 |
| `Terms`, `Overflow`, `Unrecorded`, `Unnoted` | — | 0 | | |

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
`DEFAULT_SYM_MAX_TERMS`.

### 4. The promotion count

| | `Rat` ops | heap-path `Int` ops | promotions (`Small`→`Big`) | `BigInt` self share, release |
|---|--:|--:|--:|--:|
| slab, one replay | 19,568 | 288 (1.5 %) | 72 (0.37 %) | 1.3 % |
| slab, the whole drive | 33,022,715 | 418,392 (1.27 %) | 104,598 (0.32 %) | — |
| plate, nominal | 232,169 | 21,132 (9.1 %) | 2,967 (1.28 %) | 12.7 % |

The `BigInt` arithmetic's share of the instructions matches its share
of the operations on the slab (1.3 % against 1.5 %) and is ~1.4× it
on the plate (12.7 % against 9.1 %: a heap op costs more, and most of
that 12.7 % is `biguint_shr2` and `sub_assign` — `strip_twos` and the
gcd inside `Rat::from_parts`, run after every product). The `i128`
inline path holds: promotions are under half a percent of the ring's
operations on the slab and just over one percent on the plate.

### 5. Where the walks spend it

Release wall by the profile's clocks (test profile in brackets):

| | plain walk | early walk | door walk | of the early walk: `reduce_steps` | rule D `trig::fold` | top-residual A/B |
|---|---|---|---|---|---|---|
| slab nominal, 64.5 ms (259) | 35.9 ms (130.7) | 8.4 ms (33.1) | — | 3.6 ms (8.1), 1,994 calls | 4 µs, 8 calls | 0.3 ms, 526 calls |
| plate nominal, 338 ms (1,566) | 65.5 ms (244.8) | 150.0 ms (823.8) | 63.3 ms (293.6) | 133.8 ms (625.8), 23,001 calls | 1.2 ms (6.9), 553 calls | 0.6 ms, 610 calls |

On the slab the plain walk is the tier (56 % of the replay's
instructions inclusive; 1,506 calls building 10,604 forms), the early
walk a quarter of it, and rules A/B per node 4 % — the slab has 56
atoms and eight trig nodes. On the plate the early and door walks are
80 % of the replay and `reduce_steps` — rules A/B per node — is 53 %
alone, 89 % of the early walk's wall; rule D's fold is 0.4 %, its
closed forms memoized per argument.

### What a fix would target — one proposal per candidate, the number behind it

Ask 3 (the change) is NOT taken here; these are the next unit's
inputs, each with the count that argues for it, none with a design.

- **Candidate 3, term storage — the largest number on both documents
  (57 % of instructions in release).** A form averages 1.5 terms and
  never exceeds 10 on the slab (90 on the plate); every one is a
  `BTreeMap<Vec<(u128, u32)>, Rat>` — a tree node plus one heap `Vec`
  per monomial, cloned on every `add`, `mul`, `neg` and memo insert,
  and the allocator alone is a third of the count. The proposal: a
  small-vector polynomial — terms as a sorted `Vec` with the monomial
  inline for the one-to-three-indeterminate case — so a form of ten
  terms is one allocation, not eleven. The bound on the win is the
  storage share itself; the merge loops it feeds are 9.5 %.
- **The volume behind it — not one of the three, and the second
  number on the slab.** 10,604 plain forms per leaf for 1,490
  decisions, and the same 8.5 k-node DAG interned afresh in each of
  2,559 sessions (21.7 M `intern`s; `intern` is 12.6 % of a replay in
  release, 35 % at opt-0 where `Hash128::word`'s byte loop is not
  inlined). The plain form reads no value — it is a function of the
  node's content hash alone — so its memo is valid across leaves of
  one drive; today the session "holds nothing across leaves" by
  design (module header, D9 section). The proposal is a drive-scoped
  plain memo keyed by `SymId`, which would remove up to the plain
  walk's 56 % from every leaf after the first; it is a session-model
  change and needs the design conversation before the code.
- **Candidate 2, the ring (10 % slab, 27 % plate).** `Rat::from_parts`
  runs a gcd and two `strip_twos` after EVERY `add` and `mul` (self
  3.5 % slab / 7.5 % plate; the `BigInt` shifts and subtractions
  under it are most of the plate's 12.7 %). Promotions are rare
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
  plate's 1,312 freezes are `Degree` on kids at 40–117, inside the
  per-node A/B reduction that is 53 % of the plate's instructions.
  The fix that targets it is the one
  `derived-frame-placement-freezes-on-the-symbolic-lane` names — a
  degree-resetting atom for a value-exact norm, or normalisation
  simplified before squaring — and that row owns it; the number this
  unit adds is that the term budget is never the wall (no `Terms`
  freeze anywhere; max 90 terms against 4,096) and the coefficient
  bound is a fifth of the freezes (280, widest refused 401 bits).

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

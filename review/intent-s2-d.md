# Review — INTENT stage 2 PR D (#4355), "measure is an operation"

Head `5e99a9ba2` (frozen; branch tip = head at review). Reviewed D's own diff,
`b57e0376a` (B's tip, the merge-base) → `5e99a9ba2`. Correctness lane + style lane
(`docs/prompts/reviewer-style-lane.md`). Confidence words: sure / likely / unsure.

## Verdict: APPROVE-WITH-FIXES

The core of the unit holds. I could not falsify C1, C2 or C4: the observed rule fires
on every edit arm and at load, through definitions, and the rows bite (4 of 5
mutations went red in the unit's own rows). The corpus's assertion verdicts are bit-equal
to B's tip at f64, Interval and Dual. The fixes are: Monte Carlo loses the asserted
value's population, which the spec says must not move (MINOR-1). A legal edit
pointing an assertion at a measure through a redefinition is refused at this head
(MINOR-2). And two of test 13's "breaks if" clauses have no row (MINOR-3, MINOR-4).

## Runs (my own, private `CARGO_TARGET_DIR`, foreground)

- editor-core `--profile ci`: **2857/2857** at head.
- editor-core slow set (the `ci` filter's editor-core clauses, `--profile default`): **63/63**.
  The timing row the PR body reports red passed here.
- Corpus assertion verdicts dumped at B's tip and at head (probe over
  `corpus::documents()`, every `ValuePayload::Assertion`): **identical** at f64,
  Interval (lo/hi) and Dual (value/deriv). `measured_web` is the only corpus document
  with an assertion.
- Probes: a scratch `tests/zz_review_d_probes.rs`, deleted afterwards. P1: an assertion
  value reading a `Body` output refuses (`declared body … reads it as length`). P2: an
  `InsertNode` of an extrude reading an output refuses `ConstructionReadsObserved`.
  P3: a load whose extrude reads a *named definition* over an output refuses
  `ObservedRead`. P4: an assertion whose *bound* is another measure is accepted and holds.
  P5: a Count bound against a Length value refuses `AssertionDimension`. P6 is MINOR-2.
- Mutations, each reverted:
  - M1, dropping the `door` backstop (`edit.rs:5999`): red, test 14.
  - M2, `observed()` without the definition closure: red, tests 13 and 14 and P3.
  - M3, the content key also hashing `r.at`: red, test 15.
  - M4, the `ObservedRead` walk returning `None`: red, test 14 (load arm) and P3.
  - M5, `upstream_of` dropping the measures under an assertion's value: the unit's
    rows stayed **green**. Only `m10_2_measure::an_assertion_over_a_failed_measure_is_poisoned`
    and `m10_4_seed::the_memo_never_serves_one_parameters_pass_to_another` went red (MINOR-3).

## Findings

**MINOR-1 — Monte Carlo has no population for an asserted value; the spec says MC rows
do not move.** `mc.rs` (the `is_measure` branch) still keeps one `McMeasure` row per
`Measure` node. For `distance − r_a − r_b`, the row was the web before D and is the
separation after it. The asserted value has no population at all, only verdict counts.
The spec's §8 table says "Analysis axes, MC draws, stackup rows: —" for every PR. The tour shows the
consequence. `demos/tour/src/mcplate.rs` now checks its bit-for-bit replay against the
separation and computes the web itself. `demos/tour/src/tolerance.rs` `cut_wall`
(`let row = |p: &Plate|`, :309) still asserts "the cut plate's sampled **web** … is the
study's bit for bit", but it now compares the separation. That number does not depend on
either radius draw, so the guard narrowed silently and its comment (:305–308) went stale.
The PR files this as `work/intent/monte-carlo-summarizes-measures-not-asserted-values.md`,
but that file has no program, priority or blocking unit. Shown by reading the diff
and the tour; confidence **sure**. Fix: give the row to the observed variable an
assertion reads, or schedule the item into stage 2 and restore `cut_wall`'s
radius-sensitive check.

**MINOR-2 — a redefinition that makes an existing assertion read a measure is refused
by the roots backstop.** P6: declare `w` (free); insert `Assertion{value: w}`; insert a
measure `m`; then `DefineVar w = m + 0.25 mm`. The result is refused: "product root
Measure … is an ancestor of product root Assertion". This is legal under D10, since an
assertion may read an observed value. `InsertNode` escapes the backstop through root
maintenance, and `DefineVar` has none. Ruling 6 orders `ConstructionReadsObserved`
ahead of the roots backstop, which helps a construction but not an assertion. The bug
should disappear when C deletes `roots.rs`, and the PR says it merges after C. Confidence
**sure** that it happens at this head; **likely** that it goes away with C. Fix: a row
for this case, which must be green once C is merged in.

**MINOR-3 — test 13's "breaks if the definition is bound before its measures run" has
no row in this unit.** Under M5 the assertion no longer depends on its measures, and
`measure_arithmetic_is_a_definition_the_assertion_reads` stays green. That is because
the fixture inserts the measures before the assertion, so the schedule runs them first
anyway. Only two older rows caught M5. Confidence **sure** (shown by execution). A row
needs an assertion that comes before its measures in document order. Today the only way
to get one is MINOR-2's path, so this row and MINOR-2 are tied together.

**MINOR-4 — test 13's "same `AssertionVerdict` bits as pre-D on `tolerance` and
`mcplate`" is not pinned.** No row or tour test compares those verdicts against pre-D
bits. The tour narration only checks that its own replay agrees with itself. I verified
the corpus analogue, `measured_web`, at three lanes, but not the tour stops (I did not
run the tour at both commits). Confidence **sure** that the gap exists; **unsure**
whether any tour bit moved.

**NOTE-1 — C3 holds.** An assertion's value can be re-pointed by only three paths:
re-insertion, `DefineVar` (VR3 fixes the kind) and the refactor remap (the kind is
preserved). So B's slot-door gap has no D analogue. P1 and P5 refuse at the door, and
`assertion_bound_fault` is one predicate shared by the edit and load doors. One soft edge:
`assertion_bound_fault` returns `None` when the value has no dimension. The read check
refuses that case first (P1), but the predicate's own `?` would let it pass.
Confidence **likely**.

**NOTE-2 — C4.** Every re-baselined pin I traced moved for its stated reason. The fence
diff touches only the three id-bearing numbers. The id-masked fence
(`the_corpus_geometry_is_bit_identical_with_ids_masked`) is unchanged in the diff and
green in my ci run. But that fence hashes node outcomes and body points only, **not
measured values or verdicts**, so it cannot back the spec's "measured bits unmoved". My
verdict dump does back it, for the corpus. The `PLATE_LEDGER` +1 is consistent with one
`refuse_non_finite` door: the definition is evaluated through `expr::eval` after the
primitive's own `finite`. I followed that by reading, not by re-measuring. Confidence
**likely**.

**NOTE-3 — an assertion's bound may read an observed variable** (P4: `m1 ≤ m2` holds).
That is consistent with D10, which exempts the assertion. `certified()` looks only at
the value, so a `min_clearance` bound is treated as a full enclosure. Confidence
**unsure** whether that is intended.

## The implementer's rulings

1. **`MeasureExpr` kept as builder input — under-specified, and it leaves two ways
   to say one thing.** The spec both deletes `MeasureExpr`/`MeasureKind` and keeps
   `Node::measure(expr, refs)`, so some authored type was needed. What survives, though,
   is a full second arithmetic language beside `Formula`, with its own AST, `Drop`,
   nesting bound and `lattice`, re-exported to Rust and Python. Its only job is to be
   lowered into a `Formula`. Ev's transcript names exactly this: "many ways to say these
   things". `Measured.value` is already a `Formula` over the outputs. Per-primitive
   builders plus ordinary `Formula` arithmetic would be one way. No retirement is filed.
2. **The builder takes the doc and returns `Measured` — right.**
3. **Python — `Doc.measure`, `Evaluation.reading` and `Node.assertion(value, …)` are
   right; `Node.measure` is wrong in shape.** `Node.measure` takes a `MeasureExpr` and
   raises when it holds arithmetic, so its type admits what the door refuses. It should
   take a `MeasurePrimitive`. As it stands, one lone measure has two spellings.
4. **Stackup subject = any scalar `VarId` — right. MC per `Measure` node — wrong
   against §8:** it leaves asserted values with no population (MINOR-1). Filed but not
   scheduled.
5. **One extra non-finite door — right**, and it is the minimal one (NOTE-2).
6. **One backstop on every arm, ahead of the roots backstop — right** (M1 shows the
   row bites). It does not save the assertion-side case (MINOR-2).
7. **E3/E10/VR4 re-worded as descriptions — right.** VR4's exception carried its own
   sunset ("until stage 2 makes `Measure` an operation"; written in `a76c45b05`).
   E3's mechanism is retired by D10's observed sentence (FORK-5, #4218, Ev). The
   checkout is shallow, so `git log -S` on the E3 sentences only reaches the graft
   (`560bb1f7e`/`4b06b7e95`); I could not find E3's original ratifying commit. One new
   sentence makes a claim rather than a description: E3's "a measure is evaluated where a
   reader needs it" (`docs/ERROR-DESIGN.md:180`). I found nothing that makes evaluation
   lazy. The old text said "lazily evaluable", a capability, not a fact. See S6.

## Style

- **S1 (Q1) `MeasureExpr` duplicates `Formula`'s arithmetic** (`crates/editor-core/src/measure.rs:175`,
  `lattice` :254, `formula`). It is a parallel AST whose only consumer converts it
  into the other one. The PR body's sweep looked for readers of measured *values* and
  could not see this. **likely**.
- **S2 (Q1) `measure::observe` re-implements `Doc::bind_definitions`' per-definition
  body** (`eval/measure.rs:804`, `doc.rs:2157`). Both carry the Count-vs-continuous
  `eval_count`/`eval` split and the anonymous-`written` bookkeeping, and no comment says
  they are kept in step. **sure**.
- **S3 (Q1) one rule with two homes**: `check_node_slots` asks `doc.observed_read(&doc.observed(), node)`
  (`edit.rs:5386`), and `door` asks `new.observed_read_fault()` (`edit.rs:5999`) on
  every edit. Each one recomputes the whole observed set and scans every node, so an
  insert does it twice. **likely**.
- **S4 (Q4/Q7) "observed" is keyed on "an output with a dimension"**, not on "a
  `Measure`'s output" (`doc.rs:1328`). Today only `Measure` has a scalar port, but any
  future scalar port (say a pattern's count) would become observed silently. **unsure**.
- **S5 `Payload` drops the unavailable value without its position** (`eval/mod.rs:523`).
  The doc comment says only "the bound alone when the value reads a measure with no
  value", but a bound-side unavailable drops the bound instead. The content key is fed
  the survivor with no position word, so the two cases hash the same payload words.
  **unsure** whether that can ever collide in practice.
- **S6 (Q5)** E3's "a measure is evaluated where a reader needs it" (`docs/ERROR-DESIGN.md:180`):
  no code makes this true. **unsure**.
- **S7 (Q2) `tolerance.rs:305–330` comment and assert message still say "web"** for a row
  that is the separation (also MINOR-1). **sure**.
- **S8 (Q7) `remap_node`'s assertion arm swallows a `RemapMiss::Read`**
  (`refactor.rs:2372`) and keeps the old `VarId`. That is right for a slot variable, but
  a measure output whose node was not remapped would be carried silently. **unsure**.
- **S9 `upstream_of` collects into a `Vec` mid-chain and immediately iterates it again**
  (`doc.rs:1650`). **sure**, trivial.
- **S10 (Q1) Python has two spellings of one measure**: `Node.measure` and `Doc.measure`
  (`pncad.pyi:2724`, `:4021`). The Rust side has `Recording::measure` and `fn measure`.
  The transcript asks for one. **likely**.
- **Q8**: I read `src/measure.rs` end to end (1055 lines). Its header is accurate about
  what is stored. Its "One lattice, asked rather than restated" section now serves only
  the authored-input language (S1).

## Claims exercised

- C1 by execution: P2, P3, test 14, mutations M1, M2 and M4. I did not trace
  refactor split/inline replay. They go through `apply_replayed` → `door`, so the
  backstop covers them (**likely**).
- C2 by reading plus ci (goldens round-trip; P3 saved and loaded its own bytes).
  `rg` finds no `MeasureExpr` in the stored or wire form. It survives as builder input
  (ruling 1).
- C3 by P1, P5 and reading every re-point path (NOTE-1).
- C4 by the fence diff, my ci run and the verdict dump across commits. The tour was not
  run at both commits.
- C5 by `git log -S` (limited by the shallow checkout).
- C6 by reading `mc.rs`, the tour and the stackup diff.
- C7 by five mutations (M1–M5).
- Not exercised: the Python wheel and its tests, the viewer suites, `demos/tour` runs,
  and the eps rows.

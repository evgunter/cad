# Review r2: INTENT-LITERALS PR D (#4277), "Expr holds no float"

Frozen head `1c47c8984a3664b3f9016c3149299b9d1c0b1725`, base `9d2b780fe`.
Correctness lane plus the style lane (`docs/prompts/reviewer-style-lane.md`).
I read neither the PR's comments nor any other `review/*` branch, and glimpsed none.

## Verdict: APPROVE-WITH-FIXES

Most of the PR holds up under execution. The f64 corpus pins hold. `Ratio` is
exact and bounded. The tokens, the minting order and the load refusals behave
as the spec says, and the new rows go red when I mutate the code they guard.
Two MAJORs must be fixed before merge. Both are small and local, and both were
shown by execution:

1. A logged edit that holds a toleranced dimensionless number loses its
   variable and its distribution on save/load. Nothing refuses.
2. An unspaced `INT/INT` inside a `/` chain changes the chain's
   associativity. Text that used to refuse now evaluates to a wrong value.

## Findings

**MAJOR-1. Save/load silently drops a toleranced dimensionless number** (C5, D9, fail-loud)
- **Where:** `crates/editor-core/src/persist/wire.rs:349` (`WireFormula::Quantity`
  rebuild), with `formula.rs:182` (the `Scalar` branch of `literal_with_unit`)
  and `formula.rs:347` (`carrying`).
- **Mechanism:** `Formula::number(0.5).with_distribution(Normal{σ=0.01})` is a
  `Quantity` leaf, as ruling 1 intends. Its wire form is
  `{"Quantity":{"value":0.5,"dim":"Scalar","unit":"","distribution":{…}}}`.
  The rebuild sends it through `literal_with_unit`, which ruling 1 turns into
  `Ratio(1/2)`. `.carrying(distribution)` then finds no `Quantity` leaf and
  drops the distribution without a word.
- **Shown by execution**, with probes p2 and p3 (in `tests/r2_probes.rs`,
  scratch, not committed):
  - A serde round trip of the formula comes back as `Ratio { num: 1, den: 2 }`.
  - Snapshot = `{w}`, log = `[DeclareVar h := w * number(0.5)±σ]`.
    `save` accepts the file, because its own replay check runs on the
    in-memory edits. `load` succeeds. The live doc has 3 variables, one of
    them `Some(Normal{σ:0.01})`. The loaded `doc` has 2 variables and no
    distribution: `h := w * 1/2`.
- **Consequences:**
  - Replay is not the document: an MC or stackup axis vanishes.
  - Nothing refuses (D2 "never a silent best-effort load").
  - The same path is reachable from a slot root (a lone toleranced scalar) and
    from `Doc::written`/`slot_expansion`, which write such a `Quantity` for a
    toleranced anonymous `Scalar`.
- **Why the suite misses it:** no row round-trips a toleranced number through
  the log.
- Confidence: **sure**.

**MAJOR-2. An unspaced ratio inside a `/` chain re-associates, so previously refused text gets a wrong value** (C6, rulings 2–3)
- **Where:** `crates/editor-core/src/parse.rs:350` (the lexer's `Fraction`
  lookahead) and `parse.rs:997` (`fraction`). The module grammar at `parse.rs:14`
  says `term` is left-assoc.
- **Measured by execution** (probe p1, f64):

  | Text | Read as | Value | Left-assoc value |
  |---|---|---|---|
  | `turn/4/2` | `turn / (4/2)` | π | π/4 |
  | `w/2/3` | `w / (2/3)` | 1.5·w | w/6 |

  With spaces, `turn / 4 / 2` gives π/4.
- **Before D:** both texts refused. `turn` did not exist, and `w / 2` was
  Scalar / Count. Ruling 2 now admits the operands, and ruling 3's token
  silently groups the wrong pair.
- **Related:** `unparse` writes `Div(w, 2/3)` as `w / 2/3`. That text
  round-trips through the parser, but a person reads it as `(w/2)/3`.
- **Untested reading:** `6/2/3` → `3.0 / 3.0` and `1/2/3` → `0.5 / 3.0` happen
  to be right, because there the fraction is the left operand.
- Confidence: **sure**.

**MINOR-1. Ruling 1 makes a number's identity depend on how many digits its shortest decimal has** (C4/VR1, ruling 1)
- **Where:** `formula.rs:563` (`exact_ratio`) and `formula.rs:182`.
- **Shown by execution** (probe p4): two `DeclareVar`s of `w * Formula::scalar(0.25)`
  are `==`; both are `Ratio(1/4)` constants and share one token. Two of
  `w * Formula::scalar(0.1+0.2)` are not: each mints its own anonymous
  variable, so they get distinct tokens.
- **Why it matters:** an API caller passing a computed double inside a formula
  (Python `Formula.literal(math.sin(x))`) gets a constant or a typed variable
  depending on whether the shortest decimal fits in 2^53/10^k. Structure
  evidence (VR8) then flips with the value, which D10 says structure must
  never read.
- **Text door:** the same split happens there (`w * 1e300` mints).
- Confidence: **sure** (mechanism), **likely** (that it matters in practice).

**MINOR-2. A decimal the parser cannot hold exactly is stored as a different exact constant, against the module docs** (C6)
- **Where:** `parse.rs:991` falls back to `Formula::scalar(value)`, which
  re-normalises the double back to a `Ratio`.
- **Shown by execution** (probe p5):
  - `0.1000000000000000055511151231257827` parses to `Ratio(1/10)`.
  - `1.0000000000000000001` parses to `Ratio(1)`.

  Each is an exact constant, but not the number that was written.
- **The docs say otherwise:** `parse.rs:42` says such a decimal "is instead a
  written `Scalar` value, its correctly-rounded double". The f64 bits are
  unchanged. At Interval, though, the enclosure claims exactness for a value
  that was never written.
- **Second instance:** this is the parser's own copy of the "shortest decimal
  is exact" rule (`parse.rs:985`) disagreeing with `exact_ratio` (see Style S1).
- Confidence: **sure**.

**MINOR-3. Asymmetric refusals at the new token boundary** (C6)
- `2/3.5` refuses with `'.' is outside this grammar's alphabet`, which is
  misleading. `2 / 3.5` admits (4/7).
- `1/3` admits and `1 / 3` refuses Count/Count.

  The second pair is documented. The first gives the wrong diagnosis.
- Shown by execution (p1). Confidence: **sure**.

**MINOR-4. Python `Formula.ratio` takes `den: u64`** (C7)
- **Where:** `crates/pncad-py/src/py/expr.rs:271`.
- A negative Python denominator raises a raw `OverflowError` from argument
  extraction, not the documented `LiteralError(constant_out_of_range)`.
- Found by inspection. Not executed (no Python build in this lane).
  Confidence: **likely**.

**NOTE-1. C1, the Interval claim**
- **How checked:** by inspection of the corpus sources
  (`crates/editor-core/tests/corpus/*.rs`).
- **What I found:**
  - Every corpus number is a written quantity at a slot root, an `ang(…)`
    inside `sink.rs:122`'s `h * sin(90°)`, or a `Formula::count`.
  - No bare decimal sits inside a formula, so no non-dyadic `Ratio` reaches
    corpus geometry.
  - The written quantities became untoleranced variables, which bind as their
    exact nominal (§11), the same point the old `Lit` was.
- **Verdict:** the PR's "Interval dump byte-identical" claim is consistent
  with §1. It holds because the corpus has no such constant, not because
  §1's widening is absent.
- **What I ran:** `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
  passed in my full run. I did not re-take the PR's id-free dump.
- Confidence: **likely**.

**NOTE-2. Row 13's Interval assertion cannot see a wrong-direction enclosure**
- **Where:** `intent_literals_d_constants.rs:150` checks `lo < hi` and that the
  *double* `0.1` is inside.
- An enclosure `[0.1d, next_up(0.1d)]` passes it, and that excludes the true
  1/10, which lies below `0.1d`. A row that checks against the true rational
  would close this.
- No Sym row checks that `Ratio(1/10)` folds to the exact rational rather
  than `Rat::of_f64(0.1)`.
- Found by inspection (style Q3). Confidence: **likely**.

**NOTE-3. Row 13's Sym half departs from the spec's text**
- **Where:** `intent_literals_d_constants.rs:210`.
- It compares two *toleranced* `90 deg` parameters and evaluates bare `Expr`s,
  not "two slots".
- Under §11's revised VR8, two **untoleranced** `90 deg` bind as exact
  nominals and would decide Zero too. So the spec's "`90 deg − 90 deg` does
  not" now holds only with a tolerance.
- The adaptation is sound. §8 row 13 should say "toleranced".
- Confidence: **sure**.

**NOTE-4. Two survivors on the stored form** (C2)
- `ExprTree::literal_bits` (`expr.rs:1332`) survives on the stored form, where
  it always pushes nothing.
- `Formula::literal`/`literal_with_unit` survive under their old names, and
  `Formula::number` is a pure alias of `scalar`.
- Each is documented. They are vacuous or duplicate API, not a float in
  `Expr`.
- Otherwise C2 holds:
  - `StoredLeaf` is uninhabited.
  - The `ExprKind` leaves are exactly `Ratio`, `Integer`, `Turn` and `Var`.
  - No authoring door calls `from_f64` into an `Expr`.
  - `GeomPred::DatumDistance.value` is a query-time `Formula`, never persisted.
- Confidence: **sure**.

**NOTE-5. C4 undo and mint-log coverage not shown**
- Retirement is tested (`a_definitions_quantities_mint_first_and_retire_after_it`).
- No D row asserts that the mint log keeps a definition's quantity ids, or
  that undo restores them. C's rows cover only slot-root variables.
- I did not execute this. Confidence: **unsure**.

## The four spec-undecided rulings

1. **A dimensionless value is the bare number its text is.**
   - **At the text door: sound.** It matches VR6 ("a number inside an
     operator tree is a constant"), and VR5 forces it for in-range decimals.
   - **For the Rust and Python API:** it contradicts §1's letter
     ("`Formula::scalar(f64)` writes a Scalar quantity with unit ONE").
   - **Problems:**
     - The out-of-range fallback makes identity value-dependent (MINOR-1).
     - The wire rebuild re-applies it and drops a distribution (MAJOR-1).
   - **No other reading is forced.** The spec's reading (an API scalar is
     always a quantity) is coherent and keeps identity independent of the
     value. It costs only `unparse`'s tree round trip for API-built scalars
     (value bits are unchanged), which §1 already allows "up to identity".
   - **Verdict:** keep it at the text door, but the wire rebuild must not
     re-normalise. I lean to the spec's reading for `Formula::scalar`.
   - Pinned: reverting to the spec's reading (`exact_ratio` → `None`) turns 9
     rows red.
2. **A bare integer beside a non-count reads as a scalar (infix fold only).**
   - **Sound in isolation.** §1 spells a right angle `turn/4`, which F1 would
     refuse without this, and `w * 2` is the natural spelling.
   - It contradicts no ratified text: F1 governs the `Expr` constructors,
     which still refuse.
   - Composed with ruling 3, it is what lets MAJOR-2 through.
3. **Unspaced `INT/INT` is one ratio token.**
   - **Unsound as built:** it breaks the grammar's stated left-associativity
     inside `/` chains (MAJOR-2).
   - A different reading is forced. Either the token only applies where no
     `/` or `*` operator precedes it (so `w/2/3` and `turn/4/2` parse
     left-assoc or refuse), or `unparse` writes a non-terminating constant
     bracketed, `(1/3)`, and the lexer drops the special token. The spacing
     sensitivity (`1/3` versus `1 / 3`) is a smell either way.
4. **`turn` is reserved, refused at the name door and at load.**
   - **Sound**, and forced by §1 ("`turn` is a keyword").
   - The load refusal is the consequence of a name the grammar cannot read
     back.
   - Fine as ruled. No corpus document uses the name.

## Claims exercised

| Claim | How checked | Result |
|---|---|---|
| C1 | `m10_p_fence` and the whole editor-core suite run; corpus read | f64 holds; the Interval dump was not re-taken (NOTE-1) |
| C2 | Inspected | Holds (NOTE-4) |
| C3 | Ran `ratio` unit rows and row 13 | Holds. Mutant: a point division `from_f64(num/den)` turns `a_constant_is_its_exact_value` red |
| C4 | Inspected; ran the D rows | Two typed `5 mm` are two variables; quantities mint pre-order before their definition and are retired with it. Undo is untested (NOTE-5) |
| C5 | Ran D rows and `m4_pr6_refusal` | Row 14 holds. Mutant: `Ratio::reduced` → `Ratio::new` at the rebuild turns `m4_pr6_refusal::dimension_refusals_cross_the_load_door_whole` red. Save/load is NOT bit-exact for a toleranced scalar (MAJOR-1). I did not diff the bless files byte by byte, only read the pin table against the minted ids |
| C6 | Probes p1, p5 | Counterexamples found (MAJOR-2, MINOR-2, MINOR-3). Negative ratios, `-(1/3)`, exponent forms, `6/4` and `0/5` round-trip |
| C7 | New arms exercised by the D rows; Python surface inspected | Python not built or run |
| C8 | Inspected | VS-Q4, the DESIGN row and the module docs move with the code. `parse.rs:42` is false (MINOR-2) and `expr.rs:2234` is stale (S2) |
| C9 | Three mutants rerun (above) | All killed |

**Not exercised:**
- the Python suite and `ty`;
- viewer;
- the ε 1e-6/1e-12 rows;
- the workspace-wide run;
- the tour regeneration.

**My run:** `cargo nextest run -p editor-core --profile default` on the head,
with my scratch probe file present: 2875/2876. The one red was
`every_suite_file_is_aggregated`, caused by my untracked probe file. No other
red, and the `name_words` timing row passed. The 9-red result above is the
ruling-1 mutant (`exact_ratio` → `None`).

## Style

- **S1** — `parse.rs:985-991` vs `formula.rs:563` (`exact_ratio`). The
  "a constant spells it exactly and keeps the bits" rule is written twice.
  The parser applies it to the text and `exact_ratio` to `{value:?}`. The two
  disagree on long decimals (MINOR-2), and the parser's fallback then calls
  the other copy. This is Q1's "two spellings of one rule". **sure**
- **S2** — `expr.rs:2234` and `expr.rs:2245` still cite `ExprKind::Literal`
  ("minted nowhere else") and "a stored literal", which no longer exist. It is
  stale prose in an `unreachable!` justification. Sweep `expr.rs`'s
  `write_quantity` comments for the class. **sure**
- **S3** — `formula.rs:226` `Formula::number` is a body-for-body alias of
  `Formula::scalar`, and `Formula::count` of `Formula::integer`. Two public
  names for one door (Q1, Q7). **likely**
- **S4** — `edit.rs:759` `quantity_unminted` returns `EditError` but only
  panics. The type promises a refusal the code cannot give, and three match
  arms route to it. An invariant held by convention where the type could carry
  it (Q7). **likely**
- **S5** — `with_distribution` on a lone `Ratio` silently changes the leaf
  kind (constant → `Quantity`), and `carrying` silently does nothing on a
  non-quantity. The second is the root of MAJOR-1. A setter that is a no-op on
  the wrong leaf is the shape to look for: `carrying` is the instance; sweep
  the other `kind_mut` users. **sure**
- **S6** — `Doc::unparse` (`doc.rs`, the `written_formula` path) now writes a
  toleranced anonymous variable as its bare value. The text grammar has no
  distribution syntax, so a viewer text edit of a formula that reads one
  re-mints it without its tolerance. This follows from the spec's
  `unparse` rule, but nothing tells the editing surface. **unsure**
- **S7** — The spacing-sensitive grammar (`1/3` versus `1 / 3`, `2/3.5`
  versus `2 / 3.5`) is how I would not have done it. The justification is
  `unparse`'s convenience, and the cost lands on every person who types a
  formula (Q7). **likely**
- **S8** — `work/intent/set-expression-is-still-slot-addressed.md` and
  `explicit-placement-frames-hold-floats.md` are filed with no `program:`. They
  are disclosed deviations from §1, scheduled only as open issues. That is
  acceptable under Q6, but the DESIGN row now says "built (stage 1)" while §1's
  `SetExpression` shape is unbuilt. **likely**
- **S9** — Q8, whole-file read: I read `ratio.rs` and the `parse.rs` and
  `formula.rs` diffs in full, and skimmed `expr.rs` (2285 lines) for stale
  `literal` prose (S2). I did not read `edit.rs` end to end. **sure** (about
  what I did)

Style questions exercised: Q1, Q2, Q3, Q5, Q6, Q7, Q8 (partially). Q4: I
grepped for the deleted `NameLeafWritten`, `UnloweredName` and
`SlotIsNotALiteral` and found no citation left.

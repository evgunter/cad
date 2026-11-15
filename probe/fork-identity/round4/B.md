## For Ev

**Recommendation (likely).** At a symbolic scalar, the question "is this exactly zero?" is answered from the value's expression and never from its number. That question is `is_exact_zero`. The profile's one direction mint branches on it, not on a sign reading.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the *value channel*) carried beside its expression over the document's variables.
- **Plain scalars**: `f64`, `Interval`, and `Dual` (a value with its first derivatives).
- **Decision**: a sign read against a band.
- **Verdict log**: the per-node decisions compared row for row between the `f64` witness build and the certified leaf, and between two builds of an edited document.
- **Valuation**: the parameter point or box a document is evaluated at.

**The question, per scalar: "zero at every point this scalar stands for".**
- **`f64`**: yes where the value is 0.
- **`Interval`**: yes only for `[0, 0]`.
- **`Dual`**: yes only where the value and every derivative are 0. Today it reads the value alone.
- **`Sym`**: the tier's normal form, with every rule that reads a value switched off: the certified read at `min`/`max`/`select`, and the registered-identity door, whose entries are witnessed against values.

Its rules are fixed. Its term budget is the session's, as for every other identity the tier proves. Under these rules the answer is a function of the expression graph, so one document gets one answer at every valuation, eager or deferring. It counts no decision, queues nothing and writes no log row.

Its licence is to choose between two spellings of one real value. It never glues, splits or refuses. No function at a symbolic scalar reads "exactly zero" off the number, so a new caller cannot reintroduce the defect the five corpus rows caught. (likely)

**The mint: one function.**
- **The rule.** Where one component of the vector is zero by the question, store the axis, signed by a sign reading of the other component. Otherwise store the vector over its length and take no reading.
- **Who calls it.** The `line_to` verb calls it and records the director, and that hop's junction checks read the same value. Validation calls it only for a table-authored profile. An arc's radial uses it too.
- **The sign reading is a new fact.** Vertex separation decides the length, not which way the line points.
  - It sits outside the verdict log under its audit row; the re-valuation instrument still records it.
  - If it fails to decide (at the verb no separation has run, and over a box an identically-zero component still has width in its value channel), the escalation propagates as the verb's error. There is no fallback to the quotient.
- **What goes:** `sym::aside` and the readings it discarded.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`:** every symbolic lane stores `(0, 1)` at every valuation. `f64` stores `(0, 1)` either way, to the bit.
- **Hop `(x, 0) → ((x + w) − w, 1)`:** the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same direction to rounding.
- **Corner variables `a`, `b` that agree at the nominal:**
  - `b − a` is not identically zero, so every symbolic valuation stores the quotient. The five corpus rows stay green.
  - At `Dual` the derivative is not zero, so the side keeps its real tilt.
- **`(max(a, b), 0) → (b, 1)`:** the quotient at every valuation.

**The other sites.**
- **`literal_sign`** (a bulge's sign) is a fact about a value. It stays an ordinary recorded decision.
- **`is_exact_zero`'s roughly 24 callers are audited when the change lands.**
  - A read of a literal the kernel wrote gets the same answer as today.
  - A spelling choice on a computed zero gets "no" at a symbolic scalar: the same real value.
  - A branch that changes what is built becomes a margined decision in the same change. The bulge-zero-means-line lowering is one. Until it is converted, the witness and the leaf disagree loudly on such a document.
  - Whether any shipped symbolic-lane result moves is unmeasured. (likely on the rule; unsure per caller)
- **"Was this Zero proven"** (`MarginDiag::proven`, `deferred::discharge`) stays. Its readers are refusal texts, the re-valuation instrument and the unproven-coincidence lint; none steers construction. The lint's "structural" includes a construction's registered identities, which are theorems the construction proved.

**One choice for you, if the designers still differ: does the mint call `is_exact_zero`, or its own query?**
- **One function (recommended, likely).**
  - For: the mint asks exactly the question `is_exact_zero` now answers. Two functions that must agree at every symbolic scalar are two ways of deciding one fact ([one-way]), kept in step by hand.
  - Cost: the `Dual` clause (value and every derivative zero), without which the mint at `Dual` would store a constant axis on a coincidence. The clause also corrects `Dual` for the other callers' "zero" shortcuts.
  - Cost: in the tier-off driver at plain `Interval`, a literal-zero component stores the axis, so that replay stops being byte-exact with the pre-E12 build there.
- **A separate query that answers "not shown" at every plain scalar.**
  - For: the plain lanes take no branch and no sign reading; `Dual` needs no new clause; the tier-off replay stays byte-exact.
  - Against: it is a second function that must agree with `is_exact_zero` at `Sym`.

Either way the mint's stored values are the same: the same bits at `f64`, the quotient at `Dual` on a coincidence, and the same symbolic answers. The choice is only between one function plus a `Dual` clause, and two functions with no clause.

**Ratified text this changes** (yours):
- **ERROR-DESIGN E12** (agent text, approved "lgtm" on #1712) gains: a construction may ask whether a value is zero at every valuation, to choose between spellings of one real value; the answer is not a decision. Code docs claiming a `Sym` run's value channel equals the plain run's bit for bit (`sym.rs`, `drive.rs`) gain "except where a construction chose a closed spelling by structure".
- **DESIGN `[verdicts.consumers]`, "Zero takes the closed form"**: at a symbolic lane, a closed form that drops a dependence on the variables is taken only on identically zero. This applies to every lane choice, with an audit to follow.
- **DESIGN `[scalars]`**: name the question and say what each scalar answers.
- **`crates/profile/README.md`, the direction clause** (your "makes sense", PR 6936): a hop's verb makes its director from its two points by the one mint; only a table's directions are minted at validation.

**Rejected:** PR 6990's spelling; taking the axis rule out; making the value channel exact; a tagged direction type.

**Reversible.** With the `Sym` answer forced to "no", the mint stores the quotient everywhere.

## For the orchestrator

1. **Mint calls the one door: held, and this is the only thing left.**
   - With A's hybrid (`is_exact_zero` at `Sym` by the walk, plain scalars as today), the one remaining difference is whether the mint calls that function or a separate query that agrees with it at `Sym` and says "not shown" at the plain scalars.
   - That is one fact decided by two functions, and [one-way] decides it for one.
   - What A's separate query buys is real but small: no `Dual` clause, no sign reading at plain lanes, and a byte-exact tier-off replay where a literal-zero component would otherwise mint the axis. I weigh it below keeping one function.
   - Not moved. If A holds, it is the one choice stated above.
2. **`Dual`: yes, 1 and 2 are one choice.** The clause is needed exactly when the mint calls the one door, and is unneeded under A's separate query. The other callers' benefit from it is unshown, so I do not argue it independently.
3. **Budget: moved to the session's.**
   - By A: the `max_terms` dial already changes which decisions are proven everywhere, so the query is no special case.
   - No reader sees more than width; C's lint-findings case is findings only.
   - The rules (value reads off) stay fixed. Only the budget follows the session. Converged with A; C said it would not fight.

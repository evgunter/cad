## For Ev

**Recommendation (likely).** The kernel gets one structural question that is not a decision: **"is this value zero at every point this scalar stands for?"** It is `geom_core::is_exact_zero`, redefined. The profile's one direction mint branches on it, and so does every other caller.

**Terms.**
- **Symbolic scalar** (`Sym<T>`): a number (the *value channel*) carried beside its expression over the document's variables.
- **Plain scalars**: `f64`, `Interval`, and `Dual` (a value with its first derivatives).
- **Decision**: a sign read against a band. It is counted and logged, and in a deferring session (one that postpones proofs until a reader asks) it is queued.
- **Valuation**: the parameter point or box a run is evaluated at.

**What each scalar answers.**
- **`f64`**: yes at 0.
- **`Interval`**: yes only at `[0, 0]`.
- **`Dual`**: yes only where the value and every derivative are 0. Today it reads the value alone, so a caller that takes a zero shortcut drops a sensitivity.
- **`Sym`**: yes only where the symbolic tier proves the expression is the zero function. The tier runs its normal form and retry ladder with every rule that reads a value switched off: the certified read at `min`/`max`/`select`, and the registered-identity door, whose entries are witnessed against values. The answer is then the same at every valuation and in eager and deferring sessions. At a symbolic scalar nothing reads "exactly zero" off the number, so no new caller can write the defect the five corpus rows caught.

**What it is not.** It counts as no decision, queues nothing and adds no log row. Its one licence is to choose between two spellings of one real value; it never glues, splits or refuses. The callers are a kept list behind a CI gate.

**The mint: one function.**
- **The rule.** Where one component of the vector is zero by the question, store the axis `(0, ±1)` or `(±1, 0)`. Otherwise store the vector over its length and take no reading.
- **The axis's sign.** It is a new reading of the other component: vertex separation decides the length, not which way the line points.
  - It is kept, and its escalation propagates with no fallback.
  - It stays outside the cross-lane verdict log under its audit row, because the witness and the leaf can take different arms.
  - The re-valuation instrument still sees it.
- **Who calls it.** The `line_to` verb calls it and records the director, and its junction checks read the same value. Validation calls it only for table-authored profiles. An arc's radial uses it too.
- **What goes:** `sym::aside`, the zero-width-band readings and the discarded answers.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`:** the axis at every valuation in every symbolic lane.
- **Hop `(x, 0) → ((x + w) − w, 1)`:** the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same direction to rounding.
- **Corners `a`, `b` that agree at the nominal:**
  - `Sym`: the quotient at every valuation, so the five corpus rows stay green.
  - `Dual`: the quotient, so the derivative is the side's real tilt.
  - `f64`: the two arms are the same bits.
- **`(max(a, b), 0) → (b, 1)`:** the quotient.

**Other sites.**
- **`literal_sign`** stays a recorded decision: a bulge's sign is the arc's sense.
- **Callers of `is_exact_zero` are audited when it is redefined.** A caller that only chooses a spelling keeps the door. A branch that changes what is built becomes a margined decision; the clearest case is the bulge's line-or-arc lowering, which repeats validation's straightness decision. Whether a shipped body or refusal moves is unmeasured, and the corpus run is the test.
- **"Was this Zero proven"** (`MarginDiag::proven`, `deferred::discharge`) stays as it is. Its readers render, record or lint, and none steers construction. The unproven-coincidence lint keeps registered identities, since a construction proved them. One measured run is wanted before ruling.
- **The amended `[verdicts.consumers]` sentence applies to every lane choice in the kernel**, with an audit to follow. At a symbolic scalar, a closed form that drops a dependence on the variables is taken only where the question says yes.

**Choice for you: the question's budget** (the designers split; it changes no body).
- **Fixed kernel-wide (lean).** The question needs a memo of its own anyway, so a fixed budget costs nothing extra. It makes "a function of the document" true across symbolic lanes and sessions. A user's `max_terms` dial then never changes a stored spelling.
- **The session's.** No reader sees more than width: the replay's values are `f64`, and content keys come from the `f64` build. The dial already changes which decisions are proven, so the question would be no special case.

**Ratified text this changes** (yours):
- **ERROR-DESIGN E12** gains the question and its licence. It is agent text approved with "lgtm" on #1712.
- **DESIGN `[scalars]`** names what each scalar answers.
- **`[verdicts.consumers]`**: as above.
- **The profile README's direction clause** (your "makes sense", PR 6936): a hop's verb mints its director, and only a table's are minted at validation.
- **Code docs** that claim a `Sym` value channel is bit-identical to the plain run gain "except where a construction chose a closed spelling by structure".

**Rejected:**
- **PR 6990's spelling:** value first, and decisions taken in order to be discarded.
- **Taking the axis rule out:** the slab's wall turns over the box.
- **Exact value channels:** `h/|h|` still needs a sign.
- **A tagged direction type.**
- **Box-certified reads in the question:** they vary with the valuation and leave no record.
- **A separate query beside `is_exact_zero`** (answering "not shown" at the plain scalars while the two functions agree at `Sym`). It is two spellings of one question that must be kept in step ([one-way]). What it would buy:
  - the tier-off `Interval` replay staying byte-exact pre-E12, but pre-E12 had no mint to reproduce;
  - no sign reading at an exact `f64` zero, but that reading is unlogged and changes no bit.

**Reversible:** yes. With `Sym` answering by the value channel again, the mint reverts to PR 6990's behaviour less its readings.

## For the orchestrator

1. **Mint calls the one door: held, not moved.** A's hybrid ("`is_exact_zero` at `Sym` by the walk, plain scalars as today", with the mint on a separate "not shown" query) is two functions that agree at `Sym`. [one-way] decides it. A's two concrete gains are moot or free:
   - the tier-off replay's byte-exactness concerns a lane that had no mint before E12;
   - the `f64` sign reading at an exact 0 is unlogged and changes no bit.

   If A accepts the mint calling `is_exact_zero`, nothing is left on this point.
2. **`Dual`: one choice with 1, as you suspected, and still needed.** If the mint calls the one door, the clause is what stops a constant axis on a coincidence. It is also not "a mechanism with no user": every existing `is_exact_zero` caller at `Dual` reads the value alone today, and a computed-difference caller drops a sensitivity there. That is real, though I did not trace which of them run at `Dual`. I hold the clause.
3. **Budget: held fixed, weakly. It is the one surviving difference, put to Ev.** I would not block convergence on it. A's point that `max_terms` already moves proofs is fair, and the stakes are spelling only.

**Checked this round:** nothing new in the tree. B's `PINNED_UNDER_GUIDANCE` trace closes my guided-pass question.

**Still open for the lane, not designers':** B's question of whether an `Err` from inside `unlogged` reaches the driver's bisection cue.

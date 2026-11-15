## For Ev

**Recommendation (likely).** Give the kernel one structural question that is not a decision: **"is this value zero at every point this scalar stands for?"** At a symbolic scalar it is answered from the value's expression, never from its number. This question becomes `is_exact_zero`, redefined. The profile's one direction mint branches on it.

**Terms.**
- **Symbolic scalar** (`Sym<T>`): a number (the *value channel*) carried beside its expression over the document's variables.
- **Plain scalars**: `f64`, `Interval`, and `Dual` (a value with its first derivatives).
- **Decision**: a sign read against a band. It is counted, logged, and queued in a deferring session (a session that postpones proofs until a reader asks).
- **Verdict log**: the decisions a build records. It is compared row by row between the `f64` witness build and the certified leaf, and across parameter edits.
- **Valuation**: the parameter point or box a run is evaluated at.

**What each scalar answers.**
- `f64` stands for one point, so it answers yes at 0.
- `Interval` stands for its enclosure, so it answers yes only at `[0, 0]`.
- `Dual` stands for a value and its first-order change, so it answers yes only where the value and every derivative are 0. A side between `W₁` and `W₂` that agree at the nominal therefore stores the quotient, and keeps its real sensitivity.
- `Sym` stands for every valuation, so it answers by the tier's normal form and retry ladder with every rule that reads a value off: the certified read at `min`/`max`/`select`, and the registered-identity door. Its answer is a function of the expression graph, the same at every valuation, in eager and deferring sessions.

**What the question is not.** It counts as no decision, queues nothing and adds no log row. It returns a proof or nothing, with no answer meaning "differs". Its one licence is to choose between two spellings of one real value; it never glues, splits or refuses. The callers are a kept list behind a CI gate, as `is_exact_zero`'s are today.

**The mint: one function.**
- **The rule.** Where one component of the vector is zero by the question, store the axis `(0, ±1)` or `(±1, 0)`. Otherwise store the vector over its length and take no reading.
- **The axis's sign** is a new reading of the other component. Vertex separation decides the length, not which way the line points. It is kept, and it propagates its escalation with no fallback. It stays out of the verdict log under its audit row, because the witness and the leaf can take different arms. The re-valuation instrument still sees it.
- **Who calls it.** The `line_to` verb calls it and records the director, and the hop's junction checks read that same value. Validation calls it only for table-authored profiles. An arc's radial uses it too.
- **What goes:** `sym::aside`, the zero-width-band readings and the discarded answers.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`:** the axis at every valuation in every symbolic lane. `f64` stores `(0, 1)` either way.
- **Hop `(x, 0) → ((x + w) − w, 1)`:** the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`.
- **Corners `a`, `b` that agree at the nominal:** the quotient at every valuation at `Sym` and at `Dual`. At `f64` the two arms are the same bits.
- **`(max(a, b), 0) → (b, 1)`:** the quotient.

**Other sites.**
- **`literal_sign`** stays a recorded decision. A bulge's sign is the arc's sense, a fact about a value.
- **Callers of `is_exact_zero` whose branch changes what is built** become margined decisions, and land together with the redefinition. The bulge's line-or-arc lowering is the clearest case, and it repeats validation's straightness decision. A caller that only chooses a spelling keeps the door and becomes valuation-independent at `Sym`.
- **"Was this Zero proven"** (`MarginDiag::proven`, `deferred::discharge`) stays as it is. Its readers render, record or lint, and none steers construction. The unproven-coincidence lint keeps registered identities: a construction proved them, so they are structural. One measured run is wanted before ruling.

**Scope rules.**
- **The question's rules and budget are fixed kernel-wide**, not taken from the session. The answer is then a function of the document across symbolic lanes too. It costs a memo of its own. (likely; whether it earns its place is the choice below)
- **The amended `[verdicts.consumers]` sentence applies to every lane choice in the kernel**, with an audit to follow. At a symbolic lane, a closed form that drops a dependence on the variables is taken only where the question says yes.

**Choice for you: one door or two.** Both give the mint the same final state. They differ only in how `is_exact_zero`'s other callers are protected.
- **One door, as above (recommended).**
  - A new caller at `Sym` cannot read a coincidence of values, because the definition forbids it, not because an audit does.
  - A caller that misuses it for a structural act is right at `Sym` and reads the bit-equal value at `f64`. So the witness and the leaf disagree, loudly, at the driver's comparison.
  - It fixes `Dual`, where today's `is_exact_zero` drops a derivative wherever a caller takes a zero shortcut.
- **Two doors.** The query answers "not shown" at every plain scalar, and `is_exact_zero` stays, narrowed by audit to reads of literals the kernel wrote.
  - The plain lanes take no branch, which is the simplest thing to say about them.
  - The tier-off driver, a plain-`Interval` run over a box documented as the byte-exact pre-E12 replay, stays byte-exact. Under one door, a literal-zero component there would store the axis.
  - A misuse of `is_exact_zero(a − b)` at `Sym` reads the coincidence the same way at the witness and the leaf: silent, caught only by the gate and review.
- **The day the door changes:**
  - Callers reading a stored literal answer as before at every scalar.
  - Callers reading a computed difference change at `Sym` (from the coincidence to the identity) and at `Dual` (derivatives now count). They are unchanged at `f64` and `Interval`.
  - Whether a shipped body or refusal moves is unmeasured. The three-kind audit has to land with the door, and the corpus run is the test.

**Ratified text this changes** (yours):
- **ERROR-DESIGN E12** gains the question and its licence. It is agent text approved with "lgtm" on #1712.
- **DESIGN `[scalars]`** names the question and what each scalar answers.
- **`[verdicts.consumers]`**: as above.
- **The profile README's direction clause** (your "makes sense", PR 6936): a hop's verb mints its director, and only a table's are minted at validation.
- **Code docs** claiming that a `Sym` value channel is bit-identical to the plain run gain "except where a construction chose a closed spelling by structure".

**Rejected:**
- **PR 6990's spelling:** value first, and decisions taken in order to be discarded.
- **Taking the axis rule out:** the slab's wall turns over the box.
- **Exact value channels:** `h/|h|` still needs a sign.
- **A tagged direction type.**
- **Box-certified reads in the question:** they vary with the valuation and leave no record.

**Reversible.** With the question answering "not shown", the mint stores the quotient everywhere.

## For the orchestrator

**1. One door or two: I moved, to one door with A's `Dual` clause.**
- **What moved me:** the protection argument, plus a point of my own that favours it. Under one door, a structural-act misuse makes the witness and the leaf disagree loudly. Under two, it is silently consistent and wrong ([loud-failure]).
- **My `Dual` objection is answered** by "value and every derivative zero".
- **The log argument is neutral**, as A and B both now say: the sign reading is unlogged under either shape.
- **The disputed fact, settled from B's citation:** plain `Interval` over a box exists as the tier-off driver (`drive.rs` `classify` when the symbolic dial is off). It changes only one thing: under one door, that replay stops being byte-exact pre-E12 where a component is a literal zero. I put that cost in the two-doors column. It does not change the mint.
- **Answer to the question put to both sides:**
  - Under two doors, only the gate and review stop `is_exact_zero(a − b)` at `Sym`.
  - Under one door, the existing callers move at `Sym` and `Dual` only. The audit lands with the door, and any movement would show in the corpus. Unmeasured.
  - The honest summary is "same final state for the mint; the difference is the protection". I prefer protection by definition.

**2. Fixed rules for the query: held "yes", weakly.**
- **Readers checked:** I found no body reader for whom a budget difference is more than width. The doors' unit reads the director at the leaf only, and the witness is `f64`, so it never takes the arm. I found no content key or digest over stored directions, though I did not search exhaustively.
- **The one reader beyond width** is the lint at the coincidence replay (`Sym<f64>`, const `BUDGET`). A director stored as the quotient there and as the axis at the leaf can change which downstream margins prove: findings, not bodies.
- **The case for it:** a hop identity is a tiny form that no realistic budget freezes, so fixing the rules costs almost nothing and buys the cleaner statement, "a function of the document". If Ev would rather not add the memo, I would not fight for it.

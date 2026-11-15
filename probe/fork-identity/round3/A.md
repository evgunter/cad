## For Ev

**Recommendation (likely).** Give the symbolic tier a question that is not a decision: "is this value identically zero in the document's variables?" It is answered from the value's expression and never from its number. The profile's direction mint branches on it instead of on a sign reading.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the value channel) beside its expression over the document's variables. **Plain scalars**: `f64`, `Interval` and `Dual` (a value with its first derivatives), which hold no expression.
- **Decision**: a sign read against a band. **Verdict log**: the per-node list of decisions the kernel compares row for row between the `f64` witness build and the certified leaf, and between two builds of an edited document.
- **Valuation**: the parameter point or box a document is evaluated at. **Deferring session**: one that postpones proofs until a reader asks.

**The query.**
- At a symbolic scalar it is the tier's normal-form test with every rule that reads a value off: the certified read at `min`/`max`/`select`, and the registered-identity door. Its answer is a function of the expression graph, so one document gets one answer at every valuation, in eager and deferring sessions alike.
- At the plain scalars it answers "not shown", so those lanes take no branch.
- It counts as no decision and queues nothing. It returns a proof or nothing; there is no answer meaning "differs".
- Its one licence: to choose between two spellings of one real value. It never glues, splits or refuses. Callers are a short kept list with a gate.
- It runs under the session's own budget. A smaller budget can turn an axis into a quotient, which costs enclosure width and nothing else.

**The mint, one function.**
- Where one component of the vector is identically zero, store the axis, signed by a sign reading of the other component. Otherwise store the vector over its length and read nothing.
- The sign reading is new (vertex separation reads the chord's size, not a component's sign). It sits outside the verdict log under its audit row, because the `f64` witness has no such row; the re-valuation instrument still sees it. If it does not decide, the escalation propagates; there is no fallback to the quotient.
- The `line_to` verb calls the mint and records the director, and that hop's junction checks read the same value. Validation calls it only for a table-authored profile. An arc's radial uses it too.
- `sym::aside` and the discarded readings go.

**Worked example.**
- Slab side `(W, 0) → (W, 1)`: `W − W` is zero as an expression, so every symbolic lane stores `(0, 1)` at every valuation.
- Hop `(x, 0) → ((x + w) − w, 1)`: the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same direction to rounding.
- Two corner variables that agree at the nominal: `b − a` is not identically zero, so the quotient at every valuation. The five corpus rows stay green.
- `(max(a, b), 0) → (b, 1)`: the quotient at every valuation.

**The other sites.**
- `literal_sign` (a bulge's sign) is a fact about a value and is an ordinary recorded decision.
- The unproven-coincidence lint and the other readers of "was this Zero proven" stay as they are. A registered identity is a construction's own theorem and counts as established.
- DESIGN `[verdicts.consumers]`, "Zero takes the closed form", is amended for every lane choice, with an audit to follow: at a symbolic lane, a closed form that drops a dependence on the variables is taken only on identically zero.

**The one choice left for you: how `is_exact_zero`'s other callers are protected.**

The designers now agree on everything the mint does, in every lane. What remains is `geom_core::is_exact_zero`, which about two dozen callers use and which at a symbolic scalar reads the number. That is the same exposure the five corpus rows caught: two different variables that agree at this valuation read as "exactly zero".

- **By audit (B and C).** `is_exact_zero` stays as it is, and its callers are sorted once: a read of a literal the kernel wrote stays; a choice of spelling from a computed value moves to the query; a branch that changes what is built (a bulge of zero lowering to a line) becomes a margined decision. A caller list with a gate keeps it that way. Strongest form: nothing shipped moves on the day; each caller changes when it has been read and understood; the plain lanes and the symbolic lanes keep one meaning for the function.
- **By the scalar (my lean, likely).** The same sorting, and in addition `is_exact_zero` at a symbolic scalar answers from the expression, as the query does, and never from the value channel. At the plain scalars it reads the stored number as today. Strongest form: a new caller that writes `is_exact_zero(a − b)` at a symbolic scalar cannot reintroduce the defect, because there is no longer any function there that reads "exactly zero" off the number. An audit protects the callers that exist; this protects the ones not yet written.
- **What it costs.** On the day it lands, a caller that reads a literal gets the same answer. A caller that reads a computed zero gets "no" at the symbolic leaf where the `f64` witness gets "yes". For a choice of spelling that is the same real value. For a branch that changes what is built it is a loud disagreement between witness and leaf, so those callers must become decisions in the same change. I have not measured whether any shipped document reaches one; the bulge lowering is the one I know of, and no corpus document has a variable bulge.

This is a difference in protection only. The mint, the slab and the corpus rows come out the same under both.

**Ratified text this changes** (yours):
- ERROR-DESIGN E12: a construction may ask whether a value is identically zero, to choose between spellings of one real; the answer is not a decision. (Agent text, approved "lgtm" on #1712.)
- DESIGN `[verdicts.consumers]`: as above. As written it licenses what PR 6963 did.
- DESIGN `[scalars]`: names the structural query; under my lean, also that no reading at a symbolic scalar takes "exactly zero" from the value.
- `crates/profile/README.md`, the direction clause (your "makes sense", PR 6936): a hop's verb makes its director from the two points by the one mint; only a table's are minted at validation.

**Rejected as final states.** PR 6990's spelling; taking the axis rule out; making the value channel exact; a tagged direction type; the plain scalars answering the query from their number.

**Reversible.** The query is additive; with it answering "not shown" the mint is the quotient everywhere.

## For the orchestrator

**1. One door or two: I move, on the part that is about the mint.** The query answers "not shown" at `f64`, `Interval` and `Dual`. By what:
- At `f64` a yes-at-0 buys nothing (same bits), which B and C said and I had conceded in substance.
- B is right on the disputed fact: `drive.rs` runs `Evaluation<Interval>` when `symbolic.enabled` is false (around line 1579), documented as the byte-exact pre-E12 driver. So plain `Interval` over a real box is a production lane behind a dial. It changes nothing: in that lane `W − W` is wide whatever the door says, checked identities refuse across the box anyway, and only literal-equal coordinates would gain an axis.
- The `Dual` clause is then a mechanism with no user.

**What I hold** is the protection, which is separable from the mint and is the honest content of the choice: under two doors nothing but a list stops a new `is_exact_zero(a − b)` at `Sym`. My answer to your question to both sides: I would rather have the scalar enforce it (at `Sym`, `is_exact_zero` answers by the value-free walk; plain scalars unchanged). Put to Ev as the one remaining choice, with B and C's audit-only form stated at its strongest. If B and C accept that hybrid there is no difference left.

On "what does each caller get on the day": a literal read, the same; a spelling choice on a computed zero, "no" at the leaf (same real); a structure-changing branch on a computed zero, witness and leaf disagree loudly. Not measured which shipped rows, if any, move. So the audit comes first under either form; my form only adds the enforcement after it.

**2. Fixed rules and budget kernel-wide: I move to B's withdrawal.** I looked for a reader for whom the difference is more than width and found none: the replay's value channel is `f64`, where axis and quotient differ by rounding; content keys and digests come from the `f64` build; the corpus is one session configuration across valuations; the sign reading is unlogged. And the driver's `max_terms` dial already changes which decisions are proven everywhere, so the query is no special case. The walk with the reads off still needs its own memo.

**Carried, not mine to close:** B's open item (does an escalation taken inside `unlogged` and returned as an `Err` make the driver bisect, or does `classify` read only the log) and C's (does the C6 guided pass refuse a decision it has no entry for; read `profile::PINNED_UNDER_GUIDANCE`). Both need tracing before the mint is built. I did not trace either.

Not checked: the walk with reads off on the corpus's hop identities; the `is_exact_zero` caller audit; one measured run of the lint without registrations.

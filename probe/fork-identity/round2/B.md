## For Ev

**Recommendation (likely).** Give the symbolic tier a question that is not a decision: **"is this value identically zero in the document's variables?"** It is answered from the value's expression, never from its number. The profile's one direction mint branches on that answer.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the *value channel*) carried beside its expression over the document's variables.
- **Point scalars**: `f64`, `Interval`, `Dual`, which carry no expression.
- **Decision**: a sign read against a band. It is counted and logged, and in a deferring session (one that postpones proofs until a reader asks) it is queued.
- **Verdict log**: the decisions a build records, which the box driver compares row by row against the `f64` witness's, and naming compares across parameter edits.
- **Valuation**: the parameter point or box a run is evaluated at.

**The query.**
- **At `Sym`:** the tier's normal form plus its retry ladder, memoised per node under its own rules. Every rule that reads a value is off: the certified read at `min`/`max`/`select`, and the registered-identity door, whose entries are witnessed against values. The answer is then a function of the expression graph alone, which is the same at every valuation and in eager and deferring sessions. (likely)
- **At every point scalar:** "not shown", so the point lanes take no branch. (likely)
- It adds no decision count, no queued entry and no log row. It returns a proof token or nothing, with no answer meaning "differs", so it cannot drive a glue, split or refusal. Its one licence is to choose between two spellings of one real value. The callers are a hand-kept list behind a CI gate. (likely)
- With the tier off (`SymbolicDials::off`), it answers "not shown". The tier-off driver stays the byte-exact pre-E12 replay it is documented to be. (likely)

**The mint: one function, called once where a direction is first made.**
- **The rule.** Where one component of the vector is identically zero, store the axis `(0, ±1)` or `(±1, 0)`. Otherwise store the vector over its length and take no reading.
- **The `line_to` verb** calls the mint, records the director, and its junction decisions read that same value. Today the hop's heading and the stored director are computed separately, so over a box the junction reads a wide value while the segment stores an exact one.
- **Validation** calls the mint only for profiles authored as tables. An arc's radial uses it on the vertex minus the centre.
- **The axis's sign** is a reading of the other component, taken only where the axis is stored, and kept. It is a fact that no other decision answers: vertex separation decides the length, not which way the line points. Two properties of the reading:
  - **Its escalation propagates** to the subdivision driver; there is no fallback. Over a box, an identically-zero component still has value-channel width that can pass the length test while the sign straddles zero, and at the verb the length may not yet be decided.
  - **It stays out of the verdict log**, like today's F43 readings. The `f64` witness never takes it, and a row one lane cannot have breaks the driver's row-for-row comparison. The re-valuation instrument still records it, because it records every symbolic decision.

  (likely)
- The aside, the zero-width-band readings and the discarded answers go.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`:** `W − W` is zero as an expression. Symbolic lanes store `(0, 1)` at every valuation, point or box, eager or deferring. `f64` stores the quotient, which is `(0, 1)` to the bit.
- **`(x, 0) → ((x + w) − w, 1)`:** the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same real number to rounding.
- **A rectangle side between corner variables that agree at the nominal:** `b − a` is not identically zero, so every valuation stores the quotient. This keeps the five corpus rows green.
- **`(max(a, b), 0) → (b, 1)`:** the quotient at every valuation.
- **`Dual` (a value with its derivative):** a side between `W₁` and `W₂` that agree at the nominal stores the quotient. Its derivative is the side's real tilt, not the zero tilt of a constant axis.

**The other sites.**
- **`literal_sign`** reads a bulge's sign: the arc's sense, which a flip must make visible to re-valuation. It is a recorded decision, not a "choice no reader rests on". (likely)
- **`is_exact_zero`** keeps one meaning: this stored value is a literal zero the kernel wrote. Its roughly two dozen callers are audited into three kinds:
  - a read of a stored literal, which stays;
  - a choice of spelling from a computed difference, which moves to the query;
  - a branch that changes what is built, which becomes a margined decision. The bulge's line-or-arc lowering is one: the same class as the five red rows, latent, and a second reading of validation's straightness decision.

  (likely on the rule; unsure per caller)
- **"Was this Zero proven"** (`MarginDiag::proven`, `deferred::discharge`) stays as it is. In production its readers are refusal texts, the re-valuation instrument and the unproven-coincidence lint, and none steers construction once the mint changes. The lint runs at `Sym<f64>`, where the tier never takes the certified read. Its "structural" therefore already means the expression plus registered identities, and a registered identity is a theorem its construction proved over the reals. (likely)

**Ratified text this changes** (design choices, so yours):
- **ERROR-DESIGN E12** gains: a construction may ask the tier whether a value is identically zero, to choose between two spellings of one real value; the answer is not a decision. E12's body is agent text approved with "lgtm" on #1712. Code docs that say a `Sym` run's value channel is the plain run's bit for bit (`sym.rs`, `drive.rs`) gain "except where a construction chose a closed spelling by structure". PR 6990 already breaks this.
- **DESIGN `[verdicts.consumers]`, "Zero takes the closed form":** at a symbolic lane, a closed form that drops a dependence on the variables is taken only on *identically zero*; a Zero read off the value holds at this valuation only. This applies to every lane choice in the kernel, with an audit to follow. As written, the clause licenses what PR 6963 did. (likely on the rule; unsure how many sites)
- **`crates/profile/README.md`, the direction clause** (your "makes sense" on PR 6936): a hop's director is made by its verb from its two points, and this is the one place that divides a vertex difference by its length. Only a table's directions are minted at validation.

**Rejected as final states.**
- **PR 6990's spelling:** value first by design, and it takes decisions only to discard them.
- **Taking the axis rule out:** the slab's wall turns over the box.
- **Making the value channel exact instead of branching:** it costs a normal form per value, and `h/|h|` still needs a sign.
- **A tagged direction type.**
- **One door shared with `is_exact_zero`, answering from the number at the point scalars:**
  - `Dual` would store a constant axis with zero sensitivity where the side really tilts.
  - `f64` would take a reading at one valuation and not the next.
  - It buys a sharper axis only for a literal-zero component in the tier-off driver, which refuses identities across a box anyway.

**Possible later.** A "constant ratio" form, which would make a 45° side with a boxed size exact. Measure on a real document first. (unsure)

**Reversible.** The query is additive and its caller list says where it is used. Removing it reverts the mint to always storing the quotient.

## For the orchestrator

1. **Point scalars, one door or two: moved back to "not shown", by C's `Dual` argument plus the tree.**
   - **`Dual` reaches the mint.** `editor-core/src/eval/mod.rs` has `impl ProfileLane for Dual<T>`. `Dual::sign_within` reads only the value (dual.rs ~868), and `is_exact_zero` is value arithmetic. So under my round-1 shape, `W₁ − W₂` at a nominal coincidence stores a constant axis with a zero tangent.
   - **A principled one-door `Dual` answer exists** ("value and tangent zero", what a `Dual` stands for, first order). But it buys nothing: at `f64` the two arms are equal to the bit.
   - **Plain `Interval` over a real box is a production lane**, the tier-off driver (`drive.rs` `classify`, `Evaluation<Interval>` when `!symbolic.enabled`). C's "only at a point box" is wrong for a component that is a literal 0 in both corners. But that lane is documented as the byte-exact pre-E12 replay, which refuses identities across a box anyway, so the need is not real.
   - **On A's argument (ii)**, the verdict log is the decisive instrument. `k_stats::unlogged`'s doc says the log is compared witness against leaf, row for row, and `diff_verdicts` (resolve, drive) mixes scalars and parameter edits. So any reading the point lanes take at one valuation and not another must be unlogged under either shape, which leaves the point-lane branch with no benefit to pay for it.
2. **The lint: moved to A.**
   - **Checked:** the read is gated on `sess.witness == Some(Witness::Exact)` (sym.rs ~3510), so the lint's `Sym<f64>` deferring lane never takes it. The only change my round-1 version made was dropping registered identities.
   - Those are construction theorems over the reals (profile README: "registered by the construction that proves it"), so [intent.structural]'s "zero as a polynomial" should include them. Dropping them would turn constructed arcs' joints into findings for no reason.
   - Withdrawn. The door stays off for the *query* only because no chord needs it and its entries are witnessed against values.
3. **The sign decision: conceded that it can escalate.** Over a box, `x ≡ 0` still has value-channel width that inflates the length enclosure past the band while `y` straddles zero, and at the verb separation may not have run. So it propagates.
   - **New, not a second reading.** Separation decides the length (`seg.rs` `decide(Gate::VertexSeparation, Margin::of(len))`), not the sign, and no other gate reads `y`'s sign.
   - **Merging the two is not available.** With `x ≡ 0`, a definite sign of `y` implies separation, but not conversely over a box. Posing separation on `y` at symbolic lanes would change a logged row's sign against the `f64` witness's.
   - **Open, not checked:** whether an unlogged escalation that propagates as an `Err` still makes the driver bisect, or whether `classify` reads only the log for its cue. If only the log, this reading needs a logged twin at every lane, or a ruling on how an unlogged escalation reaches the driver.
4. **Scope.**
   - **(a)** Yes, the `[verdicts.consumers]` amendment is general, with an audit (A's).
   - **(b)** Kernel-wide fixed rules for the query: withdrawn. No instrument compares two symbolic lanes' stored spellings. The parity is witness against leaf, and the corpus is one session config across valuations. Different budgets cost width only. A's point stands that the query needs its own memo, cost unmeasured.

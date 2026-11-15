## For Ev

**Recommendation (likely), agreed by all three designers.** The kernel gets one structural question that is not a decision: "is this value zero at every point this scalar stands for?" That becomes the meaning of `is_exact_zero`. At a symbolic scalar it is answered from the value's expression and never from its number. The profile's direction mint branches on it in place of a sign reading.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the value channel) carried beside its expression over the document's variables. **Plain scalars**: `f64`, `Interval`, and `Dual` (a value with its first derivatives).
- **Decision**: a sign read against a band. **Verdict log**: the per-node decisions the kernel compares row for row between the `f64` witness build and the certified leaf, and between two builds of an edited document.
- **Valuation**: the parameter point or box a document is evaluated at. **Deferring session**: one that postpones proofs until a reader asks.

**What each scalar answers.**
- `f64`: yes where the value is 0.
- `Interval`: yes only for `[0, 0]`.
- `Dual`: yes only where the value and every derivative are 0. Today it reads the value alone, which drops a derivative wherever a caller takes a "zero" shortcut.
- `Sym`: yes only where the tier's normal form proves the expression zero, with every rule that reads a value off: the certified read at `min`/`max`/`select`, and the registered-identity door. Its rules and term budget are fixed for the whole kernel, not taken from the session. So the answer is a function of the document alone, at every valuation, in every symbolic lane, eager or deferring.

It counts no decision, queues nothing and writes no log row. Its one licence is to choose between two spellings of one real value. It never glues, splits or refuses. Its callers are a kept list with a gate.

**Why one function.** After the change no function at a symbolic scalar reads "exactly zero" off the number, so the defect the five corpus rows caught cannot be written again by a new caller. A misuse that changes what is built makes the witness and the leaf disagree loudly, where today it is silently consistent and wrong.

**The mint, one function.**
- Where one component of the vector is zero by the door, store the axis, signed by a sign reading of the other component. Otherwise store the vector over its length and read nothing.
- The `line_to` verb calls it and records the director, and that hop's junction checks read the same value. Validation calls it only for a table-authored profile. An arc's radial uses it too.
- The sign reading is a new fact: vertex separation decides the chord's length, not which way the line points. It sits outside the verdict log under its audit row, because the `f64` witness and the leaf can take different arms; the re-valuation instrument still records it. If it does not decide it propagates as the verb's error, with no fallback to the quotient.
- `sym::aside` and the readings it discarded go.

**Worked examples.**
- Slab side `(W, 0) → (W, 1)`: `W − W` is zero as an expression, so every symbolic lane stores `(0, 1)` at every valuation. `f64` stores `(0, 1)` by either arm, to the bit.
- Hop `(x, 0) → ((x + w) − w, 1)`: the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same direction to rounding.
- Corner variables `a`, `b` that agree at the nominal: `b − a` is not identically zero, so every symbolic valuation stores the quotient, and `Dual` keeps the side's real tilt. The five corpus rows stay green.
- `(max(a, b), 0) → (b, 1)`: the quotient at every valuation.

**The other sites.**
- `literal_sign` (a bulge's sign) is a fact about a value and is an ordinary recorded decision.
- `is_exact_zero`'s roughly two dozen other callers are audited in the same change. A read of a literal the kernel wrote gets the same answer as today. A choice of spelling from a computed value changes only where two values agree by coincidence or an identity rounds, and the real value is the same either way. A branch that changes what is built (a bulge of zero lowering to a line) becomes a margined decision. Whether any shipped body or refusal moves is unmeasured.
- The readers of "was this Zero proven" and the unproven-coincidence lint stay as they are. A construction's registered identities count as established.
- DESIGN `[verdicts.consumers]`, "Zero takes the closed form", is amended for every lane choice, with an audit to follow: at a symbolic lane, a closed form that drops a dependence on the variables is taken only where the door says yes.

**What it costs.**
- Every caller's answer at `Sym` and at `Dual` can change the day it lands, so the audit is part of the change, not a follow-up.
- With the symbolic tier switched off by its dial, the driver runs plain `Interval`. There a component that is the literal zero now stores the axis, so that replay is no longer byte-for-byte the pre-E12 one on such a profile. Nothing else in that lane moves.
- The query needs a memo of its own, keyed by its fixed rules. Cost unmeasured; a hop identity is a very small form.

**Ratified text this changes** (yours):
- ERROR-DESIGN E12: a construction may ask whether a value is identically zero, to choose between spellings of one real; the answer is not a decision. (Agent text, approved "lgtm" on #1712.)
- DESIGN `[verdicts.consumers]`: as above. As written it licenses what PR 6963 did.
- DESIGN `[scalars]`: names the structural query and what each scalar answers.
- `crates/profile/README.md`, the direction clause (your "makes sense", PR 6936): a hop's verb makes its director from the two points by the one mint; only a table's are minted at validation.

**Rejected as final states.** PR 6990's spelling (a decision reads the value first by design, and it takes decisions to discard them); taking the axis rule out (the slab's wall turns over the box); making the value channel exact (a normal form per value, and `h/|h|` still needs a sign); a tagged direction type.

**To check before building** (none changes the recommendation): how an escalation taken outside the verdict log reaches the subdivision driver; one measured run of the walk with the reads off on the corpus's hop identities; the per-caller audit.

**Reversible.** With the door answering "no" at `Sym` the mint is the quotient everywhere; the caller list says where it is used.

## For the orchestrator

I move on all three, and there is no choice section left from me.

1. **One function: moved.** Once `is_exact_zero` at `Sym` is the value-free walk, my separate query differs from it only at the plain scalars, and there it buys nothing I can name: at `f64` the arms are the same bits; the sign reading is unlogged either way, and read at the least band it cannot escalate at `f64` unless the component is exactly 0, which separation refuses anyway. Two functions that must agree at `Sym` against one is [one-way]'s case. The one concrete thing the separate query kept, the tier-off replay staying byte-exact on a literal-zero component, I list as a cost and do not think it outweighs that. C's loud-versus-silent point adds to the protection argument I already held.
2. **`Dual`: moved, and yes, 1 and 2 are one choice.** The clause is needed exactly when the mint calls the one door; without it a coincidence at `Dual` stores a constant axis with a zero tangent. Under the one door it also repairs the existing callers' shortcut. My "mechanism with no user" was true only under the separate query.
3. **Budget fixed kernel-wide: moved back to yes.** I still find no body reader who sees more than width, but nothing real separates us: the memo is needed anyway, a hop identity is too small for any realistic budget to freeze, and C names a reader beyond width that I had not weighed (the lint at the replay, where a different stored spelling can change which downstream margins prove; findings only). "A function of the document" is the cleaner statement for the price of a constant.

One note for the build: the mint's sign reading should be taken at the least band (sign only), so that the plain lanes' axis arm introduces no new escalation; I did not check which band B and C assume.

Carried open items, unchanged: the unlogged escalation's route to the driver; the corpus run with reads off; the caller audit; one measured lint run.

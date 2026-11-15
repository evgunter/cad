## For Ev

**Recommendation (likely).** The kernel gets one structural question that is not a decision: **"is this value zero at every point this scalar stands for?"** That question becomes the meaning of `is_exact_zero`. At a symbolic scalar it is answered from the value's expression and never from its number. The profile's direction mint branches on it in place of a sign reading.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the *value channel*) carried beside its expression over the document's variables.
- **Plain scalars**: `f64`, `Interval`, and `Dual` (a value with its first derivatives).
- **Decision**: a sign read against a band.
- **Verdict log**: the per-node decisions the kernel compares row for row between the `f64` witness build and the certified leaf, and between two builds of an edited document.
- **Valuation**: the parameter point or box a document is evaluated at.
- **Deferring session**: a session that postpones proofs until a reader asks.

**What each scalar answers.**
- **`f64`**: yes where the value is 0.
- **`Interval`**: yes only for `[0, 0]`.
- **`Dual`**: yes only where the value and every derivative are 0. Today it reads the value alone, which drops a derivative wherever a caller takes a "zero" shortcut.
- **`Sym`**: yes only where the tier's normal form proves the expression zero, with every rule that reads a value off: the certified read at `min`/`max`/`select`, and the registered-identity door, whose entries are witnessed against values. Its rules and term budget are fixed for the whole kernel, not taken from the session, so the answer is a function of the document alone in every symbolic lane, eager or deferring. (likely)

It counts no decision, queues nothing, and writes no log row. Its one licence is to choose between two spellings of one real value: it never glues, splits or refuses.

**The mint: one function.**
- **The rule.** Where one component of the vector is zero by the door, store the axis, signed by a sign reading of the other component. Otherwise store the vector over its length and read nothing.
- **Callers.** The `line_to` verb calls it and records the director, and that hop's junction checks read the same value. Validation calls it only for a table-authored profile. An arc's radial uses it too.
- **The sign reading** is a new fact: vertex separation decides the length, not which way the line points.
  - It sits outside the verdict log under its audit row, since the `f64` witness and the leaf take different arms on an identity that rounds. The re-valuation instrument still records it.
  - It can fail to decide: at the verb no separation has run yet, and over a box a component that is identically zero still carries width in its value channel. Then it propagates as the verb's error, with no fallback to the quotient.
- **`sym::aside`** and the readings it discarded go.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`.** `W − W` is zero as an expression, so every symbolic lane stores `(0, 1)` at every valuation. `f64` stores `(0, 1)` by either arm, to the bit.
- **Hop `(x, 0) → ((x + w) − w, 1)`.** The axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same direction to rounding.
- **Corner variables `a`, `b` that agree at the nominal.** `b − a` is not identically zero, so every symbolic valuation stores the quotient, and `Dual` keeps the side's real tilt. The five corpus rows stay green.
- **`(max(a, b), 0) → (b, 1)`.** The quotient at every valuation.

**The other sites.**
- **`literal_sign`** (a bulge's sign) is a fact about a value. It stays an ordinary recorded decision. (likely)
- **`is_exact_zero`'s roughly 24 other callers.**
  - At `Sym`, a caller reading a literal the kernel wrote gets the same answer as today.
  - A caller comparing two computed values changes where they agree only by coincidence (yes becomes no) and where an identity rounds (no becomes yes).
  - A caller whose branch changes what is built becomes a margined decision in the same change, or the witness and the leaf disagree on such a document. The bulge-zero-means-line lowering is one.
  - The audit lands with the door. Whether any shipped symbolic-lane body or refusal moves is unmeasured. (likely on the rule; unsure per caller)
- **"Was this Zero proven"** (`MarginDiag::proven`, `deferred::discharge`) stays. Its readers are refusal texts, the re-valuation instrument and the unproven-coincidence lint, and none steers construction. The lint's "structural" includes a construction's registered identities, which are theorems the construction proved. (likely)

**A choice for you: one door or two.** For the mint, the final state is the same either way: identical bits at `f64`, the quotient at `Dual` on a coincidence, and the same symbolic-lane answers. The difference is only in how `is_exact_zero`'s other callers are protected.
- **One door (recommended, likely).** After the change, no function at a symbolic scalar reads "exactly zero" off the number. So the defect the five corpus rows caught cannot be written again by a new caller; that is ruled out by definition rather than policed by review. It is one definition with one meaning per scalar, and it repairs `Dual`. Cost: every caller's behaviour at `Sym` changes the day it lands, so the audit and the structural-act conversions must land with it. And each `is_exact_zero` at `Sym` becomes a memoised form walk, some in hot paths (`pcurve_cache`); this cost is unmeasured.
- **Two doors.** A separate query answers "not shown" at every plain scalar, so the plain lanes take no branch at all. `is_exact_zero` stays, narrowed by audit to reads of literals the kernel wrote. That is the smallest change on the day, and the plain lanes have the simplest story. Against: at `Sym` it still reads the value channel, and only a hand-kept caller list and review stop a new `is_exact_zero(a − b)`. That is the exact defect, writable again.

**Ratified text this changes** (yours):
- **ERROR-DESIGN E12** (agent text, approved "lgtm" on #1712): a construction may ask whether a value is zero at every valuation, to choose between spellings of one real; the answer is not a decision. Code docs claiming a `Sym` run's value channel equals the plain run's bit for bit (`sym.rs`, `drive.rs`) gain "except where a construction chose a closed spelling by structure".
- **DESIGN `[verdicts.consumers]`, "Zero takes the closed form"**: at a symbolic lane, a closed form that drops a dependence on the variables is taken only on identically zero. This applies to every lane choice, with an audit to follow; as written it licenses what PR 6963 did.
- **DESIGN `[scalars]`**: names the structural query and what each scalar answers.
- **`crates/profile/README.md`, the direction clause** (your "makes sense", PR 6936): a hop's verb makes its director from the two points by the one mint; only a table's directions are minted at validation.

**Rejected:** PR 6990's spelling; taking the axis rule out; making the value channel exact; a tagged direction type.

**Reversible.** With the `Sym` answer forced to "no", the mint is the quotient everywhere.

## For the orchestrator

**1. One door: I move to A, and recommend it.**
- What moved me: my round-2 case for two doors rested on the point-lane branch having "nothing to pay for it". All three now agree the sign reading is unlogged under either shape, and A's derivative clause meets the `Dual` objection. So the mint's final state is identical under both shapes. What remains is protecting the other callers, and I prefer protection by definition to protection by audit, per [unrepresentable].
- **Plain `Interval`, settled from the tree.** `drive.rs` `classify` runs `Evaluation<Interval>` when `!symbolic.enabled`, so a profile is evaluated at plain `Interval` over a real box. That path is reachable only through `SymbolicDials::off()`; `Default` sets `enabled: true`. It is a non-default, user-selectable reproduction lane, documented as byte-exact pre-E12. It is real but changes nothing: pre-E12 had no mint to reproduce.
- **C's guided-pass question, checked.** `validate.rs` `PINNED_UNDER_GUIDANCE` pins only `CanonicalOrderX/Y` and `LoopOrientation`. The guide consumes nothing of the mint, so a sign reading that only the symbolic lanes take is neither consumed nor refused there.

**2. Fixed kernel-wide rules: I move to yes, on cost, not on a reader.**
- I found no reader for whom a different budget is more than width. The doors' unit at the leaf and at the replay: a downstream coincidence margin built on `0` or on `(W−W)/L` normalises to the same form. `persist::content_pin` keys the document, not results. `DriveMemo` keys nodes by content.
- But the query needs a memo of its own anyway (its rules differ from the session's), so fixing the budget costs nothing extra. It also makes "a function of the document" true across symbolic lanes, not just within one, and a user's `max_terms` dial no longer changes a stored spelling.

**Not checked:** the cost of form walks at hot `is_exact_zero` sites; the per-caller audit; one measured run of the lint with the registry rung (A's ask).

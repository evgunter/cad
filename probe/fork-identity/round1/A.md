## For Ev

**Recommendation (likely).** Give the symbolic tier a second question that is not a decision: "is this value identically zero in the document's variables?" It is answered from the value's expression alone and never from its number. The profile's direction mint branches on that answer instead of on a sign reading.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the value channel) carried beside its expression over the document's variables. **Plain scalars**: `f64` and `Interval`, which carry no expression.
- **Decision**: a sign read against a band; it is counted, recorded, and in a deferring session (one that postpones proofs until a reader asks) queued.
- **Valuation**: the parameter point or box a document is evaluated at.

**The query.**
- At a symbolic scalar it is the tier's normal-form test, run with every rule that reads a value switched off: the certified read at `min`/`max`/`select`, and the registered-identity door (whose entries are witnessed numerically). The rule set without the reads already exists (`SymRules::without_the_reads`). So the answer is a function of the expression graph alone, which is the same at every valuation. (likely)
- At the plain scalars it answers "not shown". They hold no expression, so they take no branch. (likely)
- It is not a decision: no count in a decision column, no queued entry, no record, and the same call in eager and deferring sessions. (sure)
- It returns a proof token or nothing. There is no answer that means "differs", so it cannot drive a glue, a split or a refusal. Its one licence is to choose between two spellings of one real value. Callers are a short kept list with a gate, and a dial that makes it answer "not shown" everywhere must leave every body's real values unchanged. (likely)

**The mint, one rule, one function.**
- Where one component of the vector is identically zero, store the axis, signed by an ordinary, kept decision on the other component. Otherwise store the vector over its length and read nothing.
- If that sign does not decide, the escalation propagates. There is no fallback to the quotient: with one component identically zero the length is the other component's size, so an undecided sign means the segment may be degenerate on the box.
- The `line_to` verb calls it and records the director, and the junction decisions at that hop read the same value. Today the hop's heading and the stored director are computed separately, so over a box a following tangent reads the wide one while the segment stores the exact one. Validation calls the same function only for a table-authored profile. An arc's radial uses it on the vertex minus the centre.
- `sym::aside`, the zero-width-band readings and the discarded answers leave the mint.

**Worked example.**
- Slab side `(W, 0) → (W, 1)`: `W − W` is zero as an expression. Symbolic lanes store `(0, 1)` at every valuation, point or box, eager or deferring. `f64` stores the quotient, which is `(0, 1)` to the bit.
- Hop `(x, 0) → ((x + w) − w, 1)`: identically zero, so the axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same real direction to rounding.
- A rectangle side between two corner variables that agree at the nominal: `b − a` is not identically zero, so every valuation stores the quotient. The five corpus rows stay green.
- `(max(a, b), 0) → (b, 1)`: not shown without the read, so the quotient at every valuation.

**The other sites.**
- **`literal_sign`** reads a bulge's sign. That is a fact about a value, the arc's sense rests on it, and a flip must be visible to re-valuation. It is a recorded decision, not a "choice no reader rests on". (likely)
- **`is_exact_zero`** keeps one meaning: this stored value is the literal zero the kernel wrote. Its roughly two dozen callers are audited into three kinds: a read of a stored literal stays; a choice of spelling from a computed value moves to the query; a branch that changes what is built (a bulge of zero lowering to a line) becomes a margined decision. The bulge case is the same class as the five red rows, latent, and duplicates validation's straightness decision. (likely; the per-caller sorting is unsure, not done)
- **"Was this Zero proven"** (`MarginDiag::proven`, `deferred::discharge`, read by the unproven-coincidence lint and by certificates) stays as it is. Those readers ask how a decision taken for its own sake was settled. The lint runs at `Sym<f64>`, where the tier never takes the certified read, so its answer already rests on the expression plus the registered identities; removing the registered identities would report every constructed arc's joints as unproven. A `.proven()` read that steers construction, if an audit finds one, moves to the query. (likely)

**Ratified text this changes** (design choices, so yours):
- **ERROR-DESIGN E12** gains a clause: a construction may ask the tier whether a value is identically zero, to choose between two spellings of one real; the answer is not a decision. E12's body was agent-written and approved with "lgtm" on #1712.
- **DESIGN `[verdicts.consumers]`, "Zero takes the closed form"**: at a symbolic lane, a closed form that drops a dependence on the variables (a literal axis) is taken only on identically zero. A Zero read off the value holds at this valuation only. As written, the clause licenses exactly what PR 6963 did, and contradicts E12 there.
- **`crates/profile/README.md`, the direction clause** (your "makes sense" on PR 6936): a hop's director is made by its verb from the two points, the axis where a component is identically zero; the sentence that nothing divides a vertex difference by its length should say this is the one place that does.

**A question the three reports did not ask.** The `[verdicts.consumers]` sentence covers every lane choice in the kernel, not only the profile's. Any of them that, at a symbolic lane, takes a closed form on a value-read Zero and thereby stores a constant has the defect the five corpus rows caught. Is the amended rule meant for all of them, with an audit to follow? I recommend yes. (likely on the rule; unsure how many sites it touches)

**Rejected as final states.** PR 6990's spelling (a decision reads the value first by design, so it cannot be valuation-independent, and it takes decisions to discard them). Taking the axis rule out (sound, but the slab's wall turns over the box). Making the value channel exact instead of branching (a normal form per stored value, and `h/|h|` still needs a sign). One door shared with `is_exact_zero` that answers from the number at plain scalars (the `f64` lane would then take a sign decision at one valuation and not at another, and differently from the symbolic lane replaying it).

**Possible later, not now.** "The direction is identically constant" (the two components in a constant ratio) would also make a 45° side with a boxed size exact; today it stores about `[0.58, 0.86]` for `0.7071`. Allowing box-certified reads into the query is likewise additive. Measure on a real document first. (unsure)

**Reversible.** The query is additive and its caller list says where it is used; removing it reverts the mint to the quotient.

## For the orchestrator

Your reading of the four splits is right. Where I stand now:

1. **Plain scalars and one door: moved to B and C.** B's alternative D answers my argument. Under my shape the `f64` lane takes the axis arm, with its kept sign decision, when a component happens to read 0.0 and not at the next valuation; and it takes it where the `Sym<f64>` replay of the same build does not. That is a lane taking different decisions by a coincidence, which is what the fork is meant to end. What my shape bought, a plain-`Interval` `[0, 0]` component minting the axis, is given up: the certified lane is `Sym<Interval>`, and nothing shown needs plain `Interval` there. Checked: `is_exact_zero` at `Sym` is arithmetic on the value channel (`x·(1/x)` poison), so it cannot be the query; narrowed to stored literals with the three-kind audit.
2. **Box-certified reads: moved to B and C.** Not because the `max` case is wrong (the axis is true on a box where `a < b` throughout) but by the same alignment argument: the read is never asked at an inexact witness (`combine`'s `read` is gated on `Witness::Exact`), so with it on, `Sym<Interval>` and `Sym<f64>` would take different decision sequences for one document. No document needs it, and widening later is additive under the licence. C's point that the read is an unrecorded decision when it settles a decision's Zero is DECIDE's, and I agree it should be recorded.
3. **Where the mint lives: moved to B.** I had flagged the two spellings (`path::chord_director` at `line_to_point`/`line_to_start`, and validation's mint) as off-question; B makes it the design, and it is the README's own rule that the verb holds the direction. Also took B's "the sign's escalation propagates" over my "cannot escalate": at the verb the separation decision may not have been taken yet.
4. **Proven readers: held, against B's change and C's.** Checked rather than argued: the lint's lane is `Sym<f64>` deferring, where the read is already off, so C's "discharge runs the value-free walk" changes nothing there except removing the registry door, and B's version removes it explicitly. Registered identities are what a constructed arc's endpoint facts are proved by; I expect dropping them to turn those joints into findings (inferred, not measured). If a deferring `Sym<Interval>` lane ever ships, revisit.

Adopted from the others: C's proof-token return with no "differs" arm; B's `[verdicts.consumers]` amendment, which I missed and which is the ratified sentence PR 6963 was following.

Still mine and unanswered by either: the query's walk runs under different rules from the session's early walk, so it needs its own memo (as each retry has); cost unmeasured. And the tier's budget and retry settings differ between sessions, so two sessions can answer differently for one document: width only, but a cross-lane comparison must expect it.

Trait placement (`Real` in my report, `Decide` in C's) I do not hold: the mint is `T: Decide` either way.

Not checked: whether the form walk with the reads off still proves the corpus's hop identities (polynomial, so it should); the per-caller audits of `is_exact_zero` and of `.proven()` in `geom-brep`.

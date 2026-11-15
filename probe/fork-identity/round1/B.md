## For Ev

**Recommendation (likely).** The kernel gets one door for a structural question, as distinct from a sign decision: **"is this value zero at every point this scalar stands for?"** It reads no sign and has no band. It counts no decision, leaves nothing to discharge, and is the same call in an eager session and in a deferring one (a session that postpones proofs until a reader asks). That door *is* `is_exact_zero`, redefined. The profile's one direction mint branches on it.

What each scalar stands for, and so what it answers:
- **`f64`** stands for one point: yes where the number is 0.
- **`Interval`** stands for its enclosure: yes only for `[0, 0]`.
- **`Sym<T>`** (a value plus its expression over the document's variables; the scalar of the symbolic lanes) stands for **every valuation of the document**. It answers yes only where the symbolic tier proves the expression is the zero function, and it never reads its value channel. That means no rule that reads a value or depends on the valuation:
  - The tier's certified read at `min`/`max`/`select` (`SymRules::decision_read`) is off.
  - The registered-identity door is off: a registration is witnessed against values, and refused where they disagree.
  - The normal form and the retry ladder (wider ring, fewer rules) stay. Both are fixed by the expression graph.

**Premise check.**
- **No sign reading can answer this.** A decision reads the value first, by E12's ratified order, and asks the tier only where the value does not already read a definite sign. So any mint built on decisions lets the number decide whether the expression is asked. PR 6990's three failures are that order (the 2.8e-17 never asked about; the escalation in a deferring session; the discarded readings). (sure)
- **The axis-or-quotient choice is not a topology decision.** Both arms are the same real direction wherever the axis arm applies; the axis just carries no width. So it needs no verdict and no record. It must only never let a point value pick an arm in a lane that stands for more than that point. (sure)
- **The invariant the item wants is this:** every branch is a function of what the scalar stands for. At `Sym` that means the expression alone, so one document takes one branch at every valuation, in eager and deferring sessions. At `f64` the valuation *is* what the scalar stands for, and nothing re-values it, so branching on an exact 0 there is harmless. The two arms are equal bit for bit, except the sign of a zero. (likely)

**The final state.**
1. **The door.** It is a method of the scalar trait (`Real`); `is_exact_zero` today reads the value channel at `Sym` through `is_poison`. Its contract: the answer may only choose between two spellings of one real value. It never refuses, never glues, never picks cells. A wrong "no" costs width; a "yes" is a theorem.
   - Its callers are a short, hand-kept list, checked by a CI gate, as today.
   - The receipt counts its questions in a column of their own, outside the symbolic and numeric decision columns.
   - A session dial makes the door answer "no" everywhere. Every corpus document must then build the same body with the same real values; that differential is the check that no caller uses it as a decision. (unsure whether the dial earns its place, or whether the list and gate are enough)
2. **The mint.** One function makes the unit direction of a vector, for a line's director and an arc's radial alike.
   - Where one component is zero by the door, the direction is the axis, (0, ±1) or (±1, 0).
   - Its sign comes from one sign decision on the other component, kept and recorded. That decision cannot escalate: the segment's length was already decided positive, and with one component zero the length is the other component's size.
   - Otherwise the direction is the quotient, with no reading taken.
   - `sym::aside`, the least-band readings and the discharge call leave the mint. (likely)
3. **One mint, called where the direction is first made.** The `line_to` verb already normalises its hop for its junction decisions (`path::chord_director`), and validation mints the stored director again from the same two points. Two spellings of one direction mean that over a box the junction reads a wide heading while the segment stores an exact one. The hop's verb therefore calls the one mint, records the result and reads it at its junctions. Validation mints only for profiles authored as tables. (likely)
4. **`literal_sign` differs.** A bulge's sign is a fact about a value (the arc's sense), and the radial identities its lowering registers rest on it. It is a decision, taken once and kept. Its "choice no reader rests on" spelling (`k_stats::choose`) is wrong and becomes an ordinary recorded decision. (likely)
5. **Two kinds of `is_exact_zero` caller.**
   - *Choices of spelling* stay on the door, and become independent of the valuation at `Sym`. Examples: a ray's start, a torus hub, an iso line's held slope.
   - *Branches that change what is built* become margined, recorded decisions. They are the same class as the five red corpus rows, which is a value coincidence deciding structure. The clearest case is the bulge's line-or-arc lowering, which is also a second reading of validation's `segment_straightness`, so the two merge into that one decision. Other candidates: a value sitting exactly on a chart's end, and a zero chord standing in for `+x`.
   - Sorting the roughly 24 callers needs a one-by-one audit. (likely on the rule; unsure on each caller)
6. **The readers of "was this Zero proven."** In production these are three: the refusal texts in `predicate.rs`, the unproven-coincidence lint (`topo::coincidence::Discharge::of`, `editor-core`'s replay via `deferred::discharge`), and the re-valuation instrument. The `geom-brep` reads are all in test modules. None steers construction once the mint stops, so they stay. One change: the lint's question is [intent.structural]'s "zero as a polynomial in the document's variables", so it should be the door's walk, which reads no value, not the decision's walk. The decision's walk may fold a `max` by a fact about this box only. (unsure: this moves findings only, never a body, unmeasured)

**Worked example.**
- The hop `(x, 0) → ((x + w) − w, 1)` at `Sym<f64>`: x's expression is the zero polynomial, so the axis is stored at `(1, 1)` and at `(0.1, 0.3)` alike, with one sign decision on y. `f64` stores (2.8e-17, 1): the same real number to rounding ([simple.conditioned]).
- Corners `a`, `b` that agree at the nominal: `b − a` is not the zero polynomial, so every valuation stores the quotient. The five corpus rows stay green.
- `(max(a, b), 0) → (b, 1)`: the quotient at every valuation and in every symbolic lane.
- The slab side `(W, 0) → (W, H)` over a box, eager or deferring: the axis (0, +1). Nothing escalates, because nothing was decided except H's sign.
- At plain `Interval`, a component that is the literal 0 is `[0, 0]`, so the axis is stored rather than `H/√(H²)`, which would be as wide as the box.

**Ratified text I would change (each yours).**
- **ERROR-DESIGN E12**: a construction may ask whether a value vanishes at every valuation. The answer is not a decision and may only choose between spellings of one real value. The `sym.rs` claim that a `Sym<T>` run is bit-identical to a `T` run (code docs, never ratified, already false under PR 6990) becomes "except where a construction chose a closed spelling by structure". E12 is agent text approved with "lgtm" on #1712.
- **DESIGN [verdicts.consumers]**, *lane choice*: in a symbolic lane, a closed form that drops a dependence on the variables (a literal axis) is taken only where the door says yes, never on a Zero read from the value.
- **DESIGN [scalars]**: name this door as the scalar's one structural query.
- **`crates/profile/README.md`** direction clause (your "makes sense", PR 6936): a direction no verb states is minted once from its points by the one mint. As written the clause forbids that division, and the mint performs it.

**Rejected as final states.**
- **PR 6990's spelling.** Value first, decisions taken in order to be discarded. (sure)
- **Taking the axis rule out.** Sound, but the slab's wall turns over the box, which this program exists to stop. Easy to reverse.
- **Asking the tier before the number at every decision.** It overrules the ratified order and pays the identity test on every decision.
- **Making `Sym`'s value exact as it computes.** It pays a normal form per operation, and it still cannot turn `H/|H|` into ±1 without a sign.
- **A tagged direction type.** Downstream doors would branch on the tag, where literal 0 and ±1 already compute exactly.

**Reversible:** yes. The door's answer at `Sym` can be widened or narrowed rule by rule, and with it answering "no" the mint is the quotient everywhere.

## For the orchestrator

**1. One door, and what the plain scalars answer: I move to A.**
- **What moved me:** A's definition, "zero at every point the scalar stands for". It makes the plain-scalar answers principled rather than a special case. It also makes one door ([one-way]) where I had two that answer identically at `Sym`.
- **My alternative D argued** that `f64` would then branch on a coincidence and take a sign decision at one valuation and not another. That is answered: at a point lane nothing re-values, and the two arms are the same bits. What E12 guards is a constant entering a *symbolic* lane, which the door at `Sym` never does.
- **A's plain-`Interval` case is real** (a literal-0 component over a box stores `H/√(H²)` otherwise). Unmeasured whether any production lane runs plain `Interval` over a box.
- **Checked:** `is_exact_zero` at `Sym` reads the value channel (`Sym::is_poison` returns `self.value.is_poison()`, sym.rs ~5325). An `f64` mint stores (+0, ±1) under A's shape and (±0, ±1) under the quotient.
- **C's split** (stored-literal reads versus comparisons of computed values) is right about the callers. Under one door it becomes my item 5's two kinds rather than two doors.

**2. Box-certified reads: I hold, and A contradicts itself.** A defines `Sym` as standing for every valuation of the document, then lets `Sym<Interval>` fold `max` by a fact about one box. That stores a literal axis in the expression graph where the function varies across boxes. That is exactly the constant E12's re-valuation instrument exists to catch, latent only because the corpus re-values at `Sym<f64>`, where A turns the read off. C's "unrecorded decision" is the same point. For decisions the read stays: it is right for a decision about this box, and the decision's own sign is recorded. A, C and I all exclude the registered-identity door, or would until registrations are shown independent of the valuation.

**3. Where the mint lives: I hold, and A supports it.** A files the two mints (`chord_director`, `mint_director`, plus `ChordFrame::unit`) as off-question and says a tangent arc after an axis hop would inherit the wide heading. That is my argument. The point is one function. Calling it at the verb is what makes the junction and the stored director the same value. C's "one mint at validation" leaves the junction on its own spelling.

**4. Readers of "proven": settled from the tree.**
- **Checked:** every `.proven()` in `geom-brep` (certify.rs ×3, dihedral.rs, pcurve_cache.rs) is inside a `#[cfg(test)]` module. The production readers are `predicate.rs` (rendering), `topo::coincidence::Discharge::of` (lint), `sym/revalue.rs` (instrument), plus `deferred::discharge*` in `editor-core`'s replay (lint).
- So C's "move construction-steering `.proven()` reads to the query" covers an empty set once the mint moves, and A's "stay" is right for rendering.
- The lint question remains. I now agree with C's concrete form (the lint's discharge runs the walk that reads no value), and extend it to `Discharge::of` in eager sessions. That needs the margin to carry whether its proof used a fact about this box. Still unsure; it moves findings only.

**A question none of us asked.** The door's answer at `Sym` depends on the session's budget, rules and ladder (A noted this). The coincidence replay (`Sym<f64>`, deferring) and the driver (`Sym<Interval>`) run different sessions, so one document can store the axis in one and the quotient in the other. The two are equal as real values, so no body differs. But should the door's rules be fixed kernel-wide rather than per session, so that "a function of the document" holds across symbolic lanes too? I lean yes (a fixed rule set for the door, independent of the session's differential dials). Unsure about cost.

**Kept from A and C, off-question:**
- A direction with components in constant ratio (a 45° side) is the general form of the axis. Measure before building.
- `deferred.rs`'s "the decisions are the same either way" is false at an exact witness over a box. DECIDE.

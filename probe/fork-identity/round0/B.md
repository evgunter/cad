## For Ev

**Recommendation (likely).** Stop asking "is this an identity?" by taking a sign reading. Give the symbolic tier a query that is not a decision: **`identically_zero(x)`**, "is `x`'s expression the zero function of the document's variables". It is answered from the expression alone and never reads a number. At `f64`, `Interval` and `Dual` it always answers *no*, because those scalars hold no expression. Then a direction is minted by one rule in one place: **where one component of the vector is identically zero, store the axis (0, ±1) or (±1, 0), with the sign taken from a recorded decision on the other component. Otherwise store the quotient (vector over its length), and take no reading at all.** The bulge's line-or-arc choice is the opposite kind of question and becomes a margined decision. `is_exact_zero` is narrowed to reading values the kernel stored as literals.

Terms:
- **Value channel**: the number (or interval) a symbolic scalar `Sym<T>` carries beside its expression.
- **Form**: the tier's normal form of that expression in the document's variables.
- **Decision / reading**: a sign taken against a band. It is counted, recorded for re-valuation and, in a deferring session, queued.
- **Eager / deferring session**: the tier proves a decision's Zero when the decision is taken (eager), or later when someone asks (deferring).
- **Point lanes**: `f64` and plain `Interval`. **Symbolic lanes**: `Sym<f64>` and `Sym<Interval>`.

**Premise check.**
- **The defect is the question's channel, not how `axis_or` reads proof.** It asks a question about the expression through a reading of the value. A decision is value-first by design: the tier is consulted only where the value channel does not already read a definite sign. That is E12's numeric-first order, and it is sound for decisions. So any mint built on a decision inherits "the value decides whether the expression is asked". PR 6990's probes show that order, seen from a caller that wanted the other question. (sure)
- **The mint reads at `Band::least()`.** That makes it as sensitive to the value as it can be: a rounding residue of 2.8e-17 reads definite. A wider band would hide that case, but not the deferring-session escalation and not the principle. (sure)
- **`max` is not this question's problem.** The tier's *decision read* folds `max(a, b)` to whichever branch a certified comparison on this box shows. That is right for a decision, which is about this box, and the decision's own sign is recorded and flips at another valuation. A query about the whole function must not use the read, so the max case disappears from the mint. (likely)
- **Two places spell one hop's direction today.** `path::chord_director` normalises the hop for the junction decisions at `line_to`, and validation mints the stored director again. That is [one-way]'s "two readings of one fact". Over a box the junction reads the wide quotient while the stored director is the axis. (likely)

**The final state.**
1. **The query.** `identically_zero` is answered by the tier's own algebra: the normal form plus the retry ladder, which changes only the ring width and the rule set.
   - Three rule groups are off, because each reads a value or a witness that depends on the valuation: the decision read at `min`, `max` and `select`, and the registered-identity door, whose entries are witnessed numerically at registration.
   - It is memoised per expression node, and it is the same in eager and deferring sessions.
   - It counts in no decision column, makes no K sample, leaves no deferred entry and writes no re-valuation record.
   - Its answer is a function of the expression graph alone, and the graph is a function of the document and of the decisions already recorded. So re-valuing a document cannot change the answer unless a recorded decision changes first. (likely)
2. **Its one licence.** It may only choose between two spellings of one real number: a closed form that equals the general form wherever the identity holds, which is everywhere. It may never choose cells, refuse or glue. That keeps [intent.geometry] ("a body is the same whatever…") true across lanes, because the point lanes, which answer *no*, build the same real values. It is enforced by a hand-kept list of callers plus a CI gate, as `is_exact_zero`'s list is today. (likely)
3. **The direction mint, one place.**
   - The `line_to` verb mints its hop's director by the rule above and records it, and its junctions read that same director. Validation mints only for table-authored profiles. An arc's radial at each end uses the same rule on `p − centre`.
   - The sign of the non-zero component is a real decision: which way the line points. It is taken at the run band, recorded and kept.
   - If that sign escalates, the escalation propagates and the driver subdivides; there is no fallback to the quotient. With one component identically zero, the chord's length is the other component's absolute value, so an undecided sign means the segment may be degenerate on the box.
   - At the point lanes the mint is the quotient, with no branch. At `f64` that is already the axis exactly wherever a component is exactly 0, apart from the sign of zero.
   - `aside`/`discard` go, and so do the readings that are thrown away. (likely)
4. **`literal_sign`** (a bulge's sign, which signs the lowering's radials) answers a different question: a sign, not an identity. It stays a recorded decision. A tie leaves the radials to validation, whose straightness check refuses such a shallow arc anyway. (likely)
5. **The bulge's line-or-arc lowering** (`profile::bulge_leg`, `lower_to`) decides a *structural* act by `is_exact_zero(bulge)` on the value channel.
   - At `Sym<f64>`, a bulge `b₁ − b₂` whose two variables agree at the nominal lowers to a line, and re-valued it lowers to an arc. That is the same class as the five red corpus rows, and it is latent.
   - It is also a second reading of validation's `segment_straightness`. It should become that one margined decision, with a recorded Zero meaning a line.
   - It must not use the identity query, because a structural act has to follow the values in every lane. (likely)
6. **`is_exact_zero`** keeps one meaning: "this stored value is the literal zero the kernel wrote". Examples are an iso line's held slope (which fork 271's held-line representation retires), a ray's start, and a spline chart's stated end. Each of its 24 callers is audited into one of three kinds:
   - a read of a stored literal, which stays;
   - a choice of spelling from a computed value, which moves to `identically_zero`;
   - a structural act, which becomes a decision. (likely)
7. **`MarginDiag::proven` / `deferred::discharge`** keep answering "why was *this decision's* Zero taken", which the shape report and refusal texts need. The `unproven-coincidence` lint's readers (`topo::coincidence::Discharge::of`, `editor-core`'s replay) ask whether a coincidence is *structural*, which [intent.structural] defines as "zero as a polynomial in the document's variables". So they should ask the test that reads no value, not the decision's stamp, which can include the read that holds only on this box. (unsure: it moves findings only, never a body, and I have not measured which findings would move)

**Worked example** (one rule, five documents, every lane):
- **Slab side `(W,0)→(W,1)`.** x is `W − W`, identically zero. Symbolic lanes store (0, +1) at every valuation, eager or deferring, point or box. Point lanes store the quotient, which is (0, 1) at `f64`.
- **Hop `(x,0)→((x+w)−w, 1)`.** x is identically zero, so it is the axis at (1,1) and at (0.1,0.3) alike. `f64` stores (2.8e-17, 1), the same real number as (0, 1).
- **Rectangle side between corner variables `a`, `b`, with a = b at the nominal.** `b − a` is not identically zero, so every valuation stores the quotient. That keeps the five corpus rows green.
- **`(max(a,b),0)→(b,1)`.** Not identically zero without the read, so the quotient at every valuation.
- **Vertical side `(W,a)→(W,b)`.** The axis (0, sign(b − a)), with the sign a recorded decision. It flips only where the side would reverse, and that flip is recorded.

**Ratified text this changes.**
- **E12**, "Nothing in the kernel changes: `Sym<T>` is one more scalar…". It gains: a construction may ask the tier whether a value is identically zero, to choose a closed spelling of one real. The code's claim that a `Sym<T>` run's value channel is bit-identical to a `T` run (in `sym.rs`, not ratified) is already false under PR 6990. It becomes: the same except where a construction chose a closed spelling by structure. E12 was agent-written text approved with "lgtm" on #1712; its opening sentence is Ev's own. (sure on provenance)
- **[verdicts.consumers]**, *lane choice*, "Zero takes the closed form". In a symbolic lane, a closed form that drops a dependence on the variables (a literal axis) is taken only on *identically zero*. A Zero read from the value is a coincidence that holds at this valuation only (E12). Without this sentence the two clauses contradict each other at a symbolic lane. (likely)

**Alternatives weighed.**
- **A. Keep PR 6990 (proof read off a decision).** Reading the value first is that door's purpose, so it cannot be made independent of the valuation. It also takes decisions only to discard them. Reject. (sure)
- **B. Take the axis rule out.** Sound, with no new query. The cost is that the slab's side is wide over a box, and the sweep doors refuse the wall that BOXFINISH exists to certify. Reject unless the query's cost proves prohibitive. Easy to reverse in either direction.
- **C. Tighten the value channel from the form instead.** A `settle` operation would give a value whose form is a constant that constant's enclosure. It has no branch, and it would also make other stored values exact. But it cannot produce a unit y-component, because `(b − a)/|b − a|` needs a sign. It rewrites the value-channel contract everywhere and costs a normal form per stored value. Worth keeping as a later generalisation if a second value needs it; not this fork. (likely)
- **D. Make `is_exact_zero` itself answer by form at `Sym`** (one door). Tempting, but at `f64` the value is all there is. The mint would then branch on a coincidence there, taking a sign decision at one valuation and not at another. Keeping the point lanes branch-free needs the query to answer *no* where there is no structure. (likely)
- **E. Tag the stored direction** (a variant "axis X/Y, sign" beside "unit vector"). That is [unrepresentable]'s tagged encoding, but downstream doors would then branch on the tag. The literal 0 and ±1 values already compute exactly, and a tag could not come from `f64` structure ([scalars]' "f64 structure + T payload"). Reject. (likely)

**Reversibility:** the query is additive, and its list of callers says where it is used. Rolling back is B.

## For the orchestrator

- **Not checked:**
  - Whether any test compares `f64` against `Sym<f64>` stored values bit for bit. The rounding-identity hop would differ.
  - The cost of value-free forms in deferring sessions on large documents.
  - Whether `signed::order` (the decision read) is the only rule that reads a value. I found it gated by `SymRules::decision_read` at `min`/`max`/`select`; `manifest`'s rule F reads syntax only.
  - Whether excluding the registered-identity door loses a mint the doors need. For a chord between authored points I expect none, since a constructed arc's radials are recorded by its verb.
- **Assumed:** the sweep doors need only exact literal values, not a tag. The brief says the doors' unit reads the eager `Sym<Interval>` lane, where PR 6990 holds.
- **Off-question, worth filing:**
  1. `bulge_leg`/`lower_to` decide line or arc (a structural act) by bit equality, duplicating `segment_straightness`. Profile, P2, design. Same class as the corpus rows; no corpus document has a variable bulge.
  2. `path::chord_director` and the validation mint are two spellings of a hop's director. `chord_director`'s zero-chord `+x` stand-in (`is_exact_zero(witness)`) stores a constant at `Sym` where a value coincidence would vary. Low risk: vertex separation refuses the zero hop later.
  3. `deferred.rs` says "deferring changes which sign an escalation reads and nothing else". But an identity that escalates is a Zero when eager and an escalation when deferring, so the two sessions branch differently on it. The review's `Sym<Interval>` probe is this. It belongs to DECIDE with the general row.
  4. The 24 `is_exact_zero` callers need the three-kind audit. The iso-line readers (`row_of`, `iso_image_row`, `patch.rs`) fall to fork 271's representation.
- **Relation to the two rows:**
  - This answers DECIDE's row: the query, its licence, how it relates to `is_exact_zero`, and the plain-`Interval` `[0,0]` bullet (point lanes answer *no*, by design).
  - The BOXFINISH row's "closed when" holds under it, and the two review probes become ordinary green rows at the symbolic lanes.
  - `sym11` re-baselines: two fewer readings per direction minted as a quotient, one more sign decision per axis.

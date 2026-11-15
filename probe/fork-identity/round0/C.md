## For Ev

**Recommendation (likely).** Give the symbolic tier a second question that is not a decision: "is this value identically zero in the document's variables?" It is answered from the expression alone and never by reading a value. Have the one direction mint branch on that answer instead of a sign reading. Terms used throughout:
- **Symbolic tier**: E12's check that an expression's polynomial normal form, built from the parameter symbols, is zero.
- **Lane**: a scalar the kernel runs at (`f64`, `Interval`, `Sym<f64>`, `Sym<Interval>`), each in an eager or a deferring session.
- **Valuation**: the parameter point or box the document is evaluated at.

The query is:
- `Decide::identity(x) -> Option<Identity>`. `Identity` is a proof token that only the tier can create. The answer is `None` at `f64`, `Interval` and `Dual`. At `Sym` it is the tier's normal-form walk, memoised per node.
- Never a decision. It adds no decision count and leaves no deferred entry. It runs immediately inside a deferring session too, because deferring postpones the discharge of decisions, and this is not one.
- Run with every rule that reads a value of the box switched off. Today the walk takes a certified read at `min`/`max` (`SymRules::decision_read`). Without that read, the answer depends only on the expression graph (the DAG), which is the same at every valuation.
- Typed so it cannot be read as "nonzero". `None` means "not shown". There is no arm that says "differs", so the answer cannot drive a glue or a split. It may only choose between two spellings of one real function: a *lane choice* in `[verdicts.consumers]`.

The mint becomes: if the chord's x is identically zero, store `(0, sign(y))`; the same with the components swapped; otherwise store `w/|w|`. The sign is a real decision and is kept. Once one component is identically zero, the vertex-separation decision already taken means the sign reads definite. No reading is taken only to be thrown away.

**Premise check.**
1. **The defect is the question, not the mint (sure).** "Is this identity" is a fact about structure. The kernel can only reach it through `Sym::sign_within`, which is built for decisions:
   - It reads the value first, and a definite value short-circuits the form. At an inexact witness (`f64`) the numeric answer wins even against a zero form (counted as "disputed").
   - It counts the reading.
   - It defers it inside a deferring session.

   Every one of the three failures the review found is one of those properties leaking through: the `2.8e-17` that is never asked about, the escalation in the deferring session, and the discarded readings. No spelling built on sign readings can be valuation-independent. PR 6990 is the best one possible, and it still is not.
2. **The demand is per lane, not across lanes (likely).** `f64` has no expression graph, so it cannot know that `(x+w)−w−x` is zero. It stores `(2.8e-17, 1)/L`, while `Sym<f64>` stores `(0, 1)`. That is rounding noise, which `[simple.conditioned]` treats as the same real number. Within each lane the branch is now a function of the document alone. Proof is checked by one tier walk, so it is identical in eager and deferring sessions.
3. **Making the value exact instead of branching ("settle") does not cover the case (sure).** The alternative is to replace a `Sym` value by its exact constant wherever the normal form is constant, so the mint needs no branch. A slab side `(W, 0)→(W, H)` with `H` boxed shows why that fails:
   - Settling the chord gives `(0, H)`, but `H/√(H²)` still encloses `[H_lo/H_hi, H_hi/H_lo]`.
   - Only the axis form (an identity, plus a sign decision) gives `(0, ±1)`.

**The three sites.**
- **Line director and arc radial (sure):** the same question. Both go through the one mint, `axis_or`, with the query in place of the readings.
- **`literal_sign` is different (likely).** It reads the sign of a bulge, a quantity the author wrote. A flip there is a real change of the arc's sense, and the re-valuation recorder should see it. It stays a recorded choice.

**Ratified text I would change.**
- **E12.** It says: "Nothing in the kernel changes … no funnel site is edited", and only `sign_within` behaves differently. `sym.rs` also promises that the value channel matches the plain `T` run bit for bit. Both need an amendment along these lines: "the tier also answers a construction's structural query, which may only choose between two spellings of one function, never decide". Provenance: E12 was agent-written text that Ev approved with "lgtm" on #1712. The sentences in `sym.rs` are in code docs and were never ratified. PR 6990 already breaks the bit-identity in effect.

**What becomes of the other "exactly zero?" and "proven?" readers.**
- **`geom_core::is_exact_zero` has two meanings (likely).** Split them:
  1. *Reading a record*: a field a construction set to a literal zero (an iso-line's slope, a stored bulge). That reads the same at every valuation and lane. Keep it.
  2. *Comparing two computed values*: `x − lo`, `w0 − w2`, `p.x − first.x`, and others in `geom-brep`, `topo` and `geom`. This is PR 6963's defect in another form: it reads equal where two variables agree at this valuation, and at `Dual` it turns a sensitivity into a constant. Those callers should use the structural query if they are lane choices, or a decision if they are not.
- **`MarginDiag::proven` / `Discharge::of` in `topo`, and `discharge_or_why` in `editor-core`'s replay (likely): keep them.** They report how a decision that was taken for its own sake was discharged. That is the unproven-coincidence lint's question, and it is asked after the decision, not instead of it. `deferred::discharge` should run the same value-free walk as the query, so that "structurally one" (`[intent.structural]`: zero as a polynomial in the document's variables) means one thing everywhere. A `.proven()` read that steers construction has the mint's exposure and should move to the query.
- **The `min`/`max` read is an unrecorded decision (likely).** Inside the tier it is literally `select(B − A, A, B)`. When it is used for a decision's Zero, it should be recorded as a decision, so that re-valuation sees the flip. That belongs to the DECIDE row.

**Final state.** One door for structural identity, one for decisions, and a type that keeps them apart. The mint takes the same branch at every valuation, in every lane and session. A constant is stored only where the tier proved it. Dropped readings and stray deferred entries are gone.

Possible later: the same query, generalised to "the chord's direction is identically constant" (a constant ratio of its components), would also make a chamfer side such as `(W, 0)→(W+c, c)` exact. That should be measured on a real document before it is built (unsure).

Reversible: yes. The query is additive, and removing it reverts the mint to always taking the quotient.

## For the orchestrator

- **Root cause of the `Sym<f64>` probe.** `axis_or` reads at `Band::least()`. At a K·ε band, `2.8e-17` would have been in band and the tier would have been asked. The query removes the least-band chord readings altogether.
- **Not checked:** whether the form walk with `decision_read` off still proves the corpus's hop identities. It should, since they are polynomial. The lane should measure it on the five `revalue_corpus` rows and the two review probes (`80ffc4c5`).
- **Found off-question: `.proven()` readers outside `topo`/`editor-core`.** `geom-brep/src/certify.rs` (two `Refused::Zero … if margin.proven()`, one `shown.proven()`), `dihedral.rs` and `pcurve_cache.rs:~15869`. If any of them steers construction rather than reporting, it has the mint's exposure. Worth an audit row under DECIDE.
- **`is_exact_zero` on a difference:** `topo/boolean/spline_known.rs:118`, `geom-brep/description.rs:466`, `pcurve_cache.rs:8655`, `8676`, `9184`, `patch.rs:86`. Also `profile/lib.rs:310`, `330` and `529` on a bulge that may be computed. Candidates for one issue: "is_exact_zero compares computed values".
- **Assumed:** the DAG is a function of the document plus the decisions taken, so a value-free walk is valuation-independent. Float literals are keyed by their bits; I did not audit for any value-keyed node minting.
- **Budget freezing and SymRetry:** both depend only on form size and ring guards, which are fixed by the DAG. Unverified beyond reading the docs.

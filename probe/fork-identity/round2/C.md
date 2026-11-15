## For Ev

**Recommendation (likely).** Stop asking "is this an identity?" through a sign reading. Give the symbolic tier a query that is not a decision: `identically_zero(x)`, meaning "is `x`'s expression the zero function of the document's variables?". The profile's one direction mint branches on it.

Terms:
- **Symbolic lanes**: `Sym<f64>` and `Sym<Interval>`, which carry a value plus its expression over the document's variables.
- **Point lanes**: `f64`, `Interval` and `Dual` (value plus a sensitivity), which carry no expression.
- **Decision**: a sign read against a band. It is counted, logged, and queued in a deferring session (a session that postpones proofs until a reader asks).
- **Valuation**: the parameter point or box a run is evaluated at.

**The query.**
- At a symbolic lane it runs the tier's normal form and retry ladder with every rule that reads a value switched off: the certified read at `min`/`max`/`select`, and the registered-identity door, whose entries are witnessed against values.
- Its rule set and budget are fixed kernel-wide rather than taken from the session. The answer is then a function of the document across symbolic lanes and sessions, not only across valuations.
- It is not a decision: no count, no deferred entry, no log row, and the same answer in eager and deferring sessions.
- It returns a proof token or nothing, with no arm meaning "differs". Its one licence is to choose between two spellings of one real value; it never glues, splits or refuses. Its callers are a kept list with a gate.
- At the point lanes it answers "not shown".

**The mint: one function.** The `line_to` verb calls it and records the director, and its junction decisions read that same value. Validation calls it only for profiles authored as tables. An arc's radial uses it on the vertex minus the centre.
- **One component identically zero:** store the axis, signed by a decision on the other component. That decision is kept. It is a different fact from the vertex-separation decision, which bounds the length and not which way the segment points. It is taken only in the symbolic lanes, so it is taken outside the cross-lane verdict log, under its audit row. That is the existing door, `k_stats::unlogged`, for a reading "that would put rows in one lane's vector that the other lane cannot have". If it escalates, the node refuses; there is no fallback to the quotient.
- **Otherwise:** the vector over its length, with no reading taken.
- The aside and the discarded readings go.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`:** the axis at every valuation, point or box, eager or deferring. `f64` stores `(0, 1)` bit for bit.
- **Hop `(x, 0) → ((x + w) − w, 1)`:** the axis at both valuations the review tried. `f64` stores `(2.8e-17, 1)`, the same real direction to rounding.
- **Rectangle side between corner variables `a`, `b` that agree at the nominal:** the quotient everywhere, so the five corpus rows stay green.
- **`(max(a, b), 0) → (b, 1)`:** the quotient everywhere.

**Other sites.**
- **`literal_sign`** reads a bulge's sign, a fact about a value that the arc's sense rests on. It stays a recorded decision.
- **`is_exact_zero`** keeps one meaning: a zero the kernel wrote as a literal. Callers that apply it to a difference of computed values are audited:
  - one that only chooses a spelling moves to the query;
  - one that changes what is built becomes a margined decision. The bulge's line-or-arc lowering is the clearest case, and it repeats validation's straightness check.
- **The readers of "was this Zero proven"** render text, record how a coincidence was discharged, or feed the re-valuation instrument. None steers construction once the mint stops, so they stay. That includes the unproven-coincidence lint's use of registered identities. A construction that registers an identity has proved it ("constructing them as one, which proves it", `[intent.statements]`), so it is structural for the lint even though the query does not use it.

**Ratified text this changes.**
- **ERROR-DESIGN E12** gains a clause: a construction may ask the tier whether a value is identically zero, to choose a closed spelling of one real. The answer is not a decision. E12 was agent text approved with "lgtm" on #1712.
- **DESIGN `[verdicts.consumers]`, "Zero takes the closed form."** At a symbolic lane, a closed form that drops a dependence on the variables is taken only on *identically zero*. This applies to every lane choice in the kernel, with an audit of them to follow. As written, the clause licenses what PR 6963 did.
- **`crates/profile/README.md`, the direction clause.** A verb records the directions it makes, a `line_to` hop's included. Only a table's are minted, at validation, by the one mint.
- **The `sym.rs` code-doc claim (not ratified)** that a `Sym<T>` value channel matches a plain `T` run bit for bit gains that exception.

**One choice still for you: what the point lanes answer.**
- **(a) "Not shown", and the query is its own door (recommended, likely).**
  - At `Dual` the profile does run (`ProfileLane for Dual`, so "a Dual seed on a profile dimension carries its tangent"). `Dual`'s exact-zero test reads only the value channel.
  - So under (b), a side between `W₁` and `W₂` that agree at the nominal would store a constant axis. Its sensitivity to `W₁` would be zero while the real side tilts: a wrong number in the error analysis.
  - The point lanes are also re-valued: `diff_verdicts` diffs two `f64` runs across a parameter edit. Under (b), `f64` takes the axis arm at one valuation and the quotient at the next on a coincidence of values.
  - What (a) gives up: plain `Interval` over a box, where a literal-0 coordinate at both ends gives `[0, 0]`, stores the wide `H/√(H²)`. The box-certifying lane is `Sym<Interval>`, which stores the axis there.
- **(b) One door: `is_exact_zero` redefined as "zero at every point this scalar stands for".** `f64` answers yes at 0, `Interval` yes at `[0, 0]`, `Sym` by the value-free walk.
  - For it: [one-way] (one structural question, one door). Nothing it stores differs from (a) at `f64`. Plain `Interval` gets the exact axis in the literal-zero case.
  - Against it: the `Dual` sensitivity above. And at a point box, `Interval`'s `[0, 0]` is the same coincidence of values `f64` has.

**Rejected:**
- PR 6990's spelling: value first by design, with decisions taken in order to be discarded.
- Taking the axis rule out: the slab's wall turns over the box.
- Tightening the value channel: `h/|h|` still needs a sign.
- Box-certified reads in the query: the box changes with the valuation, with no record.

**Reversible:** the query is additive, and its caller list says where it is used.

**Possible later (unsure):** a constant-ratio form of the query, for a 45° side between formula corners. Measure on a real document first.

## For the orchestrator

1. **Point lanes: held, with what I checked in the tree.**
   - `Dual` reaches the profile. `eval/mod.rs` has `impl ProfileLane for Dual<T>`, and its docs say the guided second pass lets "a Dual seed on a profile dimension carry its tangent". `Dual::is_poison` reads only the value channel (`dual.rs` ~562), so `is_exact_zero` at `Dual` answers on the value. My earlier `Dual` argument is now sure.
   - Point lanes are compared across valuations and lanes:
     - `diff_verdicts` is scalar-generic and diffs `f64` runs across parameter edits (`resolve/mod.rs` ~1564 and ~1594);
     - the profile's `f64` log "compares with any lane's" (C6 guidance, `ProfileLane` docs);
     - `drive.rs` diffs the witness against the leaf.
   - So B's "nothing re-values a point lane" is false in the tree.
   - **Correction to my own earlier shape.** The symbolic lanes' axis sign decision is itself a row that `f64` never has, so it would break vector parity in `diff_verdicts` and `drive.rs`. It belongs in `k_stats::unlogged` under F43, which is that door's stated purpose. That costs B's f64-log argument against A's old shape some force within a single log, but the `Dual` argument does not depend on logs.
   - **Plain `Interval` over a box:** `ProfileLane for Interval` exists ("an interval parameter widens the loop"). I did not find whether a production path drives plain `Interval` over a real box through a sweep, so I left it as (a)'s stated cost.
2. **Lint: moved to A.**
   - A is right that the lint runs at `Sym<f64>` deferring, where the value read is already gated off (`Witness::Exact`). My change would only have dropped registrations, and a constructed arc's joints are proved by them.
   - On whether "structural" is meant to include them: `[intent.statements]` makes "constructing them as one, which proves it" an established identity, and a registration is the construction stating its proof. So yes for the lint. For the mint's query, still off: no need is shown, and the witness verdicts depend on the valuation.
3. **Sign decision: traced, partly.** The `vertex_separation` margin is the chord length `|b − a|` (`seg.rs` ~272): a magnitude, not the sign of a component. So the sign is a new fact, not a second reading ([one-way] holds). "Cannot escalate" (B) is not sure at `Sym<Interval>`: `x`'s value still carries dependency width, so `|chord| > 0` over the box does not strictly imply that `y` excludes 0. Propagating a refusal is the safe arm, and A's point that the verb may run before separation is decided agrees.
4. **Scope:**
   - `[verdicts.consumers]` amendment for all lane choices, with an audit: yes (A).
   - Query rules and budget fixed kernel-wide: yes (B). It needs a memo keyed by that fixed set, separate from the session's walks. Cost unmeasured.

**A question none of us asked.** The C6 guided pass consumes the `f64` first pass's structure decisions in every lane and "none is remade". Is the mint's symbolic-only sign decision outside that record (fine under `unlogged`), or does the guide refuse a decision it has no entry for? Someone should read `profile::PINNED_UNDER_GUIDANCE` and the guide's consume path before building. I did not finish tracing it.

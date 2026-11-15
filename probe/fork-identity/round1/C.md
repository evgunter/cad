## For Ev

**Recommendation (likely).** Stop asking "is this an identity?" through a sign reading. Give the symbolic tier one query that is not a decision: `identically_zero(x)`, meaning "is `x`'s expression the zero function of the document's variables?". It is answered from the expression and never from a number. Terms:
- **Symbolic tier**: E12's polynomial normal-form test over the parameter symbols.
- **Lane**: the scalar a run computes in: `f64`, `Interval` or `Dual` (the *point lanes*), or `Sym<f64>` and `Sym<Interval>` (the *symbolic lanes*, which carry an expression beside the number). Each runs in an eager session (the tier proves a Zero as it is taken) or a deferring one (proofs wait until someone asks).
- **Valuation**: the parameter point or box a run is evaluated at.
- **Decision**: a sign read against a band, counted, recorded for re-valuation, and queued in a deferring session.

**The query.**
- It answers *no* at every point lane, because those lanes hold no expression. At `Sym` it runs the normal form and the retry ladder, memoised per node.
- Every rule that reads a value is switched off: the certified read at `min`/`max`/`select`, and the registered-identity door, whose entries are witnessed against values.
- It is the same in eager and deferring sessions. It adds no decision count, no deferred entry and no re-valuation record.
- Its answer is a function of the expression graph alone, and that graph is a function of the document and of the decisions already recorded. So re-valuing a document cannot change the answer without a recorded decision changing first.

**Its licence.** It may only choose between two spellings of one real number: a closed form that equals the general form wherever the identity holds, which is everywhere. It never glues, splits or refuses. A caller list kept by hand, with a CI gate, holds this, as `is_exact_zero`'s list is held today.

**The direction mint: one function, called once per segment.**
- Where one component of the vector is identically zero, store the axis `(0, ±1)` or `(±1, 0)`. Its sign is a decision on the other component, taken at the run band, recorded and kept. If that sign escalates, the escalation propagates to the subdivision driver and there is no fallback: with one component identically zero, an undecided sign means the segment may be degenerate on the box.
- Otherwise store the quotient (the vector over its length), and take no reading.
- At the point lanes it is always the quotient. At `f64` that is already the axis bit for bit wherever a component is exactly 0.
- The line director and the arc radial are this one question. A `line_to` hop's director is minted once, by the verb that drew it, and both its junction checks and the stored segment read it. Today the junction normalises the chord itself (`path::chord_director`) and validation mints it a second time, which is two spellings of one fact. Validation mints only for directions that no verb made (a table's segments).
- The aside and the readings it threw away go.

**Worked examples.**
- **Slab side `(W, 0) → (W, 1)`.** The x component is `W − W`, identically zero. Every symbolic lane stores `(0, 1)` at every valuation, point or box, eager or deferring.
- **Hop `(x, 0) → ((x + w) − w, 1)`.** The axis at `(1, 1)` and at `(0.1, 0.3)` alike. `f64` stores `(2.8e-17, 1)`, the same real number to rounding.
- **Rectangle side between corner variables `a` and `b` that agree at the nominal.** `b − a` is not identically zero, so every valuation stores the quotient. That keeps the five corpus rows green.
- **`(max(a, b), 0) → (b, 1)`.** Not identically zero once the value read is off, so the quotient at every valuation.

**`literal_sign`** asks for a bulge's sign: a fact about a value that decides which way the arc turns. It stays a decision, recorded and kept, and a flip there is a real flip.

**`is_exact_zero`** keeps one meaning: "this is a zero the kernel wrote as a literal" (an iso line's held slope, a stored start). A caller that applies it to a *difference of computed values* is reading a coincidence of values. Each such caller moves:
- to `identically_zero`, if it only chooses a spelling;
- to a margined decision, if it changes what is built. The bulge's line-or-arc lowering is one: it is a structural act, and it repeats validation's straightness check.

**It is not this query, at the point lanes (likely).** The alternative is that `f64` answers *yes* on the value 0, so `is_exact_zero` becomes the door. At `Dual`, a side between `W₁` and `W₂` that agree at the nominal would then store a constant axis, whose sensitivity to `W₁` is zero while the real side tilts. And at `f64`, one valuation would take a sign decision that another does not.

**"Was this Zero proven" readers** (`MarginDiag::proven`, `deferred::discharge`). After this change, none of them steers construction: the remaining reads are refusal texts, `topo`'s record of how a coincidence was discharged, and the coincidence replay. They stay. One refinement (unsure): the unproven-coincidence lint asks whether a coincidence is *structural*. [intent.structural] defines that as "zero as a polynomial in the document's variables", so the lint should run the value-free walk rather than the decision's stamp, which may rest on a value read that holds on this box only. This moves lint findings at most, never a body, and is unmeasured.

**What the value read at `min`/`max` is (likely).** It is right inside a *decision*: the decision's outcome is recorded, so a flip at another valuation is seen. It is wrong only for a question about the whole function, which is why the query switches it off.

**Ratified text this changes.**
- **ERROR-DESIGN E12.** It gains: "a construction may ask the tier whether a value is identically zero, to choose a closed spelling of one real; the answer is not a decision." Agent-written text approved with "lgtm" on #1712.
- **`[verdicts.consumers]`, the lane-choice rule "Zero takes the closed form".** At a symbolic lane, a closed form that drops a dependence on the variables is taken only on *identically zero*. A Zero read off the value is a coincidence at this valuation only.
- **`crates/profile/README.md`, the direction clause.** A verb records the directions it makes, a `line_to` hop's included, and only a table's are minted at validation, by the one mint.
- **The `sym.rs` claim (not ratified)** that a `Sym<T>` value channel is bit-identical to a plain `T` run. It becomes "except where a construction chose a closed spelling by structure".

**Rejected.**
- PR 6990's proof read off a decision: value-first by design.
- Taking the axis rule out: the slab's wall turns over the box.
- Tightening the value channel: it cannot make `h/|h|` into ±1 without a sign, and it costs a normal form per value.
- A tagged direction: a downstream branch on the tag, which `f64` structure cannot supply.

**Reversibility.** The query is additive and its caller list shows where it is used. Rolling back means taking the axis rule out.

**Possible later (unsure).** A "constant ratio" form of the query, for a 45° chamfer between formula corners. Not built until a document needs it.

## For the orchestrator

1. **Point scalars: held B/C, with an argument of my own added.** A's shape (`f64` answers *yes* at 0, the door is `is_exact_zero`) stores a constant axis at `Dual` for a value coincidence, which gives a zero tangent where the side really tilts. This is likely; I did not run `Dual`. A's own case for the shape is plain `Interval` with `[0, 0]`, "wide by the box". That case only occurs at a point box, because `W − W` over a real box is not `[0, 0]`, so the width there is ulps. B's alternative D is the same argument at `f64`.
2. **Value reads: held "every value-reading rule off".**
   - **Moved by B:** the registered-identity door is also off. I had missed that registrations are witnessed against values (B, and A's own caveat).
   - **Retracted:** my claim that the `min`/`max` read is an *unrecorded decision*. B is right that inside a decision the outcome is recorded and the flip is seen.
   - **Not moved by A's "truth about each box":** the box changes with the valuation, so the stored form would change with no record. That is the defect class the brief names.
3. **Where the mint lives: moved to B's placement, kept as one function.** The verb mints a `line_to` hop's director with the one mint function and its junctions read it, and validation mints only for tables. What moved me was B's and A's `chord_director` observation, which I checked in `path.rs`: the junction normalises the chord separately, so the two spellings would disagree over a box. The positions are compatible: one function, called by whoever makes the segment.
4. **Proven readers: corrected myself against the tree.** The geom-brep `.proven()` reads I flagged (`certify.rs` ×3, `dihedral.rs`, `pcurve_cache.rs:~15869`) are all in tests or refusal rendering, and none steers construction. Withdraw that off-question item. Position now: keep them (A); the lint's structural question runs the value-free walk (B and me, unsure).

**A question none of the three asked.** At a symbolic lane, is the mint's sign decision ever new, or does it repeat vertex separation's decision on the length? If it repeats it, the axis could take its sign from that decision rather than take a second reading. That is [one-way]'s "two ways of deciding one fact". I did not trace `vertex_separation`'s margin to check.

**Agreed off-question rows, worth filing:**
- the bulge lowering's bit-equality line-or-arc test (B), a structural act;
- the `deferred.rs` "nothing else changes" claim, false at an exact witness (A, B);
- the `literal_sign` ledger sentence contradicting audit row F43 (A);
- the three-kind audit of `is_exact_zero`'s callers.

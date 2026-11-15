## For Ev

**Recommendation (likely).** Give the kernel a structural question that is not a decision: "is this value zero at every point this scalar stands for?" At a symbolic scalar it is answered from the value's expression and never from its number. The profile's direction mint branches on it instead of on a sign reading.

Terms:
- **Symbolic scalar** (`Sym<T>`): a number (the value channel) beside its expression over the document's variables. **Plain scalars**: `f64`, `Interval`, and `Dual` (a value with its first derivatives).
- **Decision**: a sign read against a band. **Verdict log**: the per-node list of decisions that the kernel compares row for row between the `f64` witness build and the certified leaf, and between two builds of an edited document.
- **Valuation**: the parameter point or box a document is evaluated at.
- **Deferring session**: one that postpones proofs until a reader asks.

**Agreed by all three designers.**
- *The query at a symbolic scalar* is the tier's normal-form test with every rule that reads a value off: the certified read at `min`/`max`/`select`, and the registered-identity door. Its answer is a function of the expression graph, so one document gets one answer at every valuation, eager or deferring. It counts as no decision, queues nothing, and returns a proof or nothing; there is no answer meaning "differs".
- *Its licence:* it may only choose between two spellings of one real value. It never glues, splits or refuses.
- *The mint, one function.* Where one component of the vector is identically zero, store the axis, signed by a sign reading of the other component. Otherwise store the vector over its length and read nothing. The `line_to` verb calls it and records the director, and that hop's junction checks read the same value; validation calls it only for a table-authored profile. An arc's radial uses it too. `sym::aside` and the discarded readings go.
- *`literal_sign`* (a bulge's sign) is a fact about a value and stays an ordinary recorded decision.
- *Rejected:* PR 6990's spelling, taking the axis rule out, making the value channel exact, a tagged direction type.

**Worked example.** Slab side `(W, 0) → (W, 1)`: `W − W` is zero as an expression, so every symbolic lane stores `(0, 1)` at every valuation. Hop `(x, 0) → ((x + w) − w, 1)`: the axis at `(1, 1)` and `(0.1, 0.3)` alike; `f64` stores `(2.8e-17, 1)`, the same direction to rounding. Two corner variables that agree at the nominal: `b − a` is not identically zero, so the quotient at every valuation. `(max(a, b), 0) → (b, 1)`: the quotient at every valuation.

**Choice 1 for you: one door or two.**
- *One door (my lean, likely; B's round-1 position).* `is_exact_zero` is redefined as the question above. `f64` answers yes at 0; `Interval` at `[0, 0]`; `Dual` only where the value and every derivative are 0; `Sym` by the value-free test. For it: after the change no function at a symbolic scalar reads "exactly zero" off the number, so the defect the five corpus rows caught cannot be written again by a new caller; it is one definition, not a special case per scalar; and it repairs `Dual`, where today's `is_exact_zero` reads the value alone and so already drops a derivative wherever a caller takes a "zero" shortcut.
- *Two doors (C's position, and mine in round 1).* The query answers "not shown" at every plain scalar, and `is_exact_zero` stays, narrowed by audit to reads of literals the kernel wrote. For it: the plain lanes take no branch at all, which is the simplest thing to say about them.
- *What the tree settles.* (a) The objection that `f64` would take a sign decision at one valuation and not the next does not separate the two: the `f64` witness and the symbolic leaf take different arms under either shape (the rounding hop above), and the verdict log is compared row for row between them, so the sign reading must sit outside that log either way. At `f64` the two arms are then the same bits. (b) The `Dual` objection is real against "yes on the value 0" and is met by the derivative clause. (c) No production lane runs plain `Interval` over a box through a profile (the driver's leaf is `Sym<Interval>`), so the plain-`Interval` case decides nothing in practice.
- *Either way,* callers of `is_exact_zero` whose branch changes what is built (a bulge of zero lowering to a line) become margined decisions. Under one door that conversion has to land with the door, or witness and leaf disagree loudly on such a document.

**Choice 2 for you: does "structurally one" include a registered construction identity?** The unproven-coincidence lint reports coincidences the document does not establish. DESIGN `[intent.structural]` defines established as "zero as a polynomial in the document's variables". B and C would have the lint use the value-free test. I would leave the lint as it is. Its lane is `Sym<f64>`, where the certified read is already never taken, so the only thing the change removes is the registered identities: the facts a construction proves about what it built where the algebra cannot close them (`crates/profile/README.md`: "An identity the algebra does not close is registered by the construction that proves it"). Without them I expect a constructed arc's own joints to be reported as unproven (inferred, not measured). That is the noise `[intent.structural]` says the rule exists to cut. Against: a registration is witnessed against values, so in principle a finding could appear at one valuation only. This moves findings, never a body. (likely)

**The mint's sign reading.** It is a new reading (the sign of one component, where vertex separation reads the chord's size), not a second reading of the same margin. It carries no fact the log lacks: once separation is decided positive and the other component is identically zero, the sign cannot change without separation failing first. So it stays outside the verdict log under its audit row, and at a symbolic scalar it is still seen by the re-valuation instrument. At the verb, separation has not been decided yet, so the reading can fail to decide; then the escalation propagates and there is no fallback to the quotient. (likely)

**Two scope rules I recommend.**
- *The query's rules and budget are fixed kernel-wide,* not taken from the session. The driver's term budget is a user dial and the coincidence replay has its own constant; left to the session, one document could mint the axis in one lane and the quotient in another. (likely; cost unmeasured)
- *The amended `[verdicts.consumers]` sentence applies to every lane choice,* with an audit to follow: at a symbolic lane, a closed form that drops a dependence on the variables is taken only on identically zero. (likely on the rule; unsure how many sites)

**Ratified text this changes** (yours):
- ERROR-DESIGN E12: a construction may ask whether a value is identically zero, to choose between spellings of one real; the answer is not a decision. (Agent text, approved "lgtm" on #1712.)
- DESIGN `[verdicts.consumers]`, "Zero takes the closed form": as above. As written it licenses what PR 6963 did.
- DESIGN `[scalars]`: names the structural query; under one door, what each scalar answers.
- `crates/profile/README.md`, the direction clause (your "makes sense", PR 6936): a hop's verb makes its director from the two points by the one mint; only a table's are minted at validation.

**Reversible.** The query is additive; with it answering "not shown" the mint is the quotient everywhere.

## For the orchestrator

**1. Plain scalars: moved back to one door, with a `Dual` clause, as a lean and a choice for Ev.** What I checked:
- `k_stats::unlogged`'s doc and `resolve/vdiff.rs::diff_verdicts`: the verdict log is compared row for row between the `f64` witness and the leaf, and across parameter edits, "scalar-independent data". So B's "nothing re-values a point lane" is false as stated. But it cuts against my round-1 argument too: with plain scalars answering "not shown", the leaf takes the axis and its sign reading where the witness takes the quotient, so the sign reading must be `unlogged` (as F43 is today) under either shape. Once it is, `f64` yes-at-0 changes no log row and no bit. My argument (ii) is answered.
- `Dual`: `Decide for Dual` delegates to the value, `is_poison` is the value's, so today's `is_exact_zero` at `Dual` reads the value alone. C's argument (iii) holds against a naive one door and is met by "value and all derivatives zero". `Dual` is used in `editor-core` production (`analysis.rs`, `checks/at_rest.rs`, others); I did not trace it through the profile mint.
- `drive.rs`: the leaf is `Evaluation<Sym<Interval>>`. I found no production plain-`Interval` run through a profile over a box. C's rebuttal of the `[0, 0]` case is not right (literal coordinates `(3, a) → (3, b)` give `[0, 0]` over a real box), but the case has no production lane.
- What tips me to one door: with two, `is_exact_zero` at `Sym` still reads the value channel and only an audit keeps computed values out of it.

**2. Lint: held.** `[coincidence.structural]` and the profile README both treat a construction's proof as establishing. I could not confirm from `coincide/replay.rs` or `topo/coincidence.rs` that discharge goes through the registry rung (no mention there; it is inside the tier's ladder), so "arc joints become findings" stays inferred. Worth one measured run before Ev rules.

**3. Sign: traced.** `line_to_point` and `line_to_start` call `chord_director(d)` before `push_line`, with no separation decision before it (a zero chord gets a `+x` stand-in). So B's "cannot escalate" holds at validation and not at the verb. Answer to C: a new reading, implied by separation, hence unlogged. Open for the lane: an escalation inside `unlogged` is dropped from the frame, so the mint must return it as the verb's error for the driver to see it.

**4. Scope.** B's question: yes, fix the query's rules; `drive.rs` exposes `max_terms` as a dial and `coincide/replay.rs` has a const `BUDGET`. It needs its own memo. Mine stands.

Not checked: the walk with reads off on the corpus's hop identities; the three-kind `is_exact_zero` audit; any row comparing `f64` and `Sym<f64>` stored values bit for bit.

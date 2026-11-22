## For Ev

**Yes: it can be Sym-only, with asking at `f64`, `Interval` or `Dual` a compile error, and I think that is plainly the better final state.** (likely)

Terms: **`Sym`** is the symbolic scalar, a number carried beside its expression over the document's variables. **Numeric scalars** are `f64`, `Interval`, `Dual` and the test `Probe`. **The question** is "is this value identically zero in the document's variables". **The mint** is the one function that makes a segment's unit direction (`seg::axis_or`).

**What the type error is.** The question becomes a method of the expression, not of the scalar: `identically_zero()` is defined on the expression handle that only `Sym` holds. No trait carries it. So `x.identically_zero()` written on an `f64`, an `Interval`, a `Dual`, or on a generic `T` fails to compile: no such method. There is no "no" for a numeric scalar to return, because there is nothing to call. (sure that this compiles as described; it is ordinary Rust)

**How one generic body still works.** The scalar trait gains one associated type, "my expression", and one method that hands it out:
- at `Sym` the type is the expression handle, and the method returns it;
- at every numeric scalar the type is the empty type (one with no values), so the method can only ever return "none".

The mint stays one body: "if this component has an expression and the expression is identically zero, store the axis; otherwise the chord over its length". At a numeric scalar the first arm cannot be entered, by type, and the compiler removes it. What a numeric scalar states is that it holds no expression, which is a fact about the type. It never holds the question. (likely)

**The other shape, and why I lean away from it.** Put the choice in a per-scalar trait whose method is the spelling itself ("the unit direction of this vector"), written once per scalar: the numeric ones return the quotient, `Sym` asks. That is the most literal reading of your suggestion, since generic code then never mentions the question at all. Its costs: the mint stops being one function (its two arms live in different impls, and the quotient arm is written four times); every future site that wants a structural answer needs another trait method; and the bound spreads through every profile signature that reaches the mint. The associated-type shape keeps one mint and one place for the question. Both give the compile error you asked for. (likely)

**Does anything else need to ask at a generic scalar?** No. Last round's audit of `is_exact_zero`'s callers found tags the kernel wrote, equality of two stored copies, and two computed values; none asks about identity, and only the mint does. Main has more call sites now (30 lines mention it, against 24); I have not re-audited the new ones. (likely on the old ones; unsure on the new)

**What it leaves possible that it should not.** Nothing new. It removes one thing the "answers no" text allowed: a caller at a numeric scalar asking, getting "no", and reading that as "differs".

**Replacement text, DESIGN `[scalars]`:**
> - **The structural question**: whether a value is identically zero in the document's variables. It is asked of an expression, and only `Sym` has one: the question is a method of `Sym`'s expression, answered where the symbolic tier proves the expression the zero function, by rules that read no value and a budget no session sets (ERROR-DESIGN E12), so the answer never comes from the value channel and is the same at every valuation. `f64`, `Interval` and `Dual` have no expression, and asking one of them does not typecheck. Code generic over the scalar reaches the question only through the scalar's expression, which a numeric scalar's type says is absent; there it takes the general spelling. At a numeric scalar zero is the band's ([verdicts]), and a zero that means something is a variant, never a number read back ([unrepresentable]). The question is not a decision: it reads no sign and no band, counts and records nothing, and defers nothing. Its answer may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]); a branch that changes what is built is a decision ([verdicts]).

**ERROR-DESIGN E12's bullet:** "A numeric scalar is never asked." becomes "A numeric scalar cannot be asked: the question is a method of the expression, and it has none."

**One scope question this raises, not for this PR.** Two existing hooks have the shape you are objecting to: `Real::register_equal` (a numeric scalar "registers" an identity by checking it and recording nothing) and `Decide::free_identity` (a numeric scalar returns itself). If the rule is "what only `Sym` can do, only `Sym`'s expression offers", they would move the same way. I would file it and not widen this change. (unsure)

## For the orchestrator

- **Verdict: plainly right in principle; a real but small choice of form** (associated expression type against a per-scalar spelling trait). I recommend the first; each is stated above as a final state.
- **Concretely, on main today:** `axis_or<T: Decide>` reads each component through `read_component`, a counted sign reading at `Band::least()` inside `sym::aside`, with `Component::ProvenZero` coming from `sym::deferred::discharge`. In the final state: `Real` gains `type Expr` and `fn expr(self) -> Option<Self::Expr>`; `f64`, `Probe`, `Interval`, `Dual<T>` set `Expr = Infallible` (or a named empty enum); `Sym<T>` sets it to its node handle. `identically_zero(self) -> Option<Identity>` is an inherent method on that handle, running the value-free walk. `axis_or` keeps its signature; `read_component`, `Component::UnprovenZero` / `Escalated` and the `aside` go; the sign of the other component is the one remaining reading.
- **Nested scalars:** `Dual<Sym<T>>`, if it exists or appears, sets `Expr` to the empty type like every `Dual`. Passing the inner expression through would store a constant axis with a zero tangent on a coincidence, the case C raised in round 2. I did not check whether that nesting is instantiated anywhere.
- **`ProfileLane`** (`editor-core/src/eval/mod.rs`, sealed, five impls) is the precedent for the per-scalar-trait shape, and where its bound would land if Ev prefers that one.
- **Not checked:** `READS_BY_SUPPORT` (your brief names it; `grep -rn READS_BY_SUPPORT crates` finds nothing at ae076e05, so either it is renamed or not on main); the told-quantity type (`geom-core/src/told.rs`), which I did not read and which I assume is not a scalar the mint runs at; the six `is_exact_zero` mentions added since the audit.
- **Interim clause from last round** ("at `Sym`, `is_exact_zero` answers by the expression test") is unaffected: `is_exact_zero` stays a `Real`-generic function over values, and is not the question.
- **Nothing in the earlier agreement moves:** value-reading rules off, fixed budget, the sign reading outside the verdict log and propagating, `literal_sign` a decision, the `[verdicts.consumers]` amendment.

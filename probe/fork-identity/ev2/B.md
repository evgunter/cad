## For Ev

**Yes (likely):** the question can be a method of `Sym` alone, so asking it of `f64`, `Interval`, `Dual` or a generic scalar does not compile. The generic code then never asks a question. It offers two spellings, and only the symbolic scalar chooses between them.

Terms:
- **Numeric scalars**: `f64`, `Interval`, `Dual`.
- **Symbolic scalar** (`Sym<T>`): a number carried beside its expression in the document's variables.
- **The question**: "is this value zero as a function of the document's variables?", answered by the symbolic tier with every value-reading rule off.
- **Spelling**: one of two formulas for the same real value, e.g. the axis `(0, ±1)` or the chord divided by its length.
- **General spelling**: the one that is right everywhere.

**The final state.**
1. **The question is an inherent method of `Sym`, not of any trait.** It is `Sym::is_identically_zero(self) -> bool`. `x.is_identically_zero()` with `x: f64`, or with `x: T` where `T: Real` or `T: Decide`, fails with E0599 (no such method). That is the type error, written at the call. (sure that this is expressible)
2. **The scalar trait gains one method that is not a question**, written for example `fn spelled<R>(x: Self, closed: impl FnOnce() -> R, general: impl FnOnce() -> R) -> R`. It means "use `closed` where `x` is identically zero, else `general`".
   - The default body calls `general()`. Every numeric scalar keeps that default, so a numeric scalar never constructs the closed spelling and no answer exists to branch on.
   - `Sym` overrides it with its inherent question.
   - No bool leaves the scalar, so no caller can build structure on the answer. That is the licence ("only choose between two spellings of one real") held by the shape of the API rather than by a list of callers. (likely)
3. **The direction mint is one generic body.**
   - For each component of the chord, it calls `T::spelled(w.x, || axis signed by a sign reading of w.y, || quotient)`, then the same with the components swapped.
   - At a numeric scalar it is the quotient, branch-free. This is what PR 6990 stores today, and at `f64` it is the axis to the bit except for the sign of zero.
   - At `Sym` the sign reading runs only inside the closed spelling. An escalation there propagates as the result, with no fallback.
   - `line_to` and table validation call this one mint. `axis_or`'s `aside`, `read_component` and `Component::ProvenZero/UnprovenZero` go. (likely)
4. **Other generic callers that need it.** One, from last round's audit of `is_exact_zero` (21 sites on main):
   - `arc_rim_normal_weight` (`w₁/w₀` where the end weights are one value, else `w₁/√(w₀w₂)`) exists to keep a symbolic identity at `Sym`, so it becomes a `spelled` call.
   - The other spelling shortcuts (`cylinder_rim_axial`'s fast path, the torus hub) take the general spelling everywhere, or `spelled` where a symbolic identity is wanted.
   - The structural literals (an iso line's held slope, a ray's zero start) become types or variants.
   - Compared copies become stored-once data or a named "same stored bits" read.
   - The structure-changing branches (the bulge lowering, the zero-chord stand-in) become decisions.
   - So nothing generic needs the bare question, and `is_exact_zero` is retired. (likely; per-site, not measured)

**The alternative, as a final state.** The question is a method of the scalar trait, and numeric scalars answer "no".
- It works, and it is one method rather than a method plus a combinator.
- But a bool comes back to generic code, so only review stops a caller from branching on it for structure, or from mistaking "no" for "differs".
- And a numeric scalar carries a question that is meaningless for it, which is what you are asking to remove.

**Is it plainly right?** Close to it.
- The one real cost is that `spelled` is a closure-taking method on the scalar trait, a shape the trait does not have today.
- There is one honest limit: the closures could still do different things. The type stops a caller from *reading* the answer, not from writing two spellings that are not the same real value. A debug check at `Sym` that both spellings agree at the value channel would hold the rest.
- I recommend the `Sym`-only question with `spelled`. (likely)

**Ratified text.**
- **`[scalars]` bullet:**
  > **The structural question** is asked only of the symbolic scalar: `Sym` answers whether a value is zero as a function of the document's variables, wherever the build takes the decisions it took, by rules that read no value and a budget no session sets (ERROR-DESIGN E12). It is no method of `Real` or `Decide`, so a numeric scalar or generic code cannot ask it. Generic code offers two spellings of one real value through `Real::spelled`; a numeric scalar takes the general one, and `Sym` takes the closed one where the question holds. It is not a decision: it reads no sign and no band and records nothing. A zero the kernel knows by construction is a type or a variant; a branch that changes what is built is a decision ([verdicts]).
- **E12's structural-question bullet:** replace "A numeric scalar is never asked" with "The question is `Sym`'s alone and does not typecheck elsewhere; generic code reaches it only through `Real::spelled`, whose numeric default is the general spelling."

## For the orchestrator

- **Read on current `origin/main` (2026-11-22):**
  - `seg.rs` `axis_or`, `Component`, `read_component`. They still use `aside` and a sign reading plus `deferred::discharge`, i.e. PR 6990's narrowing.
  - `Decide`'s surface: `sign_within`, `enclosure_probe`, `free_identity`.
  - `Real`'s `WITNESS` const and `register_equal`.
  - I did not find `Real::READS_BY_SUPPORT` on main (`git grep READS_BY` returns nothing). The brief may mean something renamed or unmerged. `told.rs` is render-only and has no bearing here.
- **Precedent cutting against strict type purity:** `Real::register_equal` is already a facility that means something only at `Sym`, and every numeric scalar implements it trivially (`real.rs` ~599, `interval.rs` ~604, `dual.rs` ~514). If Ev's principle ("Sym only, type error elsewhere") is adopted, it should be stated generally, and the registry door is the next candidate. Worth one line in the PR.
- **Other readers of "proven":** `editor-core/coincide/replay.rs:396` (`deferred::discharge`) is the lint and unaffected.
- **`Sym`'s override takes a sign reading inside a closure that only symbolic lanes run.** The earlier open item still applies: the reading is unlogged (parity with the `f64` witness), so check that an escalation returned through the mint's `Result` reaches the driver as a bisect cue.
- **Not checked:**
  - whether any lane runs `Sym<Dual>` or `Dual<Sym>` (the override and default would then need care);
  - compile-time cost of a closure-generic trait method across the kernel's monomorphizations;
  - the per-site conversions above.

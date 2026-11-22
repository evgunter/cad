## For Ev

**Yes, and no decision is left (likely).** Asking at `f64`, `Interval` or `Dual` becomes a compile error. The question moves off the scalar and onto the *expression* that only the symbolic scalar carries.

Terms:
- **Numeric scalars:** `f64`, `Interval`, and `Dual` (a value with its derivatives), plus the test scalar `Probe`.
- **`Sym`:** a number carried beside its expression over the document's variables.
- **The question:** "is this value identically zero in the document's variables?", answered by the symbolic tier with every value-reading rule off.
- **Spelling:** one of two formulas for the same real value, for example the axis `(0, ±1)` or the chord over its length.
- **Uninhabited type:** a type with no values.

**The final state.**
- **What the trait gains.** `Real` gets one associated type and one accessor:
  - `type Expression`: at `Sym` the node handle; at every numeric scalar an uninhabited type.
  - `fn expression(self) -> Option<Self::Expression>`.

  `Real::Carries` already does the same for `Dual`'s certificate (`Uncertified`), so the pattern is not new.
- **Where the question lives.** It is a method of the expression, `identically_zero() -> Option<Identity>`, where `Identity` is a proof token only the tier mints. `x.identically_zero()` on an `f64`, or on a generic `T`, does not compile. No impl anywhere answers "no".
- **The mint is ordinary generic code:**
  ```
  let zero = |c: T| c.expression().and_then(|e| e.identically_zero()).is_some();
  if zero(w.x) { axis(0, sign_of(w.y)?) }
  else if zero(w.y) { axis(sign_of(w.x)?, 0) }
  else { quotient }
  ```
  At a numeric scalar `zero` is statically false and the compiler drops both axis arms, so the sign reading does not exist there.
- **Sites that ask about several values read the same way:** the mint's two components, `arc_rim_normal_weight`'s `w₀ − w₂`, `one_value` over held weights, and `UnitVec3::is_exactly` over three components (today through `Real::is_exact_zero_in_every_channel`, new on main, which reads only the value channel at `Sym`).
- **Nesting.** `Sym<Dual<T>>` is instantiated in tests (`sym11_witness_kind_rows`). It has an expression, and a proven identity's derivative is truly zero, so the axis is right there. `Dual<Sym<T>>` and `Probe` have none, so they take the general spelling.

**What the licence still relies on.** The licence: the answer "may only choose between two spellings of one real value".
- **What the type does.** It stops a numeric scalar being asked, and stops anyone reading "no" as "differs", since there is no "no".
- **What only review stops.** It does not stop a generic caller from building something different on a proof (gluing, splitting). That is a short list of callers behind the existing gate.
- **No shape on the table does better.** A closure-taking `spelled(x, closed, general)` that keeps the answer inside the scalar was the other candidate. Its two closures can also build different things, so the type protects no more there. It also cannot serve a site that needs a yes/no (`one_value` feeds an `&&`), and it nests awkwardly for the mint. (likely)

**The same rule fits two existing hooks with no second mechanism.** `Real::register_equal` and `Decide::free_identity` are also things only an expression can do, and every numeric scalar implements them as no-ops today. Both become methods of `Real::Expression`. Moving them is a separate filed change, not this PR.

**Replacement text, DESIGN `[scalars]` bullet:**
> - **What only an expression can do is asked of the expression.** Each scalar declares the expression it carries (`Real::Expression`): `Sym` its node in the document's expression graph; `f64`, `Interval` and `Dual` none, by an uninhabited type, so what is asked of an expression does not compile at them, and code generic over the scalar takes the general spelling where there is none. **The structural question** is such a method: whether a value is identically zero in the document's variables, wherever the build takes the decisions it took, answered by the symbolic tier with rules that read no value and a budget no session sets (ERROR-DESIGN E12). It is not a decision: it reads no sign or band, and counts, records and defers nothing. Its answer may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]); a branch that changes what is built is a decision ([verdicts]), and a value the kernel writes to encode structure is carried by its type ([unrepresentable]).

**ERROR-DESIGN E12, the structural-question bullet:** "A numeric scalar is never asked" becomes "A numeric scalar carries no expression, so the question cannot be put to it (`Real::Expression`, DESIGN [scalars])."

## For the orchestrator

1. **Moved to A and C's shape.**
   - **What moved me:** C's `Carries<C>`/`Uncertified` precedent, which I checked (`real.rs:344`, `dual.rs:473`). The trait already expresses "this scalar has none of these" by an uninhabited associated type.
   - **My argument for `spelled`** ("no answer leaves the scalar") is answered by the fact that its closures are unconstrained, so it protects no more than a proof token does. It also cannot serve `one_value` (`edge_nurbs.rs:1390`, `pcurve_cache.rs:9471` use it in conditions) or `is_exactly` (`sweep/skin.rs:1976`, `rigid_motion.rs:1853`, `mate/value.rs:424`) without restructuring each caller into a pair of closures.
   - **The mint under my shape, for the record:**
     ```
     T::spelled(w.x, || axis(0, sign(w.y)), || T::spelled(w.y, || axis(sign(w.x), 0), || quotient))
     ```
     That is one level of nesting per component, with the sign reading hidden in a closure. Nothing about it is better.
2. **The licence under each shape.** Both shapes leave "build different structure on a proof" to review and the caller gate. The proof token adds that a numeric "no" cannot exist.
3. **Nesting, checked.**
   - `Sym<Dual<f64>>` and `Sym<Dual<Interval>>` appear in `geom-core/tests/sym11_witness_kind*`.
   - `dual.rs:485` mentions `Dual<Sym<_>>`.
   - I did not find either in production lanes.
   - `Probe` wraps `f64` (`k_stats.rs` ~2048) and gets no expression.
4. **Principle stated generally, with the two moves filed** (register_equal, free_identity to `Real::Expression`).
   - The merged text would then describe a state the code reaches by one filed change, rather than needing re-ratification.
   - If you prefer the text never to be ahead of the code at merge, narrow the first sentence to the question only, and add the general sentence with the filed change. The rest of the bullet is unchanged either way.
5. **`is_exact_zero_in_every_channel`** (new since the audit) reads the value at `Sym` (`sym.rs:5357`). It is the same defect class and belongs in the conversion audit with `is_exact_zero`'s callers. `is_exactly`'s uses look like identity shortcuts (spelling) except `skin.rs:1976`, which skips a frame (structure, so a decision). Not traced further.
6. **Not checked:** a build; whether `Expression` needs bounds that `Infallible` lacks (C says no).

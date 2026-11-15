## For Ev

**Recommendation now (likely): (b).** The structural question has an answer only at the symbolic scalar. `f64`, `Interval` and `Dual` always answer "no", so they always take the general spelling: the quotient, which is what PR 6990 already does. A zero the kernel knows *by construction* is carried by a type or a variant, not read off a stored number. `is_exact_zero` is retired. I withdraw "`f64` yes at 0, `Interval` yes at `[0, 0]`".

Terms:
- **Numeric scalars**: `f64`, `Interval`, `Dual`.
- **Symbolic scalar** (`Sym`): a number carried beside its expression in the document's variables.
- **The band**: ε·K, inside which a margin reads Zero ([verdicts]).
- **Spelling choice**: picking between two formulas for one real value, for example the axis `(0, ±1)` or the chord divided by its length.

**Your discomfort, stated exactly (sure).**
- At a numeric scalar the kernel's only notion of zero is the band. An exact `0.0` there is a fact about rounding as often as a fact about the shape: one identity computes `0.0` at one valuation and `2.8e-17` at the next.
- "`f64` yes at 0" gives the numeric scalars an exactness the rest of the design denies them. It is the same dependence on the valuation that the fork set out to remove, moved one scalar down.
- It was tolerable only because both arms are the same real number. But if both arms are always the same real, the answer at `f64` buys nothing, and "no" is equally correct with no dependence on the valuation.

**What today's callers actually read** (22 sites, checked on main):
1. **A zero the kernel wrote as structure.**
   - An iso line's slope component: `row_of`, `iso_image_row`, `patch.rs`. Fork 271 replaces this with a stored held axis.
   - A ray that starts at its origin: `Ray::start_point`, three sites. That should be an `Option` or a variant.
   - Each is structure encoded as an in-band number, which [unrepresentable] forbids.
2. **A stored copy compared with its original, bit for bit.**
   - A row's fixed value against the chart's domain end (`domain_side`, `spline_known::corners`).
   - An arc rim's stored end weights against its column (`pcurve_cache` ~9184, which refuses on a mismatch).
   - Held weights all one value (`one_value`).
   - These are record reads. They go away where the value is stored once (fork 271), or become a named "same stored bits" read, never "zero".
3. **Pure spelling choices.** `arc_rim_normal_weight` (`w₁/w₀` or `w₁/√(w₀w₂)`), the shortcut in `cylinder_rim_axial`, the torus hub in `cover.rs`. Under (b), "no" at the numeric scalars is always correct here.
4. **A branch that changes what is built**, on a value that may be computed.
   - The bulge lowering: a zero bulge becomes a line.
   - `chord_director`'s zero-chord `+x` stand-in.
   - These are decisions and belong to the band.

**The three final states.**
- **(a) As recommended to you.**
  - True: one function.
  - Still possible: a numeric lane branches on rounding (yes at one valuation, no at the next).
  - It also keeps a function named "exactly zero" that classes 1 and 2 will keep using for structure and copies. It needs a `Dual` clause to avoid freezing a tilt.
- **(b) Symbolic only, structure in types.** (recommended)
  - True in every lane: a numeric lane never claims an exactness the band denies. The answer at `Sym` is a theorem that holds wherever the build takes the decisions it took (E12 as now merged). The mint's stored value is a function of the document in every lane.
  - Still possible, and acceptable: none.
  - Costs: class 1 needs types (the iso line half is already fork 271), class 2 needs single storage or a named bit read, and class 4 needs decisions. These were owed anyway.
- **(c) At the numeric scalars, the question is the band's Zero.**
  - It becomes a decision: logged, with a margin, and it can escalate.
  - Under it, a numeric lane stores the axis where the component is merely within ε, which moves the stored direction by up to the band on a coincidence.
  - It puts a row in the `f64` witness's log that the symbolic leaf must then match. For the leaf to agree it must read the band too, which brings back the original defect: a constant stored on a value coincidence.
  - It destroys the three properties the query exists for ("not a decision, no log entry, same answer at every valuation").
  - Right for class 4, which already is a decision. Wrong for spelling choices.

**Replacement for `[scalars]`' bullet:**

> - **The structural question**: whether a value is zero as a function of the document's variables, wherever the build takes the decisions it took. Only a symbolic scalar can answer it: `Sym` answers yes where the symbolic tier proves its expression zero, by rules that read no value and a budget no session sets (ERROR-DESIGN E12), so its answer never comes from the value channel and is the same at every valuation. `f64`, `Interval` and `Dual` hold no expression and answer no; at a numeric scalar the only zero is the band's ([verdicts]). The answer is not a decision: it reads no sign and no band, counts and records nothing, defers nothing, and may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]), so the general spelling is always correct and a numeric lane always takes it. A zero the kernel knows by construction is a type or a variant, never read off a stored number; a branch that changes what is built is a decision ([verdicts]).

**E12's "structural question" bullet:**
- Replace "(`is_exact_zero`, DESIGN [scalars])" with "(DESIGN [scalars], the structural question)".
- Add after its first sentence: "At a numeric scalar there is nothing to ask: the general spelling is taken."
- Its first sentence says "identically zero". It should say "zero as a function of the parameters wherever the build takes the decisions this replay took", to match the merged mechanism.

**`[verdicts.consumers]`**: the sentence in front of you stands as written. It already speaks only of the symbolic scalar.

## For the orchestrator

- **The ev branch is behind main.** `git diff origin/main origin/boxfinish/ev-identity-question` shows E12's merged sentences ("…at which the build takes the decisions this replay took…", and the registration "equal at every parameter value" paragraph) being reverted. Merge main into the branch before the revision, or the PR undoes a ratified change.
- **Registered-identity door.** Under merged E12 a registration is an identity at every parameter value, and a fact that holds only past a decision is spelled as a literal sign. So my original reason for switching the door off (that its entries depend on the valuation) is weaker. They are only *witnessed* against values, and a refusal is a counted defect. I would keep it off for the query (no chord needs it) and say so as a choice of scope, not soundness.
- **Classification:** done from reading each site on main. Not measured: whether any shipped `f64` or `Interval` result changes when classes 1 and 2 move to types or named bit reads. Some should change nothing, by construction.
- **Retracted:** my rounds 3–5 "one door with plain scalars by value". The protection argument still holds, more strongly: with `is_exact_zero` gone, no function anywhere reads "exactly zero" off a number.
- **`Dual` clause:** no longer needed. `Dual` answers no, and the quotient carries its derivative.
- **Off-question, still open:** the bulge lowering (class 4), and the unlogged-escalation-to-driver trace for the mint's sign reading.

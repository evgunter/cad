## For Ev

**Yes (likely).** Asking the question at `f64`, `Interval` or `Dual` becomes a compile error. One decision of form is left, and I lean towards the first of the two shapes below. Both store the same thing in every lane.

**Terms**
- **Numeric scalars:** `f64`, `Interval`, `Dual` (a value with derivatives), and the test scalar `Probe`.
- **`Sym`:** a number carried beside its expression over the document's variables.
- **The question:** "is this value identically zero in the document's variables?", answered by the symbolic tier with every value-reading rule off.
- **Spelling:** one of two formulas for one real value. For a segment's direction these are the axis `(0, ±1)` and the chord over its length (the *general* spelling).
- **The mint:** the one function that makes a segment's unit direction, called by the `line_to` verb and by validation.

**Shape 1: an expression type (recommended).**
- The scalar trait states what expression a scalar carries: `type Expression`. At `Sym` that is its node. At every numeric scalar it is an *empty type* (one with no values).
- A method `expression()` hands it out, and at a numeric scalar it can only return "none". The question is a method of the expression, so asking `x.identically_zero()` of an `f64`, or of a generic `T`, fails to compile.
- The mint, as one generic body:
  - if `w.x` has an expression proven zero, store `(0, sign(w.y))`;
  - else if `w.y` does, store `(sign(w.x), 0)`;
  - otherwise store the quotient.
- At a numeric scalar the compiler removes the first two arms, so there is no sign reading there.
- A site that asks about several values reads naturally. "All three component differences are identically zero" (`UnitVec3::is_exactly`) is one `all(...)`, and so is "all held weights are one value" (`one_value`).
- The same shape takes the two other `Sym`-only facilities already on the scalar traits:
  - registering an identity (`Real::register_equal`) becomes a method of the expression;
  - limb 2's identity door (`Decide::free_identity`) becomes "if there is an expression, attach the identity".

  So one mechanism covers everything only `Sym` can do.

**Shape 2: a spelling method.**
- The question is an inherent method of `Sym`, and generic code cannot name it.
- The scalar trait gains `spelled(x, closed, general)`. At numeric scalars it always runs `general`; `Sym` overrides it to run `closed` where `x` is identically zero.
- The mint becomes `spelled(w.x, axis-from-w.y, || spelled(w.y, axis-from-w.x, quotient))`.
- Its claimed advantage is that no yes/no answer reaches generic code. That does not hold: `spelled(x, || true, || false)` is the answer in one line.
- Several values need nesting, or a second slice-taking method.
- `register_equal` and `free_identity` are actions, not choices of spelling, so they would need a second mechanism.

**What enforces the licence** (that the answer may only choose between two spellings of one real): in both shapes, review and the list of callers, not the type. A caller can always route the answer into a structural act. Both shapes can add a debug check at `Sym`: the two spellings agree on the value channel wherever the closed one is taken. (sure)

**Nested scalars, checked in the tree.**
- `Sym<Dual<…>>` is instantiated (the witness-kind tests, and `certifies::<Sym<Dual64>>`). Under either shape it asks its own expression. A proof that the function is zero also makes its derivative zero, so the constant axis is right there.
- `Dual<Sym<…>>` is not instantiated. Under either shape it carries no expression and takes the general spelling. That is safe: a `Dual` over `Sym` must not take a constant on its inner proof without its tangent.
- `Probe` wraps `f64`, so it carries no expression.

**Who else asks, on today's `main`.**
- *Spelling choices:* `arc_rim_normal_weight`, `one_value`, the torus hub, and `UnitVec3::is_exactly`. The last is new: it is used by the least turn's identity, by `mate::value` and by the sweep skin's still step, through `Real::is_exact_zero_in_every_channel`. These move to the expression question. At a numeric scalar they take the general arithmetic.
- *Structure that is currently read off a number* (the iso line's held slope, a ray's zero start) becomes variants.
- *Branches that change what is built* (chart corner and side, row ends, the bulge lowering, the zero-chord stand-in) become decisions.
- `is_exact_zero` and `is_exact_zero_in_every_channel` are then retired. (likely; the per-site conversions are not traced)

**Replacement text, DESIGN `[scalars]`:**
> - **What only the symbolic tier can do is a method of a value's expression.** Each scalar states the expression it carries (`Real::Expression`): `Sym` its node in the document's expression graph; `f64`, `Interval`, `Dual` and `Probe` none, by a type with no values, so nothing that needs an expression can be asked of them and such a call does not compile. Code generic over the scalar reaches an expression only through that accessor and takes the general spelling where there is none. **The structural question** is the first such method: whether the expression is the zero function of the document's variables, answered by the symbolic tier with rules that read no value and a budget no session sets (ERROR-DESIGN E12). It is not a decision: it reads no sign and no band, and counts, records and defers nothing. Its answer may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]). A branch that changes what is built is a decision ([verdicts]), and a zero the kernel knows by construction is a type or a variant, never a number read back.

**E12's structural-question bullet:** "A numeric scalar is never asked" becomes "A numeric scalar carries no expression, so the question cannot be put to it (`Real::Expression`, DESIGN [scalars])."

Stating the principle generally ("what only `Sym` can do is a method of its expression") means the sentence need not be re-ratified when `register_equal` and `free_identity` move. Moving them is a later change, not this one.

## For the orchestrator

1. **What moved me, and what did not.**
   - *A:* the same shape as mine. I took A's sentences on the band and on variants into the text.
   - *B:* I took the debug agreement check. I was not moved by B's main claim, that `spelled` keeps the licence by the API's shape. `spelled(x, || true, || false)` recovers the bool, so both shapes hold the licence by review only.
   - My own argument (the `Carries` precedent, one mechanism) is not answered by B. B's shape needs a second mechanism for `register_equal`, which B itself raises.
2. **Multi-value sites under B:** the mint nests two calls, which is fine. `one_value` and `is_exactly` need a fold of nested closures or a slice variant of `spelled`. Under A and C each is one `all(...)`.
3. **Nesting, checked on `main`:**
   - `Sym<Dual<f64>>` and `Sym<Dual<Interval>>` appear in `sym11_witness_kind_rows.rs` and `sym11_witness_kind_interval_rows.rs`, and `real.rs` ~2188 has `certifies::<Sym<Dual64>>`.
   - `Dual<Sym>` appears only in a `dual.rs` comment.
   - `Sym::is_exact_zero_in_every_channel` forwards to the inner dual (`sym.rs` ~5357). Under the final state it is retired, not forwarded.
4. **`is_exactly` callers:** `rigid_motion.rs:1853` (least turn), `mate/value.rs:424`, `sweep/skin.rs:1976`. `skin.rs` gates a still step with `!t.is_exactly(next)`, which may change what is built: that one should go to the audit as possibly a decision rather than a spelling. Not traced.
5. **Not checked:** compile cost of either shape; whether the `Infallible` associated type meets every bound `Real` places on associated types (`Carries` does, by analogy).

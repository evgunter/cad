**(a) Amended: the helper is right, but it does not belong in the ratified text.**
- A free function over the expression, taking a value and two spellings and returning one, is the right normal door. A's `spelled(x, || true, || false)` point stands against my withdrawn trait method.
- "Offers two spellings and receives one" and "read directly only at listed sites" say how code is laid out and how the gate is kept. They bind no design choice beyond the licence, which the bullet already states.
- Put the helper and the list of direct readers in `geom-core`'s README and behind the gate, not in DESIGN.

**(b) Both clauses are true, of different things, so the text should carry both.**
- *What is the same at every valuation* is the answer itself, meaning whether a proof exists. With every value-reading rule off, it is a function of the expression graph alone.
- *What the proof asserts* is conditional:
  - The graph denotes the computation only where the build takes the decisions it took, because every branch upstream picked the nodes.
  - The tier's quotient rule cancels a factor the replay divided by, so the identity holds only where that divisor is nonzero, which the build also presumes.
- E12's Decision paragraph on main says exactly this of a symbolic Zero: "zero at every parameter value in the leaf's box at which the build takes the decisions this replay took". So "identically zero", written bare, overclaims.

**(c) No: don't name `Probe`.** It is a feature-gated test scalar wrapping `f64`. Ratified text should state the rule for every numeric scalar, which covers `Probe` and any future wrapper, and name `f64`, `Interval` and `Dual` only as examples.

**Merged `[scalars]` text: accept, with two amendments, marked [B:] below.**

> - **What only a symbolic scalar can do is asked of its expression.** Each scalar declares the expression it carries (`Real::Expression`): `Sym` its node in the document's expression graph; [B: every numeric scalar (`f64`, `Interval`, `Dual`)] none, by a type with no values, so nothing that needs an expression can be asked of them: such a call does not typecheck, and code generic over the scalar takes the general spelling where there is no expression. **The structural question** is such a method: whether a value is [B: zero as a function of the document's variables wherever the build takes the decisions it took (ERROR-DESIGN E12)], answered by the symbolic tier with rules that read no value and a budget no session sets, so the answer never comes from the value channel and is the same at every valuation. Its answer is a proof, and there is no answer that means "differs". It is not a decision: it reads no sign and no band, counts and records nothing, and defers nothing. The proof may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]), so the general spelling is always correct; it never glues, splits or refuses, and a branch that changes what is built is a decision ([verdicts]). At a numeric scalar zero is the band's ([verdicts]), and a zero that means something is a variant, never a number read back ([unrepresentable]).

**E12's sentence: accept as given.**

**In the other two replies:**
- A's [scalars] text says "identically zero in the document's variables … the same at every valuation" without the decision condition. That is amended above; it is the one claim I think is wrong as written.
- A lists the multi-value direct readers as "none in production". They are candidates now: `UnitVec3::is_exactly` via `is_exact_zero_in_every_channel` (`rigid_motion.rs:1853`, `mate/value.rs:424`, `sweep/skin.rs:1976`) and `one_value` (`edge_nurbs.rs:1390`, `pcurve_cache.rs:9471`). The conversion audit decides which of them stay on the token, become a spelling choice, or become decisions. `skin.rs:1976` looks structural.
- Nothing in C's reply I disagree with.

No change to my recommendation to Ev beyond the two amended phrases above.

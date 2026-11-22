**(a) The helper: an implementation choice, so the text should not carry it.**
- The licence ("the proof may only choose between two spellings of one real") is the design.
- A free-function door with listed direct-read sites is a good way to build it, and belongs in geom-core's README or the audit, not in `[scalars]`.
- Ratifying "the proof is read directly only at listed sites" would put a caller list into DESIGN, which then has to be re-ratified every time a site is added.

**(b) Amended: carry E12's clause, not "the same at every valuation".**
- With every value-reading rule off, the answer is a function of the expression alone and never comes from the value channel. That part is true.
- But the expression is the one this build constructed. A valuation that takes different decisions builds a different graph.
- The quotient rule's cancellations hold "wherever D ≠ 0" (`sym/quotient.rs`, steps 1 and 2). That is where the computation is defined, which clause 1 already certifies over the box.
- So the proved zero is exactly E12's: zero at every parameter value at which the build takes the decisions this replay took. "The same at every valuation" overstates it.
- No value-dependent condition remains. The only conditions are the decisions already recorded and the domain.

**(c) `Probe`: drop the name.** Write "every numeric scalar", which covers `f64`, `Interval`, `Dual`, `Probe` and any future test scalar. This also covers `Dual<Sym<…>>`: a `Dual` carries no expression whatever it wraps.

**The `[scalars]` text: accept, with two amendments (marked [A1] and [A2]):**

> - **What only a symbolic scalar can do is asked of its expression.** Each scalar declares the expression it carries (`Real::Expression`): `Sym` its node in the document's expression graph; every numeric scalar [A1: was "`f64`, `Interval` and `Dual`"] none, by a type with no values, so nothing that needs an expression can be asked of them: such a call does not typecheck, and code generic over the scalar takes the general spelling where there is no expression. **The structural question** is such a method: whether a value is identically zero in the document's variables, answered by the symbolic tier with rules that read no value and a budget no session sets (ERROR-DESIGN E12). The answer never comes from the value channel: it is a function of the expression alone, and the zero it proves holds at every parameter value at which the build takes the decisions it took [A2: was "so the answer never comes from the value channel and is the same at every valuation"]. Its answer is a proof, and there is no answer that means "differs". It is not a decision: it reads no sign and no band, counts and records nothing, and defers nothing. The proof may only choose between two spellings of one real value (a lane choice, [verdicts.consumers]), so the general spelling is always correct; it never glues, splits or refuses, and a branch that changes what is built is a decision ([verdicts]). At a numeric scalar zero is the band's ([verdicts]), and a zero that means something is a variant, never a number read back ([unrepresentable]).

**The E12 sentence: accept as given.**

**What I think is wrong in the other two replies:** nothing beyond (a) and (b). The rest agrees with mine.

**Carried forward for the build, not for the text:**
- `sweep/skin.rs:1976` (`!t.is_exactly(next)`) may be a structural act, so it should go to the audit as a decision rather than a spelling.
- `Sym::is_exact_zero_in_every_channel` should be retired, not forwarded.

DESIGNER A — fork 272, answer to addendum (d). Read `84bfd40f0` and `259641935` on main: DESIGN `[unrepresentable]` "A signature is a claim too", `designer.md` §3's two new paragraphs, `geom-core/README.md` DL5.2. Nothing run. My answers to (a), (b), (c) stand.

**(d) The test, applied. Three signatures are in play, and they come out differently.**

1. **The question, `identically_zero`.** Its sentence at `Sym`: "a proof that this value is zero at every parameter value at which the build takes the decisions this replay took, or nothing". At `f64`, `Interval`, `Dual`, `Probe` there is no sentence. So by the clause it is excluded by its bound, and it is: it is a method of the expression, which those scalars do not have. Pass.

2. **The mint, `axis_or<T: Decide>`.** It has a sentence at every scalar, about the direction and not about the question. `f64`: the chord over its length. `Interval`: an enclosure of that. `Dual`: that with its derivatives. `Probe`: the `f64` answer, recorded. `Sym`: the same real direction, as the exact axis where a component is proved identically zero. So it is rightly admitted everywhere. Pass.

3. **The accessor, `expression() -> Option<Self::Expression>`, admitted at every scalar.** Its sentences: at `Sym`, "this value's node in the replay's expression graph". At `f64`, `Interval`, `Dual`, `Probe`: "nothing, because this scalar carries no expression; the type has no values, so nothing else can be returned". Those are true sentences about what the result means, and they are the same kind as `Real::Carries` at `Dual` (`Uncertified`), which the design already has. No value is inspected to decide it, so it is not the run-time check the clause forbids.

**But it passes on the weaker reading, and I think the clause asks for more.** "Ask whether a thing is needed before asking when it is safe": what breaks without an accessor every numeric scalar implements as "none"? Only that one generic body could not reach the question. And the clause's own examples (a door bounded `Certifies` so a dual cannot reach it) exclude by bound, not by an accessor that returns an absence. In generic code `if let Some(e) = x.expression()` reads as a check, even though the compiler deletes it. So I amend my shape:

- **Everything that means something only at `Sym` is bounded.** A trait `Symbolic` that only `Sym` implements, with a total `expression()` (no `Option`). The question, and later `register_equal` and `free_identity`, are reachable only under `T: Symbolic`. No numeric scalar implements an accessor at all.
- **The mint is then two pieces.** The general spelling is the generic body, admitted at every scalar. The closed spelling is a function bounded `Symbolic`. Which one a scalar takes is said once per scalar, in the one place the kernel already says per-scalar facts about profile elaboration: a per-scalar trait item whose sentence is about the direction ("this scalar's unit direction of a vector": the quotient at the numeric scalars, the closed-or-quotient at `Sym`). Rust has no other way to pick per scalar without a trait item admitted at each; the difference from the accessor is what that item claims. Here every admitted type has a sentence about geometry, and none has a sentence that says "nothing".
- **Cost, as a final state:** the numeric arm is written per scalar (or once as a default the numeric scalars inherit, which is the "general spelling" and has a sentence at each), and each future site that wants a closed spelling adds one such item. Today that is one site. I had weighed that as a reason for the accessor; under the new clause I think it is the right price, because each such item must then state its sentence per scalar, which is exactly the review the clause wants.

**So, as a choice for Ev, in one line each:**
- *Bounded (my lean now, likely):* nothing symbolic is reachable at a numeric scalar by any path; each site that chooses a spelling by scalar has a per-scalar item with a sentence about its own result.
- *Accessor (my position an hour ago, and the merged text's):* one mechanism for every site; numeric scalars implement it as a typed absence, on the `Carries` precedent.
Both make the question a type error at a numeric scalar. They differ in whether a numeric scalar implements anything at all that exists only for `Sym`'s sake.

**Should the `[scalars]` text cite `[unrepresentable]`?** Yes, as its reason, in either form. Amended first sentences under the bounded form (the rest of the merged bullet, with my (b) amendment, unchanged):

> - **What only a symbolic scalar can do is bounded to it** ([unrepresentable]: a signature is a claim). `Sym` carries its value's node in the document's expression graph; `f64`, `Interval` and `Dual` carry no expression, and code that needs one is bounded so that they cannot reach it: such a call does not typecheck. Where one construction has a closed spelling at `Sym` and a general spelling everywhere, each scalar says which it takes, and the general spelling is always correct. **The structural question** is such a bounded method: …

If Ev keeps the accessor form, the merged text stands and gains "([unrepresentable]: `Real::Expression` at a numeric scalar is a type with no values, as `Real::Carries` is at `Dual`)" after "by a type with no values".

**E12's sentence** under the bounded form: "A numeric scalar cannot reach the question: it is bounded to `Sym` (DESIGN [scalars]), and the general spelling is taken."

**What I did not check:** whether `Probe` or any nested scalar (`Sym<Dual<…>>` exists in tests) makes the per-scalar item awkward: `Sym<Dual<f64>>` is `Symbolic` and takes the closed spelling, which is right (identically zero has zero derivative); how the profile's existing per-scalar trait (`editor-core`'s `ProfileLane`, sealed, five impls) relates, since the mint lives a crate below it and the item would have to live in `profile` or `geom-core`.

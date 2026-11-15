## For Ev

**Recommendation (likely): a registration stays strict, and a symbolic zero is conditional.**
- A symbolic **zero** holds wherever the build takes the decisions this replay took. Those are the only values at which the body exists, and on a leaf that certifies that is the whole box. The wording for this is already agreed.
- A **registration** records two nodes as one function of the parameters, equal at every parameter value. The registration sentence becomes:
  > a session-level record that two DAG nodes denote one function of the parameters — equal at every parameter value, not only where a decision of the registrant came out as it did — made by the site that guarantees it. A fact that holds only past a decision is not registered: the construction spells that decision as a literal sign (`4·atan(σ·b)`, a radius `σ·ρ`), and the fact is then an identity of the expressions it built.

**Why strict for registrations.**
- A registration is the one thing in the symbolic tier that nothing checks. Its check cannot tell an identity from a coincidence.
- Under the strict rule a reviewer can verify a fact by reading it, because the decision appears in the expression as `σ`.
- Under a conditional rule, "true where my gate passed" rests on a gate the reader has to find and trust. That is how facts shaped like `|r| ≡ r` got written.
- The profile already works this way. Its `Facts::Registered` is documented as "a theorem of the reals at every value of the construction's inputs". A construction whose endpoint holds only to a decision takes `Facts::Decided` and registers nothing.

**What the registration callers show.** Only two files call the registration door: `geom-core` `arc.rs` and `sweep` `swept.rs`. I read every registration they make.
- **Strict today:**
  - the sweep's rigidity fact and its placed carrier end;
  - `register_endpoints`' landings and carrier ends;
  - its rim facts, while `|r|` is stored (`√(r²) = |r|` holds for every `r`).
- **Conditional:** the four facts that retire with the radius change.
- **One more whose doc reads conditional:** the line×line fillet's incoming tangency (`register_incoming_tangency`, through `Arc2::register_tangent_at`) and the fillet's endpoint facts (`fillet_facts`). Both say they hold "at every value of the corner's data the door's decisions hold for".
  - The fillet already spells its turn side as the literal `sgn` ("so no sign atom enters"). So the fact may already be an identity, with only its doc being over-cautious.
  - Under the strict rule it must be one or the other: shown to be an identity under the literals and its doc re-worded, or moved to `Facts::Decided` and not registered.

**Is there one text both readings can live under?** I don't recommend one.
- "Strict by default; conditional only if its doc names its decisions and it is listed apart" would permit a class of fact that nothing needs, once the radius change lands and the fillet is settled.
- It would also keep the unverifiable form available.
- If a future registration genuinely cannot spell its decision as a sign, add the class then, as a revision.

**The defensible second option, if you prefer it:** conditional, with the decisions named in the doc.
> a session-level record that two DAG nodes denote one real at every parameter value at which the decisions its doc comment names come out as they did here, made by the site that guarantees it.

Its cost: nothing checks that the named decisions are the right ones, or all of them.

**Confidence:**
- Strict for registrations: likely.
- No conditional registration besides the four radius facts and the fillet: likely. I read the two calling files, not every downstream doc.

## For the orchestrator

- **I've moved to A's position on registrations.** The deciding evidence is that `Facts` in `profile/src/lib.rs` is already strict by contract, and `Decided` is the existing home for a fact that holds only past a decision.
- **New item, to file or fold into the fillet's owner:** check `register_incoming_tangency` and the `Registered` arm of `fillet_facts` (`profile/src/path.rs`) against the strict rule. Either work out the identity under `sgn` and re-word the doc, or downgrade the fact to `Decided`.
- **Provenance:** A traced DESIGN.md's "takes the decisions it took" sentence to PR #4322, a design fork Ev approved (fork-log rows 92–93). That replaces my "not found".

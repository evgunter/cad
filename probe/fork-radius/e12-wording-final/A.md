## For Ev

**Recommendation (likely).** Keep a registration strict, and say why in one sentence. The two designers agree on everything else: a symbolic Zero holds wherever the construction takes the decisions it took, and E12's Decision paragraph should say so.

**What a registration is.** A site telling the symbolic tier that two expressions are equal. Nothing checks it: its witness cannot tell an identity from a coincidence.

**The split, in one line.** May the registrant's own proof rest on a decision that is not visible in the two expressions?
- *Strict (recommended):* no. The expressions a build makes already carry its decisions as literal signs, so a correctly spelled fact is equal at every value of its inputs. A fact that needs a sign the expressions do not carry is spelled with that sign, not registered.
- *Conditional (the other designer):* yes, provided the registrant's doc comment names the decisions its proof rests on.

**Why strict.**
- It is what the registration door's own doc already says, in two sentences side by side: the record is that two nodes "denote ONE function of the parameters"; a Zero through it is "zero wherever the construction takes the decisions it took". The record is strict, the Zero is conditional because the expressions exist only on this path. E12 should say the same two things.
- A reviewer can test "equal at every value of its inputs" by reading two expressions. "Equal where my gate passed" needs the reviewer to know which decisions ran and that nothing reads the fact elsewhere.
- No shipped registrant needs the conditional form. I read the allowlist: the registering sites are `Arc2::register_endpoints`, `register_radials`, `register_tangent_at`, and the sweep's `register_rigidity` and placed-carrier-end. The only facts that rest on an unseen decision are the ones that retire with the radius change (`register_radials`; the rim facts once `r` is stored without `abs`). The rest are identities of what they built: the tangency carries its side as a signed offset, the sweep's facts carry the turn as a literal axis. So strict makes nothing sound illegal. (likely; I did not trace every caller of `register_tangent_at`)
- Under the conditional sentence, `|r| ≡ r` with "rests on the radius gate" in its doc is a legal registration. Both designers rejected that fact for reasons the sentence would not capture.

**The two texts.** E12's registration bullet, first sentence:
- *Strict:* "a session-level record that two DAG NODES denote one function of the parameters, equal at every value of their inputs, made by the site that guarantees it. The expressions are the ones this path built, so its decisions are already in them as literals; a fact that needs a decision they do not carry is spelled with it, not registered."
- *Conditional:* "a session-level record that two DAG nodes denote one real wherever the construction takes the decisions it took, made by the site that guarantees it, whose doc comment names the decisions its proof rests on."

A combined text (strict by default, conditional ones listed apart) would create a category with no members today, so I do not offer it.

**Stakes.** Small. After the radius change both texts admit exactly the same registrations; they differ only in what a future registrant may do. The strict one is easier to loosen later than the conditional one is to tighten.

**Confidence.** Strict: likely. That no shipped registrant needs the conditional form: likely.

## For the orchestrator

- **B's provenance.** It does not move me, and I could not check all of it.
  - The phrase "zero wherever the construction takes the decisions it took" is in the door's doc on main (`sym.rs`, `Sym::register_equal`: "A zero through it is E12's claim like any other — zero wherever the construction takes the decisions it took"; and the module header: "zero wherever the construction takes the decisions it took AND the registrant's theorem holds").
  - Both sit beside "denote ONE function of the parameters" in the same doc, which is my split, not B's merge.
  - I did not find "Each kind was E12's claim" under `work/`, `docs/` or `crates/` on `origin/main`, and did not read #4901 / #4929. If that ruling says in Ev's words that the registrant's theorem may be conditional, it outranks my reading and the conditional text is the one to send.
- **Correction to me.** On `origin/main`, `register_radials` registers four facts (two points, per component), as B said; the eight I cited is the `doors-read` branch's version with the turned pair added.
- **Allowlist read.** `scripts/gates/register-equal-allowlist.sh` on main names `geom-core/src/arc.rs` and `sweep/src/swept.rs` as registrant homes (plus the scalar impls). Under magnitude storage the rim and `carrier_end` facts are unconditional (both sides non-negative); they become conditional only when `r` is stored, which is why they retire in the same unit.
- **Not checked.** `fillet_facts`' choice between registered and decided facts per arm; the callers of `register_tangent_at`.

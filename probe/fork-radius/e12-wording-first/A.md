## For Ev

**Recommendation (likely).** E12 should say two different things about two different objects, and today it says one thing about both.
- A **symbolic Zero** is a theorem at every parameter value in the leaf's box at which the build takes the decisions this replay took. That is DESIGN.md's wording, and it is what the shipped tier rests on. E12's Decision paragraph changes to say it.
- A **registration** stays strict: the two expressions are equal at every parameter value. A fact true only past a decision is not registered; the construction spells it with the decision's literal sign. E12's registration sentence keeps "one function of the parameters" and gains one sentence saying so.

**Provenance.**
- "IDENTICALLY ZERO in the parameters … for every parameter value" (E12 Decision): agent-written proposal, 2026-09-03, PR #1712; you approved it ("lgtm"). Your own words there were the goal ("make this parameter-aware"), not this sentence.
- "two DAG NODES denote one function of the parameters" and "a registration is an AXIOM": agent-written on 2026-09-06 inside the implementation PR #2048 (M10-9), merged by agents as "reserve recorded as taken". I found no ruling of yours on that wording.
- "a theorem for every value at which the construction takes the decisions it took here (E12)" (DESIGN.md, structural coincidence): written 2026-10-08 in PR #4322, a design fork on an `ev/` branch that you approved. It is the later text and the one you ruled on most directly.

**Which reading the tier rests on.** The conditional one, already, in three places: the quotient rule cancels a denominator because the replay's own arithmetic divided by it and had a value; a root's argument is non-negative for the same reason; and the decision read takes the arm of a `min`, `max` or select whose comparison is certified over the whole box. Only the tier's plain polynomial form proves the strict statement. So E12's "for every parameter value" claims more than the code proves, and has since those rules shipped. (sure)

**Why registrations stay strict.** A registration is the one thing in the tier nobody checks: its witness cannot tell an identity from a coincidence. A build's expressions already carry its decisions as literal signs, so a correctly spelled fact is an identity of the expressions that path built, and the strict test is one a reviewer can apply by reading. Allowing "true where my gate passed" is what let `|r| ≡ r`-shaped facts be written under wording that excluded them. (likely)

**Replacement text.**

E12 Decision, replacing "a funnel margin whose expression is IDENTICALLY ZERO in the parameters decides `Zero` before any enclosure is consulted — for every parameter value, at any box width":

> a funnel margin whose expression is zero at every parameter value in the leaf's box at which the build takes the decisions this replay took decides `Zero` before any enclosure is consulted, at any box width. Those are the only values at which this body exists. Where the plain normal form is the zero polynomial, that is every parameter value.

E12 registration bullet, after "a session-level record that two DAG NODES denote one function of the parameters, made by the site that guarantees it.":

> Equal at every parameter value, not only where a decision of the registrant came out as it did. A fact that holds only past a decision is not registered: the construction spells it with that decision's literal sign (`4·atan(σ·b)`, a radius `σ·ρ`), and it is then an identity of the expressions that path built.

DESIGN.md's clause does not change.

**Defects under this wording** (all retire with the radius change already agreed): `Arc2::register_radials` ("wherever the door decided `r` positive"); the tangent arc's radial facts ("wherever `ρ` has the sign σ"); the rim facts of `register_endpoints` and `register_rigidity` once `r` is stored without `abs` (`√(r²) ≡ r`). I know of no other.

**Confidence.** Zero's wording: sure. Registrations strict: likely; the alternative (let a registration be conditional if its doc names the decision) is defensible and I lean against it because nothing can check it.

## For the orchestrator

- **Provenance method.** The checkout was shallow (every `-S` hit landed on a graft), so I ran `git fetch --unshallow --filter=blob:none origin main` locally and `git log origin/main --reverse -S'<phrase>' -- <file>`. The commits:
  - `5b17b062e6` (#1712);
  - `b773363cc1` and `e80941f1bf` (both reach main through #2048, branch `m10/m10-9-registered-identity`);
  - `67f8116154` (#4322, branch `ev/intent-s4-how-the-door-proves`; the next merge message reads "fork log rows 92-93: Ev approved (PR 4322)").
- **Not read.** I did not read the PRs' comment threads; "no ruling found" on #2048 is from the branch name and merge messages, so check its comments before the body says it.
- **Not audited.** The full `register-equal-allowlist.sh` list against the strict test. The four defects above are the ones this question touched. That audit is a small item worth filing with U2.
- **No other E12 edits.** The Soundness bullet ("the identity holds at every real parameter point in the box") is consistent with the new Decision sentence and needs no edit; "What 'identically zero' buys" likewise.
- **If Ev prefers conditional registrations.** The registration sentence becomes: "denote one real at every parameter value at which the registrant's own decisions, named in its doc comment, hold", and the four defects become documentation fixes instead; the radius change is unaffected either way.

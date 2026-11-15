## For Ev

**Recommendation (likely).** E12 should state the conditional reading: *a symbolic zero holds wherever the construction takes the decisions it took*. Its strict wording should go: "identically zero in the parameters … for every parameter value" in the Decision paragraph, and "one function of the parameters" in the registration sentence.
- **You already ruled this meaning on #4901 (2026-10-12).** You took the designers' answer. The PR that implemented it (#4929) states: "Each kind was E12's claim (zero wherever the construction takes the decisions it took)".
- **Rule C was retired by the same ruling** "because it rests on a sign of the box", meaning a sign the construction never compared.
- **E12's own text predates that ruling and was never updated.** Its Decision paragraph comes from #1712, where you approved agent text with "lgtm". The registration sentence comes from #2048, which was agent-written and merged without comment.

**The shipped tier already rests on the conditional reading (sure).** Three examples:
- **The decision read** (`sym/signed.rs`). A `min`/`max`/`Select` is replaced by the branch the construction took, wherever that decision's sign is certified over the box. The result is not an identity in the parameters.
- **Rule E's "clause 1"** (`sym/quotient.rs`). It cancels a common factor only where the value channel did not divide by zero, which the box's certification guarantees.
- **The radial registration** (`Arc2::register_radials`). It holds only "wherever the door decided `r` positive". The registration door's own doc comment already says "zero wherever the construction takes the decisions it took".

So no tier shipped since #4929 matches the strict wording.

**What "the decisions it took" means, and why it is the right claim.**
- It means the construction's own comparisons: a gate, or a branch of a select. Each is decided at an exact witness, `Interval`, where the comparison is a certified sign over the whole box.
- It does not include a sign the construction never compared. That was rule C, and you retired it.
- A leaf that certifies has every decision definite across its box, or zero and proven. So "wherever it takes its decisions" is the whole box, which is exactly what a certified leaf needs.
- At an inexact witness (`f64`, a probe), a rounded comparison can disagree with the exact one, so the decision read is not asked there. The shipped tier already behaves this way.

**Replacement text.**
1. **E12's Decision paragraph.** Replace "a funnel margin whose expression is IDENTICALLY ZERO in the parameters decides `Zero` … for every parameter value, at any box width" with:
   > a funnel margin whose expression is zero wherever the construction takes the decisions it took — its own gates and branches, certified at an exact witness — decides `Zero` before any enclosure is consulted, at every point of the leaf's box, at any box width. A sign the construction never compared is not one of its decisions.
2. **E12's Soundness bullet.** Replace "the identity holds at every real parameter point in the box" with:
   > the zero holds at every point of the box where the construction takes the decisions it took; on a leaf that certifies, that is every point.
3. **The registration sentence.** Replace "a session-level record that two DAG NODES denote one function of the parameters, made by the site that guarantees it" with:
   > a session-level record that two DAG nodes denote one real wherever the construction takes the decisions it took, made by the site that guarantees it, whose doc comment names the decisions its proof rests on.
4. **DESIGN.md's `[coincidence]` clause** ("a theorem for every value at which the construction takes the decisions it took here (E12)"): unchanged. It already states this reading.

**Defects this exposes.** No registration is wrong in kind under this reading, but one states its proof wrongly:
- `Arc2::register_endpoints` says its rim facts hold "over the reals at every value of its inputs" and names no decision.
- Once a signed radius is stored, the rim fact is `√(r²) ≡ r`, which rests on the radius gate.
- It retires with the signed-radius change (U2), so there is nothing separate to file.

Every other registrant should be checked against change 3 when it is next touched: its doc comment either names the gates its proof rests on, or it rests on none.

**Confidence:**
- The conditional reading is the ruled and shipped one: sure.
- The replacement wording: likely.

## For the orchestrator

- **Provenance as traced.** This checkout is shallow, so `git log -S` only reaches the graft (`dffe67073`, 2026-11-13). I used GitHub search instead.
  - "takes the decisions it took": in the PR bodies of #4644, #4901 and #4929, and in #4929's diffs to `sym/signed.rs`, `k_stats.rs`, `topo/coincidence.rs` and `k-lint`.
  - "one function of the parameters": #2048 only, merged by Ev with no comment.
  - I did not find the PR that wrote DESIGN.md's `[coincidence]` sentence. Its file diffs in #3990, #4222, #4644, #5555, #5905 and #6004 do not contain it. Trace it before the PR cites it as Ev's own words; the meaning, though, is Ev's, from the ruling on #4901.
- **This is ratified text, so it waits for Ev.** Because #4901 already ruled the meaning, this is arguably a reword to match a ruling. Still put it to Ev as the PR's one question, as planned.
- **PR 6993 touches the same paragraphs** (E12, `[verdicts.consumers]`). Sequence the two so that neither restates the other.

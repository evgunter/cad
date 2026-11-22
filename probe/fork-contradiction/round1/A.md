## For Ev

**Recommendation as it now stands (likely).** I make the same three moves as before, and the other designer agrees with all three:
1. **A definite numeric sign returns at once.** The symbolic tier is asked nothing. Both the theorem cross-check and the registry check leave the decision path.
2. **A plain assertion runs in every profile.** Wherever the tier answers `Zero` at the exact witness (`Sym<Interval>`), the certified enclosure must contain 0.
3. **Four receipt columns go.** `theorems_disputed`, `checks_expanded`, `checks_frozen` and `registrations_contradicted` leave `SymCounts`, the per-run decision counts.

**Where we still differ: the old cross-check lives in the test suite, not in the product (likely).** It asks "would the algebra have claimed zero on a margin the numbers already decided?" I recommend it becomes a test instrument behind a test-only build feature. No shipped build contains it, and no receipt carries its report.

From the other report I take two things:
- The instrument carries all three events, not only the theorem one: algebra against a certified bracket, algebra against a rounded number, and registry against a definite sign. It also carries the retry attempts.
- It runs per PR in the rows that pin those columns today, and nightly over the `Sym<Interval>` document drives.

What I don't take is a dial on the user's drive, and with it the question of that dial's default.

Why it belongs to the suite and not the product:
- **Every event it can report is about the kernel, never about the user's document.**
  - At `Sym<Interval>`, a contradiction proves a kernel bug, and the right response is a panic, not a count.
  - A registry contradiction means a registrant in kernel code stated a false identity. That is also a kernel bug.
  - At `Sym<f64>`, a "dispute" says that `f64` rounding lost a theorem of the reals. It tells the user nothing about their part. And no shipped lane decides at `Sym<f64>`: the driver replays at `Sym<Interval>`.
  
  So a user asking for the report learns only whether our algebra has a bug, which is our test suite's question.
- **`[simple.checks]` settles it.** "No representation, stage or decision exists only so a check can read it." A drive dial and a report type would exist only for this check.
- **`[failure]` row 5 agrees.** Its contract is that a check's absence changes nothing shipped, and today's check breaks that (below).
- **Who reads the pinned columns today, and why.** They are all test rows, and every one is about the check or about the kernel's own registrants, never about a document's behaviour:
  - the witness-kind rows and the far-placement table measure the check's own false positives at `f64`;
  - the thread-count digests guard the check's coverage;
  - `m10_9_no_registrant_lies…` tests our registrants.
  
  They become rows of the test feature. `drive.rs` prints two of the columns only when they are non-zero; nothing in the product acts on them.
- **On a default:** with a test feature there is no default to decide. If a drive dial existed, "on until publish" would put back today's cost in every drive and every test row. That is the cost this question exists to remove, so I don't think the design leaves it open as your choice.
- **What we give up:** a document that has never been through the suite gets no per-run algebra cross-check. Before publishing, every document we run is either in the suite or in a demo, and a nightly lane can build the demos with the feature on.

**Is the always-on assertion sound? Yes, at every rung (likely).** E12 ratifies this: a symbolic `Zero` holds at every parameter value in the leaf's box at which the build takes the decisions this replay took.

The argument:
- In a replay that reaches the decision, every earlier decision was either definite over the whole box (certified) or itself a proven zero. So the set of box points where "the decisions hold" is the whole box.
- The margin is therefore 0 at every point of the box, and a certified enclosure contains its value at every point. So it contains 0.

Per rung:
- **Plain form and the rule rungs** (sqrt-square, Pythagoras, top-residual reduction): interval arithmetic includes the value of the expression at every admissible point, whatever atom values it reads, so an expression that is identically zero encloses 0.
  - A domain clamp (sqrt of a straddling argument) only widens the enclosure.
  - An uncertified enclosure (NaI) is refused as invalid before the tier is asked.
- **The quotient rule:** a zero numerator with a denominator that straddles 0 gives an enclosure that is unbounded or wide, never one that excludes 0.
- **The decision read and rule F's sign literals:** at `Interval`, the sign read was certified over the whole box, so the literal is the true one everywhere.
- **A retry:** the same claim as the first attempt.
- **A registration** is the one rung where the enclosure can exclude 0. That can only happen when the registrant's axiom is false over this box: the two sides differ by less than the band, and the door's "the enclosures meet" witness let it in. The assertion then panics. I think that is right: an exact witness refuting a registered identity is what the door's own `Contradicted` arm means, and registrants already assert on it. A firing can also be a defect in the value channel (an enclosure not containing its real, e.g. a transcendental's), which is a finding too.

In every case, a correct `Zero` with an enclosure that excludes 0 is impossible. If the assertion fires, either the proof or the enclosure is wrong.

Two cases sharpen what it catches:
- The case it catches that nothing catches today: an in-band sliver like `[2e-9, 5e-9]` with zero band 1e-9 and escalation band 1e-8. `Interval::sign_within` returns indeterminate, the tier then proves zero, and today that is stamped proven.
- At `Sym<f64>` there is nothing to check. A rounded number inside the band neither proves nor refutes zero, and a number outside it never reaches the tier.

**Ratified text.**
- One sentence in E12's registry bullet changes: "where a registration WOULD have, the run counts the contradiction" becomes: a registered zero that decides is checked against the exact enclosure, and the corpus instrument checks it against definite signs. That sentence is agent text from a fix pass (e80941f1bf), under the "lgtm" on #1712.
- `[simple.checks]` needs no change.

## For the orchestrator

**Q1: what moved me, and what didn't.**

From B I take:
- `door_zero` runs before `profile::set_origin(Origin::Assertion)`, so the cost profile charges its walks to the decision. That is a real attribution defect: file it under SYM.
- The instrument should carry all three events plus retries. I had only the theorem event "with the whole ladder"; B's grouping is better.
- `form_in`'s memo makes a repeated decision free, so the cost is first-time forms.
- B's "not the whole ladder" correction to the item title is right (`Path::Contradiction`: the first attempt, no door, no retry).

What didn't move me: a dial on the user's drive, and putting its default to Ev (Q2 above). My earlier argument that the check sits on the wrong population is restated in B's first bullet, so we agree.

**Q4: B's premise, one correction.**
- B says the 96 frozen nodes per slab finish are the check's and land in `frozen`. Only plain-walk freezes count there: `form_in`'s `frozen` closure does `if !early { sess.counts.frozen += 1 }`. M5's 96 were `Refusal::Ring` freezes from the `BOXFINISH_TRACE` hook in `form_in`, and its trace doesn't say which walk.
- The check's plain-walk freezes do pollute `frozen` outside a drive. Inside a drive, the column is rewritten as the leaf's NEED (`leaf_need`), and whether that closure includes nodes only the check asked about is unchecked.
- So "frozen returns to 0 under the fix" needs a measurement, not an assertion. The direction is right; the 96 is unverified as a `frozen` count.
- Otherwise B's premise matches mine and the tree.

**For the unit:**
- The test feature hooks the definite-sign arm exactly where the arm is today, under `#[cfg(feature = …)]`. Precedent: `sym-revalue-testing` and `revalue::note`/`settle`.
- Its report is a thread-local recorder, like `revalue::record`, not a `SymCounts` field.
- The assertion should state the argument above at the site, and its message should name the rung (`Rung` is available) and the enclosure.
- Flip `predicate::tests::a_proven_zero_is_offered_no_tolerance` to a straddling fixture.
- Before trusting the assertion, run the full suite plus the nightly drives: a firing on a registration rung is a registrant finding, not a reason to weaken it.

**Remaining risk I couldn't close.** `Hull` nodes are values computed from enclosures and stand in as free unknowns in a form. Inclusion still holds if the hull node's interval contains the value the build used at each point. I believe it does, by construction, but I didn't read `Hull`'s minting.

DESIGNER REPORT: an-undecided-at-rest-decision-climbs-the-ladder-on-a-margin-that-is-not-zero

## For Ev

**Recommendation (confidence: likely).** Before the symbolic tier builds any form, it should run a cheap test that can prove the answer is "no". The tier evaluates the margin's expression graph at one point of the leaf's box, in certified interval arithmetic. If the result excludes zero, the margin cannot be an identity, and the decision stays undecided without climbing the ladder. If the result contains zero, the ladder runs exactly as today. This changes no verdict anywhere. It turns "no" from "ran out of attempts" into a proof, and the proof costs one pass over the graph instead of thousands of forms.

Terms. *Margin*: the number a decision reads the sign of. *Undecided*: its box reading straddles zero. *The tier*: the symbolic identity test of ERROR-DESIGN E12. It turns the margin's expression graph (DAG) into a polynomial normal form. Its only positive answer is `Zero`: the margin is zero as an identity. *Ladder*: the tier's stages, in order: plain form, early walk, top reduction, registered-identity door, then two retry attempts under other rule sets. *Leaf*: one sub-box the subdivision driver replays. *Refutation*: the test proposed here.

**Premise check.**
- The item says the tier "pays the whole ladder to say nothing". More exactly, the ladder cannot say "no" at all (sure). A ladder exit means only "not shown under these rules". Opaque atoms can be related in ways no rule knows. The rules are also deliberately not monotone, which is the whole reason the retries exist. So no amount of climbing turns into evidence that a margin is not an identity. The missing piece is a test that can prove "no". It is not a cheaper ladder.
- The measured "sharing of forms across margins" is a symptom (likely). It is what building forms costs for margins that could never be zero. Skipping by predicate name moved the cost to the next margin because the skip was keyed by name. A skip keyed by evidence about the margin itself does not move the cost.
- For the caller, "no" and "not shown" are the same answer: undecided, which reads as in band (DESIGN `[verdicts.consumers]`: in band means only that the special case is not shown) (sure). The difference is only cost, so this is a cost and soundness design, not a semantics change.

**Why the refutation is sound (likely).** E12's soundness bullet makes a claim: a symbolic `Zero` is a theorem of real arithmetic about the DAG, and it holds at every point of the box. The code already asserts the box form of this. `discharge_checked` checks that a proven zero lies in the margin's certified enclosure over the box, by inclusion isotonicity. The refutation applies the same argument to a narrower input: each parameter at a single point of the leaf's box. A zero form is zero there too, so an enclosure at that point that excludes 0 means no rung could have answered. Three details:
- *Values with no expression*: opaque values (an iterated foot, for example), hulls, the free variable of a free identity, and nodes frozen on the budget. The form treats each of these as an unconstrained unknown, and the value-reading rule (`sym::signed`) declines on them. So any value is admissible. Each is evaluated at one point of its own value channel, which the session records when the value is minted, the way it already records parameter brackets.
- *Value-reading rules*: the min/max/select read takes an arm only when that arm holds over the whole box, so it also holds at the chosen point.
- *A false registration*: a registration is an axiom, and the door cannot tell an identity from a coincidence (E12). A false one is refuted at the point and stops being folded. That is a defect being caught, not a verdict being lost.

**The point.** It is fixed and deterministic: for each parameter, an off-centre fraction of its bracket that differs from parameter to parameter (keyed by the parameter's symbol). It is not the midpoint. A document's designed coincidences sit at the nominal, bisection puts the nominal on child boxes' faces, and two parameters with equal brackets would otherwise meet. A margin that happens to be about zero at that point climbs as today. That is rare, and the cost is the same as now.

**Worked example.** Take `|x| − x` with x over [−1, 1].
- At an off-centre x = −0.24, the refutation reads 0.48 and the decision stays numeric with no forms built. That is correct: over this box the select read cannot take an arm.
- In the child box [0, 1], the refutation reads [0, 0], the ladder runs, and the select read proves zero.
- The tour's pose discriminant reads [−1.4e-3, 1.4e-3] over the box. Its f64 witness is 4.9e-4, so at a point its enclosure is about 4.9e-4 ± 1e-15. It is refuted in one evaluation instead of a 113 ms climb.

**What each stage and retry buys, and for which asks.** Every stage, and both retries, can only ever buy a `Zero` on an identity (sure). On a refuted margin they buy nothing, and the refutation is what tells the two populations apart. Inside the ladder, nothing is reordered or removed.

**Where answers live.**
- *Positive (`Zero`)*: as today. It is taken at the decision, recorded where the decision is a structural act, and its forms are memoized per leaf, with plain forms also in the drive memo.
- *Negative (refuted)*: as one interval value per DAG node at the leaf's point, memoized for the leaf's session. A subgraph shared by several margins is evaluated once per leaf. That is the measured sharing, at the cost of one interval operation per node.
- Nothing negative goes into the drive memo. "No" depends on the box, as the `|x| − x` example shows, and recomputing it is cheap.

**What the caller and the driver are owed.** The same undecided reading as today, with the same margin, the same K-funnel outcome and the same receipts. The answer depends only on the DAG and the leaf's box, not on what was asked earlier in the leaf. The cost scales with the margin's graph, not with its forms.

**The per-drive store** (approved, not yet built). Its design needs nothing for this. The store serves whole node results across leaves, and an undecided decision is already part of the result it holds. The two are complementary: the store removes repeated keys, and the refutation removes the climb on first computation.

**Other answers, and why not.**
- (a) Skip the tier at at-rest decisions altogether (the probe's scratch change). This loses identities that are E12's own population. `curve_carrier_gap`'s `Zero` is an endpoint lying on its carrier. Rejected (sure).
- (b) Cap the forms each ask may build. This loses identities, and the answer then depends on order. Rejected (likely).
- (c) Ask the tier only from readings that use a `Zero` arm. `curve_carrier_clear` is a lower bound read only for `Positive`, so its `Zero` changes nothing at `topo::solid_contain::curved_edges::passes`, and at `Foot::on_carrier` it changes only which refusal is given. The answer is semantically right, but it needs a second decision kind beside the trilean. With the refutation in place, that ask costs microseconds. Lean: don't build it unless a measurement after the refutation shows such readings still costly (likely).

**Ratified text.** Nothing contradicts this. E12 has a decision read the value first and ask the tier only where the value reads no definite sign. That text was agent-written (commit eca9909b90, fork 272's recommendation, 2026-11-15) and is kept. E12 is silent on what "no" costs. Proposed addition to E12's "identity test, on demand" bullet:

> Before any form is built, the margin's DAG is enclosed with each parameter at one fixed point of the leaf's box, and each value without an expression at one point of its own. A zero form is zero there, so an enclosure that excludes 0 proves that no rung can answer, and the decision stays numeric.

E12's structural question (`Expression::proves_zero`) is left as it is. It has to be a function of the DAG alone, it is rarely asked, and the refutation would not change its answer anyway.

**Reversibility:** high. It is one gate in front of `ladder`, and with it on or off every verdict is the same.

## For the orchestrator

- **Not verified in code.** The node table stores no values, so opaque, hull and free leaves need one point recorded at mint (`Sym::opaque`, `free_identity`, and wherever `Hull` is minted). I did not read how the normal form treats a `Hull`. If it is not a free unknown in the form, its point must come from its own enclosure. A builder should confirm this in `sym/form.rs` before relying on the "any value" argument.
- **Not measured.** How long a point evaluation of these margins' closures takes; my estimate is ≤1 ms when fresh. Also whether all four margins actually refute at an off-centre point. That is likely for three of them: the f64 witnesses are ±0.024, 4.9e-4 and 0.3 to 8.6 mm. `curve_carrier_gap` and `azimuth_near_cut_side` are unknown: the gap has no f64 witness in M11, but with an opaque foot the form cannot be zero anyway. The acceptance check should be a re-run of M11-a and M11 item 5: same verdicts (211/301/511, symbolic and numeric counts unchanged), and the climb count on the four predicates near zero.
- **Soundness rests on every rule being a real identity over the box.** That is E12's claim, already asserted at the box by `discharge_checked`. Debug builds could cross-check by running the ladder on a sample of refuted asks and asserting it returns `None`; that is a test instrument, per [simple.checks].
- **Off the question.**
  - (1) The growth guard in `ladder` (`SymRetry::max_forms`, checked per attempt per leaf) makes a decision's answer depend on asks made earlier in the leaf. That bears on `a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state` and on the store's "same with it on or off" claim. The refutation lowers the pressure on the cap but does not remove the dependence. Worth a line on that row.
  - (2) M11-b's unbounded shape-report render (`report::render_node` in `Sym::decide_sign`) runs on every undecided decision while the report is installed. A refuted decision has no blocking form to render, so the gate should come before it too. The renderer still needs its own bound; that is its own row if none exists.
  - (3) In `passes`, `curve_carrier_clear` is decided and then projected with `== Ok(Positive)`. That is fine, but it is the call site for option (c) if it is ever wanted.
- I assumed "at-rest" means the finished-body checks under `topo::AtRestPolicy`. The recommendation does not depend on that: the gate applies to every undecided decision at a symbolic scalar.

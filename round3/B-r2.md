## For Ev

**Recommendation (likely; changed by the measurement).** One final state, in three parts:
1. **The engine says why a local certificate did not hold on a cell pair, and halves the cell only when a smaller cell can change that reason.** Otherwise it refuses that cell at once, typed.
2. **The answer is one fact per face pair**: *apart* (certified), *crosses* (with its witness) or *unread* (with the engine's refusals). The gate lists every pair that is not apart.
3. **A verified crossing settles its face pair. A refusal settles only the cell pair it was made on.** Nothing stops an ask, or a pair, at a refusal.

**First step, until part 1 is built:** the ask stops at its first refusal. It is one line, it is measured, and it comes out again when part 1 lands.

I withdraw "a refusal settles its face pair", which both first reports recommended. The measurement answers it on both counts: price and crossings.

**Terms.**
- An *ask*: the question put to one solid, "is your boundary embedded".
- A *cell pair*: a patch of one face against a patch of another. The engine halves cells until each pair is decided. A halving is a *split*.
- *Touching* cell pairs share a point of the boundary and are decided by a *local certificate* (how the two faces leave their shared edge or vertex). The rest are decided by distance.
- A *refusal*: a cell pair the engine could not decide.
- The *depth floor*: 40 halvings, where `f64` stops carrying information. The *budget*: 65,536 cell pairs an ask.

**What the measurement shows** (372 refusing asks: the twelve dearest, a sample by refusal shape, and the asks of the 22 tests that cross).
- **The seconds are spent reaching refusals, not running on after them.** On the twelve dearest (13 asks, 4,434 s, which is 39% of all refusal seconds in the suites) the first refusal comes at once, within 3 splits. The pair that refused spends nothing more. Then 4 to 21 other pairs refuse in their turn: 3,250 s go by before each reaches its own first refusal, 1,150 s after. Pairs that certify cost 28 s.
- On the cheap asks the first refusal comes at the depth floor, and 71 to 93% of the time precedes it.
- **So the row's title names the wrong thing.** What is dear is how the engine finds out that a pair cannot be read: by halving it until the `f64` grid or the budget runs out.
- Two thirds of refusing asks hold more than one refusing pair (2 to 12). 38 of 136 crossing asks also hold one.
- Of the 38 asks that found a crossing after a refusal, the crossing is in the pair that refused first in 19, in another refusing pair in 2, in a pair that never refused in 17.

**Part 1, the engine (what the numbers point at).**
- Today a local certificate that fails answers "split: a smaller pair may", with no reason (`Outcome::Split`). The engine cannot tell "this cell is too big" from "no reading exists for this shape at any size". It learns the second only by exhausting depth or budget.
- The arm knows more than it says. A face's part of a cell can be unread "for a reason no smaller cell changes" (`Unpresented::Structural`), but the pair still splits unless both parts are. A departure sign decided in the band, or wholly inside it, comes back as "no sign" (`interval_sign_as`) and splits. Distance cells already refuse there (`Sliver`).
- Final state: a failed certificate carries its reason and the quantity a smaller cell would reduce. A margin that straddles the band's edge: split. More boundary in the cell than one reading takes: split. A margin inside the band, a shape no reading takes, or a child failing exactly as its parent did: refuse that cell now.
- This is `[one-way.readings]` ("an unlisted arm refuses typed, naming what it lacks; no arm answers a default value") and `[topology.tiers]` ("a reading inside the band refuses as ambiguous") applied where the code now reaches them only by exhaustion.
- At `Interval` and `Sym` a margin can be wide because the parameter box is. No smaller cell narrows that, so it refuses at once and names the box.
- It retires `CUSP_DEPTH`, the one existing early floor. Its own doc calls it "a trade and not a derivation" and says an uncertifiable edge otherwise ends "by budget exhaustion".
- Leaves possible: a reason typed wrongly refuses a cell a smaller one would have certified. That is an over-refusal, never a wrong "embedded".

**Parts 2 and 3, the answer.**
- Makes true: every finding names its faces; an unread pair lists each shared edge or vertex it is unread at, once; no ranking between a crossing and a refusal is left.
- Why a crossing settles its pair and a refusal does not: a crossing is a fact about the two faces, and nothing read later changes it. A refusal is a hole in a proof about one place. The two faces may still cross elsewhere: they did in 19 of the 38.
- Leaves possible: a pair refused whole at its first cell (a shape unread at any size) is not searched for a crossing. What the engine cannot read it does not search.
- A crossing no longer ends the ask. Measured: 7.8 s becomes 15.3 s over 136 crossing asks.
- The clearance assertion keeps its single verdict.

**Worked example.** A fillet T (a torus face) meets a plate P along edge e and a wall W along edge g. Each edge ends in a corner the engine has no reading for. T also dips through P away from e. A rib R passes through W.
- Today: the cells around each corner are halved again and again, each failing as its parent did, down to the depth floor or the budget. That is the 150 to 564 s asks. One crossing is reported and the unread corners are dropped.
- Stop the ask at its first refusal: "T and P: unread at the corner of e", within a second. Neither crossing is looked for.
- Settle each pair at its first finding (the first reports): T against P closes at the corner, so the dip is never reported. T against W still costs its thousands of splits. R through W is reported.
- Recommended: a corner cell refuses when its child fails exactly as it did, a few splits in. Everything else is read once. Listed: "T and P cross here", "T and W: unread at the corner of g", "R and W cross here".

**The price, measured.**
- Stopping the ask saves 99.9% on the twelve dearest and 6 to 37% elsewhere (mostly under 20%). It reports all 38 of those crossings as kernel limits, which is 28% of crossing asks.
- Settling per face pair saves 25.9% on the twelve dearest and 0.5 to 37% elsewhere. It reports 21 of the 38 as kernel limits.
- Part 1's saving is not measured. Its target is the 3,250 s and the 71 to 93% that precede refusals, which no stopping rule touches. The sample is not the whole suite.

**Why this order.** Part 3 without part 1 is today's behaviour and today's price. Stopping the ask is the only cut that removes the minutes-long asks now. Once part 1 exists the stop buys nothing and hides crossings, so it goes.

**The alternative you may prefer: stopping the ask as the final state.** Simplest, and its price is certain. It leaves a self-crossing body reported as a kernel limit in 28% of crossing asks, with no lever for the person; one unread pair reported where two thirds of refusing asks hold several; and the cost of reaching the first refusal untouched.

**Rejected.** Today's rule. Settling per face pair as a stopping rule (above). A second, smaller budget for the search.

**Ratified text I would change.**
- `[topology.tiers]`, after "one fact per pair of faces": the three facts; a crossing settles its pair; a refusal settles the cell it was made on; the gate's findings are the pairs not apart.
- `ERROR-DESIGN` E7's single trichotomy (agent-drafted, ratified with the whole document, older than the gate): for the embedding question it is read per face pair. The clearance assertion's stays.
- Nothing in `[one-way.readings]` or the in-band sentence changes. Part 1 makes the code meet them.

**Confidence.**
- Today's rule is wrong, and the seconds are in reaching refusals: sure (measured).
- A refusal does not settle a face pair: sure (19 of 38).
- Part 1 as the right place to act: likely. That it removes most of the 3,250 s: unsure; it needs the measurement named below.
- The first step: sure on its numbers. Whether to take it is sequencing.

## For the orchestrator

**1. What moved.** I no longer recommend the face pair as a stopping unit. I keep it as the shape of the answer. The final state adds an engine part that neither first report had and drops every stop at a refusal. First step: stop the ask.

**2. My first report against the numbers and the other report.**
- I guessed the run-on was in the pair that refused. It is not: a = 0 on the dearest. The mechanism I described is real, but it runs in other pairs, before their own first refusal.
- My conditional ("if the spend in other pairs dominates, stopping is the answer to the price and the pair a later step") has its condition met, and I hold to the first half as a first step. The second half is answered by M3 and by c-pre: the pair as a stopping unit is not the later step.
- The other report's "stopping early saves less than the row suggests" holds on the bulk of asks (6 to 19%, first refusal at the depth floor) and fails on the twelve dearest. It did flag that those "may be of the immediate kind". Its "a refusing ask costs about what the same body costs once the engine reads the refused pair" is refuted: 4 to 21 further pairs refuse, at thousands of splits each.
- Both first reports called crossings and refusals behaving alike a merit. M3 says they are not alike.
- I said a budget run-out should name the pairs still open at a level overflow. d = 0: run-outs arrive at admission, cell by cell, so each already belongs to a face pair.
- I put reading on after a crossing at about nine times. Measured: three.
- The new paragraph "ask whether a thing is needed before asking when it is safe" applies to both first reports: each gave a rule for when a pair may stop being read before asking whether stopping is needed. With part 1 it is not.

**3. The engine part, in the tree.**
- `Outcome::Split { arm, shared }` (`injective.rs`) gains its reason. `Bound::pair`'s `Outcome::Split` arm in `clearance.rs` gets the test distance cells already have (`Bound::terminal`).
- `departure` returns `split` whenever either part is not presented. `stuck` refuses only when both are. `work/embed/a-contradicted-extent-refuses-with-no-typed-cause.md` records the one-sided case running to the depth budget.
- `Unpresented::Local` is returned from about 30 sites, among them the fall-through `_ =>` arms of `present_chart` and its neighbours. Each needs the quantity that shrinks (edges in reach, copies of a periodic sheet) or becomes an unread shape. The two "reads local at every depth" rows are instances.
- `interval_sign_as` maps `Ok(Sign::Zero)` and `Err(Indeterminate)` both to `None`. The departure margin in `sign_on` is `grad·f_x` times the cell's span, a length that halves with the cell, so below some size it is inside the band at every depth. I read this; I did not run it.
- "A signature is a claim", for `embedded<T: Decide + CertifiedBounds>`. `f64` and `Probe`: one body; a margin's width is the cell's. `Interval` and `Sym`: a family; a crossing at one member is a crossing; a margin wide because the family is refuses at once and names the box. That replaces `blocking_width` on `embed/gate`, which guesses from vertex coordinates. A dual is excluded by the bound (the `compile_fail` doc test on `embedded`).

**Measurement part 1 turns on (M5).** On the 13 dearest asks and a sample of depth-floor asks, for every touching cell that answers `Split`: which part failed and how (`Structural`, `Local` and its site, presented with a sign undecided, and then its margin and whether it is wholly in band), and whether the child failed as its parent did. Report, per refusing pair, the depth at which a non-shrinking reason first appears and the splits after it. That is part 1's saving.

**Sequencing.**
- The first step is `Break` at each lower-bound refusal site of the injectivity question, on `embed/gate`. It needs no type change. It is the injectivity question's alone: for a metric lower bound a sliver must not hide a definite violation.
- Part 2 can land with or after it: per-candidate state keyed by `Task.k`, `Side::windows` answering per face, `EmbeddingUncertified` gaining its faces.
- Part 1 touches `injective.rs` beside the engine rows in flight. The nine causes traced on the cone and torus buckets are the first reasons to type; I have not seen that trace.
- Lane C waits on the engine rows in any case, so the first step buys cheaper census runs and removes the 4 tests' 4,434 s. It protects nothing on main.

**Seams.** `Sweep::run` and `Bound` are shared with CLEAR's metric questions. `validate.rs` is TOPO's.

**Could not check.**
- That the pairs behind the 3,250 s split for non-shrinking reasons. This is inferred from the rows and the code, and M5 decides it.
- Whether the 19 same-pair crossings are still found once part 1 refuses cells earlier and so sheds fewer distance cells beside an unread edge. Re-run M3 on a part 1 prototype.
- The suite-wide saving of any rule: the sample is cheapest-first within each shape.

**Brief and housekeeping.**
- The two new paragraphs of `designer.md` are not in this checkout's working tree. I read them from `origin/main` after a `git fetch`, which updated the checkout's remote refs. Nothing else was written.
- `measurement.md`'s second row reads 89 asks in 2.8 s; I took it as the sample's total.
- Not read: `draw.md`.
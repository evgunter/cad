## For Ev

**You are asked two things. I recommend yes to both.**
- **(a) Now:** the embedding question ends at its first finding, a verified crossing or a refusal, and reports that one finding.
- **(b) Rule the final state now, not later:** the question ends at its first verified crossing and at nothing else. With no crossing, it refuses and lists every refusal it met, each once. The stop at a refusal in (a) is then a named exception that lasts until the engine finds out cheaply that a place cannot be read.

Both reports hold (a), the engine change below, and the final state in (b). What is left is only whether you rule (b) now or are asked again once it is measured.

**Terms.**
- An *ask*: the question put to one solid, "is your boundary embedded".
- A *cell pair*: a patch of one face against a patch of another. The engine halves cells (a *split*) until each pair is decided. *Touching* cell pairs share a point of the boundary and are decided by a *local certificate* (how the two faces leave their shared edge or vertex). The rest are decided by distance.
- A *refusal*: a cell pair the engine could not decide. Its *place* is its two faces and the edge or vertex they share there.
- A *crossing*: a verified pair of points where the boundary meets itself.
- The *depth floor*: 40 splits, where `f64` stops carrying information.

**Why the final state is this one (sure).**
- "Embedded" says every pair of faces is apart. One crossing settles that as false, whatever else is unread. A refusal settles nothing: another pair may still cross. So the question may end at a crossing and has no reason of its own to end at a refusal.
- The two verdicts have different reasons. "Crosses" is established by one witness, so one is reported. "Refused" is established by every place the proof could not be made, and no one of them more than another, so all are reported. That is "every finding kept, none folded" (your words, PR 6326) with nothing ranked.
- It is what the person is owed. A crossing is the model's fault and has a lever. A place the kernel cannot read has none. A body with both should hear about the crossing.

**Why (a) comes first.** Today the final state's logic is paid at a price nobody can wait for, because of a defect in the engine. Stopping at the first refusal is one line and removes the minutes-long asks now.

**The engine defect (likely).**
- A local certificate that fails answers "split, a smaller cell may hold" with no ground for it. The engine learns that no cell of any size will hold only by reaching the depth floor or the budget.
- Final state: "split" carries its ground, the thing a smaller cell would reduce. A failure with no such ground refuses that cell at once, typed.
- `[one-way.readings]` already says so: "an unlisted arm refuses typed, naming what it lacks; no arm answers a default value". This is a defect to file and a unit to build, not a ruling.

**Worked example.** A fillet T (a torus face) meets a plate P along edge e, and a wall W along edge g. Each edge ends in a corner no certificate reads.
- *This body, which is valid.*
  - Today: the cells at each corner are halved again and again, each failing as its parent did. After 150 to 564 s: "T and P: unread". The other corner is dropped.
  - Under (a): the same one finding, in about a third of a second.
  - Final state: each corner refuses a few splits in, and the rest is read as on a body that holds. "T and P: unread at the corner of e. T and W: unread at the corner of g."
- *The same body with a rib R passing through W.*
  - Today: "R and W cross here", quickly.
  - Under (a): "T and P: unread" if the walk meets that corner first. Nothing about the rib.
  - Final state: "R and W cross here", and the ask ends there. One crossing a run; the next finish reports the next.

**The measured price.** Population: 372 refusing asks instrumented by the gate lane. They are the 13 dearest in the suites, 153 drawn by refusal shape (cheapest first), and 206 from the 22 tests whose asks cross. It is a sample, not the suite.
- The 13 dearest take 4,434 s, 39% of the suites' 11,434 s on refusing asks. Each refuses within 3 splits. Then 4 to 21 other face pairs refuse, and 3,250 s pass before each reaches its own first refusal.
- (a) leaves 4.6 s of those 4,434 s. On the cheap asks, whose first refusal comes at the depth floor, it saves 6 to 37%.
- Over the suites (a) should remove roughly 5,000 to 5,600 s. That figure is extrapolated.
- What (a) gives up: 38 asks, in 22 tests, found a crossing after meeting a refusal. Under (a) each reports the refusal. That is 38 out of several thousand asks that cross.
- The engine change is not measured. Its target is the 3,250 s and the 71 to 93% of the cheap asks' time that precedes their first refusal, which no stopping rule touches.
- The final state takes effect when a refusing ask, with the engine change in, costs about what a body of its size costs when it holds. If the change saves less than that, (a) stays the rule and you are asked again.

**What each leaves possible.**
- (a): a body that crosses and also has an unread place is told only of the unread place, when the walk meets it first. Which refusal is reported depends on face order and depth.
- Final state: a place refused whole (a shape unread at any size) is not searched for a crossing inside it. A reason typed wrongly refuses a cell a smaller one would have certified, which is an over-refusal and never a wrong "embedded".

**Scope.** The embedding question only. The clearance assertion keeps today's rule: there a violation must still outrank a refusal met earlier, and nothing measured says it is dear.

**Dropped by both reports.** One finding per face pair as the answer's shape. A face pair settled by its first finding (19 of the 38 crossings lie in the pair that refused first). Reading on after a crossing. A second, smaller budget. The change to `ERROR-DESIGN` E7: its three verdicts per question stay.

**Ratified text.**
- If you rule (b) now, one change to `[topology.tiers]`, after "one fact per pair of faces": "The question ends at its first verified crossing. Without one, every refusal it met refuses it, each listed once by its faces and place. Until [the engine row], it also ends at its first refusal." The table of canonical places already carries an exception in this form (yours, PR 6371).
- If you rule only (a): "The embedding question ends at its first crossing or refusal; that is its finding." Taking the final state later is then a second change.
- Every refusal names its two faces. Three kinds name none today (a budget, a sliver, an unverified witness).

**Confidence.**
- (a): sure on its numbers.
- The final state's logic: sure. That it becomes affordable: unsure until the engine change is measured.
- The engine defect as the cause: likely. It fits the code and the tracker's own rows; nobody has traced it cell by cell.

## For the orchestrator

**I sign `joint.md` with three amendments.**
- **J1, add:** every refusal the ask can report names the two faces of the cell pair it was made on. `Budget` is not the only one without them: `Sliver { predicate }` and `WitnessUnverified { what }` carry none either. A sliver is the refusal with the model's lever, so a person most needs to know where it is. J3's "each once, by its place" needs the faces on all three as well.
- **J3, change the condition:** the gate is the price alone. "The 38 are still found" should be reported with the measurement and not be a condition. Stopping finds none of them, so walking on is no worse on that count however many it reaches. The engine change may shed fewer distance cells beside an unread edge, so I expect fewer than 38 and would not want that to block J3.
- **J6, add the third form:** if Ev rules J3 now, it is one text change and not two, written as the rule plus a named exception "until" the engine row, on the precedent of the at-rest row of `[one-way.places]` (Ev, PR 6371). That is what (b) above offers. I recommend ruling now: the logic does not depend on the measurement, only its affordability does.

**Where I moved, so the two forwarded sections agree.**
- Round 3 dropped every list on the needs test. I now take J3's list of refusals met. My test was against paying to find them. In J3 the walk has them in hand, reporting one would need a rule for which, and some are the model's to act on (`Sliver`, `TorusNotRing`). The account given to Ev (the refusals are jointly the reason for "refused"; one witness is the reason for "crosses") is why the list and J4 do not conflict.
- I hold J4. The other round-3 report's question to Ev sets its position against "read on and list every face pair", which was my round-2 position and not my round-3 one. If its final section still poses that question, the two will disagree about what I hold.
- The other round-3 report repeats "28% of the asks that cross". That was my round-2 error: 136 is the crossing asks of the 22 tests, not the suite's. It should not reach Ev.

**New for the unit.**
- Under J3, `Embedding::Refused` and `BoundVerdict::Refused` carry a non-empty set for the injectivity question, deduplicated by payload once every refusal has its faces. The metric questions keep one. E7's wording does not change.
- `topo::shell`'s `CavityUncertified { refusal }` reads the same door. SHELL decides whether it takes the set or its first entry; announce the seam.
- The J3 criterion made measurable: on a prototype of J2, over the refusing population, the seconds of walking on against the seconds of stopping, set beside the holding asks of the same face-count bucket in the census's distribution table. "About what the body costs when it holds" cannot be measured on the body itself, since it does not hold.

**Not checked.** How many of the 38 survive J2. The count of crossing asks in the second census; "several thousand" rests on the first (8,727). Not read: `draw.md`.